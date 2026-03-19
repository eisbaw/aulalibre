//! Notification domain models.
//!
//! These types represent in-app notifications, push notification settings,
//! device registration, and notification configuration.
//!
//! See `notification_messaging.md` Sections 3-4 and `data_models.md`
//! Models.Calendar.CalenderNotification / Models.PushNotifications.

use serde::{Deserialize, Serialize};

use crate::enums::common::Platform;
use crate::enums::notifications::{NotificationArea, NotificationEventType, NotificationType};

use super::calendar::HtmlDto;
use super::profiles::InstitutionCode;

// ---------------------------------------------------------------------------
// Notification item (from GET /notifications)
// ---------------------------------------------------------------------------

/// In-app notification returned by `GET /notifications`.
///
/// Maps to `Models.Calendar.CalenderNotification.NotificationItemDto`.
/// This is the primary DTO for the notification feed, containing all possible
/// fields for any notification type (messages, events, posts, gallery, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationItemDto {
    pub notification_id: Option<String>,
    pub general_information_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub notification_event_type: Option<NotificationEventType>,
    pub notification_area: Option<NotificationArea>,
    pub notification_type: Option<NotificationType>,
    pub institution_code: Option<InstitutionCode>,
    pub expires: Option<String>,
    pub response_deadline: Option<String>,
    pub triggered: Option<String>,
    pub url: Option<String>,
    pub content: Option<HtmlDto>,
    pub related_child_institution_profile_id: Option<i64>,
    pub related_child_name: Option<String>,
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub event_id: Option<i64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub other_calendar_person_name: Option<String>,
    pub other_calendar_institution_profile_id: Option<i32>,
    pub responder_name: Option<String>,
    pub sender_name: Option<String>,
    pub message_text: Option<String>,
    pub related_institution: Option<String>,
    pub folder_id: Option<i64>,
    pub thread_id: Option<i64>,
    pub post_title: Option<String>,
    pub post_id: Option<i64>,
    pub group_name: Option<String>,
    pub group_id: Option<i64>,
    pub album_id: Option<i64>,
    pub album_name: Option<String>,
    pub media_id: Option<i64>,
    pub media_ids: Option<Vec<i64>>,
    pub document_id: Option<i64>,
    pub common_file_id: Option<i64>,
    pub room_name: Option<String>,
    pub event_start_time: Option<String>,
    pub event_end_time: Option<String>,
    pub vacation_registration_response_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
    pub common_inbox_name: Option<String>,
    pub note_to_guardians: Option<String>,
    #[serde(default)]
    pub is_presence_times_required: bool,
    pub vacation_request_name: Option<String>,
    pub notification_message: Option<String>,
    pub occurrence_date_time: Option<String>,
    pub cancelled_by: Option<String>,
    pub widget_id: Option<i32>,
    pub widget_name: Option<String>,
    pub message: Option<String>,
    pub resource_name: Option<String>,
    pub occurrence_date: Option<String>,
    pub exception_event_id: Option<i64>,
    pub comment_id: Option<i64>,
    pub profile_picture_institution_profile_id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Push notification settings
// ---------------------------------------------------------------------------

/// User notification settings controlling which push notifications are delivered.
///
/// Maps to `Models.PushNotifications.NotificationSettings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub scheduled_time: Option<String>,
    #[serde(default)]
    pub instant: bool,
    // Day-of-week filters
    #[serde(default)]
    pub monday: bool,
    #[serde(default)]
    pub tuesday: bool,
    #[serde(default)]
    pub wednesday: bool,
    #[serde(default)]
    pub thursday: bool,
    #[serde(default)]
    pub friday: bool,
    #[serde(default)]
    pub saturday: bool,
    #[serde(default)]
    pub sunday: bool,
    // Content channel toggles
    #[serde(default)]
    pub notify_messages: bool,
    #[serde(default)]
    pub notify_messages_from_employees: bool,
    #[serde(default)]
    pub notify_messages_from_children: bool,
    #[serde(default)]
    pub notify_messages_from_guardians: bool,
    #[serde(default)]
    pub notify_calendar: bool,
    #[serde(default)]
    pub notify_gallery: bool,
    #[serde(default)]
    pub notify_posts: bool,
    // Delivery platform controls
    #[serde(default)]
    pub email_disabled: bool,
    #[serde(default)]
    pub email_available: bool,
    #[serde(default)]
    pub app_disabled: bool,
    #[serde(default)]
    pub app_available: bool,
    // Special content toggles
    #[serde(default)]
    pub notify_assigned_as_substitute_teacher: bool,
    #[serde(default)]
    pub notify_lesson_note_changed: bool,
    // Nested settings
    pub come_go_notification_settings: Option<Vec<ComeGoNotificationSettings>>,
    pub device_list: Option<Vec<SimpleDevice>>,
    /// C# `NotificationSettings.WidgetSettings` has `[JsonProperty("widgetNotificationSettingDtos")]`.
    #[serde(rename = "widgetNotificationSettingDtos")]
    pub widget_settings: Option<Vec<WidgetNotificationSettings>>,
}

/// Presence/come-go notification channel setting.
///
/// Maps to `Models.PushNotifications.ComeGoNotificationSettings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoNotificationSettings {
    pub come_go_type: Option<String>,
    #[serde(default)]
    pub activated: bool,
}

/// Per-widget notification setting.
///
/// Maps to `Models.PushNotifications.WidgetNotificationSettings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetNotificationSettings {
    pub title: Option<String>,
    pub widget_id: Option<i32>,
    #[serde(default)]
    pub is_active: bool,
}

