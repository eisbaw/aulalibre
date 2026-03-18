//! Widget service.
//!
//! Maps to `AulaNative.Services.Web.WidgetWebService` (1 method) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.21.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_aula_token` | GET | `/widget/token` |

use serde::{Deserialize, Serialize};

use crate::session::Session;

/// Widget SSO token response.
///
/// Inferred from `GetAulaToken` return type; the token is used for
/// authenticating with third-party widgets embedded in Aula.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetTokenResponse {
    pub token: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Get a widget SSO token for authenticating with embedded widgets.
///
/// Maps to `WidgetWebService.GetAulaToken()`.
///
/// # Endpoint (inferred)
///
/// `GET /widget/token`
pub async fn get_aula_token(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.get("widget/token").await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_token_response_deserializes() {
        let json = r#"{"token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."}"#;
        let r: WidgetTokenResponse = serde_json::from_str(json).unwrap();
        assert!(r.token.as_deref().unwrap().starts_with("eyJ"));
    }
}
