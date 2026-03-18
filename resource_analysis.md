# Resource Files and Assets Analysis

**APK**: com.netcompany.aulanativeprivate v2.15.4
**Framework**: .NET MAUI / Xamarin (.NET assemblies packed in `libassemblies.x86_64.blob.so`)

---

## 1. Network Security Configuration

**No `network_security_config.xml` found.** The AndroidManifest.xml does not reference a network security configuration via `android:networkSecurityConfig`.

**Implications:**
- The app uses Android's default network security policy (for targetSdk 35):
  - Cleartext (HTTP) traffic is blocked by default
  - System CA store is trusted for HTTPS connections
  - No custom certificate pinning at the Android resource level
- **No certificate pinning is configured via Android resources.** If cert pinning exists, it would be implemented in the .NET layer (e.g., via `HttpClientHandler` or a custom `ServerCertificateCustomValidationCallback` in the C# code within the packed assemblies).
- The app sets `android:allowBackup="false"` and `android:allowClearUserData="false"`, indicating security-conscious defaults.

---

## 2. Hardcoded URLs and API Base URLs

### From Android Resources (strings.xml)

| Resource Name | Value | Purpose |
|--------------|-------|---------|
| `firebase_database_url` | `https://aula-private.firebaseio.com` | Firebase Realtime Database |
| `google_api_key` | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` | Google/Firebase API key |
| `google_crash_reporting_api_key` | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` | Same key, crash reporting |
| `google_storage_bucket` | `aula-private.appspot.com` | Firebase Storage bucket |
| `google_app_id` | `1:811573413698:android:4b7d0241cc907d17` | Firebase App ID |
| `gcm_defaultSenderId` | `811573413698` | FCM sender ID (GCP project number) |
| `project_id` | `aula-private` | Firebase/GCP project ID |
| `default_web_client_id` | `811573413698-mnjq3uvi6b23ajkghjp4rodbtgks0uqt.apps.googleusercontent.com` | OAuth2 web client ID |
| `library_name` | `AulaNative.Droid.Private` | .NET assembly name |

### From AndroidManifest.xml (Deep Links / Intent Filters)

| Domain | Context |
|--------|---------|
| `app-private.aula.dk` | OAuth callback deep link (autoVerify=true, WebAuthenticationCallbackActivity) |
| `*.aula.dk` | Query intent filter (BROWSABLE, VIEW, https) |
| `*.ncaula.com` | Query intent filter (BROWSABLE, VIEW, https) - staging environment |
| `com.netcompany.aulanativeprivate://onedrive2redirect` | OneDrive OAuth redirect (CloudStorageAuthInterceptor) |
| `com.netcompany.aulanativeprivate:/googleoauth2redirect` | Google Drive OAuth redirect (CloudStorageAuthInterceptor) |

### From .NET Assembly Blob (strings analysis)

The packed .NET assemblies reference:
- `IdentityModel.OidcClient.dll` - OpenID Connect client library
- `GO_TO_LOGIN_WEBVIEW` - navigation constant
- `PrepareLoginAsync` - async login method
- `ProfileByLoginTo` - profile lookup
- `GET_PRESENCE_CONFIGURATION_BY_CHILD_IDS` - API operation name
- `ValidateEndpoints` - endpoint validation logic

**Note:** The actual API base URLs (e.g., `https://api.aula.dk/...`) are embedded in the compressed .NET assemblies within `libassemblies.x86_64.blob.so` and cannot be extracted via simple `strings` analysis. Extracting these requires decompressing the Xamarin assembly blob (see follow-up task).

### Danish UI Strings (Tab Bar)

| Resource | Value | Translation |
|----------|-------|-------------|
| `tabbar_calendar` | Kalender | Calendar |
| `tabbar_gallery` | Galleri | Gallery |
| `tabbar_messages` | Beskeder | Messages |
| `tabbar_overview` | Overblik | Overview |

---

## 3. Embedded Databases and Data Files

### No Embedded Databases Found

No `.db`, `.sqlite`, `.realm`, or other database files found in the APK resources or assets.

However, the native library `libe_sqlite3.so` is included, indicating SQLite is used at runtime for local data storage (cache, session data, etc.).

### Data Files Catalogue

| File / Directory | Type | Purpose |
|-----------------|------|---------|
| `assets/font/` | 10 TTF files | Lato font family (regular, bold, light, hairline, italic variants) |
| `assets/AboutAssets.txt` | Text | Standard Xamarin assets readme |
| `res/font/` | 15 files (11 TTF + 4 XML) | Lato + Baloo font families with XML font-family declarations |
| `res/raw/firebase_common_keep.xml` | XML | Firebase ProGuard keep rules |
| `res/raw/*.svg` | 2 SVG files | Shadow vectors (circle, triangle) |
| `client_analytics.proto` | Protobuf | Firebase transport client analytics schema |
| `messaging_event.proto` | Protobuf | Firebase Cloud Messaging event schema |
| `messaging_event_extension.proto` | Protobuf | FCM event extension schema |
| `stamp-cert-sha256` | Binary | APK signing certificate stamp |
| `resources.arsc` | Binary | Compiled Android resource table |

### Properties Files (Library Versions)

| Library | Version |
|---------|---------|
| firebase-encoders | 17.0.0 |
| firebase-encoders-proto | 16.0.0 |
| firebase-iid-interop | 17.1.0 |
| firebase-measurement-connector | 20.0.1 |
| play-services-basement | 18.7.1 |
| play-services-base | 18.7.2 |
| play-services-cloud-messaging | 17.3.0 |
| play-services-stats | 17.1.0 |
| play-services-tasks | 18.3.2 |
| google_play_services_version | 12451000 |

### Native Libraries (in config.x86_64 split)

| Library | Purpose |
|---------|---------|
| `libassemblies.x86_64.blob.so` | Packed .NET assemblies (main business logic) |
| `libarc.bin.so` | Xamarin archive |
| `libxamarin-app.so` | Xamarin Android runtime |
| `libmonodroid.so` | Mono Android bridge |
| `libmonosgen-2.0.so` | Mono SGen garbage collector |
| `libe_sqlite3.so` | SQLite database engine |
| `libdatastore_shared_counter.so` | AndroidX DataStore |
| `libSystem.Native.so` | .NET System native |
| `libSystem.Security.Cryptography.Native.Android.so` | .NET crypto native |
| `libSystem.Globalization.Native.so` | .NET globalization native |
| `libSystem.IO.Compression.Native.so` | .NET compression native |

---

## 4. String Resources Scan (API Keys, Endpoints, Feature Flags)

### API Keys Found

| Key | Value | Risk Level |
|-----|-------|-----------|
| `google_api_key` | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` | Low - Firebase API keys are designed to be public, restricted by SHA-1/package name |
| `default_web_client_id` | `811573413698-mnjq3uvi6b23ajkghjp4rodbtgks0uqt.apps.googleusercontent.com` | Low - OAuth client ID, public by design |
| `gcm_defaultSenderId` | `811573413698` | Low - GCP project number, semi-public |

### Feature Flags / Configuration

No feature flags or toggles found in Android XML resources. These are likely managed:
1. Server-side (fetched at runtime via API)
2. In the .NET assemblies (compiled constants)
3. Via Firebase Remote Config (Firebase SDK is present)

### Authentication Architecture (from manifest analysis)

The app uses a WebView-based authentication flow:
- `LoginActivity` - login screen
- `LoginWithPinActivity` - PIN-based quick login
- `OTPSelectionActivity` - OTP/2FA selection
- `WebAuthenticationCallbackActivity` - OAuth callback handler for `https://app-private.aula.dk`
- `IdentityModel.OidcClient.dll` - OpenID Connect client in .NET layer

This suggests the app authenticates via an OIDC provider at `app-private.aula.dk`, likely using Denmark's national login infrastructure (UniLogin/MitID) as the identity provider.

### Cloud Storage Integration

The app integrates with:
- **OneDrive** - via `com.netcompany.aulanativeprivate://onedrive2redirect`
- **Google Drive** - via `com.netcompany.aulanativeprivate:/googleoauth2redirect`

Both use the `CloudStorageAuthInterceptor` activity.

### Other Notable Findings

- **Samsung-specific permission**: `com.samsung.android.providers.context.permission.WRITE_USE_APP_FEATURE_SURVEY`
- **Biometric auth**: Both `USE_FINGERPRINT` (deprecated) and `USE_BIOMETRIC` permissions
- **Phone calls**: `CALL_PHONE` permission - app can initiate calls (likely for calling teachers/parents)
- **Kotlin Multiplatform**: Project structure metadata shows KMP modules (iOS, Android, web, native)
- **File sharing**: FileProvider configured with paths for attachments, images, movies, downloads

---

## 5. Resource Statistics

| Category | Count |
|----------|-------|
| Drawable XML files | 472 |
| Drawable images (PNG/JPG/WEBP) | 7 |
| Layout XML files | ~100+ (layout, layout-land, layout-sw600dp) |
| Font files (TTF) | 21 (10 in assets + 11 in res) |
| Color resources | ~100+ (extensive Aula branding palette) |
| String resources | ~200 (mostly library strings, 4 Aula-specific) |
| Style definitions | ~50+ (multiple Aula themes) |
| XML config files | 4 (file paths, split config) |

---

## Key Takeaways

1. **No certificate pinning at Android level** - security relies on default TLS or .NET-level pinning
2. **Firebase is central** - project `aula-private` with Realtime Database, Cloud Messaging, Storage
3. **Business logic is opaque** - all app-specific URLs, API endpoints, and logic are in compressed .NET assemblies
4. **OIDC authentication** via `app-private.aula.dk` with IdentityModel library
5. **Cloud storage integration** with OneDrive and Google Drive
6. **No embedded databases** - SQLite used at runtime only
7. **Minimal hardcoded config** - most configuration appears to be runtime/server-driven
