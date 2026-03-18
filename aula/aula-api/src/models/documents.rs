//! Document and secure-document models.
//!
//! Types from `Models.Document` covering secure documents, document sharing,
//! revisions, common files, and document creation/query arguments.
//!
//! See `data_models.md` Models.Document namespace.

use serde::{Deserialize, Serialize};

use crate::enums::common::SortOrderEnum;
use crate::enums::documents::{
    CommonFileSortEnum, DocumentCategoryEnum, DocumentTypeEnum, ImplicitSharingPermissionOverride,
    JournalingStatusEnum, RevisionChangeTypeEnum, SecureDocumentExportStatus,
    SecureDocumentSortEnum,
};
use crate::enums::profiles::PortalRole;

use super::files::{AulaFileResultDto, MembershipCountResultModel};
use super::messaging::{AttachMessagesToSecureDocumentRequest, RichTextWrapperDto};

// ---------------------------------------------------------------------------
// Sharing and association types
// ---------------------------------------------------------------------------

/// Creator of a secure document.
///
/// Maps to `Models.Document.SecureDocumentCreatorDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentCreatorDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub alias: bool,
    pub metadata: Option<String>,
}

/// Institution profile associated with a secure document.
///
/// Maps to `Models.Document.SecureDocumentAssociateInstitutionProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentAssociateInstitutionProfileDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub alias: bool,
    pub metadata: Option<String>,
}

/// Group associated with a secure document.
///
/// Maps to `Models.Document.SecureDocumentAssociateGroupDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentAssociateGroupDto {
    pub id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub main_group: bool,
    pub membership_count: Option<MembershipCountResultModel>,
}

/// Group sharing on a secure document (with edit permission).
///
/// Maps to `Models.Document.SecureDocumentShareWithGroupDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentShareWithGroupDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub allow_members_to_be_shown: bool,
    pub membership_count: Option<MembershipCountResultModel>,
}

/// Institution profile sharing on a secure document.
///
/// Maps to `Models.Document.SecureDocumentShareWithInstitutionProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentShareWithInstitutionProfileDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_name: Option<String>,
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub alias: bool,
    pub metadata: Option<String>,
    pub role: Option<PortalRole>,
}

/// Implicit sharing profile entry.
///
/// Maps to `Models.Document.ImplicitSharingProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitSharingProfileDto {
    pub simple_institution_profile_dto: Option<SecureDocumentShareWithInstitutionProfileDto>,
    pub permission_override_enum: Option<ImplicitSharingPermissionOverride>,
}

/// Implicit sharing override for a specific profile.
///
/// Maps to `Models.Document.ImplicitSharingOverride`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitSharingOverride {
    pub institution_profile_id: Option<i64>,
    pub permission_override_enum: Option<ImplicitSharingPermissionOverride>,
}

// ---------------------------------------------------------------------------
// Secure document DTOs
// ---------------------------------------------------------------------------

/// Core secure document entity.
///
/// Maps to `Models.Document.SecureDocumentDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentDto {
    pub id: Option<i64>,
    #[serde(default)]
    pub has_media: bool,
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub can_edit_locked_status: bool,
    #[serde(default)]
    pub is_locked: bool,
    pub journaling_status: Option<JournalingStatusEnum>,
    pub category: Option<String>,
    pub document_template_title: Option<String>,
    pub institution_code: Option<String>,
    pub document_type: Option<String>,
    pub associated_institution_profiles: Option<Vec<SecureDocumentAssociateInstitutionProfileDto>>,
    pub shared_with_groups: Option<Vec<SecureDocumentShareWithGroupDto>>,
    pub shared_with_institution_profiles: Option<Vec<SecureDocumentShareWithInstitutionProfileDto>>,
    pub implicit_sharings: Option<Vec<ImplicitSharingProfileDto>>,
    pub creator: Option<SecureDocumentCreatorDto>,
    pub created_at: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
    pub version: Option<i32>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_shared_with_guardian: bool,
    #[serde(default)]
    pub is_shareable: bool,
    pub shareable_guardian_ids: Option<Vec<i64>>,
    pub template_title: Option<String>,
}

