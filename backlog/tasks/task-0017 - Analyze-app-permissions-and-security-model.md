---
id: TASK-0017
title: Analyze app permissions and security model
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:59'
updated_date: '2026-03-19 05:16'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula app requests notable permissions: CAMERA, CALL_PHONE, USE_BIOMETRIC/USE_FINGERPRINT, SYSTEM_ALERT_WINDOW, POST_NOTIFICATIONS, plus Firebase C2DM for push. Also has a Samsung-specific analytics permission. Investigate what each permission is used for and whether any are over-privileged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Map all declared AndroidManifest.xml permissions to their actual usage in decompiled code
- [x] #2 Analyze TLS/certificate handling: AlwaysAcceptingHostnameVerifier, FakeSSLSession, cert pinning keys
- [x] #3 Document authentication and token storage mechanisms
- [x] #4 Document data storage and encryption practices
- [x] #5 Assess overall security posture with specific concerns and findings
- [x] #6 Produce security_analysis.md with all findings
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Map all 14 AndroidManifest permissions to code usage
2. Analyze TLS/cert pinning: CertificatePinningUtils + DEX-level ServerCertificateCustomValidator
3. Document auth/token storage (SecureStorage, EncryptionManager, PIN validation)
4. Document data storage encryption (SQLite [Encrypt] attribute pattern)
5. Assess security posture: strengths and concerns
6. Write security_analysis.md
<!-- SECTION:PLAN:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Comprehensive security analysis of the Aula Android app (v2.15.4) covering all declared permissions, TLS/certificate handling, authentication mechanisms, and data encryption.

Changes:
- Produced security_analysis.md (220 lines) at the repo root with 7 major sections
- Mapped all 14 AndroidManifest permissions to their actual code usage, identifying SYSTEM_ALERT_WINDOW and Samsung analytics permission as potentially over-privileged
- Analyzed dual-layer TLS: .NET CertificatePinningUtils (SPKI hash pinning with 3 production + 3 test keys) plus Xamarin DEX-level ServerCertificateCustomValidator bridge
- Documented OIDC+PKCE auth flow via IdentityModel.OidcClient with two authentication levels (UniLogin L2, MitID L3)
- Documented token/credential storage chain: SecureStorageManager -> Plugin.SecureStorage -> Android Keystore
- Documented SQLite encryption via EncryptionManager (AES-CBC, PBKDF2 key derivation with 300 iterations and hardcoded salt)
- Identified 9 security findings with severity ratings, including: PIN brute-force risk (no lockout), weak PBKDF2 parameters, unused SYSTEM_ALERT_WINDOW permission, no root detection
- Fixed exported components table to accurately reflect all 9 exported components from the manifest

Key findings:
- Certificate pinning is properly implemented for *.aula.dk domains
- PIN stored as plaintext in SecureStorage with no brute-force protection
- PBKDF2 uses only 300 iterations (OWASP recommends 600,000+)
- Test credentials (aula-user:Aula-1337) shipped in production APK
- 3 settings activities exported without clear necessity
<!-- SECTION:FINAL_SUMMARY:END -->
