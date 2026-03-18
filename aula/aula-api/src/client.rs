//! HTTP client for the Aula school platform API.
//!
//! Wraps [`reqwest::Client`] with automatic cookie handling, CSRF token
//! management, and environment-aware base URL construction.
//!
//! # CSRF flow
//!
//! The Aula API sets a `Csrfp-Token` cookie after login. Every subsequent
//! mutating request must echo that value back as a `csrfp-token` HTTP header.
//! [`AulaClient`] handles this transparently.

use std::sync::Arc;

use reqwest::cookie::{CookieStore, Jar};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::AulaError;
use crate::response::{AulaServiceResponse, WebResponseStatusSubCode};

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

/// Aula deployment environment.
///
/// Each variant maps to the (backend, auth) host pair discovered in the APK's
/// `EnvironmentFactory` (see `auth_flow.md` Section 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    /// Production (`www.aula.dk` / `login.aula.dk`).
    Production,
    /// Pre-production (`www1-preprod.aula.dk` / `login-preprod.aula.dk`).
    Preprod,
    /// Hotfix (`www1-hotfix.aula.dk` / `login-hotfix.aula.dk`).
    Hotfix,
    /// Test1 (`www1-test1.ncaula.com`).
    Test1,
    /// Test3 (`www1-test3.ncaula.com`).
    Test3,
    /// Dev1 (`www1-dev1.ncaula.com`).
    Dev1,
    /// Dev3 (`www1-dev3.ncaula.com`).
    Dev3,
    /// Dev11 (`www1-dev11.ncaula.com`).
    Dev11,
}

impl Environment {
    /// Backend host for API calls.
    pub fn backend_host(&self) -> &str {
        match self {
            Self::Production => "www.aula.dk",
            Self::Preprod => "www1-preprod.aula.dk",
            Self::Hotfix => "www1-hotfix.aula.dk",
            Self::Test1 => "www1-test1.ncaula.com",
            Self::Test3 => "www1-test3.ncaula.com",
            Self::Dev1 => "www1-dev1.ncaula.com",
            Self::Dev3 => "www1-dev3.ncaula.com",
            Self::Dev11 => "www1-dev11.ncaula.com",
        }
    }

    /// Auth (login) host.
    pub fn auth_host(&self) -> &str {
        match self {
            Self::Production => "login.aula.dk",
            Self::Preprod => "login-preprod.aula.dk",
            Self::Hotfix => "login-hotfix.aula.dk",
            // Non-prod test/dev share the same host for both backend and auth.
            Self::Test1 => "www1-test1.ncaula.com",
            Self::Test3 => "www1-test3.ncaula.com",
            Self::Dev1 => "www1-dev1.ncaula.com",
            Self::Dev3 => "www1-dev3.ncaula.com",
            Self::Dev11 => "www1-dev11.ncaula.com",
        }
    }

    /// Whether this environment requires HTTP Basic Auth for access.
    ///
    /// All non-production environments use `aula-user:Aula-1337`.
    pub fn requires_basic_auth(&self) -> bool {
        !matches!(self, Self::Production)
    }
}

// ---------------------------------------------------------------------------
// CSRF cookie name and header
// ---------------------------------------------------------------------------

/// Cookie name set by the Aula backend.
const CSRF_COOKIE_NAME: &str = "Csrfp-Token";

/// Header name the API expects the CSRF token in.
const CSRF_HEADER_NAME: &str = "csrfp-token";

/// Basic auth username for non-production environments.
const BASIC_AUTH_USER: &str = "aula-user";

/// Basic auth password for non-production environments.
const BASIC_AUTH_PASS: &str = "Aula-1337";

// ---------------------------------------------------------------------------
// AulaClient
// ---------------------------------------------------------------------------

/// Configuration for building an [`AulaClient`].
#[derive(Debug, Clone)]
pub struct AulaClientConfig {
    /// Target environment (default: [`Environment::Production`]).
    pub environment: Environment,
    /// API version number (default: 19).
    pub api_version: u32,
}

