//! Push notification service.
//!
//! Maps to `AulaNative.Services.Web.RemoteNotificationWebService` (7 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.17.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `register_device` | POST | `/pushNotifications/devices` |
//! | `unregister_device` | DELETE | `/pushNotifications/devices/{id}` |
//! | `delete_all_devices` | DELETE | `/pushNotifications/devices` |
//! | `get_devices` | GET | `/pushNotifications/devices` |
//! | `get_notification_settings` | GET | `/pushNotifications/settings` |
//! | `update_notification_settings` | PUT | `/pushNotifications/settings` |
//! | `clear_notification_badges` | POST | `/pushNotifications/badges/clear` |

use crate::models::notifications::{ConfigureDeviceModel, NotificationSettings, SimpleDevice};
use crate::session::Session;

use serde::{Deserialize, Serialize};

/// Module identifier for clearing notification badges.
///
/// Inferred from `ClearNotificationBadgesByModule` method signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBadgesRequest {
    pub module: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Register a device for push notifications.
///
/// Maps to `RemoteNotificationWebService.RegisterDevice()`.
///
/// # Endpoint (inferred)
///
/// `POST /pushNotifications/devices`
pub async fn register_device(
    session: &mut Session,
    device: &ConfigureDeviceModel,
) -> crate::Result<serde_json::Value> {
    session.post("pushNotifications/devices", device).await
}

/// Unregister a specific device from push notifications.
///
/// Maps to `RemoteNotificationWebService.UnregisterDevice()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /pushNotifications/devices/{deviceId}`
pub async fn unregister_device(
    session: &mut Session,
    device_id: &str,
) -> crate::Result<serde_json::Value> {
    session
        .delete(&format!("pushNotifications/devices/{device_id}"))
        .await
}

/// Delete all registered devices for the current profile.
///
/// Maps to `RemoteNotificationWebService.DeleteAllDevices()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /pushNotifications/devices`
pub async fn delete_all_devices(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.delete("pushNotifications/devices").await
}

/// Get all registered devices for the current profile.
///
/// Maps to `RemoteNotificationWebService.GetDevices()`.
///
/// # Endpoint (inferred)
///
/// `GET /pushNotifications/devices`
pub async fn get_devices(session: &mut Session) -> crate::Result<Vec<SimpleDevice>> {
    session.get("pushNotifications/devices").await
}

/// Get the notification settings for the current profile.
///
/// Maps to `RemoteNotificationWebService.GetNotificationSettings()`.
///
/// # Endpoint (inferred)
///
/// `GET /pushNotifications/settings`
pub async fn get_notification_settings(
    session: &mut Session,
) -> crate::Result<NotificationSettings> {
    session.get("pushNotifications/settings").await
}

/// Update notification settings for the current profile.
///
/// Maps to `RemoteNotificationWebService.UpdateNotificationSettings()`.
///
/// # Endpoint (inferred)
///
/// `PUT /pushNotifications/settings`
pub async fn update_notification_settings(
    session: &mut Session,
    settings: &NotificationSettings,
) -> crate::Result<serde_json::Value> {
    session.put("pushNotifications/settings", settings).await
}

/// Clear notification badge counts for a specific module.
///
/// Maps to `RemoteNotificationWebService.ClearNotificationBadgesByModule()`.
///
/// # Endpoint (inferred)
///
/// `POST /pushNotifications/badges/clear`
pub async fn clear_notification_badges(
    session: &mut Session,
    request: &ClearBadgesRequest,
) -> crate::Result<serde_json::Value> {
    session
        .post("pushNotifications/badges/clear", request)
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::notifications::{ConfigureDeviceModel, NotificationSettings, SimpleDevice};

    #[test]
    fn configure_device_roundtrip() {
        let device = ConfigureDeviceModel {
            current_token: Some("fcm-token".into()),
            device_id: Some("dev-001".into()),
            device_description: Some("Test Phone".into()),
            device_access_granted: true,
            platform: Some(crate::enums::common::Platform::Android),
        };
        let json = serde_json::to_value(&device).unwrap();
        assert_eq!(json["currentToken"], "fcm-token");
        assert_eq!(json["deviceAccessGranted"], true);
    }

    #[test]
    fn clear_badges_request_serializes() {
        let req = ClearBadgesRequest {
            module: Some("Messages".into()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["module"], "Messages");
    }

    #[test]
    fn simple_device_list_deserializes() {
        let json = r#"[
            {"deviceId": "dev-001"},
            {"deviceId": "dev-002"}
        ]"#;
        let devices: Vec<SimpleDevice> = serde_json::from_str(json).unwrap();
        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].device_id.as_deref(), Some("dev-001"));
    }

    #[test]
    fn notification_settings_roundtrip() {
        let settings = NotificationSettings {
            scheduled_time: Some("07:00".into()),
            instant: true,
            monday: true,
            tuesday: true,
            wednesday: true,
            thursday: true,
            friday: true,
            saturday: false,
            sunday: false,
            notify_messages: true,
            notify_messages_from_employees: true,
            notify_messages_from_children: false,
            notify_messages_from_guardians: false,
            notify_calendar: true,
            notify_gallery: true,
            notify_posts: true,
            email_disabled: false,
            email_available: true,
            app_disabled: false,
            app_available: true,
            notify_assigned_as_substitute_teacher: false,
            notify_lesson_note_changed: true,
            come_go_notification_settings: None,
            device_list: None,
            widget_settings: None,
        };
        let json = serde_json::to_value(&settings).unwrap();
        assert_eq!(json["instant"], true);
        assert_eq!(json["saturday"], false);
        // Roundtrip
        let back: NotificationSettings = serde_json::from_value(json).unwrap();
        assert!(back.instant);
        assert!(!back.saturday);
    }
}