// ---------------------------------------------------------------------------
// Device registration
// ---------------------------------------------------------------------------

/// Device registration payload for push notifications.
///
/// Maps to `Models.PushNotifications.ConfigureDeviceModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureDeviceModel {
    pub current_token: Option<String>,
    pub device_id: Option<String>,
    pub device_description: Option<String>,
    #[serde(default)]
    pub device_access_granted: bool,
    pub platform: Option<Platform>,
}

/// Minimal device identity (just an ID).
///
/// Maps to `Models.PushNotifications.SimpleDevice`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDevice {
    pub device_id: Option<String>,
}

// ---------------------------------------------------------------------------
// Remote notification (parsed FCM payload)
// ---------------------------------------------------------------------------

/// Parsed remote/push notification payload.
///
/// Maps to `Models.PushNotifications.RemoteNotification`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteNotification {
    pub profile_id: Option<i64>,
    pub element_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub notification_type: Option<String>,
    pub related_child_inst_profile_id: Option<i64>,
    pub common_inbox_id: Option<i64>,
    pub comment_id: Option<i32>,
    pub occurrence_date_time: Option<String>,
    pub profile_picture_institution_profile_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_notification_item_minimal() {
        let json = r#"{
            "notificationId": "abc-123",
            "notificationEventType": "newMessagePrivateInbox",
            "notificationArea": "messages",
            "notificationType": "alert",
            "institutionCode": "101001",
            "title": "Ny besked",
            "threadId": 42
        }"#;
        let n: NotificationItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(n.notification_id.as_deref(), Some("abc-123"));
        assert_eq!(
            n.notification_event_type,
            Some(NotificationEventType::NewMessagePrivateInbox)
        );
        assert_eq!(n.notification_area, Some(NotificationArea::Messages));
        assert_eq!(n.notification_type, Some(NotificationType::Alert));
        assert_eq!(n.thread_id, Some(42));
    }

    #[test]
    fn deserialize_notification_item_full() {
        let json = r#"{
            "notificationId": "evt-456",
            "notificationEventType": "invitedToEventResponseRequired",
            "notificationArea": "calendar",
            "notificationType": "alert",
            "institutionCode": "101001",
            "institutionProfileId": 100,
            "title": "Forældremøde",
            "originalTitle": "Forældremøde 3.A",
            "eventId": 789,
            "startTime": "2026-04-01T10:00:00",
            "endTime": "2026-04-01T11:00:00",
            "responseDeadline": "2026-03-25T23:59:59",
            "triggered": "2026-03-18T08:00:00",
            "content": {"html": "<p>Invitation</p>"},
            "mediaIds": [1, 2, 3],
            "isPresenceTimesRequired": false,
            "threadId": 0
        }"#;
        let n: NotificationItemDto = serde_json::from_str(json).unwrap();
        assert_eq!(n.event_id, Some(789));
        assert_eq!(n.media_ids, Some(vec![1, 2, 3]));
        assert!(n.content.is_some());
    }

    #[test]
    fn deserialize_notification_settings() {
        let json = r#"{
            "scheduledTime": "07:00",
            "instant": true,
            "monday": true,
            "tuesday": true,
            "wednesday": true,
            "thursday": true,
            "friday": true,
            "saturday": false,
            "sunday": false,
            "notifyMessages": true,
            "notifyMessagesFromEmployees": true,
            "notifyMessagesFromChildren": false,
            "notifyMessagesFromGuardians": false,
            "notifyCalendar": true,
            "notifyGallery": true,
            "notifyPosts": true,
            "emailDisabled": false,
            "emailAvailable": true,
            "appDisabled": false,
            "appAvailable": true,
            "notifyAssignedAsSubstituteTeacher": false,
            "notifyLessonNoteChanged": true,
            "comeGoNotificationSettings": [
                {"comeGoType": "Arrival", "activated": true}
            ],
            "deviceList": [{"deviceId": "dev-001"}],
            "widgetNotificationSettingDtos": [
                {"title": "Skoleintra", "widgetId": 5, "isActive": true}
            ]
        }"#;
        let s: NotificationSettings = serde_json::from_str(json).unwrap();
        assert!(s.instant);
        assert!(s.monday);
        assert!(!s.saturday);
        assert!(s.notify_messages);
        assert_eq!(s.come_go_notification_settings.as_ref().unwrap().len(), 1);
        assert_eq!(s.device_list.as_ref().unwrap().len(), 1);
        assert_eq!(s.widget_settings.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_configure_device() {
        let json = r#"{
            "currentToken": "fcm-token-xyz",
            "deviceId": "device-001",
            "deviceDescription": "Samsung Galaxy S24",
            "deviceAccessGranted": true,
            "platform": "android"
        }"#;
        let d: ConfigureDeviceModel = serde_json::from_str(json).unwrap();
        assert_eq!(d.current_token.as_deref(), Some("fcm-token-xyz"));
        assert_eq!(d.platform, Some(Platform::Android));
        assert!(d.device_access_granted);
    }

    #[test]
    fn deserialize_remote_notification() {
        let json = r#"{
            "profileId": 42,
            "elementId": "elem-1",
            "id": "notif-1",
            "type": "NewMessagePrivateInbox",
            "relatedChildInstProfileId": 99,
            "commonInboxId": null,
            "commentId": null,
            "occurrenceDateTime": null,
            "profilePictureInstitutionProfileId": null
        }"#;
        let r: RemoteNotification = serde_json::from_str(json).unwrap();
        assert_eq!(r.profile_id, Some(42));
        assert_eq!(
            r.notification_type.as_deref(),
            Some("NewMessagePrivateInbox")
        );
    }
}
