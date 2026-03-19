---
id: TASK-0080
title: Investigate Firebase Analytics transport protocol and data collection
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:48'
updated_date: '2026-03-19 06:34'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula APK uses Firebase Data Transport SDK with protobuf-encoded client analytics (client_analytics.proto). Investigate what analytics data the app collects and sends to Google, including event cache behavior and log source metrics. This could reveal what user actions are tracked.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all Firebase Analytics event names logged by the app
- [x] #2 Map the transport protocol: endpoints, encoding, batch/cache behavior
- [x] #3 Catalog user/device data fields collected in analytics payloads
- [x] #4 Check .NET/C# side for additional analytics instrumentation (AppCenter, etc.)
- [x] #5 Document findings in a structured analysis file
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search DEX decompiled code for Firebase transport/analytics classes
2. Decode obfuscated CCT endpoint URLs
3. Trace the CctTransportBackend request protocol
4. Map MessagingAnalytics event flow (Scion + Firelog paths)
5. Read bundled .proto files for payload schemas
6. Examine SQLite event cache schema and config
7. Search .NET/C# assemblies for analytics instrumentation
8. Document all findings
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- Decoded obfuscated StringMerger URLs: DEFAULT endpoint is firebaselogging.googleapis.com, LEGACY is firebaselogging-pa.googleapis.com
- Found two log sources: FCM_CLIENT_EVENT_LOGGING (notification metrics) and GDT_CLIENT_METRICS (transport self-monitoring)
- Firebase Analytics SDK is NOT present -- only the measurement-connector interface exists (required FCM dependency)
- Scion analytics events are silently dropped because AnalyticsConnector has no implementation
- No analytics instrumentation found in .NET/C# assemblies (no AppCenter, no custom tracking)
- Device metadata collected: model, hardware, OS build, locale, country, carrier MCC/MNC, app version
- Event cache: SQLite DB, 10MB max, 7-day cleanup, 200-event upload batches, 80KB max per row
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated Firebase Analytics transport protocol and data collection in the Aula APK.

Key findings:
- Aula does NOT implement custom analytics -- no app-level event tracking in either DEX or .NET code
- Only Firebase SDK internal telemetry exists: FCM notification delivery metrics (MessagingAnalytics) and transport self-monitoring (GDT_CLIENT_METRICS)
- Firebase Analytics SDK is absent; only the measurement-connector interface ships (required by FCM), so Scion events are silently dropped
- Transport uses gzipped JSON over HTTPS to firebaselogging.googleapis.com with protobuf-encoded payloads
- Device metadata sent: model, OS, locale, country, carrier MCC/MNC, app version (no advertising IDs, no IMEI, no PII)
- Event cache: SQLite DB with 10MB cap, 7-day cleanup, 200-event upload batches
- Decoded obfuscated endpoint URLs from StringMerger class
- Documented MessagingClientEvent and ClientMetrics protobuf schemas with field-level analysis

Analysis file: analysis/firebase_analytics_transport.md
<!-- SECTION:FINAL_SUMMARY:END -->
