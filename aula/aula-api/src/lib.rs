//! Rust client library for the Aula school platform API.
//!
//! This crate provides typed access to the Aula RPC-style API (v23),
//! including authentication, messaging, calendar, and other endpoints
//! discovered through APK reverse engineering of the Aula Android app.

pub mod auth;
pub mod client;
pub mod e2e;
pub mod enums;
pub mod error;
pub mod models;
pub mod response;
pub mod services;
pub mod session;
pub mod token_store;

// Re-export key types at crate root for convenience.
pub use client::{AulaClient, AulaClientConfig, Environment};
pub use error::{AulaError, Result};
pub use response::{
    AulaErrorResponse, AulaServiceResponse, DataArrayResponse, WebResponseStatus,
    WebResponseStatusSubCode,
};
pub use session::{Session, SessionConfig};
pub use token_store::TokenStore;

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
