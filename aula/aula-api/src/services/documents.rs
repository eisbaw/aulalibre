//! Secure document service.
//!
//! Maps to `AulaNative.Services.Web.SecureDocumentWebService` (18+ methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.11.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_secure_documents` | GET | `/documents/secure` |
//! | `get_common_files` | GET | `/documents/common` |
//! | `update_sharings` | PUT | `/documents/{id}/sharings` |
//! | `remove_own_sharings` | DELETE | `/documents/{id}/sharings/own` |
//! | `get_implicit_sharings` | GET | `/documents/{id}/implicitSharings` |
//! | `get_document_revisions` | GET | `/documents/{id}/revisions` |
//! | `get_external_document_details` | GET | `/documents/external/{id}` |
//! | `get_external_secure_document_revision` | GET | `/documents/external/{id}/revision` |
//! | `get_internal_document_details` | GET | `/documents/internal/{id}` |
//! | `get_internal_secure_document_revision` | GET | `/documents/internal/{id}/revision` |
//! | `create_internal_secure_document` | POST | `/documents/internal` |
//! | `update_internal_secure_document` | PUT | `/documents/internal/{id}` |
//! | `update_document_locked_status` | PUT | `/documents/{id}/locked` |
//! | `soft_delete_secure_document` | DELETE | `/documents/{id}` |
//! | `get_shareable_secure_documents` | GET | `/documents/shareable` |
//! | `get_max_documents_per_export` | GET | `/documents/export/maxCount` |
//! | `create_export_for_multiple` | POST | `/documents/export` |
//! | `track_export` | GET | `/documents/export/{id}/status` |
//! | `create_pdf_for_single` | POST | `/documents/{id}/pdf` |
//! | `track_create_pdf` | GET | `/documents/{id}/pdf/status` |

use serde::{Deserialize, Serialize};

