---
id: TASK-0015
title: Analyze Kotlin Multiplatform native modules
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 13:59'
updated_date: '2026-03-18 23:43'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The base APK contains multiple Kotlin Multiplatform target directories (commonMain, nativeMain, linuxMain, androidNativeMain, jsAndWasmSharedMain, etc.) with linkdata/module files. These suggest the app uses KMP for cross-platform code sharing. Investigate what functionality is in these native modules vs the .NET layer.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Identify all Kotlin/KMP-related packages in the decompiled output
- [x] #2 Determine whether Kotlin code is app-authored KMP or just transitive dependencies (AndroidX, Firebase, etc.)
- [x] #3 Catalog any KMP module linkdata directories and their contents
- [x] #4 Document findings with evidence (file paths, package names)
- [x] #5 Create follow-up tasks for any interesting tangents discovered
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Catalog all KMP source-set directories and their linkdata/module files
2. Examine module files to identify which KMP libraries are bundled
3. Search jadx decompiled DEX for Kotlin packages (kotlinx.*, kotlin.coroutines, etc.)
4. Determine if any Kotlin code is app-authored vs library dependency
5. Cross-reference with TASK-3 Kotlin findings
6. Document findings and create follow-up tasks for tangents
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
## Investigation Results

### KMP Source-Set Directories Found (16 total)
Located under apktool_out/unknown/ and the extracted APK root:
commonMain, concurrentMain, nativeMain, hashFunctions, nonAppleMain,
androidNativeMain, unixMain, linuxMain, systemFileSystemMain, webMain,
nonJvmMain, nativeUnixLikeMain, zlibMain, nonJsMain, jsAndWasmSharedMain,
nativeOtherMain

Each contains default/linkdata/module files and .knm (Kotlin Native Metadata) files.

### Module Identification from linkdata
All KMP modules belong to three third-party libraries:
1. **Okio** (com.squareup.okio) - commonMain, nativeMain, linuxMain, unixMain, nonAppleMain, nonJvmMain, nonJsMain, hashFunctions, zlibMain, systemFileSystemMain
2. **kotlinx.atomicfu** (org.jetbrains.kotlinx) - concurrentMain, androidNativeMain, nativeUnixLikeMain, jsAndWasmSharedMain
3. **kotlinx-coroutines-core** (org.jetbrains.kotlinx) - nativeOtherMain
4. **androidx.collection** - webMain

### kotlin-project-structure-metadata.json
The metadata file belongs to Okio and describes its KMP source set hierarchy. It is NOT Aula app code.

### DEX Kotlin Packages (classes2.dex)
- kotlin.* (383 files) - Kotlin stdlib
- kotlinx.coroutines.* - coroutines library (v1.10.2)
- kotlinx.atomicfu.* - atomicfu library
- kotlinx.serialization.* including protobuf - serialization library
- kotlinx.parcelize.* - Android parcelize
- okio.* (46 files) - Square Okio I/O library

### Key Finding: No App-Authored Kotlin Code
- Zero Kotlin @Metadata annotations in Xamarin ACW (crc64*) classes
- Zero imports of kotlin/kotlinx packages from app code
- No Netcompany/Aula references in Kotlin library code (only generic error strings)
- All crc64* classes are Xamarin Android Callable Wrappers delegating to .NET via mono.android.Runtime

### Why Kotlin is Present
Kotlin enters as transitive dependencies of:
- **AndroidX DataStore** (v1.1.7) - KMP library using Okio for I/O
- **AndroidX Lifecycle** - KMP-aware components
- **AndroidX Collection** - KMP port
- **Firebase Messaging/Analytics** - uses kotlinx.coroutines
- **Google Play Services** - Kotlin stdlib dependency

### Proto Files
Three .proto files found are Google-authored (Firebase transport/messaging), not app-specific.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
## Finding: No Kotlin Multiplatform App Code -- All KMP Artifacts Are Library Dependencies

The 16 KMP source-set directories (commonMain, nativeMain, linuxMain, etc.) with linkdata/module files found in the APK are NOT Aula app code. They are pre-compiled KMP library metadata bundled by three third-party dependencies:

**Libraries identified:**
- **Okio** (com.squareup.okio) -- 10 source sets, used by AndroidX DataStore for I/O
- **kotlinx.atomicfu** (org.jetbrains.kotlinx) -- 4 source sets, concurrency primitives
- **kotlinx-coroutines-core** -- 1 source set
- **androidx.collection** -- 1 source set (webMain)

**Why Kotlin is in a Xamarin app:**
AndroidX DataStore v1.1.7, AndroidX Lifecycle, Firebase Messaging/Analytics, and Google Play Services all depend on Kotlin stdlib and kotlinx libraries. These are pulled in as transitive dependencies. The Kotlin DEX code (876 files across kotlin/kotlinx/okio packages in classes2.dex) is entirely library code.

**Evidence that Aula writes zero Kotlin:**
- No @kotlin.Metadata annotations in any Xamarin ACW (crc64*) class
- No imports of kotlin/kotlinx from app code
- All app logic flows through Xamarin ACWs that delegate to .NET via mono.android.Runtime
- The app is purely C#/.NET (Xamarin.Android) with no Kotlin application code

**Follow-up tasks created:**
- TASK-0078: Investigate kotlinx.serialization.protobuf usage in AndroidX DataStore
- TASK-0079: Map Xamarin crc64 ACW namespaces to .NET assemblies
<!-- SECTION:FINAL_SUMMARY:END -->
