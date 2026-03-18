//! Authentication subcommands: login, logout, status, refresh.
//!
//! The `login` command performs an interactive OIDC Authorization Code + PKCE
//! flow: it starts a local HTTP server, opens the user's browser at the
//! authorize URL, waits for the redirect callback carrying the authorization
//! code, exchanges it for tokens, and saves them to disk.

use clap::Subcommand;

use aula_api::auth::{self, AuthLevel, AuthorizeParams, LoginData, OidcEndpoints, PkceChallenge};
use aula_api::client::{AulaClient, AulaClientConfig, Environment};
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

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
    },
    /// Log out and clear the current session.
    Logout,
    /// Show current authentication status.
    Status,
    /// Force a token refresh using the stored refresh token.
    Refresh,
}

/// Resolve CLI environment string to an `Environment`.
fn resolve_environment(env: Option<&str>) -> Environment {
    match env {
        Some("preprod") => Environment::Preprod,
        Some("hotfix") => Environment::Hotfix,
        Some("test1") => Environment::Test1,
        Some("test3") => Environment::Test3,
        Some("dev1") => Environment::Dev1,
        Some("dev3") => Environment::Dev3,
        Some("dev11") => Environment::Dev11,
        _ => Environment::Production,
    }
}

fn parse_auth_level(level: u8) -> Result<AuthLevel, String> {
    match level {
        2 => Ok(AuthLevel::Level2),
        3 => Ok(AuthLevel::Level3),
        _ => Err(format!("invalid auth level {level}: must be 2 or 3")),
    }
}

fn token_store() -> TokenStore {
    TokenStore::default_location().unwrap_or_else(|| {
        eprintln!("warning: could not determine data directory, using ./aula-data");
        TokenStore::new("./aula-data")
    })
}

pub async fn handle(cmd: &AuthCommand, env_override: Option<&str>) {
    match cmd {
        AuthCommand::Login { level, timeout } => handle_login(*level, *timeout, env_override).await,
        AuthCommand::Logout => handle_logout(env_override).await,
        AuthCommand::Status => handle_status(),
        AuthCommand::Refresh => handle_refresh(env_override).await,
    }
}

// ---------------------------------------------------------------------------
// Login
// ---------------------------------------------------------------------------

