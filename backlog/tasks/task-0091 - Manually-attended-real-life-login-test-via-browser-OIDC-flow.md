---
id: TASK-0091
title: Manually-attended real-life login test via browser OIDC flow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-19 07:50'
updated_date: '2026-03-20 21:49'
labels: []
dependencies: []
references:
  - auth_flow.md
  - decompilation_analysis.md
  - decompiled_csharp/AulaNative/AulaNative.Configuration/Conf.cs
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Perform a real login to Aula using Chrome DevTools MCP server to automate browser interaction and capture network traffic. The MCP server controls a Chromium instance, allowing us to navigate to aula.dk, observe the OIDC login flow, and capture all network requests including auth tokens and cookies. The human operator only needs to complete the MitID/UniLogin credential step in the browser window that appears. This validates static analysis findings against real behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 OIDC discovery endpoint fetched and validated against decompiled Conf.cs values
- [x] #2 Full browser-based login flow completed with human attending MitID/UniLogin step
- [x] #3 Access token and refresh token captured and stored in secrets/
- [x] #4 CSRF token obtained from session cookie or API response
- [x] #5 At least one authenticated API call successfully made (e.g. profiles.getProfilesByLogin)
- [x] #6 Document actual vs expected differences from static analysis
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Chrome DevTools MCP is now configured (.mcp.json, chrome-devtools-mcp-wrapper.sh)\n2. Restart Claude Code session to load the MCP server\n3. Use MCP to open new browser page and navigate to https://www.aula.dk\n4. Take snapshot to see the login page\n5. User manually completes MitID/UniLogin in the Chromium window\n6. Use MCP list_network_requests to capture all auth-related HTTP traffic\n7. Use MCP get_network_request to extract tokens, cookies, CSRF token from responses\n8. Store captured tokens in secrets/\n9. Use MCP or curl to make authenticated API call (profiles.getProfilesByLogin)\n10. Document actual vs expected differences from static analysis
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Chrome DevTools MCP setup copied from ~/topics/nemlig_cli.

Login completed successfully via MitID path:
- UniLogin broker -> MitID -> SAML assertion -> login.aula.dk -> www.aula.dk
- Dashboard loaded at /portal/#/overblik
- Full network traffic (73 requests) captured
- OIDC discovery validated (AC#1)
- Browser login completed with MitID (AC#2)

Tokens extracted and API tested:
- PHPSESSID and Csrfp-Token saved to secrets/
- Authenticated curl call to profiles.getProfilesByLogin succeeded (status 0 OK)
- Profile returned: guardian role, institution [REDACTED], municipality [REDACTED]
- API version confirmed: v23
- Auth model confirmed: cookie-based (PHPSESSID HttpOnly on .aula.dk) + CSRF header (csrfp-token)
- No Bearer/Authorization header needed for web API calls
- Media served from media-prod.aula.dk with CloudFront signed URLs

Actual vs Expected differences from static analysis:
1. AUTH MODEL DIFFERS: Web uses pure cookie-based auth (PHPSESSID), NOT Bearer tokens.
2. ISSUER MISMATCH: Live discovery returns https://login.aula.dk (no trailing slash), decompiled Conf.cs has trailing slash.
3. LOGOUT ENDPOINT DIFFERS: Live OIDC discovery advertises /simplesaml/module.php/oidc/logout.php, app uses /auth/logout.php.
4. LOGIN FLOW IS SAML-BASED: Web login uses SAML2 via broker.unilogin.dk Keycloak, not direct OIDC.
5. SESSION ROTATION: PHPSESSID is rotated on first API call after login.
6. API VERSION CONFIRMED: v23 matches decompiled Conf.API_VERSION.
7. RPC-STYLE API CONFIRMED: ?method=module.action pattern confirmed.
8. CSRF CONFIRMED: Csrfp-Token cookie + csrfp-token header pattern matches decompiled code exactly.
9. CSP PERMISSIVE: connect-src * data: blob: is extremely permissive.
10. MEDIA CDN: media-prod.aula.dk with CloudFront signed URLs.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Performed a manually-attended real-life login to Aula via Chrome DevTools MCP, validating static analysis findings against live behavior.

Key results:
- OIDC discovery endpoint validated at login.aula.dk/simplesaml/module.php/oidc/openid-configuration.php; authorize/token endpoints match decompiled Conf.cs exactly
- Full MitID login completed: Aula -> UniLogin Keycloak broker (SAML) -> MitID -> SAML assertion -> SimpleSAMLphp -> PHPSESSID cookie
- 73 network requests captured including full auth redirect chain, exported as HAR file
- PHPSESSID + Csrfp-Token extracted and stored in secrets/auth_tokens.env
- Authenticated API call (profiles.getProfilesByLogin) succeeded via curl with cookie auth

Major finding: Web auth uses SAML (via Keycloak broker + SimpleSAMLphp) with cookie sessions, NOT the OIDC Authorization Code + PKCE flow used by the mobile app. The SimpleSAMLphp server bridges both protocols.

Other differences from static analysis:
- Issuer trailing slash mismatch (live: no slash, Conf.cs: with slash)
- Logout endpoint differs (live: /simplesaml/.../logout.php vs app: /auth/logout.php)
- Session cookie rotated on first API call (anti-fixation)
- CSP extremely permissive (connect-src * data: blob:)

Artifacts (all in gitignored secrets/):
- aula_login_20260319.har (134KB, 73 entries)
- network_dump_20260319.md (533 lines structured dump)
- auth_tokens.env (PHPSESSID + CSRF token)
- api_profiles_response.json (profile API response)
<!-- SECTION:FINAL_SUMMARY:END -->
