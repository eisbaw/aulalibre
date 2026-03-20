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
///
/// The web frontend sends `activeChildrenIds[]` and `activeInstitutionCodes[]`
/// query parameters to scope notifications to the active children.
pub async fn get_notifications(
    session: &mut Session,
    children_ids: &[i64],
    institution_codes: &[String],
) -> crate::Result<Vec<NotificationItemDto>> {
    let mut query = Vec::new();
    for id in children_ids {
        query.push(format!("activeChildrenIds[]={id}"));
    }
    for code in institution_codes {
        query.push(format!("activeInstitutionCodes[]={code}"));
    }

    let path = if query.is_empty() {
        "?method=notifications.getNotificationsForActiveProfile".to_string()
    } else {
        format!(
            "?method=notifications.getNotificationsForActiveProfile&{}",
            query.join("&")
        )
    };

    session.get(&path).await
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
