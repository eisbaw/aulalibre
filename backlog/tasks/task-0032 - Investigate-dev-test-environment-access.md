---
id: TASK-0032
title: Investigate dev/test environment access
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 15:24'
updated_date: '2026-03-19 19:28'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Non-production environments use basic auth credentials aula-user:Aula-1337. Test environments at ncaula.com domain. Could be useful for development testing.
<!-- SECTION:DESCRIPTION:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing research (TASK-20, ncaula_staging_domain.md) - DONE
2. Probe portal/backend hosts with curl + basic auth
3. Probe data API hosts (app-private-dev, app-staff-dev)
4. Probe staging/api/www.ncaula.com
5. Try API endpoint patterns on responsive hosts
6. Document findings in task final summary
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Probe Results (2026-03-19)

### Portal/Backend Hosts (ncaula.com)

| Environment | Host | HTTP Status | Basic Auth | Notes |
|---|---|---|---|---|
| TEST1 | www1-test1.ncaula.com | 302->200 (portal), 403 (API=Login required) | Accepted | Full PHP stack traces in errors |
| TEST3 | www1-test3.ncaula.com | 403 (API=Login required) | Accepted | Full debug traces |
| DEV1 | www1-dev1.ncaula.com | 403 (API=Login required) | Accepted | Full debug traces |
| DEV3 | www1-dev3.ncaula.com | 403 (API=Login required) | Accepted | Full debug traces |
| DEV11 | www1-dev11.ncaula.com | 403 (API=Login required) | Accepted | Full debug traces |
| DEV21 | www1-dev21.ncaula.com | 401 | Rejected | Different realm "Registry Authentication" |
| DEV22 | www1-dev22.ncaula.com | 401 | Rejected | Same as DEV21 |
| DEV31 | www1-dev31.ncaula.com | 403 (API=Login required) | Accepted | Full debug traces |
| DEV32 | www1-dev32.ncaula.com | 410 (Unsupported API v23) | Accepted | Running older API version |

### Unreachable Hosts (timeout)
- staging.ncaula.com
- api.ncaula.com
- www.ncaula.com
- www1-hotfix.ncaula.com
- www1-preprod.aula.dk
- www1-hotfix.aula.dk

### Data/Notification Hosts
- www1-app-private-dev.ncaula.com: 200 (S3/CloudFront static page, redirects to auth)
- www1-app-private-test1.ncaula.com: 200 (same pattern)

### Key Findings
1. Test envs leak full PHP stack traces (file paths, class names, method args)
2. Production returns "trace": "n/a" and empty error messages
3. Basic auth (aula-user:Aula-1337) accepted on 7/9 portal hosts
4. DEV21/DEV22 use different credentials ("Registry Authentication" realm)
5. All APIs still require OIDC session login after basic auth
6. TLS cert: *.ncaula.com wildcard, Amazon RSA 2048, valid Jan 2026 - Feb 2027
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
7 of 9 dev/test hosts at ncaula.com are publicly reachable with basic auth aula-user:Aula-1337. However, all API calls still return 448 (login required) - basic auth is only the infrastructure gate, full OIDC login with valid test credentials is needed. Debug traces with PHP stack traces, file paths, class names are exposed on dev/test (not on prod). Staging/preprod/hotfix hosts are unreachable (likely VPN-only). Environments are NOT practically usable without valid test OIDC credentials.
<!-- SECTION:FINAL_SUMMARY:END -->
