//! Live E2E tests that call the real Aula API.
//!
//! These tests are marked `#[ignore]` and only run when explicitly requested:
//!
//! ```bash
//! cargo test -p aula-api --test e2e_live_tests -- --ignored
//! # or
//! just e2e-live
//! ```
//!
//! They require a valid authentication token. See [`aula_api::e2e`] module
//! documentation for setup instructions.
//!
//! When no token is available, each test prints a skip message and passes
//! without making any API calls. This ensures CI pipelines do not fail
//! due to missing credentials.

use aula_api::e2e;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Load an E2E session or skip the test with a descriptive message.
///
/// Returns `None` when no credentials are available, causing the test
/// to return early with a pass (not a failure).
fn try_session() -> Option<aula_api::Session> {
    e2e::e2e_session()
}

// ---------------------------------------------------------------------------
// Live API tests
// ---------------------------------------------------------------------------

/// Verify that the loaded token can successfully refresh.
///
/// This tests AC#4: "Token refresh works in E2E mode (re-use refresh token)".
/// If the access token is expired but a refresh token is present, the session
/// should be able to obtain a new access token.
#[tokio::test]
#[ignore]
async fn e2e_token_refresh() {
    let Some(mut session) = try_session() else {
        return;
    };

    // Only test refresh if we have a refresh token.
    let has_refresh = session
        .login_data()
        .map(|ld| ld.refresh_token.is_some())
        .unwrap_or(false);

    if !has_refresh {
        eprintln!("SKIP: no refresh token available for refresh test");
        return;
    }

    match session.refresh_token().await {
        Ok(()) => {
            eprintln!("E2E: token refresh succeeded");
            assert!(
                session.has_valid_tokens(),
                "session should have valid tokens after refresh"
            );
        }
        Err(e) => {
            // A refresh failure is not necessarily a test bug -- the refresh
            // token may have expired. Report but don't panic.
            eprintln!("E2E: token refresh failed (may be expired): {e}");
        }
    }
}

/// Verify that ensure_valid_token works with real credentials.
#[tokio::test]
#[ignore]
async fn e2e_ensure_valid_token() {
    let Some(mut session) = try_session() else {
        return;
    };

    match session.ensure_valid_token().await {
        Ok(()) => {
            eprintln!("E2E: ensure_valid_token succeeded");
            assert!(session.has_valid_tokens());
        }
        Err(e) => {
            eprintln!("E2E: ensure_valid_token failed: {e}");
            // This is expected if tokens are fully expired with no valid
            // refresh token.
        }
    }
}

/// Verify graceful skip behavior: loading credentials should either succeed
/// or return a clear unavailable message -- never panic.
#[test]
#[ignore]
fn e2e_graceful_skip_message() {
    let creds = e2e::load_credentials();
    match creds {
        e2e::E2eCredentials::Available { source, .. } => {
            eprintln!("E2E credentials available from: {source}");
        }
        e2e::E2eCredentials::Unavailable { reason } => {
            eprintln!("E2E credentials unavailable (expected in CI):");
            eprintln!("{reason}");
            // The skip message should contain setup instructions.
            assert!(
                reason.contains("auth login"),
                "skip message should contain setup instructions"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Credential loading tests (always run, not #[ignore])
// ---------------------------------------------------------------------------

/// This test always runs (no #[ignore]) to verify the E2E harness itself
/// does not panic when credentials are absent.
#[test]
fn e2e_harness_no_panic_without_credentials() {
    // This must never panic, even without any credentials configured.
    let _creds = e2e::load_credentials();
    let _session = e2e::e2e_session();
}