impl Default for AulaClientConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Production,
            api_version: 19,
        }
    }
}

/// Async HTTP client for the Aula API.
///
/// Manages:
/// - A [`reqwest::Client`] with an attached cookie store.
/// - Automatic CSRF token extraction from the `Csrfp-Token` cookie and
///   injection as the `csrfp-token` request header.
/// - Base URL construction per environment.
/// - Non-production basic auth.
pub struct AulaClient {
    http: reqwest::Client,
    cookie_jar: Arc<Jar>,
    base_url: Url,
    environment: Environment,
    /// Whether to send basic auth on every request.
    use_basic_auth: bool,
}

impl AulaClient {
    /// Create a new client with default configuration (production, API v19).
    pub fn new() -> crate::Result<Self> {
        Self::with_config(AulaClientConfig::default())
    }

    /// Create a new client with the given configuration.
    pub fn with_config(config: AulaClientConfig) -> crate::Result<Self> {
        let base_url = format!(
            "https://{}/api/v{}/",
            config.environment.backend_host(),
            config.api_version,
        );
        let base_url = Url::parse(&base_url).expect("valid base URL");

        let cookie_jar = Arc::new(Jar::default());

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let builder = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .default_headers(default_headers)
            .user_agent("AulaNative/2.15.4");

        let http = builder.build()?;

        let use_basic_auth = config.environment.requires_basic_auth();

        Ok(Self {
            http,
            cookie_jar,
            base_url,
            environment: config.environment,
            use_basic_auth,
        })
    }

    /// Create a client pointing at a custom base URL (for testing with mock servers).
    ///
    /// The URL must end with a trailing slash (e.g., `http://127.0.0.1:9090/api/v19/`).
    /// No basic auth is applied and the environment is set to Production.
    pub fn with_base_url(base_url: &str) -> crate::Result<Self> {
        let base_url = Url::parse(base_url).expect("valid base URL");

        let cookie_jar = Arc::new(Jar::default());

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .default_headers(default_headers)
            .user_agent("AulaNative/2.15.4")
            .build()?;

        Ok(Self {
            http,
            cookie_jar,
            base_url,
            environment: Environment::Production,
            use_basic_auth: false,
        })
    }

    /// The environment this client targets.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// The base URL for API requests (e.g. `https://www.aula.dk/api/v19/`).
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Reference to the underlying [`reqwest::Client`].
    ///
    /// Useful for making raw requests that bypass the Aula envelope parsing.
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    // -- Request decoration -------------------------------------------------

    /// Apply common headers to a request builder: CSRF token and basic auth.
    fn decorate(&self, mut req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = self.csrf_token() {
            req = req.header(CSRF_HEADER_NAME, token);
        }
        if self.use_basic_auth {
            req = req.basic_auth(BASIC_AUTH_USER, Some(BASIC_AUTH_PASS));
        }
        req
    }

    // -- CSRF ---------------------------------------------------------------

    /// Read the current CSRF token from the cookie jar, if present.
    fn csrf_token(&self) -> Option<String> {
        // We look up cookies for the base URL; the backend sets Csrfp-Token
        // there after authentication.
        let header_value = self.cookie_jar.cookies(&self.base_url)?;
        let header_str = header_value.to_str().ok()?;

        // Cookie header format: "name1=value1; name2=value2"
        for pair in header_str.split(';') {
            let pair = pair.trim();
            if let Some(value) = pair.strip_prefix(CSRF_COOKIE_NAME) {
                // Handle "Csrfp-Token=VALUE"
                let value = value.strip_prefix('=')?;
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    // -- Request helpers ----------------------------------------------------

    /// Build a full URL by appending `path` to the base URL.
    fn url(&self, path: &str) -> Url {
        // Strip leading slash if caller includes one, since base_url ends
        // with a slash.
        let path = path.strip_prefix('/').unwrap_or(path);
        self.base_url
            .join(path)
            .expect("path should be valid URL segment")
    }

    /// Send a GET request and deserialize the envelope payload.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> crate::Result<T> {
        let resp = self.decorate(self.http.get(self.url(path))).send().await?;
        self.handle_response(resp).await
    }

    /// Send a POST request with a JSON body and deserialize the envelope payload.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        let resp = self
            .decorate(self.http.post(self.url(path)).json(body))
            .send()
            .await?;
        self.handle_response(resp).await
    }

    /// Send a POST request without a body.
    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> crate::Result<T> {
        let resp = self.decorate(self.http.post(self.url(path))).send().await?;
        self.handle_response(resp).await
    }

    /// Send a PUT request with a JSON body and deserialize the envelope payload.
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        let resp = self
            .decorate(self.http.put(self.url(path)).json(body))
            .send()
            .await?;
        self.handle_response(resp).await
    }

