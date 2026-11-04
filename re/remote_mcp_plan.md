# Remote MCP Server — Plan

Status: **proposal, not started**
Scope: move `aula-mcp` off the Mac so one server serves both Claude Desktop and Claude mobile.

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
| The pieces needed for a paste-based hosted login already exist: authorize-URL builder, state generator, code extractor, code exchange. | `auth.rs:186,205,262,451` |
| Claude connectors accept **OAuth or unauthenticated** remote servers; there is **no custom Authorization-header field**. | Anthropic connector docs |
| Dynamic Client Registration is deprecated (MCP spec 2026-07-28), so a self-hosted OAuth server may not be registrable by Claude. Unverified without a deploy. | MCP authorization spec |
| Installing connectors from mobile is still beta; add the connector from Desktop/web, it then works on mobile. | Anthropic docs |

## Target architecture

```
Claude Desktop ─┐
                ├─ HTTPS ─→ aula-mcp --http  (container)
Claude mobile ──┘            ├─ POST /mcp/<secret>   JSON-RPC (stateless)
                             ├─ GET  /healthz
                             └─ /data/tokens.json    (volume, 0600)
                                     │
                                     └─→ www.aula.dk/api/v25 (aula-api)
```

- One `tools::Server`, one `Session`, one token store. All clients share it; calls serialize
  behind the existing mutex, which is fine for interactive use.
- Transport is stateless: each POST carries a complete JSON-RPC message and gets a JSON
  response. No SSE, no session ids — sufficient for a tools-only server.
- Refresh stays server-side and headless (`Session::ensure_valid_token`), so the Mac no
  longer needs to be awake or authenticated.

## Work breakdown

### R1 — Extract the JSON-RPC layer from `main.rs`
Move `handle()` and `error()` into `aula-mcp/src/rpc.rs`; `main.rs` keeps only the stdio
loop. Both transports then share one dispatcher.
**AC:** stdio behaviour unchanged; the 7 framing tests move with the code and still pass.

### R2 — HTTP transport
Add `aula-mcp --http <bind>` using `axum` (hyper/tower already in the lockfile via reqwest):
`POST /mcp` → `rpc::handle`, `GET /healthz` → `ok`. Stateless; no SSE.
**AC:** `initialize`, `tools/list`, `tools/call` succeed over HTTP against the live account;
`tools/call` for an unknown tool still returns `-32602`; covered by `tower::ServiceExt::oneshot`
tests that do not hit the network.

### R3 — Endpoint authentication (v1: secret path)
Require a 32-byte random secret path segment (`/mcp/<secret>`), compared in constant time.
Reject anything else with 404 (not 401 — no oracle).
**AC:** missing/wrong secret → 404; correct secret → normal JSON-RPC; unit test for both.
Note: this is a deliberate v1 shortcut. See "Decisions pending".

### R4 — Container and configuration
Multi-stage Dockerfile (rust builder → slim runtime). Env: `AULA_DATA_DIR=/data`,
`AULA_HTTP_BIND=0.0.0.0:8080`, `AULA_MCP_SECRET`, `AULA_ENV`.
**AC:** `docker run` with a mounted volume serves `/healthz` and `/mcp/<secret>`;
image starts in <2s; no tokens or secrets in the image or logs.

### R5 — Token provisioning (no login UI)
`aula auth login` on the Mac, then push `tokens.json` to the volume
(`fly ssh sftp put`, or `fly secrets`-style injection at boot for first run).
Add a `just push-tokens` recipe that documents the exact command.
**AC:** after a push, a `tools/call` from Claude mobile returns live data; no Mac dependency.

### R6 — Deploy
Fly.io: shared-cpu-1x 256MB + 1GB volume, TLS and hostname from the platform.
**AC:** connector added on Desktop answers `aula_whoami`; the same connector works in
Claude mobile; server survives a redeploy with tokens intact.

### R7 — Re-login page (only if the refresh token expires)
`GET /login` renders the Aula authorize URL + a paste box; `POST /login` runs
`auth::extract_code_from_redirect` → `auth::exchange_code(redirect_uri = app-private.aula.dk)`
→ writes tokens to the volume. Reachable only with the admin secret.
**AC:** a deliberately corrupted refresh token is recoverable end-to-end through `/login`
without touching the Mac.
**Trigger:** measure refresh-token lifetime first (see "Open risks"); skip until it bites.

Effort: R1–R6 ≈ 1.5 days. R7 +0.5 day. OAuth (v2) +1–2 days.

## Token lifecycle

1. **Initial**: `aula auth login` on the Mac (MitID), tokens land in
   `~/Library/Application Support/aula/tokens.json`.
2. **Provision**: file copied to the server volume; the server loads it on first tool call
   (logged to stderr: `tokens loaded from <path> (expires …)`).
3. **Refresh**: hourly, automatic, server-side; new tokens persisted back to the volume.
4. **Recovery**: when refresh fails, tools return an explicit message rather than a bare 403
   (`not logged in: no Aula tokens at <path>` / refresh error). R7 automates recovery;
   until then, re-push the file.

## Security and PII

- The endpoint holds tokens for a children's school account. A secret path reduces exposure
  but leaks through logs, browser history and referrers — acceptable for a single private
  user, not for anything shared.
- Tokens are stored `0600` in a volume, never logged (existing behaviour); the server logs
  the token *path* and expiry only.
- No IP allow-listing is possible: Anthropic's infrastructure calls the endpoint.
- If this is ever shared, per-user tokens and OAuth 2.1 (RFC 9728 metadata, PKCE) are
  required, and that is a different design (multi-tenancy), not a tweak.

## Decisions pending

1. **Host**: Fly.io (recommended — volume + TLS + `sftp put`), Cloud Run (no disk, scale to
   zero, harder state), or a small VPS.
2. **Auth for v1**: secret path (R3 as written) or OAuth 2.1 from the start.
3. **Scope**: single user (one token store, one session) or eventually multiple guardians
   (multi-tenant).

## Out of scope

- Native iOS/Android app (Rust core + `uniffi` + Swift/Kotlin UI) — weeks, and it would
  rebuild UI Claude already provides.
- Multi-user token stores, per-user rate limits, audit logs.
- Local Mac + tunnel variant (documented alternative, not chosen).

## Verification

- `cargo test --workspace` (currently 8 suites) plus new HTTP transport tests.
- Live smoke over HTTPS: `initialize` → `tools/list` → `aula_whoami` → `aula_presence`.
- Connector test from both Claude Desktop and Claude mobile.
- Restart/redeploy test: tokens and session survive.
- Failure-path test: wrong secret → 404; expired refresh token → actionable error.

## Open risks

- **Refresh-token lifetime is unknown.** "Runs unattended forever" is unproven; measure it
  before deciding whether R7 is optional.
- **OAuth registrability is unverified.** If Claude's connector flow needs DCR (deprecated),
  the secret-path variant may be the only option short of running our own authorization
  server.
- **Cold starts / timeouts**: Aula calls are fast, but platform request limits (Fly ~60s)
  should be checked for the largest tool (`aula_thread` on a 49-participant thread).
- **Aula rejects a custom `redirect_uri`** — inferred, not tested. If wrong, R7 becomes a
  clean redirect flow instead of a paste box.
