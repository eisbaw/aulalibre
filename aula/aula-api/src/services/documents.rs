//! Secure document service.
//!
//! Maps to `AulaNative.Services.Web.SecureDocumentWebService` (18+ methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_SECURE_DOCUMENTS` | `documents.getSecureDocuments` |
//! | `GET_COMMON_FILES` | `commonFiles.getCommonFiles` |
//! | `DOCUMENT_UPDATE_SHARINGS` | `documents.updateSharings` |
//! | `DOCUMENT_REMOVE_OWN_SHARINGS` | `documents.removeOwnSharings` |
//! | `POST_IMPLICIT_SHARINGS` | `documents.getImplicitSharings` |
//! | `GET_DOCUMENT_REVISIONS` | `documents.getDocumentRevisions` |
//! | `GET_EXTERNAL_SECURE_DOCUMENTS` | `documents.getExternalSecureFile` |
//! | `GET_DOCUMENT_REVISION` | `documents.getDocumentRevision` |
//! | `GET_INTERNAL_SECURE_DOCUMENTS` | `documents.getInternalSecureDocument` |
//! | `CREATE_INTERNAL_SECURE_DOCUMENT` | `documents.createInternalSecureDocument` |
//! | `UPDATE_INTERNAL_SECURE_DOCUMENT` | `documents.updateInternalSecureDocument` |
//! | `UPDATE_DOCUMENT_LOCK_STATUS` | `documents.updateLockedStatus` |
//! | `SOFT_DELETE_SECURE_DOCUMENT` | `documents.deleteDocument` |
//! | `GET_SHAREABLE_SECURE_DOCUMENT` | `documents.getShareableSecureDocuments` |
//! | `GET_MAX_DOCUMENTS_PER_EXPORT` | `documents.getMaxDocumentsPerExport` |
//! | `CREATE_ARCHIVE_FOR_MULTIPLE_SECURE_DOCUMENTS` | `documents.createArchiveForMultipleSecureDocuments` |
//! | `TRACK_CREATE_ARCHIVE_FOR_MULTIPLE_SECURE_DOCUMENTS` | `documents.trackCreateArchiveForMultipleSecureDocumentsRequest` |
//! | `CREATE_PDF_FOR_SINGLE_DOCUMENT` | `documents.createPDFForSingleDocument` |
//! | `TRACK_CREATE_PDF_FOR_SINGLE_DOCUMENT` | `documents.trackCreatePDFForSingleDocument` |

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
/// # Endpoint
///
/// `POST ?method=documents.getSecureDocuments`
///
/// Despite the `GET_SECURE_DOCUMENTS` name in `Urls.cs`, the decompiled C#
/// code shows `documentService.Post<GetSecureDocumentsResult>(GET_SECURE_DOCUMENTS, arguments)`
/// -- this endpoint uses POST with the arguments as a JSON body.
pub async fn get_secure_documents(
    session: &mut Session,
    args: &GetSecureDocumentsArguments,
) -> crate::Result<GetSecureDocumentsResult> {
    session
        .post("?method=documents.getSecureDocuments", args)
        .await
}

/// Fetch common (institution-level) files.
///
/// Maps to `SecureDocumentWebService.GetCommonFiles()`.
///
/// # Endpoint
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
        "?method=commonFiles.getCommonFiles".to_string()
    } else {
        format!("?method=commonFiles.getCommonFiles&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Update sharings on one or more secure documents.
///
/// Maps to `SecureDocumentWebService.UpdateSharings()`.
///
/// # Endpoint
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
    session.post("?method=documents.updateSharings", args).await
}

/// Remove the current user's own sharings from documents.
///
/// Maps to `SecureDocumentWebService.RemoveOwnSharings()`.
///
/// # Endpoint
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
        .post("?method=documents.removeOwnSharings", args)
        .await
}

