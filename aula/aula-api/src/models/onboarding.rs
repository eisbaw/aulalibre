//! Onboarding domain models.
//!
//! These types represent the response from `getProfilesByLogin` (the initial
//! profile fetch after authentication). The real API returns an
//! `OnboardingResponseDto` wrapping rich institution profiles and children --
//! not the simpler `Profile` type used elsewhere.
//!
//! See `data_models.md` DTOs.Onboarding.

use serde::{Deserialize, Serialize};

use super::profiles::{Address, InstitutionProfileId, ProfileId};

// ---------------------------------------------------------------------------
// Onboarding response types
// ---------------------------------------------------------------------------

/// Top-level onboarding response.
///
/// Maps to `DTOs.Onboarding.OnboardingResponseDto`.
/// This is what `getprofilesbylogin` returns inside the `data` envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingResponseDto {
    pub profiles: Vec<OnboardingProfileDto>,
}

/// Per-profile onboarding information.
///
/// Maps to `DTOs.Onboarding.OnboardingProfileDto`.
/// Each entry represents one user identity (typically one per institution).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingProfileDto {
    pub profile_id: Option<ProfileId>,
    pub display_name: Option<String>,
    pub portal_role: Option<String>,
    #[serde(default)]
    pub is_latest_data_policy_accepted: bool,
    #[serde(default)]
    pub support_role: bool,
    pub over_consent_age: Option<bool>,
    pub contact_info_editable: Option<bool>,
    pub age_18_and_older: Option<bool>,
    pub institution_profiles: Option<Vec<LoginInstitutionProfile>>,
    pub children: Option<Vec<LoginChild>>,
}

/// Rich institution profile as returned by `getprofilesbylogin`.
///
/// The real API returns far more fields than the decompiled C#
/// `StubbedInstitutionProfile` stub suggests. This struct captures the
/// fields we need, using `Option<serde_json::Value>` for complex nested
/// objects we don't parse yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInstitutionProfile {
    /// Institution profile ID (the pivot entity for most API calls).
    /// Note: the JSON key is `"id"`, not `"institutionProfileId"`.
    pub id: InstitutionProfileId,
    pub profile_id: ProfileId,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub municipality_code: Option<String>,
    pub municipality_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub gender: Option<String>,
    pub role: Option<String>,
    pub email: Option<String>,
    pub mobile_phone_number: Option<String>,
    pub metadata: Option<String>,
    #[serde(default)]
    pub new_institution_profile: bool,
    #[serde(default)]
    pub is_primary: bool,
    pub address: Option<Address>,
    pub profile_picture: Option<serde_json::Value>,
    // Fields we don't need to parse in detail yet
    pub groups: Option<serde_json::Value>,
    pub permissions: Option<serde_json::Value>,
    pub main_group: Option<serde_json::Value>,
    pub page_configuration: Option<serde_json::Value>,
    pub module_configurations: Option<serde_json::Value>,
    pub blocked_communication: Option<serde_json::Value>,
}

/// Child entry as returned by `getprofilesbylogin`.
///
/// Has both top-level convenience fields and a nested `institutionProfile`
/// with full institution-scoped details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginChild {
    pub id: Option<i64>,
    pub profile_id: Option<ProfileId>,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub institution_code: Option<String>,
    #[serde(default)]
    pub has_custody_or_extended_access: bool,
    pub profile_picture: Option<serde_json::Value>,
    pub institution_profile: Option<LoginChildInstitutionProfile>,
}

/// Institution profile for a child within the login response.
///
/// Similar to `LoginInstitutionProfile` but for the child's own
/// institution membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginChildInstitutionProfile {
    pub id: Option<InstitutionProfileId>,
    pub profile_id: Option<ProfileId>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub role: Option<String>,
    pub gender: Option<String>,
    pub metadata: Option<String>,
    pub profile_picture: Option<serde_json::Value>,
    pub main_group: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Convenience methods
// ---------------------------------------------------------------------------

impl OnboardingProfileDto {
    /// Get the first institution profile ID, which is the primary pivot
    /// entity for most API calls.
    pub fn first_institution_profile_id(&self) -> Option<InstitutionProfileId> {
        self.institution_profiles.as_ref()?.first().map(|ip| ip.id)
    }

    /// Get all institution profile IDs for this profile.
    pub fn institution_profile_ids(&self) -> Vec<InstitutionProfileId> {
        self.institution_profiles
            .as_ref()
            .map(|ips| ips.iter().map(|ip| ip.id).collect())
            .unwrap_or_default()
    }

