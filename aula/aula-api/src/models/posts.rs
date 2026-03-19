//! Post and comment models.
//!
//! Types from `Models.Posts.Api` and `Models.Comments` covering posts,
//! post creation/filtering, comments, and comment moderation.
//!
//! See `data_models.md` Models.Posts.Api and Models.Comments namespaces.

use serde::{Deserialize, Serialize};

use crate::enums::common::CommentType;
use crate::enums::profiles::PortalRole;

use super::documents::SimpleInstitutionProfile;
use super::files::{AulaFileResultDto, LinkedGroupRequestModel, ShareWithGroupDto};
use super::messaging::{DownloadFileFromAulaArguments, RichTextWrapperDto};

// ---------------------------------------------------------------------------
// Profile types used by posts
// ---------------------------------------------------------------------------

/// Profile info in post context (owner, related).
///
/// Maps to `Models.Common.Api.ProfileApiDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileApiDto {
    pub institution_profile_id: Option<i64>,
    pub profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub role: Option<String>,
    pub short_name: Option<String>,
    pub main_group_name: Option<String>,
    pub metadata: Option<String>,
    pub institution: Option<serde_json::Value>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

// ---------------------------------------------------------------------------
// Post DTOs
// ---------------------------------------------------------------------------

/// A post on the activity feed / bulletin board.
///
/// Maps to `Models.Posts.Api.PostApiDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostApiDto {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: Option<RichTextWrapperDto>,
    pub comment_count: Option<i32>,
    pub time_stamp: Option<String>,
    pub owner_profile: Option<ProfileApiDto>,
    #[serde(default)]
    pub allow_comments: bool,
    #[serde(default)]
    pub is_important: bool,
    pub important_from: Option<String>,
    pub important_to: Option<String>,
    pub related_profiles: Option<Vec<ProfileApiDto>>,
    pub shared_with_groups: Option<Vec<ShareWithGroupDto>>,
    pub attachments: Option<Vec<AulaFileResultDto>>,
    #[serde(default)]
    pub can_current_user_report: bool,
    #[serde(default)]
    pub can_current_user_delete: bool,
    #[serde(default)]
    pub can_current_user_comment: bool,
    pub publish_at: Option<String>,
    pub expire_at: Option<String>,
    pub edited_at: Option<String>,
    #[serde(default)]
    pub is_bookmarked: bool,
}

/// Parameters for creating or updating a post.
///
/// Maps to `Models.Posts.Api.CreatePostApiParameter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostApiParameter {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub institution_code: Option<String>,
    pub creator_institution_profile_id: Option<i64>,
    #[serde(default)]
    pub allow_comments: bool,
    #[serde(default)]
    pub is_important: bool,
    pub important_from: Option<String>,
    pub important_to: Option<String>,
    pub shared_with_groups: Option<Vec<LinkedGroupRequestModel>>,
    pub attachment_ids: Option<Vec<i64>>,
    pub publish_at: Option<String>,
    pub expire_at: Option<String>,
}

/// Parameters for filtering/querying posts.
///
/// Maps to `Models.Posts.Api.GetPostApiParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostApiParameters {
    pub group_id: Option<i64>,
    pub is_important: Option<bool>,
    pub creator_portal_role: Option<String>,
    pub institution_profile_ids: Option<Vec<i64>>,
    pub related_institutions: Option<Vec<String>>,
    #[serde(default)]
    pub own_post: bool,
    #[serde(default)]
    pub is_unread: bool,
    #[serde(default)]
    pub is_bookmarked: bool,
    pub limit: Option<i32>,
    pub index: Option<i32>,
}

/// Paginated result of post queries.
///
/// Maps to `Models.Posts.Api.GetPostApiResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostApiResult {
    #[serde(default)]
    pub has_more_posts: bool,
    pub pagination_start: Option<String>,
    pub pagination_end: Option<String>,
    pub page: Option<i32>,
    pub posts: Option<Vec<PostApiDto>>,
}

