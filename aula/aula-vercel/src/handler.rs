//! Maps a Vercel HTTP request onto the MCP dispatcher.
//!
//! Request shape: `POST /api/mcp?secret=<AULA_MCP_SECRET>`.
//!
//! The secret travels in the query string because Claude connectors have no
//! field for a custom header. It must be URL-safe (no percent-encoding is
//! applied). A mismatch answers 404 rather than 401 so the endpoint is not an
//! oracle for valid secrets.
//!
//! Tokens come from Supabase per invocation: a function has no persistent
//! disk, so the row is the only durable place. The session runs on a
//! [`TokenStore::in_memory`] seeded with those tokens, and anything the
//! session refreshes is written back before responding.

use aula_api::{SupabaseTokens, TokenStore};
use aula_mcp::{rpc, tools};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use vercel_runtime::{Request, Response, ResponseBody};

/// Env var holding the secret every MCP call must present.
pub const SECRET_ENV: &str = "AULA_MCP_SECRET";

/// Query parameter carrying the secret.
pub const SECRET_PARAM: &str = "secret";

/// Header names Claude connectors can send without asking Anthropic for
/// approval. Per the connector docs `authorization`, `x-api-key` and
/// `x-auth-token` work for every connector; any other name must be approved by
/// Anthropic first and is rejected in the add-connector dialog.
const SECRET_HEADERS: [&str; 2] = ["x-api-key", "authorization"];

/// Hosted configuration: the shared secret and the token store.
pub struct HostState {
    secret: String,
    tokens: SupabaseTokens,
}

impl HostState {
    /// Read configuration from the environment, `None` when incomplete.
    pub fn from_env() -> Option<Self> {
        let secret = std::env::var(SECRET_ENV)
            .ok()
            .filter(|value| !value.is_empty())?;
        let tokens = SupabaseTokens::from_env()?;
        Some(Self { secret, tokens })
    }
}

/// How a request should be answered, decided without any I/O.
#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Handle the JSON-RPC body.
    Dispatch,
    /// Not a POST (connectors may probe with GET for SSE).
    MethodNotAllowed,
    /// Missing or wrong secret.
    NotFound,
    /// The deployment is missing its secret or token store.
    Misconfigured,
}

/// Decide what to do with a request.
pub fn verdict(method: &str, provided: Option<&str>, expected: Option<&str>) -> Verdict {
    if method != "POST" {
        return Verdict::MethodNotAllowed;
    }
    let Some(expected) = expected else {
        return Verdict::Misconfigured;
    };
    match provided {
        Some(provided) if secret_matches(provided, expected) => Verdict::Dispatch,
        _ => Verdict::NotFound,
    }
}

/// Compare secrets without an early exit on the first differing byte.
fn secret_matches(provided: &str, expected: &str) -> bool {
    if provided.len() != expected.len() {
        return false;
    }
    provided
        .bytes()
        .zip(expected.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}

/// Extract the secret from an approved header, falling back to the query string.
///
/// Headers are preferred: a query parameter ends up in access logs and in the
/// stored connector URL.
fn secret_from(headers: &http::HeaderMap, uri: &str) -> Option<String> {
    for name in SECRET_HEADERS {
        let Some(value) = headers.get(name).and_then(|value| value.to_str().ok()) else {
            continue;
        };
        // `authorization` may carry a scheme; `x-api-key` is the bare secret.
        let token = value
            .strip_prefix("Bearer ")
            .or_else(|| value.strip_prefix("bearer "))
            .unwrap_or(value)
            .trim();
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }
    secret_from_query(uri)
}

/// Extract `secret` from a URI's query string.
fn secret_from_query(uri: &str) -> Option<String> {
    let query = uri.split_once('?')?.1;
    query.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        (key == SECRET_PARAM && !value.is_empty()).then(|| value.to_string())
    })
}

/// Entry point used by the Vercel function.
pub async fn respond(request: Request) -> Response<ResponseBody> {
    let method = request.method().as_str().to_string();
    let secret = secret_from(request.headers(), &request.uri().to_string());
    let state = HostState::from_env();
    let expected = state.as_ref().map(|state| state.secret.as_str());

    match verdict(&method, secret.as_deref(), expected) {
        Verdict::MethodNotAllowed => method_not_allowed(),
        Verdict::NotFound => text_response(404, "not found"),
        Verdict::Misconfigured => text_response(500, "server is not configured"),
        Verdict::Dispatch => {
            let state = state.expect("dispatch requires configuration");
            let body = match request.into_body().collect().await {
                Ok(collected) => String::from_utf8_lossy(&collected.to_bytes()).into_owned(),
                Err(e) => return text_response(400, &format!("unreadable body: {e}")),
            };
            dispatch(&state, &body).await
        }
    }
}

