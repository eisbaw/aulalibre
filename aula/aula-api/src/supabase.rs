//! Supabase (PostgREST) token storage for hosted deployments.
//!
//! A hosted function has no persistent disk, so tokens live in one row holding
//! the same [`LoginData`] JSON the CLI writes to `tokens.json`:
//!
//! ```sql
//! create table public.aula_tokens (
//!   id         text primary key,          -- 'default'
//!   data       jsonb not null,
//!   updated_at timestamptz not null default now()
//! );
//! alter table public.aula_tokens enable row level security;  -- no policies
//! ```
//!
//! Access uses the project's **secret key**, which Supabase expects on the
//! `apikey` header — it is not a JWT, so `Authorization: Bearer` does not work.
//! The secret key bypasses RLS, so it belongs only in server-side env vars.
//!
//! This type is async on purpose: it is used by the hosted transport, which
//! loads tokens, seeds a [`TokenStore::in_memory`](crate::TokenStore::in_memory)
//! session, and writes back whatever the session ended up with.

use serde::Deserialize;
use serde_json::json;

use crate::auth::LoginData;
use crate::error::AulaError;

/// Table holding the token row.
const TABLE: &str = "aula_tokens";

/// Row id used unless overridden.
const DEFAULT_ROW_ID: &str = "default";

/// How much of an error body to keep in an error message.
const BODY_LIMIT: usize = 200;

/// Supabase-backed token storage.
#[derive(Debug, Clone)]
pub struct SupabaseTokens {
    base_url: String,
    secret_key: String,
    row_id: String,
    http: reqwest::Client,
}

#[derive(Deserialize)]
struct Row {
    data: LoginData,
}

impl SupabaseTokens {
    /// Create a client for `https://<project-ref>.supabase.co`.
    pub fn new(base_url: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            secret_key: secret_key.into(),
            row_id: DEFAULT_ROW_ID.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Build from `SUPABASE_URL` and `SUPABASE_SECRET_KEY`.
    ///
    /// `AULA_TOKEN_ROW` overrides the row id. Returns `None` when the
    /// environment does not configure Supabase, which is how a file-backed
    /// deployment stays unaffected.
    pub fn from_env() -> Option<Self> {
        let base_url = std::env::var("SUPABASE_URL").ok()?;
        let secret_key = std::env::var("SUPABASE_SECRET_KEY").ok()?;
        let mut client = Self::new(base_url, secret_key);
        if let Ok(row_id) = std::env::var("AULA_TOKEN_ROW") {
            client.row_id = row_id;
        }
        Some(client)
    }

    /// The row this client reads and writes.
    pub fn row_id(&self) -> &str {
        &self.row_id
    }

    /// Read the stored tokens. `Ok(None)` when the row does not exist yet.
    pub async fn load(&self) -> crate::Result<Option<LoginData>> {
        let url = format!(
            "{}/rest/v1/{}?id=eq.{}&select=data",
            self.base_url, TABLE, self.row_id
        );

        let response = self
            .http
            .get(url)
            .header("apikey", &self.secret_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            return Err(AulaError::Api {
                message: format!("supabase read failed (HTTP {status}): {}", truncate(&body)),
                status: None,
            });
        }

        let rows: Vec<Row> = serde_json::from_str(&body)?;
        Ok(rows.into_iter().next().map(|row| row.data))
    }

    /// Upsert the tokens into the row.
    pub async fn save(&self, data: &LoginData) -> crate::Result<()> {
        let url = format!("{}/rest/v1/{}", self.base_url, TABLE);
        let body = json!([{ "id": self.row_id, "data": data }]).to_string();

        let response = self
            .http
            .post(url)
            .header("apikey", &self.secret_key)
            .header("Content-Type", "application/json")
            .header("Prefer", "resolution=merge-duplicates,return=minimal")
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AulaError::Api {
                message: format!("supabase write failed (HTTP {status}): {}", truncate(&body)),
                status: None,
            });
        }
        Ok(())
    }
}

fn truncate(body: &str) -> String {
    let collapsed = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() <= BODY_LIMIT {
        collapsed
    } else {
        collapsed.chars().take(BODY_LIMIT).collect::<String>() + "…"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthLevel;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn tokens(access: &str) -> LoginData {
        LoginData {
            access_token: access.to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_in: Some(3600),
            access_token_expiration: Some(1_800_000_000),
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        }
    }

    #[tokio::test]
    async fn load_returns_none_for_an_empty_table() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .and(header("apikey", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let store = SupabaseTokens::new(server.uri(), "secret");
        assert!(store.load().await.expect("load should succeed").is_none());
    }

    #[tokio::test]
    async fn load_parses_the_stored_row() {
        let server = MockServer::start().await;
        let body = serde_json::json!([{ "data": tokens("stored_access") }]).to_string();
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .and(header("apikey", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let store = SupabaseTokens::new(server.uri(), "secret");
        let loaded = store
            .load()
            .await
            .expect("load should succeed")
            .expect("a row");
        assert_eq!(loaded.access_token, "stored_access");
        assert_eq!(loaded.auth_level, AuthLevel::Level2);
    }

    #[tokio::test]
    async fn save_posts_an_upsert_with_the_apikey_header() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/rest/v1/aula_tokens"))
            .and(header("apikey", "secret"))
            .respond_with(ResponseTemplate::new(201))
            .expect(1)
            .mount(&server)
            .await;

        let store = SupabaseTokens::new(server.uri(), "secret");
        store
            .save(&tokens("fresh"))
            .await
            .expect("save should succeed");

        // Assert on what was actually sent, not just that a mock matched.
        let requests = server.received_requests().await.expect("requests recorded");
        let request = requests.first().expect("one request");
        assert_eq!(
            request.headers.get("prefer").and_then(|v| v.to_str().ok()),
            Some("resolution=merge-duplicates,return=minimal"),
        );

        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("body is JSON");
        assert_eq!(body[0]["id"], "default");
        assert_eq!(body[0]["data"]["access_token"], "fresh");
    }

    #[tokio::test]
    async fn http_errors_surface_as_api_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .respond_with(ResponseTemplate::new(401).set_body_string("no api key"))
            .mount(&server)
            .await;

        let store = SupabaseTokens::new(server.uri(), "wrong");
        let err = store
            .load()
            .await
            .expect_err("401 must not look like 'no tokens'");
        let message = err.to_string();
        assert!(message.contains("401"), "message was: {message}");
    }

    #[test]
    fn from_env_is_none_without_configuration() {
        // The test environment does not set these; a file-backed deployment
        // must therefore stay unaffected by this module existing.
        assert!(SupabaseTokens::from_env().is_none());
    }
}