async fn handle_login(level: u8, timeout_secs: u64, env_override: Option<&str>) {
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

    // Start local HTTP server on a random port.
    let listener = match tokio::net::TcpListener::bind("127.0.0.1:0").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error: failed to bind local HTTP server: {e}");
            std::process::exit(1);
        }
    };

    let local_addr = listener.local_addr().unwrap();
    let redirect_uri = format!("http://127.0.0.1:{}/callback", local_addr.port());

    let params = AuthorizeParams {
        auth_level,
        code_challenge: pkce.code_challenge.clone(),
        state: state.clone(),
        redirect_uri: Some(redirect_uri.clone()),
    };

    let authorize_url = auth::build_authorize_url(&endpoints, &params);

    // Print instructions.
    eprintln!("Starting Aula login ({auth_level})...");
    eprintln!();
    eprintln!("Opening your browser for authentication.");
    eprintln!("If the browser does not open, visit this URL manually:");
    eprintln!();
    eprintln!("  {authorize_url}");
    eprintln!();
    eprintln!(
        "Waiting for callback on http://127.0.0.1:{} (timeout: {timeout_secs}s)...",
        local_addr.port()
    );

    // Open browser (best-effort).
    if let Err(e) = open::that(authorize_url.as_str()) {
        eprintln!("warning: failed to open browser: {e}");
        eprintln!("Please open the URL above manually.");
    }

    // Wait for the redirect callback.
    let code = match wait_for_callback(&listener, &state, timeout_secs).await {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    eprintln!("Authorization code received, exchanging for tokens...");

    // Exchange code for tokens.
    let http = reqwest::Client::new();
    let token_response = match auth::exchange_code(
        &http,
        &endpoints,
        auth_level,
        &code,
        &pkce.code_verifier,
        Some(&redirect_uri),
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

    // Save tokens.
    let store = token_store();
    if let Err(e) = store.save(&login_data) {
        eprintln!("error: failed to save tokens: {e}");
        std::process::exit(1);
    }

    eprintln!();
    eprintln!("Login successful!");
    eprintln!("  Auth level: {auth_level}");
    if let Some(exp) = login_data.access_token_expiration {
        eprintln!("  Token expires: {}", format_unix_timestamp(exp));
    }
    eprintln!("  Tokens saved to: {}", store.dir().display());
}

// ---------------------------------------------------------------------------
// Callback server
// ---------------------------------------------------------------------------

/// Wait for the OIDC redirect callback on the local HTTP server.
///
/// Parses the `code` and `state` query parameters from the request URI.
/// Returns the authorization code on success, sends an HTML response to the
/// browser, and shuts down.
async fn wait_for_callback(
    listener: &tokio::net::TcpListener,
    expected_state: &str,
    timeout_secs: u64,
) -> Result<String, String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::time::{timeout, Duration};

    let duration = Duration::from_secs(timeout_secs);

    let (mut stream, _addr) = timeout(duration, listener.accept())
        .await
        .map_err(|_| format!("timed out after {timeout_secs}s waiting for browser callback"))?
        .map_err(|e| format!("failed to accept connection: {e}"))?;

    // Read the HTTP request (just need the first line for the URI).
    let mut buf = vec![0u8; 4096];
    let n = stream
        .read(&mut buf)
        .await
        .map_err(|e| format!("failed to read request: {e}"))?;
    let request = String::from_utf8_lossy(&buf[..n]);

    // Parse the request line: "GET /callback?code=...&state=... HTTP/1.1"
    let request_line = request
        .lines()
        .next()
        .ok_or_else(|| "empty HTTP request".to_string())?;

    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "malformed HTTP request line".to_string())?;

    // Parse query parameters from the path.
    let full_url = format!("http://localhost{path}");
    let parsed =
        url::Url::parse(&full_url).map_err(|e| format!("failed to parse callback URL: {e}"))?;

    // Extract code using the existing auth module helper.
    let result = auth::extract_code_from_redirect(&parsed, Some(expected_state));

    // Send response to browser before returning.
    let (status_line, body) = match &result {
        Ok(_) => (
            "HTTP/1.1 200 OK",
            "<html><body><h2>Login successful!</h2><p>You can close this tab and return to the terminal.</p></body></html>",
        ),
        Err(_) => (
            "HTTP/1.1 400 Bad Request",
            // The actual error is returned to the CLI.
            "<html><body><h2>Login failed</h2><p>Check the terminal for details.</p></body></html>",
        ),
    };

    let response =
        format!("{status_line}\r\nContent-Type: text/html\r\nConnection: close\r\n\r\n{body}");
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.shutdown().await;

    result.map_err(|e| format!("authorization failed: {e}"))
}

// ---------------------------------------------------------------------------
// Logout
// ---------------------------------------------------------------------------

async fn handle_logout(env_override: Option<&str>) {
    let environment = resolve_environment(env_override);
    let store = token_store();

    // Check if there are tokens to clear.
    if !store.exists() {
        eprintln!("No active session found.");
        return;
    }

    let client = match AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 19,
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

    println!("Logged in");
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
            println!("  Status: EXPIRED");
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
        println!("  Error: {err}");
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
        api_version: 19,
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
    // Format using the system time display (basic but no extra deps needed).
    let elapsed = dt.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    let total_secs = elapsed.as_secs();

    // Manual UTC breakdown.
    let secs_per_day: u64 = 86400;
    let days = total_secs / secs_per_day;
    let day_secs = total_secs % secs_per_day;
    let hours = day_secs / 3600;
    let mins = (day_secs % 3600) / 60;
    let secs = day_secs % 60;

    // Days since epoch to Y-M-D (simplified Gregorian).
    let (year, month, day) = days_to_ymd(days);

    format!("{year:04}-{month:02}-{day:02} {hours:02}:{mins:02}:{secs:02} UTC")
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
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
