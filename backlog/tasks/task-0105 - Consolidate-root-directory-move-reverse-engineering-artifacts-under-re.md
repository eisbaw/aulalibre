---
id: TASK-0105
title: 'Consolidate root directory: move reverse engineering artifacts under re/'
status: Done
assignee:
  - '@claude'
created_date: '2026-03-20 21:24'
updated_date: '2026-03-20 21:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The root directory has ~45 visible items including 25+ reverse engineering analysis docs, extraction scripts, and PRD files scattered at the top level. Clean up by moving all RE artifacts under a new re/ directory, keeping root clean for the Rust project, config, and docs/.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Create re/ directory for all reverse engineering artifacts
- [x] #2 Move all RE analysis .md files (api_endpoints.md, architecture.md, auth_flow.md, data_models.md, domain_concepts.md, milestone2_analysis.md, decompilation_analysis.md, dex_catalog.md, dex_decompilation_analysis.md, assembly_inventory.md, crc64_namespace_mapping.md, resource_analysis.md, native_library_inventory.md, firebase_analysis.md, firebase_analytics_transport.md, cloud_storage_integration.md, datastore_protobuf_analysis.md, notification_messaging.md, messagingcenter_event_bus.md, keepalive_polling_analysis.md, preference_storage_analysis.md, security_analysis.md, document_locking_analysis.md, esdh_journaling_analysis.md, magtanvendelse_analysis.md, locale_analysis.md, module_widget_configuration.md, widget_sso_analysis.md, ncaula_staging_domain.md, task24_jadx_classes2_analysis.md) into re/
- [x] #3 Move RE scripts (apk_extract.sh, extract_assemblies.py, extract_all_assemblies.py, decompile_assemblies.sh, assemblies_to_decompile.txt) into re/
- [x] #4 Move PRD/genesis files (prd.apk_decompile.md, original_prd_prompt.md) into re/
- [x] #5 Move chrome-devtools-mcp-wrapper.sh into scripts/
- [x] #6 Update .gitignore paths if any gitignored dirs/patterns need adjusting for the new layout
- [x] #7 Remove empty reports/ directory
- [x] #8 Root level after cleanup contains only: aula/, backlog/, docs/, re/, scripts/, secrets/, CLAUDE.md, Justfile, shell.nix, .gitignore, .mcp.json
- [x] #9 Reserve docs/ for Rust codebase documentation (do not mix RE docs in)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Create re/ directory
2. git mv all RE analysis .md files into re/
3. git mv RE scripts into re/
4. git mv PRD/genesis files into re/
5. git mv chrome-devtools-mcp-wrapper.sh into scripts/
6. Update .gitignore: prefix gitignored artifact paths with re/ where needed
7. Remove empty reports/ directory
8. Verify root contents match AC #8 (note: no docs/ per user instruction)
9. Run just e2e to verify nothing broken
10. Mark ACs complete and finalize task
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Moved 30 RE analysis .md files to re/
- Moved 5 RE scripts to re/
- Moved 2 PRD/genesis files to re/
- Moved chrome-devtools-mcp-wrapper.sh to scripts/
- Moved all gitignored RE artifacts (apktool_out, extracted dirs, .xapk, decompiled dirs, etc.) to re/
- Moved .chrome-profile to re/
- Removed empty reports/ directory
- Updated .gitignore: all RE artifact patterns now prefixed with re/
- Updated .mcp.json: chrome-devtools wrapper path updated to scripts/
- Updated CLAUDE.md: key file paths updated, removed references to non-existent milestone3 files
- All e2e tests pass (build, test, lint, fmt-check)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Consolidated root directory by moving all reverse engineering artifacts under re/.

Changes:
- Moved 30 RE analysis .md files from root to re/
- Moved 5 RE scripts (apk_extract.sh, extract_assemblies.py, etc.) to re/
- Moved PRD/genesis files (prd.apk_decompile.md, original_prd_prompt.md) to re/
- Moved chrome-devtools-mcp-wrapper.sh to scripts/
- Physically relocated all gitignored RE artifacts (apktool_out, decompiled dirs, extracted assemblies, .xapk, jadx output, .chrome-profile, etc.) to re/
- Removed empty reports/ directory
- Updated .gitignore: all RE artifact patterns now prefixed with re/; removed stale root-level patterns
- Updated .mcp.json: chrome-devtools wrapper path -> scripts/
- Updated CLAUDE.md: key file paths corrected, removed references to non-existent milestone3 files

Root now contains only: aula/, backlog/, re/, scripts/, secrets/, CLAUDE.md, Justfile, shell.nix, .gitignore, .mcp.json

Tests: just e2e passes (build, test, lint, fmt-check)
<!-- SECTION:FINAL_SUMMARY:END -->
