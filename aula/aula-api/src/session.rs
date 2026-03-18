//! Session management with automatic token refresh.
//!
//! Wraps [`AulaClient`] to provide:
//!
//! - **Automatic token refresh**: Checks token expiry before each API call
//!   and refreshes proactively using `Conf.BufferOnTokenExpiration`.
//! - **401 retry**: On `Unauthorized` or `InvalidAccessToken` errors, attempts
//!   one token refresh and retries the request.
//! - **Session keep-alive**: Periodic `POST /profiles/keepAlive` to extend the
//!   backend session (mirrors `SessionPromptManager` from the APK).
//! - **Logout**: Clears stored tokens and hits the logout endpoint.
//!
//! See `auth_flow.md` Sections 6-7, 9.

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::auth::{self, AuthLevel, LoginData, OidcEndpoints};
use crate::client::AulaClient;
use crate::error::AulaError;
use crate::token_store::TokenStore;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default buffer (seconds) before token expiration to trigger proactive
/// refresh. Mirrors `Conf.BufferOnTokenExpiration` from the APK.
const DEFAULT_EXPIRY_BUFFER_SECS: u64 = 60;

/// Logout endpoint path (relative to auth backend, not API base).
const LOGOUT_PATH: &str = "/auth/logout.php";

// ---------------------------------------------------------------------------
// SessionConfig
// ---------------------------------------------------------------------------

/// Configuration for [`Session`].
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Seconds before actual token expiry to trigger refresh.
    /// Default: 60.
    pub expiry_buffer_secs: u64,

    /// Authentication level for token refresh.
    /// Default: [`AuthLevel::Level2`].
    pub auth_level: AuthLevel,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            expiry_buffer_secs: DEFAULT_EXPIRY_BUFFER_SECS,
            auth_level: AuthLevel::Level2,
        }
    }
}

// ---------------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------------

/// A managed session that owns an [`AulaClient`] and [`TokenStore`], providing
/// automatic token lifecycle management.
///
/// # Token flow
///
/// ```text
/// API call requested
///   -> check token expiry (with buffer)
///     -> if expired: refresh token via OIDC endpoint
///       -> persist new tokens
///   -> execute API call
///     -> if 401/InvalidAccessToken: refresh and retry once
/// ```
pub struct Session {
    client: AulaClient,
    store: TokenStore,
    config: SessionConfig,
    endpoints: OidcEndpoints,
    /// Cached login data (mirrors what's on disk).
    login_data: Option<LoginData>,
}

impl Session {
    /// Create a new session with the given client, token store, and config.
    ///
    /// Attempts to load persisted tokens from the store. If tokens exist and
    /// are valid, the session is immediately usable for API calls.
    pub fn new(
        client: AulaClient,
        store: TokenStore,
        config: SessionConfig,
    ) -> crate::Result<Self> {
        let endpoints = OidcEndpoints::for_environment(client.environment());
        let login_data = store.load()?;

        Ok(Self {
            client,
            store,
            config,
            endpoints,
            login_data,
        })
    }

    /// The underlying [`AulaClient`].
    pub fn client(&self) -> &AulaClient {
        &self.client
    }

    /// The current login data, if any.
    pub fn login_data(&self) -> Option<&LoginData> {
        self.login_data.as_ref()
    }

    /// Whether the session has valid (non-expired) tokens.
    pub fn has_valid_tokens(&self) -> bool {
        self.login_data
            .as_ref()
            .map(|ld| !ld.is_expired_with_buffer(self.config.expiry_buffer_secs))
            .unwrap_or(false)
    }

    /// Set login data (e.g., after initial OIDC login flow) and persist it.
    pub fn set_login_data(&mut self, data: LoginData) -> crate::Result<()> {
        self.store.save(&data)?;
        self.login_data = Some(data);
        Ok(())
    }

    // -- Token refresh ------------------------------------------------------

    /// Ensure the access token is valid, refreshing if necessary.
    ///
    /// Returns `Ok(())` if tokens are valid (possibly after refresh).
    /// Returns `Err` if no tokens exist (initial login needed) or refresh fails.
    pub async fn ensure_valid_token(&mut self) -> crate::Result<()> {
        let login_data = self.login_data.as_ref().ok_or_else(|| AulaError::Auth {
            error: "no_tokens".to_string(),
            description: Some("no persisted tokens; initial login required".to_string()),
        })?;

        if !login_data.is_expired_with_buffer(self.config.expiry_buffer_secs) {
            return Ok(());
        }

        self.refresh_token().await
    }

