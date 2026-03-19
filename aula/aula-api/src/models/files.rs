//! File and attachment models.
//!
//! Types from `Models.Common.Api.Files` covering file uploads, results,
//! media content, and shared-group references used across posts, gallery,
//! documents, and messaging.
//!
//! See `data_models.md` Models.Common.Api.Files namespace.

use serde::{Deserialize, Serialize};

use crate::enums::documents::{FileScanningStatus, FileStatusEnum};
use crate::enums::gallery::ConversionStatusEnum;
use crate::enums::profiles::PortalRole;

// ---------------------------------------------------------------------------
// Shared value types
// ---------------------------------------------------------------------------

/// Simple object with an id.
///
/// Maps to `DTOs.ObjectWithId`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectWithId {
    pub id: Option<i64>,
}

/// Membership counts by portal role.
///
/// Maps to `DTOs.Group.ResultModel.MembershipCountResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembershipCountResultModel {
    pub employees: Option<i64>,
    pub children: Option<i64>,
    pub guardians: Option<i64>,
    pub total: Option<i64>,
}

/// Group sharing reference with portal roles and membership count.
///
/// Maps to `Models.Common.Api.Files.Result.ShareWithGroupDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShareWithGroupDto {
    pub id: Option<i64>,
    pub portal_roles: Option<Vec<PortalRole>>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub membership_count: Option<MembershipCountResultModel>,
    #[serde(default)]
    pub allow_members_to_be_shown: bool,
}

/// Linked group request (for post/album creation).
///
/// Maps to `DTOs.Group.LinkedGroupRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedGroupRequestModel {
    pub group_id: Option<i64>,
    pub portal_roles_enum: Option<Vec<PortalRole>>,
}

// ---------------------------------------------------------------------------
// File content types
// ---------------------------------------------------------------------------

/// File content stored in S3 (name, URL, bucket, key).
///
/// Maps to `Models.Common.Api.Files.Result.AulaFileContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileContent {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub bucket: Option<String>,
    pub key: Option<String>,
    pub created: Option<String>,
    pub scanning_status: Option<FileScanningStatus>,
}

/// External link content (cloud storage).
///
/// Maps to `Models.Common.Api.Files.Result.AulaLinkContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaLinkContent {
    pub service: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Document link content (reference to a secure document).
///
/// Maps to `Models.Common.Api.Files.Result.AulaDocumentLinkContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaDocumentLinkContent {
    pub id: Option<i64>,
    pub title: Option<String>,
    #[serde(default)]
    pub can_access: bool,
    pub document_type: Option<String>,
    #[serde(default)]
    pub is_deleted: bool,
}

/// Album info embedded in file results.
///
/// Maps to `Models.Common.Api.Files.Result.AulaFileAlbumDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileAlbumDto {
    pub name: Option<String>,
    pub shared_groups: Option<Vec<ShareWithGroupDto>>,
}

/// Profile info on a file result (creator, tag).
///
/// Maps to `Models.Common.Api.Files.Result.AulaFileResultProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileResultProfileDto {
    pub inst_profile_id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub role: Option<String>,
    pub profile_id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub profile_picture: Option<AulaFileContent>,
}

/// Media file content with thumbnails, tags, and permissions.
///
/// Maps to `Models.Common.Api.Files.Result.AulaMediaFileContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaMediaFileContent {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub allows_comments: bool,
    #[serde(default)]
    pub can_view_comments: bool,
    pub file: Option<AulaFileContent>,
    pub media_type: Option<String>,
    pub tags: Option<Vec<AulaFileResultProfileDto>>,
    pub duration_number: Option<f64>,
    pub album: Option<AulaFileAlbumDto>,
    pub thumbnail_url: Option<String>,
    pub large_thumbnail_url: Option<String>,
    pub medium_thumbnail_url: Option<String>,
    pub small_thumbnail_url: Option<String>,
    pub extra_small_thumbnail_url: Option<String>,
    #[serde(default)]
    pub has_video_thumbnail: bool,
    #[serde(default)]
    pub current_user_can_delete: bool,
    #[serde(default)]
    pub current_user_can_edit_metadata: bool,
    #[serde(default)]
    pub current_user_can_report: bool,
    #[serde(default)]
    pub current_user_can_edit_tags: bool,
    #[serde(default)]
    pub is_uploading_pending: bool,
    pub conversion_status: Option<ConversionStatusEnum>,
}

