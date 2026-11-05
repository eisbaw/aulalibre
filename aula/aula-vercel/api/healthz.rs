//! Vercel Function: health probe, and the keepalive for the token store.
//!
//! Unauthenticated on purpose — it reveals only liveness and whether the token
//! store answers, never any data.
//!
//! A daily cron hits this (see `vercel.json`). That read against Supabase is the
//! point: free-tier projects pause after a week without activity, and a cron
//! that only touched Vercel would keep the function warm while the database
//! still went to sleep.

use aula_api::SupabaseTokens;
use serde_json::json;
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(healthz)).await
}

async fn healthz(_request: Request) -> Result<Response<ResponseBody>, Error> {
    let store = match SupabaseTokens::from_env() {
        None => "not-configured",
        Some(tokens) => match tokens.load().await {
            Ok(_) => "ok",
            Err(e) => {
                // Never log token values; the error carries only the HTTP status.
                eprintln!("[healthz] token store unavailable: {e}");
                "error"
            }
        },
    };

    let body = json!({"ok": true, "store": store}).to_string();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(ResponseBody::from(body))
        .unwrap_or_else(|_| Response::new(ResponseBody::from("{}"))))
}
