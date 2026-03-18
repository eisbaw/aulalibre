//! Rust client library for the Aula school platform API.
//!
//! This crate provides typed access to the Aula REST API,
//! including authentication, messaging, calendar, and other endpoints
//! discovered through APK reverse engineering.

pub mod error;
pub mod response;

// Re-export key types at crate root for convenience.
pub use error::{AulaError, Result};
pub use response::{
    AulaErrorResponse, AulaServiceResponse, DataArrayResponse, WebResponseStatus,
    WebResponseStatusSubCode,
};

/// Returns the crate version string.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert_eq!(version(), "0.1.0");
    }

    #[test]
    fn re_exports_accessible() {
        // Verify the public API surface is reachable from crate root.
        let _: fn() -> Result<()> = || Err(AulaError::NoNetwork);
    }
}
