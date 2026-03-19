//! Search domain models.
//!
//! These types represent the search system in Aula, covering global search,
//! recipient search, message search, and all result item types. The search
//! system uses a polymorphic result pattern where `SearchResultItem` is the
//! base type and specialized result types add domain-specific fields.
//!
//! See `data_models.md` Models.Search namespace.

use serde::{Deserialize, Serialize};

use crate::enums::common::{
    FilterAndSortType, GroupSearchScopeEnum, SearchProfilePortalRoleEnum,
    SearchRecipientDocTypeEnum, SearchRecipientMailBoxOwnerType, SearchRecipientModuleEnum,
    SearchRecipientPortalRoleEnum, SearchResultItemType, SortOrderEnum,
};
use crate::enums::documents::FileScanningStatus;
use crate::enums::gallery::ConversionStatusEnum;
use crate::enums::messaging::{SensitivityLevel, ThreadType};
use crate::enums::profiles::{GroupTypeEnum, InstitutionRole, PortalRole};

use super::files::{AulaFileContent, AulaFileResultProfileDto, MembershipCountResultModel};
use super::messaging::{
    DownloadFileFromAulaArguments, MessageDraft, MessageParticipantDto, MessageRegardingChildren,
    RecipientApiModel,
};
use super::profiles::Address;

// ---------------------------------------------------------------------------
// Search response types
// ---------------------------------------------------------------------------

/// Top-level search response from global search.
///
/// Maps to `Models.Search.SearchResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub total_size: Option<i32>,
    pub doc_type_count: Option<Vec<SearchResultCountItem>>,
    pub group_type_count: Option<Vec<SearchResultGroupCountItem>>,
    pub results: Option<Vec<SearchResultItem>>,
}

/// Doc-type facet count.
///
/// Maps to `Models.Search.SearchResultCountItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultCountItem {
    pub name: Option<String>,
    pub count: Option<i32>,
}

/// Group-type facet count with enum key.
///
/// Maps to `Models.Search.SearchResultGroupCountItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupCountItem {
    pub name: Option<String>,
    pub count: Option<i32>,
    pub key: Option<GroupTypeEnum>,
}

/// Search result highlight fragment.
///
/// Maps to `Models.Search.SearchResultHighlight`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultHighlight {
    pub property: Option<String>,
    pub fragment: Option<String>,
}

// ---------------------------------------------------------------------------
// Base search result item
// ---------------------------------------------------------------------------

/// Base search result item with fields common to all result types.
///
/// Maps to `Models.Search.SearchResultItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultItem {
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Specialized search result items
// ---------------------------------------------------------------------------

/// Search result for a common (public) file.
///
/// Maps to `Models.Search.SearchResultCommonFile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultCommonFile {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub title: Option<String>,
    pub created: Option<String>,
    pub file_key: Option<String>,
    pub file_bucket: Option<String>,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub scanning_status: Option<FileScanningStatus>,
}

/// Search result for a common inbox.
///
/// Maps to `Models.Search.SearchResultCommonInboxItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultCommonInboxItem {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub score: Option<f32>,
    pub aula_email: Option<String>,
}

/// Search result for a calendar event.
///
/// Maps to `Models.Search.SearchResultEventItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultEventItem {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub title: Option<String>,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub creator_aula_name: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
}

/// Search result for a group.
///
/// Maps to `Models.Search.SearchResultGroupItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupItem {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub status: Option<String>,
    pub access: Option<String>,
    pub dashboard_enabled: Option<bool>,
    #[serde(default)]
    pub current_user_can_access_group_dashboard: bool,
    pub membership_role: Option<String>,
    #[serde(rename = "type")]
    pub group_type: Option<GroupTypeEnum>,
    pub is_group_member: Option<bool>,
    pub short_name: Option<String>,
    pub membership_count: Option<MembershipCountResultModel>,
    #[serde(default)]
    pub allow_members_to_be_shown: bool,
    pub admins: Option<Vec<SearchResultGroupAdmin>>,
}

/// Admin entry within a group search result.
///
/// Maps to `Models.Search.SearchResultGroupItem.SearchResultGroupAdmin`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupAdmin {
    pub institution_profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
}

