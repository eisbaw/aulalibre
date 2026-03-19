//! Authentication subcommands: login, logout, status, refresh.
//!
//! The `login` command performs an interactive OIDC Authorization Code + PKCE
//! flow: it starts a local HTTP server, opens the user's browser at the
//! authorize URL, waits for the redirect callback carrying the authorization
//! code, exchanges it for tokens, and saves them to disk.
//!
//! By default, the flow uses a localhost callback server to automatically
//! capture the authorization code. If the OIDC provider rejects the localhost
//! redirect URI, use `--manual` to fall back to the copy-paste approach.

use std::time::Duration;

use clap::Subcommand;

use aula_api::auth::{self, AuthLevel, AuthorizeParams, LoginData, OidcEndpoints, PkceChallenge};
use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::session::{Session, SessionConfig};

use crate::callback_server;
use crate::output::bold;
use crate::session_util::{resolve_environment, token_store};

/// Authenticate with the Aula platform.
#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    /// Log in via browser-based OIDC flow (UniLogin or MitID).
    Login {
        /// Authentication level: 2 for UniLogin (default), 3 for MitID.
        #[arg(long, default_value = "2")]
        level: u8,

        /// Timeout in seconds waiting for the browser callback.
        #[arg(long, default_value = "120")]
        timeout: u64,

        /// Use manual copy-paste flow instead of the localhost callback server.
        ///
        /// In manual mode, the browser redirects to a URL that won't load,
        /// and you paste the full URL from the address bar into the terminal.
        #[arg(long)]
        manual: bool,
    },
    /// Log out and clear the current session.
    Logout,
    /// Show current authentication status.
    Status,
    /// Force a token refresh using the stored refresh token.
    Refresh,
}

fn parse_auth_level(level: u8) -> Result<AuthLevel, String> {
    match level {
        2 => Ok(AuthLevel::Level2),
        3 => Ok(AuthLevel::Level3),
        _ => Err(format!("invalid auth level {level}: must be 2 or 3")),
    }
}

pub async fn handle(cmd: &AuthCommand, env_override: Option<&str>) {
    match cmd {
        AuthCommand::Login {
            level,
            timeout,
            manual,
        } => {
            if *manual {
                handle_login_manual(*level, *timeout, env_override).await;
            } else {
                handle_login_auto(*level, *timeout, env_override).await;
            }
        }
        AuthCommand::Logout => handle_logout(env_override).await,
        AuthCommand::Status => handle_status(),
        AuthCommand::Refresh => handle_refresh(env_override).await,
    }
}

// ---------------------------------------------------------------------------
// Login (automatic callback server)
// ---------------------------------------------------------------------------

