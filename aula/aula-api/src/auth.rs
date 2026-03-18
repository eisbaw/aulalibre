//! OIDC Authorization Code + PKCE authentication for the Aula platform.
//!
//! Implements the authentication flow discovered in the decompiled APK
//! (see `auth_flow.md`). The provider is a SimpleSAMLphp instance at
//! `login.aula.dk` with two authentication levels:
//!
//! - **Level 2 (UniLogin)**: standard school login, scope `"aula"`
//! - **Level 3 (MitID)**: elevated access for sensitive documents, scope `"aula-sensitive"`
//!
//! # Protocol overview
//!
//! 1. Generate a PKCE code verifier and S256 challenge.
//! 2. Build the authorization URL and direct the user's browser there.
//! 3. Intercept the redirect to `https://app-private.aula.dk` carrying the
//!    authorization code.
//! 4. Exchange the code (+ code verifier) at the token endpoint for tokens.
//!
//! This module does **not** automate the browser interaction; it provides the
//! URL construction, PKCE cryptography, and token exchange HTTP calls.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

use crate::client::Environment;
use crate::error::AulaError;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Redirect URI used by the Aula mobile app.
const REDIRECT_URI: &str = "https://app-private.aula.dk";

/// Authorize endpoint path (SimpleSAMLphp OIDC module).
const AUTHORIZE_PATH: &str = "/simplesaml/module.php/oidc/authorize.php";

/// Token endpoint path (SimpleSAMLphp OIDC module).
const TOKEN_PATH: &str = "/simplesaml/module.php/oidc/token.php";

/// PKCE code verifier length in bytes (before base64url encoding).
/// RFC 7636 recommends 32 bytes (256 bits) of entropy.
const CODE_VERIFIER_BYTES: usize = 32;

// ---------------------------------------------------------------------------
// AuthLevel
// ---------------------------------------------------------------------------

/// Authentication level, mirroring the step-up mechanism in the APK.
///
/// The Aula app stores this in `SecureStorageManager` under the key
/// `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthLevel {
    /// Level 2: standard UniLogin authentication.
    #[serde(rename = "level2")]
    Level2,
    /// Level 3: elevated MitID/NemID authentication for sensitive data.
    #[serde(rename = "level3")]
    Level3,
}

impl AuthLevel {
    /// OIDC client ID for this authentication level.
    ///
    /// These are the literal client IDs found in the decompiled
    /// `AuthenticationManager.CreateOidcClient()`.
    pub fn client_id(&self) -> &'static str {
        match self {
            Self::Level2 => "_742adb5e2759028d86dbadf4af44ef70e8b1f407a6",
            Self::Level3 => "_99949a54b8b65423862aac1bf629599ed64231607a",
        }
    }

    /// OIDC scope for this authentication level.
    pub fn scope(&self) -> &'static str {
        match self {
            Self::Level2 => "aula",
            Self::Level3 => "aula-sensitive",
        }
    }
}

impl std::fmt::Display for AuthLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Level2 => write!(f, "Level 2 (UniLogin)"),
            Self::Level3 => write!(f, "Level 3 (MitID)"),
        }
    }
}

// ---------------------------------------------------------------------------
// PKCE
// ---------------------------------------------------------------------------

/// PKCE (Proof Key for Code Exchange) challenge pair per RFC 7636.
///
/// The code verifier is a high-entropy random string; the code challenge is
/// its SHA-256 hash, base64url-encoded without padding.
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    /// The plain code verifier sent during token exchange.
    pub code_verifier: String,
    /// The S256 challenge sent during authorization.
    pub code_challenge: String,
}

impl PkceChallenge {
    /// Generate a fresh PKCE challenge pair using the system CSPRNG.
    pub fn generate() -> Self {
        let mut bytes = [0u8; CODE_VERIFIER_BYTES];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self::from_verifier_bytes(&bytes)
    }

