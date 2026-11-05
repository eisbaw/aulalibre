# Remote MCP Server — Plan

Status: **in progress** — R1 done, R2 de-risked
Progress:

- **R1 done**: dispatch extracted to `aula-mcp/src/rpc.rs`, crate is now lib + bin, 14 tests green, verified live over stdio.
- **R3 done**: secret enforced in the handler (constant-time, 404 on mismatch, 405 for non-POST), and `scripts/verify_deployment.sh` checks from outside that Deployment Protection is off — a no-secret request must be 404, not Vercel's 401.
- **R5 done**: `scripts/push_tokens.sh` seeds the row from the local login. Verified live: forced an expired token in Supabase, the hosted path refreshed it, persisted the new token and kept the refresh token.
- **R6 in progress**: deployed to Vercel (root `aula/aula-vercel`, region `dub1`). Verified from outside: healthz 200, no-secret 404 (Deployment Protection off), wrong secret 404, GET 405, tools/list 12 tools. Remaining: a real tool call through the deployment, and adding the connector.
- **R2 done**: new `aula-vercel` crate (bins `mcp` + `healthz`). Endpoint is `POST /api/mcp?secret=<AULA_MCP_SECRET>` — a query param rather than a `vercel.json` rewrite, since each `api/*.rs` file is already its own route. Wrong/missing secret answers 404, non-POST answers 405, and the whole path (Supabase load → dispatch → write-back) is covered by tests with wiremock as the store.
- **R4 done**: `TokenStore` is a `File | Memory` backend and `aula_api::SupabaseTokens` does async load/save through PostgREST (secret key on the `apikey` header, upsert via `Prefer: resolution=merge-duplicates`); 6 new wiremock tests.
- **R2 risk resolved**: `vercel_runtime::Response` is a re-export of `http::Response` (crate 2.4.1), so `Response::builder().status(..).header(..)` works — JSON-RPC error codes and the 405 for `GET` are supported.

Target: **Vercel** (Functions, official Rust runtime) + **Supabase** (Postgres via PostgREST) for token storage.
Scope: move `aula-mcp` off the Mac so one server serves Claude Desktop and Claude mobile.
Fly.io is kept as a documented alternative at the end.

## Problem

The MCP server currently runs as a stdio child of Claude Desktop on a single Mac:

- iOS/Android cannot spawn subprocesses, so Claude mobile can only reach **remote** MCP
  servers through Connectors (Streamable HTTP). Nothing can run on the phone.
- A server bound to one Mac is unavailable whenever the Mac sleeps or is elsewhere.
- Two local instances (one per Claude window) share one token store, which caused the
  refresh-token race fixed in `Session::refresh_token` — a single remote server removes
  that class of bug entirely.

Goal: one HTTPS endpoint holding one token store, used by every Claude surface.

## Constraints discovered (evidence)

| Constraint | Evidence |
|---|---|
| Only `aula-mcp/src/main.rs` is transport-bound (~40 of 167 lines). `tools::{Server, list, call}` takes/returns `serde_json::Value` and is transport-agnostic. | `aula/aula-mcp/src/tools.rs:37,149,258` |
| `aula-api` has exactly one platform-specific call — `dirs::data_dir()` — already overridable via `AULA_DATA_DIR`. | `aula/aula-api/src/token_store.rs:66` |
| Aula's OIDC `redirect_uri` is hardcoded to Aula's own domain, so a hosted "log in on my domain" redirect flow is unavailable unless Aula accepts a custom URI. | `aula/aula-api/src/auth.rs:36` (`https://app-private.aula.dk`) |
| The pieces needed for a paste-based login already exist: authorize-URL builder, state generator, code extractor, code exchange. | `auth.rs:186,205,262,451` |
| Claude connectors accept **OAuth or unauthenticated** remote servers; there is **no custom Authorization-header field**. | Anthropic connector docs |
| Dynamic Client Registration is deprecated (MCP spec 2026-07-28), so a self-hosted OAuth server may not be registrable by Claude. Unverified without a deploy. | MCP authorization spec |
| Installing connectors from mobile is still beta; add the connector from Desktop/web, it then works on mobile. | Anthropic docs |
| **Rust is an official Vercel runtime** (`vercel_runtime` v2); each `api/*.rs` file becomes a function, declared as a `[[bin]]`. | Vercel Rust runtime docs |
| Vercel functions have a read-only filesystem plus ephemeral `/tmp` (500 MB). No persistent disk. Hobby: 300 s max duration, 2 GB RAM, 250 MB bundle. | Vercel functions docs |
| Vercel has no built-in KV any more; storage is Blob, Global Config, or **Marketplace** (Supabase, Neon, Upstash) provisioned by integration. | Vercel storage docs |
| Supabase keys: **secret key goes on the `apikey` header, not `Authorization: Bearer`** (it is not a JWT). Secret keys map to `service_role` and `BYPASSRLS` — they skip every RLS policy. | Supabase API-keys docs |
| Supabase free plan: unlimited API requests, 500 MB DB, 5 GB egress, **projects pause after 1 week of inactivity**, max 2 active projects. | Supabase pricing |

