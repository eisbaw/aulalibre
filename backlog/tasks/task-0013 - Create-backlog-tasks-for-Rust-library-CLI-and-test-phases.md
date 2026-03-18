---
id: TASK-0013
title: 'Create backlog tasks for Rust library, CLI, and test phases'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:33'
updated_date: '2026-03-18 16:16'
labels: []
dependencies:
  - TASK-0008
  - TASK-0009
  - TASK-0010
  - TASK-0011
  - TASK-0012
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After the exploration/reverse engineering phase (tasks 1-12) is complete, create detailed fine-grained backlog tasks for the next phases: Rust library implementation, CLI tool, unit tests, integration tests, and end-to-end tests. Task definitions should be driven by the actual API surface, auth flow, and data models discovered during reverse engineering.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Rust library tasks created covering: API client, auth, data models, error handling
- [x] #2 CLI tool tasks created covering: commands, output formats, configuration
- [x] #3 Test tasks created covering: unit tests, integration tests, e2e tests with manual login bootstrap
- [x] #4 Task dependencies reflect the actual architecture discovered
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read all analysis files (auth_flow.md, api_endpoints.md, data_models.md, domain_concepts.md, notification_messaging.md)
2. Design task breakdown for Rust library (models, auth, HTTP client, services)
3. Design task breakdown for CLI (scaffolding, auth, domain commands, formatting)
4. Design task breakdown for tests (unit, mock server, integration, e2e bootstrap, e2e tests)
5. Create all tasks with proper dependencies, references, and acceptance criteria
6. Verify AC coverage and mark task done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Created 30 backlog tasks (TASK-40 through TASK-69) organized into three phases:

Rust Library (aula-api crate) - 19 tasks:
- TASK-40: Workspace scaffolding (Cargo workspace, shell.nix, justfile)
- TASK-41: Error types and API response wrappers
- TASK-42: Core domain enums (136 enums)
- TASK-43: Profile/institution models
- TASK-44: Messaging/thread models
- TASK-45: Calendar/event models
- TASK-46: Presence (ComeGo) models
- TASK-47: Posts, comments, gallery, document models
- TASK-48: Notification, search, group models
- TASK-49: HTTP client with cookie/CSRF handling
- TASK-50: OIDC auth (Auth Code + PKCE)
- TASK-51: Token storage, refresh, session management
- TASK-52: Profile/config API service
- TASK-53: Messaging API service (26 methods)
- TASK-54: Calendar API service (35+ methods)
- TASK-55: Presence API service (40+ methods)
- TASK-56: Posts, comments, gallery services
- TASK-57: Documents and files services
- TASK-58: Notification, search, minor services

CLI (aula-cli crate) - 6 tasks:
- TASK-59: CLI scaffolding with clap
- TASK-60: CLI auth command (browser login)
- TASK-61: CLI messaging commands
- TASK-62: CLI calendar commands
- TASK-63: CLI presence, posts, remaining commands
- TASK-64: CLI output formatting

Tests - 5 tasks:
- TASK-65: Unit test infrastructure and model serialization tests
- TASK-66: Mock HTTP server for integration tests
- TASK-67: Integration tests for API service modules
- TASK-68: E2E test bootstrap (manual login token capture)
- TASK-69: E2E tests for key API workflows
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created 30 fine-grained backlog tasks (TASK-40 through TASK-69) for the Rust implementation phase, driven by the reverse-engineered API surface from tasks 8-12.

Scope:
- 19 Rust library tasks covering: workspace scaffolding, error types, 136 enums, 6 domain model groups (profiles, messaging, calendar, presence, posts/gallery/docs, notifications/search), HTTP client with CSRF, OIDC auth with PKCE, token management, and 7 API service modules covering all 200+ API operations
- 6 CLI tasks covering: clap scaffolding, browser-based auth login, messaging/calendar/presence/posts commands, and output formatting (JSON/table/human-readable)
- 5 test tasks covering: unit test fixtures and model deserialization, mock HTTP server, integration tests per service, E2E bootstrap with manual login, and E2E tests for key read-only workflows

Architecture decisions reflected in task structure:
- InstitutionProfileId used as the API pivot entity throughout
- Two-tier auth (Level 2 UniLogin / Level 3 MitID) modeled explicitly
- CSRF cookie-to-header flow built into HTTP client foundation
- ComeGo treated as a major subsystem with dedicated model and service tasks
- E2E tests designed to skip gracefully without auth tokens (no CI dependency on real credentials)

All tasks have proper dependency chains, reference the analysis files as context, and are scoped to approximately one PR each.
<!-- SECTION:FINAL_SUMMARY:END -->
