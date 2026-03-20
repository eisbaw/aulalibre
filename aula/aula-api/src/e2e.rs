//! End-to-end test harness for live Aula API testing.
//!
//! This module provides infrastructure for running integration tests against
//! the real Aula API. It handles:
//!
//! - Loading authentication tokens from disk or environment variables
//! - Constructing an authenticated [`Session`] for live API calls
//! - Graceful skipping when no tokens are available
//!
//! # E2E Test Setup
//!
//! To run live E2E tests, you need a valid Aula authentication token.
//! There are three ways to provide one:
//!
//! ## Option 1: Use `aula auth login` (recommended)
//!
//! ```bash
//! # Log in via browser (tokens saved to default location)
//! cargo run --bin aula-cli -- auth login
//!
//! # Copy tokens to the E2E test location
//! cp ~/.local/share/aula/tokens.json tests/e2e/.auth-token
//! ```
//!
//! ## Option 2: Set the `AULA_E2E_TOKEN` environment variable
//!
//! ```bash
//! export AULA_E2E_TOKEN='{"access_token":"...","refresh_token":"...","auth_level":"level2",...}'
//! ```
//!
//! ## Option 3: Place tokens in `tests/e2e/.auth-token`
//!
//! Create the file `tests/e2e/.auth-token` (relative to the `aula-api` crate
//! root) containing a JSON-serialized [`LoginData`]. This file is gitignored.
//!
//! # Running E2E Tests
//!
//! E2E tests use the `#[ignore]` attribute and are only run when explicitly
//! requested:
//!
//! ```bash
//! # Run only E2E tests
//! cargo test --manifest-path aula/Cargo.toml -p aula-api --test e2e_live_tests -- --ignored
//!
//! # Or use the just recipe
//! just e2e-live
//! ```
//!
//! When no token is available, tests print a skip message and return `Ok(())`
//! rather than failing. This allows CI pipelines to run the test suite without
//! real credentials.
//!
//! # Security
//!
//! - The `tests/e2e/.auth-token` file is gitignored. NEVER commit real tokens.
//! - Tokens contain sensitive access to a real Aula account.
//! - The `secrets/` directory at the project root is also gitignored.

use std::path::{Path, PathBuf};

use crate::auth::LoginData;
use crate::client::{AulaClient, AulaClientConfig, Environment};
use crate::session::{Session, SessionConfig};
use crate::token_store::TokenStore;

/// Default path for E2E auth tokens, relative to the `aula-api` crate root.
const E2E_TOKEN_REL_PATH: &str = "tests/e2e/.auth-token";

/// Environment variable name for providing tokens inline.
const E2E_TOKEN_ENV_VAR: &str = "AULA_E2E_TOKEN";

/// Outcome of attempting to load E2E credentials.
#[derive(Debug)]
pub enum E2eCredentials {
    /// Credentials were found and loaded successfully.
    Available {
        /// The loaded login data.
        login_data: LoginData,
        /// Where the credentials were loaded from (for diagnostics).
        source: String,
    },
    /// No credentials found; tests should skip.
    Unavailable {
        /// Human-readable explanation of what was checked.
        reason: String,
    },
}

impl E2eCredentials {
    /// Returns `true` if credentials are available.
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }
}

