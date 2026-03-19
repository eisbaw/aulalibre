---
id: TASK-0036
title: Analyze widget SSO token mechanism
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:50'
updated_date: '2026-03-19 05:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Widgets get authenticated via GetAulaToken which provides an SSO token. This is interesting from a security perspective -- how are third-party widgets authenticated, what data do they receive, and what is the token scope? The WidgetNotification system also sends push notifications from widgets.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Document the SSO token acquisition flow (API endpoint, request/response)
- [x] #2 Document the SSO token injection method (how token reaches third-party widget)
- [x] #3 Document the data exposed to widgets via SSO parameters (user identity, children, institutions, CSRF)
- [x] #4 Document the two widget rendering paths (in-app WebView vs external browser)
- [x] #5 Document security observations (token in URL query params, CSRF token exposure, BasicAuth in WebView)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Trace GetAulaToken API call through WidgetService -> WidgetServiceManager -> WidgetDto
2. Analyze SsoWidgetDirectLinkArguments to identify all data passed to widgets
3. Analyze WidgetWebViewFragment vs Browser.OpenAsync rendering paths
4. Analyze AulaPortalWebView BasicAuth and URL interception
5. Document findings in widget_sso_analysis.md
6. Note security observations
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed the widget SSO token mechanism in the Aula app. Documented the full flow from token acquisition (aulaToken.getAulaToken API) through injection (URL query parameters via SsoWidgetDirectLinkArguments) to rendering (two paths: in-app WebView for non-SSO widgets, external browser for SSO widgets).

Key findings:
- SSO tokens are widget-specific, acquired per widget ID from the backend
- Tokens plus user context (sessionUuid, childFilter, institutionFilter, CSRF token, portalRole, assuranceLevel) are passed as URL query parameters to third-party widget URLs
- Non-SSO widgets render in an AulaPortalWebView with auto-BasicAuth and JS enabled
- SSO widgets open in the system browser with no app-level security controls
- Security concerns: token/PII exposure via URL params (browser history, referer leaks, server logs), CSRF token forwarding to third parties, hardcoded assuranceLevel="3"

Produced: widget_sso_analysis.md
<!-- SECTION:FINAL_SUMMARY:END -->
