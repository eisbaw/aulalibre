#!/usr/bin/env bash
# aula_login_helper.sh - Helper for capturing Aula session after manual browser login
#
# Usage:
#   1. Log in at https://www.aula.dk in your browser
#   2. Open DevTools (F12) -> Application/Storage -> Cookies -> www.aula.dk
#   3. Copy all cookie values and paste when prompted
#   OR: Use the --cookie-file flag to load from a Netscape cookie file
#
# The script will:
#   - Save cookies to secrets/aula_cookies.txt
#   - Extract the CSRF token
#   - Make a test API call to validate the session

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SECRETS_DIR="$PROJECT_DIR/secrets"
COOKIE_JAR="$SECRETS_DIR/aula_cookies.txt"
API_BASE="https://www.aula.dk/api/v19"

mkdir -p "$SECRETS_DIR"

usage() {
    echo "Usage: $0 [--cookie-string 'cookie1=val1; cookie2=val2'] [--test-only]"
    echo ""
    echo "Options:"
    echo "  --cookie-string STR   Provide cookies as a single string (from browser DevTools)"
    echo "  --test-only           Only test existing saved cookies"
    echo "  --api-version VER     API version number (default: 19)"
    echo ""
    echo "To get cookies from browser:"
    echo "  1. Log in at https://www.aula.dk"
    echo "  2. Open DevTools (F12) -> Console"
    echo "  3. Run: document.cookie"
    echo "  4. Copy the output and pass with --cookie-string"
    echo ""
    echo "Note: document.cookie will NOT show HttpOnly cookies."
    echo "Instead, use DevTools -> Application -> Cookies -> www.aula.dk"
    echo "and manually construct the cookie string from all visible cookies."
}

save_cookies() {
    local cookie_str="$1"
    echo "$cookie_str" > "$COOKIE_JAR"
    chmod 600 "$COOKIE_JAR"
    echo "Cookies saved to $COOKIE_JAR"
}

extract_csrf() {
    local cookie_str="$1"
    # Extract Csrfp-Token from cookie string
    local csrf=$(echo "$cookie_str" | grep -oP 'Csrfp-Token=\K[^;]+' || true)
    if [ -n "$csrf" ]; then
        echo "$csrf"
    else
        echo ""
    fi
}

test_api_call() {
    local cookie_str="$1"
    local api_version="${2:-19}"
    local api_base="https://www.aula.dk/api/v${api_version}"
    local csrf=$(extract_csrf "$cookie_str")

    echo ""
    echo "=== Testing API connection ==="
    echo "API base: $api_base"
    echo "CSRF token: ${csrf:-(not found in cookies)}"
    echo ""

    # Test 1: GET profiles.getProfilesByLogin (no CSRF needed for GET)
    echo "--- Test 1: GET profiles.getProfilesByLogin ---"
    local response
    response=$(curl -s -w "\n%{http_code}" \
        -H "Cookie: $cookie_str" \
        -H "Accept: application/json" \
        ${csrf:+-H "csrfp-token: $csrf"} \
        "$api_base/?method=profiles.getProfilesByLogin" 2>&1)

    local http_code=$(echo "$response" | tail -1)
    local body=$(echo "$response" | sed '$d')

    echo "HTTP Status: $http_code"

    if [ "$http_code" = "200" ]; then
        echo "Response (first 500 chars):"
        echo "$body" | head -c 500
        echo ""

        # Save full response
        echo "$body" | python3 -m json.tool > "$SECRETS_DIR/profiles_response.json" 2>/dev/null || echo "$body" > "$SECRETS_DIR/profiles_response.json"
        echo "Full response saved to secrets/profiles_response.json"
    else
        echo "Response body:"
        echo "$body" | head -c 1000
    fi

    echo ""

    # Test 2: Try to get the CSRF token from a page load if not in cookies
    if [ -z "$csrf" ]; then
        echo "--- Attempting to get CSRF from API response headers ---"
        local headers
        headers=$(curl -s -D - -o /dev/null \
            -H "Cookie: $cookie_str" \
            "$api_base/?method=profiles.getProfilesByLogin" 2>&1)
        echo "$headers" | grep -i 'set-cookie\|csrfp' || echo "(no CSRF-related headers found)"

        # Try extracting from set-cookie header
        local csrf_from_header=$(echo "$headers" | grep -oP 'Csrfp-Token=\K[^;]+' || true)
        if [ -n "$csrf_from_header" ]; then
            echo "Found CSRF token in response headers: $csrf_from_header"
            echo "$csrf_from_header" > "$SECRETS_DIR/csrf_token.txt"
            chmod 600 "$SECRETS_DIR/csrf_token.txt"
            echo "CSRF token saved to secrets/csrf_token.txt"

            # Update cookie string with CSRF
            cookie_str="${cookie_str}; Csrfp-Token=${csrf_from_header}"
            save_cookies "$cookie_str"
        fi
    else
        echo "$csrf" > "$SECRETS_DIR/csrf_token.txt"
        chmod 600 "$SECRETS_DIR/csrf_token.txt"
        echo "CSRF token saved to secrets/csrf_token.txt"
    fi
}

# Parse arguments
COOKIE_STRING=""
TEST_ONLY=false
API_VERSION="19"

while [[ $# -gt 0 ]]; do
    case $1 in
        --cookie-string)
            COOKIE_STRING="$2"
            shift 2
            ;;
        --test-only)
            TEST_ONLY=true
            shift
            ;;
        --api-version)
            API_VERSION="$2"
            shift 2
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

if [ "$TEST_ONLY" = true ]; then
    if [ -f "$COOKIE_JAR" ]; then
        COOKIE_STRING=$(cat "$COOKIE_JAR")
        test_api_call "$COOKIE_STRING" "$API_VERSION"
    else
        echo "No saved cookies found at $COOKIE_JAR"
        exit 1
    fi
    exit 0
fi

if [ -z "$COOKIE_STRING" ]; then
    echo "Please provide cookies using --cookie-string"
    echo ""
    usage
    exit 1
fi

save_cookies "$COOKIE_STRING"
test_api_call "$COOKIE_STRING" "$API_VERSION"