async fn handle_login_auto(level: u8, timeout_secs: u64, env_override: Option<&str>) {
    let auth_level = match parse_auth_level(level) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let environment = resolve_environment(env_override);
    let endpoints = OidcEndpoints::for_environment(&environment);
    let pkce = PkceChallenge::generate();
    let state = auth::generate_state();

    // Start the local callback server on an OS-assigned port.
    let timeout = Duration::from_secs(timeout_secs);
    let (port, server_handle) = match callback_server::start_callback_server(timeout).await {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!("Falling back to manual mode. Re-run with --manual flag.");
            std::process::exit(1);
        }
    };

    let redirect_uri = callback_server::localhost_redirect_uri(port);

    let params = AuthorizeParams {
        auth_level,
        code_challenge: pkce.code_challenge.clone(),
        state: state.clone(),
        redirect_uri: Some(redirect_uri.clone()),
    };

    let authorize_url = auth::build_authorize_url(&endpoints, &params);

    eprintln!("Starting Aula login ({auth_level})...");
    eprintln!("Callback server listening on http://localhost:{port}/callback");
    eprintln!();
    eprintln!("Opening your browser for authentication.");
    eprintln!("If the browser does not open, visit this URL manually:");
    eprintln!();
    eprintln!("  {authorize_url}");
    eprintln!();
    eprintln!("Waiting up to {timeout_secs} seconds for login to complete...");

    if let Err(e) = open::that(authorize_url.as_str()) {
        eprintln!("warning: failed to open browser: {e}");
        eprintln!("Please open the URL above manually.");
    }

    // Wait for the callback.
    let callback = match server_handle.await {
        Ok(Ok(result)) => result,
        Ok(Err(callback_server::CallbackError::Timeout)) => {
            eprintln!();
            eprintln!("error: timed out waiting for login callback ({timeout_secs}s)");
            eprintln!("Try again with a longer --timeout, or use --manual mode.");
            std::process::exit(1);
        }
        Ok(Err(e)) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: callback server task failed: {e}");
            std::process::exit(1);
        }
    };

    // Extract the authorization code from the callback URI.
    let (code, callback_state) =
        match callback_server::extract_code_from_callback(&callback.request_uri) {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        };

    // Verify state if present.
    if let Some(ref s) = callback_state {
        if s != &state {
            eprintln!("error: OIDC state mismatch (possible CSRF attack)");
            std::process::exit(1);
        }
    }

    complete_token_exchange(
        auth_level,
        &endpoints,
        &code,
        &pkce.code_verifier,
        Some(&redirect_uri),
    )
    .await;
}

// ---------------------------------------------------------------------------
// Login (manual copy-paste)
// ---------------------------------------------------------------------------

async fn handle_login_manual(level: u8, _timeout_secs: u64, env_override: Option<&str>) {
    let auth_level = match parse_auth_level(level) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let environment = resolve_environment(env_override);
    let endpoints = OidcEndpoints::for_environment(&environment);
    let pkce = PkceChallenge::generate();
    let state = auth::generate_state();

    // Use the app's registered redirect URI. The OIDC provider only accepts
    // this specific URI; localhost callbacks are rejected.
    let params = AuthorizeParams {
        auth_level,
        code_challenge: pkce.code_challenge.clone(),
        state: state.clone(),
        redirect_uri: None, // uses default: https://app-private.aula.dk
    };

    let authorize_url = auth::build_authorize_url(&endpoints, &params);

    eprintln!("Starting Aula login ({auth_level}) in manual mode...");
    eprintln!();
    eprintln!("Opening your browser for authentication.");
    eprintln!("If the browser does not open, visit this URL manually:");
    eprintln!();
    eprintln!("  {authorize_url}");
    eprintln!();

    if let Err(e) = open::that(authorize_url.as_str()) {
        eprintln!("warning: failed to open browser: {e}");
        eprintln!("Please open the URL above manually.");
    }

    eprintln!("After authenticating, your browser will redirect to a page that");
    eprintln!("won't load (https://app-private.aula.dk?code=...). This is expected.");
    eprintln!();
    eprintln!("Copy the FULL URL from your browser's address bar and paste it here:");
    eprintln!();

    let callback_url = match read_callback_url() {
        Ok(url) => url,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    let code = match auth::extract_code_from_redirect(&callback_url, Some(&state)) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: failed to extract authorization code: {e}");
            std::process::exit(1);
        }
    };

    complete_token_exchange(auth_level, &endpoints, &code, &pkce.code_verifier, None).await;
}

// ---------------------------------------------------------------------------
// Shared: token exchange + save
// ---------------------------------------------------------------------------

