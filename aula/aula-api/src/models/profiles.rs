//! Profile domain models.
//!
//! These types represent the layered identity model in Aula:
//! `User` -> `Profile` -> `InstitutionProfile[]`
//!
//! `InstitutionProfileId` is the pivot entity for nearly all API operations.
//! See `domain_concepts.md` Section 5.1 and `data_models.md` Models.ProfileModels.

use serde::{Deserialize, Serialize};

use crate::enums::common::PermissionEnum;
use crate::enums::profiles::{InstitutionRole, InstitutionTypeEnum, PortalRole};

use super::groups::Group;
use super::institutions::{Institution, InstitutionIdentity};

// ---------------------------------------------------------------------------
// Domain-specific type aliases
// ---------------------------------------------------------------------------

/// Institution profile identifier — the pivot entity for most API calls.
pub type InstitutionProfileId = i64;

/// Profile identifier (user-level, spans institutions).
pub type ProfileId = i64;

/// Institution code string (e.g. "101001").
pub type InstitutionCode = String;

// ---------------------------------------------------------------------------
// Value types
// ---------------------------------------------------------------------------

/// Postal address.
///
/// Maps to `Models.ProfileModels.Address`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: Option<String>,
    pub postal_code: Option<String>,
    pub postal_district: Option<String>,
}

/// Profile picture stored in S3/CDN.
///
/// Maps to `DTOs.ProfilePictureDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePictureDto {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub bucket: Option<String>,
    pub is_image_scaling_pending: Option<bool>,
    pub url: Option<String>,
    pub scanning_status: Option<String>,
}

/// File content reference (used for profile pictures in some contexts).
///
/// Maps to `Models.Common.Api.Files.Result.AulaFileContent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileContent {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub bucket: Option<String>,
    pub key: Option<String>,
    pub created: Option<String>,
    pub scanning_status: Option<String>,
}

/// Communication blocking flags per profile type.
///
/// Maps to `Models.ProfileModels.BlockedCommunication`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockedCommunicationInfo {
    #[serde(default)]
    pub child: bool,
    #[serde(default)]
    pub employee: bool,
    #[serde(default)]
    pub guardian: bool,
    #[serde(default)]
    pub is_blocked_all_profile_types: bool,
}

// ---------------------------------------------------------------------------
// Profile hierarchy
// ---------------------------------------------------------------------------

/// Base fields shared by all institution profile variants.
///
/// Maps to `Models.ProfileModels.InstitutionProfileBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfileBase {
    pub institution_profile_id: InstitutionProfileId,
    pub profile_id: ProfileId,
    pub uni_person_id: Option<i64>,
    pub mail_box_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub encryption_key: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub main_group: Option<String>,
}

/// Full institution profile with contact info and relations.
///
/// Maps to `Models.ProfileModels.InstitutionProfile` (extends InstitutionProfileBase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfile {
    // -- base fields --
    pub institution_profile_id: InstitutionProfileId,
    pub profile_id: ProfileId,
    pub uni_person_id: Option<i64>,
    pub mail_box_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub encryption_key: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub main_group: Option<String>,

    // -- extended fields --
    pub institution_role: Option<InstitutionRole>,
    #[serde(default)]
    pub communication_block: bool,
    #[serde(default)]
    pub upload_block: bool,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
    pub birthday: Option<String>,
    pub relations: Option<Vec<RelationProfile>>,
    #[serde(default)]
    pub alias: bool,
    pub group_home_id: Option<i64>,
    pub institution: Option<InstitutionIdentity>,
}

/// Institution profile for a child, with institution name/code on the struct.
///
/// Maps to `Models.ProfileModels.InstitutionProfileChild` (extends InstitutionProfileBase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionProfileChild {
    // -- base fields --
    pub institution_profile_id: InstitutionProfileId,
    pub profile_id: ProfileId,
    pub uni_person_id: Option<i64>,
    pub mail_box_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub encryption_key: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub main_group: Option<String>,

    // -- extended fields --
    pub institution_role: Option<InstitutionRole>,
    #[serde(default)]
    pub communication_block: bool,
    #[serde(default)]
    pub upload_block: bool,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
    pub birthday: Option<String>,
    pub relations: Option<Vec<RelationProfile>>,
    #[serde(default)]
    pub alias: bool,
    pub institution_code: Option<InstitutionCode>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
}

