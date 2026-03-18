---
id: TASK-0064
title: 'CLI output formatting (JSON, table, human-readable)'
status: To Do
assignee: []
created_date: '2026-03-18 16:14'
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
- [ ] #1 JSON output mode: serialize any API response to formatted JSON
- [ ] #2 Table output mode: configurable columns, header, alignment
- [ ] #3 Human-readable output mode: formatted detail views
- [ ] #4 Date/time formatting appropriate for Danish context
- [ ] #5 Color/bold for status indicators (terminal colors with fallback)
- [ ] #6 Trait-based design so each domain type implements its own formatting
<!-- AC:END -->
