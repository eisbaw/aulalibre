//! User profile, group, and institution enums.

use serde::{Deserialize, Serialize};

/// Portal-level role for a user.
///
/// The API returns lowercase values (`"guardian"`, `"employee"`, etc.) so we
/// use `rename_all = "camelCase"` which maps PascalCase variants to lowercase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PortalRole {
    Other,
    Employee,
    Child,
    Guardian,
    Otp,
}

/// Role within an institution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InstitutionRole {
    Unknown,
    Guardian,
    Daycare,
    Leader,
    PreschoolTeacher,
    Teacher,
    EarlyStudent,
    MiddleLateStudent,
    Child,
    Other,
}

/// Type of institution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstitutionTypeEnum {
    Unknown,
    School,
    Daycare,
    Municipality,
    Central,
}

/// Role of a user within a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupRole {
    Unknown,
    Member,
    Indirect,
    Applied,
    Removed,
    Rejected,
    Inactive,
}

/// Status of a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupStatus {
    Unidentified,
    Active,
    Inactive,
}

/// Relationship type between users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserRelationType {
    Others,
    Child,
    Guardian,
    Otp,
    Teacher,
}

/// Group action (join/leave).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupActionType {
    Leave,
    Join,
}

/// Group membership role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupMembershipRole {
    Other,
    Applied,
    Member,
    Removed,
    #[serde(rename = "Application_Removed")]
    ApplicationRemoved,
    Indirect,
}

/// Type of group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupTypeEnum {
    Unknown,
    Institutional,
    Municipal,
    #[serde(rename = "Cross_institutional")]
    CrossInstitutional,
    Other,
}

/// Group access type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupsAccessType {
    Other,
    Closed,
    Open,
    Application,
}

/// Contact list filter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContactListFilteringProfileTypeEnum {
    AllChildren,
    Boy,
    Girl,
    Employee,
    Guardian,
}

/// Gender in profile context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProfileRoleGenderEnum {
    Boy,
    Girl,
    Unknown,
}

/// Sort order for contact list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GetProfileContactSortOrderFieldEnum {
    Birthday,
    Name,
}

/// Sort order for personal reference data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GetPersonalReferenceDataOrderFieldEnum {
    Answers,
    DisplayName,
}

/// Login authentication method level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoginAuthenticationMethod {
    Unknown,
    Level2,
    Level3NemId,
    Level3Employees,
}

/// Return code from profile information update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateProfileInformationReturnCodeEnum {
    Success,
    Error,
    ErrorUserDeactivated,
    ErrorUserAccessDenied,
    WrongUserTypeLoggedInAsEmployee,
    WrongUserTypeLoggedInAsGuardian,
    WrongUserTypeLoggedInAsChild,
    #[serde(rename = "WrongUserTypeLoggedInAsOTP")]
    WrongUserTypeLoggedInAsOtp,
}

/// Onboarding step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnboardingStep {
    AppOnboarding,
    PolicyAcceptance,
    MasterData,
    Consents,
    AdditionalMasterData,
    NotificationSettings,
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

    roundtrip_test!(portal_role, PortalRole, PortalRole::Guardian);
    roundtrip_test!(institution_role, InstitutionRole, InstitutionRole::Teacher);
    roundtrip_test!(
        institution_type,
        InstitutionTypeEnum,
        InstitutionTypeEnum::School
    );
    roundtrip_test!(group_role, GroupRole, GroupRole::Member);
    roundtrip_test!(group_status, GroupStatus, GroupStatus::Active);
    roundtrip_test!(user_relation, UserRelationType, UserRelationType::Guardian);
    roundtrip_test!(group_action, GroupActionType, GroupActionType::Join);
    roundtrip_test!(
        group_membership,
        GroupMembershipRole,
        GroupMembershipRole::ApplicationRemoved
    );
    roundtrip_test!(group_type, GroupTypeEnum, GroupTypeEnum::CrossInstitutional);
    roundtrip_test!(groups_access, GroupsAccessType, GroupsAccessType::Open);
    roundtrip_test!(
        contact_filter,
        ContactListFilteringProfileTypeEnum,
        ContactListFilteringProfileTypeEnum::Boy
    );
    roundtrip_test!(
        profile_gender,
        ProfileRoleGenderEnum,
        ProfileRoleGenderEnum::Girl
    );
    roundtrip_test!(
        contact_sort,
        GetProfileContactSortOrderFieldEnum,
        GetProfileContactSortOrderFieldEnum::Birthday
    );
    roundtrip_test!(
        ref_data_sort,
        GetPersonalReferenceDataOrderFieldEnum,
        GetPersonalReferenceDataOrderFieldEnum::Answers
    );
    roundtrip_test!(
        login_auth,
        LoginAuthenticationMethod,
        LoginAuthenticationMethod::Level3NemId
    );
    roundtrip_test!(
        update_profile_return,
        UpdateProfileInformationReturnCodeEnum,
        UpdateProfileInformationReturnCodeEnum::WrongUserTypeLoggedInAsOtp
    );
    roundtrip_test!(onboarding_step, OnboardingStep, OnboardingStep::MasterData);

    #[test]
    fn group_membership_application_removed_rename() {
        let json = serde_json::to_string(&GroupMembershipRole::ApplicationRemoved).unwrap();
        assert_eq!(json, r#""Application_Removed""#);
    }

    #[test]
    fn group_type_cross_institutional_rename() {
        let json = serde_json::to_string(&GroupTypeEnum::CrossInstitutional).unwrap();
        assert_eq!(json, r#""Cross_institutional""#);
    }
}
