#!/usr/bin/env bash
# Seed the hosted MCP server's token row from the local CLI login.
#
# Run after `aula auth login` whenever the refresh token has expired and the
# hosted server reports "not logged in".
#
# Requires SUPABASE_URL and SUPABASE_SECRET_KEY (e.g. `set -a; . secrets/supabase.env; set +a`).
set -euo pipefail

: "${SUPABASE_URL:?set SUPABASE_URL}"
: "${SUPABASE_SECRET_KEY:?set SUPABASE_SECRET_KEY}"

row_id="${AULA_TOKEN_ROW:-default}"
tokens="${AULA_DATA_DIR:-$HOME/Library/Application Support/aula}/tokens.json"

if [ ! -f "$tokens" ]; then
  echo "error: no token file at $tokens - run 'aula auth login' first" >&2
  exit 1
fi

payload=$(jq -n --arg id "$row_id" --slurpfile data "$tokens" '[{id: $id, data: $data[0]}]')

status=$(curl -sS -o /tmp/aula_push_tokens.out -w '%{http_code}' \
  -X POST "$SUPABASE_URL/rest/v1/aula_tokens" \
  -H "apikey: $SUPABASE_SECRET_KEY" \
  -H "Content-Type: application/json" \
  -H "Prefer: resolution=merge-duplicates,return=minimal" \
  --data-binary "$payload")

if [ "$status" != "201" ] && [ "$status" != "204" ]; then
  echo "error: upsert failed with HTTP $status" >&2
  head -c 300 /tmp/aula_push_tokens.out >&2; echo >&2
  exit 1
fi

# Read back the expiry so the operator can see what was actually stored.
stored=$(curl -sS "$SUPABASE_URL/rest/v1/aula_tokens?id=eq.$row_id&select=data" \
  -H "apikey: $SUPABASE_SECRET_KEY" -H "Accept: application/json" |
  jq -r '.[0].data.access_token_expiration // "unknown"')

rm -f /tmp/aula_push_tokens.out
echo "seeded row '$row_id' (access token expires at epoch $stored)"