## Target architecture

```
Claude Desktop ─┐
                ├─ HTTPS ─→ Vercel Function (Rust, fra1)
Claude mobile ──┘              ├─ POST /mcp/<secret>   → rpc::handle  (stateless JSON-RPC)
                               ├─ GET  /healthz        → 200 ok
                               └─ apikey: <supabase secret key>
                                        │
                                        ├─→ Supabase PostgREST  (tokens row)
                                        └─→ www.aula.dk/api/v25 (aula-api)
```

- One JSON-RPC dispatcher, two transports: stdio locally, HTTP on Vercel.
- No SSE and no session ids — each POST carries a complete JSON-RPC message and gets a JSON
  response, which is all a tools-only server needs.
- Refresh stays server-side and headless, so the Mac no longer needs to be awake.

## Storage design

One row, one JSON blob — the same payload the CLI already writes to `tokens.json`:

```sql
create table public.aula_tokens (
  id          text primary key,           -- 'default'
  data        jsonb not null,             -- LoginData
  updated_at  timestamptz not null default now()
);
alter table public.aula_tokens enable row level security;  -- no policies: publishable keys see nothing
```

Access from the function (plain `reqwest`, no new dependency):

- read: `GET /rest/v1/aula_tokens?id=eq.default&select=data`, header `apikey: <secret key>`
- write: `POST /rest/v1/aula_tokens` with `Prefer: resolution=merge-duplicates` (upsert)

A Storage bucket would work too, but a single row is simpler, transactional and free-tier
ample. RLS is enabled with no policies so a leaked *publishable* key reads nothing; only the
secret key (server-side env var) can touch the row.

## Work breakdown

### R1 — Extract the JSON-RPC layer and lib-ify the crate
Move `handle()`/`error()` from `main.rs` into `aula-mcp/src/rpc.rs`; add `src/lib.rs`
exposing `rpc`, `tools`, `compact`; keep `src/main.rs` as the stdio bin.
**AC:** stdio behaviour unchanged; the 7 framing tests move with the code and still pass;
`cargo test -p aula-mcp` green.

### R2 — Vercel function adapter
Add `api/mcp.rs` using `vercel_runtime` v2 (declared as a `[[bin]]`), mapping the incoming
request body to `rpc::handle` and returning a response with the right status/headers.
`GET` answers 405; the secret travels as `?secret=` because connectors cannot send headers
(no `vercel.json` rewrite needed — each `api/*.rs` file is its own route).
**AC:** `initialize`, `tools/list`, `tools/call` succeed over the deployed URL;
unknown tool still returns `-32602`; `GET` returns 405.
**Verify first:** that `vercel_runtime` allows a fully custom `Response` (status + headers) —
confirmed: it re-exports `hyper::Response` and `ResponseBody`, so both are available.

### R3 — Endpoint authentication (v1: secret path)
Validate the secret path segment in the handler with a constant-time compare; mismatch → 404
(not 401 — no oracle). **Deployment Protection must be disabled** on the production domain,
otherwise Anthropic's servers get a 401 and connectors cannot send a bypass header.
**AC:** wrong secret → 404; correct secret → normal JSON-RPC; unit test for both;
`curl` from outside the network reaches the function.

### R4 — Token store backends
Turn `TokenStore` into a backend enum: `File` (unchanged, CLI/stdio) and `Supabase`
(PostgREST). Selected by env (`AULA_STORE=supabase`). Same 0600-equivalent care: the secret
key never leaves the server env, never logged.
**AC:** `aula auth login` writes to Supabase when configured; a fresh process reads the row,
refreshes an expired token, and writes it back; round-trip covered by a unit test with a mock
HTTP server (the repo already uses `wiremock`).

### R5 — Provisioning
Create the table, generate a Supabase secret key, set Vercel env vars
(`SUPABASE_URL`, `SUPABASE_SECRET_KEY`, `AULA_MCP_SECRET`, `AULA_ENV`), and seed the row once
by running `aula auth login` locally with `AULA_STORE=supabase`.
**AC:** after seeding, a `tools/call` from Claude mobile returns live data; no Mac dependency.

### R6 — Deploy
Vercel project (Hobby), region `fra1` (EU, close to `www.aula.dk`), Supabase project in a
matching EU region. Add the connector on Desktop, confirm it appears on mobile.
**AC:** `aula_whoami` answers from both surfaces; a redeploy keeps the token row intact.