/// Simplified child profile (used in institution children lists).
///
/// Maps to `Models.ProfileModels.ChildProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildProfile {
    pub inst_code: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub inst_profile_id: Option<InstitutionProfileId>,
    pub profile_id: Option<ProfileId>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub user_id: Option<String>,
    #[serde(default)]
    pub has_custody_or_extended_access: bool,
    #[serde(default)]
    pub selected: bool,
}

/// A related user (guardian, child, etc.) as seen from an institution profile.
///
/// Maps to `Models.ProfileModels.RelationProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationProfile {
    pub institution_profile_id: Option<InstitutionProfileId>,
    pub profile_id: Option<ProfileId>,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub last_name: Option<String>,
    pub mail_box_id: Option<i64>,
    pub short_name: Option<String>,
    pub main_group_name: Option<String>,
    pub metadata: Option<String>,
    pub profile_picture: Option<AulaFileContent>,
    pub institution: Option<Institution>,
    pub role: Option<PortalRole>,
}

/// Child's relation profile (simplified, used in search results).
///
/// Maps to `Models.ProfileModels.ChildRelationsProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildRelationsProfile {
    pub profile_id: Option<ProfileId>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
    pub role: Option<String>,
    pub aula_email: Option<String>,
}

// ---------------------------------------------------------------------------
// Top-level profile
// ---------------------------------------------------------------------------

/// Editor plugin detail (widget/plugin metadata).
///
/// Maps to `Models.ProfileModels.EditorPluginDetail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorPluginDetail {
    pub name: Option<String>,
    pub municipal_code: Option<String>,
    pub institution_type: Option<InstitutionTypeEnum>,
}

/// Main group for a profile.
///
/// Maps to `Models.ProfileModels.MainGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainGroup {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
    /// C# `MainGroup.IsMainGroup` has `[JsonProperty("MainGroup")]`.
    #[serde(default, rename = "MainGroup")]
    pub is_main_group: bool,
}

/// Widget configuration on a page.
///
/// Maps to `Models.ProfileModels.WidgetConfigurationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetConfigurationDto {
    pub id: Option<i32>,
    pub widget: Option<WidgetDto>,
    pub placement: Option<String>,
    pub aggregated_display_mode: Option<String>,
    pub order: Option<i32>,
}

/// Widget definition.
///
/// Maps to `Models.ProfileModels.WidgetDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetDto {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_employee: Option<String>,
    pub icon_hover: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub widget_type: Option<String>,
    #[serde(default)]
    pub usable_for_groups: bool,
    pub ordering: Option<i32>,
    pub widget_id: Option<String>,
    pub widget_version: Option<String>,
    #[serde(default)]
    pub can_access_on_mobile: bool,
}

/// Page configuration with widgets and editor plugins.
///
/// Maps to `Models.ProfileModels.PageConfiguration`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageConfiguration {
    pub widget_configurations: Option<Vec<WidgetConfigurationDto>>,
    pub editor_plugin_details: Option<Vec<EditorPluginDetail>>,
}

/// Module definition.
///
/// Maps to `Models.ProfileModels.ModuleDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleDto {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub module_type: Option<String>,
    pub ordering: Option<i32>,
    #[serde(default)]
    pub can_be_placed_on_group: bool,
}

/// Module configuration.
///
/// Maps to `Models.ProfileModels.ModuleConfigurationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleConfigurationDto {
    pub id: Option<i32>,
    pub module: Option<ModuleDto>,
    pub order: Option<i32>,
    pub aggregated_display_mode: Option<String>,
}

/// Role definition.
///
/// Maps to `Models.ProfileModels.RoleDefinition`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDefinition {
    pub id: Option<i32>,
    pub role_name: Option<String>,
}

