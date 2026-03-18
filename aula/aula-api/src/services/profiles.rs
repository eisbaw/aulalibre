//! Profile and master data service.
//!
//! Maps to `AulaNative.Services.Web.ProfileServiceManager` and related
//! profile-fetching operations from the APK.
//!
//! # Endpoint paths
//!
//! All endpoints use RPC-style routing via `?method=module.action` query
//! parameters. Paths are sourced from the decompiled `Urls.cs` class.
//!
//! | Method | HTTP | RPC method |
//! |--------|------|------------|
//! | `get_profiles_by_login` | GET | `profiles.getprofilesbylogin` |
//! | `get_profile_context` | GET | `profiles.getProfileContext` |
//! | `get_profile_master_data` | GET | `profiles.getProfileMasterData` |
//! | `get_onboarding_master_data` | GET | `profiles.getProfilesByLogin` |
//! | `post_master_data` | POST | `profiles.updateProfileMasterData` |
//! | `update_profile_picture` | POST | `profiles.updateProfilePicture` |
//! | `get_contact_list` | GET | `profiles.getContactList` |
//! | `get_contact_parents` | GET | `profiles.getContactParents` |
//! | `mark_onboarding_completed` | POST | `profiles.markOnboardingCompleted` |
//! | `get_all_profiles` | GET | `profiles.getAllProfiles` |
//! | `get_profile_types` | GET | `profiles.getProfileTypesByLogin` |

use serde::{Deserialize, Serialize};

use crate::models::onboarding::OnboardingResponseDto;
use crate::models::profiles::{InstitutionProfileId, Profile};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response / request types specific to this service
// ---------------------------------------------------------------------------

/// Response from `GetProfilesByLogin`.
///
/// The API returns the logged-in user's profiles (typically one per
/// institution) wrapped in the standard `AulaServiceResponse` envelope.
/// The `data` field is a `Vec<Profile>`.
pub type ProfilesByLoginResponse = Vec<Profile>;

/// Request body for `PostMasterData`.
///
/// Inferred from `ProfileServiceManager.PostMasterData()` parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMasterDataRequest {
    /// External (personal) email address.
    pub external_email: Option<String>,
    /// Phone number.
    pub phonenumber: Option<String>,
    /// Work phone number.
    pub work_phonenumber: Option<String>,
    /// Home phone number.
    pub home_phonenumber: Option<String>,
    /// Mobile phone number.
    pub mobile_phonenumber: Option<String>,
}

/// Request body for `PostUpdateProfilePicture`.
///
/// Sends the S3 key/bucket of the already-uploaded image to associate
/// it with the user's profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfilePictureRequest {
    /// Institution profile to update the picture for.
    pub institution_profile_id: InstitutionProfileId,
    /// S3 key of the uploaded image.
    pub key: String,
    /// S3 bucket name.
    pub bucket: String,
}

/// Simplified master data response.
///
/// The shape mirrors what `GetProfileMasterData` returns inside the
/// `AulaServiceResponse.data` field.
pub type ProfileMasterDataResponse = Profile;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch the logged-in user's profiles after authentication.
///
/// Maps to `ProfileServiceManager.GetProfilesByLogin()`.
///
/// # Endpoint
///
/// `GET ?method=profiles.getprofilesbylogin`
pub async fn get_profiles_by_login(
    session: &mut Session,
) -> crate::Result<ProfilesByLoginResponse> {
    session.get("?method=profiles.getprofilesbylogin").await
}

/// Fetch profile master data for the current user.
///
/// Maps to `ProfileServiceManager.GetProfileMasterData()`.
///
/// # Endpoint
///
/// `GET ?method=profiles.getProfileMasterData`
pub async fn get_profile_master_data(
    session: &mut Session,
) -> crate::Result<ProfileMasterDataResponse> {
    session.get("?method=profiles.getProfileMasterData").await
}

/// Fetch onboarding master data (first-login flow).
///
/// Maps to `ProfileServiceManager.GetOnboardingMasterData()`.
///
/// # Endpoint
///
/// `GET ?method=profiles.getProfilesByLogin`
pub async fn get_onboarding_master_data(
    session: &mut Session,
) -> crate::Result<OnboardingResponseDto> {
    session.get("?method=profiles.getProfilesByLogin").await
}

/// Update profile master data (contact info).
///
/// Maps to `ProfileServiceManager.PostMasterData()`.
///
/// # Endpoint
///
/// `POST ?method=profiles.updateProfileMasterData`
pub async fn post_master_data(
    session: &mut Session,
    request: &UpdateMasterDataRequest,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=profiles.updateProfileMasterData", request)
        .await
}

/// Update the profile picture for an institution profile.
///
/// Maps to `ProfileServiceManager.PostUpdateProfilePicture()`.
///
/// # Endpoint
///
/// `POST ?method=profiles.updateProfilePicture`
pub async fn update_profile_picture(
    session: &mut Session,
    request: &UpdateProfilePictureRequest,
) -> crate::Result<serde_json::Value> {
    session
        .post("?method=profiles.updateProfilePicture", request)
        .await
}

/// Send a keep-alive ping to extend the backend session.
///
/// Maps to `SessionPromptManager`'s periodic keep-alive call.
///
/// # Endpoint
///
/// `POST ?method=session.keepAlive`
pub async fn keep_alive(session: &mut Session) -> crate::Result<()> {
    let _: serde_json::Value = session.post_empty("?method=session.keepAlive").await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_master_data_request_serializes() {
        let req = UpdateMasterDataRequest {
            external_email: Some("user@example.com".to_string()),
            phonenumber: Some("12345678".to_string()),
            work_phonenumber: None,
            home_phonenumber: None,
            mobile_phonenumber: Some("87654321".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["externalEmail"], "user@example.com");
        assert_eq!(json["phonenumber"], "12345678");
        assert_eq!(json["mobilePhonenumber"], "87654321");
        assert!(json["workPhonenumber"].is_null());
    }

    #[test]
    fn update_profile_picture_request_serializes() {
        let req = UpdateProfilePictureRequest {
            institution_profile_id: 42,
            key: "photos/42/pic.jpg".to_string(),
            bucket: "aula-prod".to_string(),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["institutionProfileId"], 42);
        assert_eq!(json["key"], "photos/42/pic.jpg");
        assert_eq!(json["bucket"], "aula-prod");
    }

    #[test]
    fn update_master_data_request_deserializes() {
        let json = r#"{
            "externalEmail": "a@b.com",
            "phonenumber": null,
            "workPhonenumber": null,
            "homePhonenumber": null,
            "mobilePhonenumber": null
        }"#;
        let req: UpdateMasterDataRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.external_email.as_deref(), Some("a@b.com"));
        assert!(req.phonenumber.is_none());
    }
}
