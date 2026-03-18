---
id: TASK-0064
title: 'CLI output formatting (JSON, table, human-readable)'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 16:14'
updated_date: '2026-03-18 20:15'
labels:
  - rust
  - aula-cli
dependencies:
  - TASK-0059
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement a unified output formatting layer for the CLI. Three modes: (1) JSON: raw serde_json output of API responses, activated by --json flag. (2) Table: columnar output using comfy-table or similar, the default for list commands. (3) Human-readable: prose-like output for detail/show commands. The formatter should handle common patterns: date/time formatting (Danish locale), truncation of long text fields, color highlighting for status fields (unread=bold, sick=red, present=green), and pagination hints for large result sets.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 JSON output mode: serialize any API response to formatted JSON
- [x] #2 Table output mode: configurable columns, header, alignment
- [x] #3 Human-readable output mode: formatted detail views
- [x] #4 Date/time formatting appropriate for Danish context
- [x] #5 Color/bold for status indicators (terminal colors with fallback)
- [x] #6 Trait-based design so each domain type implements its own formatting
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create shared output module: aula-cli/src/output.rs
   - Shared helper functions: truncate, truncate_date, strip_html_tags, format_datetime
   - Color support with NO_COLOR env var respect
   - JSON output helper (single fn wrapping serde_json::to_string_pretty)
   - Table printing struct/helpers for consistent column formatting
2. Define CliDisplay trait for domain types (table row + detail view)
3. Refactor all 12 command files to use shared output module
   - Remove duplicated truncate/truncate_date/strip_html_tags from each file
   - Remove duplicated resolve_environment/token_store/build_session (extract to common module)
   - Use shared json_output() helper instead of inline serde_json calls
   - Use shared color helpers for status indicators
4. Add color for status fields: presence (sick=red, present=green), unread markers (bold)
5. Run just e2e, fix any issues
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented shared output module (output.rs) and session utility module (session_util.rs).
Refactored all 12 command files to use shared formatting.
Eliminated duplicated truncate/truncate_date/strip_html_tags/resolve_environment/token_store/build_session across 11 files.
Added color support with NO_COLOR env var respect.
Added Table struct for consistent columnar output.
All 439 tests pass, clippy clean, fmt clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented a unified output formatting layer for the aula-cli.

Changes:
- New `output.rs` module providing: JSON output (print_json), Table struct for columnar output with headers/alignment/truncation, color helpers (bold/red/green/yellow/dim) respecting NO_COLOR env var, shared text utilities (truncate, format_datetime, split_datetime, extract_time, strip_html_tags), status-aware coloring (presence status, unread markers), pagination hints, and a CliDisplay trait for extensibility.
- New `session_util.rs` module extracting shared resolve_environment/token_store/build_session that were copy-pasted across 11 command files.
- Refactored all 12 command modules (auth, calendar, documents, gallery, groups, messages, notifications, posts, presence, profile, search, config) to use the shared modules, eliminating ~400 lines of duplicated helper functions.
- Detail views use bold headers and dim separators for readability.
- Presence status shows colored output: sick=red, present=green, absent=yellow.
- Auth status shows EXPIRED in red.

Tests:
- All 439 existing tests pass.
- Clippy clean (zero warnings).
- Formatting clean (cargo fmt --check passes).
<!-- SECTION:FINAL_SUMMARY:END -->