    /// Build a challenge from raw verifier bytes.
    ///
    /// Useful in tests where deterministic output is needed.
    pub fn from_verifier_bytes(bytes: &[u8]) -> Self {
        let code_verifier = URL_SAFE_NO_PAD.encode(bytes);
        let digest = Sha256::digest(code_verifier.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(digest);
        Self {
            code_verifier,
            code_challenge,
        }
    }
}

// ---------------------------------------------------------------------------
// OidcEndpoints
// ---------------------------------------------------------------------------

/// OIDC endpoint URLs derived from an [`Environment`].
#[derive(Debug, Clone)]
pub struct OidcEndpoints {
    /// Full authorize endpoint URL.
    pub authorize_url: Url,
    /// Full token endpoint URL.
    pub token_url: Url,
    /// Issuer / authority base URL.
    pub issuer: Url,
}

impl OidcEndpoints {
    /// Construct endpoints for the given environment.
    pub fn for_environment(env: &Environment) -> Self {
        let base = format!("https://{}", env.auth_host());
        let issuer = Url::parse(&format!("{base}/")).expect("valid issuer URL");
        let authorize_url =
            Url::parse(&format!("{base}{AUTHORIZE_PATH}")).expect("valid authorize URL");
        let token_url = Url::parse(&format!("{base}{TOKEN_PATH}")).expect("valid token URL");

        Self {
            authorize_url,
            token_url,
            issuer,
        }
    }
}

// ---------------------------------------------------------------------------
// Authorization URL builder
// ---------------------------------------------------------------------------

/// Parameters for building an authorization URL.
#[derive(Debug, Clone)]
pub struct AuthorizeParams {
    /// Authentication level (determines client_id and scope).
    pub auth_level: AuthLevel,
    /// PKCE code challenge (S256).
    pub code_challenge: String,
    /// OIDC state parameter for CSRF protection.
    pub state: String,
}

/// Build the full authorization URL with all required query parameters.
///
/// The returned URL can be opened in a browser to start the login flow.
pub fn build_authorize_url(endpoints: &OidcEndpoints, params: &AuthorizeParams) -> Url {
    let mut url = endpoints.authorize_url.clone();

    {
        let mut q = url.query_pairs_mut();
        q.append_pair("response_type", "code");
        q.append_pair("client_id", params.auth_level.client_id());
        q.append_pair("scope", params.auth_level.scope());
        q.append_pair("redirect_uri", REDIRECT_URI);
        q.append_pair("code_challenge", &params.code_challenge);
        q.append_pair("code_challenge_method", "S256");
        q.append_pair("state", &params.state);
    }

    url
}

/// Generate a random state string for the OIDC state parameter.
pub fn generate_state() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

// ---------------------------------------------------------------------------
// Token exchange
// ---------------------------------------------------------------------------

/// Token response from the OIDC token endpoint.
///
/// Mirrors the fields from a standard OIDC token response. The Aula app
/// wraps these into its `LoginData` type after receiving them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// The access token (opaque string; no client-side JWT validation).
    pub access_token: String,

    /// Token type, typically `"Bearer"`.
    pub token_type: String,

    /// Lifetime of the access token in seconds.
    #[serde(default)]
    pub expires_in: Option<u64>,

    /// Refresh token for obtaining new access tokens.
    #[serde(default)]
    pub refresh_token: Option<String>,

    /// OIDC ID token (JWT, but not validated client-side per Aula's impl).
    #[serde(default)]
    pub id_token: Option<String>,

    /// Granted scope (may differ from requested scope).
    #[serde(default)]
    pub scope: Option<String>,
}

/// Error response from the token endpoint (RFC 6749 Section 5.2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenErrorResponse {
    /// Error code (e.g. `"invalid_grant"`, `"invalid_request"`).
    pub error: String,

    /// Human-readable error description.
    #[serde(default)]
    pub error_description: Option<String>,
}

/// Exchange an authorization code for tokens at the token endpoint.
///
/// Sends a `POST` with `application/x-www-form-urlencoded` body containing
/// the authorization code, code verifier, redirect URI, and client ID.
pub async fn exchange_code(
    http: &reqwest::Client,
    endpoints: &OidcEndpoints,
    auth_level: AuthLevel,
    code: &str,
    code_verifier: &str,
) -> crate::Result<TokenResponse> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", REDIRECT_URI),
        ("client_id", auth_level.client_id()),
        ("code_verifier", code_verifier),
    ];

    let resp = http
        .post(endpoints.token_url.as_str())
        .form(&params)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        // Try to parse as OAuth error response.
        if let Ok(err) = serde_json::from_str::<TokenErrorResponse>(&body) {
            return Err(AulaError::Auth {
                error: err.error,
                description: err.error_description,
            });
        }
        return Err(AulaError::Auth {
            error: format!("token_exchange_failed (HTTP {status})"),
            description: Some(body),
        });
    }

    serde_json::from_str(&body).map_err(AulaError::from)
}

