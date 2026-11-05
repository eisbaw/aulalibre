//! Transport-agnostic JSON-RPC 2.0 dispatch.
//!
//! Both transports — the stdio loop in `main.rs` and the HTTP function used
//! when hosted — funnel every message through [`handle`]. It takes a raw JSON
//! string and returns the response as a `serde_json::Value`, so nothing here
//! knows or cares how the bytes arrive.

use serde_json::{json, Value};

use crate::tools;

/// Protocol version returned when the client does not ask for one.
pub const PROTOCOL_VERSION: &str = "2024-11-05";

/// Handle one JSON-RPC message. Returns `None` for notifications (no `id`).
pub async fn handle(server: &tools::Server, line: &str) -> Option<Value> {
    let request: Value = match serde_json::from_str(line) {
        Ok(value) => value,
        Err(e) => return Some(error(Value::Null, -32700, &format!("invalid JSON: {e}"))),
    };

    // No `id` means a notification (`notifications/initialized`, `cancelled`, …):
    // the spec forbids replying to those.
    let id = request.get("id").cloned()?;
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let params = request.get("params").cloned().unwrap_or(Value::Null);

    let result = match method {
        "initialize" => Ok(json!({
            "protocolVersion": params
                .get("protocolVersion")
                .and_then(Value::as_str)
                .unwrap_or(PROTOCOL_VERSION),
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "aula", "version": env!("CARGO_PKG_VERSION")},
            "instructions": "Aula is a Danish school communication platform. Start with \
                             aula_whoami: it returns the children, the institutions, and the \
                             institution profile ids the other tools take as arguments.",
        })),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(json!({"tools": tools::list()})),
        "tools/call" => tools::call(server, &params).await,
        other => Err((-32601, format!("method not found: {other}"))),
    };

    Some(match result {
        Ok(value) => json!({"jsonrpc": "2.0", "id": id, "result": value}),
        Err((code, message)) => error(id, code, &message),
    })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn call(line: &str) -> Option<Value> {
        handle(&tools::Server::new(), line).await
    }

    #[tokio::test]
    async fn initialize_echoes_protocol_version() {
        let response = call(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#)
            .await
            .expect("initialize must be answered");
        assert_eq!(response["id"], 1);
        assert_eq!(response["result"]["protocolVersion"], "2025-06-18");
        assert_eq!(response["result"]["serverInfo"]["name"], "aula");
    }

    #[tokio::test]
    async fn initialize_falls_back_to_our_protocol_version() {
        let response = call(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#)
            .await
            .expect("initialize must be answered");
        assert_eq!(response["result"]["protocolVersion"], PROTOCOL_VERSION);
    }

    #[tokio::test]
    async fn tools_list_exposes_every_tool_with_schema() {
        let response = call(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#)
            .await
            .expect("tools/list must be answered");
        let tools = response["result"]["tools"]
            .as_array()
            .expect("tools must be an array");
        assert_eq!(tools.len(), 13);
        for tool in tools {
            assert!(tool["name"]
                .as_str()
                .is_some_and(|n| n.starts_with("aula_")));
            assert!(!tool["description"].as_str().unwrap_or("").is_empty());
            assert_eq!(tool["inputSchema"]["type"], "object");
        }
    }

    #[tokio::test]
    async fn notifications_get_no_response() {
        assert!(
            call(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#)
                .await
                .is_none()
        );
    }

    #[tokio::test]
    async fn unknown_method_is_an_error() {
        let response = call(r#"{"jsonrpc":"2.0","id":3,"method":"resources/list"}"#)
            .await
            .expect("errors still carry the id");
        assert_eq!(response["error"]["code"], -32601);
    }

    #[tokio::test]
    async fn unknown_tool_is_an_error() {
        let response =
            call(r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"aula_nope"}}"#)
                .await
                .expect("errors still carry the id");
        assert_eq!(response["error"]["code"], -32602);
    }

    #[tokio::test]
    async fn invalid_json_is_a_parse_error() {
        let response = call("{not json").await.expect("parse errors are reported");
        assert_eq!(response["error"]["code"], -32700);
    }

    #[tokio::test]
    async fn write_tools_reject_empty_arguments_before_touching_the_network() {
        let response = call(r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"aula_send_message","arguments":{"recipients":[]}}}"#)
            .await
            .expect("tool errors are content, not protocol errors");
        assert_eq!(response["result"]["isError"], true);
        assert!(response["result"]["content"][0]["text"]
            .as_str()
            .unwrap_or("")
            .contains("recipients must not be empty"));
    }
}