/// Search result for a media item.
///
/// Maps to `Models.Search.SearchResultMediaItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMediaItem {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub creator: Option<AulaFileResultProfileDto>,
    pub tags: Option<Vec<AulaFileResultProfileDto>>,
    pub title: Option<String>,
    pub album_title: Option<String>,
    pub album_description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub large_thumbnail_url: Option<String>,
    pub medium_thumbnail_url: Option<String>,
    pub small_thumbnail_url: Option<String>,
    #[serde(default)]
    pub has_video_thumbnail: bool,
    pub extra_small_thumbnail_url: Option<String>,
    pub media_type: Option<String>,
    pub file: Option<AulaFileContent>,
    pub current_user_can_delete: Option<bool>,
    #[serde(default)]
    pub can_comment: bool,
    pub comment_count: Option<i32>,
    pub conversion_status: Option<ConversionStatusEnum>,
}

/// Search result for a post.
///
/// Maps to `Models.Search.SearchResultPostItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultPostItem {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub timestamp: Option<String>,
    pub publish_at: Option<String>,
    pub edited_at: Option<String>,
    pub receiver_groups: Option<Vec<String>>,
    pub creator: Option<Creator>,
}

/// Creator reference for search results.
///
/// Maps to `Models.Search.Creator`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Profile search results
// ---------------------------------------------------------------------------

/// Base profile search result.
///
/// Maps to `Models.Search.SearchResultProfileItemBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultProfileItemBase {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Profile-specific fields
    pub profile_id: Option<i64>,
    pub address: Option<Address>,
    pub portal_role: Option<String>,
    pub institution_role: Option<InstitutionRole>,
    /// C# `SearchResultProfileItemBase.InstitutionProfileId` has `[JsonProperty("id")]`.
    #[serde(rename = "id")]
    pub institution_profile_id: Option<i64>,
    pub home_phone_number: Option<String>,
    pub mobile_phone_number: Option<String>,
    pub work_phone_number: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub aula_email: Option<String>,
    /// C# `SearchResultProfileItemBase.ExternalEmail` has `[JsonProperty("email")]`.
    #[serde(rename = "email")]
    pub external_email: Option<String>,
    pub role_definitions: Option<Vec<SearchRoleDefinition>>,
    pub main_group: Option<MainGroup>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub related_groups: Option<Vec<RelatedGroup>>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Profile search result for "find recipients" context.
///
/// Maps to `Models.Search.SearchResultProfileItemFindRecipients`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultProfileItemFindRecipients {
    #[serde(flatten)]
    pub base: SearchResultProfileItemBase,
    pub related_profiles: Option<Vec<RelatedProfile>>,
    pub group_homes: Option<Vec<SearchGroupHome>>,
}

/// Profile search result for global search context.
///
/// Maps to `Models.Search.SearchResultProfileItemGlobalSearch`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultProfileItemGlobalSearch {
    #[serde(flatten)]
    pub base: SearchResultProfileItemBase,
    pub relations: Option<Vec<RelatedProfile>>,
}

/// Role definition as seen in search results (portal role + institution role pair).
///
/// Distinct from `profiles::RoleDefinition` which has `id` + `roleName`.
/// Maps to the `RoleDefinitions` field on `SearchResultProfileItemBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRoleDefinition {
    pub portal_role: Option<PortalRole>,
    pub institution_role: Option<InstitutionRole>,
}

/// Main group reference.
///
/// Maps to `Models.Search.MainGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainGroup {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Related group reference.
///
/// Maps to `Models.Search.RelatedGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedGroup {
    pub id: Option<i64>,
    pub name: Option<String>,
}

/// Related profile reference.
///
/// Maps to `Models.Search.RelatedProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub relation_type: Option<String>,
    pub aula_email: Option<String>,
    pub main_group: Option<MainGroup>,
    pub metadata: Option<String>,
}

/// Group-home reference for recipient search.
///
/// Maps to `Models.Search.SearchGroupHome`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupHome {
    pub name: Option<String>,
    pub otp_inbox_id: Option<i64>,
    pub id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Secure file search results
// ---------------------------------------------------------------------------

