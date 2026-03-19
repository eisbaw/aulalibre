# ESDH Integration and Journaling Flow Analysis

Investigation of Aula's integration with ESDH (Elektronisk Sags- og Dokumenthandtering) -- the Danish municipal electronic case and document management system. This analysis traces how secure documents flow from Aula to municipal ESDH systems.

## Overview

Aula's secure documents module supports journaling (Danish: journalisering) -- the process of exporting child-related documents to official municipal ESDH records. This is a server-driven workflow: the app does **not** initiate ESDH transfers directly. Instead, the backend manages the actual ESDH submission, and the app tracks status through polling and revision history.

Key finding: **The app is a thin client for ESDH.** It displays journaling status, filters by journaling state, and shows ESDH events in revision history, but the actual journaling logic lives entirely server-side.

## Journaling State Machine

### JournalingStatusEnum

Each `SecureDocumentDto` carries a `JournalingStatus` field with one of four values:

| Status | Wire value | Meaning |
|--------|-----------|---------|
| `NotProcessed` | `"not_processed"` | Default. Document has not been submitted to ESDH |
| `InProgress` | `"in_progress"` | Document is being processed by ESDH system |
| `Failed` | `"failed"` | ESDH journalization failed |
| `Completed` | `"completed"` | Successfully journalized to ESDH |

The unknown/fallback value is `NotProcessed` (set via `DefaultUnknownEnumConverter`).

### Revision Change Types (ESDH-specific)

The `RevisionChangeTypeEnum` includes six ESDH-related events that appear in document revision history:

| Event | Wire value | Description |
|-------|-----------|-------------|
| `SentToESDH` | `"sent_to_esdh"` | Document was submitted to ESDH |
| `JournalizedToESDH` | `"journalized_to_esdh"` | Successfully journalized in ESDH |
| `EsdhJournalizationFailed` | `"esdh_journalization_failed"` | Journalization attempt failed |
| `ResentToESDH` | `"resent_to_esdh"` | Document was re-submitted after failure |
| `ManuallyJournalizedToESDH` | `"manually_journalized"` | Marked as journalized by manual intervention |
| `MarkForManualJournalize` | `"mark_for_manual_journalize"` | Flagged for manual journalization |

### Inferred State Transitions

```
NotProcessed
    |
    v  (triggered server-side)
SentToESDH --> InProgress
    |
    +---> JournalizedToESDH --> Completed
    |
    +---> EsdhJournalizationFailed --> Failed
              |
              +---> ResentToESDH --> InProgress (retry)
              |
              +---> MarkForManualJournalize
                        |
                        +---> ManuallyJournalizedToESDH --> Completed
```

The state machine supports both automatic retry (resend) and manual fallback paths.

## Permission Model

### JOURNALING_TO_ESDH Permission

ESDH functionality is gated by a per-institution permission:

- **Permission ID**: `PermissionEnum.JOURNALING_TO_ESDH = 132`
- **Scope**: Per institution (checked against `institution.Permissions`)
- **Check**: `AllSelectedInstitutionsAllowedToJournalize` -- returns true if **any** of the user's currently selected institutions has the `JOURNALING_TO_ESDH` permission

This permission controls whether journaling-related filter options appear in the document overview UI.

### Related Permissions

| Permission | ID | Role |
|-----------|-----|------|
| `READ_SECURE_FILES` | 116 | View secure documents |
| `EXPORT_SECURE_FILES` | 115 | Export/download secure documents as PDF |
| `JOURNALING_TO_ESDH` | 132 | Access journaling status filters |

The `EXPORT_SECURE_FILES` permission controls whether an employee can export documents to PDF (via `CanExportSecureFiles(institutionCode)`). This is distinct from ESDH journaling -- PDF export is local download, journaling is submission to municipal systems.

## API Endpoints

### Document Retrieval (with Journaling Filter)

**`documents.getSecureDocuments`** (URL ID 1201)

The `GetSecureDocumentsArguments` request model includes:
- `FilterJournalingStatus: JournalingStatusEnum?` -- Optional filter for journaling status
- Used by the overview page to filter documents by their ESDH state

### Document Export (PDF Generation, Not ESDH)