/// External secure document with attachment.
///
/// Maps to `Models.Document.ExternalSecureDocumentDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalSecureDocumentDetailsDto {
    // Base fields from SecureDocumentDto
    pub id: Option<i64>,
    #[serde(default)]
    pub has_media: bool,
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub can_edit_locked_status: bool,
    #[serde(default)]
    pub is_locked: bool,
    pub journaling_status: Option<JournalingStatusEnum>,
    pub category: Option<String>,
    pub document_template_title: Option<String>,
    pub institution_code: Option<String>,
    pub document_type: Option<String>,
    pub associated_institution_profiles: Option<Vec<SecureDocumentAssociateInstitutionProfileDto>>,
    pub shared_with_groups: Option<Vec<SecureDocumentShareWithGroupDto>>,
    pub shared_with_institution_profiles: Option<Vec<SecureDocumentShareWithInstitutionProfileDto>>,
    pub implicit_sharings: Option<Vec<ImplicitSharingProfileDto>>,
    pub creator: Option<SecureDocumentCreatorDto>,
    pub created_at: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
    pub version: Option<i32>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_shared_with_guardian: bool,
    #[serde(default)]
    pub is_shareable: bool,
    pub shareable_guardian_ids: Option<Vec<i64>>,
    pub template_title: Option<String>,
    // Extension
    pub attachment: Option<AulaFileResultDto>,
}

/// Internal secure document with rich content and attachments.
///
/// Maps to `Models.Document.InternalSecureDocumentDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalSecureDocumentDetailsDto {
    // Base fields from SecureDocumentDto
    pub id: Option<i64>,
    #[serde(default)]
    pub has_media: bool,
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub can_edit_locked_status: bool,
    #[serde(default)]
    pub is_locked: bool,
    pub journaling_status: Option<JournalingStatusEnum>,
    pub category: Option<String>,
    pub document_template_title: Option<String>,
    pub institution_code: Option<String>,
    pub document_type: Option<String>,
    pub associated_institution_profiles: Option<Vec<SecureDocumentAssociateInstitutionProfileDto>>,
    pub shared_with_groups: Option<Vec<SecureDocumentShareWithGroupDto>>,
    pub shared_with_institution_profiles: Option<Vec<SecureDocumentShareWithInstitutionProfileDto>>,
    pub implicit_sharings: Option<Vec<ImplicitSharingProfileDto>>,
    pub creator: Option<SecureDocumentCreatorDto>,
    pub created_at: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
    pub version: Option<i32>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_shared_with_guardian: bool,
    #[serde(default)]
    pub is_shareable: bool,
    pub shareable_guardian_ids: Option<Vec<i64>>,
    pub template_title: Option<String>,
    // Extension
    pub attachments: Option<Vec<AulaFileResultDto>>,
    pub content: Option<RichTextWrapperDto>,
}

/// Secure document export tracking.
///
/// Maps to `Models.Document.SecureDocumentExportDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureDocumentExportDto {
    pub request_export_job_id: Option<i64>,
    pub status: Option<SecureDocumentExportStatus>,
    pub progress: Option<f32>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Common files
// ---------------------------------------------------------------------------

/// Institution info on a common file.
///
/// Maps to `Models.Document.CommonFileDto.CommonFileInstitutionDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonFileInstitutionDto {
    pub institution_code: Option<String>,
    pub name: Option<String>,
}

/// Group restriction on a common file.
///
/// Maps to `Models.Document.CommonFileDto.CommonFileGroupRestrictionDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonFileGroupRestrictionDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<String>,
}