/// Refresh an access token using a refresh token.
///
/// Sends a `POST` with `grant_type=refresh_token` to the token endpoint.
pub async fn refresh_token(
    http: &reqwest::Client,
    endpoints: &OidcEndpoints,
    auth_level: AuthLevel,
    refresh_tok: &str,
) -> crate::Result<TokenResponse> {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_tok),
        ("client_id", auth_level.client_id()),
    ];

    let resp = http
        .post(endpoints.token_url.as_str())
        .form(&params)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<TokenErrorResponse>(&body) {
            return Err(AulaError::Auth {
                error: err.error,
                description: err.error_description,
            });
        }
        return Err(AulaError::Auth {
            error: format!("token_refresh_failed (HTTP {status})"),
            description: Some(body),
        });
    }

    serde_json::from_str(&body).map_err(AulaError::from)
}

// ---------------------------------------------------------------------------
// LoginData
// ---------------------------------------------------------------------------

/// Persisted login data, mirroring `AulaNative.OAuth.LoginData` from the APK.
///
/// Constructed from a [`TokenResponse`] after successful authentication or
/// token refresh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    /// The access token.
    pub access_token: String,

    /// The refresh token (if issued).
    pub refresh_token: Option<String>,

    /// Token lifetime in seconds from the original response.
    pub expires_in: Option<u64>,

    /// Unix timestamp (seconds) when the access token expires.
    /// Computed as `now + expires_in` at the time of token receipt.
    pub access_token_expiration: Option<u64>,

    /// The authentication level used to obtain these tokens.
    pub auth_level: AuthLevel,

    /// Error message if login failed.
    pub error: Option<String>,

    /// Detailed error description.
    pub error_description: Option<String>,
}

impl LoginData {
    /// Create `LoginData` from a successful token response.
    pub fn from_token_response(resp: TokenResponse, auth_level: AuthLevel) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let expiration = resp.expires_in.map(|ei| now + ei);

        Self {
            access_token: resp.access_token,
            refresh_token: resp.refresh_token,
            expires_in: resp.expires_in,
            access_token_expiration: expiration,
            auth_level,
            error: None,
            error_description: None,
        }
    }

    /// Check whether the access token has expired.
    ///
    /// Returns `true` if the expiration time is known and has passed.
    /// Returns `false` if expiration is unknown (conservative: assume valid).
    pub fn is_expired(&self) -> bool {
        let Some(exp) = self.access_token_expiration else {
            return false;
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        now >= exp
    }

    /// Create an error `LoginData` (no tokens, just error info).
    pub fn error(error: String, description: Option<String>, auth_level: AuthLevel) -> Self {
        Self {
            access_token: String::new(),
            refresh_token: None,
            expires_in: None,
            access_token_expiration: None,
            auth_level,
            error: Some(error),
            error_description: description,
        }
    }
}

// ---------------------------------------------------------------------------
// Redirect URI helper
// ---------------------------------------------------------------------------

/// The redirect URI used in authorization requests.
pub fn redirect_uri() -> &'static str {
    REDIRECT_URI
}