/// Search result for a secure file.
///
/// Maps to `Models.Search.SearchResultSecureFile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultSecureFile {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Specialized fields
    pub id: Option<i64>,
    pub category: Option<String>,
    pub child_associations: Option<Vec<SearchResultSecureFileChildAssociation>>,
    pub group_associations: Option<Vec<SearchResultSecureFileGroupAssociation>>,
    pub created: Option<String>,
    pub edited: Option<String>,
    pub creator_name: Option<String>,
    /// Note: original .NET field is misspelled as `Metada`.
    pub metada: Option<String>,
    pub title: Option<String>,
}

/// Group association on a secure file search result.
///
/// Maps to `Models.Search.SearchResultSecureFile.SearchResultSecureFileGroupAssociation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultSecureFileGroupAssociation {
    pub id: Option<i64>,
    pub name: Option<String>,
    /// Note: original .NET field is misspelled as `InstituionCode`.
    pub instituion_code: Option<String>,
}

/// Child association on a secure file search result.
///
/// Maps to `Models.Search.SearchResultSecureFileChildAssociation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultSecureFileChildAssociation {
    pub profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Message search types
// ---------------------------------------------------------------------------

/// Base message search result item.
///
/// Maps to `Models.Search.SearchResultItems.Messages.BaseSearchResultMessageItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSearchResultMessageItem {
    // Base SearchResultItem fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Message-specific fields
    #[serde(default)]
    pub marked: bool,
    #[serde(default)]
    pub muted: bool,
    pub thread: Option<SearchResultMessageThreadItem>,
    pub leave_time: Option<String>,
    pub sensitivity_level: Option<i32>,
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub selected_in_multi_edit_mode: bool,
    pub message_draft: Option<MessageDraft>,
    pub mail_box_owner: Option<RecipientApiModel>,
    pub folder_id: Option<i64>,
    pub folder_name: Option<String>,
    pub subscription_id: Option<i64>,
    pub regarding_children: Option<Vec<MessageRegardingChildren>>,
}

/// Individual message within a search result.
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessage {
    pub id: Option<String>,
    pub text: Option<serde_json::Value>, // RichTextWrapperDto
    pub send_date_time: Option<String>,
    pub sender_email: Option<String>,
    pub sender_display_name: Option<String>,
    pub message_type: Option<String>,
    #[serde(default)]
    pub unread: bool,
}

/// Message search result for global search context.
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessageGlobalSearchItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessageGlobalSearchItem {
    #[serde(flatten)]
    pub base: BaseSearchResultMessageItem,
    pub message: Option<SearchResultMessage>,
}

/// Simple message search result (used in typeahead/simple search).
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessageItemSimple`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessageItemSimple {
    // Base fields
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Simple message fields
    pub message_id: Option<String>,
    pub message: Option<String>,
    pub subscription_id: Option<i64>,
    pub author: Option<String>,
    pub metadata: Option<String>,
    pub thread_id: Option<i64>,
    pub title: Option<String>,
    #[serde(default)]
    pub step_up_required: bool,
    pub latest_message_send_time: Option<String>,
    pub mail_box_owner: Option<RecipientApiModel>,
}

/// Message search result for message module.
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessageMessageModuleItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessageMessageModuleItem {
    #[serde(flatten)]
    pub base: BaseSearchResultMessageItem,
    pub search_message: Option<SearchResultMessage>,
    pub recipients: Option<Vec<MessageParticipantDto>>,
    pub creator: Option<MessageParticipantDto>,
    pub extra_recipients_count: Option<i64>,
}

/// Thread info within a message search result.
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessageThreadItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessageThreadItem {
    pub id: Option<i64>,
    pub subject: Option<String>,
    pub sensitivity_level: Option<SensitivityLevel>,
    #[serde(default)]
    pub is_forwarded: bool,
    pub thread_type: Option<ThreadType>,
}

/// Response wrapper for message search.
///
/// Maps to `Models.Search.SearchResultItems.Messages.SearchResultMessagesResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultMessagesResponse {
    pub total_hits: Option<i32>,
    pub results: Option<Vec<SearchResultMessageMessageModuleItem>>,
}

