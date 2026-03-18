//! Messaging service.
//!
//! Maps to `AulaNative.Services.Web.MessagingWebService` (26 methods) and
//! `AulaNative.Services.Web.FolderService` (6 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Sections 3.7 and 3.13.
//!
//! ## MessagingWebService (Section 3.7)
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_thread_list` | GET | `/messaging/threads` |
//! | `get_thread_by_id` | GET | `/messaging/threads/{id}` |
//! | `start_new_thread` | POST | `/messaging/threads` |
//! | `reply_to_thread` | POST | `/messaging/threads/{id}/reply` |
//! | `reply_in_new_thread` | POST | `/messaging/threads/{id}/replyNew` |
//! | `forward_thread` | POST | `/messaging/threads/{id}/forward` |
//! | `delete_threads` | DELETE | `/messaging/threads` |
//! | `leave_thread` | POST | `/messaging/threads/{id}/leave` |
//! | `leave_threads` | POST | `/messaging/threads/leave` |
//! | `add_recipients_to_thread` | POST | `/messaging/threads/{id}/recipients` |
//! | `get_message_list` | GET | `/messaging/threads/{id}/messages` |
//! | `get_message_info_light` | GET | `/messaging/messages/{id}/info` |
//! | `delete_message` | DELETE | `/messaging/messages/{id}` |
//! | `edit_message` | PUT | `/messaging/messages/{id}` |
//! | `set_last_read_message` | PUT | `/messaging/threads/{id}/lastRead` |
//! | `set_thread_muted` | PUT | `/messaging/threads/{id}/muted` |
//! | `set_thread_marked` | PUT | `/messaging/threads/{id}/marked` |
//! | `set_sensitive_level` | PUT | `/messaging/threads/{id}/sensitive` |
//! | `set_auto_reply` | POST | `/messaging/autoReply` |
//! | `get_auto_reply` | GET | `/messaging/autoReply` |
//! | `delete_auto_reply` | DELETE | `/messaging/autoReply` |
//! | `get_threads_in_bundle_list` | GET | `/messaging/threads/bundle` |
//! | `set_subscription_status` | PUT | `/messaging/threads/subscription` |
//! | `check_recipients_for_blocked_channels` | POST | `/messaging/recipients/blocked` |
//! | `attach_messages_to_secure_document` | POST | `/messaging/messages/attachToDocument` |
//! | `send_event_reminder` | POST | `/messaging/eventReminder` |
//!
//! ## FolderService (Section 3.13)
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_folders` | GET | `/messaging/folders` |
//! | `create_folder` | POST | `/messaging/folders` |
//! | `update_folder` | PUT | `/messaging/folders/{id}` |
//! | `delete_folder` | POST | `/messaging/folders/{id}` |
//! | `move_threads_to_folder` | POST | `/messaging/folders/{id}/moveThreads` |
//! | `get_common_inboxes` | GET | `/messaging/commonInboxes` |

use serde::{Deserialize, Serialize};