    /// Get children's names as a convenience.
    pub fn children_names(&self) -> Vec<String> {
        self.children
            .as_ref()
            .map(|kids| kids.iter().filter_map(|c| c.name.clone()).collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_onboarding_response() {
        let json = r#"{
            "profiles": [
                {
                    "profileId": 99001,
                    "displayName": "Henrik Jensen",
                    "isLatestDataPolicyAccepted": true,
                    "portalRole": "guardian",
                    "supportRole": false,
                    "overConsentAge": null,
                    "contactInfoEditable": null,
                    "age18AndOlder": null,
                    "children": [],
                    "institutionProfiles": [
                        {
                            "id": 12055,
                            "profileId": 6701,
                            "institutionCode": "280371",
                            "institutionName": "Bakkeskolen",
                            "firstName": "Henrik",
                            "lastName": "Jensen",
                            "fullName": "Henrik Jensen",
                            "role": "guardian",
                            "newInstitutionProfile": false,
                            "isPrimary": true,
                            "profilePicture": null
                        }
                    ]
                }
            ]
        }"#;
        let r: OnboardingResponseDto = serde_json::from_str(json).unwrap();
        assert_eq!(r.profiles.len(), 1);
        let p = &r.profiles[0];
        assert!(p.is_latest_data_policy_accepted);
        assert_eq!(p.portal_role.as_deref(), Some("guardian"));
        assert_eq!(p.profile_id, Some(99001));
        assert_eq!(p.display_name.as_deref(), Some("Henrik Jensen"));
        let ip = p.institution_profiles.as_ref().unwrap();
        assert_eq!(ip[0].id, 12055);
        assert!(!ip[0].new_institution_profile);
        assert!(ip[0].is_primary);
    }

    #[test]
    fn deserialize_login_institution_profile() {
        let json = r#"{
            "id": 99002,
            "profileId": 99001,
            "institutionCode": "X99999",
            "institutionName": "Testinstitutionen",
            "municipalityCode": "101",
            "municipalityName": "Testkommune",
            "firstName": "Henrik",
            "lastName": "Jensen",
            "fullName": "Henrik Jensen",
            "gender": "M",
            "role": "guardian",
            "shortName": "HJ",
            "newInstitutionProfile": false,
            "isPrimary": true,
            "email": "test@example.com",
            "mobilePhoneNumber": "12345678",
            "address": {
                "street": "Testvej 1",
                "postalCode": "2100",
                "postalDistrict": "Testby"
            },
            "profilePicture": {"id": 123, "url": "https://example.com/pic.jpg"},
            "metadata": "Child Name"
        }"#;
        let ip: LoginInstitutionProfile = serde_json::from_str(json).unwrap();
        assert_eq!(ip.id, 99002);
        assert_eq!(ip.profile_id, 99001);
        assert_eq!(ip.institution_name.as_deref(), Some("Testinstitutionen"));
        assert!(ip.is_primary);
        assert_eq!(ip.email.as_deref(), Some("test@example.com"));
        assert!(ip.address.is_some());
    }

    #[test]
    fn deserialize_login_child() {
        let json = r#"{
            "institutionProfile": {
                "id": 99003,
                "profileId": 99004,
                "institutionCode": "X99999",
                "institutionName": "Testinstitutionen",
                "firstName": "Barn",
                "lastName": "Jensen",
                "fullName": "Barn Jensen",
                "role": "child",
                "shortName": "BJ"
            },
            "id": 99003,
            "profileId": 99004,
            "userId": "1000025ce3",
            "name": "Barn Jensen",
            "profilePicture": null,
            "shortName": "BJ",
            "institutionCode": "X99999",
            "hasCustodyOrExtendedAccess": false
        }"#;
        let child: LoginChild = serde_json::from_str(json).unwrap();
        assert_eq!(child.name.as_deref(), Some("Barn Jensen"));
        assert_eq!(child.id, Some(99003));
        assert!(!child.has_custody_or_extended_access);
        let ip = child.institution_profile.as_ref().unwrap();
        assert_eq!(ip.full_name.as_deref(), Some("Barn Jensen"));
        assert_eq!(ip.role.as_deref(), Some("child"));
    }

    #[test]
    fn convenience_methods() {
        let profile = OnboardingProfileDto {
            profile_id: Some(100),
            display_name: Some("Test".to_string()),
            portal_role: Some("guardian".to_string()),
            is_latest_data_policy_accepted: true,
            support_role: false,
            over_consent_age: None,
            contact_info_editable: None,
            age_18_and_older: None,
            institution_profiles: Some(vec![LoginInstitutionProfile {
                id: 42,
                profile_id: 100,
                institution_code: Some("123".to_string()),
                institution_name: Some("Test School".to_string()),
                municipality_code: None,
                municipality_name: None,
                first_name: None,
                last_name: None,
                full_name: None,
                short_name: None,
                gender: None,
                role: None,
                email: None,
                mobile_phone_number: None,
                metadata: None,
                new_institution_profile: false,
                is_primary: true,
                address: None,
                profile_picture: None,
                groups: None,
                permissions: None,
                main_group: None,
                page_configuration: None,
                module_configurations: None,
                blocked_communication: None,
            }]),
            children: Some(vec![LoginChild {
                id: Some(10),
                profile_id: Some(11),
                user_id: None,
                name: Some("Child One".to_string()),
                short_name: None,
                institution_code: None,
                has_custody_or_extended_access: false,
                profile_picture: None,
                institution_profile: None,
            }]),
        };

        assert_eq!(profile.first_institution_profile_id(), Some(42));
        assert_eq!(profile.institution_profile_ids(), vec![42]);
        assert_eq!(profile.children_names(), vec!["Child One".to_string()]);
    }
}