/// Extract the authorization code from a redirect URL.
///
/// After the user authenticates, the browser is redirected to
/// `https://app-private.aula.dk?code=...&state=...`. This function
/// extracts the `code` parameter and optionally verifies the `state`.
pub fn extract_code_from_redirect(
    redirect_url: &Url,
    expected_state: Option<&str>,
) -> crate::Result<String> {
    let mut code: Option<String> = None;
    let mut state: Option<String> = None;
    let mut error: Option<String> = None;
    let mut error_description: Option<String> = None;

    for (key, value) in redirect_url.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.into_owned()),
            "state" => state = Some(value.into_owned()),
            "error" => error = Some(value.into_owned()),
            "error_description" => error_description = Some(value.into_owned()),
            _ => {}
        }
    }

    // Check for OAuth error in the redirect.
    if let Some(err) = error {
        return Err(AulaError::Auth {
            error: err,
            description: error_description,
        });
    }

    // Verify state if expected.
    if let Some(expected) = expected_state {
        match &state {
            Some(s) if s != expected => {
                return Err(AulaError::Auth {
                    error: "state_mismatch".to_string(),
                    description: Some("OIDC state parameter does not match".to_string()),
                });
            }
            None => {
                return Err(AulaError::Auth {
                    error: "missing_state".to_string(),
                    description: Some("redirect URL is missing the state parameter".to_string()),
                });
            }
            _ => {}
        }
    }

    code.ok_or_else(|| AulaError::Auth {
        error: "missing_code".to_string(),
        description: Some("redirect URL is missing the authorization code".to_string()),
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- AuthLevel ----------------------------------------------------------

    #[test]
    fn auth_level_client_ids() {
        assert_eq!(
            AuthLevel::Level2.client_id(),
            "_742adb5e2759028d86dbadf4af44ef70e8b1f407a6"
        );
        assert_eq!(
            AuthLevel::Level3.client_id(),
            "_99949a54b8b65423862aac1bf629599ed64231607a"
        );
    }

    #[test]
    fn auth_level_scopes() {
        assert_eq!(AuthLevel::Level2.scope(), "aula");
        assert_eq!(AuthLevel::Level3.scope(), "aula-sensitive");
    }

    #[test]
    fn auth_level_display() {
        assert_eq!(AuthLevel::Level2.to_string(), "Level 2 (UniLogin)");
        assert_eq!(AuthLevel::Level3.to_string(), "Level 3 (MitID)");
    }

    #[test]
    fn auth_level_serde_roundtrip() {
        let json = serde_json::to_string(&AuthLevel::Level2).unwrap();
        assert_eq!(json, r#""level2""#);
        let back: AuthLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(back, AuthLevel::Level2);

        let json = serde_json::to_string(&AuthLevel::Level3).unwrap();
        assert_eq!(json, r#""level3""#);
        let back: AuthLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(back, AuthLevel::Level3);
    }

    // -- PKCE ---------------------------------------------------------------

    #[test]
    fn pkce_deterministic_from_known_bytes() {
        // Use known bytes so we can verify the output.
        let bytes = [0u8; CODE_VERIFIER_BYTES];
        let pkce = PkceChallenge::from_verifier_bytes(&bytes);

        // Verifier should be base64url of 32 zero bytes.
        assert_eq!(pkce.code_verifier, URL_SAFE_NO_PAD.encode([0u8; 32]));

        // Challenge should be SHA-256 of the verifier string, base64url encoded.
        let expected_digest = Sha256::digest(pkce.code_verifier.as_bytes());
        let expected_challenge = URL_SAFE_NO_PAD.encode(expected_digest);
        assert_eq!(pkce.code_challenge, expected_challenge);
    }

    #[test]
    fn pkce_generate_produces_unique_values() {
        let a = PkceChallenge::generate();
        let b = PkceChallenge::generate();
        // Extremely unlikely to collide with 256 bits of entropy.
        assert_ne!(a.code_verifier, b.code_verifier);
        assert_ne!(a.code_challenge, b.code_challenge);
    }

    #[test]
    fn pkce_verifier_is_base64url_without_padding() {
        let pkce = PkceChallenge::generate();
        // No padding characters.
        assert!(!pkce.code_verifier.contains('='));
        // No standard base64 characters that differ from base64url.
        assert!(!pkce.code_verifier.contains('+'));
        assert!(!pkce.code_verifier.contains('/'));
    }

    #[test]
    fn pkce_challenge_is_base64url_without_padding() {
        let pkce = PkceChallenge::generate();
        assert!(!pkce.code_challenge.contains('='));
        assert!(!pkce.code_challenge.contains('+'));
        assert!(!pkce.code_challenge.contains('/'));
    }

    // -- OidcEndpoints ------------------------------------------------------

    #[test]
    fn endpoints_production() {
        let ep = OidcEndpoints::for_environment(&Environment::Production);
        assert_eq!(
            ep.authorize_url.as_str(),
            "https://login.aula.dk/simplesaml/module.php/oidc/authorize.php"
        );
        assert_eq!(
            ep.token_url.as_str(),
            "https://login.aula.dk/simplesaml/module.php/oidc/token.php"
        );
        assert_eq!(ep.issuer.as_str(), "https://login.aula.dk/");
    }

    #[test]
    fn endpoints_preprod() {
        let ep = OidcEndpoints::for_environment(&Environment::Preprod);
        assert_eq!(
            ep.authorize_url.as_str(),
            "https://login-preprod.aula.dk/simplesaml/module.php/oidc/authorize.php"
        );
        assert_eq!(
            ep.token_url.as_str(),
            "https://login-preprod.aula.dk/simplesaml/module.php/oidc/token.php"
        );
    }

    #[test]
    fn endpoints_test_env_uses_shared_host() {
        let ep = OidcEndpoints::for_environment(&Environment::Test1);
        assert!(ep.authorize_url.as_str().contains("www1-test1.ncaula.com"));
        assert!(ep.token_url.as_str().contains("www1-test1.ncaula.com"));
    }

    // -- Authorization URL --------------------------------------------------

    #[test]
    fn authorize_url_level2() {
        let ep = OidcEndpoints::for_environment(&Environment::Production);
        let params = AuthorizeParams {
            auth_level: AuthLevel::Level2,
            code_challenge: "test_challenge".to_string(),
            state: "test_state".to_string(),
        };
        let url = build_authorize_url(&ep, &params);
        let url_str = url.as_str();

        assert!(
            url_str.starts_with("https://login.aula.dk/simplesaml/module.php/oidc/authorize.php?")
        );
        assert!(url_str.contains("response_type=code"));
        assert!(url_str.contains(&format!("client_id={}", AuthLevel::Level2.client_id())));
        assert!(url_str.contains("scope=aula"));
        assert!(url_str.contains("redirect_uri=https%3A%2F%2Fapp-private.aula.dk"));
        assert!(url_str.contains("code_challenge=test_challenge"));
        assert!(url_str.contains("code_challenge_method=S256"));
        assert!(url_str.contains("state=test_state"));
    }

    #[test]
    fn authorize_url_level3() {
        let ep = OidcEndpoints::for_environment(&Environment::Production);
        let params = AuthorizeParams {
            auth_level: AuthLevel::Level3,
            code_challenge: "challenge_xyz".to_string(),
            state: "state_abc".to_string(),
        };
        let url = build_authorize_url(&ep, &params);
        let url_str = url.as_str();

        assert!(url_str.contains(&format!("client_id={}", AuthLevel::Level3.client_id())));
        assert!(url_str.contains("scope=aula-sensitive"));
    }

    // -- State generation ---------------------------------------------------

    #[test]
    fn generate_state_is_unique() {
        let a = generate_state();
        let b = generate_state();
        assert_ne!(a, b);
    }

    #[test]
    fn generate_state_is_base64url() {
        let s = generate_state();
        assert!(!s.contains('='));
        assert!(!s.contains('+'));
        assert!(!s.contains('/'));
        assert!(!s.is_empty());
    }

    // -- Redirect code extraction -------------------------------------------

    #[test]
    fn extract_code_success() {
        let url = Url::parse("https://app-private.aula.dk?code=abc123&state=mystate").unwrap();
        let code = extract_code_from_redirect(&url, Some("mystate")).unwrap();
        assert_eq!(code, "abc123");
    }

    #[test]
    fn extract_code_without_state_check() {
        let url = Url::parse("https://app-private.aula.dk?code=xyz").unwrap();
        let code = extract_code_from_redirect(&url, None).unwrap();
        assert_eq!(code, "xyz");
    }

    #[test]
    fn extract_code_state_mismatch() {
        let url = Url::parse("https://app-private.aula.dk?code=abc&state=wrong").unwrap();
        let err = extract_code_from_redirect(&url, Some("expected")).unwrap_err();
        match err {
            AulaError::Auth { error, .. } => assert_eq!(error, "state_mismatch"),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[test]
    fn extract_code_missing_state() {
        let url = Url::parse("https://app-private.aula.dk?code=abc").unwrap();
        let err = extract_code_from_redirect(&url, Some("expected")).unwrap_err();
        match err {
            AulaError::Auth { error, .. } => assert_eq!(error, "missing_state"),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[test]
    fn extract_code_missing_code() {
        let url = Url::parse("https://app-private.aula.dk?state=abc").unwrap();
        let err = extract_code_from_redirect(&url, None).unwrap_err();
        match err {
            AulaError::Auth { error, .. } => assert_eq!(error, "missing_code"),
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    #[test]
    fn extract_code_oauth_error() {
        let url = Url::parse(
            "https://app-private.aula.dk?error=access_denied&error_description=User+cancelled",
        )
        .unwrap();
        let err = extract_code_from_redirect(&url, None).unwrap_err();
        match err {
            AulaError::Auth {
                error, description, ..
            } => {
                assert_eq!(error, "access_denied");
                assert_eq!(description.as_deref(), Some("User cancelled"));
            }
            other => panic!("expected Auth error, got {other:?}"),
        }
    }

    // -- TokenResponse serde ------------------------------------------------

    #[test]
    fn token_response_full_deserialize() {
        let json = r#"{
            "access_token": "at_123",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "rt_456",
            "id_token": "ey...",
            "scope": "aula"
        }"#;
        let resp: TokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "at_123");
        assert_eq!(resp.token_type, "Bearer");
        assert_eq!(resp.expires_in, Some(3600));
        assert_eq!(resp.refresh_token.as_deref(), Some("rt_456"));
        assert_eq!(resp.id_token.as_deref(), Some("ey..."));
        assert_eq!(resp.scope.as_deref(), Some("aula"));
    }

    #[test]
    fn token_response_minimal_deserialize() {
        let json = r#"{
            "access_token": "at",
            "token_type": "Bearer"
        }"#;
        let resp: TokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.access_token, "at");
        assert!(resp.expires_in.is_none());
        assert!(resp.refresh_token.is_none());
        assert!(resp.id_token.is_none());
    }

    // -- LoginData ----------------------------------------------------------

    #[test]
    fn login_data_from_token_response() {
        let resp = TokenResponse {
            access_token: "at".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: Some(3600),
            refresh_token: Some("rt".to_string()),
            id_token: None,
            scope: None,
        };
        let ld = LoginData::from_token_response(resp, AuthLevel::Level2);
        assert_eq!(ld.access_token, "at");
        assert_eq!(ld.refresh_token.as_deref(), Some("rt"));
        assert_eq!(ld.auth_level, AuthLevel::Level2);
        assert!(ld.access_token_expiration.is_some());
        assert!(ld.error.is_none());
    }

    #[test]
    fn login_data_error_variant() {
        let ld = LoginData::error(
            "bad_grant".to_string(),
            Some("expired".to_string()),
            AuthLevel::Level3,
        );
        assert!(ld.access_token.is_empty());
        assert_eq!(ld.error.as_deref(), Some("bad_grant"));
        assert_eq!(ld.auth_level, AuthLevel::Level3);
    }

    #[test]
    fn login_data_is_expired_with_past_time() {
        let ld = LoginData {
            access_token: "at".to_string(),
            refresh_token: None,
            expires_in: Some(0),
            access_token_expiration: Some(0), // Unix epoch = definitely past
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        };
        assert!(ld.is_expired());
    }

    #[test]
    fn login_data_is_expired_with_future_time() {
        let far_future = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 999_999;
        let ld = LoginData {
            access_token: "at".to_string(),
            refresh_token: None,
            expires_in: Some(999_999),
            access_token_expiration: Some(far_future),
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        };
        assert!(!ld.is_expired());
    }

    #[test]
    fn login_data_is_expired_unknown() {
        let ld = LoginData {
            access_token: "at".to_string(),
            refresh_token: None,
            expires_in: None,
            access_token_expiration: None,
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        };
        // Conservative: unknown expiration means not expired.
        assert!(!ld.is_expired());
    }

    #[test]
    fn login_data_serde_roundtrip() {
        let ld = LoginData {
            access_token: "token".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_in: Some(3600),
            access_token_expiration: Some(1700000000),
            auth_level: AuthLevel::Level3,
            error: None,
            error_description: None,
        };
        let json = serde_json::to_string(&ld).unwrap();
        let back: LoginData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.access_token, "token");
        assert_eq!(back.auth_level, AuthLevel::Level3);
        assert_eq!(back.access_token_expiration, Some(1700000000));
    }

    // -- Redirect URI -------------------------------------------------------

    #[test]
    fn redirect_uri_is_correct() {
        assert_eq!(redirect_uri(), "https://app-private.aula.dk");
    }

    // -- TokenErrorResponse -------------------------------------------------

    #[test]
    fn token_error_response_deserialize() {
        let json = r#"{
            "error": "invalid_grant",
            "error_description": "Authorization code expired"
        }"#;
        let err: TokenErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err.error, "invalid_grant");
        assert_eq!(
            err.error_description.as_deref(),
            Some("Authorization code expired")
        );
    }
}
