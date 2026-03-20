# DEX File Catalog — Aula APK v2.15.4

## DEX File Inventory

| File | Parent APK | Size | Classes |
|------|-----------|------|---------|
| `classes.dex` | `com.netcompany.aulanativeprivate.apk` | 8.2 MB | 7,229 |
| `classes2.dex` | `com.netcompany.aulanativeprivate.apk` | 4.6 MB | 5,163 |
| **Total** | | **12.8 MB** | **12,392** |

No DEX files exist in the config split APKs (`config.en.apk`, `config.mdpi.apk`, `config.x86_64.apk`) — these contain only resources, locale data, and native libraries.

## Classification Summary

| Category | Classes | % | Type |
|----------|---------|---|------|
| AndroidX | 4,984 | 40.2% | Third-party (Google Jetpack) |
| Google (Firebase/Play Services) | 3,910 | 31.6% | Third-party |
| Kotlin stdlib/coroutines/serialization | 1,093 | 8.8% | Third-party |
| CRC64 Aula app code | 678 | 5.5% | **App code** |
| Glide (com.bumptech) | 653 | 5.3% | Third-party (image loading) |
| Xamarin/Mono runtime | 438 | 3.5% | Runtime bridge |
| Android support library | 168 | 1.4% | Third-party (legacy compat) |
| Daimajia (com.daimajia) | 146 | 1.2% | Third-party (animations) |
| OkHttp/OkIO | 67 | 0.5% | Third-party (HTTP) |
| Android platform stubs | 64 | 0.5% | Platform |
| JetBrains annotations | 55 | 0.4% | Third-party |
| NineOldAndroids | 51 | 0.4% | Third-party (animation compat) |
| javax annotations | 37 | 0.3% | Third-party |
| Other third-party | 30 | 0.2% | Third-party |
| .NET MAUI / Xamarin internals | 18 | 0.1% | Runtime |

## App Code: CRC64 Namespace Analysis

All 678 Aula-specific classes reside in `classes2.dex` under CRC64-hashed namespaces (Xamarin convention: `crc64<hash>.<ClassName>`). There are **179 unique CRC64 namespace hashes** representing the original .NET namespaces.

### Key Aula-Specific Class Groups (by CRC64 hash)

Notable class families identified by name patterns:

**Core UI/Navigation:**
- `crc649881f3fa1611df58` — `MainActivity`, `MainApplication` (app entry point)
- `crc64727613c41f254141` — `LauncherActivity`, `LoginActivity`, `LoginWithPinActivity`, `ForceUpdateActivity`, `OTPSelectionActivity`, `WebAuthenticationCallbackActivity`
- `crc649512c951229a8649` — `AulaBaseAppCompatActivity`, `AulaBaseFragment`, `AulaBaseListFragment`, `AulaBaseFragmentActivity`, `AulaBaseAbstractPushNotificationNavigatorActivity`
- `crc64ef1cca385a178d29` — `AulaNavigationFragment`, `AulaNaivgationRootFragment`, `AulaFragmentViewPagerAdapter`

**Messaging:**
- `crc64117d19181bd40e67` — `MessageThreadActivity`, `MessageThreadAdapter`, `MessageThreadFragment`
- `crc642ab953f6613e37b6` — `MessagesOverviewFragment`, `MessagesOverviewListAdapter`, `AdvancedSearchActivity`
- `crc64982ffc6c9fa05072` — `CreateMessageActivity`
- `crc644b585a7d6893d48b` — `MessageFolderActivity`, `MessageFolderFragment`, `MessageFolderAdapter`

**Calendar/Events:**
- `crc64464655eca3cd0aac` — `CalendarFragment`, `CalendarOverviewFragment`, `CalendarViewPagerAdapter`
- `crc647e759e71f16a7378` — `CalendarPortraitFragment`, `CalendarLandscapeFragment`, `EventDetailsActivity`, `EventEditFormActivity`, `EditLessonActivity`, `EditMeetingActivity`, `ShareCalendarActivity` (22 classes)
- `crc64df89d32432429ade` — `CalendarSynchronisationCreationActivity`, `CalendarSynchronisationOverviewActivity`
- `crc6450301ba970a0a5ed` — `BirthdayCalendarAdapter`, `CalendarMenuAdapter`, `EventFilterViewHolder`

