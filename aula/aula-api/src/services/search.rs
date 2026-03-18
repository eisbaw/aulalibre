//! Search service.
//!
//! Maps to `AulaNative.Services.Web.SearchWebService` (9 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.15.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `global_search` | GET | `/search` |
//! | `search_for_messages` | GET | `/search/messages` |
//! | `search_for_profiles` | GET | `/search/profiles` |
//! | `search_for_profiles_and_groups` | GET | `/search/profilesAndGroups` |
//! | `search_for_recipients` | GET | `/search/recipients` |
//! | `search_for_recipients_for_personal_reference` | GET | `/search/recipients/personalReference` |
//! | `search_for_recipients_for_secure_document` | GET | `/search/recipients/secureDocument` |
//! | `search_for_groups_to_associate_document` | GET | `/search/groups/document` |
//! | `search_groups` | GET | `/search/groups` |

use crate::models::search::{
    GlobalSearchParameters, SearchForAssociateSecureDocumentsParameter,
    SearchForProfilesAndGroupsParameters, SearchGroupRequestModel, SearchGroupResultModel,
    SearchMessageRequestModel, SearchRecipientParameters, SearchRecipientResponse, SearchResponse,
    SearchResultMessagesResponse, SearchResultProfileItemGlobalSearch,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Global search across all content types (posts, events, profiles, etc.).
///
/// Maps to `SearchWebService.GlobalSearch()`.
///
/// # Endpoint (inferred)
///
/// `GET /search?<query params>`
pub async fn global_search(
    session: &mut Session,
    params: &GlobalSearchParameters,
) -> crate::Result<SearchResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(limit) = params.page_limit {
        query.push(format!("pageLimit={limit}"));
    }
    if let Some(page) = params.page_number {
        query.push(format!("pageNumber={page}"));
    }
    if let Some(gid) = params.group_id {
        query.push(format!("groupId={gid}"));
    }
    if params.doc_type_count {
        query.push("docTypeCount=true".to_string());
    }
    if let Some(ref dt) = params.doc_type {
        query.push(format!("docType={dt:?}"));
    }

    let path = if query.is_empty() {
        "search".to_string()
    } else {
        format!("search?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for messages.
///
/// Maps to `SearchWebService.SearchForMessages()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/messages` (params as query string)
///
/// The .NET service may use POST for complex search bodies; the exact
/// HTTP method has not been verified.
pub async fn search_for_messages(
    session: &mut Session,
    params: &SearchMessageRequestModel,
) -> crate::Result<SearchResultMessagesResponse> {
    // Complex search bodies are likely POSTed in .NET
    session.post("search/messages", params).await
}

/// Search for profiles.
///
/// Maps to `SearchWebService.SearchForProfiles()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/profiles`
pub async fn search_for_profiles(
    session: &mut Session,
    params: &SearchForProfilesAndGroupsParameters,
) -> crate::Result<Vec<SearchResultProfileItemGlobalSearch>> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if params.only_profiles {
        query.push("onlyProfiles=true".to_string());
    }
    if params.typeahead {
        query.push("typeahead=true".to_string());
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "search/profiles".to_string()
    } else {
        format!("search/profiles?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for profiles and groups combined.
///
/// Maps to `SearchWebService.SearchForProfilesAndGroups()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/profilesAndGroups`
pub async fn search_for_profiles_and_groups(
    session: &mut Session,
    params: &SearchForProfilesAndGroupsParameters,
) -> crate::Result<SearchResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if params.only_profiles {
        query.push("onlyProfiles=true".to_string());
    }
    if params.typeahead {
        query.push("typeahead=true".to_string());
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "search/profilesAndGroups".to_string()
    } else {
        format!("search/profilesAndGroups?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for message recipients.
///
/// Maps to `SearchWebService.SearchForRecipients()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/recipients`
pub async fn search_for_recipients(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(ref inst) = params.inst_code {
        query.push(format!("instCode={inst}"));
    }

    let path = if query.is_empty() {
        "search/recipients".to_string()
    } else {
        format!("search/recipients?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for recipients for personal reference.
///
/// Maps to `SearchWebService.SearchForRecipientsForPersonalReference()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/recipients/personalReference`
pub async fn search_for_recipients_for_personal_reference(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "search/recipients/personalReference".to_string()
    } else {
        format!("search/recipients/personalReference?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for recipients for secure document sharing.
///
/// Maps to `SearchWebService.SearchForRecipientsForSecureDocument()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/recipients/secureDocument`
pub async fn search_for_recipients_for_secure_document(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "search/recipients/secureDocument".to_string()
    } else {
        format!("search/recipients/secureDocument?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for groups to associate with a document.
///
/// Maps to `SearchWebService.SearchForGroupsToAssociateDocument()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/groups/document`
pub async fn search_for_groups_to_associate_document(
    session: &mut Session,
    params: &SearchForAssociateSecureDocumentsParameter,
) -> crate::Result<SearchGroupResultModel> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(ref codes) = params.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={code}"));
        }
    }

    let path = if query.is_empty() {
        "search/groups/document".to_string()
    } else {
        format!("search/groups/document?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for groups.
///
/// Maps to `SearchWebService.SearchGroups()`.
///
/// # Endpoint (inferred)
///
/// `GET /search/groups`
pub async fn search_groups(
    session: &mut Session,
    params: &SearchGroupRequestModel,
) -> crate::Result<SearchGroupResultModel> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={text}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(offset) = params.offset {
        query.push(format!("offset={offset}"));
    }
    if let Some(ref codes) = params.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={code}"));
        }
    }

    let path = if query.is_empty() {
        "search/groups".to_string()
    } else {
        format!("search/groups?{}", query.join("&"))
    };
    session.get(&path).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::search::{
        GlobalSearchParameters, SearchForProfilesAndGroupsParameters, SearchGroupRequestModel,
        SearchGroupResultModel, SearchRecipientParameters, SearchRecipientResponse, SearchResponse,
    };

    #[test]
    fn global_search_params_serialize() {
        let params = GlobalSearchParameters {
            text: Some("test".into()),
            page_limit: Some(20),
            page_number: Some(1),
            group_id: None,
            doc_type_count: true,
            doc_type: None,
            group_types: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["text"], "test");
        assert_eq!(json["pageLimit"], 20);
        assert_eq!(json["docTypeCount"], true);
    }

    #[test]
    fn search_profiles_params_serialize() {
        let params = SearchForProfilesAndGroupsParameters {
            only_profiles: true,
            text: Some("lars".into()),
            portal_roles: None,
            typeahead: true,
            limit: Some(10),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["onlyProfiles"], true);
        assert_eq!(json["typeahead"], true);
    }

    #[test]
    fn search_recipient_response_deserializes() {
        let json = r#"{
            "totalHits": 5,
            "results": [
                {
                    "docId": "p-1",
                    "docType": "Profile",
                    "institutionCode": "101001",
                    "institutionName": "Viby Skole",
                    "municipalityCode": null,
                    "municipalityName": null,
                    "name": "Test User",
                    "description": null
                }
            ]
        }"#;
        let r: SearchRecipientResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_hits, Some(5));
        assert_eq!(r.results.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn search_group_result_model_deserializes() {
        let json = r#"{
            "results": [
                {
                    "institutionCode": "101001",
                    "institutionName": "Viby Skole",
                    "name": "3.A",
                    "id": 42
                }
            ]
        }"#;
        let r: SearchGroupResultModel = serde_json::from_str(json).unwrap();
        assert_eq!(r.results.as_ref().unwrap().len(), 1);
        assert_eq!(r.results.as_ref().unwrap()[0].id, Some(42));
    }

    #[test]
    fn search_group_request_model_serializes() {
        let params = SearchGroupRequestModel {
            text: Some("music".into()),
            institution_codes: Some(vec!["101001".into()]),
            limit: Some(10),
            offset: Some(0),
            from_module_value: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["text"], "music");
        assert_eq!(json["limit"], 10);
    }

    #[test]
    fn search_recipient_params_serialize() {
        let params = SearchRecipientParameters {
            text: Some("lars".into()),
            from_module: None,
            doc_types: None,
            portal_roles: None,
            group_search_scope: None,
            limit: Some(10),
            scope_employees_to_institution: None,
            group_id: None,
            inst_code: Some("101001".into()),
            institution_codes: None,
            regarding_children: None,
            mail_box_owner_type: None,
            mail_box_owner_id: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["text"], "lars");
        assert_eq!(json["instCode"], "101001");
    }

    #[test]
    fn empty_search_response_deserializes() {
        let json = r#"{
            "totalSize": 0,
            "docTypeCount": [],
            "groupTypeCount": [],
            "results": []
        }"#;
        let r: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_size, Some(0));
        assert!(r.results.as_ref().unwrap().is_empty());
    }
}