use crate::models::messaging::{
    AddRecipientArguments, AttachMessagesToSecureDocumentRequest, CommonInboxesDto,
    CreateFolderArguments, DeleteMessageRequest, DeleteThreadArguments, EditMessageRequest, Folder,
    ForwardThreadRequestArguments, GetFoldersArguments, GetMessageInfoLightDto,
    GetMessagesForThreadArguments, GetThreadListArguments, GetThreadsInBundleArguments,
    LeaveThreadArguments, LeaveThreadsRequest, MarkThreadsRequest, MessageAutoReplyResult,
    MessageThreadSubscriptionList, MessagesInThreadDto, MoveThreadsToFolderRequestArguments,
    MuteThreadRequestArguments, RecipientApiModel, ReplyMessageArgument, SetAutoReplyArguments,
    SetLastMessageRequestArguments, SetSensitivityLevelRequest, StartNewThreadRequestArguments,
    UpdateFolderArguments, UpdateMessageThreadsSubscriptionStatusRequest,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Response from `StartNewThread` / `ReplyInNewThread` / `ForwardThread`.
///
/// The API likely returns the thread ID of the newly created thread.
/// Using `serde_json::Value` since the exact shape is unverified.
pub type NewThreadResponse = serde_json::Value;

/// Response from `ReplyToThread`.
///
/// The API likely returns the new message ID or a confirmation.
pub type ReplyResponse = serde_json::Value;

/// Response from `EditMessage`.
pub type EditMessageResponse = serde_json::Value;

/// Response from `DeleteMessage`.
pub type DeleteMessageResponse = serde_json::Value;

/// Response from `DeleteThreads`.
pub type DeleteThreadsResponse = serde_json::Value;

/// Response from `LeaveThread` / `LeaveThreads`.
pub type LeaveThreadResponse = serde_json::Value;

/// Response from `AddRecipientsToThread`.
pub type AddRecipientsResponse = serde_json::Value;

/// Response from `SetThreadMuted`.
pub type SetMutedResponse = serde_json::Value;

/// Response from `SetThreadMarked`.
pub type SetMarkedResponse = serde_json::Value;

/// Response from `SetSensitiveLevel`.
pub type SetSensitiveLevelResponse = serde_json::Value;

/// Response from `SetLastReadMessage`.
pub type SetLastReadResponse = serde_json::Value;

/// Response from `DeleteAutoReply`.
pub type DeleteAutoReplyResponse = serde_json::Value;

/// Response from folder mutations.
pub type FolderMutationResponse = serde_json::Value;

/// Response from `SetMessageThreadsSubscriptionStatus`.
pub type SetSubscriptionStatusResponse = serde_json::Value;

/// Response from `CheckRecipientsForBlockedChannels`.
///
/// Returns a list of blocked recipient identifiers.
pub type BlockedChannelsResponse = serde_json::Value;

/// Response from `AttachMessagesToSecureDocument`.
pub type AttachToDocumentResponse = serde_json::Value;

/// Request body for `SendEventReminder`.
///
/// Inferred from `MessagingWebService.SendEventReminder()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEventReminderRequest {
    /// Thread ID for the event reminder.
    pub thread_id: Option<i64>,
    /// Event entity ID.
    pub entity_id: Option<String>,
}

/// Response from `SendEventReminder`.
pub type SendEventReminderResponse = serde_json::Value;

// ===========================================================================
// Thread CRUD (AC #1)
// ===========================================================================

/// List message threads (inbox view) with filtering, sorting, and pagination.
///
/// Maps to `MessagingWebService.GetThreadList()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/threads`
///
/// NOTE: The GET request sends query parameters derived from
/// `GetThreadListArguments`. Since the argument struct has many fields,
/// the API likely accepts them as query parameters. However, some .NET
/// APIs accept complex filter objects via POST. If this fails, try
/// `POST /messaging/threads/list` instead.
pub async fn get_thread_list(
    session: &mut Session,
    args: &GetThreadListArguments,
) -> crate::Result<MessageThreadSubscriptionList> {
    // Build query string from the arguments.
    let mut params = Vec::new();
    if let Some(page) = args.page {
        params.push(format!("page={page}"));
    }
    if let Some(folder_id) = args.folder_id {
        params.push(format!("folderId={folder_id}"));
    }
    if let Some(ref ft) = args.filter_type {
        params.push(format!(
            "filterType={}",
            serde_json::to_string(ft).unwrap().trim_matches('"')
        ));
    }
    if let Some(ref st) = args.sort_type {
        params.push(format!(
            "sortType={}",
            serde_json::to_string(st).unwrap().trim_matches('"')
        ));
    }
    if let Some(ref so) = args.sort_order {
        params.push(format!(
            "sortOrder={}",
            serde_json::to_string(so).unwrap().trim_matches('"')
        ));
    }
    if let Some(ref mbot) = args.mail_box_owner_type {
        params.push(format!(
            "mailBoxOwnerType={}",
            serde_json::to_string(mbot).unwrap().trim_matches('"')
        ));
    }
    if let Some(ref owners) = args.mail_box_owners {
        for id in owners {
            params.push(format!("mailBoxOwners={id}"));
        }
    }
    if let Some(ref children) = args.active_children {
        for id in children {
            params.push(format!("activeChildren={id}"));
        }
    }
    if let Some(ref tids) = args.thread_ids {
        for id in tids {
            params.push(format!("threadIds={id}"));
        }
    }

    let path = if params.is_empty() {
        "messaging/threads".to_string()
    } else {
        format!("messaging/threads?{}", params.join("&"))
    };
    session.get(&path).await
}

