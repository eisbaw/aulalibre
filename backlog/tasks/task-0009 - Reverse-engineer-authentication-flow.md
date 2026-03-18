---
id: TASK-0009
title: Reverse engineer authentication flow
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:32'
updated_date: '2026-03-18 15:25'
labels: []
dependencies:
  - TASK-0008
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Trace the complete authentication flow: login screen -> credential submission -> token acquisition -> token refresh -> session management. Identify OAuth2/OIDC providers, SAML, or custom auth. Understand how tokens are stored and refreshed.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Login flow documented step by step (UI -> network calls)
- [x] #2 Auth provider identified (OAuth2, OIDC, SAML, custom)
- [x] #3 Token format identified (JWT, opaque, etc.)
- [x] #4 Token storage mechanism documented (SharedPreferences, KeyStore, etc.)
- [x] #5 Token refresh mechanism documented
- [x] #6 Session expiry and re-auth behavior documented
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Analyze .NET assemblies using ikdasm/monodis for auth-related classes
2. Trace OIDC configuration: authority, client IDs, scopes, endpoints
3. Trace login flow: LoginPageViewModel -> AuthenticationManager -> OidcClient
4. Document token storage: SecureStorageHelper -> MAUI Essentials SecureStorage
5. Document token refresh: RefreshTokenAsync via IdentityModel.OidcClient
6. Document session management: SessionPromptManager, CSRF tokens, keep-alive
7. Write auth_flow.md with complete findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Used ikdasm to fully decompile assembly_187_AulaNative.dll (777K lines IL)
- Found OIDC provider is SimpleSAMLphp at login.aula.dk
- Extracted exact client IDs, scopes, and endpoint URLs from IL
- Traced CreateOidcClient() method showing OidcClientOptions configuration
- Found two auth levels: level 2 (UniLogin, scope=aula) and level 3 (MitID, scope=aula-sensitive)
- Token storage uses MAUI Essentials SecureStorage via SecureStorageHelper
- Session management uses CSRF tokens (Csrfp-Token cookie/header) and keep-alive endpoint
- Found all environment configs including dev/test credentials (aula-user:Aula-1337)
- PIN and biometric login reuse persisted tokens without OIDC round-trip
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Complete reverse engineering of Aula authentication flow from .NET assembly IL analysis.

Key findings:
- OIDC Authorization Code + PKCE flow against SimpleSAMLphp at login.aula.dk
- Two auth levels: level 2 (UniLogin, client_id=_742adb5e..., scope=aula) and level 3 (MitID, client_id=_99949a54..., scope=aula-sensitive)
- Authorize endpoint: /simplesaml/module.php/oidc/authorize.php
- Token endpoint: /simplesaml/module.php/oidc/token.php
- Redirect URI: https://app-private.aula.dk
- Tokens stored via MAUI Essentials SecureStorage (Android Keystore-backed)
- CSRF protection via Csrfp-Token cookie sent as csrfp-token header
- Session keep-alive endpoint, countdown timer with user warning
- PIN/biometric login reuses persisted tokens without full OIDC round-trip
- No client-side JWT validation (empty KeySet)
- 8 environment configs found including dev credentials

Deliverable: auth_flow.md with complete documentation for Rust library implementation
<!-- SECTION:FINAL_SUMMARY:END -->
