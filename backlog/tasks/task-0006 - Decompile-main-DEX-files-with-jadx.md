---
id: TASK-0006
title: Decompile main DEX files with jadx
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:31'
updated_date: '2026-03-18 14:43'
labels: []
dependencies:
  - TASK-0003
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Use jadx to decompile the primary DEX files containing com.netcompany.aula.* classes. Store output in structured directories. Assess decompilation quality and identify obfuscated vs readable code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 jadx produces Java source for main DEX files
- [x] #2 Output stored in classes.dex.decompiled.jadx/ directories
- [x] #3 Decompilation success rate documented (% of classes)
- [x] #4 Obfuscation patterns identified (ProGuard, R8, etc.)
- [x] #5 Key packages identified: API clients, models, auth, UI
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run jadx on classes2.dex (app code) first, output to classes2.dex.decompiled.jadx/
2. Run jadx on classes.dex (third-party), output to classes.dex.decompiled.jadx/
3. Assess decompilation quality: count successful vs failed classes
4. Identify obfuscation patterns (ProGuard/R8 signatures)
5. Catalog key packages: CRC64 app classes, API clients, auth, models, UI
6. Add output dirs to .gitignore
7. Document findings in dex_decompilation_analysis.md
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
- jadx 1.5.0 decompiled both DEX files with 99.9% success rate (6 errors total out of 4420 classes)
- classes2.dex contains 678 CRC64 ACW classes mapping to AulaNative.Droid namespace
- No ProGuard/R8 obfuscation detected
- CRC64 packages are standard Xamarin namespace hashing, not obfuscation
- Rich namespace hierarchy extracted from Runtime.register() calls
- All CRC64 classes are thin shells delegating to Mono native methods
- Key findings documented in dex_decompilation_analysis.md
- Added *.decompiled.jadx/ to .gitignore
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Decompiled both DEX files (classes.dex 8.2MB, classes2.dex 4.6MB) from Aula APK v2.15.4 using jadx 1.5.0.

Results:
- 99.9% decompilation success rate (4414/4420 classes, 6 errors)
- 6785 Java source files produced across two output directories
- 745 files (11%) contain non-fatal JADX warnings (type inference, enum restoration)

Key findings:
- No ProGuard/R8 obfuscation applied to DEX layer
- 678 CRC64 classes are Xamarin Android Callable Wrappers (ACWs) mapping to AulaNative.Droid .NET namespace
- ACW registrations reveal complete app architecture: Login/OAuth, Messages, Calendar, Albums, Documents, ComeGo (child check-in), Staff management, User profiles, Onboarding
- Authentication uses BiometricPrompt (Plugin.Fingerprint) + SecureStorage + OAuth/WebAuthenticator
- Third-party: Firebase, Google Material, Tink crypto, OkHttp, Kotlin coroutines, full AndroidX suite
- DEX layer is thin shell; real business logic is in .NET assemblies loaded by Mono runtime

Files:
- dex_decompilation_analysis.md -- full analysis document
- classes.dex.decompiled.jadx/ -- decompiled output (gitignored)
- classes2.dex.decompiled.jadx/ -- decompiled output (gitignored)
- .gitignore -- updated with *.decompiled.jadx/ pattern
<!-- SECTION:FINAL_SUMMARY:END -->
