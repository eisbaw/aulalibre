//! Post service.
//!
//! Maps to `AulaNative.Services.Web.PostWebService` (8 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.8.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_posts` | GET | `/posts` |
//! | `get_post_by_id` | GET | `/posts/{id}` |
//! | `create_post` | POST | `/posts` |
//! | `edit_post` | PUT | `/posts/{id}` |
//! | `delete_post` | DELETE | `/posts/{id}` |
//! | `report_post` | POST | `/posts/{id}/report` |
//! | `bookmark_post` | POST | `/posts/{id}/bookmark` |
//! | `unbookmark_post` | DELETE | `/posts/{id}/bookmark` |

use crate::models::posts::{
    CreatePostApiParameter, CreatePostResult, GetPostApiParameters, GetPostApiResult, PostApiDto,
    ReportApiParameter,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch posts matching the given filter/pagination parameters.
///
/// Maps to `PostWebService.GetPosts()`.
///
/// # Endpoint (inferred)
///
/// `GET /posts?<query params>`
///
/// The filter struct is serialised as query parameters. If the API
/// actually expects a POST body, this will need adjustment.
pub async fn get_posts(
    session: &mut Session,
    params: &GetPostApiParameters,
) -> crate::Result<GetPostApiResult> {
    let mut query = Vec::new();
    if let Some(gid) = params.group_id {
        query.push(format!("groupId={gid}"));
    }
    if let Some(imp) = params.is_important {
        query.push(format!("isImportant={imp}"));
    }
    if let Some(ref role) = params.creator_portal_role {
        query.push(format!("creatorPortalRole={role}"));
    }
    if let Some(ref ids) = params.institution_profile_ids {
        for id in ids {
            query.push(format!("institutionProfileIds={id}"));
        }
    }
    if let Some(ref insts) = params.related_institutions {
        for inst in insts {
            query.push(format!("relatedInstitutions={inst}"));
        }
    }
    if params.own_post {
        query.push("ownPost=true".to_string());
    }
    if params.is_unread {
        query.push("isUnread=true".to_string());
    }
    if params.is_bookmarked {
        query.push("isBookmarked=true".to_string());
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(index) = params.index {
        query.push(format!("index={index}"));
    }

    let path = if query.is_empty() {
        "posts".to_string()
    } else {
        format!("posts?{}", query.join("&"))
    };

    session.get(&path).await
}

/// Fetch a single post by its ID.
///
/// Maps to `PostWebService.GetPostById()`.
///
/// # Endpoint (inferred)
///
/// `GET /posts/{id}`
pub async fn get_post_by_id(session: &mut Session, post_id: i64) -> crate::Result<PostApiDto> {
    session.get(&format!("posts/{post_id}")).await
}

/// Create a new post.
///
/// Maps to `PostWebService.CreatePost()`.
///
/// # Endpoint (inferred)
///
/// `POST /posts`
pub async fn create_post(
    session: &mut Session,
    params: &CreatePostApiParameter,
) -> crate::Result<CreatePostResult> {
    session.post("posts", params).await
}

/// Edit an existing post.
///
/// Maps to `PostWebService.EditPost()`.
///
/// # Endpoint (inferred)
///
/// `PUT /posts/{id}`
///
/// NOTE: The `id` field inside `params` may be redundant with the path
/// parameter. Both are sent to match the likely .NET routing convention.
pub async fn edit_post(
    session: &mut Session,
    post_id: i64,
    params: &CreatePostApiParameter,
) -> crate::Result<serde_json::Value> {
    session.put(&format!("posts/{post_id}"), params).await
}

/// Delete a post.
///
/// Maps to `PostWebService.DeletePost()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /posts/{id}`
pub async fn delete_post(session: &mut Session, post_id: i64) -> crate::Result<serde_json::Value> {
    session.delete(&format!("posts/{post_id}")).await
}

/// Report a post (flag for moderation).
///
/// Maps to `PostWebService.ReportPost()`.
///
/// # Endpoint (inferred)
///
/// `POST /posts/{id}/report`
pub async fn report_post(
    session: &mut Session,
    post_id: i64,
    params: &ReportApiParameter,
) -> crate::Result<serde_json::Value> {
    session
        .post(&format!("posts/{post_id}/report"), params)
        .await
}

/// Bookmark a post for the current user.
///
/// Maps to `PostWebService.BookmarkPost()`.
///
/// # Endpoint (inferred)
///
/// `POST /posts/{id}/bookmark`
pub async fn bookmark_post(
    session: &mut Session,
    post_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .post_empty(&format!("posts/{post_id}/bookmark"))
        .await
}

/// Remove bookmark from a post.
///
/// Maps to `PostWebService.UnbookmarkPost()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /posts/{id}/bookmark`
pub async fn unbookmark_post(
    session: &mut Session,
    post_id: i64,
) -> crate::Result<serde_json::Value> {
    session.delete(&format!("posts/{post_id}/bookmark")).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_posts_builds_query_string() {
        // Verify query-string building logic by testing the params struct
        // serialization. We cannot call the async function without a session,
        // so we test the parameter types round-trip instead.
        let params = GetPostApiParameters {
            group_id: Some(5),
            is_important: Some(true),
            creator_portal_role: None,
            institution_profile_ids: Some(vec![10, 20]),
            related_institutions: None,
            own_post: false,
            is_unread: false,
            is_bookmarked: true,
            limit: Some(20),
            index: Some(0),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["groupId"], 5);
        assert_eq!(json["isBookmarked"], true);
        assert_eq!(json["limit"], 20);
    }

    #[test]
    fn create_post_params_serialize() {
        let params = CreatePostApiParameter {
            id: None,
            title: Some("Test".into()),
            content: Some("<p>Hello</p>".into()),
            institution_code: Some("101001".into()),
            creator_institution_profile_id: Some(42),
            allow_comments: true,
            is_important: false,
            important_from: None,
            important_to: None,
            shared_with_groups: None,
            attachment_ids: None,
            publish_at: None,
            expire_at: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["title"], "Test");
        assert_eq!(json["allowComments"], true);
        assert!(json["id"].is_null());
    }

    #[test]
    fn report_params_serialize() {
        let params = ReportApiParameter {
            id: Some(42),
            report_reason: Some("Inappropriate content".into()),
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["id"], 42);
        assert_eq!(json["reportReason"], "Inappropriate content");
    }
}
