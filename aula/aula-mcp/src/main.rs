//! `aula-mcp` stdio transport.
//!
//! Reads newline-delimited JSON-RPC from stdin and writes responses to stdout —
//! the transport MCP clients launch directly. Dispatch lives in
//! [`aula_mcp::rpc`]; this file only moves bytes.
//!
//! ```text
//! aula-mcp                  # run over stdio
//! AULA_ENV=preprod aula-mcp # target a non-production environment
//! ```

use std::io::{self, BufRead, Write};

use aula_mcp::{rpc, tools};

#[tokio::main]
async fn main() {
    let server = tools::Server::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = rpc::handle(&server, &line).await {
            if writeln!(out, "{response}").is_err() {
                break;
            }
            let _ = out.flush();
        }
    }
}