    /// Refresh the access token using the stored refresh token.
    ///
    /// On success, persists the new tokens to disk and updates the in-memory
    /// cache. If the refresh token itself is missing, returns an auth error.
    pub async fn refresh_token(&mut self) -> crate::Result<()> {
        let login_data = self.login_data.as_ref().ok_or_else(|| AulaError::Auth {
            error: "no_tokens".to_string(),
            description: Some("cannot refresh: no persisted tokens".to_string()),
        })?;

        let refresh_tok = login_data
            .refresh_token
            .as_deref()
            .ok_or_else(|| AulaError::Auth {
                error: "no_refresh_token".to_string(),
                description: Some("stored tokens have no refresh token".to_string()),
            })?;

        let token_response = auth::refresh_token(
            self.client.http(),
            &self.endpoints,
            self.config.auth_level,
            refresh_tok,
        )
        .await?;

        let new_data = LoginData::from_token_response(token_response, self.config.auth_level);
        self.store.save(&new_data)?;
        self.login_data = Some(new_data);

        Ok(())
    }

    // -- API call wrappers with auto-refresh --------------------------------

    /// Proactively refresh if token is about to expire, then execute the
    /// request. On 401/InvalidAccessToken, refresh and retry once.
    async fn pre_refresh(&mut self) {
        if self.login_data.is_some() {
            let _ = self.ensure_valid_token().await;
        }
    }

