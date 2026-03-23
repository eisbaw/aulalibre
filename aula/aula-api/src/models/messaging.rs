//! Messaging domain models.
//!
//! These types represent the message threading and messaging system in Aula,
//! covering threads, subscriptions, messages, folders, auto-reply, and
//! common inboxes.
//!
//! See `data_models.md` Models.MessageThreads, Models.Messages, and
//! `domain_concepts.md` Section 1.3.

use serde::{Deserialize, Serialize};

use crate::enums::common::{FilterAndSortType, SortOrderEnum};
use crate::enums::messaging::{
    CommonInboxType, FolderType, RecipientApiType, SensitivityLevel, SubscriptionType, ThreadType,
};
use crate::enums::profiles::PortalRole;

use super::profiles::ProfilePictureDto;
use crate::serde_helpers::deserialize_optional_string_from_any;

// ---------------------------------------------------------------------------
// Shared value types
// ---------------------------------------------------------------------------

/// Rich text content wrapper (HTML body).
///
/// Maps to `Models.RichTextWrapperDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichTextWrapperDto {
    pub html: Option<String>,
}

/// File download reference (name + URL).
///
/// Maps to `Models.Common.Api.Files.Argument.DownloadFileFromAulaArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFileFromAulaArguments {
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Recipient/mailbox-owner identity used across the messaging domain.
///
/// Maps to `Models.MessageThreads.Argument.RecipientApiModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientApiModel {
    pub id: Option<i64>,
    pub otp_inbox_id: Option<i64>,
    pub mail_box_owner_type: Option<String>,
    pub profile_id: Option<i64>,
    #[serde(default)]
    pub is_deactivated: bool,
    #[serde(default)]
    pub is_deleted: bool,
    pub portal_role: Option<PortalRole>,
}

/// Simple mailbox identity (sender/recipient in latest-message summaries).
///
/// Maps to `Models.MessageThreads.MailBox`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailBox {
    pub id: Option<i64>,
    /// C# `MailBox.Email` has `[JsonProperty("address")]`.
    #[serde(rename = "address")]
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub relation: Option<String>,
    pub short_name: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Thread models
// ---------------------------------------------------------------------------

/// A simple subscription reference embedded in `MessageThread.otherRecipients`.
///
/// Maps to `MessageThread.SimpleMessageThreadSubscription`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleMessageThreadSubscription {
    pub display_name: Option<String>,
    pub relation: Option<String>,
    pub short_name: Option<String>,
}

/// Core message thread entity.
///
/// Maps to `Models.MessageThreads.MessageThread`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThread {
    pub started_date_time: Option<String>,
    pub subject: Option<String>,
    #[serde(default)]
    pub required_step_up: bool,
    pub sensitivity_level: Option<SensitivityLevel>,
    pub creator: Option<serde_json::Value>,
    pub other_recipients: Option<Vec<SimpleMessageThreadSubscription>>,
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub is_forwarded: bool,
}

/// Link between a thread and an external entity (event, vacation request).
///
/// Maps to `Models.MessageThreads.ThreadEntityLinkDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadEntityLinkDto {
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub entity_id: Option<String>,
    pub thread_type: Option<ThreadType>,
}

/// Latest message summary within a thread subscription.
///
/// Maps to `Models.MessageThreads.MessageThreadLatestMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThreadLatestMessage {
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub id: Option<String>,
    pub thread_id: Option<i64>,
    pub send_date_time: Option<String>,
    pub text: Option<RichTextWrapperDto>,
    pub sender: Option<MailBox>,
    pub new_recipient: Option<MailBox>,
    #[serde(default)]
    pub has_attachments: bool,
    #[serde(default)]
    pub pending_media: bool,
}

/// Child related to a thread subscription (regarding-children tagging).
///
/// Maps to `Models.MessageThreads.MessageThreadSubscriptionRelatedChild`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThreadSubscriptionRelatedChild {
    pub id: Option<i64>,
    pub display_name: Option<String>,
}

