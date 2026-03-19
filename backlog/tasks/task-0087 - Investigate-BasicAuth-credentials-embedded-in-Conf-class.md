---
id: TASK-0087
title: Investigate BasicAuth credentials embedded in Conf class
status: To Do
assignee: []
created_date: '2026-03-19 05:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The AulaPortalWebView auto-responds to HTTP Basic Auth challenges with Conf.BasicAuthUserName and Conf.BasicAuthUserPassword. These same credentials appear in the LOGOUT_DEV_URL and HttpClientManager.CreateBasicAuthorizationHeaderValue(). Investigate: what are these credentials (hardcoded or fetched?), are they the same for all environments, and what do they protect (portal-level vs user-level access). This is relevant to security analysis since any in-app widget WebView can trigger a Basic Auth challenge and receive these credentials.
<!-- SECTION:DESCRIPTION:END -->