**Posts/Feed:**
- `crc64b07562da40b875be` — `EditPostActivity`, `ViewPostActivity`, `PostListViewWithFilterFragment`
- `crc64ee5c41ee04c846c1` — `OverviewFragment`, `PostsAllNotificationActivity`
- `crc64b727de19c91cf26f` — `PostListAdapter`, `PostListViewHolder`

**Gallery/Media:**
- `crc64ce60827e1bb093e8` — `AulaImageView`, `AulaPhotoView`, `AulaVideoView`, `MediaOverviewActivity`, `MediaOverviewAdapter` (17 classes)
- `crc640d97cec38b77325f` — `AlbumEditFormActivity`, `GalleryPickerActivity`, `MediaSelectionFragment`
- `crc64f48a125ae4e10cda` — `AlbumDetailsActivity`, `GalleryFragment`, `MediaTaggingPageActivity`
- `crc6466f7fa310a49d51d` — `AndroidAulaEditor`, `AndroidAulaMessageEditor`, `MediaTagActivity`

**Secure Documents:**
- `crc6449307630a8b601a5` — `SecureDocumentViewRevisionActivity`
- `crc64629431144609bb38` — `SecureDocumentOverviewFragment`, `SecureDocumentOverviewAdapter` (13 classes)
- `crc64755e4274aa5e2b1a` — `SecureDocumentDetailsActivity`
- `crc64db7f9aea8cfbb131` — `SecureDocumentShareWithFormActivity`, `InternalSecureDocumentFormActivity`

**Presence/ComeGo:**
- `crc64a171c606f2682d2d` — `ComeGoFragment`, `PlanningPageFragment`
- `crc645eb5d8e01aef11fc` — `ComeGoEmployeeFragment`, `ComeGoStaffOverviewFragment`
- `crc64964578a205e3fa0b` — `ComeGoViewTimesActivity`, `ComeGoViewTimesAdapter`
- `crc64fe3874eccf51a10b` — `ComeGoOpeningHoursAndClosedDaysOverviewPageActivity`

**Profiles/Contacts:**
- `crc64e3bc9524bf401dcf` — `ProfileActivity`, `ProfileAdapter`
- `crc64a7dcd4496e5253be` — `ContactListOverviewFragment`, `ContactListAdapter`
- `crc648d07b834a19501a6` — `MasterDataActivity`

**Notifications:**
- `crc645d81b9b77a3a8305` — `AulaFirebaseMessagingService`
- `crc6486715a2de441f74b` — `NotificationSettingsActivity`, `NotificationSettingsEditActivity` (12 classes)
- `crc6485dd9830b50fd4a3` — `AllNotificationActivity`

**Absence/Vacation:**
- `crc644caf61526f1f2e48` — `AbsenceVacationActivity`, `VacationRequestPageFragment`
- `crc648e6078e8b9fed96c` — `VacationRegistrationOverviewActivity`, `VacationDetailsActivity`
- `crc64439a32af6af2bcba` — `VacationListOverviewFragment`

**Groups:**
- `crc6426d4d8d262dda33c` — `GroupDashboardActivity`, `GroupMembershipActivity`, `GroupsListAdapter`, `GroupsMenuFragment`

**Authentication/Onboarding:**
- `crc6407ec88ddc45e0223` — `OnboardingActivity`, `OnboardingFragmentPagerAdapter`
- `crc64e6d0b84c6264ccdd` — `AuthenticationHandler`
- `crc6468b6408a11370c2f` — `WebAuthenticatorCallbackActivity`

**Search:**
- `crc64c1d40e40ba7282d6` — `GlobalSearchActivity`, `SearchAdapter` (14 classes covering posts, events, profiles, groups, etc.)

**Consent/Privacy:**
- `crc64bbda72139c7d8a39` — `ConsentActivity`, `ConsentListAdapter`
- `crc640f83e066fd36f625` — `DataPolicyWebView`