/// Institution related to a thread subscription.
///
/// Maps to `Models.MessageThreads.MessageThreadSubscriptionRelatedInstitution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThreadSubscriptionRelatedInstitution {
    pub institution_code: Option<String>,
    pub name: Option<String>,
}

/// Participant in a message thread (extended mailbox with profile info).
///
/// Maps to `Models.MessageThreads.MessageParticipantDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageParticipantDto {
    pub mail_box_owner: Option<RecipientApiModel>,
    pub full_name: Option<String>,
    pub metadata: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub last_read_message_id: Option<String>,
    pub last_read_message_timestamp: Option<String>,
    pub short_name: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
}

/// Children tagged on a thread (regarding-children in compose/view).
///
/// Maps to `Models.MessageThreads.MessageRegardingChildren`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRegardingChildren {
    pub profile_id: Option<i64>,
    pub display_name: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Draft message state persisted on a subscription.
///
/// Maps to `Models.MessageThreads.MessageDraft` (inferred from usage).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDraft {
    pub text: Option<String>,
    pub attachment_ids: Option<Vec<i64>>,
}

// ---------------------------------------------------------------------------
// Subscription models
// ---------------------------------------------------------------------------

/// A user's subscription to a message thread.
///
/// This is the primary entity for the thread list ("inbox view"). It combines
/// the thread metadata with per-user state (read, muted, marked, folder).
///
/// Maps to `Models.MessageThreads.MessageThreadSubscription`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThreadSubscription {
    pub id: Option<i64>,
    pub leave_time: Option<String>,
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub marked: bool,
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub sensitive: bool,
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub last_read_message_id: Option<String>,
    pub institution_code: Option<String>,
    pub creator: Option<MessageParticipantDto>,
    pub recipients: Option<Vec<MessageParticipantDto>>,
    pub regarding_children: Option<Vec<MessageRegardingChildren>>,
    pub latest_message: Option<MessageThreadLatestMessage>,
    pub subject: Option<String>,
    pub message_draft: Option<MessageDraft>,
    pub mail_box_owner: Option<RecipientApiModel>,
    pub current_folder: Option<Folder>,
    pub subscription_id: Option<i64>,
    #[serde(default)]
    pub is_thread_or_subscription_deleted: bool,
    pub subscription_type: Option<SubscriptionType>,
    pub number_of_bundle_items: Option<i64>,
    pub extra_recipients_count: Option<i64>,
    pub bundle_id: Option<i64>,
    pub thread_entity_link_dto: Option<ThreadEntityLinkDto>,
    pub primary_subscription_id: Option<i64>,
}

/// Paginated list of thread subscriptions.
///
/// Maps to `Models.MessageThreads.MessageThreadSubscriptionList`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageThreadSubscriptionList {
    pub threads: Option<Vec<MessageThreadSubscription>>,
    pub page: Option<i32>,
    pub bundle_id: Option<i64>,
    #[serde(default)]
    pub more_messages_exist: bool,
}

// ---------------------------------------------------------------------------
// Message models
// ---------------------------------------------------------------------------

/// A single message within a thread.
///
/// Maps to `Models.Messages.MessageDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub id: Option<String>,
    pub message_type: Option<String>,
    pub send_date_time: Option<String>,
    pub text: Option<RichTextWrapperDto>,
    pub sender: Option<MessageRecipient>,
    #[serde(default)]
    pub can_reply_to_message: bool,
    pub attachments: Option<Vec<serde_json::Value>>,
    pub new_recipient: Option<MessageRecipient>,
    pub new_recipients: Option<Vec<MessageRecipient>>,
    pub original_recipients: Option<Vec<MessageRecipient>>,
    pub leaver_name: Option<String>,
    pub inviter_name: Option<String>,
    pub leaver_names: Option<Vec<String>>,
}

