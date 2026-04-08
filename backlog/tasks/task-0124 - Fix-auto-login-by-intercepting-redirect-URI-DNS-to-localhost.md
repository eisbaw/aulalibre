---
id: TASK-0124
title: Fix auto-login by intercepting redirect URI DNS to localhost
status: To Do
assignee: []
created_date: '2026-04-08 07:07'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The OIDC provider only accepts redirect_uri=https://app-private.aula.dk. The auto login flow needs to intercept this redirect so it resolves to our local callback server while keeping the domain name intact in the URL. Investigate approaches: LD_PRELOAD DNS interception (e.g. override getaddrinfo to resolve app-private.aula.dk to 127.0.0.1 for a child browser process), mitmproxy with custom DNS, /etc/hosts manipulation, or a local DNS resolver. The goal is a seamless browser-based login where the OIDC redirect lands on our localhost server without the user needing to copy-paste URLs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Chosen approach documented with trade-offs (LD_PRELOAD vs mitmproxy vs hosts file vs local DNS)
- [ ] #2 Implementation resolves app-private.aula.dk to 127.0.0.1 for the browser subprocess only
- [ ] #3 OIDC redirect_uri remains https://app-private.aula.dk (accepted by provider)
- [ ] #4 Local callback server handles the redirected request and extracts auth code
- [ ] #5 TLS handled: either terminate TLS locally with a self-signed cert or strip to HTTP
- [ ] #6 Works without root/sudo privileges
- [ ] #7 --wip-auto flag activates the new flow
- [ ] #8 README documents the approach and any prerequisites
<!-- AC:END -->