### R7 — Re-login page (only if the refresh token expires)
`/login` renders the Aula authorize URL + a paste box; `POST` runs
`auth::extract_code_from_redirect` → `auth::exchange_code(redirect_uri = app-private.aula.dk)`
→ writes the Supabase row. Reachable only with the admin secret.
**AC:** a deliberately corrupted refresh token is recoverable end-to-end without touching the Mac.
**Trigger:** measure refresh-token lifetime first; skip until it bites.

### R8 — Optional: cache the onboarding payload
Every invocation is a fresh process, so `ensure_context_initialized()` costs 2 Aula requests
per tool call (~400 ms). Cache `OnboardingResponseDto` in the same table with a TTL.
**AC:** a warm call makes 1 Aula request instead of 3; stale cache older than the TTL is ignored.

Effort: R1–R6 ≈ 2 days. R7 +0.5 day. R8 +0.5 day. OAuth (v2) +1–2 days.

## Token lifecycle

1. **Initial**: `aula auth login` with `AULA_STORE=supabase` (MitID), writing straight to the row.
2. **Load**: each invocation reads the row, logs the token path/expiry (never the token).
3. **Refresh**: hourly, automatic, server-side; the new token is written back to Supabase.
4. **Recovery**: when refresh fails, tools return an explicit message rather than a bare 403
   (`not logged in` / refresh error). R7 automates recovery; until then, re-seed the row.

## Security and PII

- The endpoint holds tokens for a children's school account. A secret path reduces exposure
  but leaks through access logs and browser history — acceptable for a single private user,
  not for anything shared. If this is ever shared, per-user tokens and OAuth 2.1 are required,
  and that is a different design (multi-tenancy), not a tweak.
- The Supabase secret key bypasses all RLS. It lives only in Vercel env vars; a leak exposes
  the whole project, so it must never reach the repo, logs, or the client.
- Tokens are never logged; only the store kind and expiry are.
- No IP allow-listing is possible: Anthropic's infrastructure calls the endpoint.
- **Free-tier pause**: a Supabase project unused for a week is paused, and the server starts
  returning 500s until it is resumed in the dashboard. Normal connector use counts as
  activity; add a weekly Vercel Cron hitting `/healthz` if the server may sit idle.

## Decisions pending

1. **Auth for v1**: secret path (R3 as written) or OAuth 2.1 from the start.
2. **Scope**: single user (one row, one session) or eventually multiple guardians (multi-tenant).
3. **Region pairing**: confirm the Supabase project and Vercel function share an EU region.

## Alternative: Fly.io

| | Vercel + Supabase | Fly.io |
|---|---|---|
| Cost | free (Hobby + Supabase free) | ~$2–5/month |
| Process model | per-invocation, cold starts possible | one long-lived process, session reused |
| Aula calls per tool call | 3 (2 context + 1) unless R8 | 1 |
| Token storage | Postgres row via PostgREST | file on a persistent volume |
| Code changes | R1–R5 (Vercel adapter + store backend) | R1 + axum transport + volume |
| Ops | nothing to babysit | server, volume, redeploys |

Vercel is chosen for cost and zero ops; Fly wins on runtime simplicity (no store rewrite, no
per-call context init). Moving to Fly later is cheap: the store backend abstraction from R4
stays useful, and only the transport adapter changes.

## Out of scope

- Native iOS/Android app (Rust core + `uniffi` + Swift/Kotlin UI) — weeks, and it would
  rebuild UI Claude already provides.
- Multi-user token stores, per-user rate limits, audit logs.
- Local Mac + tunnel variant.

## Verification

- `cargo test --workspace` plus new tests: HTTP adapter, secret-path rejection, Supabase
  round-trip against `wiremock`.
- Live smoke over HTTPS: `initialize` → `tools/list` → `aula_whoami` → `aula_presence`.
- Connector test from both Claude Desktop and Claude mobile.
- Redeploy test: the Supabase row survives; refresh still works.
- Failure paths: wrong secret → 404; expired refresh token → actionable error; paused
  Supabase project → error that says so.

## Open risks

- ~~**`vercel_runtime` custom responses unverified**~~ **Resolved**: the runtime re-exports
  `http::Response`, so status- and header-controlled responses are available.
- **Refresh-token lifetime is unknown.** "Runs unattended forever" is unproven; measure it
  before deciding whether R7 is optional.
- **OAuth registrability is unverified.** If Claude's connector flow needs DCR (deprecated),
  the secret-path variant may be the only option short of running our own authorization server.
- **Supabase free-tier pause** could look like an outage; needs the cron or manual resume.
- **Aula rejects a custom `redirect_uri`** — inferred, not tested. If wrong, R7 becomes a
  clean redirect flow instead of a paste box.
