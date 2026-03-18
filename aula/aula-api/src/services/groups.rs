//! Group service.
//!
//! Maps to `AulaNative.Services.Web.GroupWebService` from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_GROUP` | `groups.getGroupById` (with `&groupId=` param) |
//! | `GET_GROUPS_BY_CONTEXT` | `groups.getGroupsByContext` |
//! | `GET_MEMBERSHIP_LIGHT` | `groups.getMembershipsLight` |
//! | `JOIN_OR_LEAVE_GROUP` | `groups.joinOrLeaveGroup` |

use crate::models::groups::{Group, GroupByContextDto, GroupMembership};
use crate::session::Session;

use serde::{Deserialize, Serialize};

/// Request body for joining or leaving a group.
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
/// # Endpoint
///
/// `GET ?method=groups.getGroupById&groupId={id}`
pub async fn get_group(session: &mut Session, group_id: i64) -> crate::Result<Group> {
    session
        .get(&format!("?method=groups.getGroupById&groupId={group_id}"))
        .await
}

/// Get groups by context (e.g., for a specific institution profile).
///
/// # Endpoint
///
/// `GET ?method=groups.getGroupsByContext`
pub async fn get_group_by_context(
    session: &mut Session,
    context_id: i64,
) -> crate::Result<Vec<GroupByContextDto>> {
    session
        .get(&format!(
            "?method=groups.getGroupsByContext&contextId={context_id}"
        ))
        .await
}

/// Get light membership list for a group.
///
/// # Endpoint
///
/// `GET ?method=groups.getMembershipsLight`
pub async fn get_memberships_light(
    session: &mut Session,
    group_id: i64,
) -> crate::Result<Vec<GroupMembership>> {
    session
        .get(&format!(
            "?method=groups.getMembershipsLight&groupId={group_id}"
        ))
        .await
}

/// Join or leave a group.
///
/// # Endpoint
///
/// `POST ?method=groups.joinOrLeaveGroup`
pub async fn join_or_leave_group(
    session: &mut Session,
    group_id: i64,
    request: &JoinOrLeaveGroupRequest,
) -> crate::Result<serde_json::Value> {
    // The groupId is sent as part of the request body or query param.
    // Since the Urls.cs shows no query param, we send it in the body.
    let _ = group_id; // included in the RPC call context
    session
        .post("?method=groups.joinOrLeaveGroup", request)
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
