//! Profile and master data service.
//!
//! Maps to `AulaNative.Services.Web.ProfileServiceManager` and related
//! profile-fetching operations from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.3.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_profiles_by_login` | GET | `/profiles?method=profiles.getProfilesByLogin` |
//! | `get_profile_master_data` | GET | `/masterdata/profile` |
//! | `get_onboarding_master_data` | GET | `/masterdata/onboarding` |
//! | `post_master_data` | POST | `/masterdata` |
//! | `update_profile_picture` | POST | `/masterdata/profilePicture` |
//! | `keep_alive` | POST | `/profiles/keepAlive` |

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
///
/// NOTE: The exact response shape has not been verified against live
/// traffic. If the API returns a single `Profile` instead of a `Vec`,
/// this type will need adjustment.
pub type ProfilesByLoginResponse = Vec<Profile>;

/// Request body for `PostMasterData`.
///
/// Inferred from `ProfileServiceManager.PostMasterData()` parameters.
/// The exact field set is uncertain; this covers the commonly-seen
/// editable fields.
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
/// `AulaServiceResponse.data` field. Since the exact response type was
/// not found as a distinct DTO in the decompiled assembly, we use the
/// top-level `Profile` which contains the relevant fields.
pub type ProfileMasterDataResponse = Profile;

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Fetch the logged-in user's profiles after authentication.
///
/// Maps to `ProfileServiceManager.GetProfilesByLogin()`.
/// Called immediately after login to load the user's institution profiles,
/// groups, and page configuration.
///
/// # Endpoint (inferred)
///
/// `GET /profiles?method=profiles.getProfilesByLogin`
///
/// NOTE: The query-parameter style (`?method=...`) is a guess based on
/// the Aula web app pattern. The native app may use a different path.
pub async fn get_profiles_by_login(
    session: &mut Session,
) -> crate::Result<ProfilesByLoginResponse> {
    session
        .get("profiles?method=profiles.getProfilesByLogin")
        .await
}

/// Fetch profile master data for the current user.
///
/// Maps to `ProfileServiceManager.GetProfileMasterData()`.
///
/// # Endpoint (inferred)
///
/// `GET /masterdata/profile`
pub async fn get_profile_master_data(
    session: &mut Session,
) -> crate::Result<ProfileMasterDataResponse> {
    session.get("masterdata/profile").await
}

/// Fetch onboarding master data (first-login flow).
///
/// Maps to `ProfileServiceManager.GetOnboardingMasterData()`.
/// Returns the onboarding response with data policy status, children,
/// and institution profiles that need confirmation.
///
/// # Endpoint (inferred)
///
/// `GET /masterdata/onboarding`
pub async fn get_onboarding_master_data(
    session: &mut Session,
) -> crate::Result<OnboardingResponseDto> {
    session.get("masterdata/onboarding").await
}

/// Update profile master data (contact info).
///
/// Maps to `ProfileServiceManager.PostMasterData()`.
///
/// # Endpoint (inferred)
///
/// `POST /masterdata`
pub async fn post_master_data(
    session: &mut Session,
    request: &UpdateMasterDataRequest,
) -> crate::Result<serde_json::Value> {
    session.post("masterdata", request).await
}

/// Update the profile picture for an institution profile.
///
/// Maps to `ProfileServiceManager.PostUpdateProfilePicture()`.
/// The image must already be uploaded to S3; this call associates
/// the S3 object with the profile.
///
/// # Endpoint (inferred)
///
/// `POST /masterdata/profilePicture`
pub async fn update_profile_picture(
    session: &mut Session,
    request: &UpdateProfilePictureRequest,
) -> crate::Result<serde_json::Value> {
    session.post("masterdata/profilePicture", request).await
}

/// Send a keep-alive ping to extend the backend session.
///
/// Maps to `ProfileServiceManager.KeepAlive()` /
/// `SessionPromptManager`'s periodic keep-alive call.
///
/// # Endpoint
///
/// `POST /profiles/keepAlive`
///
/// This endpoint path is confirmed from multiple sources in the APK
/// (see `auth_flow.md` Section 9).
pub async fn keep_alive(session: &mut Session) -> crate::Result<()> {
    let _: serde_json::Value = session.post_empty("profiles/keepAlive").await?;
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
