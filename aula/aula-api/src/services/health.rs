//! Health / IsAlive service.
//!
//! Maps to `AulaNative.Services.Web.SimpleService.CheckIfAulaIsAlive()` from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.1.
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `is_alive` | GET | `/isAlive` |

use serde::{Deserialize, Serialize};

use crate::session::Session;

/// Health check response.
///
/// The actual response body is unknown; it may be a simple string or boolean.
/// We use a permissive struct that accepts either.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IsAliveResponse {
    pub status: Option<String>,
}

// ---------------------------------------------------------------------------
// Service functions
// ---------------------------------------------------------------------------

/// Check if the Aula backend is alive.
///
/// Maps to `SimpleService.AulaIsAlive()`.
///
/// # Endpoint (inferred)
///
/// `GET alivecheck/`
///
/// Note: This is a path-based endpoint, not RPC-style. The URL in the APK
/// is `Conf.BackendUrl + "alivecheck/"` (not via the API base URL).
pub async fn is_alive(session: &mut Session) -> crate::Result<serde_json::Value> {
    session.get("alivecheck/").await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_alive_response_deserializes() {
        let json = r#"{"status": "ok"}"#;
        let r: IsAliveResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.status.as_deref(), Some("ok"));
    }
}