/// Get a specific thread by ID with its messages.
///
/// Maps to `MessagingWebService.GetThreadById()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/threads/{thread_id}/messages`
///
/// NOTE: `GetThreadById` and `GetMessageList` may share the same
/// endpoint with different pagination. This method fetches the first
/// page of messages for the thread.
pub async fn get_thread_by_id(
    session: &mut Session,
    args: &GetMessagesForThreadArguments,
) -> crate::Result<MessagesInThreadDto> {
    let thread_id = args.thread_id.unwrap_or(0);
    let mut path = format!("messaging/threads/{thread_id}/messages");
    let mut params = Vec::new();
    if let Some(page) = args.page {
        params.push(format!("page={page}"));
    }
    if let Some(cid) = args.common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    if !params.is_empty() {
        path.push('?');
        path.push_str(&params.join("&"));
    }
    session.get(&path).await
}

/// Start a new message thread.
///
/// Maps to `MessagingWebService.StartNewThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads`
pub async fn start_new_thread(
    session: &mut Session,
    args: &StartNewThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    session.post("messaging/threads", args).await
}

/// Reply to an existing thread.
///
/// Maps to `MessagingWebService.ReplyToThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/{thread_id}/reply`
pub async fn reply_to_thread(
    session: &mut Session,
    args: &ReplyMessageArgument,
) -> crate::Result<ReplyResponse> {
    let thread_id = args.thread_id.unwrap_or(0);
    session
        .post(&format!("messaging/threads/{thread_id}/reply"), args)
        .await
}

/// Delete one or more threads.
///
/// Maps to `MessagingWebService.DeleteThreads()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /messaging/threads`
///
/// NOTE: This endpoint uses DELETE with a JSON body containing
/// subscription and thread IDs.
pub async fn delete_threads(
    session: &mut Session,
    args: &DeleteThreadArguments,
) -> crate::Result<DeleteThreadsResponse> {
    session.delete_with_body("messaging/threads", args).await
}

/// Leave a single thread.
///
/// Maps to `MessagingWebService.LeaveThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/{thread_id}/leave`
pub async fn leave_thread(
    session: &mut Session,
    args: &LeaveThreadArguments,
) -> crate::Result<LeaveThreadResponse> {
    let thread_id = args.thread_id.unwrap_or(0);
    session
        .post(&format!("messaging/threads/{thread_id}/leave"), args)
        .await
}

/// Leave multiple threads at once.
///
/// Maps to `MessagingWebService.LeaveThreads()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/leave`
pub async fn leave_threads(
    session: &mut Session,
    args: &LeaveThreadsRequest,
) -> crate::Result<LeaveThreadResponse> {
    session.post("messaging/threads/leave", args).await
}

/// Forward a thread to new recipients.
///
/// Maps to `MessagingWebService.ForwardThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/{thread_id}/forward`
///
/// NOTE: The thread ID to forward is embedded in `forward_info.forwarded_thread_id`.
/// The URL path may use that same ID.
pub async fn forward_thread(
    session: &mut Session,
    args: &ForwardThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    let thread_id = args
        .forward_info
        .as_ref()
        .and_then(|fi| fi.forwarded_thread_id)
        .unwrap_or(0);
    session
        .post(&format!("messaging/threads/{thread_id}/forward"), args)
        .await
}

