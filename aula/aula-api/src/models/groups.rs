//! Group and membership models.
//!
//! Maps to `Models.Groups` namespace from the decompiled assembly.
//! See `data_models.md` section "Models.Groups".

use serde::{Deserialize, Serialize};

use crate::enums::profiles::{GroupRole, GroupTypeEnum, InstitutionRole, PortalRole};

use super::profiles::InstitutionCode;

// ---------------------------------------------------------------------------
// Stubbed / simple groups
// ---------------------------------------------------------------------------

/// Minimal group reference (id + name).
///
/// Maps to `Models.Groups.StubbedGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StubbedGroup {
    pub id: Option<i64>,
    pub name: Option<String>,
}

/// Stubbed group extended with institution code.
///
/// Maps to `Models.Groups.GroupMemberGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberGroup {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
}

/// Simple group DTO with institution info.
///
/// Maps to `Models.Groups.SimpleGroupDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleGroupDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
    pub institution_name: Option<String>,
}

// ---------------------------------------------------------------------------
// Group module/widget references
// ---------------------------------------------------------------------------

/// Module enabled on a group.
///
/// Maps to `Models.Modules.GroupModule`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupModule {
    pub id: Option<i32>,
    pub module: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

/// Widget enabled on a group.
///
/// Maps to `Models.Widgets.GroupWidget`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupWidget {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

// ---------------------------------------------------------------------------
// Full group model
// ---------------------------------------------------------------------------

/// Full group with memberships, modules, and configuration.
///
/// Maps to `Models.Groups.Group`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub membership_type: Option<serde_json::Value>,
    pub membership_institutions: Option<Vec<String>>,
    pub access: Option<String>,
    #[serde(default)]
    pub current_user_can_access_group_dash_board: bool,
    pub status: Option<String>,
    pub role: Option<GroupRole>,
    #[serde(default)]
    pub dashboard_enabled: bool,
    pub institution_code: Option<InstitutionCode>,
    #[serde(rename = "type")]
    pub group_type: Option<GroupTypeEnum>,
    pub valid_group_modules: Option<Vec<GroupModule>>,
    #[serde(default)]
    pub allow_members_to_be_shown: bool,
    pub valid_group_widgets: Option<Vec<GroupWidget>>,
    pub memberships: Option<Vec<GroupMembership>>,
    pub end_time: Option<String>,
}

// ---------------------------------------------------------------------------
// Group membership
// ---------------------------------------------------------------------------

/// Institution profile as seen in a group membership context.
///
/// Maps to `Models.Groups.GroupMembershipInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipInstitutionProfile {
    // Base fields from InstitutionProfileBase
    pub institution_profile_id: i64,
    pub profile_id: i64,
    pub uni_person_id: Option<i64>,
    pub mail_box_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub encryption_key: Option<String>,
    pub profile_picture: Option<serde_json::Value>,
    pub main_group: Option<String>,
    // Extended
    pub main_group_name: Option<String>,
    pub relations: Option<Vec<RecipientRelation>>,
}

/// Membership of a profile in a group.
///
/// Maps to `Models.Groups.GroupMembership`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembership {
    pub id: Option<i64>,
    pub group_role: Option<GroupRole>,
    pub inactive_date: Option<String>,
    pub institution_profile: Option<GroupMembershipInstitutionProfile>,
    pub group_id: Option<i64>,
    pub member_group: Option<GroupMemberGroup>,
    pub institution_role: Option<InstitutionRole>,
}

/// Relation info as seen in group membership contexts.
///
/// Maps to `Models.Groups.RecipientRelation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipientRelation {
    pub inst_profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub main_group_name: Option<String>,
}

/// Group context for profile-based views.
///
/// Maps to `Models.Groups.GetGroupByProfileContext.GroupByContextDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupByContextDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    pub show_as_default: bool,
}

/// Grouping of memberships by portal role.
///
/// Maps to `Models.Groups.GroupMembershipGroupingByProfileTypeViewModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMembershipGroupingByProfileType {
    pub role: Option<PortalRole>,
    pub members: Option<Vec<serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_group() {
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
        assert_eq!(group.role, Some(GroupRole::Member));
        assert_eq!(group.group_type, Some(GroupTypeEnum::Institutional));
        assert!(group.dashboard_enabled);
    }

    #[test]
    fn deserialize_group_membership() {
        let json = r#"{
            "id": 10,
            "groupRole": "Member",
            "inactiveDate": null,
            "institutionProfile": null,
            "groupId": 1,
            "memberGroup": null,
            "institutionRole": "teacher"
        }"#;
        let gm: GroupMembership = serde_json::from_str(json).unwrap();
        assert_eq!(gm.id, Some(10));
        assert_eq!(gm.group_role, Some(GroupRole::Member));
        assert_eq!(gm.institution_role, Some(InstitutionRole::Teacher));
    }

    #[test]
    fn deserialize_recipient_relation() {
        let json = r#"{
            "instProfileId": 42,
            "firstName": "Morten",
            "lastName": "Pedersen",
            "fullName": "Morten Pedersen",
            "shortName": "MP",
            "metadata": null,
            "role": "Guardian",
            "mainGroupName": "3.A"
        }"#;
        let rr: RecipientRelation = serde_json::from_str(json).unwrap();
        assert_eq!(rr.inst_profile_id, Some(42));
        assert_eq!(rr.role.as_deref(), Some("Guardian"));
    }

    #[test]
    fn deserialize_simple_group_dto() {
        let json = r#"{
            "id": 5,
            "name": "Forældregruppe",
            "institutionCode": "101001",
            "institutionName": "Viby Skole"
        }"#;
        let sg: SimpleGroupDto = serde_json::from_str(json).unwrap();
        assert_eq!(sg.id, Some(5));
        assert_eq!(sg.institution_name.as_deref(), Some("Viby Skole"));
    }
}
