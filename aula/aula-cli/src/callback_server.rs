//! Minimal local HTTP server that captures the OAuth redirect callback.
//!
//! Starts a TCP listener on `127.0.0.1` with an OS-assigned port, waits for
//! a single GET request to `/callback?code=...&state=...`, extracts the query
//! parameters, and returns them to the caller.
//!
//! The server responds with a small HTML page telling the user they can close
//! the browser tab. It shuts down after receiving one callback request or when
//! the timeout expires.

use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// The result of waiting for the OAuth callback.
#[derive(Debug, Clone)]
pub struct CallbackResult {
    /// The full request URI path + query (e.g. `/callback?code=abc&state=xyz`).
    pub request_uri: String,
}

/// Error type for the callback server.
#[derive(Debug)]
pub enum CallbackError {
    /// Timed out waiting for the callback.
    Timeout,
    /// Failed to bind the TCP listener.
    Bind(std::io::Error),
    /// Failed to accept a connection or read the request.
    Io(std::io::Error),
    /// The HTTP request could not be parsed.
    #[allow(dead_code)]
    BadRequest(String),
}

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "timed out waiting for OAuth callback"),
            Self::Bind(e) => write!(f, "failed to bind callback listener: {e}"),
            Self::Io(e) => write!(f, "callback server I/O error: {e}"),
            Self::BadRequest(msg) => write!(f, "bad callback request: {msg}"),
        }
    }
}

/// HTML page shown to the user after the callback is received.
const SUCCESS_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><title>Aula CLI - Login</title></head>
<body style="font-family: sans-serif; text-align: center; padding: 40px;">
<h1>Login successful</h1>
<p>You can close this browser tab and return to the terminal.</p>
</body>
</html>"#;

/// HTML page shown when the request path is not `/callback`.
const WAITING_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><title>Aula CLI - Waiting</title></head>
<body style="font-family: sans-serif; text-align: center; padding: 40px;">
<h1>Aula CLI</h1>
<p>Waiting for authentication callback...</p>
<p>Please complete login in the other browser tab.</p>
</body>
</html>"#;

/// Start a local callback server and wait for the OAuth redirect.
///
/// Binds to `127.0.0.1:0` (OS-assigned port) and returns the port number
/// along with a future that resolves when the callback is received.
///
/// # Returns
///
/// A tuple of `(port, future)` where `port` is the local port the server is
/// listening on, and the future resolves to the callback result or error.
pub async fn start_callback_server(
    timeout: Duration,
) -> Result<
    (
        u16,
        tokio::task::JoinHandle<Result<CallbackResult, CallbackError>>,
    ),
    CallbackError,
> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(CallbackError::Bind)?;

    let port = listener.local_addr().map_err(CallbackError::Io)?.port();

    let handle = tokio::spawn(async move { run_callback_server(listener, timeout).await });

    Ok((port, handle))
}

/// Internal: run the callback server loop.
async fn run_callback_server(
    listener: TcpListener,
    timeout: Duration,
) -> Result<CallbackResult, CallbackError> {
    let result = tokio::time::timeout(timeout, accept_callback(&listener)).await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err(CallbackError::Timeout),
    }
}

/// Accept connections until we get a request to `/callback`.
async fn accept_callback(listener: &TcpListener) -> Result<CallbackResult, CallbackError> {
    loop {
        let (mut stream, _addr) = listener.accept().await.map_err(CallbackError::Io)?;

        // Read the HTTP request (we only need the first line).
        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf).await.map_err(CallbackError::Io)?;
        if n == 0 {
            continue;
        }

        let request_str = String::from_utf8_lossy(&buf[..n]);

        // Parse the request line: "GET /path?query HTTP/1.1"
        let request_line = match request_str.lines().next() {
            Some(line) => line,
            None => continue,
        };

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 2 {
            send_response(&mut stream, 400, "Bad Request").await;
            continue;
        }

        let method = parts[0];
        let uri = parts[1];

        // Only handle GET requests.
        if method != "GET" {
            send_response(&mut stream, 405, "Method Not Allowed").await;
            continue;
        }

        // Check if this is the callback path.
        if uri.starts_with("/callback") {
            // Send success page.
            send_html_response(&mut stream, 200, SUCCESS_HTML).await;

            return Ok(CallbackResult {
                request_uri: uri.to_string(),
            });
        }

        // Favicon and other browser requests -- ignore silently.
        if uri == "/favicon.ico" {
            send_response(&mut stream, 204, "").await;
            continue;
        }

        // Any other path: show the waiting page.
        send_html_response(&mut stream, 200, WAITING_HTML).await;
    }
}

/// Send a plain-text HTTP response.
async fn send_response(stream: &mut tokio::net::TcpStream, status: u16, body: &str) {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        405 => "Method Not Allowed",
        _ => "Unknown",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.flush().await;
}

/// Send an HTML HTTP response.
async fn send_html_response(stream: &mut tokio::net::TcpStream, status: u16, html: &str) {
    let reason = match status {
        200 => "OK",
        _ => "Unknown",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {html}",
        html.len()
    );
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.flush().await;
}