/// Common (institution-level) file.
///
/// Maps to `Models.Document.CommonFileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonFileDto {
    pub id: Option<i64>,
    pub attachment: Option<AulaFileResultDto>,
    pub created: Option<String>,
    pub institution: Option<CommonFileInstitutionDto>,
    #[serde(default)]
    pub is_data_policy: bool,
    pub title: Option<String>,
    pub profile_type_restrictions: Option<Vec<PortalRole>>,
    pub group_restrictions: Option<Vec<CommonFileGroupRestrictionDto>>,
}

// ---------------------------------------------------------------------------
// Revisions
// ---------------------------------------------------------------------------

/// Simple institution profile reference (used in revisions).
///
/// Maps to `Models.ProfileModels.StubbedUsers.SimpleInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleInstitutionProfile {
    pub profile_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    pub role: Option<PortalRole>,
    pub main_group: Option<String>,
    pub profile_picture: Option<super::messaging::DownloadFileFromAulaArguments>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
}

/// Document revision entry.
///
/// Maps to `Models.Document.DocumentRevisionDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRevisionDto {
    pub id: Option<i64>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    pub title: Option<String>,
    pub change_type: Option<RevisionChangeTypeEnum>,
    pub shared_with: Option<Vec<SimpleInstitutionProfile>>,
    pub unshared_with: Option<Vec<SimpleInstitutionProfile>>,
    #[serde(default)]
    pub is_available: bool,
    pub recipient_name: Option<String>,
    pub children_names: Option<Vec<String>>,
}

/// Paginated document revision list.
///
/// Maps to `Models.Document.DocumentRevisionPageDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRevisionPageDto {
    pub total_count: Option<i32>,
    pub document_revision_dtos: Option<Vec<DocumentRevisionDto>>,
}

/// Implicit sharings result wrapper.
///
/// Maps to `Models.Document.GetImplicitSharingsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetImplicitSharingsDto {
    pub implicit_sharings: Option<Vec<ImplicitSharingProfileDto>>,
}

// ---------------------------------------------------------------------------
// Document arguments (create, query)
// ---------------------------------------------------------------------------

/// Group sharing argument for document creation.
///
/// Maps to `Models.Document.Arguments.CreateDocumentArguments.CreateDocumentShareGroupArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentShareGroupArguments {
    pub group_id: Option<i64>,
    #[serde(default)]
    pub can_edit: bool,
}

/// Profile sharing argument for document creation.
///
/// Maps to `Models.Document.Arguments.CreateDocumentArguments.CreateDocumentSharedProfileArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentSharedProfileArguments {
    pub institution_profile_id: Option<i64>,
    #[serde(default)]
    pub can_edit: bool,
}

/// Arguments for creating a secure document (base).
///
/// Maps to `Models.Document.Arguments.CreateDocumentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocumentArguments {
    pub id: Option<i64>,
    pub category: Option<String>,
    pub creator_institution_profile_id: Option<i64>,
    pub regarding_institution_profile_ids: Option<Vec<i64>>,
    pub shared_with_groups: Option<Vec<CreateDocumentShareGroupArguments>>,
    pub shared_with_institution_profiles: Option<Vec<CreateDocumentSharedProfileArguments>>,
    pub title: Option<String>,
    pub version: Option<i32>,
    pub force_update: Option<bool>,
    pub attached_thread: Option<AttachMessagesToSecureDocumentRequest>,
}

/// Arguments for creating an external secure document.
///
/// Maps to `Models.Document.Arguments.CreateExternalDocumentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalDocumentArguments {
    // Base from CreateDocumentArguments
    pub id: Option<i64>,
    pub category: Option<String>,
    pub creator_institution_profile_id: Option<i64>,
    pub regarding_institution_profile_ids: Option<Vec<i64>>,
    pub shared_with_groups: Option<Vec<CreateDocumentShareGroupArguments>>,
    pub shared_with_institution_profiles: Option<Vec<CreateDocumentSharedProfileArguments>>,
    pub title: Option<String>,
    pub version: Option<i32>,
    pub force_update: Option<bool>,
    pub attached_thread: Option<AttachMessagesToSecureDocumentRequest>,
    // Extension
    pub external_file: Option<super::files::UploadFileToAulaArguments>,
}