    /// Send a DELETE request and deserialize the envelope payload.
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> crate::Result<T> {
        let resp = self
            .decorate(self.http.delete(self.url(path)))
            .send()
            .await?;
        self.handle_response(resp).await
    }

    /// Send a DELETE request with a JSON body and deserialize the envelope payload.
    pub async fn delete_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        let resp = self
            .decorate(self.http.delete(self.url(path)).json(body))
            .send()
            .await?;
        self.handle_response(resp).await
    }

    // -- Keep-alive ---------------------------------------------------------

    /// Send a keep-alive ping to extend the current session.
    ///
    /// Maps to `POST /profiles/keepAlive` which the
    /// `SessionPromptManager` calls periodically.
    pub async fn keep_alive(&self) -> crate::Result<()> {
        // The keep-alive endpoint returns an AulaServiceResponse with empty data.
        let _: serde_json::Value = self.post_empty("profiles/keepAlive").await?;
        Ok(())
    }

    // -- Test helpers -------------------------------------------------------

    /// Add a cookie string to the client's cookie jar for the base URL.
    ///
    /// Useful in tests to simulate the server setting cookies (e.g., CSRF tokens).
    pub fn set_cookie(&self, cookie_str: &str) {
        self.cookie_jar.add_cookie_str(cookie_str, &self.base_url);
    }

    // -- Response handling --------------------------------------------------

    /// Parse an HTTP response as an `AulaServiceResponse<T>`, mapping API-level
    /// errors to [`AulaError`] variants.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> crate::Result<T> {
        let status_code = resp.status();

        // Map well-known HTTP status codes to specific error variants before
        // trying to parse the body (the body may not be valid JSON on some
        // error responses).
        if status_code == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AulaError::Unauthorized);
        }
        if status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
            return Err(AulaError::Maintenance);
        }

        let body = resp.text().await?;

        let envelope: AulaServiceResponse<T> = serde_json::from_str(&body).map_err(|e| {
            // If we can't parse the envelope at all and the status was not 2xx,
            // return a generic API error rather than a confusing JSON error.
            if !status_code.is_success() {
                AulaError::Api {
                    message: format!("HTTP {status_code}: {body}"),
                    status: None,
                }
            } else {
                AulaError::Json(e)
            }
        })?;

        // Check envelope-level errors via sub-code.
        if let Some(sub_code) = envelope.status.sub_code {
            match WebResponseStatusSubCode::from_code(sub_code) {
                Some(WebResponseStatusSubCode::InvalidToken) => {
                    return Err(AulaError::InvalidAccessToken);
                }
                Some(WebResponseStatusSubCode::SessionExpired) => {
                    return Err(AulaError::SessionExpired);
                }
                Some(WebResponseStatusSubCode::AuthorizationStepUpRequired) => {
                    return Err(AulaError::StepUpRequired);
                }
                Some(WebResponseStatusSubCode::AuthorizationDeniedUserDeactivated) => {
                    return Err(AulaError::UserDeactivated);
                }
                _ => {}
            }
        }

        // Non-zero backend error code with no mapped sub-code.
        if envelope.status.backend_error_code != 0 {
            return Err(AulaError::Api {
                message: envelope.status.message.clone().unwrap_or_else(|| {
                    format!("backend error {}", envelope.status.backend_error_code)
                }),
                status: Some(envelope.status),
            });
        }

        Ok(envelope.data)
    }
}

