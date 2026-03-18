//! Onboarding service.
//!
//! Maps to `AulaNative.Services.Web.OnboardingWebService` (2 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `SET_ONBOARDING_COMPLETED` | `profiles.markOnboardingCompleted` |
//! | `GET_POLICY_LINKS` | `CommonFiles.getPersonalDataPolicies` |

use crate::session::Session;

use serde::{Deserialize, Serialize};

/// Policy link returned during onboarding.
///
/// Inferred from `GetPolicyLinks` return type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyLink {
    pub title: Option<String>,
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Mark the onboarding flow as complete for the current profile.
///
/// Maps to `OnboardingWebService.MarkOnboardingComplete()`.
///
/// # Endpoint
///
/// `POST ?method=profiles.markOnboardingCompleted`
pub async fn mark_onboarding_complete(session: &mut Session) -> crate::Result<serde_json::Value> {
    session
        .post_empty("?method=profiles.markOnboardingCompleted")
        .await
}

/// Get policy links shown during onboarding (data policy, terms, etc.).
///
/// Maps to `OnboardingWebService.GetPolicyLinks()`.
///
/// # Endpoint
///
/// `GET ?method=CommonFiles.getPersonalDataPolicies`
pub async fn get_policy_links(session: &mut Session) -> crate::Result<Vec<PolicyLink>> {
    session
        .get("?method=CommonFiles.getPersonalDataPolicies")
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_link_deserializes() {
        let json = r#"[
            {
                "title": "Privatlivspolitik",
                "url": "https://aula.dk/privacy"
            },
            {
                "title": "Vilkår",
                "url": "https://aula.dk/terms"
            }
        ]"#;
        let links: Vec<PolicyLink> = serde_json::from_str(json).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].title.as_deref(), Some("Privatlivspolitik"));
        assert_eq!(links[1].url.as_deref(), Some("https://aula.dk/terms"));
    }
}
