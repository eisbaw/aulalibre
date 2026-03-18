//! Rust client library for the Aula school platform API.
//!
//! This crate provides typed access to the Aula REST API,
//! including authentication, messaging, calendar, and other endpoints
//! discovered through APK reverse engineering.

/// Placeholder — real client implementation will follow.
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
}
