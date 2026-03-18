//! Onboarding domain models.
//!
//! These types represent the onboarding/first-login flow in Aula,
//! where profiles confirm data policy acceptance and review their
//! institution profiles and children.
//!
//! See `data_models.md` DTOs.Onboarding.

use serde::{Deserialize, Serialize};

use crate::enums::profiles::PortalRole;

use super::profiles::{AulaFileContent, InstitutionProfileChild};

// ---------------------------------------------------------------------------
// Onboarding response types
// ---------------------------------------------------------------------------

/// Top-level onboarding response.
///
/// Maps to `DTOs.Onboarding.OnboardingResponseDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingResponseDto {
    pub profiles: Option<Vec<OnboardingProfileDto>>,
}

/// Per-profile onboarding information.
///
/// Maps to `DTOs.Onboarding.OnboardingProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingProfileDto {
    #[serde(default)]
    pub is_latest_data_policy_accepted: bool,
    pub portal_role: Option<PortalRole>,
    pub children: Option<Vec<StubbedChild>>,
    pub institution_profiles: Option<Vec<StubbedInstitutionProfile>>,
    pub over_consent_age: Option<bool>,
    pub contact_info_editable: Option<bool>,
}

/// Child reference in onboarding context.
///
/// Maps to `DTOs.Onboarding.StubbedChild`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StubbedChild {
    pub institution_profile: Option<InstitutionProfileChild>,
}

/// Institution profile stub for onboarding.
///
/// Maps to `DTOs.Onboarding.StubbedInstitutionProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StubbedInstitutionProfile {
    #[serde(default)]
    pub new_institution_profile: bool,
    pub id: Option<i64>,
    pub profile_id: Option<i64>,
    pub profile_picture: Option<AulaFileContent>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_onboarding_response() {
        let json = r#"{
            "profiles": [
                {
                    "isLatestDataPolicyAccepted": true,
                    "portalRole": "Guardian",
                    "children": [],
                    "institutionProfiles": [
                        {
                            "newInstitutionProfile": false,
                            "id": 42,
                            "profileId": 100,
                            "profilePicture": null
                        }
                    ],
                    "overConsentAge": null,
                    "contactInfoEditable": true
                }
            ]
        }"#;
        let r: OnboardingResponseDto = serde_json::from_str(json).unwrap();
        let profiles = r.profiles.as_ref().unwrap();
        assert_eq!(profiles.len(), 1);
        assert!(profiles[0].is_latest_data_policy_accepted);
        assert_eq!(profiles[0].portal_role, Some(PortalRole::Guardian));
        let ip = profiles[0].institution_profiles.as_ref().unwrap();
        assert_eq!(ip[0].id, Some(42));
        assert!(!ip[0].new_institution_profile);
    }

    #[test]
    fn deserialize_stubbed_institution_profile() {
        let json = r#"{
            "newInstitutionProfile": true,
            "id": 5,
            "profileId": 10,
            "profilePicture": {
                "id": 1,
                "name": "photo.jpg",
                "url": "https://cdn.aula.dk/photo.jpg",
                "bucket": "aula-photos",
                "key": "photos/1.jpg",
                "created": "2026-01-01",
                "scanningStatus": null
            }
        }"#;
        let s: StubbedInstitutionProfile = serde_json::from_str(json).unwrap();
        assert!(s.new_institution_profile);
        assert_eq!(
            s.profile_picture.as_ref().unwrap().name.as_deref(),
            Some("photo.jpg")
        );
    }
}
