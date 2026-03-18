//! Error types for the Aula API client.
//!
//! Maps the 13 error handler types discovered in the APK to a Rust error enum,
//! plus wraps standard HTTP/serialization/network errors.

use crate::response::WebResponseStatus;

/// Convenience result type for Aula API operations.
pub type Result<T> = std::result::Result<T, AulaError>;

/// Top-level error enum for the Aula API client.
///
/// Variant grouping follows the error handler hierarchy found in the
/// decompiled `AulaNative.dll` (see `api_endpoints.md` Section 6).
#[derive(Debug, thiserror::Error)]
pub enum AulaError {
    // -- HTTP / transport errors --
    /// HTTP request failed (wraps reqwest errors).
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// No network connectivity (`NoNetworkErrorHandler`).
    #[error("no network connectivity")]
    NoNetwork,

    /// Request was aborted (`AbortRequestErrorHandler`).
    #[error("request aborted")]
    RequestAborted,

    // -- Auth errors --
    /// Invalid or expired access token (`InvalidAccessTokenError`).
    #[error("invalid or expired access token")]
    InvalidAccessToken,

    /// Session has expired (`SessionExpiredErrorHandler`).
    #[error("session expired")]
    SessionExpired,

    /// Step-up authentication required (`StepUpNeededErrorHandler`).
    #[error("step-up authentication required")]
    StepUpRequired,

    /// 401 Unauthorized (`UnauthorizedErrorHandler`).
    #[error("unauthorized")]
    Unauthorized,

    // -- API-level errors --
    /// Aula is under maintenance (`AulaMaintenanceErrorHandler`).
    #[error("Aula is under maintenance")]
    Maintenance,

    /// Aula is not responding (`AulaNotRespondingErrorHandler`).
    #[error("Aula is not responding")]
    NotResponding,

    /// Heavy load / rate limiting (`HeavyLoadingErrorHandler`).
    #[error("Aula is under heavy load")]
    HeavyLoad,

    /// User account has been deactivated (`UserDeactivatedErrorHandler`).
    #[error("user account deactivated")]
    UserDeactivated,

    /// Generic API error with optional status detail.
    #[error("API error: {message}")]
    Api {
        message: String,
        status: Option<WebResponseStatus>,
    },

    // -- Auth flow errors --
    /// OIDC authentication flow error (token exchange, redirect parsing, etc.).
    #[error("auth error: {error}{}", description.as_ref().map(|d| format!(": {d}")).unwrap_or_default())]
    Auth {
        /// OAuth error code or short identifier.
        error: String,
        /// Optional human-readable description.
        description: Option<String>,
    },

    // -- IO --
    /// File system I/O error (e.g. token storage).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // -- Serialization --
    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_implements_display() {
        let err = AulaError::SessionExpired;
        assert_eq!(err.to_string(), "session expired");
    }

    #[test]
    fn error_implements_error_trait() {
        let err: &dyn std::error::Error = &AulaError::Maintenance;
        // source() should be None for simple variants
        assert!(err.source().is_none());
    }

    #[test]
    fn api_error_with_message() {
        let err = AulaError::Api {
            message: "something went wrong".into(),
            status: None,
        };
        assert_eq!(err.to_string(), "API error: something went wrong");
    }

    #[test]
    fn json_error_conversion() {
        let json_err = serde_json::from_str::<String>("not json").unwrap_err();
        let err: AulaError = json_err.into();
        assert!(err.to_string().starts_with("JSON error:"));
    }

    #[test]
    fn result_alias_works() {
        let ok: Result<u32> = Ok(42);
        assert_eq!(ok.unwrap(), 42);

        let err: Result<u32> = Err(AulaError::NoNetwork);
        assert!(err.is_err());
    }
}