/// Reply to a thread by creating a new thread (quote-reply).
///
/// Maps to `MessagingWebService.ReplyInNewThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/{thread_id}/replyNew`
///
/// NOTE: The original thread ID is likely taken from
/// `forward_info.forwarded_thread_id` similar to `forward_thread`.
pub async fn reply_in_new_thread(
    session: &mut Session,
    args: &ForwardThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    let thread_id = args
        .forward_info
        .as_ref()
        .and_then(|fi| fi.forwarded_thread_id)
        .unwrap_or(0);
    session
        .post(&format!("messaging/threads/{thread_id}/replyNew"), args)
        .await
}

// ===========================================================================
// Message operations (AC #2)
// ===========================================================================

/// Get messages in a thread (paginated).
///
/// Maps to `MessagingWebService.GetMessageList()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/threads/{thread_id}/messages`
pub async fn get_message_list(
    session: &mut Session,
    args: &GetMessagesForThreadArguments,
) -> crate::Result<MessagesInThreadDto> {
    // Same endpoint as get_thread_by_id; both return MessagesInThreadDto.
    get_thread_by_id(session, args).await
}

/// Get lightweight message info (for notifications/previews).
///
/// Maps to `MessagingWebService.GetMessageInfoLight()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/messages/{message_id}/info`
pub async fn get_message_info_light(
    session: &mut Session,
    message_id: &str,
    common_inbox_id: Option<i64>,
    otp_inbox_id: Option<i64>,
) -> crate::Result<GetMessageInfoLightDto> {
    let mut params = Vec::new();
    if let Some(cid) = common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    if let Some(oid) = otp_inbox_id {
        params.push(format!("otpInboxId={oid}"));
    }
    let path = if params.is_empty() {
        format!("messaging/messages/{message_id}/info")
    } else {
        format!("messaging/messages/{message_id}/info?{}", params.join("&"))
    };
    session.get(&path).await
}

/// Delete a message from a thread.
///
/// Maps to `MessagingWebService.DeleteMessage()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /messaging/messages/{message_id}`
pub async fn delete_message(
    session: &mut Session,
    args: &DeleteMessageRequest,
) -> crate::Result<DeleteMessageResponse> {
    let message_id = args.message_id.as_deref().unwrap_or("0");
    let mut path = format!("messaging/messages/{message_id}");
    if let Some(tid) = args.thread_id {
        path.push_str(&format!("?threadId={tid}"));
    }
    session.delete(&path).await
}

/// Edit an existing message.
///
/// Maps to `MessagingWebService.EditMessage()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/messages/{message_id}`
pub async fn edit_message(
    session: &mut Session,
    args: &EditMessageRequest,
) -> crate::Result<EditMessageResponse> {
    let message_id = args.message_id.as_deref().unwrap_or("0");
    session
        .put(&format!("messaging/messages/{message_id}"), args)
        .await
}

/// Mark a message as the last read in a thread.
///
/// Maps to `MessagingWebService.SetLastReadMessage()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/threads/{thread_id}/lastRead`
pub async fn set_last_read_message(
    session: &mut Session,
    args: &SetLastMessageRequestArguments,
) -> crate::Result<SetLastReadResponse> {
    let thread_id = args.thread_id.unwrap_or(0);
    session
        .put(&format!("messaging/threads/{thread_id}/lastRead"), args)
        .await
}

// ===========================================================================
// Thread management (AC #3)
// ===========================================================================

/// Mute or unmute a thread.
///
/// Maps to `MessagingWebService.SetThreadMuted()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/threads/{thread_id}/muted`
///
/// NOTE: The thread ID in the URL may be redundant since the body
/// contains `thread_ids`. The API may accept either form.
pub async fn set_thread_muted(
    session: &mut Session,
    args: &MuteThreadRequestArguments,
) -> crate::Result<SetMutedResponse> {
    session.put("messaging/threads/muted", args).await
}

/// Mark or unmark a thread (star/flag).
///
/// Maps to `MessagingWebService.SetThreadMarked()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/threads/{thread_id}/marked`
///
/// NOTE: Similar to muted, the body contains `thread_ids` so the URL
/// may not need a specific thread ID.
pub async fn set_thread_marked(
    session: &mut Session,
    args: &MarkThreadsRequest,
) -> crate::Result<SetMarkedResponse> {
    session.put("messaging/threads/marked", args).await
}