/// Response wrapper for recipient search.
///
/// Maps to `Models.SearchRecipients.SearchRecipientResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecipientResponse {
    pub total_hits: Option<i32>,
    pub results: Option<Vec<SearchResultItem>>,
}

/// Child relations response for recipient context.
///
/// Maps to `Models.Search.ChildRelationsResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildRelationsResponse {
    pub child_relations_profile_list: Option<Vec<serde_json::Value>>, // ChildRelationsProfile
    pub search_recipient_group_list: Option<Vec<SearchResultGroupItem>>,
}

// ---------------------------------------------------------------------------
// Group search types
// ---------------------------------------------------------------------------

/// Group search result model.
///
/// Maps to `Models.Search.SearchResultItems.SearchGroupItemResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupItemResultModel {
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}

/// Group search response.
///
/// Maps to `Models.Search.SearchResultItems.SearchGroupResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupResultModel {
    pub results: Option<Vec<SearchGroupItemResultModel>>,
}

// ---------------------------------------------------------------------------
// Search request parameter models
// ---------------------------------------------------------------------------

/// Global search parameters.
///
/// Maps to `Models.Search.RequestModels.GlobalSearchParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSearchParameters {
    pub text: Option<String>,
    pub page_limit: Option<i32>,
    pub page_number: Option<i32>,
    pub group_id: Option<i64>,
    #[serde(default)]
    pub doc_type_count: bool,
    pub doc_type: Option<SearchResultItemType>,
    pub group_types: Option<Vec<GroupTypeEnum>>,
}

/// Secure document association search parameters.
///
/// Maps to `Models.Search.RequestModels.SearchForAssociateSecureDocumentsParameter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForAssociateSecureDocumentsParameter {
    pub institution_codes: Option<Vec<String>>,
    pub text: Option<String>,
}

/// Profile and group search parameters.
///
/// Maps to `Models.Search.RequestModels.SearchForProfilesAndGroupsParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForProfilesAndGroupsParameters {
    #[serde(default)]
    pub only_profiles: bool,
    pub text: Option<String>,
    pub portal_roles: Option<SearchProfilePortalRoleEnum>,
    #[serde(default)]
    pub typeahead: bool,
    pub limit: Option<i32>,
}

/// Group search request.
///
/// Maps to `Models.Search.RequestModels.SearchGroupRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchGroupRequestModel {
    pub text: Option<String>,
    pub institution_codes: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub from_module_value: Option<SearchRecipientModuleEnum>,
}

/// Message search request.
///
/// Maps to `Models.Search.RequestModels.SearchMessageRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMessageRequestModel {
    pub keyword: Option<String>,
    pub thread_subject: Option<String>,
    pub message_content: Option<String>,
    pub has_attachments: Option<bool>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub thread_creators: Option<Vec<serde_json::Value>>, // RecipientViewModel
    pub participants: Option<Vec<serde_json::Value>>,    // RecipientViewModel
    pub page: Option<i32>,
    pub common_inbox_id: Option<i64>,
    pub folder_id: Option<i64>,
    pub filter: Option<FilterAndSortType>,
    pub sort_type: Option<FilterAndSortType>,
    pub sort_order: Option<SortOrderEnum>,
}

/// Recipient search parameters.
///
/// Maps to `Models.Search.RequestModels.SearchRecipientParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRecipientParameters {
    pub text: Option<String>,
    pub from_module: Option<SearchRecipientModuleEnum>,
    pub doc_types: Option<SearchRecipientDocTypeEnum>,
    pub portal_roles: Option<SearchRecipientPortalRoleEnum>,
    pub group_search_scope: Option<GroupSearchScopeEnum>,
    pub limit: Option<i32>,
    pub scope_employees_to_institution: Option<bool>,
    pub group_id: Option<i32>,
    pub inst_code: Option<String>,
    pub institution_codes: Option<Vec<String>>,
    pub regarding_children: Option<Vec<i64>>,
    pub mail_box_owner_type: Option<SearchRecipientMailBoxOwnerType>,
    pub mail_box_owner_id: Option<i64>,
}

