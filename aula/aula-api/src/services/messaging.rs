//! Messaging service.
//!
//! Maps to `AulaNative.Services.Web.MessagingWebService` and
//! `AulaNative.Services.Web.FolderService` from the APK.
//!
//! # Endpoint paths
//!
//! All endpoints use RPC-style routing via `?method=messaging.<action>`.
//! Paths are sourced from the decompiled `Urls.cs` class.
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `REPLY_TO_THREAD` | `messaging.reply` |
//! | `START_NEW_THREAD` | `messaging.startNewThread` |
//! | `GET_THREADS` | `messaging.getThreads` |
//! | `MESSAGES_RESOURCE` | `messaging.getMessagesForThread` |
//! | `SET_LAST_READ_MESSAGE` | `messaging.setLastReadMessage` |
//! | `DELETE_THREAD` | `messaging.deleteThreads` |
//! | `LEAVE_THREAD` | `messaging.leaveThread` |
//! | `LEAVE_THREADS` | `messaging.leaveThreads` |
//! | `SET_THREAD_MUTED` | `messaging.setThreadsMuted` |
//! | `SET_THREAD_MARKED` | `messaging.setThreadsMarked` |
//! | `SET_AUTOREPLY` | `messaging.setAutoReply` |
//! | `GET_AUTOREPLY` | `messaging.getAutoReply` |
//! | `DELETE_AUTOREPLY` | `messaging.deleteAutoReply` |
//! | `SET_SENSITIVITY_LEVEL` | `messaging.setSensitivityLevel` |
//! | `FOLDERS_RESOURCE` | `messaging.getFolders` |
//! | `CREATE_FOLDER` | `messaging.createFolder` |
//! | `DELETE_FOLDER` | `messaging.deletefolder` |
//! | `UPDATE_FOLDER` | `messaging.updateFolder` |
//! | `MOVE_THREADS_TO_FOLDER` | `messaging.moveThreadsToFolder` |
//! | `ADD_RECIPIENTS_TO_THREAD` | `messaging.addRecipients` |
//! | `ATTACH_MESSAGES_TO_SECURE_DOCUMENT` | `messaging.attachMessagesToSecureDocument` |
//! | `GET_COMMON_INBOXES` | `messaging.getCommonInboxes` |
//! | `UPDATE_SUBSCRIPTION_STATUS` | `messaging.updateSubscriptionStatus` |
//! | `GET_THREADS_IN_BUNDLE` | `messaging.getThreadsInBundle` |
//! | `DELETE_MESSAGE` | `messaging.deleteMessage` |
//! | `EDIT_MESSAGE` | `messaging.editMessage` |
//! | `SEND_EVENT_REMINDER` | `messaging.sendEventReminder` |
//! | `GET_MESSAGE_INFO_LIGHT` | `messaging.getMessageInfoLight` |

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
use crate::services::query::encode_value;
use crate::session::Session;

/// Convert a serde-serializable enum variant to its string representation for
/// use in query parameters. Avoids the fragile
/// `serde_json::to_string(v).unwrap().trim_matches('"')` pattern.
fn enum_to_query_value<T: serde::Serialize>(value: &T) -> String {
    // Serialize to a JSON string (e.g., `"FilterAll"`), then strip the quotes.
    // This is still using serde under the hood, but the unwrap is justified:
    // these are simple fieldless enum variants that always serialize to strings.
    let json = serde_json::to_string(value).expect("fieldless enum serialization cannot fail");
    // Strip surrounding quotes from the JSON string value.
    json[1..json.len() - 1].to_string()
}

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Response from `StartNewThread` / `ReplyInNewThread` / `ForwardThread`.
pub type NewThreadResponse = serde_json::Value;

/// Response from `ReplyToThread`.
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
pub type BlockedChannelsResponse = serde_json::Value;

/// Response from `AttachMessagesToSecureDocument`.
pub type AttachToDocumentResponse = serde_json::Value;

/// Request body for `SendEventReminder`.
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
// Thread CRUD
// ===========================================================================

