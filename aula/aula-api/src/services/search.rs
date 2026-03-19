//! Search service.
//!
//! Maps to `AulaNative.Services.Web.SearchWebService` (9 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GLOBAL_SEARCH` | `search.findGeneric` |
//! | `SEARCH_IN_MESSAGES` | `search.findMessage` |
//! | `FIND_PROFILES` | `search.findProfiles` |
//! | `FIND_PROFILES_AND_GROUPS` | `search.findProfilesAndGroups` |
//! | `FIND_RECIPIENTS` | `search.findRecipients` |
//! | `FIND_RECIPIENTS_FOR_PERSONAL_REFERENCE` | `search.findRecipientsPersonalReferenceData` |
//! | `FIND_RECIPIENTS_FOR_SECURED_DOCUMENT` | `search.findProfilesAndGroupsToShareDocument` |
//! | `FIND_PROFILE_AND_GROUPS_TO_ASSOCIATE_DOCUMENT` | `search.findProfilesAndGroupsToAssociateDocument` |
//! | `FIND_GROUPS` | `search.findGroups` |

use crate::models::search::{
    GlobalSearchParameters, SearchForAssociateSecureDocumentsParameter,
    SearchForProfilesAndGroupsParameters, SearchGroupRequestModel, SearchGroupResultModel,
    SearchMessageRequestModel, SearchRecipientParameters, SearchRecipientResponse, SearchResponse,
    SearchResultMessagesResponse,
};
use crate::services::query::encode_value;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Global search across all content types (posts, events, profiles, etc.).
///
/// Maps to `SearchWebService.GlobalSearch()`.
///
/// # Endpoint
///
/// `GET ?method=search.findGeneric`
pub async fn global_search(
    session: &mut Session,
    params: &GlobalSearchParameters,
) -> crate::Result<SearchResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
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
        // Use serde serialization for enum variant name (e.g. "Post", "Event")
        let json = serde_json::to_string(dt).expect("fieldless enum serialization cannot fail");
        query.push(format!(
            "docType={}",
            encode_value(&json[1..json.len() - 1])
        ));
    }

    let path = if query.is_empty() {
        "?method=search.findGeneric".to_string()
    } else {
        format!("?method=search.findGeneric&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for messages.
///
/// Maps to `SearchWebService.SearchForMessages()`.
///
/// # Endpoint
///
/// `POST ?method=search.findMessage`
pub async fn search_for_messages(
    session: &mut Session,
    params: &SearchMessageRequestModel,
) -> crate::Result<SearchResultMessagesResponse> {
    session.post("?method=search.findMessage", params).await
}

/// Search for profiles.
///
/// Maps to `SearchWebService.SearchForProfiles()`.
///
/// # Endpoint
///
/// `GET ?method=search.findProfiles`
///
/// # Note
///
/// The decompiled .NET code declares the return type as
/// `List<SearchResultProfileItemGlobalSearch>`, but the actual API returns a
/// `SearchResponse`-shaped envelope with `results`, `from`, `query`, etc.
/// We use `SearchResponse` which captures the `results` field. Extra fields
/// like `from`, `query`, `mediaResults` are silently ignored.
pub async fn search_for_profiles(
    session: &mut Session,
    params: &SearchForProfilesAndGroupsParameters,
) -> crate::Result<SearchResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
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
        "?method=search.findProfiles".to_string()
    } else {
        format!("?method=search.findProfiles&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for profiles and groups combined.
///
/// Maps to `SearchWebService.SearchForProfilesAndGroups()`.
///
/// # Endpoint
///
/// `GET ?method=search.findProfilesAndGroups`
pub async fn search_for_profiles_and_groups(
    session: &mut Session,
    params: &SearchForProfilesAndGroupsParameters,
) -> crate::Result<SearchResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
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
        "?method=search.findProfilesAndGroups".to_string()
    } else {
        format!("?method=search.findProfilesAndGroups&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for message recipients.
///
/// Maps to `SearchWebService.SearchForRecipients()`.
///
/// # Endpoint
///
/// `GET ?method=search.findRecipients`
pub async fn search_for_recipients(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(ref inst) = params.inst_code {
        query.push(format!("instCode={}", encode_value(inst)));
    }

    let path = if query.is_empty() {
        "?method=search.findRecipients".to_string()
    } else {
        format!("?method=search.findRecipients&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Search for recipients for personal reference.
///
/// Maps to `SearchWebService.SearchForRecipientsForPersonalReference()`.
///
/// # Endpoint
///
/// `GET ?method=search.findRecipientsPersonalReferenceData`
pub async fn search_for_recipients_for_personal_reference(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "?method=search.findRecipientsPersonalReferenceData".to_string()
    } else {
        format!(
            "?method=search.findRecipientsPersonalReferenceData&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Search for recipients for secure document sharing.
///
/// Maps to `SearchWebService.SearchForRecipientsForSecureDocument()`.
///
/// # Endpoint
///
/// `GET ?method=search.findProfilesAndGroupsToShareDocument`
pub async fn search_for_recipients_for_secure_document(
    session: &mut Session,
    params: &SearchRecipientParameters,
) -> crate::Result<SearchRecipientResponse> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = if query.is_empty() {
        "?method=search.findProfilesAndGroupsToShareDocument".to_string()
    } else {
        format!(
            "?method=search.findProfilesAndGroupsToShareDocument&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Search for groups to associate with a document.
///
/// Maps to `SearchWebService.SearchForGroupsToAssociateDocument()`.
///
/// # Endpoint
///
/// `GET ?method=search.findProfilesAndGroupsToAssociateDocument`
pub async fn search_for_groups_to_associate_document(
    session: &mut Session,
    params: &SearchForAssociateSecureDocumentsParameter,
) -> crate::Result<SearchGroupResultModel> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
    }
    if let Some(ref codes) = params.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={}", encode_value(code)));
        }
    }

    let path = if query.is_empty() {
        "?method=search.findProfilesAndGroupsToAssociateDocument".to_string()
    } else {
        format!(
            "?method=search.findProfilesAndGroupsToAssociateDocument&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Search for groups.
///
/// Maps to `SearchWebService.SearchGroups()`.
///
/// # Endpoint
///
/// `GET ?method=search.findGroups`
pub async fn search_groups(
    session: &mut Session,
    params: &SearchGroupRequestModel,
) -> crate::Result<SearchGroupResultModel> {
    let mut query = Vec::new();
    if let Some(ref text) = params.text {
        query.push(format!("text={}", encode_value(text)));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(offset) = params.offset {
        query.push(format!("offset={offset}"));
    }
    if let Some(ref codes) = params.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={}", encode_value(code)));
        }
    }

    let path = if query.is_empty() {
        "?method=search.findGroups".to_string()
    } else {
        format!("?method=search.findGroups&{}", query.join("&"))
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