/// Top-level profile combining identity with groups and contact info.
///
/// Maps to `Models.ProfileModels.Profile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Option<ProfileId>,
    pub institution_profile: Option<InstitutionProfile>,
    pub groups: Option<Vec<Group>>,
    pub municipal_groups: Option<Vec<Group>>,
    pub phonenumber: Option<String>,
    /// C# `Profile.ExternalEmail` has `[JsonProperty("email")]`.
    #[serde(rename = "email")]
    pub external_email: Option<String>,
    pub work_phonenumber: Option<String>,
    pub home_phonenumber: Option<String>,
    pub mobile_phonenumber: Option<String>,
    pub administrator: Option<serde_json::Value>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_id: Option<String>,
    pub portal_role: Option<String>,
    #[serde(default)]
    pub is_stepped_up: bool,
    pub group_homes: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub is_group_home_admin: bool,
    pub page_configuration: Option<PageConfiguration>,
}

// ---------------------------------------------------------------------------
// Stubbed / simplified profile types
// ---------------------------------------------------------------------------

/// Permission held by an institution profile.
///
/// Maps to `Models.Institutions.Permission`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub permission_id: Option<PermissionEnum>,
    #[serde(default)]
    pub step_up: bool,
    pub group_scopes: Option<Vec<i32>>,
    #[serde(default)]
    pub institution_scope: bool,
}

/// Simplified institution profile (used in search, presence, etc.).
///
/// Maps to `Models.ProfileModels.StubbedUsers.SimpleInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleInstitutionProfile {
    pub profile_id: Option<ProfileId>,
    pub institution_profile_id: Option<InstitutionProfileId>,
    pub institution_code: Option<InstitutionCode>,
    pub institution_name: Option<String>,
    pub name: Option<String>,
    pub role: Option<PortalRole>,
    pub main_group: Option<String>,
    pub profile_picture: Option<serde_json::Value>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
}

/// Stubbed user (minimal user reference).
///
/// Maps to `Models.ProfileModels.StubbedUsers.StubbedUser`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StubbedUser {
    pub profile_id: Option<ProfileId>,
    pub name: Option<String>,
}

/// Child metadata used in stubbed user contexts.
///
/// Maps to `Models.ProfileModels.StubbedUsers.ChildMetadata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildMetadata {
    pub profile_id: Option<ProfileId>,
    pub name: Option<String>,
    pub id: Option<i64>,
    pub metadata: Option<String>,
    pub profile_picture: Option<AulaFileContent>,
}

/// Employee metadata (simplified).
///
/// Maps to `Models.ProfileModels.StubbedUsers.EmployeeMetadata`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeMetadata {
    pub name: Option<String>,
    pub institution_profile_id: Option<InstitutionProfileId>,
    pub institution_role: Option<InstitutionRole>,
}

/// ComeGo-specific institution profile view.
///
/// Maps to `Models.ProfileModels.StubbedUsers.ComeGoInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoInstitutionProfile {
    pub profile_id: Option<ProfileId>,
    pub institution_profile_id: Option<InstitutionProfileId>,
    pub name: Option<String>,
    pub role: Option<PortalRole>,
    pub profile_picture: Option<serde_json::Value>,
    pub short_name: Option<String>,
    pub institution_code: Option<InstitutionCode>,
}

// ---------------------------------------------------------------------------
// Contact list models
// ---------------------------------------------------------------------------