async fn complete_token_exchange(
    auth_level: AuthLevel,
    endpoints: &OidcEndpoints,
    code: &str,
    code_verifier: &str,
    redirect_uri: Option<&str>,
) {
    eprintln!("Authorization code received, exchanging for tokens...");

    let http = reqwest::Client::new();
    let token_response = match auth::exchange_code(
        &http,
        endpoints,
        auth_level,
        code,
        code_verifier,
        redirect_uri,
    )
    .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("error: token exchange failed: {e}");
            std::process::exit(1);
        }
    };

    let login_data = LoginData::from_token_response(token_response, auth_level);

    let store = token_store();
    if let Err(e) = store.save(&login_data) {
        eprintln!("error: failed to save tokens: {e}");
        std::process::exit(1);
    }

    eprintln!();
    eprintln!("{}", bold("Login successful!"));
    eprintln!("  Auth level: {auth_level}");
    if let Some(exp) = login_data.access_token_expiration {
        eprintln!("  Token expires: {}", format_unix_timestamp(exp));
    }
    eprintln!("  Tokens saved to: {}", store.dir().display());
}

/// Read a callback URL from stdin.
///
/// Handles the `app-redirect.aula.dk` intermediate redirect: if the user
/// pastes `https://app-redirect.aula.dk/?returnUri=<base64>`, we decode
/// the base64 `returnUri` to extract the real callback URL.
fn read_callback_url() -> Result<url::Url, String> {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .ok_or_else(|| "no input received".to_string())?
        .map_err(|e| format!("failed to read input: {e}"))?;

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("empty input".to_string());
    }

    let parsed = url::Url::parse(trimmed).map_err(|e| format!("invalid URL: {e}"))?;
    resolve_redirect_url(&parsed)
}

/// Resolve an `app-redirect.aula.dk` intermediate URL to the real callback.
///
/// The OIDC flow redirects through `app-redirect.aula.dk/?returnUri=<base64>`
/// where the base64 decodes to `https://app-private.aula.dk/?code=...&state=...`.
/// If the URL is not from app-redirect, it is returned unchanged.
fn resolve_redirect_url(url: &url::Url) -> Result<url::Url, String> {
    if url.host_str() != Some("app-redirect.aula.dk") {
        return Ok(url.clone());
    }

    let return_uri = url
        .query_pairs()
        .find(|(k, _)| k == "returnUri")
        .map(|(_, v)| v.to_string())
        .ok_or_else(|| "app-redirect URL missing returnUri parameter".to_string())?;

    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&return_uri)
        .map_err(|e| format!("failed to decode returnUri base64: {e}"))?;

    let url_str =
        String::from_utf8(decoded).map_err(|e| format!("returnUri is not valid UTF-8: {e}"))?;

    eprintln!("Decoded redirect URL from app-redirect.aula.dk");
    url::Url::parse(&url_str).map_err(|e| format!("decoded URL is invalid: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_redirect_decodes_base64_return_uri() {
        use base64::Engine;
        let inner = "https://app-private.aula.dk/?code=abc123&state=xyz";
        let encoded = base64::engine::general_purpose::STANDARD.encode(inner);
        let url_str = format!("https://app-redirect.aula.dk/?returnUri={encoded}");
        let url = url::Url::parse(&url_str).unwrap();

        let resolved = resolve_redirect_url(&url).unwrap();
        assert_eq!(resolved.host_str(), Some("app-private.aula.dk"));
        assert_eq!(
            resolved.query_pairs().find(|(k, _)| k == "code").unwrap().1,
            "abc123"
        );
        assert_eq!(
            resolved
                .query_pairs()
                .find(|(k, _)| k == "state")
                .unwrap()
                .1,
            "xyz"
        );
    }

    #[test]
    fn resolve_redirect_passes_through_non_redirect_url() {
        let url = url::Url::parse("https://app-private.aula.dk/?code=abc&state=xyz").unwrap();
        let resolved = resolve_redirect_url(&url).unwrap();
        assert_eq!(resolved, url);
    }

    #[test]
    fn resolve_redirect_errors_on_missing_return_uri() {
        let url = url::Url::parse("https://app-redirect.aula.dk/?other=value").unwrap();
        let err = resolve_redirect_url(&url).unwrap_err();
        assert!(err.contains("missing returnUri"));
    }
}

// ---------------------------------------------------------------------------
// Logout
// ---------------------------------------------------------------------------