/// Set the sensitivity level on a thread.
///
/// Maps to `MessagingWebService.SetSensitiveLevel()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/threads/{thread_id}/sensitive`
pub async fn set_sensitive_level(
    session: &mut Session,
    args: &SetSensitivityLevelRequest,
) -> crate::Result<SetSensitiveLevelResponse> {
    let thread_id = args.thread_id.unwrap_or(0);
    session
        .put(&format!("messaging/threads/{thread_id}/sensitive"), args)
        .await
}

/// Add recipients to an existing thread.
///
/// Maps to `MessagingWebService.AddRecipientsToThread()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/threads/{thread_id}/recipients`
pub async fn add_recipients_to_thread(
    session: &mut Session,
    args: &AddRecipientArguments,
) -> crate::Result<AddRecipientsResponse> {
    let thread_id = args.thread_id.unwrap_or(0);
    session
        .post(&format!("messaging/threads/{thread_id}/recipients"), args)
        .await
}

// ===========================================================================
// Auto-reply (AC #4)
// ===========================================================================

/// Set an auto-reply message.
///
/// Maps to `MessagingWebService.SetAutoReply()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/autoReply`
pub async fn set_auto_reply(
    session: &mut Session,
    args: &SetAutoReplyArguments,
) -> crate::Result<MessageAutoReplyResult> {
    session.post("messaging/autoReply", args).await
}

/// Get the current auto-reply configuration.
///
/// Maps to `MessagingWebService.GetAutoReply()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/autoReply`
pub async fn get_auto_reply(session: &mut Session) -> crate::Result<MessageAutoReplyResult> {
    session.get("messaging/autoReply").await
}

/// Delete the auto-reply configuration.
///
/// Maps to `MessagingWebService.DeleteAutoReply()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /messaging/autoReply`
pub async fn delete_auto_reply(session: &mut Session) -> crate::Result<DeleteAutoReplyResponse> {
    session.delete("messaging/autoReply").await
}

// ===========================================================================
// Folder management (AC #5)
// ===========================================================================

/// Get folders for the current user's mailbox.
///
/// Maps to `FolderService.GetFolders()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/folders`
pub async fn get_folders(
    session: &mut Session,
    args: &GetFoldersArguments,
) -> crate::Result<Vec<Folder>> {
    let mut params = Vec::new();
    if args.include_deleted_folders {
        params.push("includeDeletedFolders=true".to_string());
    }
    if let Some(cid) = args.common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    let path = if params.is_empty() {
        "messaging/folders".to_string()
    } else {
        format!("messaging/folders?{}", params.join("&"))
    };
    session.get(&path).await
}

/// Create a new message folder.
///
/// Maps to `FolderService.CreateFolder()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/folders`
pub async fn create_folder(
    session: &mut Session,
    args: &CreateFolderArguments,
) -> crate::Result<FolderMutationResponse> {
    session.post("messaging/folders", args).await
}

/// Rename a folder.
///
/// Maps to `FolderService.UpdateFolder()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/folders/{folder_id}`
pub async fn update_folder(
    session: &mut Session,
    args: &UpdateFolderArguments,
) -> crate::Result<FolderMutationResponse> {
    let folder_id = args.folder_id.unwrap_or(0);
    session
        .put(&format!("messaging/folders/{folder_id}"), args)
        .await
}

/// Delete a folder.
///
/// Maps to `FolderService.PostDeleteFolder()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/folders/{folder_id}` (delete via POST)
///
/// NOTE: The decompiled method is `PostDeleteFolder`, suggesting the
/// API uses POST rather than DELETE for folder deletion. This may be
/// a soft-delete pattern.
pub async fn delete_folder(
    session: &mut Session,
    folder_id: i64,
    common_inbox_id: Option<i64>,
) -> crate::Result<FolderMutationResponse> {
    let mut path = format!("messaging/folders/{folder_id}/delete");
    if let Some(cid) = common_inbox_id {
        path.push_str(&format!("?commonInboxId={cid}"));
    }
    session.post_empty(&path).await
}