use crate::models::documents::{
    CommonFileDto, CreateExportForMultipleSecureDocumentsRequest, CreateInternalDocumentArguments,
    DocumentRevisionPageDto, ExternalSecureDocumentDetailsDto, GetCommonFilesArguments,
    GetImplicitSharingsDto, GetSecureDocumentsArguments, GetSecureDocumentsResult,
    GetShareableSecureDocumentsArguments, InternalSecureDocumentDetailsDto, RemoveSharingArguments,
    SecureDocumentExportDto, UpdateSharingArguments,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Request types specific to this service
// ---------------------------------------------------------------------------

/// Request body for updating a document's locked status.
///
/// Maps to `AulaNative.Services.Web.UpdateDocumentStatusRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentLockedStatusRequest {
    /// Whether the document should be locked.
    pub is_locked: bool,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch secure documents matching the given filter arguments.
///
/// Maps to `SecureDocumentWebService.GetSecureDocuments()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/secure?<query params>`
pub async fn get_secure_documents(
    session: &mut Session,
    args: &GetSecureDocumentsArguments,
) -> crate::Result<GetSecureDocumentsResult> {
    let mut query = Vec::new();
    if let Some(ref ids) = args.filter_institution_profile_ids {
        for id in ids {
            query.push(format!("filterInstitutionProfileIds={id}"));
        }
    }
    if let Some(ref ids) = args.filter_regarding_group_ids {
        for id in ids {
            query.push(format!("filterRegardingGroupIds={id}"));
        }
    }
    if let Some(unread) = args.filter_unread {
        query.push(format!("filterUnread={unread}"));
    }
    if let Some(locked) = args.filter_locked {
        query.push(format!("filterLocked={locked}"));
    }
    if let Some(ref status) = args.filter_journaling_status {
        let s = serde_json::to_string(status)
            .unwrap_or_default()
            .trim_matches('"')
            .to_string();
        query.push(format!("filterJournalingStatus={s}"));
    }
    if args.filter_editable {
        query.push("filterEditable=true".to_string());
    }
    if let Some(ref dt) = args.document_type {
        let s = serde_json::to_string(dt)
            .unwrap_or_default()
            .trim_matches('"')
            .to_string();
        query.push(format!("documentType={s}"));
    }
    if let Some(index) = args.index {
        query.push(format!("index={index}"));
    }
    if let Some(limit) = args.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "documents/secure".to_string()
    } else {
        format!("documents/secure?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Fetch common (institution-level) files.
///
/// Maps to `SecureDocumentWebService.GetCommonFiles()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/common?<query params>`
pub async fn get_common_files(
    session: &mut Session,
    args: &GetCommonFilesArguments,
) -> crate::Result<Vec<CommonFileDto>> {
    let mut query = Vec::new();
    if let Some(page) = args.page {
        query.push(format!("page={page}"));
    }
    if let Some(ref sort_type) = args.sort_type {
        let s = serde_json::to_string(sort_type)
            .unwrap_or_default()
            .trim_matches('"')
            .to_string();
        query.push(format!("sortType={s}"));
    }
    if let Some(ref sort_order) = args.sort_order {
        let s = serde_json::to_string(sort_order)
            .unwrap_or_default()
            .trim_matches('"')
            .to_string();
        query.push(format!("sortOrder={s}"));
    }

    let path = if query.is_empty() {
        "documents/common".to_string()
    } else {
        format!("documents/common?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Update sharings on one or more secure documents.
///
/// Maps to `SecureDocumentWebService.UpdateSharings()`.
///
/// # Endpoint (inferred)
///
/// `PUT /documents/sharings`
///
/// NOTE: The decompiled method name suggests per-document (`/documents/{id}/sharings`),
/// but the `UpdateSharingArguments` contains `document_ids` (plural), indicating a
/// bulk operation. We use the bulk endpoint path.
pub async fn update_sharings(
    session: &mut Session,
    args: &UpdateSharingArguments,
) -> crate::Result<serde_json::Value> {
    session.put("documents/sharings", args).await
}

/// Remove the current user's own sharings from documents.
///
/// Maps to `SecureDocumentWebService.RemoveOwnSharings()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /documents/sharings/own`
///
/// NOTE: Uses `delete_with_body` because the `RemoveSharingArguments` contains
/// `document_ids` identifying which documents to remove sharing from.
pub async fn remove_own_sharings(
    session: &mut Session,
    args: &RemoveSharingArguments,
) -> crate::Result<serde_json::Value> {
    session
        .delete_with_body("documents/sharings/own", args)
        .await
}

/// Get implicit sharings for a document.
///
/// Maps to `SecureDocumentWebService.GetImplicitSharings()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/{id}/implicitSharings`
pub async fn get_implicit_sharings(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<GetImplicitSharingsDto> {
    session
        .get(&format!("documents/{document_id}/implicitSharings"))
        .await
}

/// Get revision history for a document.
///
/// Maps to `SecureDocumentWebService.GetDocumentRevisions()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/{id}/revisions?page=...`
pub async fn get_document_revisions(
    session: &mut Session,
    document_id: i64,
    page: Option<i32>,
) -> crate::Result<DocumentRevisionPageDto> {
    let path = match page {
        Some(p) => format!("documents/{document_id}/revisions?page={p}"),
        None => format!("documents/{document_id}/revisions"),
    };
    session.get(&path).await
}

/// Get details of an external secure document.
///
/// Maps to `SecureDocumentWebService.GetExternalDocumentDetails()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/external/{id}`
pub async fn get_external_document_details(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<ExternalSecureDocumentDetailsDto> {
    session
        .get(&format!("documents/external/{document_id}"))
        .await
}

/// Get a specific revision of an external secure document.
///
/// Maps to `SecureDocumentWebService.GetExternalSecureDocumentRevision()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/external/{id}/revision`
pub async fn get_external_document_revision(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<ExternalSecureDocumentDetailsDto> {
    session
        .get(&format!("documents/external/{document_id}/revision"))
        .await
}

/// Get details of an internal secure document.
///
/// Maps to `SecureDocumentWebService.GetInternalDocumentDetails()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/internal/{id}`
pub async fn get_internal_document_details(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<InternalSecureDocumentDetailsDto> {
    session
        .get(&format!("documents/internal/{document_id}"))
        .await
}

/// Get a specific revision of an internal secure document.
///
/// Maps to `SecureDocumentWebService.GetInternalSecureDocumentRevision()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/internal/{id}/revision`
pub async fn get_internal_document_revision(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<InternalSecureDocumentDetailsDto> {
    session
        .get(&format!("documents/internal/{document_id}/revision"))
        .await
}

/// Create a new internal secure document.
///
/// Maps to `SecureDocumentWebService.CreateInternalSecureDocument()`.
///
/// # Endpoint (inferred)
///
/// `POST /documents/internal`
pub async fn create_internal_secure_document(
    session: &mut Session,
    args: &CreateInternalDocumentArguments,
) -> crate::Result<serde_json::Value> {
    session.post("documents/internal", args).await
}

/// Update an existing internal secure document.
///
/// Maps to `SecureDocumentWebService.UpdateInternalSecureDocument()`.
///
/// # Endpoint (inferred)
///
/// `PUT /documents/internal/{id}`
pub async fn update_internal_secure_document(
    session: &mut Session,
    document_id: i64,
    args: &CreateInternalDocumentArguments,
) -> crate::Result<serde_json::Value> {
    session
        .put(&format!("documents/internal/{document_id}"), args)
        .await
}

/// Lock or unlock a secure document.
///
/// Maps to `SecureDocumentWebService.UpdateDocumentLockedStatus()`.
///
/// # Endpoint (inferred)
///
/// `PUT /documents/{id}/locked`
pub async fn update_document_locked_status(
    session: &mut Session,
    document_id: i64,
    is_locked: bool,
) -> crate::Result<serde_json::Value> {
    let body = UpdateDocumentLockedStatusRequest { is_locked };
    session
        .put(&format!("documents/{document_id}/locked"), &body)
        .await
}

/// Soft delete a secure document.
///
/// Maps to `SecureDocumentWebService.SoftDeleteSecureDocument()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /documents/{id}`
pub async fn soft_delete_secure_document(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<serde_json::Value> {
    session.delete(&format!("documents/{document_id}")).await
}

/// Get shareable secure documents matching the given filter.
///
/// Maps to `SecureDocumentWebService.GetShareableSecureDocuments()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/shareable?<query params>`
pub async fn get_shareable_secure_documents(
    session: &mut Session,
    args: &GetShareableSecureDocumentsArguments,
) -> crate::Result<GetSecureDocumentsResult> {
    let mut query = Vec::new();
    if let Some(ref ids) = args.filter_institution_profile_ids {
        for id in ids {
            query.push(format!("filterInstitutionProfileIds={id}"));
        }
    }
    if let Some(ref ids) = args.share_to_institution_profile_ids {
        for id in ids {
            query.push(format!("shareToInstitutionProfileIds={id}"));
        }
    }
    if let Some(index) = args.index {
        query.push(format!("index={index}"));
    }
    if let Some(limit) = args.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "documents/shareable".to_string()
    } else {
        format!("documents/shareable?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get the maximum number of documents allowed per export.
///
/// Maps to `SecureDocumentWebService.GetMaxDocumentsPerExport()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/export/maxCount`
pub async fn get_max_documents_per_export(session: &mut Session) -> crate::Result<i32> {
    session.get("documents/export/maxCount").await
}

/// Create a bulk export for multiple secure documents.
///
/// Maps to `SecureDocumentWebService.CreateExportForMultipleSecureDocuments()`.
///
/// # Endpoint (inferred)
///
/// `POST /documents/export`
pub async fn create_export_for_multiple(
    session: &mut Session,
    request: &CreateExportForMultipleSecureDocumentsRequest,
) -> crate::Result<SecureDocumentExportDto> {
    session.post("documents/export", request).await
}

/// Track the status of a multi-document export.
///
/// Maps to `SecureDocumentWebService.TrackExportForMultipleSecureDocuments()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/export/{id}/status`
pub async fn track_export(
    session: &mut Session,
    export_job_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .get(&format!("documents/export/{export_job_id}/status"))
        .await
}

/// Create a PDF for a single secure document.
///
/// Maps to `SecureDocumentWebService.CreatePDFForSingleDocument()`.
///
/// # Endpoint (inferred)
///
/// `POST /documents/{id}/pdf`
pub async fn create_pdf_for_single(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .post_empty(&format!("documents/{document_id}/pdf"))
        .await
}

/// Track the status of a single document PDF generation.
///
/// Maps to `SecureDocumentWebService.TrackCreatePDFForSingleDocument()`.
///
/// # Endpoint (inferred)
///
/// `GET /documents/{id}/pdf/status`
pub async fn track_create_pdf(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .get(&format!("documents/{document_id}/pdf/status"))
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::common::SortOrderEnum;
    use crate::enums::documents::{
        CommonFileSortEnum, DocumentTypeEnum, JournalingStatusEnum, SecureDocumentExportStatus,
    };
    use crate::models::documents::{
        CreateDocumentShareGroupArguments, CreateDocumentSharedProfileArguments,
        SecureDocumentExportDto,
    };

    #[test]
    fn update_locked_status_request_serializes() {
        let req = UpdateDocumentLockedStatusRequest { is_locked: true };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["isLocked"], true);
    }

    #[test]
    fn get_secure_documents_args_serializes() {
        let args = GetSecureDocumentsArguments {
            filter_institution_profile_ids: Some(vec![1, 2]),
            filter_regarding_group_ids: None,
            filter_unread: Some(false),
            filter_locked: None,
            filter_journaling_status: Some(JournalingStatusEnum::Completed),
            filter_editable: true,
            document_type: Some(DocumentTypeEnum::Internal),
            sortings: None,
            index: Some(0),
            limit: Some(20),
            filter_regarding_student_ids: None,
            filter_document_categories: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(
            json["filterInstitutionProfileIds"],
            serde_json::json!([1, 2])
        );
        assert_eq!(json["filterUnread"], false);
        assert_eq!(json["filterEditable"], true);
        assert_eq!(json["documentType"], "Internal");
        assert_eq!(json["index"], 0);
        assert_eq!(json["limit"], 20);
    }

    #[test]
    fn get_common_files_args_serializes() {
        let args = GetCommonFilesArguments {
            page: Some(1),
            sort_type: Some(CommonFileSortEnum::Title),
            sort_order: Some(SortOrderEnum::Ascending),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["page"], 1);
        assert_eq!(json["sortType"], "Title");
        assert_eq!(json["sortOrder"], "Ascending");
    }

    #[test]
    fn update_sharing_args_serializes() {
        let args = UpdateSharingArguments {
            document_ids: Some(vec![10, 20]),
            reset_sharings: false,
            shared_groups: Some(vec![
                crate::models::documents::UpdateSharingGroupArguments {
                    group_id: Some(5),
                    can_edit: true,
                },
            ]),
            shared_institution_profiles: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["documentIds"], serde_json::json!([10, 20]));
        assert_eq!(json["resetSharings"], false);
        let groups = json["sharedGroups"].as_array().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0]["groupId"], 5);
        assert!(groups[0]["canEdit"].as_bool().unwrap());
    }

    #[test]
    fn remove_sharing_args_serializes() {
        let args = RemoveSharingArguments {
            document_ids: Some(vec![42]),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["documentIds"], serde_json::json!([42]));
    }

    #[test]
    fn create_internal_document_args_serializes() {
        let args = CreateInternalDocumentArguments {
            id: None,
            category: Some("Agenda".into()),
            creator_institution_profile_id: Some(55),
            regarding_institution_profile_ids: Some(vec![100, 101]),
            shared_with_groups: Some(vec![CreateDocumentShareGroupArguments {
                group_id: Some(5),
                can_edit: true,
            }]),
            shared_with_institution_profiles: Some(vec![CreateDocumentSharedProfileArguments {
                institution_profile_id: Some(60),
                can_edit: false,
            }]),
            title: Some("Meeting notes".into()),
            version: None,
            force_update: None,
            attached_thread: None,
            content: Some("<p>Hello world</p>".into()),
            attachment_ids: Some(vec![200]),
            implicit_sharing_overrides: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["category"], "Agenda");
        assert_eq!(json["creatorInstitutionProfileId"], 55);
        assert_eq!(json["title"], "Meeting notes");
        assert_eq!(json["content"], "<p>Hello world</p>");
        assert_eq!(json["attachmentIds"], serde_json::json!([200]));
    }

    #[test]
    fn get_shareable_args_serializes() {
        let args = GetShareableSecureDocumentsArguments {
            filter_institution_profile_ids: Some(vec![1]),
            filter_regarding_group_ids: None,
            filter_unread: None,
            filter_locked: None,
            filter_journaling_status: None,
            filter_editable: false,
            document_type: None,
            sortings: None,
            index: Some(0),
            limit: Some(10),
            filter_regarding_student_ids: None,
            filter_document_categories: None,
            share_to_institution_profile_ids: Some(vec![50, 51]),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(
            json["shareToInstitutionProfileIds"],
            serde_json::json!([50, 51])
        );
        assert_eq!(json["index"], 0);
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn create_export_request_serializes() {
        let req = CreateExportForMultipleSecureDocumentsRequest {
            secure_document_ids: Some(vec![1, 2, 3]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["secureDocumentIds"], serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn deserialize_export_dto_tracking() {
        let json = r#"{
            "requestExportJobId": 99,
            "status": "Processing",
            "progress": 0.5,
            "fileUrl": null,
            "fileName": null
        }"#;
        let dto: SecureDocumentExportDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.request_export_job_id, Some(99));
        assert_eq!(dto.status, Some(SecureDocumentExportStatus::Processing));
        assert_eq!(dto.progress, Some(0.5));
        assert!(dto.file_url.is_none());
    }

    #[test]
    fn deserialize_get_secure_documents_result() {
        let json = r#"{
            "documents": [
                {
                    "id": 1,
                    "hasMedia": false,
                    "canEdit": true,
                    "canEditLockedStatus": true,
                    "isLocked": false,
                    "title": "Doc 1",
                    "isSharedWithGuardian": false,
                    "isShareable": true
                }
            ],
            "filters": {
                "regardingGroups": [],
                "regardingInstitutionProfiles": [],
                "documentCategories": [],
                "sharedGroups": [],
                "sharedInstitutionProfiles": []
            },
            "totalCount": 1
        }"#;
        let r: GetSecureDocumentsResult = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_count, Some(1));
        let docs = r.documents.unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].title.as_deref(), Some("Doc 1"));
    }

    #[test]
    fn deserialize_implicit_sharings_dto() {
        let json = r#"{"implicitSharings": []}"#;
        let dto: GetImplicitSharingsDto = serde_json::from_str(json).unwrap();
        assert!(dto.implicit_sharings.unwrap().is_empty());
    }

    #[test]
    fn deserialize_document_revision_page() {
        let json = r#"{
            "totalCount": 2,
            "documentRevisionDtos": [
                {
                    "id": 1,
                    "createdBy": "Test User",
                    "createdAt": "2024-01-15T10:00:00",
                    "title": "Initial",
                    "changeType": "Created",
                    "sharedWith": [],
                    "unsharedWith": [],
                    "isAvailable": true
                }
            ]
        }"#;
        let page: DocumentRevisionPageDto = serde_json::from_str(json).unwrap();
        assert_eq!(page.total_count, Some(2));
        let revisions = page.document_revision_dtos.unwrap();
        assert_eq!(revisions.len(), 1);
        assert_eq!(revisions[0].created_by.as_deref(), Some("Test User"));
    }
}