These endpoints handle local PDF export, not ESDH submission:

| Endpoint | URL ID | Purpose |
|----------|--------|---------|
| `documents.createPDFForSingleDocument` | 1217 | Generate PDF for one document |
| `documents.trackCreatePDFForSingleDocument` | 1218 | Poll PDF generation status |
| `documents.createArchiveForMultipleSecureDocuments` | 1212 | Create ZIP archive of multiple documents |
| `documents.trackCreateArchiveForMultipleSecureDocumentsRequest` | 1213 | Poll archive creation status |
| `documents.getMaxDocumentsPerExport` | 1219 | Query export batch size limit |

### Document Revision History

**`documents.getDocumentRevisions`** (URL ID 1205)

Returns `DocumentRevisionPageDto` containing `DocumentRevisionDto` entries. Each revision has a `ChangeType` which can be any of the ESDH-related `RevisionChangeTypeEnum` values. This is the primary mechanism for users to see ESDH journaling activity.

### Message-to-Document Attachment

**`messages.attachMessagesToSecureDocument`** (URL ID 224)

Request model `AttachMessagesToSecureDocumentRequest`:
- `SecureDocumentId: long?`
- `MessageIds: string[]?`
- `ThreadId: long`
- `CommonInboxId: long?`

This allows attaching message threads to secure documents. Messages attached to journalized documents would implicitly become part of the ESDH record.

## Data That Leaves Aula During Journaling

The `DocumentRevisionDto` reveals what data is associated with ESDH events:

### Fields on ESDH revision entries

| Field | Type | Description |
|-------|------|-------------|
| `RecipientName` | `string` | Name of the ESDH recipient (likely the municipality case handler) |
| `ChildrenNames` | `string[]` | Names of children associated with the document |
| `CreatedBy` | `string` | Who initiated the action |
| `CreatedAt` | `DateTime` | When the action occurred |

The `BuildEsdhRevisionHistoryString` method formats ESDH events as:
```
"{action text} {RecipientName} ({ChildrenName1}, {ChildrenName2}, ...)"
```

### Document content sent to ESDH (inferred)

Based on the `SecureDocumentDto` and `InternalSecureDocumentDetailsDto` models, the following data is available for journaling:

- **Document metadata**: Title, Category, Description, InstitutionCode, DocumentType
- **Document content**: `RichTextWrapperDto Content` (for internal documents)
- **Attachments**: `AulaFileResultDto[] Attachments`
- **Associated profiles**: List of `SecureDocumentAssociateInstitutionProfileDto` (children/students the document concerns)
- **Sharing information**: Who the document is shared with (groups and institution profiles)
- **Creator info**: Name, metadata, alias of document creator
- **Version info**: Version number, update timestamps

The actual data payload sent to the ESDH system is constructed server-side and not visible in the app code.

## UI Integration

### Document Overview Filtering

When `AllSelectedInstitutionsAllowedToJournalize` is true, three additional filter options appear:

| Filter Enum | Maps to JournalingStatus | Danish UI label (inferred) |
|-------------|-------------------------|---------------------------|
| `Published` | `Completed` | Published/Journalized |
| `PublishFailed` | `Failed` | Publication failed |
| `PublishInProgress` | `InProgress` | Publication in progress |

The naming convention "Published" in the filter enum is interesting -- it suggests journaling is presented to users as "publishing" rather than using the more technical "journalizing" terminology.

### Document Overview Display

Each `SecureDocumentSimpleViewModel` carries a `JournalingStatus` property that is displayed alongside the document in the overview list. This allows employees to see at a glance which documents have been journalized.

### Revision History Display

The revision history page shows ESDH events with localized strings:

| Translation Key | Context |
|-----------------|---------|
| `DOCUMENTS_REVISION_FILE_JOURNALIZED_TO_ESDH` | Successful journalization |
| `DOCUMENTS_REVISION_FILE_SENT_TO_ESDH` | Initial submission |
| `DOCUMENTS_REVISION_FILE_ESDH_JOURNALIZATION_FAILED` | Failed attempt |
| `DOCUMENTS_REVISION_FILE_RESENT_TO_ESDH` | Retry after failure |
| `DOCUMENTS_REVISION_FILE_MANUALLY_JOURNALIZED_TO_ESDH` | Manual override |
| `DOCUMENTS_REVISION_FILE_MARKED_FOR_MANUAL_JOURNALIZATION` | Flagged for manual |

