---
id: TASK-0071
title: Investigate OAuth2 client IDs and step-up authentication flow
status: To Do
assignee: []
created_date: '2026-03-18 21:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Decompilation revealed two OAuth2 client IDs for different security levels: step-level-2 (scope 'aula') and step-level-3 (scope 'aula-sensitive'). The SimpleSAMLphp OIDC flow uses login.aula.dk. Worth investigating what triggers step-up auth and what endpoints require it (e.g., sensitive messaging, secure documents). Client IDs: _742adb5e2759028d86dbadf4af44ef70e8b1f407a6 and _99949a54b8b65423862aac1bf629599ed64231607a.
<!-- SECTION:DESCRIPTION:END -->