/// Build the localhost callback redirect URI for a given port.
pub fn localhost_redirect_uri(port: u16) -> String {
    format!("http://localhost:{port}/callback")
}

/// Parse query parameters from a callback URI like `/callback?code=abc&state=xyz`.
///
/// Returns an iterator over `(key, value)` pairs.
pub fn parse_callback_query(request_uri: &str) -> Vec<(String, String)> {
    let query = match request_uri.split_once('?') {
        Some((_, q)) => q,
        None => return Vec::new(),
    };

    url::form_urlencoded::parse(query.as_bytes())
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}

/// Extract the authorization code and optional state from a callback request URI.
pub fn extract_code_from_callback(request_uri: &str) -> Result<(String, Option<String>), String> {
    let params = parse_callback_query(request_uri);

    // Check for OAuth error in the callback.
    let error = params.iter().find(|(k, _)| k == "error");
    if let Some((_, err_code)) = error {
        let desc = params
            .iter()
            .find(|(k, _)| k == "error_description")
            .map(|(_, v)| v.clone());
        return Err(format!(
            "OAuth error: {err_code}{}",
            desc.map(|d| format!(": {d}")).unwrap_or_default()
        ));
    }

    let code = params
        .iter()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.clone())
        .ok_or_else(|| "callback is missing the 'code' parameter".to_string())?;

    let state = params
        .iter()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.clone());

    Ok((code, state))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localhost_redirect_uri_format() {
        assert_eq!(
            localhost_redirect_uri(8080),
            "http://localhost:8080/callback"
        );
        assert_eq!(
            localhost_redirect_uri(12345),
            "http://localhost:12345/callback"
        );
    }

    #[test]
    fn parse_callback_query_with_code_and_state() {
        let params = parse_callback_query("/callback?code=abc123&state=xyz");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0], ("code".to_string(), "abc123".to_string()));
        assert_eq!(params[1], ("state".to_string(), "xyz".to_string()));
    }

    #[test]
    fn parse_callback_query_no_query_string() {
        let params = parse_callback_query("/callback");
        assert!(params.is_empty());
    }

    #[test]
    fn parse_callback_query_url_encoded() {
        let params = parse_callback_query("/callback?code=a%20b&state=c%2Bd");
        assert_eq!(params[0].1, "a b");
        assert_eq!(params[1].1, "c+d");
    }

    #[test]
    fn extract_code_success() {
        let (code, state) =
            extract_code_from_callback("/callback?code=mycode&state=mystate").unwrap();
        assert_eq!(code, "mycode");
        assert_eq!(state.as_deref(), Some("mystate"));
    }

    #[test]
    fn extract_code_no_state() {
        let (code, state) = extract_code_from_callback("/callback?code=mycode").unwrap();
        assert_eq!(code, "mycode");
        assert!(state.is_none());
    }

    #[test]
    fn extract_code_missing() {
        let err = extract_code_from_callback("/callback?state=abc").unwrap_err();
        assert!(err.contains("missing"));
    }

    #[test]
    fn extract_code_oauth_error() {
        let err = extract_code_from_callback(
            "/callback?error=access_denied&error_description=User+cancelled",
        )
        .unwrap_err();
        assert!(err.contains("access_denied"));
        assert!(err.contains("User cancelled"));
    }

    #[tokio::test]
    async fn callback_server_receives_code() {
        let (port, handle) = start_callback_server(Duration::from_secs(5)).await.unwrap();

        // Simulate a browser redirect by making an HTTP request to the callback.
        let url = format!("http://127.0.0.1:{port}/callback?code=testcode&state=teststate");
        let resp = reqwest::get(&url).await.unwrap();
        assert!(resp.status().is_success());

        let result = handle.await.unwrap().unwrap();
        assert!(result.request_uri.contains("code=testcode"));
        assert!(result.request_uri.contains("state=teststate"));
    }

    #[tokio::test]
    async fn callback_server_timeout() {
        let (_port, handle) = start_callback_server(Duration::from_millis(100))
            .await
            .unwrap();

        // Don't send any request -- should time out.
        let result = handle.await.unwrap();
        assert!(matches!(result, Err(CallbackError::Timeout)));
    }

    #[tokio::test]
    async fn callback_server_ignores_non_callback_requests() {
        let (port, handle) = start_callback_server(Duration::from_secs(5)).await.unwrap();

        // Request the root -- should get the waiting page, server keeps running.
        let url = format!("http://127.0.0.1:{port}/");
        let resp = reqwest::get(&url).await.unwrap();
        assert!(resp.status().is_success());
        let body = resp.text().await.unwrap();
        assert!(body.contains("Waiting"));

        // Now send the actual callback.
        let url = format!("http://127.0.0.1:{port}/callback?code=abc&state=def");
        let resp = reqwest::get(&url).await.unwrap();
        assert!(resp.status().is_success());

        let result = handle.await.unwrap().unwrap();
        assert!(result.request_uri.contains("code=abc"));
    }
}