/// Move threads to a folder.
///
/// Maps to `FolderService.MoveThreadsToFolder()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/folders/{folder_id}/moveThreads`
pub async fn move_threads_to_folder(
    session: &mut Session,
    args: &MoveThreadsToFolderRequestArguments,
) -> crate::Result<FolderMutationResponse> {
    let folder_id = args.folder_id.unwrap_or(0);
    session
        .post(&format!("messaging/folders/{folder_id}/moveThreads"), args)
        .await
}

/// Get common (shared) inboxes for the user's institution profiles.
///
/// Maps to `FolderService.GetCommonInboxes()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/commonInboxes`
///
/// NOTE: The query parameters are derived from `GetCommonInboxesArguments`.
/// Institution profile IDs are passed as repeated query params.
pub async fn get_common_inboxes(
    session: &mut Session,
    institution_profile_ids: &[i64],
    include_profile_picture_url: bool,
) -> crate::Result<Vec<CommonInboxesDto>> {
    let mut params = Vec::new();
    for id in institution_profile_ids {
        params.push(format!("institutionProfileIds={id}"));
    }
    if include_profile_picture_url {
        params.push("shouldIncludeProfilePictureUrl=true".to_string());
    }
    let path = if params.is_empty() {
        "messaging/commonInboxes".to_string()
    } else {
        format!("messaging/commonInboxes?{}", params.join("&"))
    };
    session.get(&path).await
}

// ===========================================================================
// Bundle / subscription management (AC #6)
// ===========================================================================

/// Get threads within a bundle (grouped threads).
///
/// Maps to `MessagingWebService.GetThreadsInBundleList()`.
///
/// # Endpoint (inferred)
///
/// `GET /messaging/threads/bundle?bundleId={id}`
pub async fn get_threads_in_bundle_list(
    session: &mut Session,
    args: &GetThreadsInBundleArguments,
) -> crate::Result<MessageThreadSubscriptionList> {
    let bundle_id = args.bundle_id.unwrap_or(0);
    session
        .get(&format!("messaging/threads/bundle?bundleId={bundle_id}"))
        .await
}

/// Update subscription status (read/unread) for thread subscriptions.
///
/// Maps to `MessagingWebService.SetMessageThreadsSubscriptionStatus()`.
///
/// # Endpoint (inferred)
///
/// `PUT /messaging/threads/subscription`
pub async fn set_subscription_status(
    session: &mut Session,
    args: &UpdateMessageThreadsSubscriptionStatusRequest,
) -> crate::Result<SetSubscriptionStatusResponse> {
    session.put("messaging/threads/subscription", args).await
}

/// Check whether recipients have blocked messaging channels.
///
/// Maps to `MessagingWebService.CheckRecipientsForBlockedChannels()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/recipients/blocked`
pub async fn check_recipients_for_blocked_channels(
    session: &mut Session,
    recipients: &[RecipientApiModel],
) -> crate::Result<BlockedChannelsResponse> {
    session
        .post("messaging/recipients/blocked", &recipients)
        .await
}

/// Attach messages from a thread to a secure document.
///
/// Maps to `MessagingWebService.AttachMessagesToSecureDocument()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/messages/attachToDocument`
pub async fn attach_messages_to_secure_document(
    session: &mut Session,
    args: &AttachMessagesToSecureDocumentRequest,
) -> crate::Result<AttachToDocumentResponse> {
    session
        .post("messaging/messages/attachToDocument", args)
        .await
}