async fn handle_logout(env_override: Option<&str>) {
    let environment = resolve_environment(env_override);
    let store = token_store();

    if !store.exists() {
        eprintln!("No active session found.");
        return;
    }

    let client = match AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 23,
    }) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to create client: {e}");
            std::process::exit(1);
        }
    };

    let mut session = match Session::new(client, store, SessionConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to create session: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = session.logout().await {
        eprintln!("warning: logout endpoint call failed: {e}");
        eprintln!("Local tokens have been cleared regardless.");
    } else {
        eprintln!("Logged out successfully. Tokens cleared.");
    }
}

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

fn handle_status() {
    let store = token_store();

    let login_data = match store.load() {
        Ok(Some(data)) => data,
        Ok(None) => {
            println!("Not logged in.");
            println!("Run 'aula auth login' to authenticate.");
            return;
        }
        Err(e) => {
            eprintln!("error: failed to read tokens: {e}");
            std::process::exit(1);
        }
    };

    println!("{}", bold("Logged in"));
    println!("  Auth level: {}", login_data.auth_level);
    println!(
        "  Has refresh token: {}",
        if login_data.refresh_token.is_some() {
            "yes"
        } else {
            "no"
        }
    );

    if let Some(exp) = login_data.access_token_expiration {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        println!("  Token expires: {}", format_unix_timestamp(exp));

        if exp > now {
            let remaining = exp - now;
            let mins = remaining / 60;
            let secs = remaining % 60;
            println!("  Time remaining: {mins}m {secs}s");
        } else {
            println!("  Status: {}", crate::output::red("EXPIRED"));
            if login_data.refresh_token.is_some() {
                println!("  Run 'aula auth refresh' to get a new token.");
            } else {
                println!("  Run 'aula auth login' to re-authenticate.");
            }
        }
    } else {
        println!("  Token expiry: unknown");
    }

    if let Some(ref err) = login_data.error {
        println!("  Error: {}", crate::output::red(err));
        if let Some(ref desc) = login_data.error_description {
            println!("  Error detail: {desc}");
        }
    }

    println!("  Token store: {}", store.dir().display());
}

// ---------------------------------------------------------------------------
// Refresh
// ---------------------------------------------------------------------------

async fn handle_refresh(env_override: Option<&str>) {
    let environment = resolve_environment(env_override);
    let store = token_store();

    if !store.exists() {
        eprintln!("No active session found. Run 'aula auth login' first.");
        std::process::exit(1);
    }

    let client = match AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 23,
    }) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to create client: {e}");
            std::process::exit(1);
        }
    };

    let mut session = match Session::new(client, store, SessionConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to create session: {e}");
            std::process::exit(1);
        }
    };

    match session.refresh_token().await {
        Ok(()) => {
            eprintln!("Token refreshed successfully.");
            if let Some(ld) = session.login_data() {
                if let Some(exp) = ld.access_token_expiration {
                    eprintln!("  New expiry: {}", format_unix_timestamp(exp));
                }
            }
        }
        Err(e) => {
            eprintln!("error: token refresh failed: {e}");
            eprintln!("You may need to run 'aula auth login' again.");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Format a Unix timestamp as a human-readable UTC string.
fn format_unix_timestamp(ts: u64) -> String {
    use std::time::{Duration, UNIX_EPOCH};

    let dt = UNIX_EPOCH + Duration::from_secs(ts);
    let elapsed = dt.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    let total_secs = elapsed.as_secs();

    let secs_per_day: u64 = 86400;
    let days = total_secs / secs_per_day;
    let day_secs = total_secs % secs_per_day;
    let hours = day_secs / 3600;
    let mins = (day_secs % 3600) / 60;
    let secs = day_secs % 60;

    let (year, month, day) = days_to_ymd(days);

    format!("{year:04}-{month:02}-{day:02} {hours:02}:{mins:02}:{secs:02} UTC")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