/// Top-level file result DTO (file, media, link, or document).
///
/// Maps to `Models.Common.Api.Files.Result.AulaFileResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileResultDto {
    pub id: Option<i64>,
    pub creator: Option<AulaFileResultProfileDto>,
    pub file: Option<AulaFileContent>,
    pub media: Option<AulaMediaFileContent>,
    pub link: Option<AulaLinkContent>,
    pub document: Option<AulaDocumentLinkContent>,
    pub status: Option<FileStatusEnum>,
}

/// Gallery media file result (extends AulaMediaFileContent with creator).
///
/// Maps to `Models.Common.Api.Files.Result.AulaGalleryMediaFileResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaGalleryMediaFileResultDto {
    // Base fields from AulaMediaFileContent
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub allows_comments: bool,
    #[serde(default)]
    pub can_view_comments: bool,
    pub file: Option<AulaFileContent>,
    pub media_type: Option<String>,
    pub tags: Option<Vec<AulaFileResultProfileDto>>,
    pub duration_number: Option<f64>,
    pub album: Option<AulaFileAlbumDto>,
    pub thumbnail_url: Option<String>,
    pub large_thumbnail_url: Option<String>,
    pub medium_thumbnail_url: Option<String>,
    pub small_thumbnail_url: Option<String>,
    pub extra_small_thumbnail_url: Option<String>,
    #[serde(default)]
    pub has_video_thumbnail: bool,
    #[serde(default)]
    pub current_user_can_delete: bool,
    #[serde(default)]
    pub current_user_can_edit_metadata: bool,
    #[serde(default)]
    pub current_user_can_report: bool,
    #[serde(default)]
    pub current_user_can_edit_tags: bool,
    #[serde(default)]
    pub is_uploading_pending: bool,
    pub conversion_status: Option<ConversionStatusEnum>,
    // Extension fields
    pub creator: Option<AulaFileResultProfileDto>,
    pub id: Option<i64>,
    pub comment_count: Option<i32>,
}

/// Authorized file format (whitelist entry).
///
/// Maps to `Models.Common.Api.Files.Result.AuthorizedFileFormat`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedFileFormat {
    pub id: Option<i64>,
    pub file_format: Option<String>,
    pub name: Option<String>,
}

/// File connection result (download reference).
///
/// Maps to `Models.Common.Api.Files.FileConnectionResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileConnectionResult {
    pub file_name: Option<String>,
    pub mime: Option<String>,
    pub file_path: Option<String>,
    pub length: Option<i64>,
}

// ---------------------------------------------------------------------------
// Upload types
// ---------------------------------------------------------------------------

/// S3 key info for uploads.
///
/// Maps to `Models.Common.Api.Files.Result.UploadFileKeyInfo`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileKeyInfo {
    pub key: Option<String>,
    pub bucket: Option<String>,
}

/// Upload file wrapper.
///
/// Maps to `Models.Common.Api.Files.Result.UploadFileInfo`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileInfo {
    pub key: Option<UploadFileKeyInfo>,
}

/// S3 pre-signed upload data (policy, credentials, signature).
///
/// Maps to `Models.Common.Api.Files.Result.UploadFileData`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileData {
    pub policy: Option<String>,
    /// C# has `[JsonProperty("X-Amz-Algorithm")]`.
    #[serde(rename = "X-Amz-Algorithm")]
    pub amz_algorithm: Option<String>,
    /// C# has `[JsonProperty("X-Amz-Credential")]`.
    #[serde(rename = "X-Amz-Credential")]
    pub amz_credential: Option<String>,
    /// C# has `[JsonProperty("X-Amz-Date")]`.
    #[serde(rename = "X-Amz-Date")]
    pub amz_date: Option<String>,
    /// C# has `[JsonProperty("X-Amz-Security-Token")]`.
    #[serde(rename = "X-Amz-Security-Token")]
    pub amz_security_token: Option<String>,
    /// C# has `[JsonProperty("X-Amz-Signature")]`.
    #[serde(rename = "X-Amz-Signature")]
    pub amz_signature: Option<String>,
    pub acl: Option<String>,
    pub key: Option<String>,
    pub bucket: Option<String>,
    /// C# has `[JsonProperty("Cache-Control")]`.
    #[serde(rename = "Cache-Control")]
    pub cache_control: Option<String>,
}

