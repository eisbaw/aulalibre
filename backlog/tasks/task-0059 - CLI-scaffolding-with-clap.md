---
id: TASK-0059
title: CLI scaffolding with clap
status: To Do
assignee: []
created_date: '2026-03-18 16:12'
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
- [ ] #1 clap-based CLI with derive macros for subcommand structure
- [ ] #2 Top-level subcommands: auth, messages, calendar, presence, posts, gallery, documents, notifications, search, groups, profile, config
- [ ] #3 Global flags: --json, --env, --verbose, --profile (institution profile selector)
- [ ] #4 Config file loading from XDG_CONFIG_HOME/aula/config.toml
- [ ] #5 Help text for all commands and subcommands
<!-- AC:END -->
