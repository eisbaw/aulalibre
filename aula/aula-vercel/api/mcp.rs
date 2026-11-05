//! Vercel Function: MCP endpoint.
//!
//! Routed as `POST /api/mcp?secret=<AULA_MCP_SECRET>`. All logic lives in
//! `aula_vercel::handler` so it is testable without deploying.

use aula_vercel::handler;
use vercel_runtime::{run, service_fn, Error, Request, Response, ResponseBody};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(mcp)).await
}

async fn mcp(request: Request) -> Result<Response<ResponseBody>, Error> {
    Ok(handler::respond(request).await)
}
