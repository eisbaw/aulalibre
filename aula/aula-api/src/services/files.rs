//! File service.
//!
//! Maps to `AulaNative.Services.Web.FileWebService` (8 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `CREATE_DOCUMENT_LINKS` | `files.createDocumentLinks` |
//! | `CREATE_ATTACHMENTS` | `files.createAttachments` |
//! | `UPDATE_ATTACHMENTS` | `files.updateAttachments` |
//! | `COMPLETE_MULTIPART_UPLOADING` | `files.completeMultipartUploading` |
//! | `GET_DOWNLOAD_URL_KEY` | `files.getDownloadUrl` |
//!
//! The following methods operate on **external URLs** (pre-signed S3 URLs
//! or arbitrary download URLs) and are therefore not routed through the
//! Aula API backend:
//!
//! | Method | HTTP | Target |
//! |--------|------|--------|
//! | `upload_file_to_aws` | PUT | Pre-signed AWS S3 URL |
//! | `upload_part_to_aws` | PUT | Pre-signed AWS S3 multipart URL |
//! | `fetch_http_response` | GET | Dynamic URL |
//! | `download_file_with_progress` | GET | Dynamic URL (authenticated) |
//!
//! The local-only methods (`StoreDownloadedFile`, `SaveImageFileWithProgress`)
//! are client-side helpers and have no corresponding API call.

use crate::models::files::{
    CompleteMultipartUploadingRequest, CreateAttachmentsArguments, CreateAttachmentsResult,
    DocumentLinkResult, GetUploadLinksArguments, UploadLink,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions -- Aula API endpoints
// ---------------------------------------------------------------------------

/// Create document links (attach secure documents as references).
///
/// Maps to `FileWebService.CreateDocumentLinks()`.
///
/// # Endpoint
///
/// `POST ?method=files.createDocumentLinks`
pub async fn create_document_links(
    session: &mut Session,
    document_ids: &[i64],
) -> crate::Result<Vec<DocumentLinkResult>> {
    session
        .post("?method=files.createDocumentLinks", &document_ids)
        .await
}

/// Create file/media/link attachments.
///
/// Maps to `FileWebService.CreateAttachments()` (part of the v2 attachment flow).
///
/// # Endpoint
///
/// `POST ?method=files.createAttachments`
pub async fn create_attachments(
    session: &mut Session,
    args: &CreateAttachmentsArguments,
) -> crate::Result<CreateAttachmentsResult> {
    session.post("?method=files.createAttachments", args).await
}

/// Get pre-signed upload links for one or more files.
///
/// Maps to `FileWebService.GetUploadLinks()`.
///
/// # Endpoint
///
/// `POST ?method=files.getDownloadUrl`
pub async fn get_upload_links(
    session: &mut Session,
    args: &GetUploadLinksArguments,
) -> crate::Result<Vec<UploadLink>> {
    session.post("?method=files.getDownloadUrl", args).await
}

/// Complete a multipart upload after all parts have been uploaded to S3.
///
/// Maps to `FileWebService.CompleteMultipartUpload()`.
///
/// # Endpoint
///
/// `POST ?method=files.completeMultipartUploading`
pub async fn complete_multipart_upload(
    session: &mut Session,
    request: &CompleteMultipartUploadingRequest,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=files.completeMultipartUploading", request)
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::files::{
        AttachmentFileUploadArguments, AttachmentMediaFileUploadArguments,
        CompleteMultipartUploadPartRequest, CompleteMultipartUploadingRequest,
        CreateAttachmentsArguments, DocumentLinkResult, GetUploadLinksArguments,
        MultipartUploadingInfoArguments, UploadLink,
    };

    #[test]
    fn document_link_result_deserializes() {
        let json = r#"{"id": 1, "documentId": 42}"#;
        let r: DocumentLinkResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.id, Some(1));
        assert_eq!(r.document_id, Some(42));
    }

    #[test]
    fn get_upload_links_args_serializes() {
        let args = GetUploadLinksArguments {
            upload_names: Some(vec!["photo.jpg".into(), "doc.pdf".into()]),
            institution_code: Some("101001".into()),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(
            json["uploadNames"],
            serde_json::json!(["photo.jpg", "doc.pdf"])
        );
        assert_eq!(json["institutionCode"], "101001");
    }

    #[test]
    fn create_attachments_args_serializes() {
        let args = CreateAttachmentsArguments {
            institution_code: Some("101001".into()),
            owner_institution_profile_id: Some(55),
            media: None,
            links: None,
            files: Some(vec![AttachmentFileUploadArguments {
                upload_id: Some("u1".into()),
                multipart_uploading_info: Some(MultipartUploadingInfoArguments {
                    number_of_part: Some(3),
                }),
                name: Some("report.pdf".into()),
            }]),
            attached_secure_document_ids: Some(vec![10]),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["institutionCode"], "101001");
        assert_eq!(json["ownerInstitutionProfileId"], 55);
        let files = json["files"].as_array().unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0]["name"], "report.pdf");
        assert_eq!(files[0]["uploadId"], "u1");
    }

    #[test]
    fn complete_multipart_request_serializes() {
        let req = CompleteMultipartUploadingRequest {
            file_id: Some(42),
            parts: Some(vec![
                CompleteMultipartUploadPartRequest {
                    e_tag: Some("etag1".into()),
                    part_number: Some("1".into()),
                },
                CompleteMultipartUploadPartRequest {
                    e_tag: Some("etag2".into()),
                    part_number: Some("2".into()),
                },
            ]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["fileId"], 42);
        let parts = json["parts"].as_array().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0]["eTag"], "etag1");
        assert_eq!(parts[1]["partNumber"], "2");
    }

    #[test]
    fn upload_link_deserializes() {
        let json = r#"{
            "action": "https://s3.amazonaws.com/upload",
            "file": {"key": {"key": "files/new.jpg", "bucket": "aula-prod"}},
            "data": {
                "policy": "abc",
                "amzAlgorithm": "AWS4-HMAC-SHA256",
                "amzCredential": "cred",
                "amzDate": "20240115T100000Z",
                "amzSecurityToken": "tok",
                "amzSignature": "sig",
                "acl": "private",
                "key": "files/new.jpg",
                "bucket": "aula-prod",
                "cacheControl": "max-age=31536000"
            }
        }"#;
        let ul: UploadLink = serde_json::from_str(json).unwrap();
        assert_eq!(
            ul.action.as_deref(),
            Some("https://s3.amazonaws.com/upload")
        );
        let data = ul.data.unwrap();
        assert_eq!(data.amz_algorithm.as_deref(), Some("AWS4-HMAC-SHA256"));
        assert_eq!(data.bucket.as_deref(), Some("aula-prod"));
    }

    #[test]
    fn media_attachment_args_serializes() {
        let args = AttachmentMediaFileUploadArguments {
            upload_id: Some("u2".into()),
            multipart_uploading_info: None,
            id: None,
            album_id: Some(5),
            name: Some("photo.jpg".into()),
            media_type: Some("image".into()),
            tags: Some(vec![100, 101]),
            title: Some("Beach photo".into()),
            description: Some("Vacation".into()),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["albumId"], 5);
        assert_eq!(json["mediaType"], "image");
        assert_eq!(json["tags"], serde_json::json!([100, 101]));
        assert_eq!(json["title"], "Beach photo");
    }
}
