# DEX Decompilation Analysis (jadx 1.5.0)

## Summary

Decompiled both DEX files from the Aula APK (v2.15.4) using jadx. This is a .NET MAUI/Xamarin app where the DEX layer serves as the Android bootstrap. The actual business logic resides in .NET assemblies loaded by the Mono runtime; the DEX code consists of Android Callable Wrappers (ACWs), third-party libraries, and the Mono bridge.

## Decompilation Results

| File | Size | Classes Processed | Errors | Success Rate | Java Files Output | Files with Warnings |
|------|------|-------------------|--------|-------------|-------------------|---------------------|
| classes2.dex | 4.6 MB | 2,397 | 2 | 99.9% | 3,322 | 223 (6.7%) |
| classes.dex | 8.2 MB | 2,023 | 4 | 99.8% | 3,463 | 522 (15.1%) |
| **Total** | **12.8 MB** | **4,420** | **6** | **99.9%** | **6,785** | **745 (11.0%)** |

Output directories:
- `classes.dex.decompiled.jadx/` (gitignored)
- `classes2.dex.decompiled.jadx/` (gitignored)

## Obfuscation Assessment

**No ProGuard/R8 obfuscation detected.** All packages and class names are fully readable:
- No single-letter package or class names
- No name mangling or string encryption
- All method signatures are intact
- The CRC64 package prefixes are Xamarin's standard hashing mechanism (namespace -> CRC64 hash), not obfuscation

The only "obfuscation" is the inherent indirection of the Xamarin ACW pattern: every CRC64 class delegates to native methods that call into the Mono/.NET runtime, making the Java layer a thin shell.

## Class Distribution

### classes2.dex (4.6 MB) -- Contains App Code

| Category | Count | % |
|----------|-------|---|
| CRC64 Aula app classes (ACWs) | 678 | 20.4% |
| Kotlin stdlib/coroutines | 876 | 26.4% |
| Google Material Design | 435 | 13.1% |
| Mono/Xamarin bridge | 436 | 13.1% |
| Firebase | 220 | 6.6% |
| Other Google (Tink, Play Services, etc.) | 551 | 16.6% |
| Other (javax, okio, net, org, microsoft) | 126 | 3.8% |
| **Total** | **3,322** | |

### classes.dex (8.2 MB) -- Entirely Third-Party

| Category | Count | % |
|----------|-------|---|
| AndroidX libraries | 2,018 | 58.3% |
| Other (android compat, _COROUTINE, com.*) | 1,445 | 41.7% |
| **Total** | **3,463** | |

## CRC64 App Class Analysis

179 CRC64 packages containing 678 Android Callable Wrapper classes. These map to .NET namespaces via `Runtime.register()` calls.

### .NET Assembly References

| Assembly | ACW Count |
|----------|-----------|
| AulaNative.Droid | 644 |
| Microsoft.Maui.Essentials | 19 |
| Xamarin.AndroidX.Transition | 4 |
| Xamarin.GooglePlayServices.Basement | 2 |
| Xamarin.GooglePlayServices.Base | 2 |
| Xamarin.AndroidX.Browser | 2 |
| Xamarin.GooglePlayServices.Tasks | 1 |
| Plugin.SecureStorage | 1 |
| Plugin.Fingerprint | 1 |
| Microsoft.Maui.Graphics | 1 |

### Key AulaNative.Droid Namespace Hierarchy

The ACW registrations reveal the complete activity/fragment structure of the app:

**Login & Auth:**
- `AulaNative.Droid.Activities.Login` -- LoginActivity, LoginWithPinActivity, WebAuthenticationCallbackActivity
- `AulaNative.Droid.OAuth` -- OAuth flow handling
- `Plugin.Fingerprint` -- Biometric authentication (AuthenticationHandler using BiometricPrompt)
- `Plugin.SecureStorage` -- Secure token/credential storage

**Core Features:**
- `AulaNative.Droid.Activities.Messages` -- Messaging (threads, folders, auto-reply)
- `AulaNative.Droid.Activities.Posts` -- Post feeds/lists
- `AulaNative.Droid.Activities.Calendar` -- Calendar views, events, meetings, vacation, synchronization
- `AulaNative.Droid.Activities.Album` -- Photo albums, media selection, gallery
- `AulaNative.Droid.Activities.Gallery` -- Gallery views, adapters, folder creation
- `AulaNative.Droid.Activities.Document` -- Documents, cloud integration, secure documents
- `AulaNative.Droid.Activities.Comments` -- Comment system
- `AulaNative.Droid.Activities.Search` -- Search functionality
- `AulaNative.Droid.Activities.ContactsList` -- Contact list views