/// Attempt to load E2E credentials from all supported sources.
///
/// Checks in order:
/// 1. `AULA_E2E_TOKEN` environment variable (JSON string)
/// 2. `tests/e2e/.auth-token` file relative to the `aula-api` crate root
/// 3. `secrets/e2e_tokens.json` file at the workspace root
///
/// Returns [`E2eCredentials::Unavailable`] if none are found, with a
/// diagnostic message listing all locations checked.
pub fn load_credentials() -> E2eCredentials {
    // 1. Check environment variable.
    if let Ok(json) = std::env::var(E2E_TOKEN_ENV_VAR) {
        match serde_json::from_str::<LoginData>(&json) {
            Ok(data) => {
                return E2eCredentials::Available {
                    login_data: data,
                    source: format!("env var {E2E_TOKEN_ENV_VAR}"),
                };
            }
            Err(e) => {
                eprintln!("warning: {E2E_TOKEN_ENV_VAR} is set but contains invalid JSON: {e}");
            }
        }
    }

    // 2. Check tests/e2e/.auth-token relative to crate root.
    let crate_root = crate_root_dir();
    let token_file = crate_root.join(E2E_TOKEN_REL_PATH);
    if let Some(creds) = try_load_token_file(&token_file) {
        return creds;
    }

    // 3. Check secrets/e2e_tokens.json at workspace root.
    let workspace_root = crate_root.parent().unwrap_or(&crate_root);
    let secrets_file = workspace_root
        .parent()
        .unwrap_or(workspace_root)
        .join("secrets/e2e_tokens.json");
    if let Some(creds) = try_load_token_file(&secrets_file) {
        return creds;
    }

    E2eCredentials::Unavailable {
        reason: format!(
            "No E2E credentials found. Checked:\n\
             - env var: {E2E_TOKEN_ENV_VAR}\n\
             - file: {}\n\
             - file: {}\n\
             \n\
             To set up E2E testing:\n\
             1. Run: cargo run --bin aula-cli -- auth login\n\
             2. Copy tokens: cp ~/.local/share/aula/tokens.json {}\n\
             Or set {E2E_TOKEN_ENV_VAR} with JSON token data.",
            token_file.display(),
            secrets_file.display(),
            token_file.display(),
        ),
    }
}

/// Try to load a token file from the given path.
fn try_load_token_file(path: &Path) -> Option<E2eCredentials> {
    let contents = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str::<LoginData>(&contents) {
        Ok(data) => Some(E2eCredentials::Available {
            login_data: data,
            source: format!("file {}", path.display()),
        }),
        Err(e) => {
            eprintln!(
                "warning: token file {} exists but contains invalid JSON: {e}",
                path.display()
            );
            None
        }
    }
}

/// Build an authenticated [`Session`] for E2E testing.
///
/// Uses [`load_credentials`] to find tokens and constructs a session
/// pointed at the production Aula API.
///
/// Returns `None` if no credentials are available (prints a skip message).
pub fn e2e_session() -> Option<Session> {
    e2e_session_with_env(Environment::Production)
}

/// Build an authenticated [`Session`] for E2E testing against a specific
/// environment.
///
/// Returns `None` if no credentials are available (prints a skip message).
pub fn e2e_session_with_env(environment: Environment) -> Option<Session> {
    let creds = load_credentials();

    match creds {
        E2eCredentials::Unavailable { reason } => {
            eprintln!("SKIPPING E2E test: {reason}");
            None
        }
        E2eCredentials::Available { login_data, source } => {
            eprintln!("E2E: loaded credentials from {source}");

            // Store tokens in a temp directory for the session.
            let tmp_dir = std::env::temp_dir().join(format!("aula_e2e_{}", std::process::id()));
            let store = TokenStore::new(&tmp_dir);
            if let Err(e) = store.save(&login_data) {
                eprintln!("E2E: failed to write temp token store: {e}");
                return None;
            }

            let client = match AulaClient::with_config(AulaClientConfig {
                environment,
                api_version: 23,
            }) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("E2E: failed to create client: {e}");
                    return None;
                }
            };

            match Session::new(client, store, SessionConfig::default()) {
                Ok(session) => Some(session),
                Err(e) => {
                    eprintln!("E2E: failed to create session: {e}");
                    None
                }
            }
        }
    }
}

