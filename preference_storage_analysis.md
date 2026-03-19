# Preference & Local Storage Analysis (TASK-82)

## Executive Summary

The Aula app uses **four distinct local storage mechanisms** across its Java/Kotlin and .NET layers.
AndroidX DataStore is present but used exclusively by Firebase internals (confirmed by TASK-78).
The app's own preferences are managed through .NET `SecureStorage` (backed by Android's
`Plugin.SecureStorage` / `EncryptedSharedPreferences`) and MAUI `Preferences` (backed by Android
`SharedPreferences`). Several Google/Firebase libraries also use plain `SharedPreferences` for their
own housekeeping.

---

## Storage Mechanism Overview

| Mechanism | Layer | Backing Store | Encryption | Used By |
|-----------|-------|--------------|------------|---------|
| AndroidX DataStore Preferences | Java/Kotlin | `.preferences_pb` (protobuf) | No | Firebase only |
| Android SharedPreferences | Java/Kotlin | XML files in `shared_prefs/` | No | Google/Firebase SDKs, AndroidX |
| EncryptedSharedPreferences | Java/Kotlin | XML files (AES-256 encrypted) | Yes (Tink AES-SIV + AES-GCM) | .NET SecureStorage via Plugin.SecureStorage |
| MAUI Preferences | .NET (C#) | Android SharedPreferences | No | Aula session management |

---

## 1. AndroidX DataStore Preference Keys (Firebase Only)

DataStore v1.1.7 is bundled but **no Aula application code references it**. All usage is confined
to Firebase heartbeat tracking.

| Preference File | Key | Type | Purpose | Consumer |
|----------------|-----|------|---------|----------|
| `FirebaseHeartBeat*.preferences_pb` | `fire-global` | Long | Timestamp of last global heartbeat | `HeartBeatInfoStorage` |
| `FirebaseHeartBeat*.preferences_pb` | `fire-count` | Long | Number of stored heartbeats (limit: 30) | `HeartBeatInfoStorage` |
| `FirebaseHeartBeat*.preferences_pb` | `last-used-date` | String | ISO date of last heartbeat | `HeartBeatInfoStorage` |
| `FirebaseHeartBeat*.preferences_pb` | *(dynamic SDK names)* | Set\<String\> | Dates each SDK sent a heartbeat | `HeartBeatInfoStorage` |

---

## 2. Android SharedPreferences (Java/Kotlin Layer)

These are all from Google/Firebase SDK code, not Aula application code.

### 2.1 Firebase Messaging (`com.google.firebase.messaging`)

| Key | Type | Purpose | Consumer |
|-----|------|---------|----------|
| `auto_init` | Boolean | Whether FCM auto-initialization is enabled | `FirebaseMessaging.AutoInit` |
| `proxy_notification_initialized` | Boolean | Whether proxy notification system has been initialized | `ProxyNotificationPreferences` |
| `proxy_retention` | Boolean | Whether proxied notifications should be retained | `ProxyNotificationPreferences` |
| `export_to_big_query` | Boolean | Whether to export delivery metrics to BigQuery | `MessagingAnalytics` |

### 2.2 Firebase App ID / Messaging Token Store (`com.google.android.gms.appid`)

| Key Pattern | Type | Purpose | Consumer |
|-------------|------|---------|----------|
| `\|T\|{senderId}\|{scope}` | String (JSON) | FCM registration token with app version and timestamp | `Store` / `IidStore` |
| `\|S\|id` | String | Firebase Instance ID | `IidStore` |
| `\|S\|\|P\|` | String (Base64) | RSA public key for instance ID derivation | `IidStore` |
| `topic_operation_queue` | String (CSV) | Pending topic subscribe/unsubscribe operations | `TopicsStore` |

### 2.3 Firebase Data Collection (`com.google.firebase.common.prefs:{appName}`)

| Key | Type | Purpose | Consumer |
|-----|------|---------|----------|
| `firebase_data_collection_default_enabled` | Boolean | Master toggle for Firebase data collection | `DataCollectionConfigStorage` |

### 2.4 Google Sign-In (`com.google.android.gms.signin`)

| Key Pattern | Type | Purpose | Consumer |
|-------------|------|---------|----------|
| `defaultGoogleSignInAccount` | String | ID of default signed-in Google account | `Storage` |
| `googleSignInAccount:{accountId}` | String (JSON) | Serialized `GoogleSignInAccount` | `Storage` |
| `googleSignInOptions:{accountId}` | String (JSON) | Serialized `GoogleSignInOptions` | `Storage` |
| `refreshToken` | String | OAuth refresh token for Google Sign-In | `Storage` |

### 2.5 Google Tink Cryptographic Keysets

Tink stores its cryptographic keysets in SharedPreferences. The `prefFileName` and `keysetName` are
configured by the caller (typically `EncryptedSharedPreferences`). When `prefFileName` is null,
the app's default SharedPreferences are used.

| Key | Type | Purpose | Consumer |
|-----|------|---------|----------|
| `{keysetName}` (configurable) | String (hex-encoded protobuf) | Encrypted or cleartext Tink keyset | `SharedPrefKeysetWriter` / `AndroidKeysetManager` |

For `EncryptedSharedPreferences`, this typically means two keysets are stored:
- A key keyset (for encrypting preference keys, using AES-SIV)
- A value keyset (for encrypting preference values, using AES-GCM)

### 2.6 AndroidX App Launch Checker (`android.support.AppLaunchChecker`)

| Key | Type | Purpose | Consumer |
|-----|------|---------|----------|
| `startedFromLauncher` | Boolean | Whether app was ever started from launcher | `AppLaunchChecker` |

---

## 3. .NET SecureStorage Keys (Aula Application Layer)

The Aula app uses `Plugin.SecureStorage` (`CrossSecureStorage.Current`) which on Android is backed
by `EncryptedSharedPreferences`. All values are JSON-serialized before storage. The
`DroidSecureStorageHelper` class wraps this with a `SemaphoreSlim` for thread safety.

### 3.1 Login & Authentication

| Key | Value Type | Purpose | Consumer |
|-----|-----------|---------|----------|
| `AulaNativeOAuthPrivate` | `LoginData` (JSON) | OAuth login data (access token, refresh token, expiry, etc.) | `AuthenticationManager` / `MigrationsHelper` |
| `PRIVATE_KEYSTORE_PINCODE_KEY` | `Dict<string,string>` (JSON) | User's PIN code for app lock | `SecureStorageManager` |
| `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY` | `Dict<string,string>` (JSON, int value) | Authenticator level at login time (MitID level, etc.) | `SecureStorageManager` |
| `PRIVATE_SENSITIVITY_LEVEL_KEY` | `Dict<string,string>` (JSON, int value) | Persisted authentication method type (`LoginAuthenticationMethod` enum: values 2, 3, 9) | `SecureStorageManager` |
| `PRIVATE_PORTAL_ROLE_KEY` | `Dict<string,string>` (JSON, `PortalRole` enum) | User's portal role (Guardian, Employee, Child) | `SecureStorageManager` |
| `PRIVATE_BIO_AUTH_ACTIVATED_KEY` | `Dict<string,string>` (JSON, "1"/"0") | Whether biometric authentication is activated | `SecureStorageManager` |

### 3.2 App State & UX

| Key | Value Type | Purpose | Consumer |
|-----|-----------|---------|----------|
| `PRIVATE_COUNT_USAGE_MESSAGE_MODULE` | `Dict<string,string>` (JSON, int) | Number of times messaging module used (triggers app review prompt at 15) | `SecureStorageManager` |
| `PRIVATE_DID_PROMPT_REVIEW` | `Dict<string,string>` (JSON, "1") | Whether user was prompted for App Store review | `SecureStorageManager` |
| `PRIVATE_MIGRATION_COUNT_KEY` | `Dict<string,string>` (JSON, int) | Data migration version counter (0-5) | `SecureStorageManager` |
| `com.netcompany.aulanativeprivate.EditShortcuts` | `List<BottomNavigationItemDTO>` (JSON) | User's custom bottom navigation tab order | `EditShortcutsViewModel` / `AulaMainPageViewModel` |
| `EditShortcuts` | `List<BottomNavigationItemDTO>` (JSON) | Legacy key for bottom nav order (migration fallback) | `AulaMainPageViewModel` |

### 3.3 Encryption

| Key | Value Type | Purpose | Consumer |
|-----|-----------|---------|----------|
| `EncryptionManager` | `Dict<string,string>` (JSON, contains `SQLEncryptionKey`) | AES encryption key for SQLite database (256-char random alphanumeric password, derived via PBKDF2/Rfc2898) | `EncryptionManager` |

### 3.4 OS Update Acknowledgment

| Key | Value Type | Purpose | Consumer |
|-----|-----------|---------|----------|
| `AcknowledgeOSUpdateKey` | `DateTime` (JSON) | Timestamp when user last acknowledged OS update notification | `SharedNotificationDetailViewHelper` |

---

## 4. MAUI Preferences Keys (Aula Application Layer)

These use `Microsoft.Maui.Storage.Preferences` which wraps Android `SharedPreferences` (not
encrypted). Used only by `SessionPromptManager` for transient session timing.

| Key | Type | Purpose | Consumer |
|-----|------|---------|----------|
| `lastActiveWhenPromptStart` | String (DateTime formatted "G" InvariantCulture) | Timestamp when session expiry prompt was shown | `SessionPromptManager` |
| `lastActiveBeforeBackground` | String (DateTime formatted "G" InvariantCulture) | Last active timestamp before app went to background | `SessionPromptManager` |

These are **transient** values -- they are cleared after use and serve only to preserve session
timing across app foreground/background transitions.

---

## 5. Security Observations

### Positive Findings

1. **Sensitive data uses EncryptedSharedPreferences**: OAuth tokens, PIN codes, encryption keys,
   and authentication state are all stored via `Plugin.SecureStorage` which uses AndroidX
   `EncryptedSharedPreferences` with Tink AES-256 encryption.
2. **SQLite encryption key is stored securely**: The `EncryptionManager` generates a 256-character
   random key and stores it in SecureStorage, using PBKDF2 key derivation.
3. **Session timestamps are not sensitive**: The MAUI `Preferences` (unencrypted) are only used
   for transient session timing, not for credentials or tokens.

### Concerns

1. **PIN code stored in SecureStorage**: While encrypted at rest, the PIN is stored as a plain
   string value within the JSON dictionary. If SecureStorage is compromised, the PIN is immediately
   usable. See TASK-84 for PIN brute-force analysis.
2. **No key rotation**: The SQLite encryption key is generated once and never rotated. If the
   SecureStorage key is compromised, all local database content is exposed.
3. **Migration count as security boundary**: The migration count gates which data migrations run.
   If this value is manipulated, migrations could be re-triggered or skipped.
4. **Biometric flag is a simple boolean**: The biometric authentication flag (`"1"` / `"0"`) is
   stored as a preference, not as a cryptographic assertion. Modifying this flag could bypass
   biometric requirements (though SecureStorage encryption makes this difficult in practice).

---

## 6. Data Flow Summary

```
                    Aula .NET Application
                    =====================
                           |
              +------------+-------------+
              |                          |
    SecureStorageHelper           MAUI Preferences
    (ISecureStorageHelper)        (Microsoft.Maui.Storage)
              |                          |
    DroidSecureStorageHelper         Android
    (Plugin.SecureStorage)        SharedPreferences
              |                    (plaintext XML)
    CrossSecureStorage.Current
              |
    EncryptedSharedPreferences
    (AndroidX Security Crypto)
              |
        +-----+------+
        |            |
    Tink AES-SIV  Tink AES-GCM
    (key encrypt)  (value encrypt)
        |            |
    Android SharedPreferences
    (encrypted XML in shared_prefs/)



                    Java/Kotlin SDK Layer
                    =====================
                           |
        +--------+---------+--------+---------+
        |        |         |        |         |
    Firebase   Firebase  Firebase  Google   AndroidX
    DataStore  Messaging  App     Sign-In  AppLaunch
    (protobuf) (xml)     Config   (xml)    Checker
                         (xml)             (xml)
```

---

## 7. Complete Key Inventory

Total unique preference keys identified: **~30+** (exact count varies due to dynamic keys)

| Category | Count | Storage | Encrypted |
|----------|-------|---------|-----------|
| Aula Authentication | 6 | SecureStorage | Yes |
| Aula App State/UX | 5 | SecureStorage | Yes |
| Aula Encryption | 1 | SecureStorage | Yes |
| Aula Session Timing | 2 | MAUI Preferences | No |
| Firebase Heartbeat (DataStore) | 4+ | DataStore protobuf | No |
| Firebase Messaging | 4 | SharedPreferences | No |
| Firebase App ID/Token | 4+ | SharedPreferences | No |
| Firebase Data Collection | 1 | SharedPreferences | No |
| Google Sign-In | 4 | SharedPreferences | No |
| Tink Keysets | 2 | SharedPreferences | Keyset encrypted by Android Keystore |
| AndroidX AppLaunch | 1 | SharedPreferences | No |
