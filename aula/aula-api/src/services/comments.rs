//! Comment service.
//!
//! Maps to `AulaNative.Services.Web.CommentService` (5 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `ADD_COMMENT` | `comments.addComment` |
//! | `GET_COMMENTS` | `comments.getComments` |
//! | `REPORT_COMMENT` | `comments.reportComment` |
//! | `DELETE_COMMENT` | `comments.deleteComment` |
//! | `UPDATE_COMMENT` | `comments.updateComment` |

use serde::{Deserialize, Serialize};

use crate::enums::common::CommentType;
use crate::models::posts::{
    CommentItem, DeleteCommentRequestModel, PagedCommentList, ReportCommentApiParameters,
    UpdateCommentRequestModel,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Request types specific to this service
// ---------------------------------------------------------------------------

/// Request body for `AddComment`.
///
/// Maps to `AulaNative.Services.Web.AddCommentRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentRequestModel {
    /// The item being commented on (type + id).
    pub commentable_item: CommentItem,
    /// Comment text content.
    pub content: String,
    /// Institution profile ID of the comment creator.
    pub creator_inst_profile_id: i64,
}

/// Query parameters for fetching comments.
///
/// Maps to `AulaNative.Services.Web.GetCommmentsRequestModel`
/// (note: triple-m in original decompiled name).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentsRequestModel {
    /// Type of the parent (Post, Media, etc.).
    pub parent_type: CommentType,
    /// ID of the parent item.
    pub parent_id: i64,
    /// Pagination start index.
    pub start_index: Option<i32>,
    /// Maximum number of comments to return.
    pub limit: Option<i32>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Add a comment to a post, media item, or other commentable entity.
///
/// Maps to `CommentService.AddComment()`.
///
/// # Endpoint
///
/// `POST ?method=comments.addComment`
pub async fn add_comment(
    session: &mut Session,
    request: &AddCommentRequestModel,
) -> crate::Result<serde_json::Value> {
    session.post("?method=comments.addComment", request).await
}

/// Update an existing comment's content.
///
/// Maps to `CommentService.UpdateComment()`.
///
/// # Endpoint
///
/// `POST ?method=comments.updateComment`
pub async fn update_comment(
    session: &mut Session,
    _comment_id: i64,
    request: &UpdateCommentRequestModel,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=comments.updateComment", request)
        .await
}

/// Fetch comments for a given parent item (post, media, etc.).
///
/// Maps to `CommentService.GetComments()`.
///
/// # Endpoint
///
/// `GET ?method=comments.getComments&parentType=...&parentId=...`
pub async fn get_comments(
    session: &mut Session,
    params: &GetCommentsRequestModel,
) -> crate::Result<PagedCommentList> {
    let parent_type_str = serde_json::to_string(&params.parent_type)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();

    let mut query = vec![
        format!("parentType={parent_type_str}"),
        format!("parentId={}", params.parent_id),
    ];
    if let Some(start) = params.start_index {
        query.push(format!("startIndex={start}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }

    let path = format!("?method=comments.getComments&{}", query.join("&"));
    session.get(&path).await
}

/// Report a comment for moderation.
///
/// Maps to `CommentService.ReportComment()`.
///
/// # Endpoint
///
/// `POST ?method=comments.reportComment`
pub async fn report_comment(
    session: &mut Session,
    _comment_id: i64,
    params: &ReportCommentApiParameters,
) -> crate::Result<serde_json::Value> {
    session.post("?method=comments.reportComment", params).await
}

/// Delete a comment.
///
/// Maps to `CommentService.DeleteComment()`.
///
/// # Endpoint
///
/// `POST ?method=comments.deleteComment`
pub async fn delete_comment(
    session: &mut Session,
    _comment_id: i64,
    request: &DeleteCommentRequestModel,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=comments.deleteComment", request)
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_comment_request_serializes() {
        let req = AddCommentRequestModel {
            commentable_item: CommentItem {
                comment_type: Some(CommentType::Post),
                id: Some(42),
            },
            content: "Great post!".into(),
            creator_inst_profile_id: 55,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["content"], "Great post!");
        assert_eq!(json["creatorInstProfileId"], 55);
        assert_eq!(json["commentableItem"]["type"], "Post");
        assert_eq!(json["commentableItem"]["id"], 42);
    }

    #[test]
    fn get_comments_request_serializes() {
        let req = GetCommentsRequestModel {
            parent_type: CommentType::Post,
            parent_id: 42,
            start_index: Some(0),
            limit: Some(20),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["parentType"], "Post");
        assert_eq!(json["parentId"], 42);
        assert_eq!(json["startIndex"], 0);
        assert_eq!(json["limit"], 20);
    }

    #[test]
    fn report_comment_params_serialize() {
        let params = ReportCommentApiParameters {
            comment_id: Some(10),
            report_reason: Some("Spam".into()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["commentId"], 10);
        assert_eq!(json["reportReason"], "Spam");
    }

    #[test]
    fn delete_comment_request_serializes() {
        let req = DeleteCommentRequestModel {
            comment_id: Some(10),
            parent_type: Some(CommentType::Post),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["commentId"], 10);
        assert_eq!(json["parentType"], "Post");
    }
}
