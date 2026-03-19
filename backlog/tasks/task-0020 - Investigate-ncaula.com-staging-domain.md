---
id: TASK-0020
title: Investigate ncaula.com staging domain
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 14:06'
updated_date: '2026-03-19 05:27'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AndroidManifest queries for *.ncaula.com URLs, which appears to be a Netcompany internal/staging domain for Aula. Investigate whether this domain is accessible, what endpoints it exposes, and whether there are any hardcoded staging URLs or API keys in the app.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ncaula.com domain purpose documented
- [x] #2 Any hardcoded URLs referencing ncaula.com identified in app resources
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search all decompiled source code for ncaula.com references
2. Read EnvironmentFactory.cs, Conf.cs, EnvironmentConfig.cs for full config
3. Check AndroidManifest.xml for ncaula.com references
4. DNS probing of ncaula.com and all discovered subdomains
5. Document all findings in decompilation_analysis.md or appropriate location
6. Run just e2e before committing
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Documented ncaula.com as Netcompany's internal staging/development domain for Aula, hosted on AWS eu-west-1.

Changes:
- Created ncaula_staging_domain.md with complete analysis: domain purpose, full 13-environment matrix (6 host fields each), DNS resolution results, certificate pins, hardcoded basic auth credentials, and AndroidManifest intent filter.
- Updated auth_flow.md environment table from 8 to 13 entries, added IsProduction column, and cross-referenced the detailed analysis.

Key findings:
- 10 non-production environments use ncaula.com (test1, test3, dev1/3/11/21/22/31/32, CI)
- PREPROD/HOTFIX are hybrid: aula.dk portal + ncaula.com data hosts
- All envs share www1-app-private-dev.ncaula.com for data APIs
- Non-production gated by basic auth (aula-user:Aula-1337)
- Separate cert pin set (3 SHA-256 hashes) for ncaula.com
- All subdomains resolve to live AWS infrastructure

Tests: just e2e passes (453+78 tests, clippy, fmt).
<!-- SECTION:FINAL_SUMMARY:END -->
