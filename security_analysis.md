# Aula App Security Analysis

Security analysis of the decompiled Aula Android app (com.netcompany.aulanativeprivate, v2.15.4).
Xamarin.Android app by Netcompany A/S. minSdkVersion=29, targetSdkVersion=35.

## 1. Android Permission Mapping

The app declares 14 permissions in AndroidManifest.xml. Each is mapped to its actual usage in decompiled code below.

| # | Permission | Justified | Usage in Code |
|---|-----------|-----------|---------------|
| 1 | `INTERNET` | Yes | Core networking. All API calls via `SimpleService` -> `HttpClientManager` to `https://www.aula.dk/api/v23/` |
| 2 | `ACCESS_NETWORK_STATE` | Yes | Standard for checking connectivity before network operations |
| 3 | `CAMERA` | Yes | `MediaPickerActivity` uses `IMAGE_CAPTURE` and `VIDEO_CAPTURE` intents for photo/video attachments in posts, messages, albums. Properly requests runtime permission. |
| 4 | `CALL_PHONE` | Yes | `AndroidSpecialTextPhoneCallExecutioner` enables tapping phone numbers in messages/posts to initiate calls via `ACTION_CALL` intent. Requests runtime permission before calling. |
| 5 | `READ_EXTERNAL_STORAGE` | Yes | `MediaPickerActivity` and `FilePickerActivity` for selecting photos, videos, and files to upload as attachments. Only requested on Android <11. |
| 6 | `WRITE_EXTERNAL_STORAGE` | Yes | Same media/file picker flow for saving camera captures to temp storage. Only requested on Android <11. |
| 7 | `USE_BIOMETRIC` | Yes | `BiometricUtils` via `Plugin.Fingerprint` (`CrossFingerprint.Current`) for biometric login (fingerprint/face). Modern API. |
| 8 | `USE_FINGERPRINT` | Yes | Legacy fingerprint API (deprecated). Kept for backward compatibility alongside `USE_BIOMETRIC`. |
| 9 | `POST_NOTIFICATIONS` | Yes | `AndroidNotificationPermissionService` requests this for push notification delivery (required on Android 13+). |
| 10 | `WAKE_LOCK` | Yes | Standard Firebase Cloud Messaging requirement for reliable push notification delivery. |
| 11 | `com.google.android.c2dm.permission.RECEIVE` | Yes | Firebase Cloud Messaging (FCM) push notifications. `AulaFirebaseMessagingService` handles incoming messages. |
| 12 | `SYSTEM_ALERT_WINDOW` | No | Declared in manifest but **no usage found** anywhere: not in decompiled C#, not in smali/DEX code, not in Java libraries. No calls to `canDrawOverlays`, no `TYPE_APPLICATION_OVERLAY` or `TYPE_SYSTEM_ALERT` usage. The permission sits in the app-declared block (lines 3-12) of AndroidManifest.xml, not in the library-merged section (lines 43-46), confirming it was explicitly added by the developer -- not injected by Xamarin, Firebase, or any dependency. Xamarin only auto-adds `INTERNET` and `READ_EXTERNAL_STORAGE` in debug builds. This is an unnecessary, over-privileged permission that should be removed. |
| 13 | `com.samsung.android.providers.context.permission.WRITE_USE_APP_FEATURE_SURVEY` | Unclear | Samsung-specific analytics permission. **No usage found** in decompiled C# code. Likely injected by a Samsung-specific SDK or build toolchain dependency. |
| 14 | `DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` | Yes | Custom app-level permission. Standard AndroidX pattern for securing dynamically registered broadcast receivers (added automatically by AndroidX Core 1.7+). |

**Assessment**: 12 of 14 permissions have clear, justified usage. `SYSTEM_ALERT_WINDOW` is confirmed unused and explicitly app-declared (not a framework or library artifact) -- it allows drawing over other apps and should be removed. The Samsung permission is likely a third-party SDK artifact.

## 2. TLS and Certificate Handling

### 2.1 .NET Layer: Certificate Pinning

The app implements certificate pinning at the .NET/Mono level via `CertificatePinningUtils`:

**Initialization**: Called during `MainApplication.Init()` at app startup.