### Document Categories

Documents are categorized using `DocumentCategoryEnum` values that map to Danish municipal document types:

| Category | Danish Name | English Translation |
|----------|-------------|---------------------|
| Agenda | Dagsorden | Agenda |
| AgendaAllUser | Dagsorden (Alle brugere) | Agenda (All users) |
| PlanOfAction | Handleplan | Action Plan |
| Setting | Indstilling | Recommendation/Setting |
| ForCableSchedule | Magtanvendelsesskema | Use of Force Form |
| Observation | Observation | Observation |
| EducationalNote | Paedagogisk note | Pedagogical Note |
| Summary | Referat | Minutes |
| SummaryAllUser | Referat (Alle brugere) | Minutes (All users) |
| ScratchScheme | Rive/kradseskema | Scratch/Tear Form |
| OpenTemplate | Aaben skabelon | Open Template |
| OpenTemplateAllUser | Aaben skabelon (Alle brugere) | Open Template (All users) |
| Note | Note | Note |

These categories are significant because they determine how the document is classified in the ESDH system. Categories like "Magtanvendelsesskema" (Use of Force Form) and "Handleplan" (Action Plan) are formal municipal document types with legal significance in Danish child services.

## Architectural Observations

### Server-Driven Design

The app has no ESDH client code. There are no API endpoints for "submit to ESDH" or "initiate journaling." The flow is:

1. Employee creates/edits a secure document in Aula
2. Some server-side trigger (likely admin action or automated policy) initiates ESDH journalization
3. The app polls `getSecureDocuments` with `FilterJournalingStatus` to track progress
4. Revision history shows the full ESDH lifecycle via `getDocumentRevisions`

### Locking and Journaling Interaction

Documents can be locked (`IsLocked`/`CanEditLockedStatus`). Once a document is journalized to ESDH, it is likely auto-locked to maintain record integrity. The revision history tracks both `Locked` and `JournalizedToESDH` events, suggesting these are sequential.

### Export vs Journaling

The codebase distinguishes between:
- **Export** (`SecureDocumentExportStatus`): Local PDF/archive download for the user
- **Journaling** (`JournalingStatusEnum`): Submission to municipal ESDH system

These are independent workflows with different permission checks, status tracking, and UI paths.

## Source Files Referenced

### Enums
- `AulaNative.Enums.Document/JournalingStatusEnum.cs`
- `AulaNative.Enums.Document/RevisionChangeTypeEnum.cs`
- `AulaNative.Enums.Document/DocumentCategoryEnum.cs`
- `AulaNative.Enums.Document/SecureDocumentExportStatus.cs`
- `AulaNative.Models.Institutions/PermissionEnum.cs`

### Data Models
- `AulaNative.Models.Document/SecureDocumentDto.cs`
- `AulaNative.Models.Document/DocumentRevisionDto.cs`
- `AulaNative.Models.Document/DocumentRevisionPageDto.cs`
- `AulaNative.Models.Document/InternalSecureDocumentDetailsDto.cs`
- `AulaNative.Models.Document/SecureDocumentExportDto.cs`
- `AulaNative.Models.Document.Arguments/GetSecureDocumentsArguments.cs`
- `AulaNative.Models.Messages.AttachMessagesToSecureDocument/AttachMessagesToSecureDocumentRequest.cs`

### ViewModels
- `AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentOverviewPageViewModel.cs`
- `AulaNative.ViewModels.Document.SecureDocument.Overview/SecureDocumentSimpleViewModel.cs`
- `AulaNative.ViewModels.Document.SecureDocument.Revision/DocumentRevisionItemViewModel.cs`
- `AulaNative.ViewModels.Document.SecureDocument.Revision/DocumentRevisionPageViewModel.cs`

### Services
- `AulaNative.Services.Web/DocumentService.cs`
- `AulaNative.Configuration/Urls.cs`
