//! Configuration service.
//!
//! Maps to `AulaNative.Services.Web.ConfigurationService` from the APK.
//!
//! # Endpoint paths
//!
//! All endpoints use RPC-style routing via `?method=module.action`.
//! Paths are sourced from the decompiled `Urls.cs` class.
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_MAX_FILE_SIZE` | `centralConfiguration.getMaxFileSize` |
//! | `GET_AUTHORIZED_FILE_FORMATS` | `centralConfiguration.getauthorizedfileformats` |
//! | `IS_APP_DEPRECATED` | `centralConfiguration.isAppVersionDeprecated` |
//! | `GET_DATA_POLICY` | `centralConfiguration.getDataPolicy` |
//! | `LOGIN_GET_IMPORTANT_INFO` | `centralConfiguration.getLoginImportantInformation` |
//! | `GET_ADMIN_AUTHORITIES` | `municipalConfiguration.getSameAdministrativeAuthorityInstitutions` |

use serde::{Deserialize, Serialize};

use crate::models::files::AuthorizedFileFormat;
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Max file size response.
pub type MaxFileSizeResponse = i64;

/// App deprecation status response.
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
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyPolicyResponse {
    /// Privacy policy content (may be HTML).
    pub content: Option<String>,
    /// Version identifier for the policy.
    pub version: Option<String>,
}

/// Login important information response.
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
/// # Endpoint
///
/// `GET ?method=centralConfiguration.getMaxFileSize`
pub async fn get_max_file_size(session: &mut Session) -> crate::Result<MaxFileSizeResponse> {
    session
        .get("?method=centralConfiguration.getMaxFileSize")
        .await
}

/// Get the list of authorized (allowed) file formats for upload.
///
/// # Endpoint
///
/// `GET ?method=centralConfiguration.getauthorizedfileformats`
pub async fn get_authorized_file_formats(
    session: &mut Session,
) -> crate::Result<Vec<AuthorizedFileFormat>> {
    session
        .get("?method=centralConfiguration.getauthorizedfileformats")
        .await
}

/// Check whether the current app version is deprecated (force update).
///
/// # Endpoint
///
/// `GET ?method=centralConfiguration.isAppVersionDeprecated`
pub async fn is_app_deprecated(session: &mut Session) -> crate::Result<AppDeprecatedResponse> {
    session
        .get("?method=centralConfiguration.isAppVersionDeprecated")
        .await
}

/// Get the privacy/data policy content.
///
/// # Endpoint
///
/// `GET ?method=centralConfiguration.getDataPolicy`
pub async fn get_privacy_policy(session: &mut Session) -> crate::Result<PrivacyPolicyResponse> {
    session
        .get("?method=centralConfiguration.getDataPolicy")
        .await
}

/// Get the administrative authority information.
///
/// # Endpoint
///
/// `GET ?method=municipalConfiguration.getSameAdministrativeAuthorityInstitutions`
pub async fn get_administrative_authority(
    session: &mut Session,
) -> crate::Result<AdministrativeAuthorityResponse> {
    session
        .get("?method=municipalConfiguration.getSameAdministrativeAuthorityInstitutions")
        .await
}

/// Get important information to display on the login page.
///
/// # Endpoint
///
/// `GET ?method=centralConfiguration.getLoginImportantInformation`
pub async fn get_login_important_information(
    session: &mut Session,
) -> crate::Result<LoginImportantInformationResponse> {
    session
        .get("?method=centralConfiguration.getLoginImportantInformation")
        .await
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