    /// Execute a GET request with automatic token refresh and 401 retry.
    pub async fn get<T: DeserializeOwned>(&mut self, path: &str) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.get(path).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.get(path).await
            }
            other => other,
        }
    }

    /// Execute a POST request with automatic token refresh and 401 retry.
    pub async fn post<T: DeserializeOwned, B: Serialize + Sync>(
        &mut self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.post(path, body).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.post(path, body).await
            }
            other => other,
        }
    }

    /// Execute a POST request (no body) with automatic token refresh and 401 retry.
    pub async fn post_empty<T: DeserializeOwned>(&mut self, path: &str) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.post_empty(path).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.post_empty(path).await
            }
            other => other,
        }
    }

    /// Execute a PUT request with automatic token refresh and 401 retry.
    pub async fn put<T: DeserializeOwned, B: Serialize + Sync>(
        &mut self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.put(path, body).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.put(path, body).await
            }
            other => other,
        }
    }

    /// Execute a DELETE request with automatic token refresh and 401 retry.
    pub async fn delete<T: DeserializeOwned>(&mut self, path: &str) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.delete(path).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.delete(path).await
            }
            other => other,
        }
    }

    /// Execute a DELETE request with a JSON body and automatic token refresh.
    pub async fn delete_with_body<T: DeserializeOwned, B: Serialize + Sync>(
        &mut self,
        path: &str,
        body: &B,
    ) -> crate::Result<T> {
        self.pre_refresh().await;
        match self.client.delete_with_body(path, body).await {
            Err(AulaError::Unauthorized | AulaError::InvalidAccessToken) => {
                self.refresh_token().await?;
                self.client.delete_with_body(path, body).await
            }
            other => other,
        }
    }

    // -- Session keep-alive -------------------------------------------------

    /// Send a keep-alive ping to extend the backend session.
    ///
    /// Mirrors `SessionPromptManager`'s periodic call to `POST /profiles/keepAlive`.
    pub async fn keep_alive(&self) -> crate::Result<()> {
        self.client.keep_alive().await
    }

    // -- Logout -------------------------------------------------------------

    /// Log out: clear stored tokens and hit the OIDC logout endpoint.
    ///
    /// Mirrors `AuthenticationManager.OpenLogoutAndReturnToAppWithUniversalLink()`
    /// and `AuthenticationManager.ResetData()` from the APK.
    pub async fn logout(&mut self) -> crate::Result<()> {
        // Clear persisted tokens.
        self.store.clear()?;
        self.login_data = None;

        // Hit the logout endpoint on the auth backend.
        let logout_url = format!(
            "https://{}{}",
            self.client.environment().auth_host(),
            LOGOUT_PATH
        );

        // Best-effort: the logout endpoint may return HTML or redirect.
        // We don't parse the response; clearing tokens is the primary action.
        let _ = self.client.http().get(&logout_url).send().await;

        Ok(())
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("client", &self.client)
            .field("has_tokens", &self.login_data.is_some())
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthLevel;
    use crate::client::{AulaClientConfig, Environment};

    fn test_login_data(expired: bool) -> LoginData {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let expiration = if expired {
            Some(now.saturating_sub(100)) // 100 seconds in the past
        } else {
            Some(now + 3600) // 1 hour from now
        };

        LoginData {
            access_token: "test_at".to_string(),
            refresh_token: Some("test_rt".to_string()),
            expires_in: Some(3600),
            access_token_expiration: expiration,
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        }
    }

    fn test_dir(suffix: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("aula_session_test_{}_{suffix}", std::process::id()))
    }

    #[test]
    fn session_config_defaults() {
        let cfg = SessionConfig::default();
        assert_eq!(cfg.expiry_buffer_secs, 60);
        assert_eq!(cfg.auth_level, AuthLevel::Level2);
    }

    #[test]
    fn session_new_no_stored_tokens() {
        let dir = test_dir("no_tokens");
        let store = TokenStore::new(&dir);
        let client = AulaClient::new().unwrap();
        let session = Session::new(client, store, SessionConfig::default()).unwrap();

        assert!(session.login_data().is_none());
        assert!(!session.has_valid_tokens());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn session_new_loads_stored_tokens() {
        let dir = test_dir("with_tokens");
        let store = TokenStore::new(&dir);
        let data = test_login_data(false);
        store.save(&data).unwrap();

        let client = AulaClient::new().unwrap();
        let session = Session::new(client, store, SessionConfig::default()).unwrap();

        assert!(session.login_data().is_some());
        assert!(session.has_valid_tokens());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn session_has_valid_tokens_false_when_expired() {
        let dir = test_dir("expired");
        let store = TokenStore::new(&dir);
        let data = test_login_data(true);
        store.save(&data).unwrap();

        let client = AulaClient::new().unwrap();
        let session = Session::new(client, store, SessionConfig::default()).unwrap();

        assert!(session.login_data().is_some());
        assert!(!session.has_valid_tokens());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn session_set_login_data_persists() {
        let dir = test_dir("set_data");
        let store = TokenStore::new(&dir);
        let client = AulaClient::new().unwrap();
        let mut session = Session::new(client, store, SessionConfig::default()).unwrap();

        assert!(session.login_data().is_none());

        let data = test_login_data(false);
        session.set_login_data(data).unwrap();

        assert!(session.login_data().is_some());
        assert!(session.has_valid_tokens());

        // Verify it's actually on disk.
        let store2 = TokenStore::new(&dir);
        let loaded = store2.load().unwrap();
        assert!(loaded.is_some());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn session_logout_clears_tokens() {
        let dir = test_dir("logout");
        let store = TokenStore::new(&dir);
        let data = test_login_data(false);
        store.save(&data).unwrap();

        let client = AulaClient::new().unwrap();
        let mut session = Session::new(client, store, SessionConfig::default()).unwrap();

        assert!(session.has_valid_tokens());

        // Logout will try to hit the remote endpoint (which will fail in tests),
        // but it should still clear local tokens.
        session.logout().await.unwrap();

        assert!(session.login_data().is_none());
        assert!(!session.has_valid_tokens());
        assert!(!TokenStore::new(&dir).exists());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn ensure_valid_token_errors_when_no_tokens() {
        let dir = test_dir("no_tok_ensure");
        let store = TokenStore::new(&dir);
        let client = AulaClient::new().unwrap();
        let mut session = Session::new(client, store, SessionConfig::default()).unwrap();

        let err = session.ensure_valid_token().await.unwrap_err();
        match err {
            AulaError::Auth { error, .. } => assert_eq!(error, "no_tokens"),
            other => panic!("expected Auth error, got {other:?}"),
        }

        std::fs::remove_dir_all(&dir).ok();
    }

    #[tokio::test]
    async fn ensure_valid_token_ok_when_not_expired() {
        let dir = test_dir("valid_ensure");
        let store = TokenStore::new(&dir);
        let data = test_login_data(false);
        store.save(&data).unwrap();

        let client = AulaClient::new().unwrap();
        let mut session = Session::new(client, store, SessionConfig::default()).unwrap();

        // Should succeed without network call since token is valid.
        session.ensure_valid_token().await.unwrap();

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn session_debug_impl() {
        let dir = test_dir("debug");
        let store = TokenStore::new(&dir);
        let client = AulaClient::new().unwrap();
        let session = Session::new(client, store, SessionConfig::default()).unwrap();

        let dbg = format!("{session:?}");
        assert!(dbg.contains("Session"));
        assert!(dbg.contains("has_tokens"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn session_with_custom_config() {
        let dir = test_dir("custom_cfg");
        let store = TokenStore::new(&dir);
        let client = AulaClient::with_config(AulaClientConfig {
            environment: Environment::Preprod,
            api_version: 19,
        })
        .unwrap();

        let config = SessionConfig {
            expiry_buffer_secs: 120,
            auth_level: AuthLevel::Level3,
        };
        let session = Session::new(client, store, config).unwrap();

        assert_eq!(session.client().environment(), &Environment::Preprod);

        std::fs::remove_dir_all(&dir).ok();
    }
}
