//! Notification-related enums.

use serde::{Deserialize, Serialize};

/// Area/module a notification belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationArea {
    Unknown,
    Messages,
    Calendar,
    Posts,
    Schedule,
    Administration,
    Gallery,
    Documents,
    Album,
    Presence,
    Widget,
    FileScanning,
}

/// Specific notification event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEventType {
    Other,
    NewMessagePrivateInbox,
    NewSensitiveMessagePrivateInbox,
    InvitedToEventNoResponseRequired,
    InvitedToEventResponseRequired,
    EventChangedResponseRequired,
    EventChangedNoResponseRequired,
    EventCancelledNoReason,
    InvitedToParentalMeeting,
    InvitedToSchoolHomeMeeting,
    InvitedToPerformanceMeeting,
    GrantedRightsForOtherCalendar,
    RemovedRightsForOthersCalendar,
    LostRoomBecauseOfExternalScheduling,
    SomeoneElseRemovedYourResponseToEvent,
    SomeoneElseRespondedToEvent,
    PostSharedWithMe,
    PostWasRemovedFromGroupByAdmin,
    PostWasDeleted,
    SubstituteAdded,
    DashboardWasUpdated,
    NewMedia,
    TaggedInMedia,
    TaggedInMediaWithoutPushNotification,
    MediaAddedToAlbum,
    AlbumShared,
    NewOrUpdatedSecureDocument,
    NewCommonFile,
    VacationResponseRequired,
    InvitedToRepeatingEvent,
    PresenceRegistrationUpdatedExitWith,
    UpdatedSharingCommonFile,
    NewOrUpdatedCommonFile,
    VacationRequestCancelledNoReason,
    NewMessageCommonInbox,
    NewSensitiveMessageCommonInbox,
    AddedToCommonInbox,
    RemovedFromCommonInbox,
    LessonNoteChanged,
    EventChangedBySomeoneElse,
    EventCancelledBySomeoneElse,
    WidgetNotification,
    InvitedToExceptionEvent,
    InvitedToSingleOccurrenceOfEvent,
    SingleOccurrenceEventCancelledNoReason,
    InvitedToSurvey,
    DeletionRulesInfo,
    ResourceReservationDeclined,
    GeneralInformationSent,
    NewPostComment,
    OSOutdated,
    FileScanFailedPost,
    FileScanFailedEvent,
    FileScanFailedPrivateInboxMessage,
    FileScanFailedCommonInboxMessage,
    FileScanFailedInternalSecureDocument,
    FileScanFailedExternalSecureFile,
    FileScanFailedAlbum,
    FileScanFailedProfilePicture,
    FileScanFailedCommonFile,
}

/// Category grouping for notification event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEventTypeCategories {
    AlertInviteWithResponse,
    AlertChangedWithResponse,
    AlertChangedWithoutResponse,
    AlertInviteAssignedAsSubstituteTeacher,
    AlertInviteGoToAfterClickAndShowCross,
    AlertInvite,
    NotificationWithoutAlert,
    Posts,
    Messages,
    Schedule,
    Other,
    AlertSurveyInvite,
    AlertDeletionRulesInfo,
    AlertResourceReservationDeclined,
    AlertGeneralInfo,
    AlertOSOutdated,
    FileScanFailed,
}

/// Notification settings section/subsection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationSettingsEnum {
    ContentSettingsSection,
    PlatformSettingsSection,
    MessageSubsection,
    CalendarSubsection,
    MediaSubsection,
    PostsSubsection,
    ComeGoSubsection,
    WidgetsSubsection,
    EmployeeAllowed,
    GuardianAllowed,
    ChildAllowed,
    CalendarAllowed,
    NotifySubstituteTeacherAllowed,
    NewMediaAllowed,
    NewPostsAllowed,
    VacationRegistrationRequestAllow,
    PickupActivityAllow,
    NotifyLessonsChangeAllowed,
    WidgetsAllowed,
    MobileNotAvailableText,
    MobileNotAvailableInfo,
    MobileSubsection,
    EmailSubsection,
    ConsentSubsection,
    MobileAllowed,
    EmailAllowed,
    Instant,
    DatePicker,
    AllDay,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    SetTime,
    SetDateInWeek,
    EmailViews,
}

/// High-level notification classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Badge,
    Alert,
    Irrelevant,
    Unknown,
}

/// Remote/push notification event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RemoteNotificationType {
    PostSharedWithMe,
    NewMessagePrivateInbox,
    NewSensitiveMessagePrivateInbox,
    InvitedToEventNoResponseRequired,
    InvitedToEventResponseRequired,
    InvitedToRepeatingEventResponseRequired,
    EventChangedResponseRequired,
    EventChangedNoResponseRequired,
    EventCancelledNoReason,
    InvitedToParentalMeeting,
    InvitedToSchoolHomeMeeting,
    InvitedToPerformanceMeeting,
    GrantedRightsForOtherCalendar,
    RemovedRightsForOthersCalendar,
    LostRoomBecauseOfExternalScheduling,
    SomeoneElseRespondedToEvent,
    SomeoneElseRemovedYourResponseToEvent,
    PostWasRemovedFromGroupByAdmin,
    PostWasDeleted,
    SubstituteAdded,
    DashboardWasUpdated,
    AlbumShared,
    MediaAddedToAlbum,
    TaggedInMedia,
    NewMedia,
    NewCommonFile,
    NewOrUpdatedSecureDocument,
    PresenceRegistrationUpdatedExitWith,
    VacationResponseRequired,
    VacationResponseCancelledNoResponse,
    NewMessageCommonInbox,
    NewSensitiveMessageCommonInbox,
    Unknown,
    LessonNoteChanged,
    EventChangedBySomeoneElse,
    EventCancelledBySomeoneElse,
    WidgetPushNotification,
    InvitedToExceptionEvent,
    InvitedToSingleOccurrenceOfEvent,
    InvitedToSurvey,
    GeneralInformation,
    NewPostComment,
    FileScanFailedPost,
    FileScanFailedEvent,
    FileScanFailedPrivateInboxMessage,
    FileScanFailedCommonInboxMessage,
    FileScanFailedInternalSecureDocument,
    FileScanFailedExternalSecureFile,
    FileScanFailedAlbum,
    FileScanFailedProfilePicture,
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

    roundtrip_test!(
        notification_area,
        NotificationArea,
        NotificationArea::Calendar
    );
    roundtrip_test!(
        notification_event_type,
        NotificationEventType,
        NotificationEventType::InvitedToEventResponseRequired
    );
    roundtrip_test!(
        notification_event_categories,
        NotificationEventTypeCategories,
        NotificationEventTypeCategories::AlertInviteWithResponse
    );
    roundtrip_test!(
        notification_settings,
        NotificationSettingsEnum,
        NotificationSettingsEnum::CalendarAllowed
    );
    roundtrip_test!(notification_type, NotificationType, NotificationType::Alert);
    roundtrip_test!(
        remote_notification_type,
        RemoteNotificationType,
        RemoteNotificationType::WidgetPushNotification
    );
}
