---
id: TASK-0084
title: Investigate PIN brute-force and lockout behavior
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 23:55'
updated_date: '2026-03-19 07:26'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
PIN validation in PinCodeViewModel.ValidatePin() uses plain string comparison with no rate limiting or lockout. Investigate whether there is any server-side lockout, attempt counting at the Activity/Fragment level, or other compensating controls not visible in the ViewModel layer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Trace PIN validation flow from UI (Activity/Fragment/Page) through ViewModel to storage
- [x] #2 Determine if any attempt counter or lockout timer exists at any layer
- [x] #3 Check biometric authentication path for lockout mechanisms
- [x] #4 Check Android-layer (Java/smali) for PIN-related lockout logic
- [x] #5 Document findings in security_analysis.md with concrete code references
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read PinCodeViewModel.ValidatePin() - confirm no attempt counter or lockout
2. Read LoginWithPinActivity - check _lockLoginAttempt field and EnterPressed flow
3. Read BiometricUtils.AuthenticateBiometric() - check BioAuthStatus lockout handling
4. Read AuthenticationHandler - check Android BiometricPrompt TooManyAttempts
5. Read HandlingStressClickHelper - confirm it is UI debounce not rate limiting
6. Search Java/smali DEX layers for any PIN-related native code
7. Check SecureStorageManager for any attempt counter keys
8. Document all findings in security_analysis.md section 3.3
9. Run e2e tests, commit
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Investigation complete. Traced PIN validation through all layers:
- PinCodeViewModel.ValidatePin(): plain string comparison, no attempt counter
- LoginWithPinActivity._lockLoginAttempt: UI concurrency guard, not security control
- HandlingStressClickHelper: 1-second UI debounce, not rate limiting
- SecureStorageManager: no attempt counter keys in storage schema
- Biometric path: OS-level lockout only (BiometricPrompt error code 7 -> TooManyAttempts)
- Java/smali layer: no Aula PIN logic at DEX level
- PIN is local-only, never sent to server
- PIN keyspace: 6 digits = 1M combinations

Updated security_analysis.md sections 3.3, 3.4, and finding #1 in summary table.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Investigated PIN brute-force and lockout behavior across all layers of the Aula app (TASK-84).

Findings:
- PinCodeViewModel.ValidatePin() performs a plain string comparison with zero attempt counting, lockout, or rate limiting
- LoginWithPinActivity._lockLoginAttempt is a UI concurrency guard (prevents double-submission), not a security control
- HandlingStressClickHelper provides 1-second UI debounce only
- SecureStorageManager stores no attempt counter keys
- PIN is validated locally (never sent to server), so server-side lockout is impossible
- Java/smali DEX layer contains no Aula-specific PIN logic
- PIN is 6 numeric digits (1M keyspace)
- Biometric path benefits from Android OS-level lockout (BiometricPrompt error 7 -> TooManyAttempts) but the app adds nothing on top

Changes:
- Updated security_analysis.md section 3.3 with detailed brute-force investigation findings
- Updated section 3.4 with biometric OS-level lockout documentation
- Updated finding #1 in the security summary table with confirmed details
<!-- SECTION:FINAL_SUMMARY:END -->