/// Recipient on a message (sender, new-recipient, etc.).
///
/// Maps to `Models.Messages.MessageRecipient`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecipient {
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub answer_directly_name: Option<String>,
    pub mail_box_owner: Option<RecipientApiModel>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
    pub metadata: Option<String>,
}

/// Lightweight message info (used for notifications/previews).
///
/// Maps to `Models.MessageThreads.GetMessageInfoLightDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageInfoLightDto {
    pub thread_id: Option<i32>,
    pub subject: Option<String>,
    #[serde(default)]
    pub is_sensitive: bool,
    pub message: Option<MessageDto>,
}

/// Full thread with messages (thread detail view).
///
/// Maps to `Models.MessageThreads.Messages.MessagesInThreadDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesInThreadDto {
    pub id: Option<i64>,
    pub first_message: Option<MessageDto>,
    pub messages: Option<Vec<MessageDto>>,
    #[serde(default)]
    pub is_marked: bool,
    pub thread_creator: Option<MessagesInThreadRecipientDto>,
    pub thread_started_date_time: Option<String>,
    pub recipients: Option<Vec<MessagesInThreadRecipientDto>>,
    #[serde(default)]
    pub more_messages_exist: bool,
    pub total_message_count: Option<i32>,
    pub page: Option<i32>,
    pub subject: Option<String>,
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub marked: bool,
    #[serde(default)]
    pub is_thread_forwarded: bool,
    #[serde(default)]
    pub sensitive: bool,
    #[serde(default)]
    pub has_secure_documents: bool,
    pub mailbox_owner: Option<RecipientApiModel>,
    pub thread_entity_link_dto: Option<ThreadEntityLinkDto>,
    pub folder_name: Option<String>,
}

/// Recipient within a thread detail view.
///
/// Maps to `MessagesInThreadDto.RecipientDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesInThreadRecipientDto {
    #[serde(default, deserialize_with = "deserialize_optional_string_from_any")]
    pub last_read_message_id: Option<String>,
    pub last_read_time_stamp: Option<String>,
    pub leave_time: Option<String>,
    pub deleted_at: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub mail_box_owner: Option<RecipientApiModel>,
    pub metadata: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
}

/// Stubbed child reference in message recipient relations.
///
/// Maps to `Models.MessageThreads.Messages.MessagesStubbedChild`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesStubbedChild {
    pub id: Option<i64>,
    pub display_name: Option<String>,
    pub class: Option<String>,
    pub institution_name: Option<String>,
}

/// Relation info on a message recipient.
///
/// Maps to `Models.MessageThreads.Messages.MessageRecipientRelationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecipientRelationDto {
    #[serde(rename = "type")]
    pub relation_type: Option<String>,
    pub class: Option<String>,
    pub children: Option<Vec<MessagesStubbedChild>>,
    pub institution_name: Option<String>,
}

/// Participant relation in message thread view.
///
/// Maps to `Models.MessageThreads.MessageParticipantRelationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageParticipantRelationDto {
    #[serde(rename = "type")]
    pub relation_type: Option<String>,
    /// NOTE: The server-side JSON key is `institutioName` (missing 'n') -- this
    /// is a typo in the Aula backend API, not a bug on our side.
    #[serde(rename = "institutioName")]
    pub institution_name: Option<String>,
    pub class: Option<String>,
    pub children: Option<Vec<MessagesStubbedChild>>,
}

/// Messaging participant (used in folder/common-inbox contexts).
///
/// Maps to `Models.MessageThreads.Folders.MessagingParticipantDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingParticipantDto {
    pub answer_directly_name: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub mail_box_owner: Option<RecipientApiModel>,
    pub full_name: Option<String>,
    pub metadata: Option<String>,
}

/// Deleted message marker.
///
/// Maps to `Models.MessageThreads.Messages.DeleteMessageDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMessageDto {
    pub deleted_at: Option<String>,
}

