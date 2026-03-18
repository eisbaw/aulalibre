//! Group service.
//!
//! Maps to `AulaNative.Services.Web.GroupWebService` (4 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.14.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_group` | GET | `/groups/{id}` |
//! | `get_group_by_context` | GET | `/groups/context/{id}` |
//! | `get_memberships_light` | GET | `/groups/{id}/memberships` |
//! | `join_or_leave_group` | POST | `/groups/{id}/membership` |

use crate::models::groups::{Group, GroupByContextDto, GroupMembership};
use crate::session::Session;

use serde::{Deserialize, Serialize};

/// Request body for joining or leaving a group.
///
/// Inferred from `JoinOrLeaveGroup` method signature; the action field
/// likely specifies whether the user is joining or leaving.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinOrLeaveGroupRequest {
    pub action: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get a group by its ID.
///
/// Maps to `GroupWebService.GetGroup()`.
///
/// # Endpoint (inferred)
///
/// `GET /groups/{id}`
pub async fn get_group(session: &mut Session, group_id: i64) -> crate::Result<Group> {
    session.get(&format!("groups/{group_id}")).await
}

/// Get groups by context (e.g., for a specific institution profile).
///
/// Maps to `GroupWebService.GetGroupByContext()`.
///
/// # Endpoint (inferred)
///
/// `GET /groups/context/{contextId}`
pub async fn get_group_by_context(
    session: &mut Session,
    context_id: i64,
) -> crate::Result<Vec<GroupByContextDto>> {
    session.get(&format!("groups/context/{context_id}")).await
}

/// Get light membership list for a group.
///
/// Maps to `GroupWebService.GetMembershipsLight()`.
///
/// # Endpoint (inferred)
///
/// `GET /groups/{id}/memberships`
pub async fn get_memberships_light(
    session: &mut Session,
    group_id: i64,
) -> crate::Result<Vec<GroupMembership>> {
    session.get(&format!("groups/{group_id}/memberships")).await
}

/// Join or leave a group.
///
/// Maps to `GroupWebService.JoinOrLeaveGroup()`.
///
/// # Endpoint (inferred)
///
/// `POST /groups/{id}/membership`
pub async fn join_or_leave_group(
    session: &mut Session,
    group_id: i64,
    request: &JoinOrLeaveGroupRequest,
) -> crate::Result<serde_json::Value> {
    session
        .post(&format!("groups/{group_id}/membership"), request)
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::groups::{Group, GroupByContextDto, GroupMembership};

    #[test]
    fn join_or_leave_request_serializes() {
        let req = JoinOrLeaveGroupRequest {
            action: Some("join".into()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["action"], "join");
    }

    #[test]
    fn group_deserializes() {
        let json = r#"{
            "id": 1,
            "name": "3.A",
            "description": "Class 3A",
            "membershipType": null,
            "membershipInstitutions": ["101001"],
            "access": "Closed",
            "currentUserCanAccessGroupDashBoard": true,
            "status": "Active",
            "role": "Member",
            "dashboardEnabled": true,
            "institutionCode": "101001",
            "type": "Institutional",
            "validGroupModules": [],
            "allowMembersToBeShown": true,
            "validGroupWidgets": [],
            "memberships": [],
            "endTime": null
        }"#;
        let group: Group = serde_json::from_str(json).unwrap();
        assert_eq!(group.id, Some(1));
        assert_eq!(group.name.as_deref(), Some("3.A"));
    }

    #[test]
    fn group_by_context_list_deserializes() {
        let json = r#"[
            {"id": 1, "name": "3.A", "showAsDefault": true},
            {"id": 2, "name": "Fritids", "showAsDefault": false}
        ]"#;
        let groups: Vec<GroupByContextDto> = serde_json::from_str(json).unwrap();
        assert_eq!(groups.len(), 2);
        assert!(groups[0].show_as_default);
        assert!(!groups[1].show_as_default);
    }

    #[test]
    fn membership_list_deserializes() {
        let json = r#"[
            {
                "id": 10,
                "groupRole": "Member",
                "inactiveDate": null,
                "institutionProfile": null,
                "groupId": 1,
                "memberGroup": null,
                "institutionRole": "Teacher"
            }
        ]"#;
        let members: Vec<GroupMembership> = serde_json::from_str(json).unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].group_id, Some(1));
    }
}
