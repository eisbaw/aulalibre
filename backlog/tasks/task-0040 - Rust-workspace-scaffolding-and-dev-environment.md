---
id: TASK-0040
title: Rust workspace scaffolding and dev environment
status: To Do
assignee: []
created_date: '2026-03-18 16:07'
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
- [ ] #1 Cargo workspace with aula-api (lib) and aula-cli (bin) crates
- [ ] #2 shell.nix provides Rust stable toolchain, rust-analyzer, clippy, rustfmt
- [ ] #3 Justfile with build, test, lint, fmt, run recipes
- [ ] #4 .gitignore covers target/, Cargo.lock for lib crate
- [ ] #5 cargo build and cargo test pass with empty crates
<!-- AC:END -->