**Mechanism**: Hooks `ServicePointManager.ServerCertificateValidationCallback` to intercept all .NET HTTPS connections. For certificates with Subject containing `*.aula.dk`, it computes a SHA-256 hash of the Subject Public Key Info (SPKI) and validates against a pinned set.

**Pinned keys**:
- Production (`aula.dk`): 3 keys
  - `/P3+fgXhRH6jPoKBMmAKWRrtjDoEZf4ySjxLoQuqsYc=`
  - `eLCo7AWQ2P88/2FQfow993oOhcjXal2sS/e2mZgJLJE=`
  - `9XtneGQWNOLQFi0f8LEJ62bt1f/pVrCb4ytT66RcurA=`
- Test (`ncaula.com`): 3 keys
  - `ejsQt33CcKZWEoO/ym2mcdSynXrVfK1o6QbTI868tDE=`
  - `PfUUWB6dvdMA9exWlx0W+6lKT540ElcRWUERcBRtP6o=`
  - `CC09RfvRZQ1z+bj1VeJ/jrYOeH3D0epyQR+FEXLddF8=`

**Key observation**: The hash is computed via `X509CertificateHelper.GetPublicKeyPinningHash()` which does a SHA-256 of the raw SPKI bytes. This is the standard HPKP-style public key pinning approach.

**Weakness**: The validation only checks certificates whose Subject contains `*.aula.dk`. All non-aula.dk connections (third-party services, CDNs, etc.) **fall through with `return true`**, bypassing pinning. This is by design but means only first-party Aula traffic is pin-protected.

### 2.2 DEX Layer: ServerCertificateCustomValidator

The DEX contains `xamarin.android.net.ServerCertificateCustomValidator` with `AlwaysAcceptingHostnameVerifier` and `FakeSSLSession` classes. These are **Xamarin.Android framework internals**, not app code. They exist as part of the Xamarin/MAUI `AndroidMessageHandler` implementation to bridge .NET's certificate validation model to Android's Java TLS stack.

The way this works: Xamarin intercepts the Java-level TLS validation and delegates it to the .NET `ServicePointManager.ServerCertificateValidationCallback`. The `AlwaysAcceptingHostnameVerifier` at the Java layer is intentional -- it says "let .NET handle the validation" rather than duplicating it at the Java level.

**This is not a security vulnerability in itself.** The actual certificate validation happens at the .NET layer where `CertificatePinningUtils` enforces pinning. However, it means there is no Android-level `network_security_config.xml` cert pinning -- it is all handled in .NET code.

### 2.3 No network_security_config.xml

Confirmed: No `network_security_config.xml` exists. Android-level cleartext traffic restrictions rely on the default behavior for targetSdkVersion=35 (cleartext blocked by default). Certificate pinning is handled entirely in .NET code rather than via Android's native mechanism.

## 3. Authentication and Token Storage

### 3.1 OIDC Authentication

- **Protocol**: OpenID Connect Authorization Code + PKCE via `IdentityModel.OidcClient`
- **Provider**: SimpleSAMLphp at `login.aula.dk`
- **Two authentication levels**: Level 2 (UniLogin, scope `aula`) and Level 3 (MitID/NemID, scope `aula-sensitive`)
- **PKCE**: Standard code challenge generation protects against authorization code interception
- **No client-side JWT validation**: `ProviderInformation.KeySet` is set to empty `JsonWebKeySet`. The app trusts the token endpoint implicitly. This is acceptable since the token endpoint is TLS+pinned, but it means a compromised token endpoint could issue arbitrary tokens.

### 3.2 Token Storage

Tokens are stored via:
```
SecureStorageManager -> SecureStorageHelper -> ISecureStorageHelper
  -> DroidSecureStorageHelper -> Plugin.SecureStorage (CrossSecureStorage)
    -> ProtectedFileImplementation (file-based AES encryption with Android Keystore-backed keys)
```

Values are JSON-serialized via Newtonsoft.Json before storage. Thread-safe via `SemaphoreSlim(1,1)`.

**Stored sensitive data**:
- Full `LoginData` (access token, refresh token, expiration) under `AuthenticationManager.ServiceName` key
- PIN code under `PRIVATE_KEYSTORE_PINCODE_KEY`
- Biometric auth state, auth level, portal role

