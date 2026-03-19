//! Notification service.
//!
//! Maps to `AulaNative.Services.Web.NotificationWebService` from the APK.
//!
//! # Endpoint paths
//!
//! All endpoints use RPC-style routing via `?method=notifications.<action>`.
//! Paths are sourced from the decompiled `Urls.cs` class.
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_NOTIFICATIONS_FOR_ACTIVE_PROFILE` | `notifications.getNotificationsForActiveProfile` |
//! | `DELETE_NOTIFICATIONS` | `notifications.deleteNotifications` |
//! | `DELETE_NOTIFICATIONS_FOR_RELATED_CHILDREN` | `notifications.deleteNotificationsByRelatedChild` |
//! | `DELETE_NOTIFICATIONS_BY_MODULE` | `notifications.deleteBadgeNotificationByModule` |

use crate::models::notifications::NotificationItemDto;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch in-app notifications for the active profile.
///
/// # Endpoint
///
/// `GET ?method=notifications.getNotificationsForActiveProfile`
pub async fn get_notifications(session: &mut Session) -> crate::Result<Vec<NotificationItemDto>> {
    session
        .get("?method=notifications.getNotificationsForActiveProfile")
        .await
}

/// Delete all notifications for the active profile.
///
/// # Endpoint
///
/// `POST ?method=notifications.deleteNotifications`
pub async fn delete_notifications(session: &mut Session) -> crate::Result<serde_json::Value> {
    session
        .post_empty("?method=notifications.deleteNotifications")
        .await
}

/// Delete notifications for a specific related child.
///
/// # Endpoint
///
/// `POST ?method=notifications.deleteNotificationsByRelatedChild`
pub async fn delete_notification_for_child(
    session: &mut Session,
    child_institution_profile_id: i64,
) -> crate::Result<serde_json::Value> {
    session
        .get(&format!(
            "?method=notifications.deleteNotificationsByRelatedChild&childInstitutionProfileId={child_institution_profile_id}"
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
                "notificationEventType": "newMessagePrivateInbox",
                "notificationArea": "messages",
                "notificationType": "alert",
                "title": "Ny besked",
                "threadId": 42
            }
        ]"#;
        let items: Vec<NotificationItemDto> = serde_json::from_str(json).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].notification_id.as_deref(), Some("n-1"));
    }
}