/// Upload link combining action URL, file info, and S3 data.
///
/// Maps to `Models.Common.Api.Files.Result.UploadLink`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLink {
    pub action: Option<String>,
    pub file: Option<UploadFileInfo>,
    pub data: Option<UploadFileData>,
}

/// Part info for multipart uploads.
///
/// Maps to `Models.Common.Api.Files.Result.FilePartUploadInformation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePartUploadInformation {
    pub part_index: Option<String>,
    pub pre_signed_url: Option<String>,
}

/// Multipart upload info (parts + AWS upload id).
///
/// Maps to `Models.Common.Api.Files.Result.FileUploadInformation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileUploadInformation {
    pub parts: Option<Vec<FilePartUploadInformation>>,
    pub aws_upload_id: Option<String>,
}

/// Base result DTO with an id.
///
/// Maps to `Models.Common.Api.Files.Result.BaseResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResultDto {
    pub id: Option<i64>,
}

/// File result with upload info (extends BaseResultDto).
///
/// Maps to `Models.Common.Api.Files.Parameters.FileResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileResultDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub multipart_file_upload_information: Option<FileUploadInformation>,
    pub upload_id: Option<String>,
}

/// Link result (extends BaseResultDto).
///
/// Maps to `Models.Common.Api.Files.Result.LinkResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkResultDto {
    pub id: Option<i64>,
    pub service: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Document link result.
///
/// Maps to `Models.Common.Api.Files.Result.DocumentLinkResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkResult {
    pub id: Option<i64>,
    pub document_id: Option<i64>,
}

/// Result of creating attachments.
///
/// Maps to `Models.Common.Api.Files.Result.CreateAttachmentsResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttachmentsResult {
    pub media: Option<Vec<FileResultDto>>,
    pub files: Option<Vec<FileResultDto>>,
    pub documents: Option<Vec<FileResultDto>>,
    pub links: Option<Vec<LinkResultDto>>,
    #[serde(default)]
    pub is_all_consents_valid: bool,
}

/// Result of creating media.
///
/// Maps to `Models.Common.Api.Files.Result.CreateMediaResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMediaResult {
    #[serde(default)]
    pub all_images_has_valid_consents: bool,
    pub media: Option<Vec<AulaFileResultDto>>,
}

/// Consent validation result for media tags.
///
/// Maps to `Models.Common.Api.Files.Result.MediaTagConsentsResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaTagConsentsResult {
    #[serde(default)]
    pub is_all_consents_valid: bool,
}

/// Parameters for deleting media.
///
/// Maps to `Models.Common.Api.Files.Result.DeleteMediaParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMediaParameters {
    pub media_ids: Option<Vec<i64>>,
}

// ---------------------------------------------------------------------------
// Upload parameter types
// ---------------------------------------------------------------------------

/// Content arguments for file upload (key info + name + optional id).
///
/// Maps to `Models.Common.Api.Files.Parameters.UploadFileContentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileContentArguments {
    pub key: Option<String>,
    pub bucket: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}

/// Content arguments for media upload.
///
/// Maps to `Models.Common.Api.Files.Parameters.UploadMediaContentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadMediaContentArguments {
    pub duration: Option<f64>,
    pub tags: Option<Vec<ObjectWithId>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media_type: Option<String>,
    pub file: Option<UploadFileContentArguments>,
}

/// Content arguments for link upload.
///
/// Maps to `Models.Common.Api.Files.Parameters.UploadLinkContentArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLinkContentArguments {
    pub external_file_id: Option<String>,
    pub access_token: Option<String>,
    pub service: Option<String>,
}