/// Bundle update notification DTO.
///
/// Maps to `Models.Messages.UpdateBundleMessageDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBundleMessageDto {
    #[serde(default)]
    pub is_marked: bool,
    #[serde(default)]
    pub is_sensitive: bool,
    #[serde(default)]
    pub is_unread: bool,
    pub thread: Option<MessageThreadSubscription>,
    pub lastest_message_date: Option<String>,
    #[serde(default)]
    pub is_muted: bool,
}

/// Message file URL.
///
/// Maps to `Models.Messages.MessageFileUrl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageFileUrl {
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Folder and common inbox models
// ---------------------------------------------------------------------------

/// Message folder (Normal, Deleted, or user-created).
///
/// Maps to `Models.MessageThreads.Folders.Folder`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub folder_type: Option<FolderType>,
}

/// Common (shared) inbox with participants and folders.
///
/// Maps to `Models.MessageThreads.Result.CommonInboxesDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonInboxesDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub folders: Option<Vec<Folder>>,
    pub participants: Option<Vec<MessagingParticipantDto>>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub common_inbox_type: Option<CommonInboxType>,
}

// ---------------------------------------------------------------------------
// Auto-reply models
// ---------------------------------------------------------------------------

/// Auto-reply configuration (set by user).
///
/// Maps to `Models.MessageThreads.AutoReply.Argument.SetAutoReplyArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoReplyArguments {
    pub reply_text: Option<String>,
    pub end_date_time: Option<String>,
    pub start_date_time: Option<String>,
}

/// Auto-reply result (returned by API).
///
/// Maps to `Models.MessageThreads.AutoReply.Result.MessageAutoReplyResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAutoReplyResult {
    pub id: Option<i64>,
    pub reply_text: Option<RichTextWrapperDto>,
    pub end_date_time: Option<String>,
    pub start_date_time: Option<String>,
}

// ---------------------------------------------------------------------------
// Request / argument types
// ---------------------------------------------------------------------------

/// Message content for composing / replying.
///
/// Maps to `Models.MessageThreads.Argument.StartNewThread.MessageContentRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentRequest {
    pub attachment_ids: Option<Vec<i64>>,
    pub text: Option<String>,
}

/// Start a new message thread.
///
/// Maps to `Models.MessageThreads.Argument.StartNewThread.StartNewThreadRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartNewThreadRequestArguments {
    pub message: Option<MessageContentRequest>,
    pub subject: Option<String>,
    pub recipients: Option<Vec<RecipientApiModel>>,
    pub bcc_recipients: Option<Vec<RecipientApiModel>>,
    #[serde(default)]
    pub sensitive: bool,
    pub creator: Option<RecipientApiModel>,
}

/// Forward info when forwarding a thread.
///
/// Maps to `Models.MessageThreads.Argument.StartNewThread.ForwardInfoRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardInfoRequestArguments {
    pub forwarded_thread_id: Option<i64>,
    pub forwarded_message_ids: Option<Vec<String>>,
    #[serde(default)]
    pub direct_reply: bool,
    #[serde(default)]
    pub forward_single_message: bool,
    #[serde(default)]
    pub direct_reply_to_creator: bool,
}

/// Forward a thread (extends StartNewThreadRequestArguments).
///
/// Maps to `Models.MessageThreads.Argument.StartNewThread.ForwardThreadRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForwardThreadRequestArguments {
    // Base fields from StartNewThreadRequestArguments
    pub message: Option<MessageContentRequest>,
    pub subject: Option<String>,
    pub recipients: Option<Vec<RecipientApiModel>>,
    pub bcc_recipients: Option<Vec<RecipientApiModel>>,
    #[serde(default)]
    pub sensitive: bool,
    pub creator: Option<RecipientApiModel>,
    // Extension
    pub forward_info: Option<ForwardInfoRequestArguments>,
}

/// Reply to an existing thread.
///
/// Maps to `Models.MessageThreads.Argument.ReplyMessageArgument`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageArgument {
    pub thread_id: Option<i64>,
    pub message: Option<MessageContentRequest>,
    pub common_inbox_id: Option<i64>,
    pub bundle_id: Option<i64>,
}