impl std::fmt::Debug for AulaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AulaClient")
            .field("base_url", &self.base_url.as_str())
            .field("environment", &self.environment)
            .finish_non_exhaustive()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Environment --------------------------------------------------------

    #[test]
    fn production_hosts() {
        let env = Environment::Production;
        assert_eq!(env.backend_host(), "www.aula.dk");
        assert_eq!(env.auth_host(), "login.aula.dk");
        assert!(!env.requires_basic_auth());
    }

    #[test]
    fn preprod_hosts() {
        let env = Environment::Preprod;
        assert_eq!(env.backend_host(), "www1-preprod.aula.dk");
        assert_eq!(env.auth_host(), "login-preprod.aula.dk");
        assert!(env.requires_basic_auth());
    }

    #[test]
    fn hotfix_hosts() {
        let env = Environment::Hotfix;
        assert_eq!(env.backend_host(), "www1-hotfix.aula.dk");
        assert_eq!(env.auth_host(), "login-hotfix.aula.dk");
        assert!(env.requires_basic_auth());
    }

    #[test]
    fn test_environments_share_host_for_auth() {
        for env in [Environment::Test1, Environment::Test3] {
            assert_eq!(
                env.backend_host(),
                env.auth_host(),
                "{env:?} should share backend and auth host"
            );
        }
    }

    #[test]
    fn dev_environments_share_host_for_auth() {
        for env in [Environment::Dev1, Environment::Dev3, Environment::Dev11] {
            assert_eq!(
                env.backend_host(),
                env.auth_host(),
                "{env:?} should share backend and auth host"
            );
        }
    }

    #[test]
    fn all_nonprod_require_basic_auth() {
        let nonprod = [
            Environment::Preprod,
            Environment::Hotfix,
            Environment::Test1,
            Environment::Test3,
            Environment::Dev1,
            Environment::Dev3,
            Environment::Dev11,
        ];
        for env in nonprod {
            assert!(
                env.requires_basic_auth(),
                "{env:?} should require basic auth"
            );
        }
    }

    // -- AulaClient construction --------------------------------------------

    #[test]
    fn default_config_is_production_v19() {
        let cfg = AulaClientConfig::default();
        assert_eq!(cfg.environment, Environment::Production);
        assert_eq!(cfg.api_version, 19);
    }

    #[test]
    fn client_base_url_production() {
        let client = AulaClient::new().unwrap();
        assert_eq!(client.base_url().as_str(), "https://www.aula.dk/api/v19/");
    }

    #[test]
    fn client_base_url_custom() {
        let client = AulaClient::with_config(AulaClientConfig {
            environment: Environment::Dev1,
            api_version: 20,
        })
        .unwrap();
        assert_eq!(
            client.base_url().as_str(),
            "https://www1-dev1.ncaula.com/api/v20/"
        );
    }

    #[test]
    fn client_environment_accessor() {
        let client = AulaClient::with_config(AulaClientConfig {
            environment: Environment::Test3,
            api_version: 19,
        })
        .unwrap();
        assert_eq!(client.environment(), &Environment::Test3);
    }

    #[test]
    fn client_debug_impl() {
        let client = AulaClient::new().unwrap();
        let dbg = format!("{client:?}");
        assert!(dbg.contains("AulaClient"));
        assert!(dbg.contains("www.aula.dk"));
    }

    // -- URL construction ---------------------------------------------------

    #[test]
    fn url_without_leading_slash() {
        let client = AulaClient::new().unwrap();
        let url = client.url("profiles/keepAlive");
        assert_eq!(
            url.as_str(),
            "https://www.aula.dk/api/v19/profiles/keepAlive"
        );
    }

    #[test]
    fn url_with_leading_slash() {
        let client = AulaClient::new().unwrap();
        let url = client.url("/profiles/keepAlive");
        assert_eq!(
            url.as_str(),
            "https://www.aula.dk/api/v19/profiles/keepAlive"
        );
    }

    // -- CSRF token extraction ----------------------------------------------

    #[test]
    fn csrf_token_none_when_no_cookies() {
        let client = AulaClient::new().unwrap();
        assert!(client.csrf_token().is_none());
    }

    #[test]
    fn csrf_token_extracted_from_cookie_jar() {
        let client = AulaClient::new().unwrap();
        // Simulate the backend setting the CSRF cookie.
        client
            .cookie_jar
            .add_cookie_str("Csrfp-Token=abc123; Path=/", &client.base_url);
        assert_eq!(client.csrf_token().as_deref(), Some("abc123"));
    }

    #[test]
    fn csrf_token_ignores_other_cookies() {
        let client = AulaClient::new().unwrap();
        client
            .cookie_jar
            .add_cookie_str("session=xyz; Path=/", &client.base_url);
        assert!(client.csrf_token().is_none());
    }

    #[test]
    fn csrf_token_with_multiple_cookies() {
        let client = AulaClient::new().unwrap();
        client
            .cookie_jar
            .add_cookie_str("session=xyz; Path=/", &client.base_url);
        client
            .cookie_jar
            .add_cookie_str("Csrfp-Token=tok42; Path=/", &client.base_url);
        client
            .cookie_jar
            .add_cookie_str("other=val; Path=/", &client.base_url);
        assert_eq!(client.csrf_token().as_deref(), Some("tok42"));
    }

    // -- Response handling --------------------------------------------------

    #[tokio::test]
    async fn handle_response_success() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 200,
                "backendErrorCode": 0
            },
            "data": {"greeting": "hello"}
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let result: serde_json::Value = client
            .handle_response(reqwest::Response::from(resp))
            .await
            .unwrap();
        assert_eq!(result["greeting"], "hello");
    }

    #[tokio::test]
    async fn handle_response_session_expired() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 200,
                "backendErrorCode": 0,
                "subCode": 13
            },
            "data": null
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::SessionExpired));
    }

    #[tokio::test]
    async fn handle_response_step_up_required() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 403,
                "backendErrorCode": 0,
                "subCode": 8
            },
            "data": null
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::StepUpRequired));
    }

    #[tokio::test]
    async fn handle_response_unauthorized_http_status() {
        let resp = http::Response::builder()
            .status(401)
            .body(String::new())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::Unauthorized));
    }

    #[tokio::test]
    async fn handle_response_maintenance() {
        let resp = http::Response::builder()
            .status(503)
            .body(String::new())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::Maintenance));
    }

    #[tokio::test]
    async fn handle_response_backend_error() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 200,
                "backendErrorCode": 42,
                "message": "something broke"
            },
            "data": null
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        match err {
            AulaError::Api { message, status } => {
                assert_eq!(message, "something broke");
                assert!(status.is_some());
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_response_invalid_json_on_error_status() {
        let resp = http::Response::builder()
            .status(500)
            .body("not json".to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        match err {
            AulaError::Api { message, .. } => {
                assert!(
                    message.contains("500"),
                    "message should contain status code"
                );
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn handle_response_user_deactivated() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 403,
                "backendErrorCode": 0,
                "subCode": 7
            },
            "data": null
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::UserDeactivated));
    }

    #[tokio::test]
    async fn handle_response_invalid_token() {
        use serde_json::json;

        let body = json!({
            "status": {
                "httpCode": 200,
                "backendErrorCode": 0,
                "subCode": 9
            },
            "data": null
        });

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(body.to_string())
            .unwrap();

        let client = AulaClient::new().unwrap();
        let err = client
            .handle_response::<serde_json::Value>(reqwest::Response::from(resp))
            .await
            .unwrap_err();
        assert!(matches!(err, AulaError::InvalidAccessToken));
    }
}
