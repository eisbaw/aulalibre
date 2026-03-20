# Document Locking and Journaling Interaction Analysis

Investigation of how document locking works in Aula's secure documents module and how it interacts with the ESDH journaling flow. Discovered during TASK-35 ESDH analysis.

## Key Finding: Locking and Journaling Are Independent Mechanisms

Contrary to the initial hypothesis that documents might be auto-locked during journalization, the decompiled code reveals that **locking and ESDH journaling are two entirely independent features** that happen to coexist on the same `SecureDocumentDto`. There is no code in the app that automatically locks a document when it is journalized, nor any code that prevents journalization of unlocked documents.

The app is a thin client. The server could enforce auto-locking on journalization, but the app contains no such logic -- it treats `IsLocked` and `JournalingStatus` as orthogonal properties.

## Document Locking Data Model

### SecureDocumentDto Properties

**File:** `decompiled_csharp/AulaNative/AulaNative.Models.Document/SecureDocumentDto.cs`

| Property | Type | Purpose |
|----------|------|---------|
| `IsLocked` | `bool` | Whether the document is currently locked |
| `CanEditLockedStatus` | `bool` | Server-controlled flag: whether the current user has permission to toggle the lock |
| `CanEdit` | `bool` | Whether the current user can edit the document content |
| `JournalingStatus` | `JournalingStatusEnum` | Current ESDH journaling state (independent of lock state) |

Both `IsLocked` and `CanEditLockedStatus` are set by the backend. The app never computes these locally.

### UpdateDocumentStatusRequestModel

**File:** `decompiled_csharp/AulaNative/AulaNative.Services.Web/UpdateDocumentStatusRequestModel.cs`

```
UpdateDocumentStatusRequestModel:
  - DocumentId: long
  - IsLocked: bool
```

This is a simple toggle request. It sends the desired new lock state. There is no reference to journaling status in this request model.

### API Endpoint

**File:** `decompiled_csharp/AulaNative/AulaNative.Configuration/Urls.cs`

```
UPDATE_DOCUMENT_LOCK_STATUS = AulaUrl(1210, BackendUrlApi + "document/" + "updateLockedStatus")
```

POST to `{backend}/api/v{N}/document/updateLockedStatus` with `UpdateDocumentStatusRequestModel` body.

## Lock/Unlock Toggle Flow

### UI Layer (Android)

Users toggle lock state via:
1. **Swipe action** on document list items (`SecureDocumentRecycleViewHolder.LockDocumentBtn`)
2. **Action button** on document details page (`SecureDocumentDetailsActivity._lockDocActionBtn`)

Both trigger the same ViewModel method.

### ViewModel Layer

**File:** `decompiled_csharp/AulaNative/AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentSimpleViewModel.cs`

The `UpdateLockStatus()` method:
1. Shows a confirmation dialog with text dependent on current state:
   - If currently locked: shows unlock warning (`DOCUMENTS_UNLOCKED_CONFIRM_TEXT`)
   - If currently unlocked: shows lock warning (`DOCUMENTS_LOCKED_CONFIRM_TEXT`)
2. On user confirmation, calls `DocumentServiceManager.UpdateDocumentLockedStatus()` with the **toggled** state (`!SecureDocumentDto.IsLocked`)
3. On success, calls `RefreshData()` to reload the document from the server

### Service Layer

**File:** `decompiled_csharp/AulaNative/AulaNative.ServiceManagers/DocumentServiceManager.cs`

```
DocumentServiceManager.UpdateDocumentLockedStatus(UpdateDocumentStatusRequestModel)
  -> DocumentService.UpdateDocumentLockedStatus(UpdateDocumentStatusRequestModel)
    -> SimplePost(Urls.UPDATE_DOCUMENT_LOCK_STATUS, arguments)
```

Returns `bool` indicating success/failure.

## Permission Model for Lock Toggle

**File:** `decompiled_csharp/AulaNative/AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentSimpleViewModel.cs`

```csharp
public bool AllowToEditLockStatus
{
    get
    {
        if (SecureDocumentDto.CanEditLockedStatus)
        {
            return ProfileManager.Instance.IsEmployee();
        }
        return false;
    }
}
```