/// Send an event reminder message.
///
/// Maps to `MessagingWebService.SendEventReminder()`.
///
/// # Endpoint (inferred)
///
/// `POST /messaging/eventReminder`
pub async fn send_event_reminder(
    session: &mut Session,
    args: &SendEventReminderRequest,
) -> crate::Result<SendEventReminderResponse> {
    session.post("messaging/eventReminder", args).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::messaging::*;

    #[test]
    fn send_event_reminder_request_serializes() {
        let req = SendEventReminderRequest {
            thread_id: Some(42),
            entity_id: Some("event-123".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["threadId"], 42);
        assert_eq!(json["entityId"], "event-123");
    }

    #[test]
    fn get_thread_list_args_serializes() {
        let args = GetThreadListArguments {
            folder_id: Some(1),
            filter_type: None,
            sort_type: None,
            sort_order: None,
            page: Some(0),
            thread_ids: None,
            mail_box_owner_type: None,
            mail_box_owners: None,
            active_children: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["folderId"], 1);
        assert_eq!(json["page"], 0);
    }

    #[test]
    fn delete_thread_args_serializes() {
        let args = DeleteThreadArguments {
            subscription_ids: Some(vec![1, 2, 3]),
            thread_ids: Some(vec![10, 20]),
            common_inbox_id: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["subscriptionIds"], serde_json::json!([1, 2, 3]));
        assert_eq!(json["threadIds"], serde_json::json!([10, 20]));
    }

    #[test]
    fn reply_message_arg_serializes() {
        let args = ReplyMessageArgument {
            thread_id: Some(42),
            message: Some(MessageContentRequest {
                attachment_ids: Some(vec![]),
                text: Some("Reply text".to_string()),
            }),
            common_inbox_id: None,
            bundle_id: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["threadId"], 42);
        let msg = &json["message"];
        assert_eq!(msg["text"], "Reply text");
    }

    #[test]
    fn edit_message_request_serializes() {
        let req = EditMessageRequest {
            thread_id: Some(1),
            common_inbox_id: None,
            bundle_id: None,
            message_id: Some("msg-99".to_string()),
            message_request: Some(MessageContentRequest {
                attachment_ids: None,
                text: Some("Edited".to_string()),
            }),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["messageId"], "msg-99");
    }

    #[test]
    fn mute_thread_args_serializes() {
        let args = MuteThreadRequestArguments {
            muted: true,
            owner: None,
            subscription_ids: Some(vec![5]),
            common_inbox_id: None,
            thread_ids: Some(vec![10]),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert!(json["muted"].as_bool().unwrap());
        assert_eq!(json["threadIds"], serde_json::json!([10]));
    }

    #[test]
    fn mark_threads_request_serializes() {
        let args = MarkThreadsRequest {
            marked: true,
            thread_ids: Some(vec![1, 2]),
            subscription_ids: None,
            common_inbox_id: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert!(json["marked"].as_bool().unwrap());
    }

    #[test]
    fn set_auto_reply_args_serializes() {
        let args = SetAutoReplyArguments {
            reply_text: Some("Out of office".to_string()),
            start_date_time: Some("2024-01-20T00:00:00".to_string()),
            end_date_time: Some("2024-02-01T00:00:00".to_string()),
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["replyText"], "Out of office");
    }

    #[test]
    fn create_folder_args_serializes() {
        let args = CreateFolderArguments {
            folder_name: Some("Work".to_string()),
            common_inbox_id: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["folderName"], "Work");
    }

    #[test]
    fn move_threads_to_folder_args_serializes() {
        let args = MoveThreadsToFolderRequestArguments {
            thread_ids: Some(vec![1, 2]),
            subscription_ids: Some(vec![10, 20]),
            folder_id: Some(5),
            common_inbox_id: None,
        };
        let json = serde_json::to_value(&args).unwrap();
        assert_eq!(json["folderId"], 5);
    }

    #[test]
    fn subscription_status_request_serializes() {
        let req = UpdateMessageThreadsSubscriptionStatusRequest {
            subscription_ids: Some(vec![1, 2]),
            is_read: true,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json["isRead"].as_bool().unwrap());
    }

    #[test]
    fn attach_to_document_request_serializes() {
        let req = AttachMessagesToSecureDocumentRequest {
            secure_document_id: Some(100),
            message_ids: Some(vec!["msg-1".to_string(), "msg-2".to_string()]),
            thread_id: Some(42),
            common_inbox_id: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["secureDocumentId"], 100);
        assert_eq!(json["messageIds"], serde_json::json!(["msg-1", "msg-2"]));
    }
}