### 3.3 PIN Code Validation

PIN validation in `PinCodeViewModel.ValidatePin()` is a **plain string comparison**:
```csharp
string pinCode = SecureStorageManager.GetPinCode();
ShowConfirmBox = (pinCode != enteredPin);
```

Concerns:
- **No brute-force protection**: No lockout after failed attempts, no rate limiting, no exponential backoff. A user (or attacker with device access) can try unlimited PINs.
- **PIN stored as plaintext string**: The PIN is stored as a raw string in SecureStorage, not hashed. While SecureStorage provides encryption-at-rest, the PIN is decrypted into memory for comparison.
- **No timing-safe comparison**: Uses `!=` operator which may be vulnerable to timing attacks (though practically difficult to exploit on a local device).
- **PIN login bypasses OIDC**: Successful PIN validation reuses persisted tokens without contacting the identity provider, which is the expected behavior but means PIN compromise = full session compromise.

### 3.4 Biometric Authentication

Uses `Plugin.Fingerprint` library via `BiometricPrompt` API. If biometric succeeds, the persisted tokens are used directly (same as PIN login). No separate cryptographic binding between biometric and token storage (biometric just gates access to the already-stored tokens).

### 3.5 Session Management

- 60-minute session timeout with 55-minute warning
- `SessionPromptManager` tracks activity via background timer
- Keep-alive endpoint available to extend sessions
- CSRF protection via `Csrfp-Token` cookie/header pattern

## 4. Data Storage and Encryption

### 4.1 SQLite Encryption

The app uses a custom `EncryptionManager` for encrypting SQLite data:

**Key derivation**:
1. Generates a 256-character random password from `[a-zA-Z0-9]` using `RNGCryptoServiceProvider`
2. Derives a 32-byte AES key via `Rfc2898DeriveBytes` (PBKDF2) with:
   - **Salt**: Hardcoded 8-byte value (compiled from IL, exact bytes not visible but static)
   - **Iterations**: 300

**Encryption**: AES-CBC with random IV per encryption. Ciphertext stored as `base64(ciphertext);base64(IV)`.

**Encryption key persistence**: The generated password is stored in SecureStorage under key `"EncryptionManager"`. This means the SQLite encryption key ultimately depends on Android Keystore security.

**Encrypted fields** (via `[Encrypt]` attribute on model properties):
- `LogItem` (app logs)
- `MessageThreadDraft`, `MessageDraft` (message drafts)
- `DeviceSetting` (device configuration)
- `AppSettingsModel` (app settings)
- `PostDraft` (post drafts)
- `LoginSetting` (login settings)
- `LocalDelegatedContextModel` (delegation data)

**Weaknesses**:
- **Hardcoded salt**: The PBKDF2 salt is a static byte array compiled into the assembly. This reduces the effectiveness of the key derivation (all installations use the same salt).
- **Only 300 PBKDF2 iterations**: Modern recommendations are 600,000+ for PBKDF2-SHA1 or 210,000+ for PBKDF2-SHA256. 300 iterations provides negligible resistance to brute-force if the password is somehow extracted.
- **Error handling wipes DB**: If encryption/decryption fails, `EncrpytionHelpers` calls `ForceDeleteAllTablesDontPersist()` and creates a new `EncryptionManager` instance. This is a data-loss failsafe but could be triggered by corruption.

### 4.2 Secure Storage

`DroidSecureStorageHelper` wraps `Plugin.SecureStorage` (`CrossSecureStorage`), which on Android uses `ProtectedFileImplementation` -- a file-based encryption scheme with AES keys stored in the Android Keystore. This does NOT use `EncryptedSharedPreferences` (see section 4.3 for details on why EncryptedSharedPreferences/Tink are bundled but unused).

**Notable quirk**: The `Save<T>` method loops to delete existing keys before saving (`while (CrossSecureStorage.Current.HasKey(key)) { DeleteKey(key); }`). This suggests the underlying storage sometimes returns stale values.

### 4.3 Google Tink Cryptographic Library (Bundled but Unused)

The APK includes the full Google Tink cryptographic library (486 Java classes in `com.google.crypto.tink.*` across classes2.dex) along with AndroidX Security Crypto (`EncryptedSharedPreferences`, `EncryptedFile`, `MasterKey` in classes.dex). Despite this substantial presence, **no app-level code actually calls into these libraries**.