/// Edit an existing message.
///
/// Maps to `Models.MessageThreads.Argument.EditMessageRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditMessageRequest {
    // Base from ReplyMessageArgument
    pub thread_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
    pub bundle_id: Option<i64>,
    // Extension
    pub message_id: Option<String>,
    pub message_request: Option<MessageContentRequest>,
}

/// Add recipients to an existing thread.
///
/// Maps to `Models.MessageThreads.Argument.AddRecipientArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddRecipientArguments {
    pub thread_id: Option<i64>,
    pub recipients: Option<Vec<RecipientApiModel>>,
    pub common_inbox_id: Option<i64>,
}

/// Delete a single message.
///
/// Maps to `Models.MessageThreads.Argument.DeleteMessageRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMessageRequest {
    pub message_id: Option<String>,
    pub thread_id: Option<i64>,
}

/// Delete thread(s).
///
/// Maps to `Models.MessageThreads.Argument.DeleteThreadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteThreadArguments {
    pub subscription_ids: Option<Vec<i64>>,
    pub thread_ids: Option<Vec<i64>>,
    pub common_inbox_id: Option<i64>,
}

/// Leave a thread.
///
/// Maps to `Models.MessageThreads.Argument.LeaveThreadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveThreadArguments {
    pub thread_id: Option<i64>,
}

/// Leave multiple threads.
///
/// Maps to `Models.MessageThreads.Argument.LeaveThreadsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveThreadsRequest {
    pub subscription_ids: Option<Vec<i64>>,
}

/// Mark/unmark threads.
///
/// Maps to `Models.MessageThreads.Argument.MarkThreadsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkThreadsRequest {
    #[serde(default)]
    pub marked: bool,
    pub thread_ids: Option<Vec<i64>>,
    pub subscription_ids: Option<Vec<i64>>,
    pub common_inbox_id: Option<i64>,
}

/// Mute/unmute thread(s).
///
/// Maps to `Models.MessageThreads.Argument.MuteThreadRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MuteThreadRequestArguments {
    #[serde(default)]
    pub muted: bool,
    /// C# `MuteThreadRequestArguments.Owner` has `[JsonProperty("MailBoxOwner")]`.
    #[serde(rename = "MailBoxOwner")]
    pub owner: Option<RecipientApiModel>,
    pub subscription_ids: Option<Vec<i64>>,
    pub common_inbox_id: Option<i64>,
    pub thread_ids: Option<Vec<i64>>,
}

/// Set last-read message on a thread.
///
/// Maps to `Models.MessageThreads.Argument.SetLastMessageRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLastMessageRequestArguments {
    pub message_id: Option<String>,
    pub thread_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
}

/// Set sensitivity level on a thread.
///
/// Maps to `Models.MessageThreads.Argument.SetSensitivityLevelRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSensitivityLevelRequest {
    pub thread_id: Option<i64>,
    pub sensitivity_level: Option<i32>,
    pub common_inbox_id: Option<i64>,
    pub bundle_id: Option<i64>,
}

/// Update read/unread status of thread subscriptions.
///
/// Maps to `Models.MessageThreads.Argument.UpdateMessageThreadsSubscriptionStatusRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessageThreadsSubscriptionStatusRequest {
    pub subscription_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub is_read: bool,
}

/// Get a lightweight message preview.
///
/// Maps to `Models.MessageThreads.Argument.GetMessageInfoLightRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageInfoLightRequest {
    pub thread_id: Option<i64>,
    pub message_id: Option<String>,
    pub common_inbox_id: Option<i64>,
    pub otp_inbox_id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Folder request types
// ---------------------------------------------------------------------------

/// Create a new folder.
///
/// Maps to `Models.MessageThreads.Argument.Folder.CreateFolderArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderArguments {
    pub folder_name: Option<String>,
    pub common_inbox_id: Option<i64>,
}