/// Result of creating a post (consent validation).
///
/// Maps to `Models.Posts.Api.CreatePostResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostResult {
    #[serde(default)]
    pub all_images_has_valid_consents: bool,
}

/// Parameters for reporting a post.
///
/// Maps to `Models.Posts.Api.ReportApiParameter`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportApiParameter {
    pub id: Option<i64>,
    pub report_reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Comment types
// ---------------------------------------------------------------------------

/// A comment on a post or media item (recursive for replies).
///
/// Maps to `Models.Comments.CommentResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResultModel {
    pub id: Option<i64>,
    pub creator: Option<SimpleInstitutionProfile>,
    pub content: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub comments: Option<Vec<CommentResultModel>>,
    pub comment_count: Option<i32>,
    #[serde(default)]
    pub can_delete: bool,
    #[serde(default)]
    pub can_report: bool,
    #[serde(default)]
    pub is_deleted: bool,
    #[serde(default)]
    pub is_reported: bool,
}

/// Institution profile eligible to comment.
///
/// Maps to `CommentableInstitutionProfile` (extends SimpleInstitutionProfile).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentableInstitutionProfile {
    // Base from SimpleInstitutionProfile
    pub profile_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    pub role: Option<PortalRole>,
    pub main_group: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
    pub short_name: Option<String>,
    // Extension
    pub metadata: Option<String>,
    #[serde(default)]
    pub is_selected: bool,
}

/// Paginated comment list.
///
/// Maps to `Models.Comments.PagedCommentList`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedCommentList {
    pub start_index: Option<i32>,
    pub limit: Option<i32>,
    pub total_result_count: Option<i32>,
    pub comments: Option<Vec<CommentResultModel>>,
    pub commentable_institution_profiles: Option<Vec<CommentableInstitutionProfile>>,
}

/// Comment target reference (type + id).
///
/// Maps to `Models.Comments.Parameter.CommentItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentItem {
    #[serde(rename = "type")]
    pub comment_type: Option<CommentType>,
    pub id: Option<i64>,
}

/// Request to delete a comment.
///
/// Maps to `Models.Comments.Parameter.DeleteCommentRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentRequestModel {
    pub comment_id: Option<i64>,
    pub parent_type: Option<CommentType>,
}

/// Parameters for reporting a comment.
///
/// Maps to `Models.Comments.Parameter.ReportCommentApiParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCommentApiParameters {
    pub comment_id: Option<i64>,
    pub report_reason: Option<String>,
}