**Evidence of non-use**:
- Zero cross-references from any CRC64 Xamarin Android Callable Wrapper to `EncryptedSharedPreferences`, `EncryptedFile`, `MasterKey`, or any Tink class
- Zero imports of `androidx.security.crypto.*` or `com.google.crypto.tink.*` outside the library packages themselves
- Zero smali cross-references from app code to these library classes
- The .NET `AulaNative.Droid.csproj` does not reference `Xamarin.Google.Crypto.Tink.Android.dll`
- The .NET `EncryptionManager` uses `System.Security.Cryptography.Aes` (standard .NET BCL), not Tink

**Why Tink is bundled**: Tink is a transitive dependency of `androidx.security:security-crypto`, which is commonly pulled in by AndroidX dependency graphs or Xamarin NuGet packages. The `EncryptedSharedPreferences` and `EncryptedFile` classes are present because the library was included at build time, but the app never instantiates them.

**Tink modules bundled** (all unused):

| Module | Key Types | Purpose |
|--------|-----------|---------|
| `tink.aead` | AES-GCM, AES-CTR-HMAC, AES-EAX, AES-GCM-SIV, ChaCha20-Poly1305, XChaCha20-Poly1305 | Authenticated Encryption with Associated Data |
| `tink.daead` | AES-SIV | Deterministic AEAD (for encrypting preference keys) |
| `tink.streamingaead` | AES-GCM-HKDF, AES-CTR-HMAC streaming | Streaming file encryption |
| `tink.mac` | HMAC, AES-CMAC | Message authentication codes |
| `tink.prf` | HMAC-based PRF | Pseudorandom functions |
| `tink.hybrid` | ECIES | Hybrid encryption |
| `tink.jwt` | JWT signing/verification | JSON Web Token handling |
| `tink.integration.android` | AndroidKeysetManager, AndroidKeystoreKmsClient | Android Keystore integration |
| `tink.shaded.protobuf` | N/A | Shaded protobuf runtime for key serialization |

**Actual crypto mechanisms used by the app** (separate from Tink):

| Component | Crypto Used | Key Management |
|-----------|-------------|----------------|
| `.NET EncryptionManager` | AES-CBC (System.Security.Cryptography) | PBKDF2-derived key (300 iterations, hardcoded salt), password stored in Plugin.SecureStorage |
| `Plugin.SecureStorage` | File-based AES via `ProtectedFileImplementation` with `SecretKey` | Android Keystore-backed AES keys |
| `libSystem.Security.Cryptography.Native.Android.so` | .NET BCL crypto (AES, RSA, ECDSA, etc.) | JNI bridge to Android crypto provider |

**Conclusion**: Tink and EncryptedSharedPreferences add approximately 500+ classes of dead code to the APK. The app relies entirely on its own .NET `EncryptionManager` (for SQLite field encryption) and `Plugin.SecureStorage`/`ProtectedFileImplementation` (for credential/token storage), both backed by the Android Keystore but not through Tink.

## 5. Exported Components

Components exported in AndroidManifest.xml (accessible to other apps):

| Component | Type | Risk |
|-----------|------|------|
| `MainActivity` | Activity | Low -- main app entry point, requires authentication |
| `CloudStorageAuthInterceptor` | Activity | Medium -- handles OAuth redirects for Google Drive/OneDrive; uses `singleTask` launch mode with deep link intent filters for `/googleoauth2redirect` and `/onedrive2redirect` |
| `WebAuthenticationCallbackActivity` | Activity | Medium -- handles OIDC callback redirects from `https://app-private.aula.dk`; `singleTop` + `noHistory` |
| `LauncherActivity` | Activity | Low -- standard launcher entry point with `MAIN`/`LAUNCHER` intent filter |
| `MoreMenuActivity` | Activity | Low -- settings/more menu, exported but requires authentication |
| `SettingsEditShortcutsActivity` | Activity | Low -- shortcut editor, exported but requires authentication |
| `SettingsOverviewActivity` | Activity | Low -- settings overview, exported but requires authentication |
| `FirebaseInstanceIdReceiver` | Receiver | Low -- protected by `com.google.android.c2dm.permission.SEND` |
| `ProfileInstallReceiver` | Receiver | Low -- AndroidX baseline profile installer; protected by `android.permission.DUMP` |