/// Resource search parameters.
///
/// Maps to `Models.Search.RequestModels.SearchResourceParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResourceParameters {
    pub query: Option<String>,
    pub institution_code: Option<Vec<String>>,
    pub exclude_types: Option<Vec<String>>,
    pub include_types: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_search_response() {
        let json = r#"{
            "totalSize": 42,
            "docTypeCount": [
                {"name": "Profile", "count": 10}
            ],
            "groupTypeCount": [
                {"name": "Institutional", "count": 5, "key": "Institutional"}
            ],
            "results": [
                {
                    "docId": "doc-1",
                    "docType": "Profile",
                    "institutionCode": "101001",
                    "institutionName": "Viby Skole",
                    "municipalityCode": "751",
                    "municipalityName": "Aarhus",
                    "name": "Test User",
                    "description": null
                }
            ]
        }"#;
        let r: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.total_size, Some(42));
        assert_eq!(r.results.as_ref().unwrap().len(), 1);
        assert_eq!(r.doc_type_count.as_ref().unwrap()[0].count, Some(10));
    }

    #[test]
    fn deserialize_search_result_group_item() {
        let json = r#"{
            "docId": "grp-1",
            "docType": "Group",
            "institutionCode": "101001",
            "institutionName": "Viby Skole",
            "municipalityCode": null,
            "municipalityName": null,
            "name": "3.A",
            "description": "Class 3A",
            "id": 42,
            "status": "Active",
            "access": "Closed",
            "dashboardEnabled": true,
            "currentUserCanAccessGroupDashboard": true,
            "membershipRole": "Member",
            "type": "Institutional",
            "isGroupMember": true,
            "shortName": "3A",
            "membershipCount": {"employees": 2, "children": 25, "guardians": 40, "total": 67},
            "allowMembersToBeShown": true,
            "admins": [
                {"institutionProfileId": 99, "firstName": "Lars", "lastName": "Jensen", "fullName": "Lars Jensen"}
            ]
        }"#;
        let g: SearchResultGroupItem = serde_json::from_str(json).unwrap();
        assert_eq!(g.id, Some(42));
        assert_eq!(g.group_type, Some(GroupTypeEnum::Institutional));
        assert!(g.current_user_can_access_group_dashboard);
        assert_eq!(g.admins.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_search_result_post_item() {
        let json = r#"{
            "docId": "post-1",
            "docType": "Post",
            "institutionCode": "101001",
            "institutionName": null,
            "municipalityCode": null,
            "municipalityName": null,
            "name": null,
            "description": null,
            "id": 10,
            "title": "Forældremøde",
            "content": "Kære forældre...",
            "timestamp": "2026-03-15T12:00:00",
            "publishAt": "2026-03-15T12:00:00",
            "editedAt": null,
            "receiverGroups": ["3.A"],
            "creator": {"profileId": 5, "firstName": "Anne", "lastName": "Hansen"}
        }"#;
        let p: SearchResultPostItem = serde_json::from_str(json).unwrap();
        assert_eq!(p.id, Some(10));
        assert_eq!(
            p.creator.as_ref().unwrap().first_name.as_deref(),
            Some("Anne")
        );
    }

    #[test]
    fn deserialize_global_search_parameters() {
        let json = r#"{
            "text": "forældremøde",
            "pageLimit": 20,
            "pageNumber": 1,
            "groupId": null,
            "docTypeCount": true,
            "docType": "event",
            "groupTypes": ["Institutional"]
        }"#;
        let p: GlobalSearchParameters = serde_json::from_str(json).unwrap();
        assert_eq!(p.text.as_deref(), Some("forældremøde"));
        assert!(p.doc_type_count);
        assert_eq!(p.doc_type, Some(SearchResultItemType::Event));
    }

    #[test]
    fn deserialize_search_result_message_thread_item() {
        let json = r#"{
            "id": 100,
            "subject": "Viktig besked",
            "sensitivityLevel": "level1",
            "isForwarded": false,
            "threadType": "thread"
        }"#;
        let t: SearchResultMessageThreadItem = serde_json::from_str(json).unwrap();
        assert_eq!(t.id, Some(100));
        assert_eq!(t.thread_type, Some(ThreadType::Thread));
        assert!(!t.is_forwarded);
    }
}
