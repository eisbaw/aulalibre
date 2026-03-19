# Magtanvendelsesskema (Use of Force Form) - Document Workflow Analysis

## Overview

"Magtanvendelsesskema" (Use of Force Form) is a legally mandated document type in the Danish
child services system (Serviceloven). In Aula, it is implemented as one category within the
**Secure Documents** subsystem. It does not have its own dedicated workflow, form builder, or
special-purpose code path. Instead, it is a category label applied to generic secure documents.

This analysis traces how the category integrates with document creation, sharing, access control,
ESDH journaling, and the API layer.

## Key Finding: No Special-Purpose Code

The enum member for Magtanvendelsesskema is `DocumentCategoryEnum.ForCableSchedule` -- a
clear machine-translation artifact where the decompiler (or the original obfuscator) mangled
"Magtanvendelsesskema" into "ForCableSchedule." The Danish word "magt" can mean "power/force"
but also loosely maps to cable/strength in some translation contexts.

There is **no specialized form, validation logic, or dedicated API** for Magtanvendelsesskema.
It shares the exact same code path as all other secure document categories (Handleplan,
Observation, Paedagogisk note, etc.).

## Document Category System

### The Enum

**File:** `decompiled_csharp/AulaNative/AulaNative.Enums.Document/DocumentCategoryEnum.cs`

```
DocumentCategoryEnum:
  Agenda              -> "Dagsorden"
  AgendaAllUser       -> "Dagsorden (Alle brugere)"
  PlanOfAction        -> "Handleplan"
  Setting             -> "Indstilling"
  ForCableSchedule    -> "Magtanvendelsesskema"     <-- Use of Force Form
  Observation         -> "Observation"
  EducationalNote     -> "Paedagogisk note"
  Summary             -> "Referat"
  SummaryAllUser      -> "Referat (Alle brugere)"
  ScratchScheme       -> "Rive/kradseskema"          <-- Scratch/Tear Form (related)
  OpenTemplate        -> "Aaben skabelon"
  OpenTemplateAllUser -> "Aaben skabelon (Alle brugere)"
  Note                -> "Note"
  Unknown
```

### Staff-Only Categories

`DocumentCategoryViewModel.OnlyStaffCategories` defines which categories are restricted to
employees (staff). **ForCableSchedule (Magtanvendelsesskema) is in the staff-only list:**

```csharp
// SecureDocumentEditionViewModel.DocumentCategoryViewModel.OnlyStaffCategories
obj.Add(DocumentCategoryEnum.Agenda);
obj.Add(DocumentCategoryEnum.PlanOfAction);
obj.Add(DocumentCategoryEnum.Setting);
obj.Add(DocumentCategoryEnum.ForCableSchedule);   // <-- Magtanvendelsesskema
obj.Add(DocumentCategoryEnum.Observation);
obj.Add(DocumentCategoryEnum.EducationalNote);
obj.Add(DocumentCategoryEnum.Summary);
obj.Add(DocumentCategoryEnum.ScratchScheme);
obj.Add(DocumentCategoryEnum.OpenTemplate);
```

This means parents and children never see the category picker for Magtanvendelsesskema.
They can only view documents shared with them.

### "All Users" vs Standard Categories

Some categories have "(Alle brugere)" variants (Agenda, Summary, OpenTemplate). Notably,
**Magtanvendelsesskema does NOT have an "All users" variant**, which means it can never be
made broadly visible -- it must be explicitly shared with specific recipients.

## Document Creation Flow

### Data Model

**File:** `decompiled_csharp/AulaNative/AulaNative.Models.Document.Arguments/CreateDocumentArguments.cs`

When creating a secure document:

