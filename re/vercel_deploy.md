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

Root directory: `aula` (the Rust workspace). Framework preset: other/none.

Environment variables (Production, and Preview if you use it):

| Variable | Value |
|---|---|
| `SUPABASE_URL` | `https://udmwfkrdwytntnqnhrmv.supabase.co` |
| `SUPABASE_SECRET_KEY` | project secret key (`sb_secret_…`) — bypasses RLS, server-side only |
| `AULA_MCP_SECRET` | `openssl rand -hex 24` — the shared secret callers must present |
| `AULA_ENV` | `production` (optional) |

Region: `dub1` (Dublin) matches the Supabase `eu-west-1` region; set it in
`vercel.json` or project settings. Running in `iad1` adds a transatlantic hop
on every Aula call.

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

Checks: `healthz` 200, no-secret 404 (proves protection is off), wrong-secret
404, `GET` 405, and `tools/list` returning 12 tools.

## 5. Connect Claude

Add a custom connector with the URL:

```
https://<project>.vercel.app/api/mcp?secret=<AULA_MCP_SECRET>
```

Add it from Desktop or web — installing connectors from mobile is still beta,
but once added it works on mobile too. Then ask something like *"what has the
school sent me this week?"*

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