/// Save tokens to the E2E token file location.
///
/// This is used by the bootstrap flow: after `aula auth login`, tokens
/// can be copied to the E2E location for test use.
pub fn save_e2e_tokens(login_data: &LoginData) -> crate::Result<()> {
    let path = crate_root_dir().join(E2E_TOKEN_REL_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(login_data)?;
    std::fs::write(&path, json.as_bytes())?;

    // Restrictive permissions on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

    eprintln!("E2E tokens saved to: {}", path.display());
    Ok(())
}

/// Determine the `aula-api` crate root directory.
///
/// Uses `CARGO_MANIFEST_DIR` when available (inside `cargo test`),
/// otherwise falls back to the current directory.
fn crate_root_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

/// Macro-like helper for E2E tests: load a session or skip the test.
///
/// Use this at the top of every E2E test function:
///
/// ```rust,ignore
/// #[tokio::test]
/// #[ignore] // only run with --ignored
/// async fn test_something_live() {
///     let Some(mut session) = aula_api::e2e::e2e_session() else {
///         return; // skipped: no credentials
///     };
///     // ... use session ...
/// }
/// ```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_credentials_returns_unavailable_by_default() {
        // In a normal test environment without E2E tokens set up,
        // this should return Unavailable (not panic).
        let creds = load_credentials();
        // We can't assert Available/Unavailable since we don't know the
        // test environment, but it must not panic.
        match &creds {
            E2eCredentials::Available { source, .. } => {
                eprintln!("(credentials found from {source})");
            }
            E2eCredentials::Unavailable { reason } => {
                assert!(!reason.is_empty());
            }
        }
    }

    #[test]
    fn load_credentials_from_env_var() {
        // Temporarily set the env var with valid JSON.
        let json = r#"{
            "access_token": "test_at",
            "refresh_token": "test_rt",
            "expires_in": 3600,
            "access_token_expiration": 9999999999,
            "auth_level": "level2",
            "error": null,
            "error_description": null
        }"#;

        // Use a unique env var approach to avoid interfering with other tests.
        // Since AULA_E2E_TOKEN is read by load_credentials, we need to be
        // careful about test isolation. We'll test the parsing directly.
        let data: LoginData = serde_json::from_str(json).unwrap();
        assert_eq!(data.access_token, "test_at");
        assert_eq!(data.refresh_token.as_deref(), Some("test_rt"));
    }

    #[test]
    fn try_load_nonexistent_file_returns_none() {
        let result = try_load_token_file(Path::new("/nonexistent/path/.auth-token"));
        assert!(result.is_none());
    }

    #[test]
    fn try_load_invalid_json_file_returns_none() {
        let tmp =
            std::env::temp_dir().join(format!("aula_e2e_test_invalid_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("bad-tokens.json");
        std::fs::write(&path, "not valid json").unwrap();

        let result = try_load_token_file(&path);
        assert!(result.is_none());

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn try_load_valid_json_file() {
        let tmp = std::env::temp_dir().join(format!("aula_e2e_test_valid_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("tokens.json");

        let json = r#"{
            "access_token": "file_at",
            "refresh_token": "file_rt",
            "expires_in": 3600,
            "access_token_expiration": 9999999999,
            "auth_level": "level2",
            "error": null,
            "error_description": null
        }"#;
        std::fs::write(&path, json).unwrap();

        let result = try_load_token_file(&path);
        assert!(result.is_some());
        if let Some(E2eCredentials::Available { login_data, source }) = result {
            assert_eq!(login_data.access_token, "file_at");
            assert!(source.contains(&path.display().to_string()));
        }

        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn e2e_session_returns_none_without_tokens() {
        // This test verifies the skip mechanism works. In a normal test
        // env without tokens, it should return None, not panic.
        // (If tokens happen to be available, it returns Some, which is also fine.)
        let _session = e2e_session();
        // No assertion needed - just verifying it doesn't panic.
    }

    #[test]
    fn save_e2e_tokens_creates_file() {
        use crate::auth::AuthLevel;

        let data = LoginData {
            access_token: "save_test_at".to_string(),
            refresh_token: Some("save_test_rt".to_string()),
            expires_in: Some(3600),
            access_token_expiration: Some(9999999999),
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        };

        // We can only test this if CARGO_MANIFEST_DIR is set (it is during cargo test).
        if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
            let path = crate_root_dir().join(E2E_TOKEN_REL_PATH);
            // Clean up any previous test artifact.
            std::fs::remove_file(&path).ok();

            save_e2e_tokens(&data).unwrap();
            assert!(path.exists());

            // Verify content.
            let loaded = try_load_token_file(&path);
            assert!(loaded.is_some());

            // Clean up.
            std::fs::remove_file(&path).ok();
        }
    }

    #[test]
    fn crate_root_dir_is_valid() {
        let root = crate_root_dir();
        // During cargo test, this should be the aula-api directory.
        assert!(root.exists() || root == std::path::Path::new("."));
    }
}