/// Arguments for creating an internal secure document.
///
/// Maps to `Models.Document.Arguments.CreateInternalDocumentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInternalDocumentArguments {
    // Base from CreateDocumentArguments
    pub id: Option<i64>,
    pub category: Option<String>,
    pub creator_institution_profile_id: Option<i64>,
    pub regarding_institution_profile_ids: Option<Vec<i64>>,
    pub shared_with_groups: Option<Vec<CreateDocumentShareGroupArguments>>,
    pub shared_with_institution_profiles: Option<Vec<CreateDocumentSharedProfileArguments>>,
    pub title: Option<String>,
    pub version: Option<i32>,
    pub force_update: Option<bool>,
    pub attached_thread: Option<AttachMessagesToSecureDocumentRequest>,
    // Extension
    pub content: Option<String>,
    pub attachment_ids: Option<Vec<i64>>,
    pub implicit_sharing_overrides: Option<Vec<ImplicitSharingOverride>>,
}

/// Sort model for document queries.
///
/// Maps to `Models.Document.Arguments.SortingModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortingModel {
    pub field: Option<SecureDocumentSortEnum>,
    pub order: Option<SortOrderEnum>,
}

/// Arguments for querying common files.
///
/// Maps to `Models.Document.Arguments.GetCommonFilesArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommonFilesArguments {
    pub page: Option<i32>,
    pub sort_type: Option<CommonFileSortEnum>,
    pub sort_order: Option<SortOrderEnum>,
}

/// Arguments for querying secure documents.
///
/// Maps to `Models.Document.Arguments.GetSecureDocumentsArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSecureDocumentsArguments {
    pub filter_institution_profile_ids: Option<Vec<i64>>,
    pub filter_regarding_group_ids: Option<Vec<i64>>,
    pub filter_unread: Option<bool>,
    pub filter_locked: Option<bool>,
    pub filter_journaling_status: Option<JournalingStatusEnum>,
    #[serde(default)]
    pub filter_editable: bool,
    pub document_type: Option<DocumentTypeEnum>,
    pub sortings: Option<Vec<SortingModel>>,
    pub index: Option<i32>,
    pub limit: Option<i32>,
    pub filter_regarding_student_ids: Option<Vec<i64>>,
    pub filter_document_categories: Option<Vec<DocumentCategoryEnum>>,
}

/// Arguments for querying shareable secure documents.
///
/// Maps to `Models.Document.Arguments.GetShareableSecureDocumentsArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetShareableSecureDocumentsArguments {
    // Base from GetSecureDocumentsArguments
    pub filter_institution_profile_ids: Option<Vec<i64>>,
    pub filter_regarding_group_ids: Option<Vec<i64>>,
    pub filter_unread: Option<bool>,
    pub filter_locked: Option<bool>,
    pub filter_journaling_status: Option<JournalingStatusEnum>,
    #[serde(default)]
    pub filter_editable: bool,
    pub document_type: Option<DocumentTypeEnum>,
    pub sortings: Option<Vec<SortingModel>>,
    pub index: Option<i32>,
    pub limit: Option<i32>,
    pub filter_regarding_student_ids: Option<Vec<i64>>,
    pub filter_document_categories: Option<Vec<DocumentCategoryEnum>>,
    // Extension
    pub share_to_institution_profile_ids: Option<Vec<i64>>,
}

/// Arguments for removing sharing from documents.
///
/// Maps to `Models.Document.Arguments.RemoveSharingArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveSharingArguments {
    pub document_ids: Option<Vec<i64>>,
}

