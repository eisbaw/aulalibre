//! Messaging and message thread enums.

use serde::{Deserialize, Serialize};

/// Type of message in a thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    AllMessageRelatedType,
    Message,
    RecipientAdded,
    RecipientRemoved,
    AutoReply,
    SystemForward,
    SystemReply,
    Forward,
    Other,
    RecipientsAdded,
    RecipientsRemoved,
    MessageDeleted,
    MessageEdited,
    SystemForwardSingleMessage,
}

/// Sensitivity level for messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensitivityLevel {
    Level1,
    Level2,
    Level3,
}

/// Read/unread status of a thread subscription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Read,
    Unread,
}

/// Type of message recipient.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipientType {
    Profile,
    Group,
    CommonInbox,
    Unknown,
}

/// Target area for recipients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipientsTarget {
    MessageRecipients,
    MessageBccRecipients,
    CalendarEvent,
    SecureDocument,
    Post,
}

/// Click action on a message thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageThreadClickType {
    ItemClick,
    Move,
    Mark,
    Delete,
    MultiMove,
    MultiMark,
    MultiDelete,
}

/// Bundled message display type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BundledMessageType {
    IsRegularMessage,
    FirstMessage,
    MiddleMessage,
    LastMessage,
    PrimaryMessage,
    SecondaryMessage,
    LastOfSecondaryMessage,
}

/// Type of common inbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonInboxType {
    Institutional,
    CrossInstitutional,
}

/// Form type when composing a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageFormType {
    StartNewThread,
    ReplyInThread,
    Forward,
    StartNewThreadWithUser,
    ReplyInThreadFromAnswerOptionsButton,
    ForwardSingleMessage,
}

/// More-menu action on a message thread cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageThreadCellMoreMenuActionEnum {
    MoveToFolder,
    MarkAsImportant,
    Forward,
}

/// Send message button options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SendMessageButton {
    ReplySingle,
    ReplyAll,
}

/// Subscription type for message threads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscriptionType {
    Bundle,
    BundleItem,
    Unbundled,
}

/// Dropdown action in thread details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DropdownActionEnum {
    AddRecipient,
    Forwarding,
    ToggleMute,
    Leave,
    ToggleSensitive,
    ExportThreadToDocument,
    MarkAsImportant,
    MoveToFolder,
    Delete,
    ToggleReadStatus,
    CreateDocument,
}

/// More option on a single message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageMoreOption {
    Edit,
    Delete,
    Forward,
}

/// Thread type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreadType {
    Thread,
    EventReminder,
    VacationRequestReminder,
}

/// Recipient API type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipientApiType {
    Unknown,
    InstitutionProfile,
    CommonInbox,
    OtpInbox,
}

/// Folder type for message folders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FolderType {
    Normal,
    Deleted,
    ButtonCell,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roundtrip_test {
        ($name:ident, $ty:ty, $variant:expr) => {
            #[test]
            fn $name() {
                let json = serde_json::to_string(&$variant).unwrap();
                let back: $ty = serde_json::from_str(&json).unwrap();
                assert_eq!(back, $variant);
            }
        };
    }

    roundtrip_test!(message_type, MessageType, MessageType::SystemForward);
    roundtrip_test!(
        sensitivity_level,
        SensitivityLevel,
        SensitivityLevel::Level2
    );
    roundtrip_test!(
        subscription_status,
        SubscriptionStatus,
        SubscriptionStatus::Unread
    );
    roundtrip_test!(recipient_type, RecipientType, RecipientType::CommonInbox);
    roundtrip_test!(
        recipients_target,
        RecipientsTarget,
        RecipientsTarget::CalendarEvent
    );
    roundtrip_test!(
        thread_click,
        MessageThreadClickType,
        MessageThreadClickType::MultiMove
    );
    roundtrip_test!(
        bundled_message,
        BundledMessageType,
        BundledMessageType::PrimaryMessage
    );
    roundtrip_test!(
        common_inbox_type,
        CommonInboxType,
        CommonInboxType::Institutional
    );
    roundtrip_test!(
        message_form,
        MessageFormType,
        MessageFormType::ForwardSingleMessage
    );
    roundtrip_test!(
        thread_cell_menu,
        MessageThreadCellMoreMenuActionEnum,
        MessageThreadCellMoreMenuActionEnum::Forward
    );
    roundtrip_test!(send_button, SendMessageButton, SendMessageButton::ReplyAll);
    roundtrip_test!(
        subscription_type,
        SubscriptionType,
        SubscriptionType::Unbundled
    );
    roundtrip_test!(
        dropdown_action,
        DropdownActionEnum,
        DropdownActionEnum::ToggleMute
    );
    roundtrip_test!(message_more, MessageMoreOption, MessageMoreOption::Delete);
    roundtrip_test!(thread_type, ThreadType, ThreadType::EventReminder);
    roundtrip_test!(
        recipient_api_type,
        RecipientApiType,
        RecipientApiType::OtpInbox
    );
    roundtrip_test!(folder_type, FolderType, FolderType::Deleted);

    #[test]
    fn send_button_screaming_snake() {
        let json = serde_json::to_string(&SendMessageButton::ReplySingle).unwrap();
        assert_eq!(json, r#""REPLY_SINGLE""#);
    }
}
