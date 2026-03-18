//! Onboarding service.
//!
//! Maps to `AulaNative.Services.Web.OnboardingWebService` (2 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.18.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `mark_onboarding_complete` | POST | `/onboarding/complete` |
//! | `get_policy_links` | GET | `/onboarding/policyLinks` |

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
/// # Endpoint (inferred)
///
/// `POST /onboarding/complete`
pub async fn mark_onboarding_complete(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.post_empty("onboarding/complete").await
}

/// Get policy links shown during onboarding (data policy, terms, etc.).
///
/// Maps to `OnboardingWebService.GetPolicyLinks()`.
///
/// # Endpoint (inferred)
///
/// `GET /onboarding/policyLinks`
pub async fn get_policy_links(session: &mut Session) -> crate::Result<Vec<PolicyLink>> {
    session.get("onboarding/policyLinks").await
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