```
CreateDocumentArguments (abstract):
  - Id: long
  - Category: string                          (e.g., "Magtanvendelsesskema")
  - CategoryEnum: DocumentCategoryEnum        (computed from Category string)
  - CreatorInstitutionProfileId: long
  - RegardingInstitutionProfileIds: List<long> (the child/student the doc is about)
  - SharedWithGroups: List<CreateDocumentShareGroupArguments>
  - SharedWithInstitutionProfiles: List<CreateDocumentSharedProfileArguments>
  - Title: string
  - Version: int?
  - DocumentType: string                      (Internal, External, Note, Richdocument)
  - ForceUpdate: bool?
  - AttachedThread: AttachMessagesToSecureDocumentRequest?

CreateInternalDocumentArguments (extends above):
  - Content: string                           (rich text HTML content)
  - AttachmentIds: long[]
  - ImplicitSharingOverrides: List<ImplicitSharingOverride>
```

A Magtanvendelsesskema document is created by setting `Category = "Magtanvendelsesskema"` on
an otherwise standard `CreateInternalDocumentArguments` payload. There is no form schema, no
required fields beyond Title and Content, and no structured data capture for the legal details
(type of restraint, duration, witnesses, etc.).

### Creation API

**Endpoint:** `?method=documents.createInternalSecureDocument`
**URL ID:** 1208

The same endpoint is used for all internal secure documents regardless of category.

### The Document DTO

**File:** `decompiled_csharp/AulaNative/AulaNative.Models.Document/SecureDocumentDto.cs`

The document model returned from the API:

```
SecureDocumentDto:
  - Id, Title, Description, Category, DocumentType
  - IsLocked, CanEdit, CanEditLockedStatus
  - JournalingStatus: JournalingStatusEnum    (not_processed, in_progress, failed, completed)
  - Creator: SecureDocumentCreatorDto
  - CreatedAt, UpdatedAt, UpdatedBy, Version
  - AssociatedInstitutionProfiles             (the children/students it's about)
  - SharedWithGroups, SharedWithInstitutionProfiles
  - ImplicitSharings                          (auto-shared based on role/relationship)
  - IsSharedWithGuardian, IsShareable, ShareableGuardianIds
  - HasMedia, TemplateTitle
```

## Access Control

### Permission Hierarchy

**File:** `decompiled_csharp/AulaNative/AulaNative.Services.Singleton/PermissionManager.cs`

Three tiers of secure document permissions:

| Permission | Enum Value | Description |
|------------|------------|-------------|
| `ACCESS_SECURE_FILESHARING` | 26 | Can access the secure files section at all |
| `HANDLE_SECURE_FILES` | 27 | Can create/edit/manage secure files (full access) |
| `HANDLE_SECURE_FILES_LIMITED` | 28 | Limited handling (exact restrictions unclear from client code) |
| `READ_SECURE_FILES` | 116 | Can read/view secure files |
| `SHARE_SECURE_FILES` | 48 | Can share secure files with others |
| `EXPORT_SECURE_FILES` | 115 | Can export/download secure files as PDF |

### Who Can Create Magtanvendelsesskema

The category picker for Magtanvendelsesskema is gated by:

1. **`PermissionManager.CanHandleSecureFiles()`** -- requires `HANDLE_SECURE_FILES` permission
   AND either (a) the user is not a child, or (b) the child is "stepped up" (age 15+ with
   elevated authentication).
2. **Staff-only category list** -- ForCableSchedule is in `OnlyStaffCategories`, so only
   employees see it in the category picker.

```csharp
public bool CanHandleSecureFiles(string? institutionCode = null)
{
    bool flag = !ProfileManager.Instance.IsChild() || ProfileManager.Instance.Profile.IsSteppedUp;
    if (institutionCode != null)
        return PermissionUtils.HasPermissionOnInstitution(HANDLE_SECURE_FILES, institutionCode) && flag;
    return PermissionUtils.HasPermissionsOnAnyInstitution(HANDLE_SECURE_FILES) && flag;
}
```

### Step-Up Authentication for Viewing

**File:** `decompiled_csharp/AulaNative/AulaNative.Utils/SecureDocumentAccessUtils.cs`

Viewing any secure document requires `ACCESS_SECURE_FILESHARING` permission. If the user does
not have this permission (e.g., a child under 15), a step-up dialog is shown:

- Users 15+ see: "Step-up authentication is needed"
- Users under 15 see: "You cannot access secure files" (blocked entirely)

### Implicit Sharing