**Staff/Admin Features:**
- `AulaNative.Droid.Activities.ComeGo` -- Child check-in/out (parent side: times, absence, pickup)
- `AulaNative.Droid.Activities.ComeGoStaff` -- Staff-side check-in management (overview, filters, vacation, week overview)
- `AulaNative.Droid.Activities.ActivityList` -- Activity list, vacation list, staff overview
- `AulaNative.Droid.Activities.Groups` -- Group management

**User Profile & Settings:**
- `AulaNative.Droid.Activities.UserProfile` -- Profile, consents, device settings, notification settings, master data
- `AulaNative.Droid.Activities.AdditionalMasterData` -- Extended profile data, revision history
- `AulaNative.Droid.Activities.Onboarding` -- Onboarding flows (consent, policy, master data)
- `AulaNative.Droid.Activities.PersonalReferenceData` -- Reference data views

**Infrastructure:**
- `AulaNative.Droid.Activities.Common` -- Base activities, generic lists
- `AulaNative.Droid.Activities.Overview` -- Main overview/dashboard
- `AulaNative.Droid.Activities.Menu` -- Menu/navigation
- `AulaNative.Droid.Activities.Partials` -- Reusable UI components
- `AulaNative.Droid.CustomViews` -- Rich set of custom UI components (editors, galleries, autocomplete, etc.)
- `AulaNative.Droid.Views` -- UI views (calendar, date pickers, validation, loading overlays)
- `AulaNative.Droid.FireBase` -- Firebase/push notification integration
- `AulaNative.Droid.Utils` -- Utilities (decorators, swipe layouts, back button handling)

### Largest CRC64 Packages (by class count)

| CRC64 Hash | Classes | Mapped .NET Namespace |
|------------|---------|----------------------|
| crc647e759e71f16a7378 | 36 | AulaNative.Droid.Activities.Calendar.Event |
| crc64ce60827e1bb093e8 | 18 | (multiple calendar/event classes) |
| crc648e5c3c2d7dd73a22 | 16 | (UI components) |
| crc6486715a2de441f74b | 14 | (custom views) |
| crc64629431144609bb38 | 14 | (ComeGoStaff overview) |
| crc64c1d40e40ba7282d6 | 13 | (document handling) |

## Common JADX Warnings

Most warnings are benign type inference issues. No warnings indicate data loss or unrecoverable decompilation failures.

| Warning Type | classes2.dex | classes.dex |
|-------------|-------------|-------------|
| Multi-variable type inference failed | 424 | 387 |
| 'super' call moved to top | 62 | 168 |
| Can't rename method to resolve collision | 51 | 162 |
| Illegal instructions before constructor call | 27 | 19 |
| Failed to restore enum class | 23 | 14 |
| Unknown enum class pattern | 20 | 6 |
| Code restructure failed | ~10 | ~40 |

## Key Findings

1. **No obfuscation** -- The DEX layer is completely readable. Netcompany did not apply ProGuard/R8 name obfuscation to the DEX files.

2. **ACW pattern is consistent** -- All 678 CRC64 classes follow the identical Xamarin ACW pattern:
   - Static initializer calls `Runtime.register()` with the .NET type name
   - Constructor calls `TypeManager.Activate()`
   - All lifecycle methods delegate to `n_*` native methods
   - Implements `IGCUserPeer` for GC bridging

3. **Rich namespace reveals app architecture** -- The ACW registrations expose the complete .NET namespace hierarchy, giving a full map of the app's activity/fragment structure without needing to decompile the .NET assemblies.

4. **Third-party library inventory:**
   - Google Material Design components
   - Firebase (messaging, installations, analytics)
   - Google Tink (crypto)
   - Google Play Services
   - OkHttp/OkIO (networking)
   - Kotlin coroutines
   - AndroidX (full suite)

5. **Authentication uses biometrics** -- `Plugin.Fingerprint` wraps Android's BiometricPrompt, with `Plugin.SecureStorage` for credential persistence. OAuth flow is handled via WebAuthenticator.

6. **The real analysis target is the .NET assemblies** -- The DEX layer is a thin shell. API clients, data models, business logic, and auth token handling all live in the Mono/.NET layer (the DLL files in the APK's assemblies directory).
