---
id: TASK-0026
title: Investigate OIDC authentication flow via app-private.aula.dk
status: To Do
assignee: []
created_date: '2026-03-18 14:35'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The app uses IdentityModel.OidcClient for OpenID Connect auth with app-private.aula.dk as the callback host. Investigate the full auth flow: discover the OIDC provider endpoint, check if UniLogin/MitID is the identity provider, analyze token handling and session management.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 OIDC discovery endpoint identified
- [ ] #2 Identity provider (UniLogin/MitID/other) confirmed
- [ ] #3 Token storage and refresh mechanism documented
<!-- AC:END -->