/// Load tokens, run the dispatcher, persist a refresh if one happened.
async fn dispatch(state: &HostState, body: &str) -> Response<ResponseBody> {
    let loaded = match state.tokens.load().await {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("[aula-vercel] token store read failed: {e}");
            return json_response(500, &json!({"error": format!("token store: {e}")}));
        }
    };

    // The session works on an in-memory copy; we keep a clone to see what it did.
    let store = TokenStore::in_memory(loaded.clone());
    let server = tools::Server::with_store(store.clone());

    let Some(response) = rpc::handle(&server, body).await else {
        // A notification: nothing to say, but a refresh may still have happened.
        persist_if_changed(state, &loaded, &store).await;
        // The MCP spec requires 202 Accepted with no body for notifications and
        // responses; 204 makes some clients treat the exchange as failed.
        return Response::builder()
            .status(202)
            .body(ResponseBody::from(""))
            .unwrap_or_else(|_| Response::new(ResponseBody::from("")));
    };

    persist_if_changed(state, &loaded, &store).await;
    json_response(200, &response)
}

/// Write the tokens back when the session refreshed them.
async fn persist_if_changed(
    state: &HostState,
    loaded: &Option<aula_api::auth::LoginData>,
    store: &TokenStore,
) {
    let Ok(Some(current)) = store.load() else {
        return;
    };
    let changed = loaded
        .as_ref()
        .map(|before| before.access_token != current.access_token)
        .unwrap_or(true);
    if !changed {
        return;
    }
    if let Err(e) = state.tokens.save(&current).await {
        // A lost write costs one extra refresh next time, not a failure.
        eprintln!("[aula-vercel] could not persist refreshed tokens: {e}");
    }
}

fn json_response(status: u16, value: &Value) -> Response<ResponseBody> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(ResponseBody::from(value.to_string()))
        .unwrap_or_else(|_| Response::new(ResponseBody::from("{}")))
}

fn method_not_allowed() -> Response<ResponseBody> {
    // The MCP spec requires a server without an SSE stream to answer GET with
    // 405; the Allow header tells clients what it does accept.
    Response::builder()
        .status(405)
        .header("allow", "POST")
        .header("content-type", "text/plain; charset=utf-8")
        .body(ResponseBody::from(
            "method not allowed; use POST (MCP over Streamable HTTP)",
        ))
        .unwrap_or_else(|_| Response::new(ResponseBody::from("")))
}