/// Get folders for a mailbox.
///
/// Maps to `Models.MessageThreads.Argument.Folder.GetFoldersArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFoldersArguments {
    #[serde(default)]
    pub include_deleted_folders: bool,
    pub common_inbox_id: Option<i64>,
}

/// Move threads to a folder.
///
/// Maps to `Models.MessageThreads.Argument.Folder.MoveThreadsToFolderRequestArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveThreadsToFolderRequestArguments {
    pub thread_ids: Option<Vec<i64>>,
    pub subscription_ids: Option<Vec<i64>>,
    pub folder_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
}

/// Update (rename) a folder.
///
/// Maps to `Models.MessageThreads.Argument.Folder.UpdateFolderArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderArguments {
    pub folder_id: Option<i64>,
    pub folder_name: Option<String>,
}

/// Get common inboxes for institution profiles.
///
/// Maps to `Models.MessageThreads.Argument.Folder.GetCommonInboxesArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommonInboxesArguments {
    pub institution_profile_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub should_include_profile_picture_url: bool,
}

/// Get messages in a thread (paginated).
///
/// Maps to `Models.MessageThreads.Argument.GetMessages.GetMessagesForThreadArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesForThreadArguments {
    pub thread_id: Option<i64>,
    pub page: Option<i32>,
    pub common_inbox_id: Option<i64>,
}

/// Get thread list (inbox view, filtered/sorted/paginated).
///
/// Maps to `Models.MessageThreads.Result.GetThreadListArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetThreadListArguments {
    pub folder_id: Option<i64>,
    pub filter_type: Option<FilterAndSortType>,
    pub sort_type: Option<FilterAndSortType>,
    pub sort_order: Option<SortOrderEnum>,
    pub page: Option<i32>,
    pub thread_ids: Option<Vec<i64>>,
    pub mail_box_owner_type: Option<RecipientApiType>,
    pub mail_box_owners: Option<Vec<i64>>,
    pub active_children: Option<Vec<i64>>,
}

/// Get threads within a bundle.
///
/// Maps to `Models.MessageThreads.Result.GetThreadsInBundleArguments`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetThreadsInBundleArguments {
    pub bundle_id: Option<i64>,
}

