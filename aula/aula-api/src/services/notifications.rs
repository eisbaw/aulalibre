//! Notification service.
//!
//! Maps to `AulaNative.Services.Web.NotificationWebService` (3 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.16.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_notifications` | GET | `/notifications` |
//! | `delete_notifications` | DELETE | `/notifications` |
//! | `delete_notification_for_child` | DELETE | `/notifications/child/{id}` |

use crate::models::notifications::NotificationItemDto;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch in-app notifications for the active profile.
///
/// Maps to `NotificationWebService.GetNotificationsForActiveProfile()`.
///
/// # Endpoint (inferred)
///
/// `GET /notifications`
pub async fn get_notifications(session: &mut Session) -> crate::Result<Vec<NotificationItemDto>> {
    session.get("notifications").await
}

/// Delete all notifications for the active profile.
///
/// Maps to `NotificationWebService.DeleteNotifications()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /notifications`
pub async fn delete_notifications(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.delete("notifications").await
}

/// Delete notifications for a specific related child.
///
/// Maps to `NotificationWebService.DeleteNotificationForRelatedChild()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /notifications/child/{childInstitutionProfileId}`
pub async fn delete_notification_for_child(
    session: &mut Session,
    child_institution_profile_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .delete(&format!(
            "notifications/child/{child_institution_profile_id}"
        ))
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::notifications::NotificationItemDto;

    #[test]
    fn notification_list_deserializes() {
        let json = r#"[
            {
                "notificationId": "n-1",
                "notificationEventType": "NewMessagePrivateInbox",
                "notificationArea": "Messages",
                "notificationType": "Alert",
                "title": "Ny besked",
                "threadId": 42
            }
        ]"#;
        let items: Vec<NotificationItemDto> = serde_json::from_str(json).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].notification_id.as_deref(), Some("n-1"));
    }
}