/// Get implicit sharings for a document.
///
/// Maps to `SecureDocumentWebService.GetImplicitSharings()`.
///
/// # Endpoint
///
/// `GET /documents/{id}/implicitSharings`
pub async fn get_implicit_sharings(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<GetImplicitSharingsDto> {
    session
        .get(&format!(
            "?method=documents.getImplicitSharings&documentId={document_id}"
        ))
        .await
}

/// Get revision history for a document.
///
/// Maps to `SecureDocumentWebService.GetDocumentRevisions()`.
///
/// # Endpoint
///
/// `GET /documents/{id}/revisions?page=...`
pub async fn get_document_revisions(
    session: &mut Session,
    document_id: i64,
    page: Option<i32>,
) -> crate::Result<DocumentRevisionPageDto> {
    let path = match page {
        Some(p) => {
            format!("?method=documents.getDocumentRevisions&documentId={document_id}&page={p}")
        }
        None => format!("?method=documents.getDocumentRevisions&documentId={document_id}"),
    };
    session.get(&path).await
}

/// Get details of an external secure document.
///
/// Maps to `SecureDocumentWebService.GetExternalDocumentDetails()`.
///
/// # Endpoint
///
/// `GET /documents/external/{id}`
pub async fn get_external_document_details(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<ExternalSecureDocumentDetailsDto> {
    session
        .get(&format!(
            "?method=documents.getExternalSecureFile&documentId={document_id}"
        ))
        .await
}

/// Get a specific revision of an external secure document.
///
/// Maps to `SecureDocumentWebService.GetExternalSecureDocumentRevision()`.
///
/// # Endpoint
///
/// `GET /documents/external/{id}/revision`
pub async fn get_external_document_revision(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<ExternalSecureDocumentDetailsDto> {
    session
        .get(&format!(
            "?method=documents.getDocumentRevision&documentId={document_id}"
        ))
        .await
}

/// Get details of an internal secure document.
///
/// Maps to `SecureDocumentWebService.GetInternalDocumentDetails()`.
///
/// # Endpoint
///
/// `GET /documents/internal/{id}`
pub async fn get_internal_document_details(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<InternalSecureDocumentDetailsDto> {
    session
        .get(&format!(
            "?method=documents.getInternalSecureDocument&documentId={document_id}"
        ))
        .await
}

/// Get a specific revision of an internal secure document.
///
/// Maps to `SecureDocumentWebService.GetInternalSecureDocumentRevision()`.
///
/// # Endpoint
///
/// `GET /documents/internal/{id}/revision`
pub async fn get_internal_document_revision(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<InternalSecureDocumentDetailsDto> {
    session
        .get(&format!(
            "?method=documents.getDocumentRevision&documentId={document_id}"
        ))
        .await
}

/// Create a new internal secure document.
///
/// Maps to `SecureDocumentWebService.CreateInternalSecureDocument()`.
///
/// # Endpoint
///
/// `POST /documents/internal`
pub async fn create_internal_secure_document(
    session: &mut Session,
    args: &CreateInternalDocumentArguments,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=documents.createInternalSecureDocument", args)
        .await
}

/// Update an existing internal secure document.
///
/// Maps to `SecureDocumentWebService.UpdateInternalSecureDocument()`.
///
/// # Endpoint
///
/// `PUT /documents/internal/{id}`
pub async fn update_internal_secure_document(
    session: &mut Session,
    document_id: i64,
    args: &CreateInternalDocumentArguments,
) -> crate::Result<serde_json::Value> {
    let _ = document_id; // included in args
    session
        .post("?method=documents.updateInternalSecureDocument", args)
        .await
}

/// Lock or unlock a secure document.
///
/// Maps to `SecureDocumentWebService.UpdateDocumentLockedStatus()`.
///
/// # Endpoint
///
/// `PUT /documents/{id}/locked`
pub async fn update_document_locked_status(
    session: &mut Session,
    document_id: i64,
    is_locked: bool,
) -> crate::Result<serde_json::Value> {
    let body = UpdateDocumentLockedStatusRequest { is_locked };
    let _ = document_id; // included in body context
    session
        .post("?method=documents.updateLockedStatus", &body)
        .await
}

/// Soft delete a secure document.
///
/// Maps to `SecureDocumentWebService.SoftDeleteSecureDocument()`.
///
/// # Endpoint
///
/// `DELETE /documents/{id}`
pub async fn soft_delete_secure_document(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .post(
            "?method=documents.deleteDocument",
            &serde_json::json!({"documentId": document_id}),
        )
        .await
}

/// Get shareable secure documents matching the given filter.
///
/// Maps to `SecureDocumentWebService.GetShareableSecureDocuments()`.
///
/// # Endpoint
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
        "?method=documents.getShareableSecureDocuments".to_string()
    } else {
        format!(
            "?method=documents.getShareableSecureDocuments&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get the maximum number of documents allowed per export.
///
/// Maps to `SecureDocumentWebService.GetMaxDocumentsPerExport()`.
///
/// # Endpoint
///
/// `GET /documents/export/maxCount`
pub async fn get_max_documents_per_export(session: &mut Session) -> crate::Result<i32> {
    session
        .get("?method=documents.getMaxDocumentsPerExport")
        .await
}

/// Create a bulk export for multiple secure documents.
///
/// Maps to `SecureDocumentWebService.CreateExportForMultipleSecureDocuments()`.
///
/// # Endpoint
///
/// `POST /documents/export`
pub async fn create_export_for_multiple(
    session: &mut Session,
    request: &CreateExportForMultipleSecureDocumentsRequest,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .post(
            "?method=documents.createArchiveForMultipleSecureDocuments",
            request,
        )
        .await
}

/// Track the status of a multi-document export.
///
/// Maps to `SecureDocumentWebService.TrackExportForMultipleSecureDocuments()`.
///
/// # Endpoint
///
/// `GET /documents/export/{id}/status`
pub async fn track_export(
    session: &mut Session,
    export_job_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .get(&format!(
            "?method=documents.trackCreateArchiveForMultipleSecureDocumentsRequest&exportJobId={export_job_id}"
        ))
        .await
}

/// Create a PDF for a single secure document.
///
/// Maps to `SecureDocumentWebService.CreatePDFForSingleDocument()`.
///
/// # Endpoint
///
/// `POST /documents/{id}/pdf`
pub async fn create_pdf_for_single(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .post(
            "?method=documents.createPDFForSingleDocument",
            &serde_json::json!({"documentId": document_id}),
        )
        .await
}

/// Track the status of a single document PDF generation.
///
/// Maps to `SecureDocumentWebService.TrackCreatePDFForSingleDocument()`.
///
/// # Endpoint
///
/// `GET /documents/{id}/pdf/status`
pub async fn track_create_pdf(
    session: &mut Session,
    document_id: i64,
) -> crate::Result<SecureDocumentExportDto> {
    session
        .get(&format!(
            "?method=documents.trackCreatePDFForSingleDocument&documentId={document_id}"
        ))
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
