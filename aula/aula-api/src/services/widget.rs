//! Widget service.
//!
//! Maps to `AulaNative.Services.Web.WidgetWebService` from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_AULA_TOKEN` | `aulaToken.getAulaToken` |

use serde::{Deserialize, Serialize};

use crate::session::Session;

/// Widget SSO token response.
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
/// # Endpoint
///
/// `GET ?method=aulaToken.getAulaToken&WidgetId={widgetId}`
pub async fn get_aula_token(
    session: &mut Session,
    widget_id: &str,
) -> crate::Result<serde_json::Value> {
    session
        .get(&format!(
            "?method=aulaToken.getAulaToken&WidgetId={widget_id}"
        ))
        .await
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
