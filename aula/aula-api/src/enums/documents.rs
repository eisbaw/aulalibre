//! Document and file-related enums.

use serde::{Deserialize, Serialize};

/// Category of a secure document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentCategoryEnum {
    Agenda,
    AgendaAllUser,
    PlanOfAction,
    Setting,
    ForCableSchedule,
    Observation,
    EducationalNote,
    Summary,
    SummaryAllUser,
    ScratchScheme,
    OpenTemplate,
    OpenTemplateAllUser,
    Note,
    Unknown,
}

/// Type of document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentTypeEnum {
    Unknown,
    External,
    Internal,
    Note,
    Richdocument,
}

/// Sort field for common files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonFileSortEnum {
    Title,
    UpdatedTime,
}

/// Filter for document lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentFilterEnum {
    All,
    Unread,
    Locked,
    Published,
    PublishInProgress,
    PublishFailed,
}

/// Page type in document overview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DocumentOverviewPageTypeEnum {
    SecureDocument,
    CommonFiles,
    OneDrive,
    GoogleDrive,
}

/// Permission override for implicit sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImplicitSharingPermissionOverride {
    Read,
    Write,
    NoAccess,
}

/// Journaling status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JournalingStatusEnum {
    NotProcessed,
    InProgress,
    Failed,
    Completed,
}

/// Type of change in a document revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RevisionChangeTypeEnum {
    Unshared,
    Shared,
    Unlocked,
    Locked,
    Edited,
    Exported,
    Created,
    PermissionAdded,
    PermissionRemoved,
    ImplicitUnshared,
    Deleted,
    Restored,
    JournalizedToESDH,
    SentToESDH,
    EsdhJournalizationFailed,
    ResentToESDH,
    ManuallyJournalizedToESDH,
    MarkForManualJournalize,
}

/// Export status of a secure document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureDocumentExportStatus {
    Created,
    Processing,
    Failed,
    Completed,
    Unknown,
}

/// Selection mode for secure documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureDocumentSelectingMode {
    Attachment,
    SecureDocumentExport,
    Normal,
    RemoveAssociation,
}

/// Sort field for secure documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureDocumentSortEnum {
    Unknown,
    Title,
    UpdatedAtDate,
}

/// Error type for file attachments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileAttachmentErrorType {
    UnsupportedFileType,
    FileSizeTooBig,
    MediaSizeTooBig,
    GenericError,
}

/// File scanning status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileScanningStatus {
    Available,
    Blocked,
    Processing,
    Bypassed,
}

/// File availability status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileStatusEnum {
    Available,
    Pending,
    Unavailable,
    Unknown,
}

/// Type of file (from common files API).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileTypeEnum {
    Unknown,
    Media,
    File,
    ExternalFile,
}

/// Cloud storage file type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudStorageFileType {
    Folder,
    Docs,
    Sheets,
    Excel,
    Slides,
    PowerPoint,
    Video,
    Sound,
    File,
    Pdf,
    Image,
    Unknown,
}

/// Cloud storage service provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudStorageService {
    Unknown,
    GoogleDrive,
    OneDrive,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roundtrip_test {
        ($name:ident, $ty:ty, $variant:expr) => {
            #[test]
            fn $name() {
                let json = serde_json::to_string(&$variant).unwrap();
                let back: $ty = serde_json::from_str(&json).unwrap();
                assert_eq!(back, $variant);
            }
        };
    }

    roundtrip_test!(
        doc_category,
        DocumentCategoryEnum,
        DocumentCategoryEnum::EducationalNote
    );
    roundtrip_test!(doc_type, DocumentTypeEnum, DocumentTypeEnum::Richdocument);
    roundtrip_test!(
        common_file_sort,
        CommonFileSortEnum,
        CommonFileSortEnum::UpdatedTime
    );
    roundtrip_test!(
        doc_filter,
        DocumentFilterEnum,
        DocumentFilterEnum::PublishInProgress
    );
    roundtrip_test!(
        doc_overview_page,
        DocumentOverviewPageTypeEnum,
        DocumentOverviewPageTypeEnum::OneDrive
    );
    roundtrip_test!(
        implicit_sharing,
        ImplicitSharingPermissionOverride,
        ImplicitSharingPermissionOverride::Write
    );
    roundtrip_test!(
        journaling_status,
        JournalingStatusEnum,
        JournalingStatusEnum::Completed
    );
    roundtrip_test!(
        revision_change,
        RevisionChangeTypeEnum,
        RevisionChangeTypeEnum::JournalizedToESDH
    );
    roundtrip_test!(
        doc_export_status,
        SecureDocumentExportStatus,
        SecureDocumentExportStatus::Processing
    );
    roundtrip_test!(
        doc_selecting_mode,
        SecureDocumentSelectingMode,
        SecureDocumentSelectingMode::Attachment
    );
    roundtrip_test!(
        doc_sort,
        SecureDocumentSortEnum,
        SecureDocumentSortEnum::UpdatedAtDate
    );
    roundtrip_test!(
        file_attach_error,
        FileAttachmentErrorType,
        FileAttachmentErrorType::FileSizeTooBig
    );
    roundtrip_test!(
        file_scanning,
        FileScanningStatus,
        FileScanningStatus::Bypassed
    );
    roundtrip_test!(file_status, FileStatusEnum, FileStatusEnum::Pending);
    roundtrip_test!(file_type, FileTypeEnum, FileTypeEnum::ExternalFile);
    roundtrip_test!(
        cloud_file_type,
        CloudStorageFileType,
        CloudStorageFileType::PowerPoint
    );
    roundtrip_test!(
        cloud_service,
        CloudStorageService,
        CloudStorageService::GoogleDrive
    );
}
