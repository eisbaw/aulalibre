#!/usr/bin/env bash
# Verify a deployed aula-mcp endpoint from the outside.
#
#   AULA_MCP_URL=https://<project>.vercel.app \
#   AULA_MCP_SECRET=<secret> ./scripts/verify_deployment.sh
#
# Checks the four things that actually break on Vercel:
#   1. the function runs at all (healthz)
#   2. Deployment Protection is OFF - it would answer 401 before our code runs,
#      and Claude connectors cannot send a bypass header
#   3. the secret is enforced (wrong/missing secret is a 404, not an oracle)
#   4. a real MCP call round-trips
set -uo pipefail

: "${AULA_MCP_URL:?set AULA_MCP_URL, e.g. https://aula-mcp.vercel.app}"
: "${AULA_MCP_SECRET:?set AULA_MCP_SECRET}"
url="${AULA_MCP_URL%/}"
fail=0

check() { # name expected_status actual_status extra
  if [ "$2" = "$3" ]; then
    echo "  ok   $1 ($3)"
  else
    echo "  FAIL $1 (expected $2, got $3) ${4:-}"
    fail=1
  fi
}

echo "verifying $url"

status=$(curl -s -o /tmp/aula_verify_health.json -w '%{http_code}' "$url/api/healthz")
check "healthz" 200 "$status"
store=$(jq -r '.store // "?"' /tmp/aula_verify_health.json 2>/dev/null)
case "$store" in
  ok) echo "  ok   token store reachable" ;;
  not-configured) echo "  note token store not configured (SUPABASE_URL / SUPABASE_SECRET_KEY unset)" ;;
  *) echo "  FAIL token store unreachable (store=$store) - the Supabase project may be paused"
     fail=1 ;;
esac
rm -f /tmp/aula_verify_health.json

status=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$url/api/mcp" \
  -H 'content-type: application/json' -d '{}')
if [ "$status" = "401" ]; then
  echo "  FAIL no-secret request returned 401: Deployment Protection is enabled."
  echo "       Turn it off (Settings -> Deployment Protection) or Claude cannot reach this."
  fail=1
else
  check "no secret is rejected" 404 "$status"
fi

status=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$url/api/mcp?secret=wrong" \
  -H 'content-type: application/json' -d '{}')
check "wrong secret is rejected" 404 "$status"

status=$(curl -s -o /dev/null -w '%{http_code}' "$url/api/mcp?secret=$AULA_MCP_SECRET")
check "GET is rejected" 405 "$status"

body=$(curl -sS -X POST "$url/api/mcp?secret=$AULA_MCP_SECRET" \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}')
tools=$(printf '%s' "$body" | jq -r '.result.tools | length' 2>/dev/null)
if [ "$tools" = "13" ]; then
  echo "  ok   tools/list returned 13 tools"
else
  echo "  FAIL tools/list did not return 13 tools: $(printf '%s' "$body" | head -c 200)"
  fail=1
fi

# 5. A real tool call. tools/list passes even when the token row is missing or
# Aula is unreachable, so this is the check that proves the deployment works.
who=$(curl -sS -o /tmp/aula_verify_who.json -w '%{time_total}' \
  -X POST "$url/api/mcp?secret=$AULA_MCP_SECRET" \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"aula_whoami","arguments":{}}}')
is_error=$(jq -r '.result.isError' /tmp/aula_verify_who.json 2>/dev/null)
if [ "$is_error" = "false" ]; then
  echo "  ok   aula_whoami answered in ${who}s (tokens loaded, Aula reachable)"
else
  detail=$(jq -r '.result.content[0].text // .error.message' /tmp/aula_verify_who.json 2>/dev/null | head -c 200)
  echo "  FAIL aula_whoami: ${detail:-no JSON response}"
  echo "       'not logged in' means the token row is empty: run 'just push-tokens'."
  fail=1
fi
rm -f /tmp/aula_verify_who.json

if [ "$fail" -ne 0 ]; then
  echo "deployment check FAILED"
  exit 1
fi
echo "deployment check passed"
