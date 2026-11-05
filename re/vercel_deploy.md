# Deploying the MCP server to Vercel

Operational runbook for the hosted `aula-mcp`. Design rationale lives in
[`remote_mcp_plan.md`](remote_mcp_plan.md); this is the "make it run" checklist.

## 1. Supabase (already done once)

Project `udmwfkrdwytntnqnhrmv`, region `eu-west-1` (Ireland). The schema is in
[`supabase_schema.sql`](supabase_schema.sql) — run it in the SQL editor if the
table is ever missing.

The token row is written by the local CLI and pushed:

```bash
set -a; . secrets/supabase.env; set +a      # SUPABASE_URL, SUPABASE_SECRET_KEY
just push-tokens                            # upserts row 'default'
```

Repeat that whenever the hosted server reports `not logged in` — that means the
Aula refresh token expired and only an interactive MitID login can fix it.

## 2. Vercel project

**Connect the fork, not the upstream repo.** Vercel must import `BBiering/aulalibre` and
deploy branch `feat/aula-mcp`; importing `eisbaw/aulalibre` shows the pre-work tree
(`aula-fuse` present, `aula-mcp`/`aula-vercel` absent). If the fork is missing from the
import list, grant the Vercel GitHub app access to it (Account Settings → Integrations →
GitHub → Configure → repository access).

**There is no branch picker at import.** Vercel always creates the project from `main`
(then `master`, then the repo default), so either:

- *preferred*: set the fork's GitHub default branch to `feat/aula-mcp` first, import, then
  restore the default after the PRs merge; or
- import `main` with **Root Directory empty** (typing `aula/aula-vercel` is rejected while
  `main` is the branch, because that path does not exist there), then Settings →
  Environments → Production → Branch Tracking → `feat/aula-mcp`, set Root Directory, and
  **Promote to Production** the newest `feat/aula-mcp` deployment. Changing the production
  branch does not always redeploy its head commit.

Switch the production branch back to `main` once the PRs merge.

Framework preset: **Other**. Root Directory: **`aula/aula-vercel`** — not `aula`.
The adapter's `api/` directory and its package `Cargo.toml` (with the `[[bin]]`s) live
there, and `aula/Cargo.toml` is a *virtual* workspace manifest with no `[package]`, which
the Rust runtime cannot build from. Leave Build/Output/Install commands empty: the Rust
runtime compiles the `[[bin]]`s itself.

The adapter's path dependencies (`../aula-api`, `../aula-mcp`) sit outside the root
directory. Vercel clones the whole repo and runs the build with that working directory,
so they resolve — but if a build fails with "failed to load manifest for workspace
member", that assumption is the thing to check first.

Environment variables (Production, and Preview if you use it):

| Variable | Value |
|---|---|
| `SUPABASE_URL` | `https://udmwfkrdwytntnqnhrmv.supabase.co` |
| `SUPABASE_SECRET_KEY` | project secret key (`sb_secret_…`) — bypasses RLS, server-side only |
| `AULA_MCP_SECRET` | `openssl rand -hex 24` — the shared secret callers must present |
| `AULA_ENV` | `production` (optional) |

Region: `dub1` (Dublin) matches the Supabase `eu-west-1` region; it is pinned in
`aula/aula-vercel/vercel.json` (the file must sit in the project root directory).
Running in `iad1` adds a transatlantic hop on every Aula call.

## 3. Deployment Protection must be OFF

This is the failure that looks like an auth bug and is not: with Deployment
Protection enabled, Vercel answers **401 before the function runs**, and Claude
connectors cannot send a bypass header. Turn it off for the production domain
(Settings → Deployment Protection), or use a domain without protection.

`scripts/verify_deployment.sh` detects it: a request with no secret must return
**404** (our handler) and not 401 (Vercel's protection).

## 4. Verify

```bash
AULA_MCP_URL=https://<project>.vercel.app \
AULA_MCP_SECRET=<secret> ./scripts/verify_deployment.sh
```

Checks: `healthz` 200, no-secret 404 (proves protection is off), wrong-secret 404,
`GET` 405, `tools/list` returning 12 tools, and a real `aula_whoami` call with its
latency. The last one is the only check that proves the token row loads and Aula is
reachable — `tools/list` passes on a hollow deployment. A `not logged in` failure there
means the row is empty: run `just push-tokens`.

## 5. Connect Claude

Add a custom connector with the URL:

```
https://<project>.vercel.app/api/mcp?secret=<AULA_MCP_SECRET>
```

Add it from Desktop or web — installing connectors from mobile is still beta,
but once added it works on mobile too. Then ask something like *"what has the
school sent me this week?"*

In the dialog set **Authentication → "No sign in"**: Claude probes the URL,
finds no OAuth discovery endpoints, and may otherwise attempt a flow that cannot
succeed.

If the dialog offers **Request headers** (beta, limited organizations), prefer a
header over the query parameter so the secret stays out of access logs and the
stored URL: name `x-api-key` (or `authorization` with value `Bearer <secret>`),
and drop `?secret=` from the URL. Only those standard names are accepted — a
custom name like `X-Aula-Secret` is rejected until Anthropic approves it, which
is not worth requesting for a personal deployment. The handler accepts the header
or the query parameter, so either works.

## Known operational notes

- **Secret in the URL** — connectors cannot send headers, so the secret is a
  query parameter and therefore appears in Vercel's access logs. Rotate it by
  changing `AULA_MCP_SECRET` and re-adding the connector URL.
- **Supabase free tier pauses after a week of inactivity**, after which the
  server 500s until the project is resumed. Normal use counts as activity; a
  weekly Vercel Cron hitting `/api/healthz` keeps it awake.
- **Per-invocation cost** — every tool call re-initialises the Aula session
  (2 extra Aula requests, ~400 ms) unless the optional onboarding cache is
  added (R8 in the plan).
- **Rotate the Supabase secret key** if it has ever been shared in plain text
  (chat, logs, tickets): Settings → API Keys.
