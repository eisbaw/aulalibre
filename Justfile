# Aula project recipes

# Default recipe: list available commands
default:
    @just --list

# Build the Rust workspace
build:
    cargo build --manifest-path aula/Cargo.toml

# Run all tests
test:
    cargo test --manifest-path aula/Cargo.toml

# Run clippy linter
lint:
    cargo clippy --manifest-path aula/Cargo.toml -- -D warnings

# Format code
fmt:
    cargo fmt --manifest-path aula/Cargo.toml --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --manifest-path aula/Cargo.toml --all -- --check

# Run the CLI
run *ARGS:
    cargo run --manifest-path aula/Cargo.toml --bin aula-cli -- {{ARGS}}

# End-to-end tests (build + test + lint + fmt check)
e2e: build test lint fmt-check

# Run live E2E tests against the real Aula API (requires auth token)
# See aula/aula-api/src/e2e.rs for setup instructions.
e2e-live:
    cargo test --manifest-path aula/Cargo.toml -p aula-api --test e2e_live_tests -- --ignored

# ---------------------------------------------------------------------------
# Showcase: non-destructive read-only commands (requires valid auth token)
# ---------------------------------------------------------------------------

aula := "cargo run --manifest-path aula/Cargo.toml --bin aula-cli --"

# Show auth status and current profile
who-am-i:
    {{aula}} auth status
    @echo "---"
    {{aula}} profile me

# Show profile master data (name, email, phone)
master-data:
    {{aula}} profile master-data

# List message threads (newest 10)
messages:
    {{aula}} messages list --limit 10

# Read a specific message thread
read-message id:
    {{aula}} messages read {{id}}

# List today's calendar events
today:
    {{aula}} calendar today

# List this week's calendar events
week:
    {{aula}} calendar week

# Show a specific calendar event
event id:
    {{aula}} calendar show {{id}}

# Show children's current presence status
presence:
    {{aula}} presence status

# Show this week's presence schedule
presence-schedule:
    {{aula}} presence schedule

# List recent posts (newest 10)
posts:
    {{aula}} posts list --limit 10

# Show a specific post
post id:
    {{aula}} posts show {{id}}

# List gallery albums
albums:
    {{aula}} gallery list --limit 10

# Show media in an album
album id:
    {{aula}} gallery show {{id}}

# List secure documents
documents:
    {{aula}} documents list --limit 10

# Show a specific document
document id:
    {{aula}} documents show {{id}}

# List recent notifications
notifications:
    {{aula}} notifications list --limit 10

# Search for profiles and groups
search query:
    {{aula}} search "{{query}}"

# Probe which search endpoints are alive
search-probe:
    {{aula}} search --probe dummy

# List groups for an institution profile
groups inst_profile_id:
    {{aula}} groups list --inst-profile {{inst_profile_id}}

# Show group details
group id:
    {{aula}} groups show {{id}}

# Show group members
group-members id:
    {{aula}} groups members {{id}}

# List message folders
folders:
    {{aula}} messages folders

# Run all non-destructive showcase commands (quick smoke test)
showcase:
    @echo "=== Auth Status ==="
    {{aula}} auth status
    @echo ""
    @echo "=== Profile ==="
    {{aula}} profile me
    @echo ""
    @echo "=== Messages (5) ==="
    {{aula}} messages list --limit 5
    @echo ""
    @echo "=== Calendar Today ==="
    {{aula}} calendar today
    @echo ""
    @echo "=== Presence ==="
    {{aula}} presence status
    @echo ""
    @echo "=== Posts (5) ==="
    {{aula}} posts list --limit 5
    @echo ""
    @echo "=== Notifications (5) ==="
    {{aula}} notifications list --limit 5
    @echo ""
    @echo "=== Albums (5) ==="
    {{aula}} gallery list --limit 5
    @echo ""
    @echo "=== Documents (5) ==="
    {{aula}} documents list --limit 5
    @echo ""
    @echo "=== Folders ==="
    {{aula}} messages folders