**Cloud Storage:**
- `crc6401be2761ec990c52` — `CloudStorageFragment`
- `crc646df68bf978cc5370` — `CloudIntegrationActivity`
- `crc64d48df833906fa8fe` — `CloudStorageAuthInterceptor`

**Portal WebView:**
- `crc64c23e2e7fec60456b` — `AulaPortalWebView`, `AulaPortalWebViewActivity`, `WidgetWebViewFragment`

## Xamarin/Mono Runtime Layer

438 classes forming the .NET-to-Android bridge:

- **`mono.MonoPackageManager`** — Manages .NET assembly loading
- **`mono.MonoRuntimeProvider`** — ContentProvider that boots the Mono/.NET runtime
- **`mono.android.Runtime`** / **`mono.android.TypeManager`** — Core runtime bridge
- **`mono.android.GCUserPeer`** — GC bridging between .NET and Java
- **`android.runtime.JavaProxyThrowable`** — Exception bridging
- **310 `mono.android.*` listener implementors** — Auto-generated wrappers that bridge Android callbacks to .NET delegates

## .NET MAUI-Specific Classes

- `microsoft.maui.essentials.fileProvider` — MAUI Essentials file sharing
- `net.dot.android.crypto.*` — .NET Android crypto (TLS, PBKDF2)
- `net.dot.jni.*` — .NET JNI managed peer bridge
- `xamarin.android.net.ServerCertificateCustomValidator*` — Custom TLS certificate handling

## Third-Party Libraries Identified

| Library | Package | Purpose | Classes |
|---------|---------|---------|---------|
| AndroidX (Jetpack) | `androidx.*` | Modern Android UI/lifecycle | 4,984 |
| Firebase | `com.google.firebase.*` | Push notifications, analytics | Part of 3,910 |
| Google Play Services | `com.google.android.gms.*` | Google APIs | Part of 3,910 |
| Kotlin | `kotlin.*`, `kotlinx.*` | Language runtime & serialization | 1,093 |
| Glide | `com.bumptech.glide.*` | Image loading/caching | 653 |
| Daimajia AndroidViewAnimations | `com.daimajia.*` | Animation effects | 146 |
| NineOldAndroids | `com.nineoldandroids.*` | Legacy animation compat | 51 |
| OkIO | `okio.*` | I/O library | 67 |
| ZoomableImageView | `com.jsibbold.*` | Pinch-to-zoom images | 9 |
| jspecify | `org.jspecify.*` | Nullability annotations | 4 |

## Priority Ranking for Decompilation

### Priority 1: CRC64 App Code (678 classes in classes2.dex)
These are the Xamarin-generated Android wrappers for Aula's .NET business logic. While the actual business logic lives in the packed .NET assemblies (`libassemblies.x86_64.blob.so`), these classes reveal:
- Complete UI/Activity hierarchy and screen structure
- View binding and layout inflation patterns
- Android-specific integrations (push notifications, deep links, WebView bridges)
- CRC64 hash-to-namespace mapping (can be reversed with .NET assembly analysis)

### Priority 2: Xamarin/Mono Runtime (438 classes in classes2.dex)
Understanding the runtime bridge is needed to map between Java-side callbacks and .NET-side handlers. Key classes:
- `mono.MonoPackageManager` — assembly loading
- `mono.android.TypeManager` — type resolution between runtimes
- `AuthenticationHandler` — OAuth flow

### Priority 3: Not applicable
All remaining classes (93.1%) are standard third-party libraries. No decompilation needed — behavior is well-documented.

## Critical Architectural Insight

**The DEX files are NOT where the business logic lives.** This is a .NET MAUI/Xamarin app:
- DEX = Android bootstrap layer (12,392 classes, 12.8 MB)
- .NET assemblies = Actual app logic (`libassemblies.x86_64.blob.so`, 38.8 MB — 3x larger than DEX)

The CRC64 classes in DEX are auto-generated Android Callable Wrappers (ACWs) that expose .NET types to the Android runtime. Real reverse engineering requires extracting and decompiling the .NET assemblies.

However, the DEX analysis provides a complete map of the app's screen/activity architecture, which is valuable for understanding app structure before diving into .NET assembly analysis.