Documents have an `ImplicitSharings` list. When a Magtanvendelsesskema is created "regarding"
a child, the system automatically determines who gets implicit access (e.g., the child's
guardians). The `ImplicitSharingPermissionOverride` enum allows overriding the default:

```
ImplicitSharingPermissionOverride:
  Read      - can view
  Write     - can edit
  NoAccess  - explicitly blocked from seeing
```

This is significant for Magtanvendelsesskema because the form is typically about a specific
child, and the system must decide whether the child's parents are automatically notified.

### Sharing Recipients

The sharing UI allows adding both individual profiles and groups:
- **Employee sharing** -- uses `SearchRecipientTypeEnum.Otp | SubGroupEmployee`
- **Non-employee sharing** -- uses `SearchRecipientTypeEnum.Employee | SubGroupEmployee`
- Guardians can be explicitly added as recipients
- Max 300 recipients per document

## ESDH Journaling Integration

### Journaling Status

**File:** `decompiled_csharp/AulaNative/AulaNative.Enums.Document/JournalingStatusEnum.cs`

```
JournalingStatusEnum:
  not_processed  - Default, not yet sent to ESDH
  in_progress    - Submitted, waiting for ESDH confirmation
  failed         - ESDH submission failed
  completed      - Successfully journalized in ESDH
```

### Journaling Flow

The Aula app has **no client-side ESDH submission logic**. The flow is entirely server-driven:

1. Employee creates Magtanvendelsesskema document via `documents.createInternalSecureDocument`
2. Server-side process (admin action or automated policy) initiates ESDH journalization
3. App polls `documents.getSecureDocuments` with `FilterJournalingStatus` to track progress
4. Revision history records the lifecycle events

### Revision History

**File:** `decompiled_csharp/AulaNative/AulaNative.Enums.Document/RevisionChangeTypeEnum.cs`

The revision trail captures the complete document lifecycle, including ESDH events:

```
Created -> Edited -> Shared -> Locked -> SentToESDH -> JournalizedToESDH
                                              |
                                              v (if failure)
                                         EsdhJournalizationFailed -> ResentToESDH
                                              |
                                              v (manual fallback)
                                         MarkForManualJournalize -> ManuallyJournalizedToESDH
```

### Locking Before Journalization

Documents can be locked (`IsLocked`/`CanEditLockedStatus`). The typical flow is:
1. Document is finalized
2. Document is locked (preventing further edits)
3. Document is sent to ESDH
4. ESDH confirms journalization

The revision history tracks both Locked and JournalizedToESDH events.

## API Endpoints

All secure document operations go through the `?method=documents.*` RPC-style API:

| Endpoint | URL ID | Purpose |
|----------|--------|---------|
| `documents.getSecureDocuments` | 1201 | List/filter secure documents |
| `documents.getInternalSecureDocument` | 1202 | Get details of an internal document |
| `documents.getExternalSecureFile` | 1203 | Get details of an external file |
| `documents.createInternalSecureDocument` | 1208 | Create new internal document |
| `documents.updateInternalSecureDocument` | 1209 | Edit existing document |
| `documents.updateSharings` | 1204 | Update sharing recipients |
| `documents.removeOwnSharings` | 1207 | Remove own sharing access |
| `documents.updateLockedStatus` | 1210 | Lock/unlock document |
| `documents.getDocumentRevisions` | 1205 | Get revision history |
| `documents.getDocumentRevision` | 1215 | Get specific revision |
| `documents.getImplicitSharings` | 1215* | Get auto-calculated sharing list |
| `documents.getShareableSecureDocuments` | 1214 | Get documents eligible for sharing |
| `documents.deleteDocument` | 1216 | Soft-delete a document |
| `documents.createArchiveForMultipleSecureDocuments` | 1212 | Batch PDF export |
| `documents.trackCreateArchiveForMultipleSecureDocumentsRequest` | 1213 | Track export progress |
| `messaging.attachMessagesToSecureDocument` | 224 | Attach message thread to document |

*Note: URL ID 1215 is shared between `getDocumentRevision` and `getImplicitSharings` --
likely a decompilation artifact or intentional reuse.

## Workflow Summary