/// Arguments for uploading a file to Aula.
///
/// Maps to `Models.Common.Api.Files.Parameters.UploadFileToAulaArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileToAulaArguments {
    pub size: Option<f32>,
    pub creator: Option<ObjectWithId>,
    pub file: Option<UploadFileContentArguments>,
    pub media: Option<UploadMediaContentArguments>,
    pub link: Option<UploadLinkContentArguments>,
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub is_loading: bool,
}

/// Arguments for creating attachments.
///
/// Maps to `Models.Common.Api.Files.Parameters.CreateAttachmentsArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAttachmentsArguments {
    pub institution_code: Option<String>,
    pub owner_institution_profile_id: Option<i64>,
    pub media: Option<Vec<AttachmentMediaFileUploadArguments>>,
    pub links: Option<Vec<AttachmentLinkUploadArguments>>,
    pub files: Option<Vec<AttachmentFileUploadArguments>>,
    pub attached_secure_document_ids: Option<Vec<i64>>,
}

/// Get upload link names for pre-signed URLs.
///
/// Maps to `Models.Common.Api.Files.Parameters.GetUploadLinksArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUploadLinksArguments {
    /// C# has `[JsonProperty("upload_names")]` (snake_case, not camelCase).
    #[serde(rename = "upload_names")]
    pub upload_names: Option<Vec<String>>,
    pub institution_code: Option<String>,
}

/// Add or remove a tag on a media item.
///
/// Maps to `Models.Common.Api.Files.Parameters.AddOrRemoveTagArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrRemoveTagArguments {
    pub inst_profile_id: Option<i64>,
    pub media_id: Option<i64>,
}

/// Complete a multipart upload part.
///
/// Maps to `Models.Common.Api.Files.Parameters.CompleteMultipartUploadPartRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMultipartUploadPartRequest {
    pub e_tag: Option<String>,
    pub part_number: Option<String>,
}

/// Complete a multipart upload.
///
/// Maps to `Models.Common.Api.Files.Parameters.CompleteMultipartUploadingRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMultipartUploadingRequest {
    pub file_id: Option<i64>,
    pub parts: Option<Vec<CompleteMultipartUploadPartRequest>>,
}

// ---------------------------------------------------------------------------
// Attachment v2 types
// ---------------------------------------------------------------------------

/// Multipart upload info arguments.
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.MultipartUploadingInfoArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipartUploadingInfoArguments {
    pub number_of_part: Option<i32>,
}

/// Base file upload arguments (upload id + multipart info).
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.BaseFileUploadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseFileUploadArguments {
    pub upload_id: Option<String>,
    pub multipart_uploading_info: Option<MultipartUploadingInfoArguments>,
}

/// File attachment upload arguments.
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.AttachmentFileUploadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentFileUploadArguments {
    pub upload_id: Option<String>,
    pub multipart_uploading_info: Option<MultipartUploadingInfoArguments>,
    pub name: Option<String>,
}

/// Link attachment upload arguments.
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.AttachmentLinkUploadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentLinkUploadArguments {
    pub external_file_id: Option<String>,
    pub access_token: Option<String>,
    pub service: Option<String>,
}

/// Media attachment upload arguments.
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.AttachmentMediaFileUploadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentMediaFileUploadArguments {
    pub upload_id: Option<String>,
    pub multipart_uploading_info: Option<MultipartUploadingInfoArguments>,
    pub id: Option<i64>,
    pub album_id: Option<i64>,
    pub name: Option<String>,
    pub media_type: Option<String>,
    pub tags: Option<Vec<i64>>,
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Update attachment arguments.
///
/// Maps to `Models.Common.Api.Files.Parameters.AttachmentFeatureV2.UpdateAttachmentsArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAttachmentsArguments {
    pub media: Option<Vec<AttachmentMediaFileUploadArguments>>,
}

/// Upload attachment service result.
///
/// Maps to `Models.Files.UploadAttachmentServiceResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadAttachmentServiceResult {
    pub media_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub all_consent_is_valid: bool,
    #[serde(default)]
    pub is_success: bool,
}

