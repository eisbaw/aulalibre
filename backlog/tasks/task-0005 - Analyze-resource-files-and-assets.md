---
id: TASK-0005
title: Analyze resource files and assets
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:36'
labels: []
dependencies:
  - TASK-0001
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Examine res/, assets/, and other resource directories. Identify configuration files, embedded databases, certificate pinning configs, network security configs, and any hardcoded URLs or API endpoints in resource XML/JSON.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Network security config analyzed (certificate pinning, trusted CAs)
- [x] #2 All hardcoded URLs and API base URLs extracted from resources
- [x] #3 Embedded databases or data files catalogued
- [x] #4 String resources scanned for API keys, endpoints, feature flags
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Find and analyze network_security_config.xml for cert pinning
2. Search all XML/JSON resources for URLs, API endpoints, domains
3. Scan assets/ for embedded databases, data files, config files
4. Scan string resources (res/values/) for API keys, endpoints, feature flags
5. Check AndroidManifest.xml for relevant config refs
6. Document findings in analysis/ directory
7. Create follow-up tasks for tangents
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- No network_security_config.xml found; no cert pinning at Android layer
- Found Firebase config: project aula-private, API key, storage bucket, database URL
- Found OAuth web client ID for Google/Firebase
- Deep links: app-private.aula.dk (OIDC callback), OneDrive/Google Drive redirects
- .NET assemblies packed in blob - business logic URLs not extractable via strings
- IdentityModel.OidcClient.dll confirms OIDC auth flow
- No embedded databases; SQLite used at runtime only
- Created follow-up tasks: TASK-25 (assembly extraction), TASK-26 (OIDC flow), TASK-27 (Firebase)
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed all resource files and assets in the Aula APK (v2.15.4).

Key findings:
- No network_security_config.xml present; no Android-level certificate pinning
- Firebase project "aula-private" with Realtime Database, Cloud Messaging, Storage
- Google/Firebase API key: AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc
- OAuth callback deep link: https://app-private.aula.dk (OIDC via IdentityModel.OidcClient)
- Staging domain: *.ncaula.com referenced in manifest query intents
- Cloud storage integration: OneDrive and Google Drive OAuth redirects
- No embedded databases; SQLite present for runtime use
- All business logic URLs are in compressed .NET assemblies (not extractable via strings)

Produced: resource_analysis.md with full findings
Created follow-up tasks: TASK-25 (assembly extraction), TASK-26 (OIDC investigation), TASK-27 (Firebase investigation)
<!-- SECTION:FINAL_SUMMARY:END -->
