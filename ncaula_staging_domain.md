# ncaula.com Staging Domain Analysis

Investigation of the `ncaula.com` domain referenced in the Aula Android app (TASK-20).

## Domain Purpose

`ncaula.com` is Netcompany's internal staging/development domain for the Aula platform.
The name derives from **N**et**c**ompany + **Aula**. It hosts non-production environments
used for development, testing, and CI pipelines.

- **Registrar**: AWS Route 53 (nameservers: `awsdns-*.com/net/org/co.uk`)
- **Hosting**: AWS eu-west-1 (Ireland) -- ELBs and direct EC2/ECS IPs
- **Mail**: AWS SES (`inbound-smtp.eu-west-1.amazonaws.com`)
- **SPF**: `v=spf1 mx -all` (strict, mail only from MX)

## Environment Matrix

Source: `AulaNative.Configuration.EnvironmentFactory` (decompiled from AulaNative.dll).

Each `EnvironmentConfig` has 6 host fields:

| Field | Purpose |
|-------|---------|
| `BackendHost` | Main portal/API server |
| `AuthHost` | OAuth2/OIDC authentication server |
| `PrivateDataHost` | Data API for private (parent) app |
| `StaffDataHost` | Data API for staff app |
| `PrivateNotificationsHost` | Push notification backend for private app |
| `StaffNotificationsHost` | Push notification backend for staff app |

### Full Environment Table

| Environment | Backend Host | Auth Host | Private Data Host | Staff Data Host | Private Notif Host | Staff Notif Host | Env Name | IsProduction |
|---|---|---|---|---|---|---|---|---|
| PROD | `www.aula.dk` | `login.aula.dk` | `app-private.aula.dk` | `app-staff.aula.dk` | `app-private.aula.dk` | `app-staff.aula.dk` | production | true |
| PREPROD | `www1-preprod.aula.dk` | `login-preprod.aula.dk` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | preprod | true |
| HOTFIX | `www1-hotfix.aula.dk` | `login-hotfix.aula.dk` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | hotfix | true |
| TEST1 | `www1-test1.ncaula.com` | `www1-test1.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-test1.ncaula.com` | `www1-app-staff-test1.ncaula.com` | test1 | false |
| TEST3 | `www1-test3.ncaula.com` | `www1-test3.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-test3.ncaula.com` | `www1-app-staff-test3.ncaula.com` | test3 | false |
| DEV1 | `www1-dev1.ncaula.com` | `www1-dev1.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev1.ncaula.com` | `www1-app-staff-dev1.ncaula.com` | dev1 | false |
| DEV3 | `www1-dev3.ncaula.com` | `www1-dev3.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev3.ncaula.com` | `www1-app-staff-dev3.ncaula.com` | dev3 | false |
| DEV11 | `www1-dev11.ncaula.com` | `www1-dev11.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev11.ncaula.com` | `www1-app-staff-dev11.ncaula.com` | dev11 | false |
| DEV21 | `www1-dev21.ncaula.com` | `www1-dev21.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev21.ncaula.com` | `www1-app-staff-dev21.ncaula.com` | dev21 | false |
| DEV22 | `www1-dev22.ncaula.com` | `www1-dev22.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev22.ncaula.com` | `www1-app-staff-dev22.ncaula.com` | dev22 | false |
| DEV31 | `www1-dev31.ncaula.com` | `www1-dev31.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev31.ncaula.com` | `www1-app-staff-dev31.ncaula.com` | dev31 | false |
| DEV32 | `www1-dev32.ncaula.com` | `www1-dev32.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev32.ncaula.com` | `www1-app-staff-dev32.ncaula.com` | dev32 | false |
| CI | `www1-dev1.ncaula.com` | `www1-dev1.ncaula.com` | `www1-app-private-dev.ncaula.com` | `www1-app-staff-dev.ncaula.com` | `www1-app-private-dev1.ncaula.com` | `www1-app-staff-dev1.ncaula.com` | dev1 | false |

Note: CI reuses the DEV1 configuration entirely.

### Observations

1. **PREPROD and HOTFIX are hybrid**: backend/auth on `aula.dk`, but data and notification hosts on `ncaula.com`. This means pre-production uses production-like portal URLs but development data backends. The `isProduction: true` flag means they skip basic auth but still use the ncaula.com cert pins for data host connections.

2. **Shared data hosts**: All non-production environments share `www1-app-private-dev.ncaula.com` and `www1-app-staff-dev.ncaula.com` as their `PrivateDataHost` and `StaffDataHost`. Only the notification hosts are environment-specific.

3. **Self-hosted auth for test/dev**: Unlike production (which uses `login.aula.dk`), all test/dev environments use the same host for both backend and auth (e.g., `www1-test1.ncaula.com` for both).