/// Institution profile augmented with contact list fields.
///
/// Maps to `Models.ProfileModels.Contact.ContactListInstitutionProfileResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactListInstitutionProfile {
    // Inherits InstitutionProfile fields via flattening
    #[serde(flatten)]
    pub institution_profile: InstitutionProfile,
    pub profile_picture_url: Option<String>,
    #[serde(default)]
    pub user_has_given_consent_to_show_contact_information: bool,
    #[serde(default)]
    pub current_user_can_view_contact_information: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_address() {
        let json = r#"{
            "street": "Vestergade 12",
            "postalCode": "8000",
            "postalDistrict": "Aarhus C"
        }"#;
        let addr: Address = serde_json::from_str(json).unwrap();
        assert_eq!(addr.street.as_deref(), Some("Vestergade 12"));
        assert_eq!(addr.postal_code.as_deref(), Some("8000"));
    }

    #[test]
    fn deserialize_child_profile() {
        let json = r#"{
            "instCode": "101001",
            "name": "Test Child",
            "shortName": "TC",
            "instProfileId": 42,
            "profileId": 7,
            "profilePicture": null,
            "userId": "u123",
            "hasCustodyOrExtendedAccess": true,
            "selected": false
        }"#;
        let child: ChildProfile = serde_json::from_str(json).unwrap();
        assert_eq!(child.inst_profile_id, Some(42));
        assert!(child.has_custody_or_extended_access);
    }

    #[test]
    fn deserialize_profile_minimal() {
        let json = r#"{
            "id": 100,
            "institutionProfile": null,
            "groups": null,
            "municipalGroups": null,
            "phonenumber": null,
            "email": null,
            "workPhonenumber": null,
            "homePhonenumber": null,
            "mobilePhonenumber": null,
            "administrator": null,
            "firstName": "Lars",
            "lastName": "Hansen",
            "userId": "u456",
            "portalRole": "Guardian",
            "isSteppedUp": false,
            "groupHomes": null,
            "isGroupHomeAdmin": false,
            "pageConfiguration": null
        }"#;
        let profile: Profile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.id, Some(100));
        assert_eq!(profile.first_name.as_deref(), Some("Lars"));
        assert!(!profile.is_stepped_up);
    }

    #[test]
    fn deserialize_relation_profile() {
        let json = r#"{
            "institutionProfileId": 55,
            "profileId": 10,
            "firstName": "Anne",
            "fullName": "Anne Jensen",
            "lastName": "Jensen",
            "mailBoxId": 3,
            "shortName": "AJ",
            "mainGroupName": "3.A",
            "metadata": null,
            "profilePicture": null,
            "institution": null,
            "role": "Guardian"
        }"#;
        let rel: RelationProfile = serde_json::from_str(json).unwrap();
        assert_eq!(rel.institution_profile_id, Some(55));
        assert_eq!(rel.role, Some(PortalRole::Guardian));
    }

    #[test]
    fn deserialize_permission() {
        let json = r#"{
            "permissionId": "HANDLE_EVENTS",
            "stepUp": false,
            "groupScopes": [1, 2, 3],
            "institutionScope": true
        }"#;
        let perm: Permission = serde_json::from_str(json).unwrap();
        assert_eq!(perm.permission_id, Some(PermissionEnum::HandleEvents));
        assert!(perm.institution_scope);
        assert_eq!(perm.group_scopes.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn deserialize_blocked_communication() {
        let json = r#"{
            "child": true,
            "employee": false,
            "guardian": true,
            "isBlockedAllProfileTypes": false
        }"#;
        let bc: BlockedCommunicationInfo = serde_json::from_str(json).unwrap();
        assert!(bc.child);
        assert!(!bc.employee);
        assert!(bc.guardian);
    }

    #[test]
    fn deserialize_institution_profile() {
        let json = r#"{
            "institutionProfileId": 123,
            "profileId": 456,
            "uniPersonId": 789,
            "mailBoxId": null,
            "firstName": "Test",
            "lastName": "User",
            "fullName": "Test User",
            "shortName": "TU",
            "metadata": null,
            "role": "Teacher",
            "encryptionKey": null,
            "profilePicture": null,
            "mainGroup": "5.B",
            "institutionRole": "Teacher",
            "communicationBlock": false,
            "uploadBlock": false,
            "email": "test@school.dk",
            "phone": "12345678",
            "address": null,
            "birthday": null,
            "relations": [],
            "alias": false,
            "groupHomeId": null,
            "institution": null
        }"#;
        let ip: InstitutionProfile = serde_json::from_str(json).unwrap();
        assert_eq!(ip.institution_profile_id, 123);
        assert_eq!(ip.profile_id, 456);
        assert_eq!(ip.institution_role, Some(InstitutionRole::Teacher));
        assert_eq!(ip.email.as_deref(), Some("test@school.dk"));
    }

    #[test]
    fn type_aliases_are_correct() {
        let _ipid: InstitutionProfileId = 42i64;
        let _pid: ProfileId = 7i64;
        let _ic: InstitutionCode = "101001".to_string();
    }
}
