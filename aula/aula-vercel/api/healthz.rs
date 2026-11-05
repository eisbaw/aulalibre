//! Vercel Function: health probe.
//!
//! Unauthenticated on purpose — it reveals nothing beyond liveness, and a
//! weekly cron hitting it keeps a free-tier Supabase project from pausing.

use serde_json::json;
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(healthz)).await
}

async fn healthz(_request: Request) -> Result<Response<ResponseBody>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(ResponseBody::from(json!({"ok": true}).to_string()))
        .unwrap_or_else(|_| Response::new(ResponseBody::from("{}"))))
}
