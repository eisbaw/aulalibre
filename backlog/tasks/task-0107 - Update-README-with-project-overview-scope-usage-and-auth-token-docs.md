---
id: TASK-0107
title: 'Update README with project overview, scope, usage, and auth token docs'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-20 22:24'
updated_date: '2026-03-20 22:29'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The README needs to document what this project is, its scope, how to run the CLI, where auth tokens are stored, and reference existing documentation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Describe what the project is (Aula reverse engineering / Rust CLI)
- [x] #2 Document scope: what the CLI can do
- [x] #3 Document how to run the CLI (nix-shell, cargo, etc.)
- [x] #4 Document where auth tokens are stored (secrets/ directory)
- [x] #5 Reference existing docs (re/architecture.md, re/prd.apk_decompile.md, etc.)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Gather info from codebase: CLI subcommands, auth flow, config, Justfile recipes
2. Write README.md covering all 5 AC items
3. Check off AC items
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added README.md documenting the project end-to-end.

Covers:
- Project overview (APK reverse engineering + Rust CLI)
- Full CLI command reference table with all 13 subcommand groups
- How to run: nix-shell, just recipes, cargo commands
- Auth flow explanation (OIDC PKCE, auto vs manual mode)
- Token storage location (~/.local/share/aula/tokens.json, 0600 perms)
- secrets/ directory purpose (dev artifacts, gitignored)
- Project directory structure
- Reverse engineering docs reference table (architecture.md, auth_flow.md, api_endpoints.md, etc.)
- Configuration file format (~/.config/aula/config.toml)
<!-- SECTION:FINAL_SUMMARY:END -->
