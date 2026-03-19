---
id: TASK-0087
title: Investigate BasicAuth credentials embedded in Conf class
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-19 05:59'
updated_date: '2026-03-19 07:42'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AulaPortalWebView auto-responds to HTTP Basic Auth challenges with Conf.BasicAuthUserName and Conf.BasicAuthUserPassword. These same credentials appear in the LOGOUT_DEV_URL and HttpClientManager.CreateBasicAuthorizationHeaderValue(). Investigate: what are these credentials (hardcoded or fetched?), are they the same for all environments, and what do they protect (portal-level vs user-level access). This is relevant to security analysis since any in-app widget WebView can trigger a Basic Auth challenge and receive these credentials.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Locate where BasicAuth credentials are defined (Conf class, EnvironmentFactory, etc.) and determine if hardcoded or fetched
- [x] #2 Map all code paths that use/inject BasicAuth credentials (WebView, HttpClientManager, logout URLs)
- [x] #3 Determine whether credentials differ per environment (prod vs dev/test)
- [x] #4 Identify what endpoints/resources are protected by BasicAuth
- [x] #5 Assess security implications of embedded credentials and WebView auto-response
- [x] #6 Document findings in security_analysis or architecture notes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Trace all BasicAuth credential definitions in Conf.cs
2. Map all injection points: HttpClientManager constructor, SimpleService.GenericGetRequest, IsAliveService.AulaIsAlive, AulaPortalWebView.OnReceivedHttpAuthRequest, Urls.LOGOUT_DEV_URL
3. Determine environment-gating logic (prod vs non-prod)
4. Assess security implications of each injection path
5. Update security_analysis.md with detailed findings
6. Commit and mark done
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Traced all 5 BasicAuth injection points in the decompiled C# code:
1. HttpClientManager constructor (default headers on singleton HttpClient)
2. SimpleService.GenericGetRequest (separate HttpClient for unauthenticated GETs)
3. IsAliveService.AulaIsAlive (health check with own HttpClient)
4. AulaPortalWebView.OnReceivedHttpAuthRequest (WebView auto-response to 401 challenges)
5. Urls.LOGOUT_DEV_URL (credentials embedded in URL)

Key finding: all paths are gated by EnvConfig.IsProduction -- production returns empty strings.
PREPROD and HOTFIX are marked isProduction:true so they also get empty credentials.
WebView auto-response is architecturally concerning even though credentials are empty in prod.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated BasicAuth credentials embedded in the Conf class (aula-user:Aula-1337) and traced all code paths that use them.

Findings:
- Credentials are defined in Conf.BasicAuthUserName/BasicAuthUserPassword, gated by EnvConfig.IsProduction
- Production (PROD, PREPROD, HOTFIX) returns empty strings; non-production (TEST*, DEV*, CI) returns the hardcoded credentials
- Five injection points identified: HttpClientManager default headers, SimpleService.GenericGetRequest, IsAliveService health check, AulaPortalWebView OnReceivedHttpAuthRequest auto-response, and Urls.LOGOUT_DEV_URL
- The credentials protect infrastructure-level access to non-production *.ncaula.com environments, not user-level authentication
- In production, the Authorization header carries "Basic Og==" (base64 of ":") which the server ignores

Security assessment: Low severity. Production is unaffected. The WebView OnReceivedHttpAuthRequest handler is architecturally concerning because it auto-responds to ALL Basic Auth challenges regardless of origin, though credentials are empty in production.

Updated security_analysis.md section 6.1 with comprehensive analysis and expanded the concerns table entry.
<!-- SECTION:FINAL_SUMMARY:END -->