/// Request to update a comment.
///
/// Maps to `Models.Comments.Parameter.UpdateCommentRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentRequestModel {
    pub comment_id: Option<i64>,
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_post_api_dto() {
        let json = r#"{
            "id": 42,
            "title": "Tur til naturen",
            "content": {"html": "<p>Vi skal på tur!</p>"},
            "commentCount": 5,
            "timeStamp": "2024-03-15T08:00:00",
            "ownerProfile": {
                "institutionProfileId": 55,
                "profileId": 33,
                "firstName": "Lars",
                "lastName": "Hansen",
                "fullName": "Lars Hansen",
                "role": "Employee",
                "shortName": "LH",
                "mainGroupName": "3.A",
                "metadata": null,
                "institution": null,
                "profilePicture": null
            },
            "allowComments": true,
            "isImportant": false,
            "importantFrom": null,
            "importantTo": null,
            "relatedProfiles": [],
            "sharedWithGroups": [],
            "attachments": [],
            "canCurrentUserReport": false,
            "canCurrentUserDelete": true,
            "canCurrentUserComment": true,
            "publishAt": "2024-03-15T08:00:00",
            "expireAt": "2024-06-15T23:59:59",
            "editedAt": null,
            "isBookmarked": false
        }"#;
        let p: PostApiDto = serde_json::from_str(json).unwrap();
        assert_eq!(p.id, Some(42));
        assert_eq!(p.title.as_deref(), Some("Tur til naturen"));
        assert!(p.allow_comments);
        assert!(!p.is_important);
        assert!(p.can_current_user_delete);
        assert_eq!(p.comment_count, Some(5));
        let owner = p.owner_profile.unwrap();
        assert_eq!(owner.full_name.as_deref(), Some("Lars Hansen"));
    }

    #[test]
    fn deserialize_create_post_api_parameter() {
        let json = r#"{
            "id": 0,
            "title": "New post",
            "content": "<p>Hello</p>",
            "institutionCode": "101001",
            "creatorInstitutionProfileId": 55,
            "allowComments": true,
            "isImportant": false,
            "importantFrom": null,
            "importantTo": null,
            "sharedWithGroups": [{"groupId": 5, "portalRolesEnum": ["guardian"]}],
            "attachmentIds": [1, 2],
            "publishAt": "2024-03-15T08:00:00",
            "expireAt": "2024-06-15T23:59:59"
        }"#;
        let p: CreatePostApiParameter = serde_json::from_str(json).unwrap();
        assert_eq!(p.title.as_deref(), Some("New post"));
        assert!(p.allow_comments);
        assert_eq!(p.attachment_ids.unwrap(), vec![1, 2]);
    }

    #[test]
    fn deserialize_get_post_api_result() {
        let json = r#"{
            "hasMorePosts": true,
            "paginationStart": "2024-01-01T00:00:00",
            "paginationEnd": "2024-03-15T23:59:59",
            "page": 1,
            "posts": []
        }"#;
        let r: GetPostApiResult = serde_json::from_str(json).unwrap();
        assert!(r.has_more_posts);
        assert_eq!(r.page, Some(1));
    }

    #[test]
    fn deserialize_comment_result_model() {
        let json = r#"{
            "id": 10,
            "creator": {
                "profileId": 33,
                "institutionProfileId": 55,
                "institutionCode": "101001",
                "institutionName": "Test Skole",
                "name": "Lars Hansen",
                "role": "employee",
                "mainGroup": "3.A",
                "profilePicture": null,
                "shortName": "LH",
                "metadata": null
            },
            "content": "Great idea!",
            "createdAt": "2024-03-15T09:00:00",
            "updatedAt": null,
            "comments": [],
            "commentCount": 0,
            "canDelete": true,
            "canReport": false,
            "isDeleted": false,
            "isReported": false
        }"#;
        let c: CommentResultModel = serde_json::from_str(json).unwrap();
        assert_eq!(c.id, Some(10));
        assert_eq!(c.content.as_deref(), Some("Great idea!"));
        assert!(c.can_delete);
        assert!(!c.is_deleted);
        let creator = c.creator.unwrap();
        assert_eq!(creator.name.as_deref(), Some("Lars Hansen"));
    }

    #[test]
    fn deserialize_paged_comment_list() {
        let json = r#"{
            "startIndex": 0,
            "limit": 20,
            "totalResultCount": 5,
            "comments": [],
            "commentableInstitutionProfiles": []
        }"#;
        let p: PagedCommentList = serde_json::from_str(json).unwrap();
        assert_eq!(p.total_result_count, Some(5));
        assert_eq!(p.limit, Some(20));
    }

    #[test]
    fn deserialize_comment_item() {
        let json = r#"{"type": "Post", "id": 42}"#;
        let c: CommentItem = serde_json::from_str(json).unwrap();
        assert_eq!(c.comment_type, Some(CommentType::Post));
        assert_eq!(c.id, Some(42));
    }

    #[test]
    fn serialize_get_post_api_parameters() {
        let params = GetPostApiParameters {
            group_id: Some(5),
            is_important: Some(true),
            creator_portal_role: None,
            institution_profile_ids: Some(vec![55]),
            related_institutions: None,
            own_post: false,
            is_unread: false,
            is_bookmarked: true,
            limit: Some(20),
            index: Some(0),
        };
        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("\"groupId\":5"));
        assert!(json.contains("\"isBookmarked\":true"));
    }
}
