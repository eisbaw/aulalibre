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