/// Common file model (simple file reference with title).
///
/// Maps to `Models.Common.CommonFile.CommonFileModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonFileModel {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub file: Option<super::messaging::DownloadFileFromAulaArguments>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_aula_file_content() {
        let json = r#"{
            "id": 42,
            "name": "photo.jpg",
            "url": "https://cdn.aula.dk/photo.jpg",
            "bucket": "aula-prod",
            "key": "files/42/photo.jpg",
            "created": "2024-01-15T10:00:00",
            "scanningStatus": "Available"
        }"#;
        let f: AulaFileContent = serde_json::from_str(json).unwrap();
        assert_eq!(f.id, Some(42));
        assert_eq!(f.name.as_deref(), Some("photo.jpg"));
        assert_eq!(f.scanning_status, Some(FileScanningStatus::Available));
    }

    #[test]
    fn deserialize_aula_file_result_dto() {
        let json = r#"{
            "id": 100,
            "creator": null,
            "file": {"id": 42, "name": "doc.pdf", "url": null, "bucket": null, "key": null, "created": null, "scanningStatus": null},
            "media": null,
            "link": null,
            "document": null,
            "status": "Available"
        }"#;
        let r: AulaFileResultDto = serde_json::from_str(json).unwrap();
        assert_eq!(r.id, Some(100));
        assert_eq!(r.status, Some(FileStatusEnum::Available));
        assert_eq!(r.file.unwrap().name.as_deref(), Some("doc.pdf"));
    }

    #[test]
    fn deserialize_share_with_group_dto() {
        let json = r#"{
            "id": 5,
            "portalRoles": ["guardian", "employee"],
            "name": "3.A",
            "shortName": "3A",
            "institutionCode": "101001",
            "institutionName": "Test Skole",
            "membershipCount": {"employees": 3, "children": 25, "guardians": 40, "total": 68},
            "allowMembersToBeShown": true
        }"#;
        let s: ShareWithGroupDto = serde_json::from_str(json).unwrap();
        assert_eq!(s.id, Some(5));
        assert!(s.allow_members_to_be_shown);
        let mc = s.membership_count.unwrap();
        assert_eq!(mc.total, Some(68));
    }

    #[test]
    fn deserialize_authorized_file_format() {
        let json = r#"{"id": 1, "fileFormat": ".pdf", "name": "PDF"}"#;
        let f: AuthorizedFileFormat = serde_json::from_str(json).unwrap();
        assert_eq!(f.file_format.as_deref(), Some(".pdf"));
    }

    #[test]
    fn deserialize_upload_link() {
        let json = r#"{
            "action": "https://s3.amazonaws.com/upload",
            "file": {"key": {"key": "files/new.jpg", "bucket": "aula-prod"}},
            "data": {
                "policy": "abc",
                "X-Amz-Algorithm": "AWS4-HMAC-SHA256",
                "X-Amz-Credential": "cred",
                "X-Amz-Date": "20240115T100000Z",
                "X-Amz-Security-Token": "tok",
                "X-Amz-Signature": "sig",
                "acl": "private",
                "key": "files/new.jpg",
                "bucket": "aula-prod",
                "Cache-Control": "max-age=31536000"
            }
        }"#;
        let ul: UploadLink = serde_json::from_str(json).unwrap();
        assert_eq!(
            ul.action.as_deref(),
            Some("https://s3.amazonaws.com/upload")
        );
        let data = ul.data.unwrap();
        assert_eq!(data.amz_algorithm.as_deref(), Some("AWS4-HMAC-SHA256"));
    }

    #[test]
    fn deserialize_file_connection_result() {
        let json = r#"{
            "fileName": "report.pdf",
            "mime": "application/pdf",
            "filePath": "/tmp/report.pdf",
            "length": 102400
        }"#;
        let f: FileConnectionResult = serde_json::from_str(json).unwrap();
        assert_eq!(f.file_name.as_deref(), Some("report.pdf"));
        assert_eq!(f.length, Some(102400));
    }

    #[test]
    fn deserialize_create_attachments_result() {
        let json = r#"{
            "media": [],
            "files": [{"id": 1, "name": "doc.pdf", "multipartFileUploadInformation": null, "uploadId": "u1"}],
            "documents": [],
            "links": [],
            "isAllConsentsValid": true
        }"#;
        let r: CreateAttachmentsResult = serde_json::from_str(json).unwrap();
        assert!(r.is_all_consents_valid);
        assert_eq!(r.files.unwrap().len(), 1);
    }
}