Two conditions must both be true:
1. **`CanEditLockedStatus`** -- Server-provided flag on the document itself. The server decides per-document who has this permission. The app does not know the backend logic for granting this.
2. **`IsEmployee()`** -- The current user must be logged in as staff (not a parent/guardian).

The lock/unlock action only appears in `GetActions()` when `AllowToEditLockStatus` is true. This controls visibility of the swipe action and the details page button.

## How Locking Blocks Sharing

**File:** `decompiled_csharp/AulaNative/AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentOverviewPageViewModel.cs`

The `ValidateSharingSecureDocuments()` method enforces a rule: **locked documents cannot be shared**.

```csharp
public static async Task<bool> ValidateSharingSecureDocuments(SecureDocumentSelectingResultModel result)
{
    var lockedDocs = result.Documents.Where(doc => doc.IsLocked);
    if (lockedDocs.Any())
    {
        // Show error dialog with title "DOCUMENTS_CANNOT_SHARE_WTH_LOCKED_DOCUMENTS"
        // listing the names of locked documents
        return false;  // Blocks the sharing operation
    }
    return true;
}
```

This is the only app-side enforcement of locking behavior. Notably:
- Locking does **not** hide the edit button (the `AllowToEdit` property checks `CanEdit` from the server, not `IsLocked`)
- Locking does **not** prevent PDF export
- Locking does **not** prevent deletion (if the user has delete permission)

The server likely enforces additional restrictions when `IsLocked` is true (e.g., rejecting edit API calls), but the app does not preemptively disable these actions client-side except for sharing.

## How Locking Does NOT Affect AllowToEdit

This is a notable finding. The `AllowToEdit` property:

```csharp
public bool AllowToEdit
{
    get
    {
        SecureDocumentDto dto = SecureDocumentDto;
        if (dto != null && dto.CanEdit && dto.DocumentTypeEnum == DocumentTypeEnum.Note)
        {
            return ProfileManager.Instance.IsEmployee();
        }
        return false;
    }
}
```

It checks `CanEdit` (server-provided), `DocumentTypeEnum == Note`, and `IsEmployee()`. It does NOT check `IsLocked`. This means:
- The edit button may appear for locked documents
- The server's `CanEdit` flag may already account for lock state (i.e., server sets `CanEdit = false` when locked), but this is opaque to the app
- Or the server rejects edit requests for locked documents at the API level

## Filter System: Locked and Journaling as Parallel Filters

**File:** `decompiled_csharp/AulaNative/AulaNative.Enums.Document/DocumentFilterEnum.cs`

```
DocumentFilterEnum:
  All, Unread, Locked, Published, PublishInProgress, PublishFailed
```

The filter list is built dynamically based on role and permissions:

| Filter | Condition to Show | Maps to API Parameter |
|--------|------------------|-----------------------|
| All | Always | (no filter) |
| Unread | Always | `FilterUnread = true` |
| Locked | Employee only | `FilterLocked = true` |
| Published | `JOURNALING_TO_ESDH` permission | `FilterJournalingStatus = Completed` |
| PublishInProgress | `JOURNALING_TO_ESDH` permission | `FilterJournalingStatus = InProgress` |
| PublishFailed | `JOURNALING_TO_ESDH` permission | `FilterJournalingStatus = Failed` |

These are **mutually exclusive** -- only one filter can be active at a time. The user cannot simultaneously filter for "Locked AND Published" documents. This is a single-select filter, not a compound filter.

The mapping in `GenerateGetSecureDocumentArguments()`:
```csharp
arguments.FilterLocked = documentFilterEnum.HasValue && documentFilterEnum == DocumentFilterEnum.Locked;
arguments.FilterJournalingStatus = documentFilter?.Filter switch
{
    DocumentFilterEnum.Published => JournalingStatusEnum.Completed,
    DocumentFilterEnum.PublishFailed => JournalingStatusEnum.Failed,
    DocumentFilterEnum.PublishInProgress => JournalingStatusEnum.InProgress,
    _ => null,
};
```

## Revision History: Lock and ESDH Events Are Tracked Separately

**File:** `decompiled_csharp/AulaNative/AulaNative.Enums.Document/RevisionChangeTypeEnum.cs`

The `RevisionChangeTypeEnum` has distinct entries for locking and ESDH events:

### Lock-Related Events
| Event | Wire Value | Display Text Key |
|-------|-----------|------------------|
| `Locked` | `"Locked"` (default serialization) | `DOCUMENTS_REVISION_FILE_LOCKED` |
| `Unlocked` | `"Unlocked"` (default serialization) | `DOCUMENTS_REVISION_FILE_UNLOCKED` |

### ESDH-Related Events
| Event | Wire Value | Display Text Key |
|-------|-----------|------------------|
| `SentToESDH` | `"sent_to_esdh"` | `DOCUMENTS_REVISION_FILE_SENT_TO_ESDH` |
| `JournalizedToESDH` | `"journalized_to_esdh"` | `DOCUMENTS_REVISION_FILE_JOURNALIZED_TO_ESDH` |
| `EsdhJournalizationFailed` | `"esdh_journalization_failed"` | `DOCUMENTS_REVISION_FILE_ESDH_JOURNALIZATION_FAILED` |
| `ResentToESDH` | `"resent_to_esdh"` | `DOCUMENTS_REVISION_FILE_RESENT_TO_ESDH` |
| `ManuallyJournalizedToESDH` | `"manually_journalized"` | `DOCUMENTS_REVISION_FILE_MANUALLY_JOURNALIZED_TO_ESDH` |
| `MarkForManualJournalize` | `"mark_for_manual_journalize"` | `DOCUMENTS_REVISION_FILE_MARKED_FOR_MANUAL_JOURNALIZATION` |

**File:** `decompiled_csharp/AulaNative/AulaNative.ViewModels.Document.SecureDocument.Revision/DocumentRevisionItemViewModel.cs`

Lock events render as simple strings (e.g., "Document locked"), while ESDH events render with recipient name and children names:
```
"{ESDH event text} {RecipientName} ({ChildName1}, {ChildName2})"
```

In revision history, a typical sequence for a document that was locked then journalized would look like:
```
Created -> Shared -> Locked -> SentToESDH -> JournalizedToESDH
```

But the app does NOT enforce this ordering. Lock and ESDH events can occur in any order.

## Lock Icon in Document Overview

**File:** `decompiled_csharp/AulaNative.Droid/AulaNative.Droid.Activities.Document.SecureDocuments.Overview/BaseSecureDocumentRecycleViewHolder.cs`

```csharp
((View)LockImageView).Visibility = (ViewStates)((!secureDocumentSimpleVm.IsLocked) ? 8 : 0);
```

A lock icon is shown/hidden based on `IsLocked`. Visibility 0 = VISIBLE, 8 = GONE.

## Summary of Interactions

| Question | Answer |
|----------|--------|
| Is locking automatic upon ESDH submission? | No evidence in app code. Both are independent server states. |
| Can locked documents be re-journalized? | The app does not prevent it. No client-side check linking lock state to journaling actions. |
| What happens if a locked document's ESDH submission fails? | The app handles `EsdhJournalizationFailed` and `ResentToESDH` revision events regardless of lock state. Lock state is not checked. |
| Does locking prevent editing? | Not directly in app code. `AllowToEdit` does not check `IsLocked`. The server likely handles this via the `CanEdit` flag or API-level rejection. |
| Does locking prevent sharing? | Yes -- `ValidateSharingSecureDocuments()` blocks sharing of locked documents with an error dialog. |
| Does locking prevent deletion? | No -- the app does not check `IsLocked` in `AllowToDelete()`. |
| Does locking prevent PDF export? | No -- `AllowToExportDocument` checks `CanExportSecureFiles` permission only. |
| Are lock and journaling filters independent? | They are mutually exclusive in the UI (single-select filter), but map to different API parameters. |
| Who can toggle lock status? | Employees with `CanEditLockedStatus` (server-determined per-document flag). |

## Architectural Notes

The document locking system follows the same "thin client" pattern as the ESDH journaling system:
- The app displays state but does not enforce business rules beyond basic UI gating
- Server-provided flags (`CanEdit`, `CanEditLockedStatus`, `IsLocked`) drive the UI
- The only client-side enforcement is the sharing validation for locked documents
- Both features leave audit trails in the same revision history system
- Both use the same `SecureDocumentDto` as their data carrier

The fact that `AllowToEdit` does not check `IsLocked` suggests the server communicates edit restrictions through the `CanEdit` flag rather than relying on the client to interpret `IsLocked`. This is a reasonable security posture: the server is the authority on what actions are permitted.