/// Attach messages to a secure document.
///
/// Maps to `Models.Messages.AttachMessagesToSecureDocument.AttachMessagesToSecureDocumentRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachMessagesToSecureDocumentRequest {
    pub secure_document_id: Option<i64>,
    pub message_ids: Option<Vec<String>>,
    pub thread_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_rich_text() {
        let json = r#"{"html": "<p>Hello</p>"}"#;
        let rt: RichTextWrapperDto = serde_json::from_str(json).unwrap();
        assert_eq!(rt.html.as_deref(), Some("<p>Hello</p>"));
    }

    #[test]
    fn deserialize_rich_text_null() {
        let json = r#"{"html": null}"#;
        let rt: RichTextWrapperDto = serde_json::from_str(json).unwrap();
        assert!(rt.html.is_none());
    }

    #[test]
    fn deserialize_mailbox() {
        let json = r#"{
            "id": 42,
            "address": "test@aula.dk",
            "displayName": "Test User",
            "relation": "Teacher",
            "shortName": "TU"
        }"#;
        let mb: MailBox = serde_json::from_str(json).unwrap();
        assert_eq!(mb.id, Some(42));
        assert_eq!(mb.display_name.as_deref(), Some("Test User"));
    }

    #[test]
    fn deserialize_recipient_api_model() {
        let json = r#"{
            "id": 100,
            "otpInboxId": 0,
            "mailBoxOwnerType": "institutionProfile",
            "profileId": 55,
            "isDeactivated": false,
            "isDeleted": false,
            "portalRole": "guardian"
        }"#;
        let r: RecipientApiModel = serde_json::from_str(json).unwrap();
        assert_eq!(r.id, Some(100));
        assert_eq!(r.profile_id, Some(55));
        assert_eq!(r.portal_role, Some(PortalRole::Guardian));
        assert!(!r.is_deactivated);
    }

    #[test]
    fn deserialize_folder() {
        let json = r#"{
            "id": 1,
            "name": "Inbox",
            "type": "normal"
        }"#;
        let f: Folder = serde_json::from_str(json).unwrap();
        assert_eq!(f.id, Some(1));
        assert_eq!(f.name.as_deref(), Some("Inbox"));
        assert_eq!(f.folder_type, Some(FolderType::Normal));
    }

    #[test]
    fn deserialize_message_thread() {
        let json = r#"{
            "startedDateTime": "2024-01-15T10:30:00",
            "subject": "School trip permission",
            "requiredStepUp": false,
            "sensitivityLevel": "level1",
            "creator": null,
            "otherRecipients": [
                {"displayName": "Anne Hansen", "relation": "Guardian", "shortName": "AH"}
            ],
            "threadId": "12345",
            "isForwarded": false
        }"#;
        let t: MessageThread = serde_json::from_str(json).unwrap();
        assert_eq!(t.subject.as_deref(), Some("School trip permission"));
        assert_eq!(t.sensitivity_level, Some(SensitivityLevel::Level1));
        assert!(!t.required_step_up);
        let recipients = t.other_recipients.unwrap();
        assert_eq!(recipients.len(), 1);
        assert_eq!(recipients[0].display_name.as_deref(), Some("Anne Hansen"));
    }

    #[test]
    fn deserialize_message_thread_subscription() {
        let json = r#"{
            "id": 999,
            "leaveTime": null,
            "muted": false,
            "marked": true,
            "read": true,
            "sensitive": false,
            "lastReadMessageId": "msg-42",
            "institutionCode": "101001",
            "creator": null,
            "recipients": [],
            "regardingChildren": [],
            "latestMessage": null,
            "subject": "Homework reminder",
            "messageDraft": null,
            "mailBoxOwner": null,
            "currentFolder": {"id": 1, "name": "Inbox", "type": "normal"},
            "subscriptionId": 888,
            "isThreadOrSubscriptionDeleted": false,
            "subscriptionType": "unbundled",
            "numberOfBundleItems": null,
            "extraRecipientsCount": null,
            "bundleId": null,
            "threadEntityLinkDto": null,
            "primarySubscriptionId": null
        }"#;
        let s: MessageThreadSubscription = serde_json::from_str(json).unwrap();
        assert_eq!(s.id, Some(999));
        assert!(s.marked);
        assert!(s.read);
        assert_eq!(s.subject.as_deref(), Some("Homework reminder"));
        assert_eq!(s.subscription_type, Some(SubscriptionType::Unbundled));
        let folder = s.current_folder.unwrap();
        assert_eq!(folder.name.as_deref(), Some("Inbox"));
    }

    #[test]
    fn deserialize_message_dto() {
        let json = r#"{
            "id": "msg-1",
            "messageType": "Message",
            "sendDateTime": "2024-01-15T10:35:00",
            "text": {"html": "<p>See attached</p>"},
            "sender": {
                "shortName": "LH",
                "fullName": "Lars Hansen",
                "answerDirectlyName": null,
                "mailBoxOwner": null,
                "profilePicture": null,
                "metadata": null
            },
            "canReplyToMessage": true,
            "attachments": [],
            "newRecipient": null,
            "newRecipients": null,
            "originalRecipients": null,
            "leaverName": null,
            "inviterName": null,
            "leaverNames": null
        }"#;
        let m: MessageDto = serde_json::from_str(json).unwrap();
        assert_eq!(m.id.as_deref(), Some("msg-1"));
        assert!(m.can_reply_to_message);
        let text = m.text.unwrap();
        assert_eq!(text.html.as_deref(), Some("<p>See attached</p>"));
        let sender = m.sender.unwrap();
        assert_eq!(sender.full_name.as_deref(), Some("Lars Hansen"));
    }

    #[test]
    fn deserialize_common_inbox() {
        let json = r#"{
            "id": 5,
            "name": "Kontoret",
            "address": "kontor@school.dk",
            "folders": [],
            "participants": [],
            "institutionCode": "101001",
            "institutionName": "Test Skole",
            "commonInboxType": "institutional"
        }"#;
        let ci: CommonInboxesDto = serde_json::from_str(json).unwrap();
        assert_eq!(ci.id, Some(5));
        assert_eq!(ci.name.as_deref(), Some("Kontoret"));
        assert_eq!(ci.common_inbox_type, Some(CommonInboxType::Institutional));
    }

    #[test]
    fn deserialize_auto_reply_result() {
        let json = r#"{
            "id": 10,
            "replyText": {"html": "<p>I am away</p>"},
            "endDateTime": "2024-02-01T00:00:00",
            "startDateTime": "2024-01-20T00:00:00"
        }"#;
        let ar: MessageAutoReplyResult = serde_json::from_str(json).unwrap();
        assert_eq!(ar.id, Some(10));
        let text = ar.reply_text.unwrap();
        assert_eq!(text.html.as_deref(), Some("<p>I am away</p>"));
    }

    #[test]
    fn deserialize_start_new_thread_request() {
        let json = r#"{
            "message": {"attachmentIds": [1, 2], "text": "Hello"},
            "subject": "New topic",
            "recipients": [
                {"id": 100, "otpInboxId": 0, "mailBoxOwnerType": "InstitutionProfile", "profileId": 55, "isDeactivated": false, "isDeleted": false, "portalRole": null}
            ],
            "bccRecipients": [],
            "sensitive": true,
            "creator": null
        }"#;
        let req: StartNewThreadRequestArguments = serde_json::from_str(json).unwrap();
        assert_eq!(req.subject.as_deref(), Some("New topic"));
        assert!(req.sensitive);
        let msg = req.message.unwrap();
        assert_eq!(msg.text.as_deref(), Some("Hello"));
        assert_eq!(msg.attachment_ids.unwrap(), vec![1, 2]);
    }

    #[test]
    fn deserialize_messages_in_thread() {
        let json = r#"{
            "id": 77,
            "firstMessage": null,
            "messages": [],
            "isMarked": false,
            "threadCreator": null,
            "threadStartedDateTime": "2024-01-10T08:00:00",
            "recipients": [],
            "moreMessagesExist": true,
            "totalMessageCount": 15,
            "page": 1,
            "subject": "Class photos",
            "muted": false,
            "marked": false,
            "isThreadForwarded": false,
            "sensitive": false,
            "hasSecureDocuments": false,
            "mailboxOwner": null,
            "threadEntityLinkDto": null,
            "folderName": "Inbox"
        }"#;
        let mt: MessagesInThreadDto = serde_json::from_str(json).unwrap();
        assert_eq!(mt.id, Some(77));
        assert!(mt.more_messages_exist);
        assert_eq!(mt.total_message_count, Some(15));
        assert_eq!(mt.folder_name.as_deref(), Some("Inbox"));
    }

    #[test]
    fn deserialize_thread_entity_link() {
        let json = r#"{
            "entityId": "event-123",
            "threadType": "eventReminder"
        }"#;
        let link: ThreadEntityLinkDto = serde_json::from_str(json).unwrap();
        assert_eq!(link.entity_id.as_deref(), Some("event-123"));
        assert_eq!(link.thread_type, Some(ThreadType::EventReminder));
    }

    #[test]
    fn serialize_start_new_thread() {
        let req = StartNewThreadRequestArguments {
            message: Some(MessageContentRequest {
                attachment_ids: Some(vec![]),
                text: Some("Test".to_string()),
            }),
            subject: Some("Test subject".to_string()),
            recipients: Some(vec![]),
            bcc_recipients: Some(vec![]),
            sensitive: false,
            creator: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"subject\":\"Test subject\""));
        assert!(json.contains("\"bccRecipients\":[]"));
    }
}
