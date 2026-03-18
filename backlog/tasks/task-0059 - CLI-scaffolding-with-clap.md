---
id: TASK-0059
title: CLI scaffolding with clap
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:12'
updated_date: '2026-03-18 19:19'
labels:
  - rust
  - aula-cli
dependencies:
  - TASK-0040
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the aula-cli binary crate using clap for argument parsing. Define the top-level command structure with subcommands for each domain: auth, messages, calendar, presence, posts, gallery, documents, notifications, search, groups, profile, config. Add --json flag for JSON output and --env flag for environment selection (production, preprod, test). Set up configuration file handling (XDG_CONFIG_HOME/aula/config.toml) for persistent settings like default environment, default output format, and default institution profile.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 clap-based CLI with derive macros for subcommand structure
- [x] #2 Top-level subcommands: auth, messages, calendar, presence, posts, gallery, documents, notifications, search, groups, profile, config
- [x] #3 Global flags: --json, --env, --verbose, --profile (institution profile selector)
- [x] #4 Config file loading from XDG_CONFIG_HOME/aula/config.toml
- [x] #5 Help text for all commands and subcommands
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add serde, toml, dirs dependencies to aula-cli/Cargo.toml
2. Create config module (config.rs) for XDG config file loading
3. Create commands module directory with mod.rs and one file per subcommand domain
4. Rewrite main.rs with clap derive, global flags, subcommands, and config loading
5. Run e2e tests
6. Mark ACs done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented full CLI scaffolding:
- 12 subcommands: auth, messages, calendar, presence, posts, gallery, documents, notifications, search, groups, profile, config
- Global flags: --json, --env, --verbose, --profile
- Config loading from XDG_CONFIG_HOME/aula/config.toml via dirs + toml crates
- All handlers are stubs (println)
- e2e passes: 439 tests, clippy clean, fmt clean
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
CLI scaffolding for aula-cli using clap derive macros.

Changes:
- Rewrote main.rs with top-level Cli struct, global flags (--json, --env, --verbose, --profile), and 12 subcommands via Command enum
- Added commands/ module directory with one file per domain: auth, messages, calendar, presence, posts, gallery, documents, notifications, search, groups, profile, config
- Added config.rs for XDG_CONFIG_HOME/aula/config.toml loading (serde + toml)
- Added serde, toml, dirs dependencies to Cargo.toml
- All subcommand handlers are stubs -- actual API integration deferred to later tasks

Tests:
- nix-shell --run just e2e: 439 tests pass, clippy clean, fmt clean
- Verified --help output for top-level and subcommands
<!-- SECTION:FINAL_SUMMARY:END -->