/// Arguments for updating sharing on documents.
///
/// Maps to `Models.Document.Arguments.UpdateSharingArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSharingArguments {
    pub document_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub reset_sharings: bool,
    pub shared_groups: Option<Vec<UpdateSharingGroupArguments>>,
    pub shared_institution_profiles: Option<Vec<UpdateSharingInstProfileArguments>>,
}

/// Group argument for sharing updates.
///
/// Maps to `Models.Document.Arguments.UpdateSharingArguments.UpdateSharingArgumentsGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSharingGroupArguments {
    pub group_id: Option<i64>,
    #[serde(default)]
    pub can_edit: bool,
}

/// Profile argument for sharing updates.
///
/// Maps to `Models.Document.Arguments.UpdateSharingArguments.UpdateSharingArgumentsInstProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSharingInstProfileArguments {
    pub institution_profile_id: Option<i64>,
    #[serde(default)]
    pub can_edit: bool,
}

// ---------------------------------------------------------------------------
// Document requests
// ---------------------------------------------------------------------------

/// Request to export multiple secure documents as a single file.
///
/// Maps to `Models.Document.Requests.CreateExportForMultipleSecureDocumentsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExportForMultipleSecureDocumentsRequest {
    pub secure_document_ids: Option<Vec<i64>>,
}

/// Request to create a PDF for a single document.
///
/// Maps to `Models.Document.Requests.CreatePDFForSingleDocumentRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePdfForSingleDocumentRequest {
    pub secure_document_id: Option<i64>,
}

/// Track a single document PDF export.
///
/// Maps to `Models.Document.Requests.TrackCreatePDFForSingleDocumentRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackCreatePdfForSingleDocumentRequest {
    pub request_id: Option<i64>,
}

/// Track a multi-document export.
///
/// Maps to `Models.Document.Requests.TrackExportForMultipleSecureDocumentsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackExportForMultipleSecureDocumentsRequest {
    pub request_id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Document results
// ---------------------------------------------------------------------------

/// Result of querying common files.
///
/// Maps to `Models.Document.Results.GetCommonFilesResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommonFilesResult {
    pub common_files: Option<Vec<CommonFileDto>>,
    pub total_amount: Option<i32>,
}

/// Profile in secure document filter results.
///
/// Maps to `Models.Document.Results.GetSecureDocumentsResult.GetSecureDocumentsRegardingInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSecureDocumentsRegardingInstitutionProfile {
    pub id: Option<i64>,
    pub name: Option<String>,
}

/// Filter metadata returned with secure document queries.
///
/// Maps to `Models.Document.Results.GetSecureDocumentsResult.GetSecureDocumentsFilter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSecureDocumentsFilter {
    pub regarding_groups: Option<Vec<SecureDocumentAssociateGroupDto>>,
    pub regarding_institution_profiles: Option<Vec<GetSecureDocumentsRegardingInstitutionProfile>>,
    pub document_categories: Option<Vec<String>>,
    pub shared_groups: Option<Vec<SecureDocumentAssociateGroupDto>>,
    pub shared_institution_profiles: Option<Vec<GetSecureDocumentsRegardingInstitutionProfile>>,
}