fn text_response(status: u16, message: &str) -> Response<ResponseBody> {
    Response::builder()
        .status(status)
        .header("content-type", "text/plain; charset=utf-8")
        .body(ResponseBody::from(message.to_string()))
        .unwrap_or_else(|_| Response::new(ResponseBody::from("")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_post_is_dispatched() {
        assert_eq!(
            verdict("GET", Some("s"), Some("s")),
            Verdict::MethodNotAllowed
        );
        assert_eq!(
            verdict("DELETE", Some("s"), Some("s")),
            Verdict::MethodNotAllowed
        );
        assert_eq!(verdict("POST", Some("s"), Some("s")), Verdict::Dispatch);
    }

    #[test]
    fn wrong_or_missing_secret_is_not_found() {
        assert_eq!(
            verdict("POST", Some("nope"), Some("s3cret")),
            Verdict::NotFound
        );
        assert_eq!(verdict("POST", None, Some("s3cret")), Verdict::NotFound);
        assert_eq!(verdict("POST", Some(""), Some("s3cret")), Verdict::NotFound);
    }

    #[test]
    fn missing_configuration_is_reported_separately() {
        assert_eq!(verdict("POST", Some("s"), None), Verdict::Misconfigured);
    }

    #[test]
    fn secret_comparison_handles_length_and_content() {
        assert!(secret_matches("abc123", "abc123"));
        assert!(!secret_matches("abc123", "abc124"));
        assert!(!secret_matches("abc", "abc123"));
    }

    #[test]
    fn secret_is_read_from_approved_headers() {
        let mut headers = http::HeaderMap::new();
        headers.insert("x-api-key", "from-header".parse().unwrap());
        assert_eq!(
            secret_from(&headers, "/api/mcp"),
            Some("from-header".to_string())
        );

        let mut headers = http::HeaderMap::new();
        headers.insert("authorization", "Bearer from-bearer".parse().unwrap());
        assert_eq!(
            secret_from(&headers, "/api/mcp"),
            Some("from-bearer".to_string())
        );
    }

    #[test]
    fn header_secret_wins_over_the_query_string() {
        let mut headers = http::HeaderMap::new();
        headers.insert("x-api-key", "from-header".parse().unwrap());
        assert_eq!(
            secret_from(&headers, "/api/mcp?secret=from-query"),
            Some("from-header".to_string())
        );
    }

    #[test]
    fn query_string_is_the_fallback_when_no_header_is_sent() {
        let headers = http::HeaderMap::new();
        assert_eq!(
            secret_from(&headers, "/api/mcp?secret=from-query"),
            Some("from-query".to_string())
        );
        assert_eq!(secret_from(&headers, "/api/mcp"), None);
    }

    #[test]
    fn secret_is_read_from_the_query_string() {
        assert_eq!(
            secret_from_query("/api/mcp?secret=abc123"),
            Some("abc123".to_string())
        );
        assert_eq!(
            secret_from_query("/api/mcp?foo=1&secret=abc123&bar=2"),
            Some("abc123".to_string())
        );
        assert_eq!(secret_from_query("/api/mcp?secret="), None);
        assert_eq!(secret_from_query("/api/mcp"), None);
    }

    /// Live check of the hosted path against the real Supabase project and the
    /// real Aula API (R3/R5 verification).
    ///
    /// ```text
    /// set -a; . secrets/supabase.env; set +a
    /// cargo test -p aula-vercel -- --ignored --nocapture
    /// ```
    #[tokio::test]
    #[ignore = "needs SUPABASE_URL + SUPABASE_SECRET_KEY and a seeded token row"]
    async fn live_hosted_path_answers_whoami() {
        let Some(tokens) = SupabaseTokens::from_env() else {
            eprintln!("skipping: SUPABASE_URL / SUPABASE_SECRET_KEY not set");
            return;
        };
        let state = HostState {
            secret: "live-check".to_string(),
            tokens,
        };

        let response = dispatch(
            &state,
            r#"{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"aula_whoami","arguments":{}}}"#,
        )
        .await;
        assert_eq!(response.status(), 200);

        let body = response
            .into_body()
            .collect()
            .await
            .expect("body readable")
            .to_bytes();
        let payload: Value = serde_json::from_slice(&body).expect("JSON response");
        let text = payload["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or_default();
        assert_eq!(payload["result"]["isError"], false, "whoami failed: {text}");
        assert!(
            text.contains("institution_profile_ids"),
            "unexpected whoami payload: {text}"
        );
    }

    #[test]
    fn non_post_advertises_the_allowed_method() {
        let response = method_not_allowed();
        assert_eq!(response.status(), 405);
        assert_eq!(
            response
                .headers()
                .get("allow")
                .and_then(|v| v.to_str().ok()),
            Some("POST")
        );
    }

    /// Notifications must be acknowledged with 202 Accepted and no body, per
    /// the Streamable HTTP transport rules.
    #[tokio::test]
    async fn notifications_are_acknowledged_with_202() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let supabase = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&supabase)
            .await;

        let state = HostState {
            secret: "s3cret".to_string(),
            tokens: SupabaseTokens::new(supabase.uri(), "key"),
        };

        let response = dispatch(
            &state,
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        )
        .await;
        assert_eq!(response.status(), 202);
    }

    #[test]
    fn json_responses_declare_their_content_type() {
        let response = json_response(200, &json!({"ok": true}));
        assert_eq!(response.status(), 200);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok()),
            Some("application/json")
        );
    }

    /// A stored-token fixture matching what `aula auth login` writes.
    fn stored_tokens() -> Value {
        json!({
            "access_token": "stored_access",
            "refresh_token": "stored_refresh",
            "expires_in": 3600,
            "access_token_expiration": 1_800_000_000_u64,
            "auth_level": "level2",
            "error": null,
            "error_description": null
        })
    }

    /// End-to-end over the hosted path with wiremock standing in for Supabase:
    /// load tokens, dispatch a real MCP message, answer JSON. No Aula call is
    /// made because `tools/list` never touches the network.
    #[tokio::test]
    async fn dispatch_answers_tools_list_using_tokens_from_the_store() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let supabase = MockServer::start().await;
        let row = json!([{ "data": stored_tokens() }]).to_string();
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .respond_with(ResponseTemplate::new(200).set_body_string(row))
            .expect(1)
            .mount(&supabase)
            .await;

        let state = HostState {
            secret: "s3cret".to_string(),
            tokens: SupabaseTokens::new(supabase.uri(), "key"),
        };

        let response = dispatch(&state, r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#).await;
        assert_eq!(response.status(), 200);

        let body = response
            .into_body()
            .collect()
            .await
            .expect("body readable")
            .to_bytes();
        let payload: Value = serde_json::from_slice(&body).expect("JSON response");
        assert_eq!(payload["id"], 1);
        assert_eq!(
            payload["result"]["tools"].as_array().map(Vec::len),
            Some(12),
            "all tools must be listed"
        );
    }

    /// A missing row must not crash the function: the dispatcher answers with
    /// an actionable tool error instead of a bare failure.
    #[tokio::test]
    async fn dispatch_without_stored_tokens_reports_not_logged_in() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let supabase = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/rest/v1/aula_tokens"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&supabase)
            .await;

        let state = HostState {
            secret: "s3cret".to_string(),
            tokens: SupabaseTokens::new(supabase.uri(), "key"),
        };

        let response = dispatch(
            &state,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"aula_whoami"}}"#,
        )
        .await;
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body readable")
            .to_bytes();
        let payload: Value = serde_json::from_slice(&body).expect("JSON response");
        assert_eq!(payload["result"]["isError"], true);
        let text = payload["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or_default();
        assert!(text.contains("not logged in"), "message was: {text}");
    }
}
