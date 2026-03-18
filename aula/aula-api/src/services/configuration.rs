//! Configuration service.
//!
//! Maps to `AulaNative.Services.Web.ConfigurationService` from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.2.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_max_file_size` | GET | `/configuration/maxFileSize` |
//! | `get_authorized_file_formats` | GET | `/configuration/authorizedFileFormats` |
//! | `is_app_deprecated` | GET | `/configuration/isAppDeprecated` |
//! | `get_privacy_policy` | GET | `/configuration/privacyPolicy` |
//! | `get_administrative_authority` | GET | `/configuration/administrativeAuthority` |
//! | `get_login_important_information` | GET | `/configuration/loginImportantInformation` |

use serde::{Deserialize, Serialize};

use crate::models::files::AuthorizedFileFormat;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Max file size response.
///
/// The API likely returns a simple numeric value wrapped in the standard
/// envelope. We use `i64` to handle whatever unit (bytes) the API uses.
pub type MaxFileSizeResponse = i64;

/// App deprecation status response.
///
/// Used by the app to decide whether to show a force-update dialog.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDeprecatedResponse {
    /// Whether the current app version is deprecated.
    #[serde(default)]
    pub is_deprecated: bool,
    /// Optional message to display to the user.
    pub message: Option<String>,
}

/// Privacy policy response.
///
/// The API likely returns HTML or structured text content.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyPolicyResponse {
    /// Privacy policy content (may be HTML).
    pub content: Option<String>,
    /// Version identifier for the policy.
    pub version: Option<String>,
}

/// Login important information response.
///
/// Shown on the login page as a banner or notice.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginImportantInformationResponse {
    /// The information text (may be HTML).
    pub content: Option<String>,
    /// Whether the banner should be shown.
    #[serde(default)]
    pub show: bool,
}

/// Administrative authority response.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrativeAuthorityResponse {
    /// Authority name.
    pub name: Option<String>,
    /// Contact URL or info.
    pub contact: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get the maximum allowed file upload size.
///
/// Maps to `ConfigurationService.GetMaxFileSize()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/maxFileSize`
pub async fn get_max_file_size(session: &mut Session) -> crate::Result<MaxFileSizeResponse> {
    session.get("configuration/maxFileSize").await
}

/// Get the list of authorized (allowed) file formats for upload.
///
/// Maps to `ConfigurationService.GetAuthorizedFileFormats()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/authorizedFileFormats`
pub async fn get_authorized_file_formats(
    session: &mut Session,
) -> crate::Result<Vec<AuthorizedFileFormat>> {
    session.get("configuration/authorizedFileFormats").await
}

/// Check whether the current app version is deprecated (force update).
///
/// Maps to `ConfigurationService.IsAppDeprecated()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/isAppDeprecated`
///
/// NOTE: The response shape is uncertain. It may return a plain boolean
/// or a structured object. We use `AppDeprecatedResponse` as a best
/// guess based on the method name and typical .NET API patterns.
pub async fn is_app_deprecated(session: &mut Session) -> crate::Result<AppDeprecatedResponse> {
    session.get("configuration/isAppDeprecated").await
}

/// Get the privacy policy content.
///
/// Maps to `ConfigurationService.GetPrivacyPolicy()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/privacyPolicy`
///
/// NOTE: The response shape is uncertain. It may return plain HTML
/// as a string or a structured DTO.
pub async fn get_privacy_policy(session: &mut Session) -> crate::Result<PrivacyPolicyResponse> {
    session.get("configuration/privacyPolicy").await
}

/// Get the administrative authority information.
///
/// Maps to `ConfigurationService.GetAdministrativeAuthority()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/administrativeAuthority`
pub async fn get_administrative_authority(
    session: &mut Session,
) -> crate::Result<AdministrativeAuthorityResponse> {
    session.get("configuration/administrativeAuthority").await
}

/// Get important information to display on the login page.
///
/// Maps to `ConfigurationService.GetLoginImportantInformation()`.
///
/// # Endpoint (inferred)
///
/// `GET /configuration/loginImportantInformation`
///
/// NOTE: The response shape is uncertain. It may return plain text
/// or a structured object.
pub async fn get_login_important_information(
    session: &mut Session,
) -> crate::Result<LoginImportantInformationResponse> {
    session.get("configuration/loginImportantInformation").await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_deprecated_response_deserializes() {
        let json = r#"{"isDeprecated": true, "message": "Please update"}"#;
        let r: AppDeprecatedResponse = serde_json::from_str(json).unwrap();
        assert!(r.is_deprecated);
        assert_eq!(r.message.as_deref(), Some("Please update"));
    }

    #[test]
    fn app_deprecated_response_defaults() {
        let json = r#"{}"#;
        let r: AppDeprecatedResponse = serde_json::from_str(json).unwrap();
        assert!(!r.is_deprecated);
        assert!(r.message.is_none());
    }

    #[test]
    fn privacy_policy_response_deserializes() {
        let json = r#"{"content": "<p>Policy text</p>", "version": "2.1"}"#;
        let r: PrivacyPolicyResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.content.as_deref(), Some("<p>Policy text</p>"));
        assert_eq!(r.version.as_deref(), Some("2.1"));
    }

    #[test]
    fn login_important_info_deserializes() {
        let json = r#"{"content": "Maintenance tonight", "show": true}"#;
        let r: LoginImportantInformationResponse = serde_json::from_str(json).unwrap();
        assert!(r.show);
        assert_eq!(r.content.as_deref(), Some("Maintenance tonight"));
    }

    #[test]
    fn login_important_info_defaults() {
        let json = r#"{}"#;
        let r: LoginImportantInformationResponse = serde_json::from_str(json).unwrap();
        assert!(!r.show);
        assert!(r.content.is_none());
    }

    #[test]
    fn administrative_authority_deserializes() {
        let json = r#"{"name": "Kommune X", "contact": "https://example.dk"}"#;
        let r: AdministrativeAuthorityResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.name.as_deref(), Some("Kommune X"));
    }
}
