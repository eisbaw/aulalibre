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