## DNS Resolution (March 2026)

All discovered subdomains resolve to live AWS infrastructure:

| Subdomain | Resolves To |
|---|---|
| `staging.ncaula.com` | `preprod-aula-drupal-fargate-lb-*.eu-west-1.elb.amazonaws.com` |
| `api.ncaula.com` | Same ELB as staging |
| `www.ncaula.com` | Same ELB as staging |
| `www1-test1.ncaula.com` | `test1-aula-portal-lb-*.eu-west-1.elb.amazonaws.com` |
| `www1-test3.ncaula.com` | Direct IPs (52.212.x, 3.248.x) |
| `www1-dev1.ncaula.com` | Direct IPs (34.246.x, 54.155.x) |
| `www1-dev3.ncaula.com` | Direct IPs (52.16.x, 52.214.x) |
| `www1-dev11.ncaula.com` | Direct IPs (54.155.x, 52.213.x) |
| `www1-dev21.ncaula.com` | Direct IPs (52.212.x, 79.125.x) |
| `www1-dev22.ncaula.com` | Direct IPs (52.30.x, 52.212.x) |
| `www1-dev31.ncaula.com` | Direct IPs (52.30.x, 108.132.x) |
| `www1-dev32.ncaula.com` | Direct IPs (34.242.x, 18.200.x) |
| `www1-hotfix.ncaula.com` | Same preprod ELB |
| `www1-preprod.ncaula.com` | No DNS record (NXDOMAIN) |
| `www1-app-private-dev.ncaula.com` | CloudFront-like IPs (3.174.18.x) |
| `www1-app-staff-dev.ncaula.com` | CloudFront-like IPs (3.174.18.x) |
| `www1-app-private-test1.ncaula.com` | CloudFront-like IPs (3.174.18.x) |
| `www1-app-staff-test1.ncaula.com` | CloudFront-like IPs (3.174.18.x) |

Notable: `www1-preprod.ncaula.com` does not resolve -- the PREPROD environment uses `www1-preprod.aula.dk` for its backend, not ncaula.com.

The `app-private-*` and `app-staff-*` subdomains resolve to what appear to be CloudFront edge IPs (3.174.18.x range), suggesting the data/notification APIs may sit behind a CDN or API gateway.

## Hardcoded Credentials for Non-Production

From `Conf.cs`, non-production environments (`IsProduction == false`) use HTTP Basic Auth:

- **Username**: `aula-user`
- **Password**: `Aula-1337`

This is applied as a gate before any other authentication. Production environments return empty strings for these fields.

## Certificate Pinning

From `Conf.GetCertPublicKeys()` in `Conf.cs`:

**For ncaula.com** (when `BackendUrl` contains `ncaula.com`):
- `ejsQt33CcKZWEoO/ym2mcdSynXrVfK1o6QbTI868tDE=`
- `PfUUWB6dvdMA9exWlx0W+6lKT540ElcRWUERcBRtP6o=`
- `CC09RfvRZQ1z+bj1VeJ/jrYOeH3D0epyQR+FEXLddF8=`

**For aula.dk** (production):
- `/P3+fgXhRH6jPoKBMmAKWRrtjDoEZf4ySjxLoQuqsYc=`
- `eLCo7AWQ2P88/2FQfow993oOhcjXal2sS/e2mZgJLJE=`
- `9XtneGQWNOLQFi0f8LEJ62bt1f/pVrCb4ytT66RcurA=`

## AndroidManifest.xml Reference

The manifest includes `*.ncaula.com` in `<queries>` as a browsable HTTPS intent filter:

```xml
<intent>
    <action android:name="android.intent.action.VIEW"/>
    <category android:name="android.intent.category.BROWSABLE"/>
    <category android:name="android.intent.category.DEFAULT"/>
    <data android:host="*.ncaula.com" android:scheme="https"/>
</intent>
```

This allows the app to intercept/handle deep links to any ncaula.com subdomain, matching the environment configuration pattern.

## Source Files

All findings sourced from static analysis of decompiled code:

- `decompiled_csharp/AulaNative/AulaNative.Configuration/EnvironmentFactory.cs` -- complete environment matrix
- `decompiled_csharp/AulaNative/AulaNative.Configuration/Conf.cs` -- cert pins, basic auth creds, URL construction
- `decompiled_csharp/AulaNative/AulaNative.Configuration/EnvironmentConfig.cs` -- config struct (6 host fields)
- `decompiled_csharp/AulaNative/AulaNative.Configuration/AulaEnvironment.cs` -- environment enum (13 values)
- `apktool_out/AndroidManifest.xml` -- intent filter for `*.ncaula.com`