/// Result of querying secure documents.
///
/// Maps to `Models.Document.Results.GetSecureDocumentsResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSecureDocumentsResult {
    pub documents: Option<Vec<SecureDocumentDto>>,
    pub filters: Option<GetSecureDocumentsFilter>,
    pub total_count: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_secure_document_dto() {
        let json = r#"{
            "id": 42,
            "hasMedia": false,
            "canEdit": true,
            "canEditLockedStatus": false,
            "isLocked": false,
            "journalingStatus": "NotProcessed",
            "category": "Agenda",
            "documentTemplateTitle": null,
            "institutionCode": "101001",
            "documentType": "Internal",
            "associatedInstitutionProfiles": [],
            "sharedWithGroups": [],
            "sharedWithInstitutionProfiles": [],
            "implicitSharings": [],
            "creator": {"id": 55, "name": "Lars Hansen", "alias": false, "metadata": null},
            "createdAt": "2024-01-15T10:00:00",
            "title": "Meeting notes",
            "updatedAt": "2024-01-16T08:00:00",
            "updatedBy": "Lars Hansen",
            "version": 2,
            "description": "Notes from parent meeting",
            "isSharedWithGuardian": true,
            "isShareable": true,
            "shareableGuardianIds": [100, 101],
            "templateTitle": null
        }"#;
        let d: SecureDocumentDto = serde_json::from_str(json).unwrap();
        assert_eq!(d.id, Some(42));
        assert!(d.can_edit);
        assert!(!d.is_locked);
        assert_eq!(
            d.journaling_status,
            Some(JournalingStatusEnum::NotProcessed)
        );
        assert_eq!(d.title.as_deref(), Some("Meeting notes"));
        assert!(d.is_shared_with_guardian);
    }

    #[test]
    fn deserialize_common_file_dto() {
        let json = r#"{
            "id": 10,
            "attachment": null,
            "created": "2024-01-10T00:00:00",
            "institution": {"institutionCode": "101001", "name": "Test Skole"},
            "isDataPolicy": false,
            "title": "School rules",
            "profileTypeRestrictions": ["Guardian", "Employee"],
            "groupRestrictions": [{"id": 5, "name": "3.A", "institutionCode": "101001"}]
        }"#;
        let f: CommonFileDto = serde_json::from_str(json).unwrap();
        assert_eq!(f.id, Some(10));
        assert_eq!(f.title.as_deref(), Some("School rules"));
        assert!(!f.is_data_policy);
        let inst = f.institution.unwrap();
        assert_eq!(inst.name.as_deref(), Some("Test Skole"));
    }

    #[test]
    fn deserialize_document_revision() {
        let json = r#"{
            "id": 1,
            "createdBy": "Lars Hansen",
            "createdAt": "2024-01-15T10:00:00",
            "title": "Meeting notes",
            "changeType": "Created",
            "sharedWith": [],
            "unsharedWith": [],
            "isAvailable": true,
            "recipientName": null,
            "childrenNames": ["Emma"]
        }"#;
        let r: DocumentRevisionDto = serde_json::from_str(json).unwrap();
        assert_eq!(r.id, Some(1));
        assert_eq!(r.change_type, Some(RevisionChangeTypeEnum::Created));
        assert!(r.is_available);
    }

    #[test]
    fn deserialize_get_secure_documents_result() {
        let json = r#"{
            "documents": [],
            "filters": {
                "regardingGroups": [],
                "regardingInstitutionProfiles": [],
                "documentCategories": ["Agenda"],
                "sharedGroups": [],
                "sharedInstitutionProfiles": []
            },
            "totalCount": 0
        }"#;
        let r: GetSecureDocumentsResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_count, Some(0));
        let filters = r.filters.unwrap();
        assert_eq!(filters.document_categories.unwrap(), vec!["Agenda"]);
    }

    #[test]
    fn deserialize_sorting_model() {
        let json = r#"{"field": "Title", "order": "Ascending"}"#;
        let s: SortingModel = serde_json::from_str(json).unwrap();
        assert_eq!(s.field, Some(SecureDocumentSortEnum::Title));
        assert_eq!(s.order, Some(SortOrderEnum::Ascending));
    }

    #[test]
    fn deserialize_secure_document_export() {
        let json = r#"{
            "requestExportJobId": 77,
            "status": "Completed",
            "progress": 1.0,
            "fileUrl": "https://cdn.aula.dk/export/77.pdf",
            "fileName": "export.pdf"
        }"#;
        let e: SecureDocumentExportDto = serde_json::from_str(json).unwrap();
        assert_eq!(e.request_export_job_id, Some(77));
        assert_eq!(e.status, Some(SecureDocumentExportStatus::Completed));
    }
}
