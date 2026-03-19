---
id: TASK-0081
title: Analyze Google Tink cryptographic key usage
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:48'
updated_date: '2026-03-19 06:42'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula APK includes Google Tink crypto library with proto-serialized key types (AES-GCM, AES-CTR-HMAC, ChaCha20-Poly1305, AES-SIV). Investigate what Tink is used for in the app -- likely DataStore encryption or secure token storage. Map which key types are actually instantiated.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all Tink classes present in the decompiled DEX code
- [x] #2 Determine how Tink keys are created and managed (MasterKey, KeysetHandle)
- [x] #3 Map what data is encrypted using Tink (EncryptedSharedPreferences, DataStore, etc.)
- [x] #4 Document key types actually instantiated vs merely bundled
- [x] #5 Identify relationship between Tink crypto and .NET EncryptionManager (if any)
- [x] #6 Document findings in architecture or analysis docs
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Search DEX decompiled code for Tink class presence and cross-references
2. Identify whether app code actually calls into Tink/EncryptedSharedPreferences/EncryptedFile
3. Analyze Plugin.SecureStorage implementation to determine actual crypto mechanism
4. Map relationship between .NET EncryptionManager and Tink
5. Document findings - correct any inaccuracies in existing docs
6. Update security_analysis.md with accurate Tink analysis
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Analysis complete. Key finding: Google Tink (486 classes) and EncryptedSharedPreferences are bundled as transitive dependencies of AndroidX Security but are NEVER called from any app code. The app uses two independent crypto mechanisms:
1. .NET EncryptionManager (AES-CBC via System.Security.Cryptography) for SQLite field encryption
2. Plugin.SecureStorage / ProtectedFileImplementation (Android Keystore-backed AES) for credential storage

Corrected inaccurate claims in security_analysis.md, auth_flow.md, and native_library_inventory.md that stated the app uses EncryptedSharedPreferences.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Analyzed Google Tink cryptographic library presence in the Aula APK. Key finding: Tink (486 Java classes) and AndroidX Security Crypto (EncryptedSharedPreferences, EncryptedFile, MasterKey) are bundled but completely unused -- zero cross-references from app code.

Changes:
- Added section 4.3 to security_analysis.md documenting Tink as dead code, listing all bundled modules and key types, and mapping the actual crypto mechanisms used by the app
- Corrected inaccurate claim in security_analysis.md section 4.2 (Plugin.SecureStorage uses ProtectedFileImplementation, not EncryptedSharedPreferences)
- Corrected same inaccuracy in auth_flow.md section on Android implementation
- Updated native_library_inventory.md to note Tink is a transitive dependency, not actively used

Actual crypto architecture:
- SQLite field encryption: .NET EncryptionManager using AES-CBC (System.Security.Cryptography) with PBKDF2-derived keys
- Credential/token storage: Plugin.SecureStorage using ProtectedFileImplementation with Android Keystore-backed AES keys
- Neither mechanism uses Tink
<!-- SECTION:FINAL_SUMMARY:END -->