### Creating a Magtanvendelsesskema

1. Employee with `HANDLE_SECURE_FILES` permission opens secure document creation
2. Employee selects category "Magtanvendelsesskema" from the staff-only category picker
3. Employee fills in Title, Content (rich text), and optionally attaches files
4. Employee selects the child(ren) the document concerns (`RegardingInstitutionProfileIds`)
5. System calculates implicit sharings (who auto-gets access based on role)
6. Employee optionally overrides implicit sharing (e.g., block a guardian from seeing it)
7. Employee explicitly adds additional sharing recipients
8. Document is created via `documents.createInternalSecureDocument`

### After Creation

1. Document appears in secure documents list, filtered by category
2. Recipients see it in their secure documents view (with step-up auth if needed)
3. Document can be edited, locked, and revision-tracked
4. Admin/server-side process can initiate ESDH journalization
5. Journalization status is tracked and visible in the app

### Viewing/Filtering

Documents can be filtered by:
- Category (`FilterDocumentCategories` includes `DocumentCategoryEnum.ForCableSchedule`)
- Journaling status
- Associated students
- Groups
- Locked status
- Unread status
- Editable vs read-only

## Related Document Category: Rive/kradseskema (ScratchScheme)

The "Rive/kradseskema" (Scratch/Tear Form) is another category in the same domain -- it
documents incidents where a child scratches, bites, or tears at staff or other children.
Like Magtanvendelsesskema, it is staff-only, uses the same generic document infrastructure,
and has no specialized form logic.

## Limitations of This Analysis

1. **Server-side logic is opaque**: The actual ESDH submission, form validation rules,
   mandatory fields, and journalization triggers are entirely server-side. The mobile app
   is a thin client for this workflow.

2. **No structured form**: The mobile app treats Magtanvendelsesskema as free-text rich
   content. Any structured data capture (restraint type, duration, witnesses, follow-up
   actions) is either enforced server-side, handled via document templates, or managed
   through the web portal rather than the mobile app.

3. **Template system**: The `DocumentTemplateTitle` and `TemplateTitle` fields on
   `SecureDocumentDto` suggest there may be pre-defined templates for different document
   categories. These templates would be server-configured and not visible in the mobile
   client code.

4. **The C# enum name `ForCableSchedule` is a decompiler artifact** and does not reflect
   the actual meaning. The serialized value is always "Magtanvendelsesskema."

## Source Files Referenced

- `AulaNative.Enums.Document/DocumentCategoryEnum.cs`
- `AulaNative.Enums.Document/DocumentCategoryEnumHelpers.cs`
- `AulaNative.Enums.Document/JournalingStatusEnum.cs`
- `AulaNative.Enums.Document/RevisionChangeTypeEnum.cs`
- `AulaNative.Enums.Document/DocumentTypeEnum.cs`
- `AulaNative.Enums.Document/ImplicitSharingPermissionOverride.cs`
- `AulaNative.Models.Document/SecureDocumentDto.cs`
- `AulaNative.Models.Document/ImplicitSharingProfileDto.cs`
- `AulaNative.Models.Document.Arguments/CreateDocumentArguments.cs`
- `AulaNative.Models.Document.Arguments/CreateInternalDocumentArguments.cs`
- `AulaNative.Models.Document.Arguments/GetSecureDocumentsArguments.cs`
- `AulaNative.ViewModels.Document.Edition/SecureDocumentEditionViewModel.cs`
- `AulaNative.ViewModels.Document.Edition/InternalDocumentEditionViewModel.cs`
- `AulaNative.ViewModels.Document.SecureDocument.Sharing/SecureDocumentSharingViewModel.cs`
- `AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentOverviewPageViewModel.cs`
- `AulaNative.Services.Singleton/PermissionManager.cs`
- `AulaNative.Utils/SecureDocumentAccessUtils.cs`
- `AulaNative.Configuration/Urls.cs`
- `AulaNative.ServiceManagers/DocumentServiceManager.cs`
- `AulaNative.Services.Web/DocumentService.cs`
- `AulaNative.Models.Institutions/PermissionEnum.cs`