/// List message threads (inbox view) with filtering, sorting, and pagination.
///
/// Maps to `MessagingWebService.GetThreadList()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getThreads`
pub async fn get_thread_list(
    session: &mut Session,
    args: &GetThreadListArguments,
) -> crate::Result<MessageThreadSubscriptionList> {
    let mut params = vec!["method=messaging.getThreads".to_string()];
    // The `page` parameter is required by the API -- without it, the backend
    // returns code 40 (bad request). Default to page 0.
    params.push(format!("page={}", args.page.unwrap_or(0)));
    if let Some(folder_id) = args.folder_id {
        params.push(format!("folderId={folder_id}"));
    }
    if let Some(ref ft) = args.filter_type {
        params.push(format!("filterType={}", enum_to_query_value(ft)));
    }
    if let Some(ref st) = args.sort_type {
        params.push(format!("sortType={}", enum_to_query_value(st)));
    }
    if let Some(ref so) = args.sort_order {
        params.push(format!("sortOrder={}", enum_to_query_value(so)));
    }
    if let Some(ref mbot) = args.mail_box_owner_type {
        params.push(format!("mailBoxOwnerType={}", enum_to_query_value(mbot)));
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

    let path = format!("?{}", params.join("&"));
    session.get(&path).await
}

/// Get messages for a thread.
///
/// Maps to `MessagingWebService.GetMessageList()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getMessagesForThread`
pub async fn get_thread_by_id(
    session: &mut Session,
    args: &GetMessagesForThreadArguments,
) -> crate::Result<MessagesInThreadDto> {
    let mut params = vec!["method=messaging.getMessagesForThread".to_string()];
    if let Some(tid) = args.thread_id {
        params.push(format!("threadId={tid}"));
    }
    if let Some(page) = args.page {
        params.push(format!("page={page}"));
    }
    if let Some(cid) = args.common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    let path = format!("?{}", params.join("&"));
    session.get(&path).await
}

/// Start a new message thread.
///
/// Maps to `MessagingWebService.StartNewThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.startNewThread`
pub async fn start_new_thread(
    session: &mut Session,
    args: &StartNewThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    session.post("?method=messaging.startNewThread", args).await
}

/// Reply to an existing thread.
///
/// Maps to `MessagingWebService.ReplyToThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.reply`
pub async fn reply_to_thread(
    session: &mut Session,
    args: &ReplyMessageArgument,
) -> crate::Result<ReplyResponse> {
    session.post("?method=messaging.reply", args).await
}

/// Delete one or more threads.
///
/// Maps to `MessagingWebService.DeleteThreads()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.deleteThreads`
pub async fn delete_threads(
    session: &mut Session,
    args: &DeleteThreadArguments,
) -> crate::Result<DeleteThreadsResponse> {
    session.post("?method=messaging.deleteThreads", args).await
}

/// Leave a single thread.
///
/// Maps to `MessagingWebService.LeaveThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.leaveThread`
pub async fn leave_thread(
    session: &mut Session,
    args: &LeaveThreadArguments,
) -> crate::Result<LeaveThreadResponse> {
    session.post("?method=messaging.leaveThread", args).await
}

/// Leave multiple threads at once.
///
/// Maps to `MessagingWebService.LeaveThreads()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.leaveThreads`
pub async fn leave_threads(
    session: &mut Session,
    args: &LeaveThreadsRequest,
) -> crate::Result<LeaveThreadResponse> {
    session.post("?method=messaging.leaveThreads", args).await
}

/// Forward a thread to new recipients.
///
/// Maps to `MessagingWebService.ForwardThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.startNewThread` (forward uses same endpoint as start)
pub async fn forward_thread(
    session: &mut Session,
    args: &ForwardThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    session.post("?method=messaging.startNewThread", args).await
}

/// Reply to a thread by creating a new thread (quote-reply).
///
/// Maps to `MessagingWebService.ReplyInNewThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.startNewThread`
pub async fn reply_in_new_thread(
    session: &mut Session,
    args: &ForwardThreadRequestArguments,
) -> crate::Result<NewThreadResponse> {
    session.post("?method=messaging.startNewThread", args).await
}

// ===========================================================================
// Message operations
// ===========================================================================

/// Get messages in a thread (paginated).
///
/// Maps to `MessagingWebService.GetMessageList()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getMessagesForThread`
pub async fn get_message_list(
    session: &mut Session,
    args: &GetMessagesForThreadArguments,
) -> crate::Result<MessagesInThreadDto> {
    get_thread_by_id(session, args).await
}

/// Get lightweight message info (for notifications/previews).
///
/// Maps to `MessagingWebService.GetMessageInfoLight()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getMessageInfoLight`
pub async fn get_message_info_light(
    session: &mut Session,
    message_id: &str,
    common_inbox_id: Option<i64>,
    otp_inbox_id: Option<i64>,
) -> crate::Result<GetMessageInfoLightDto> {
    let mut params = vec![
        "method=messaging.getMessageInfoLight".to_string(),
        format!("messageId={}", encode_value(message_id)),
    ];
    if let Some(cid) = common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    if let Some(oid) = otp_inbox_id {
        params.push(format!("otpInboxId={oid}"));
    }
    let path = format!("?{}", params.join("&"));
    session.get(&path).await
}

/// Delete a message from a thread.
///
/// Maps to `MessagingWebService.DeleteMessage()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.deleteMessage`
pub async fn delete_message(
    session: &mut Session,
    args: &DeleteMessageRequest,
) -> crate::Result<DeleteMessageResponse> {
    session.post("?method=messaging.deleteMessage", args).await
}

/// Edit an existing message.
///
/// Maps to `MessagingWebService.EditMessage()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.editMessage`
pub async fn edit_message(
    session: &mut Session,
    args: &EditMessageRequest,
) -> crate::Result<EditMessageResponse> {
    session.post("?method=messaging.editMessage", args).await
}

/// Mark a message as the last read in a thread.
///
/// Maps to `MessagingWebService.SetLastReadMessage()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.setLastReadMessage`
pub async fn set_last_read_message(
    session: &mut Session,
    args: &SetLastMessageRequestArguments,
) -> crate::Result<SetLastReadResponse> {
    session
        .post("?method=messaging.setLastReadMessage", args)
        .await
}

// ===========================================================================
// Thread management
// ===========================================================================

/// Mute or unmute a thread.
///
/// Maps to `MessagingWebService.SetThreadMuted()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.setThreadsMuted`
pub async fn set_thread_muted(
    session: &mut Session,
    args: &MuteThreadRequestArguments,
) -> crate::Result<SetMutedResponse> {
    session
        .post("?method=messaging.setThreadsMuted", args)
        .await
}

/// Mark or unmark a thread (star/flag).
///
/// Maps to `MessagingWebService.SetThreadMarked()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.setThreadsMarked`
pub async fn set_thread_marked(
    session: &mut Session,
    args: &MarkThreadsRequest,
) -> crate::Result<SetMarkedResponse> {
    session
        .post("?method=messaging.setThreadsMarked", args)
        .await
}

/// Set the sensitivity level on a thread.
///
/// Maps to `MessagingWebService.SetSensitiveLevel()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.setSensitivityLevel`
pub async fn set_sensitive_level(
    session: &mut Session,
    args: &SetSensitivityLevelRequest,
) -> crate::Result<SetSensitiveLevelResponse> {
    session
        .post("?method=messaging.setSensitivityLevel", args)
        .await
}

/// Add recipients to an existing thread.
///
/// Maps to `MessagingWebService.AddRecipientsToThread()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.addRecipients`
pub async fn add_recipients_to_thread(
    session: &mut Session,
    args: &AddRecipientArguments,
) -> crate::Result<AddRecipientsResponse> {
    session.post("?method=messaging.addRecipients", args).await
}

// ===========================================================================
// Auto-reply
// ===========================================================================

/// Set an auto-reply message.
///
/// Maps to `MessagingWebService.SetAutoReply()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.setAutoReply`
pub async fn set_auto_reply(
    session: &mut Session,
    args: &SetAutoReplyArguments,
) -> crate::Result<MessageAutoReplyResult> {
    session.post("?method=messaging.setAutoReply", args).await
}

/// Get the current auto-reply configuration.
///
/// Maps to `MessagingWebService.GetAutoReply()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getAutoReply`
pub async fn get_auto_reply(session: &mut Session) -> crate::Result<MessageAutoReplyResult> {
    session.get("?method=messaging.getAutoReply").await
}

/// Delete the auto-reply configuration.
///
/// Maps to `MessagingWebService.DeleteAutoReply()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.deleteAutoReply`
pub async fn delete_auto_reply(session: &mut Session) -> crate::Result<DeleteAutoReplyResponse> {
    session
        .post_empty("?method=messaging.deleteAutoReply")
        .await
}

// ===========================================================================
// Folder management
// ===========================================================================

/// Get folders for the current user's mailbox.
///
/// Maps to `FolderService.GetFolders()`.
///
/// The decompiled C# passes a `GetFoldersArguments` object via
/// `ConvertObjectToQueryUrl`, which serializes ALL properties including
/// `includeDeletedFolders=false`. The API appears to require all parameters
/// to be explicitly present.
///
/// # Endpoint
///
/// `GET ?method=messaging.getFolders&includeDeletedFolders={bool}`
pub async fn get_folders(
    session: &mut Session,
    args: &GetFoldersArguments,
) -> crate::Result<Vec<Folder>> {
    let mut params = vec![
        "method=messaging.getFolders".to_string(),
        format!("includeDeletedFolders={}", args.include_deleted_folders),
    ];
    if let Some(cid) = args.common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    let path = format!("?{}", params.join("&"));
    session.get(&path).await
}

/// Create a new message folder.
///
/// Maps to `FolderService.CreateFolder()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.createFolder`
pub async fn create_folder(
    session: &mut Session,
    args: &CreateFolderArguments,
) -> crate::Result<FolderMutationResponse> {
    session.post("?method=messaging.createFolder", args).await
}

/// Rename a folder.
///
/// Maps to `FolderService.UpdateFolder()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.updateFolder`
pub async fn update_folder(
    session: &mut Session,
    args: &UpdateFolderArguments,
) -> crate::Result<FolderMutationResponse> {
    session.post("?method=messaging.updateFolder", args).await
}

/// Delete a folder.
///
/// Maps to `FolderService.PostDeleteFolder()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.deletefolder`
pub async fn delete_folder(
    session: &mut Session,
    folder_id: i64,
    common_inbox_id: Option<i64>,
) -> crate::Result<FolderMutationResponse> {
    let mut params = vec![
        "method=messaging.deletefolder".to_string(),
        format!("folderId={folder_id}"),
    ];
    if let Some(cid) = common_inbox_id {
        params.push(format!("commonInboxId={cid}"));
    }
    let path = format!("?{}", params.join("&"));
    session.post_empty(&path).await
}

/// Move threads to a folder.
///
/// Maps to `FolderService.MoveThreadsToFolder()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.moveThreadsToFolder`
pub async fn move_threads_to_folder(
    session: &mut Session,
    args: &MoveThreadsToFolderRequestArguments,
) -> crate::Result<FolderMutationResponse> {
    session
        .post("?method=messaging.moveThreadsToFolder", args)
        .await
}

/// Get common (shared) inboxes for the user's institution profiles.
///
/// Maps to `FolderService.GetCommonInboxes()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getCommonInboxes`
pub async fn get_common_inboxes(
    session: &mut Session,
    institution_profile_ids: &[i64],
    include_profile_picture_url: bool,
) -> crate::Result<Vec<CommonInboxesDto>> {
    let mut params = vec!["method=messaging.getCommonInboxes".to_string()];
    for id in institution_profile_ids {
        params.push(format!("institutionProfileIds={id}"));
    }
    if include_profile_picture_url {
        params.push("shouldIncludeProfilePictureUrl=true".to_string());
    }
    let path = format!("?{}", params.join("&"));
    session.get(&path).await
}

// ===========================================================================
// Bundle / subscription management
// ===========================================================================

/// Get threads within a bundle (grouped threads).
///
/// Maps to `MessagingWebService.GetThreadsInBundleList()`.
///
/// # Endpoint
///
/// `GET ?method=messaging.getThreadsInBundle`
pub async fn get_threads_in_bundle_list(
    session: &mut Session,
    args: &GetThreadsInBundleArguments,
) -> crate::Result<MessageThreadSubscriptionList> {
    let bundle_id = args.bundle_id.unwrap_or(0);
    session
        .get(&format!(
            "?method=messaging.getThreadsInBundle&bundleId={bundle_id}"
        ))
        .await
}

/// Update subscription status (read/unread) for thread subscriptions.
///
/// Maps to `MessagingWebService.SetMessageThreadsSubscriptionStatus()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.updateSubscriptionStatus`
pub async fn set_subscription_status(
    session: &mut Session,
    args: &UpdateMessageThreadsSubscriptionStatusRequest,
) -> crate::Result<SetSubscriptionStatusResponse> {
    session
        .post("?method=messaging.updateSubscriptionStatus", args)
        .await
}

/// Check whether recipients have blocked messaging channels.
///
/// Maps to `MessagingWebService.CheckRecipientsForBlockedChannels()`.
///
/// # Endpoint
///
/// `POST ?method=municipalConfiguration.getBlockedCommunicationInstitutionProfilesAndGroups`
pub async fn check_recipients_for_blocked_channels(
    session: &mut Session,
    recipients: &[RecipientApiModel],
) -> crate::Result<BlockedChannelsResponse> {
    session
        .post(
            "?method=municipalConfiguration.getBlockedCommunicationInstitutionProfilesAndGroups",
            &recipients,
        )
        .await
}

/// Attach messages from a thread to a secure document.
///
/// Maps to `MessagingWebService.AttachMessagesToSecureDocument()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.attachMessagesToSecureDocument`
pub async fn attach_messages_to_secure_document(
    session: &mut Session,
    args: &AttachMessagesToSecureDocumentRequest,
) -> crate::Result<AttachToDocumentResponse> {
    session
        .post("?method=messaging.attachMessagesToSecureDocument", args)
        .await
}

/// Send an event reminder message.
///
/// Maps to `MessagingWebService.SendEventReminder()`.
///
/// # Endpoint
///
/// `POST ?method=messaging.sendEventReminder`
pub async fn send_event_reminder(
    session: &mut Session,
    args: &SendEventReminderRequest,
) -> crate::Result<SendEventReminderResponse> {
    session
        .post("?method=messaging.sendEventReminder", args)
        .await
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
