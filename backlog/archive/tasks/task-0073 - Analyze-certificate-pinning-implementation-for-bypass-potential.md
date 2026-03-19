---
id: TASK-0073
title: Analyze certificate pinning implementation for bypass potential
status: To Do
assignee: []
created_date: '2026-03-18 22:00'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Decompilation revealed certificate pinning in CertificatePinningUtils.cs and X509CertificateHelper.cs with specific SHA-256 pin hashes for aula.dk and ncaula.com domains. Analyze whether the pinning can be bypassed for traffic interception during runtime analysis. Pins: /P3+fgXhRH6jPoKBMmAKWRrtjDoEZf4ySjxLoQuqsYc=, eLCo7AWQ2P88/2FQfow993oOhcjXal2sS/e2mZgJLJE=, 9XtneGQWNOLQFi0f8LEJ62bt1f/pVrCb4ytT66RcurA= for aula.dk.
<!-- SECTION:DESCRIPTION:END -->