The OAuth callback activities are necessarily exported (they receive redirects from the browser), but this is a standard pattern. The `MoreMenuActivity`, `SettingsEditShortcutsActivity`, and `SettingsOverviewActivity` are exported without clear necessity -- they could potentially be launched by other apps on the device, though authentication state should prevent unauthorized access.

## 6. Other Security Observations

### 6.1 Hardcoded Test Credentials
Non-production environments use basic auth: `aula-user:Aula-1337`. These credentials are compiled into the app in `Conf.cs`. While this only affects test environments (gated by `EnvConfig.IsProduction`), shipping test credentials in a production APK is poor practice.

### 6.2 No android:debuggable Flag
The manifest does not explicitly set `android:debuggable`. With targetSdkVersion=35, this defaults to `false` in release builds, which is correct.

### 6.3 allowBackup=false
The app correctly sets `android:allowBackup="false"`, preventing ADB backup of app data (including encrypted tokens and SQLite database).

### 6.4 No Root/Jailbreak Detection
No evidence of root detection, SafetyNet/Play Integrity attestation, or tamper detection in the decompiled code. A rooted device could access SecureStorage values directly.

### 6.5 Client IDs in App
OIDC client IDs are compiled into the app:
- Level 2: `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6`
- Level 3: `_99949a54b8b65423862aac1bf629599ed64231607a`

This is standard for mobile OIDC (public clients), and PKCE mitigates the risk. However, it means anyone can initiate an OAuth flow against the Aula identity provider.

### 6.6 EndOfSupportDate
`Conf.cs` contains `EndOfSupportDate` set to June 15, 2025. This version will stop receiving updates after that date.

## 7. Security Posture Summary

### Strengths
1. **Certificate pinning** on production domains (SPKI hash pinning, 3 keys for rotation)
2. **OIDC with PKCE** for authentication (industry standard)
3. **Two-factor step-up** for sensitive operations (Level 3 auth via MitID)
4. **SecureStorage** backed by Android Keystore for token and credential storage
5. **allowBackup=false** prevents ADB data extraction
6. **SQLite encryption** for locally cached data (drafts, settings, logs)
7. **CSRF protection** with cookie-to-header token pattern
8. **Runtime permissions** properly requested before use (camera, phone, storage)
9. **60-minute session timeout** with user warning

### Concerns

| # | Finding | Severity | Description |
|---|---------|----------|-------------|
| 1 | PIN brute-force | Medium | No lockout or rate limiting on PIN attempts. Local attacker with device access can try all 4-6 digit PINs quickly. |
| 2 | PBKDF2 with 300 iterations | Low-Medium | SQLite encryption key derivation uses only 300 PBKDF2 iterations with a hardcoded salt. Far below OWASP minimum of 600,000. |
| 3 | SYSTEM_ALERT_WINDOW permission | Low | Explicitly declared by app developer but completely unused -- no code references in C#, smali, or Java. Not a Xamarin artifact (Xamarin only auto-adds INTERNET and READ_EXTERNAL_STORAGE). Not contributed by any library via manifest merger. Grants overlay capability that could be exploited if the app is compromised. Should be removed. |
| 4 | No root detection | Medium | No checks for rooted devices. Android Keystore is weaker on rooted devices, potentially exposing SecureStorage contents. |
| 5 | Test credentials in production APK | Low | `aula-user:Aula-1337` for test environments compiled into app. Information disclosure. |
| 6 | Hardcoded PBKDF2 salt | Low | Same salt across all installations reduces key derivation entropy. |
| 7 | Cert pinning bypass for non-aula domains | Info | Non-aula.dk connections (third-party services) are not pin-validated. By design, but worth noting. |
| 8 | No JWT validation on client | Info | App trusts token endpoint implicitly. Standard for mobile OIDC but reduces defense-in-depth. |
| 9 | PIN stored as plaintext in SecureStorage | Low | PIN is not hashed before storage. SecureStorage provides encryption, but the PIN is in memory as plaintext during validation. |
