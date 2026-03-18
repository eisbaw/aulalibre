---
id: TASK-0040
title: Rust workspace scaffolding and dev environment
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:07'
updated_date: '2026-03-18 17:08'
labels:
  - rust
  - infrastructure
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the Cargo workspace with aula-api library crate and aula-cli binary crate. Create shell.nix with Rust toolchain (rustc, cargo, rust-analyzer, clippy, rustfmt), justfile with build/test/lint/run recipes, and .gitignore for Rust artifacts. This is the foundation that all other Rust tasks depend on.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Cargo workspace with aula-api (lib) and aula-cli (bin) crates
- [x] #2 shell.nix provides Rust stable toolchain, rust-analyzer, clippy, rustfmt
- [x] #3 Justfile with build, test, lint, fmt, run recipes
- [x] #4 .gitignore covers target/, Cargo.lock for lib crate
- [x] #5 cargo build and cargo test pass with empty crates
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create aula/ directory with Cargo workspace
2. Create aula/aula-api/ lib crate with Cargo.toml and src/lib.rs
3. Create aula/aula-cli/ bin crate with Cargo.toml and src/main.rs
4. Create aula/Cargo.toml workspace manifest
5. Update shell.nix to add Rust toolchain (rustc, cargo, rust-analyzer, clippy, rustfmt) alongside existing APK tools
6. Create Justfile at project root with build, test, lint, fmt, run, e2e recipes
7. Test: nix-shell --run "cargo build --manifest-path aula/Cargo.toml"
8. Test: nix-shell --run "just build"
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Created aula/ workspace with aula-api (lib) and aula-cli (bin) crates
- Added Rust toolchain (rustc, cargo, clippy, rustfmt, rust-analyzer), pkg-config, openssl.dev, and just to shell.nix
- Created Justfile with build, test, lint, fmt, fmt-check, run, e2e recipes
- .gitignore already had target/ and Cargo.lock covered
- All recipes pass: build, test (1 test), clippy (no warnings), fmt-check
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Set up Rust workspace scaffolding under aula/ with two crates:

- aula-api: library crate with reqwest, serde, tokio, thiserror dependencies
- aula-cli: binary crate with clap, depending on aula-api

Updated shell.nix to provide Rust stable toolchain (rustc, cargo, clippy, rustfmt, rust-analyzer) plus build dependencies (pkg-config, openssl.dev, just) alongside existing APK analysis tools.

Created Justfile at project root with recipes: build, test, lint, fmt, fmt-check, run, e2e. The e2e recipe chains build+test+lint+fmt-check.

All recipes pass cleanly via nix-shell.
<!-- SECTION:FINAL_SUMMARY:END -->
