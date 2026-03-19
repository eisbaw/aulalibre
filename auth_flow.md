# Aula Authentication Flow Analysis

## Overview

The Aula app uses **OpenID Connect (OIDC)** via the `IdentityModel.OidcClient` library, with a **SimpleSAMLphp**-based OIDC provider hosted at `login.aula.dk`. The implementation supports two authentication levels (step-up authentication) with different client IDs and scopes.

## 1. OIDC Provider

The authentication backend is a **SimpleSAMLphp** instance with an OIDC module:

| Parameter | Value |
|---|---|
| **Issuer/Authority** | `https://login.aula.dk/` |
| **Authorize Endpoint** | `https://login.aula.dk/simplesaml/module.php/oidc/authorize.php` |
| **Token Endpoint** | `https://login.aula.dk/simplesaml/module.php/oidc/token.php` |
| **Logout Endpoint** | `https://login.aula.dk/auth/logout.php` |
| **Redirect URI** | `https://app-private.aula.dk` |
| **Custom Scheme Callback** | `com.netcompany.aulanativeprivate` |

Source: `Urls.cs` defines these as `AulaUrl` objects (IDs 101-103, 1501), built dynamically from `Conf.AuthBackendUrl`.

### Environment Configuration

The app supports multiple environments, selected via `AulaNative.Configuration.EnvironmentFactory`:

| Environment | Backend Host | Auth Host | IsProduction |
|---|---|---|---|
| **Production** | `www.aula.dk` | `login.aula.dk` | true |
| **Pre-prod** | `www1-preprod.aula.dk` | `login-preprod.aula.dk` | true |
| **Hotfix** | `www1-hotfix.aula.dk` | `login-hotfix.aula.dk` | true |
| **Test1** | `www1-test1.ncaula.com` | `www1-test1.ncaula.com` | false |
| **Test3** | `www1-test3.ncaula.com` | `www1-test3.ncaula.com` | false |
| **Dev1** | `www1-dev1.ncaula.com` | `www1-dev1.ncaula.com` | false |
| **Dev3** | `www1-dev3.ncaula.com` | `www1-dev3.ncaula.com` | false |
| **Dev11** | `www1-dev11.ncaula.com` | `www1-dev11.ncaula.com` | false |
| **Dev21** | `www1-dev21.ncaula.com` | `www1-dev21.ncaula.com` | false |
| **Dev22** | `www1-dev22.ncaula.com` | `www1-dev22.ncaula.com` | false |
| **Dev31** | `www1-dev31.ncaula.com` | `www1-dev31.ncaula.com` | false |
| **Dev32** | `www1-dev32.ncaula.com` | `www1-dev32.ncaula.com` | false |
| **CI** | `www1-dev1.ncaula.com` | `www1-dev1.ncaula.com` | false |

Non-production environments (`IsProduction: false`) use basic auth credentials: `aula-user:Aula-1337`.
Pre-prod and Hotfix are marked `IsProduction: true` (no basic auth) but use `ncaula.com` data/notification hosts.
See `ncaula_staging_domain.md` for the complete 6-field environment matrix including data and notification hosts.

## 2. Step-Up Authentication (Levels 2 and 3)

The app supports two authentication levels for accessing different sensitivity tiers:

| Level | Client ID | Scope | Purpose |
|---|---|---|---|
| **Level 2** | `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6` | `aula` | Standard login (UniLogin) |
| **Level 3** | `_99949a54b8b65423862aac1bf629599ed64231607a` | `aula-sensitive` | Elevated access (MitID/NemID) for secure documents |

The level is stored in `SecureStorageManager` under key `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY`. Step-up is required for operations like viewing/editing secure documents ("sikre filer") and modifying consents.

The authentication level is determined by `SecureStorageManager.GetAuthenticatorLevelDuringLogin()` and selected dynamically when `CreateOidcClient()` is called. The client IDs are initialized in `AuthenticationState` from `Conf.CLIENTID_STEP_LEVEL_TWO` and `Conf.CLIENTID_STEP_LEVEL_THREE`.

### LoginAuthenticationMethod Enum

The `LoginAuthenticationMethod` enum (in `AulaNative.Enums`) defines the persisted auth type:

| Value | Name | Description |
|---|---|---|
| 0 | `Unknown` | Default/unset |
| 2 | `Level2` | Standard login (UniLogin) |
| 3 | `Level3NemId` | Elevated login (MitID/NemID) |
| 9 | `Level3Employees` | Elevated login for staff users |

The persisted auth type is stored under key `PRIVATE_SENSITIVITY_LEVEL_KEY` via `SecureStorageManager.SetPersistenAuthType()`. The mapping from login level to auth method is in `AuthenticationManager.SaveSensitivityLevel()`: level 2 maps to `Level2`, anything else maps to `Level3NemId`.

## 3. Login Flow (Step by Step)

### 3.1 UI Entry Point and Initial Routing

```
LauncherActivity (Android entry point)
  -> LoginActivity.OnCreate()
    -> LoginPageViewModel.ViewWillAppear()
      -> Checks if PIN is set (SecureStorageManager.GetPinCode())
    -> LoginPageViewModel.InitLogin()
      -> GetPersistedAccountAndCalculateExpirationTime()
      -> If account exists AND PIN is set:
           -> Controller.EnterPincode() -> LoginWithPinActivity
      -> If no account or no PIN:
           -> ProfileManager.ResetProfileDataOnDevice()
           -> Controller.LoadLoginOptions() -> shows role selection buttons
```

The login screen shows three role buttons: Employee (EMPLOYEE), Guardian/Parent (PARENT), and Child (CHILD). Each has accessibility labels defined in `LoginPageViewModel`.

### 3.2 ClickedLoginButton Flow

When a user clicks a role button:

```
LoginPageViewModel.ClickedLoginButton(PortalRole role, int loginLevel)
  1. CleanProfile() - resets any existing profile data
  2. OAuthInProgress = true
  3. SecureStorageManager.SetPortalRole(role) - persists selected role
  4. SecureStorageManager.SetAuthenticatorLevelDuringLogin(loginLevel) - persists auth level
  5. AuthenticationManager.RequestRefreshTokenAsync(account=null, controller)
     -> Since account is null/expired, calls AulaRequestAccessTokenAsync()
     -> On first failure, retries AulaRequestAccessTokenAsync() once
     -> On success: SaveNewAccount() + UpdateAllProfileInformation()
  6. If result OK: OnAuthCompleted() -> SaveNewAccountAndFetchProfile()
  7. If result error: OnAuthError() -> shows error dialog with trace GUID
```

The `RequestRefreshTokenAsync` method has built-in retry logic: if the first attempt to call `AulaRequestAccessTokenAsync()` throws an exception, it tries exactly once more. Both exceptions are logged.

### 3.3 OIDC Client Construction

`AuthenticationManager.CreateOidcClient()` builds the OIDC client:

1. Reads current auth level from `SecureStorageManager.GetAuthenticatorLevelDuringLogin()`
2. Selects scope: level 2 = `"aula"`, default/level 3 = `"aula-sensitive"`
3. Selects client ID: level 2 = `AuthenticationState.ClientIdLevel2`, level 3 = `AuthenticationState.ClientIdLevel3`
4. Configures `OidcClientOptions`:
   - Authority = `Urls.AUTHORISE_URL.Url` (the SimpleSAMLphp authorize endpoint)
   - ClientId = selected client ID
   - Scope = selected scope
   - RedirectUri = `"https://app-private.aula.dk"`
   - Browser = `WebAuthenticatorBrowser` (custom IBrowser implementation)
   - ProviderInformation (manually set, no discovery):
     - IssuerName = `Conf.AuthBackendUrl` (`https://login.aula.dk/`)
     - AuthorizeEndpoint = `Urls.AUTHORISE_URL.Url`
     - TokenEndpoint = `Urls.ACCESS_TOKEN_URL.Url` (the token.php endpoint)
     - KeySet = empty `JsonWebKeySet` (no client-side JWT validation)
   - LoadProfile = false

### 3.4 Token Acquisition

`AuthenticationManager.AulaRequestAccessTokenAsync()`:

1. **Check for persisted account**: Calls `GetPersistedAccountAndCalculateExpirationTime()`
   - Loads `LoginData` from `SecureStorageHelper` using key `"AulaNativeOAuthPrivate"` (the `ServiceName` constant)
   - If found and token is expired, attempts refresh
2. **If token expired**: Calls `OidcClient.RefreshTokenAsync(refreshToken)` to get new tokens
   - Returns immediately with new `LoginData` from `RefreshTokenResult`
3. **If no persisted account or fresh login**: Calls `OidcClient.LoginAsync(new LoginRequest())`
   - This opens a browser (WebAuthenticatorBrowser) showing the SimpleSAMLphp login page
   - User authenticates via UniLogin (level 2) or MitID/NemID (level 3)
   - Browser redirects back to `https://app-private.aula.dk` with auth code
   - OidcClient exchanges auth code for tokens (PKCE flow)
4. **On success**: Creates `LoginData` from `LoginResult`, calls `ResetData(false)`, logs "User successfully logged in"
5. **On failure**: Calls `ResetData()` and returns null. Catches `TimeoutException`, `TaskCanceledException`, generic `Exception`

### 3.5 Browser Authentication

`WebAuthenticatorBrowser` implements `IdentityModel.OidcClient.Browser.IBrowser`:

```csharp
// Simplified from decompiled WebAuthenticatorBrowser.InvokeAsync()
WebAuthenticator.AuthenticateAsync(new WebAuthenticatorOptions {
    Url = new Uri(options.StartUrl),          // authorize URL
    CallbackUrl = new Uri(options.EndUrl),    // redirect URI
    PrefersEphemeralWebBrowserSession = true  // no shared cookies with Safari
});
```

Key details:
- Uses `Microsoft.Maui.Authentication.WebAuthenticator` (the MAUI platform API)
- `PrefersEphemeralWebBrowserSession = true` means each login gets a fresh browser session (no SSO cookie reuse)
- The callback result properties are reconstructed into a raw URL: `redirectUrl#key1=value1&key2=value2`
- On Android, `WebAuthenticationCallbackActivity` handles the HTTPS callback via an intent filter for `https://app-private.aula.dk`

### 3.6 Android Callback Activity

`WebAuthenticationCallbackActivity` is registered with:
- Intent filter: `ACTION_VIEW`, categories `DEFAULT` + `BROWSABLE`
- DataScheme: `https`, DataHost: `app-private.aula.dk`, AutoVerify: `true`

It also handles deep links for email notifications: the `OnCreate` and `OnNewIntent` methods extract the `redirectTo` query parameter and pass it to `EmailNotificationsHandler.HandlePath()`. If not an email notification deep link, it delegates to `Platform.Init()` for the standard OAuth callback.

### 3.7 Post-Login

After successful token acquisition:
```
LoginPageViewModel.OnAuthCompleted(sender, loginData)
  -> AuthenticationState.LogAndResetAuth(success: true)
  -> SaveNewAccountAndFetchProfile(loginData)
    -> AuthenticationManager.SaveNewAccount(account)
      -> SecureStorageHelper.Save(loginData, "AulaNativeOAuthPrivate")
    -> FetchProfileValidateLogin()
      -> ProfileServiceManager.UpdateAllProfileInformation(account)
        -> Fetches user profiles from API
      -> CheckAndActIfWrongButtonOrAppUsed() - validates role matches
      -> If level 3 needed: triggers step-up authentication
    -> SuccessfullLogin(account)
      -> SessionPromptManager.SetLastActive()
      -> If user has OTP role AND is guardian: Controller.StartOTPPage()
      -> Otherwise: Controller.StartOnboarding()
```

### 3.8 Error Handling

On auth failure, `OnAuthError()`:
1. Generates a unique trace GUID (`Guid.NewGuid()`)
2. Shows error dialog with the trace code (for support reference)
3. Logs the error with `AuthenticationState.LogAndResetAuth(success: false, traceGUID)`
4. Sets `OAuthInProgress = false`

The auth logging system (`AuthenticationState.AddAuthLog`) accumulates log entries separated by ` $$ ` and flushes them all at once via `LogAndResetAuth()`, logging as either "Logged in with OAuth2" or "Failed to log in with OAuth2".

## 4. Token Format and Data Model

### LoginData class (`AulaNative.OAuth.LoginData`)

| Field | Type | Description |
|---|---|---|
| `AccessToken` | `string` | The access token (opaque or JWT, no client-side validation) |
| `RefreshToken` | `string` | Refresh token for obtaining new access tokens |
| `AccessTokenExpiration` | `DateTimeOffset` | When the access token expires |
| `ExpiresIn` | `int` | Token lifetime in seconds (from `TokenResponse.ExpiresIn`) |
| `Error` | `string` | Error message if login failed |
| `ErrorDescription` | `string` | Detailed error description |
| `ErrorType` | `ResponseErrorType` | Type of error (from IdentityModel) |
| `Exception` | `Exception` | Exception if login failed |
| `IsExpired` | `bool` (computed) | `AccessTokenExpiration < DateTimeOffset.Now` |

The `LoginData` can be constructed from:
- `IdentityModel.OidcClient.LoginResult` (fresh login)
- `IdentityModel.OidcClient.Results.RefreshTokenResult` (token refresh)
- `Account` (legacy migration from older storage format, reads `expirationTime`, `refresh_token`, `access_token`, `expires_in` from `Account.Properties`)

### Token Characteristics

- **No client-side JWT validation**: The `ProviderInformation.KeySet` is set to an empty `JsonWebKeySet`, and `LoadProfile` is false. The app trusts the token endpoint implicitly.
- **PKCE is used**: The `IdentityModel.OidcClient` library includes `CryptoHelper.Pkce` for code challenge generation (standard authorization code + PKCE flow).
- **Authorization Code flow**: Standard OIDC authorization code grant with PKCE.

## 5. Token Storage

### Storage Mechanism

Tokens are persisted using a layered storage approach:

```
AuthenticationManager.SaveNewAccount(account)
  -> SaveAccount(account) [only if ErrorType == 0]
    -> SecureStorageHelper.Save<LoginData>(loginData, "AulaNativeOAuthPrivate")
      -> ISecureStorageHelper (platform-specific implementation)
        -> DroidSecureStorageHelper (Android)
          -> Microsoft.Maui.Essentials SecureStorage (Android Keystore-backed)
```

The `ServiceName` constant is `"AulaNativeOAuthPrivate"`.

### Stored Keys

| Key | Value | Storage API |
|---|---|---|
| `AulaNativeOAuthPrivate` | Full `LoginData` object (serialized JSON) | `SecureStorageHelper` |
| `PRIVATE_KEYSTORE_PINCODE_KEY` | User's PIN code (plaintext in SecureStorage) | `SecureStorageManager` |
| `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY` | Auth level as JSON int (2 or 3) | `SecureStorageManager` |
| `PRIVATE_BIO_AUTH_ACTIVATED_KEY` | Biometric state as JSON string `"1"`/`"0"` | `SecureStorageManager` |
| `PRIVATE_PORTAL_ROLE_KEY` | PortalRole enum as JSON | `SecureStorageManager` |
| `PRIVATE_SENSITIVITY_LEVEL_KEY` | LoginAuthenticationMethod enum as int string | `SecureStorageManager` |
| `PRIVATE_MIGRATION_COUNT_KEY` | Migration counter as int string | `SecureStorageManager` |
| `PRIVATE_COUNT_USAGE_MESSAGE_MODULE` | Message module usage count | `SecureStorageManager` |
| `PRIVATE_DID_PROMPT_REVIEW` | App Store review prompt flag (`"1"`) | `SecureStorageManager` |

Note: `SecureStorageManager` stores each value by wrapping it in a `Dictionary<string, string>` with the key as both the dictionary key and the storage key. This means each value is double-wrapped: `{ "KEY_NAME": "actual_value" }` serialized as JSON.

### Android Implementation

On Android, `DroidSecureStorageHelper` wraps **Microsoft.Maui.Essentials SecureStorage**, which on Android:
- Uses the **Android Keystore** for encryption key storage
- Stores encrypted values in **SharedPreferences** (EncryptedSharedPreferences on API 23+)
- Values are serialized to JSON via `Newtonsoft.Json.JsonConvert.SerializeObject/DeserializeObject`

### Thread Safety

A `SemaphoreSlim(1, 1)` in `AuthenticationManager` serializes access to token operations (`GetValidAccount`, `DeleteAccount`), preventing concurrent token refreshes.

## 6. Token Refresh Mechanism

### Automatic Refresh via AulaRequestAccessTokenAsync

In `AulaRequestAccessTokenAsync()`:
1. Loads persisted `LoginData` via `GetPersistedAccountAndCalculateExpirationTime()`
2. Checks `LoginData.IsExpired` (compares `AccessTokenExpiration` to `DateTimeOffset.Now`)
3. If expired AND account exists, calls `OidcClient.RefreshTokenAsync(refreshToken)` with the stored refresh token
4. Creates new `LoginData` from `RefreshTokenResult` and returns immediately
5. If no persisted account, falls through to full `LoginAsync()` flow

### GetAccountRequestNewToken (Background Token Refresh)

`GetAccountRequestNewToken()` is called by `GetValidAccount()` for API-initiated token refresh:
1. Gets account from SecureStorage (only if `AccountSaved` flag is true)
2. If account exists and not expired, returns it directly
3. If account expired, calls `RequestRefreshTokenAsync()` which tries `AulaRequestAccessTokenAsync()`
4. If no account and network is available (`Connectivity.NetworkAccess == 4`), triggers `LoginMessagingCenter.NotifyLogoutNeeded()` which forces re-login

### RequestRefreshTokenAsync (Retry Logic)

`RequestRefreshTokenAsync(account, controller)`:
1. If account is not null and not expired, returns it immediately (short-circuit)
2. First attempt: calls `AulaRequestAccessTokenAsync()`
3. If first attempt throws, sets `tryAgain = true` and stores the exception
4. Second attempt: if `tryAgain`, calls `AulaRequestAccessTokenAsync()` again
5. If both fail, logs both exceptions
6. Validates result has `AccessToken`, `RefreshToken`, and non-default `AccessTokenExpiration`
7. On success: saves account, calls `ProfileServiceManager.UpdateAllProfileInformation()`

### RefreshTokenDelegatingHandler

The `IdentityModel.OidcClient` library also provides `RefreshTokenDelegatingHandler`, which is an HTTP message handler that:
- Automatically refreshes tokens when API calls return 401 Unauthorized
- Fires `TokenRefreshedEventArgs` events

### InvalidAccessTokenError Handler

`AulaNative.ServiceManagers.ServiceHandling.ErrorHandlers.InvalidAccessTokenError` handles token errors during API calls:
- Intercepts HTTP responses indicating invalid tokens
- Triggers token refresh via the authentication manager
- Retries the failed request

### BufferOnTokenExpiration

`Conf.BufferOnTokenExpiration` = `TimeSpan.FromMinutes(2)` provides a 2-minute buffer before actual token expiration, ensuring proactive refresh.

## 7. Session Management

### Session Lifetime

| Parameter | Value | Description |
|---|---|---|
| `MINUTES_BEFORE_SESSION_EXPIRES` | 60 | Session timeout in minutes |
| `MINUTES_BEFORE_WARNING_SESSION_WILL_EXPIRE` | 55 | Warning shown at this minute mark |
| `BufferOnTokenExpiration` | 2 minutes | Token refresh buffer |

The user sees: "Du bliver logget ud om: {0} min. Vil du forblive logget ind?" (You will be logged out in: {0} min. Do you want to stay logged in?)

### SessionPromptManager

`AulaNative.Services.Singleton.SessionPromptManager` manages session lifecycle:
- **Countdown timer**: `ExecuteCountingThread()` runs a background timer tracking time since last activity
- **Warning prompt**: Shows dialog 5 minutes before session expires (at 55-minute mark)
- **Keep-alive**: Calls `Urls.KEEP_ALIVE` endpoint (`/api/v{VERSION}/{SESSION}keepAlive`) to extend the session
- **Session state tracking**: `IsSessionActiveAndNotLoggingOut`, `IsSessionStillActiveForPinLogin`
- **Authentication guard**: `IsAuthenticationOrOnboardingInProgress` flag prevents session timeout during login
- **Re-auth on expiry**: `SessionExpiredErrorHandler` forces re-authentication
- **Remote notification handling**: `RemoteNotificationIsHandled()` integrates with notification state

### CSRF Protection

The app implements CSRF protection for API calls:

| Component | Value |
|---|---|
| **Cookie Name** | `Csrfp-Token` |
| **Header Name** | `csrfp-token` |
| **Manager** | `HttpClientManager.SetCsrfpToken()` / `CSRFTokenExists()` |

The CSRF token is read from a cookie named `Csrfp-Token` and sent as the `csrfp-token` HTTP header on subsequent requests.

### Cookie Synchronization

After token operations, cookies are synchronized from the HTTP client to WebView:
```csharp
// In GetValidAccount()
ServiceLocator.Current.GetInstance<ICookieCrossService>()
    .SetWebViewCookies(HttpClientManager.Instance.HttpClientAccessor.Cookies
        .GetCookies(new Uri(Conf.BackendUrl)));
```

### LoginSetting (SQLite)

Login settings are persisted in a local SQLite database via `SQLiteLoginSettingManager`:
- `LoginSetting` model in `AulaNative.SQLiteData.Models`
- Managed by `SQLiteDeviceSettingManager.InsertOrUpdateLoginSetting()`
- Database name: `auladb_13_05_2019` (from `Conf.DBName`)

## 8. Biometric Authentication (PIN & Fingerprint)

### PIN Login

- After initial OIDC login, the user can set a PIN code
- PIN is stored in SecureStorage under `PRIVATE_KEYSTORE_PINCODE_KEY` (plaintext within SecureStorage encryption)
- `LoginWithPinActivity` / `PinCodeViewModel` handles PIN entry
- PIN login uses persisted tokens (no OIDC round-trip), just validates the PIN locally
- On `InitLogin()`, if a persisted account AND PIN exist, the user is sent directly to `LoginWithPinActivity`

### Biometric (Fingerprint)

- Uses `Plugin.Fingerprint` library (via `BiometricPrompt`)
- `BiometricUtils.AuthenticateBiometric()`, `IsBiometricAuthPossible()`, `GetBiometricAuthenticationType()`
- State stored in `PRIVATE_BIO_AUTH_ACTIVATED_KEY` (as JSON string `"1"` or `"0"`)
- If biometric succeeds, uses persisted tokens (same as PIN login)
- Falls back to PIN or full OIDC login on failure

### Login Authentication Methods

`LoginAuthenticationMethod` enum values:

| Value | Name | Meaning |
|---|---|---|
| 0 | `Unknown` | Default/unset |
| 2 | `Level2` | Standard UniLogin |
| 3 | `Level3NemId` | MitID/NemID elevated |
| 9 | `Level3Employees` | Staff elevated login |

### OTP Selection

`OTPSelectionActivity` / `OTPSelectionViewModel` supports one-time password selection during login. If a user has the `Otp` portal role AND is a guardian, `SuccessfullLogin()` routes to `Controller.StartOTPPage()` instead of the standard onboarding flow.

### Portal Roles

The login screen button selection maps to `PortalRole` enum values. After login, `CheckAndActIfWrongButtonOrAppUsed()` validates the user's actual profile matches the selected role. Mismatches produce specific error messages:
- Employee logging into private (guardian/child) app
- Guardian pressing child button (or vice versa)
- Child users are restricted from guardian mode
- OTP-only profiles get a specific message

## 9. Logout Flow

### Standard Logout

```
AuthenticationManager.ResetData(isiOSLogout: false)
  -> On Android: OpenLogoutAndReturnToAppWithUniversalLink(Urls.LOGOUT_URL.Url)
    -> Constructs end-session URL with parameters:
       - "code" = "logout"
       - "returnUri" = post-logout redirect (https://app-private.aula.dk)
    -> Opens URL via IUniversalLinkOpener
    -> Sets IsAuthenticationOrOnboardingInProgress = true
```

The logout URL is `{AuthBackendUrl}/auth/logout.php?code=logout&returnUri={encodedReturnUrl}`, which terminates the SSO session on the SimpleSAMLphp side and redirects back to the app.

### Cleanup on Logout

`SecureStorageManager.DeleteLoginSettings()` removes:
- `PRIVATE_DID_PROMPT_REVIEW`
- `PRIVATE_KEYSTORE_PINCODE_KEY`
- `PRIVATE_PORTAL_ROLE_KEY`
- `PRIVATE_BIO_AUTH_ACTIVATED_KEY`
- `PRIVATE_SENSITIVITY_LEVEL_KEY`
- `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY`
- Resets `PRIVATE_COUNT_USAGE_MESSAGE_MODULE` to 0

### Wipe All Sessions

A "wipe all sessions" feature uses `Urls.DELETE_ALL_DEVICES` (`/api/v{VERSION}/{NOTIFICATIONS}unregisterAllDevices`) to log out from all registered devices.

## 10. API Authentication

Once authenticated, API calls use:

1. **Cookies**: The browser-based login sets session cookies which are forwarded via `HttpClientManager.HttpClientAccessor.Cookies`
2. **CSRF Token**: `Csrfp-Token` cookie value sent as `csrfp-token` header
3. **Token-based requests**: `GetValidAccount()` obtains a valid token (refreshing if needed) before each API call
4. **Bearer token**: The access token is sent as a Bearer token in the Authorization header for API calls
5. **Basic Auth** (non-production only): Environments with `IsProduction: false` add `Authorization: Basic` header with `aula-user:Aula-1337`

### API Base URL Pattern

```
https://www.aula.dk/api/v{API_VERSION}/
```

Where `API_VERSION` = 23 (from `Conf.API_VERSION`).

## 11. Cloud Storage OAuth (Google Drive / OneDrive)

### Architecture

Cloud storage uses a completely separate OAuth flow from the main Aula login:

```
CloudStorageAuthenticatorManager(CloudStorageService provider)
  -> GenerateAuthenticatorInfo(platform) -> gets provider-specific config
  -> CreateOidcClient(infoModel) -> builds OidcClient with provider endpoints
  -> Authenticate() -> OidcClient.LoginAsync()
  -> SaveNewAccount(loginData) -> saves to AuthenticationState.AccountGoogleDrive/AccountOneDrive
```

### Google Drive Configuration

| Parameter | Value |
|---|---|
| **Auth URL** | `https://accounts.google.com/o/oauth2/auth` |
| **Token URL** | `https://oauth2.googleapis.com/token` |
| **Issuer** | `accounts.google.com` |
| **Scope** | `https://www.googleapis.com/auth/drive.readonly` |
| **Android Private Client ID** | `811573413698-9bnd25vulk5rt0pfr48hp65rej88a049.apps.googleusercontent.com` |
| **Android Debug Client ID** | `811573413698-fgk5v7ki9deta3fsr5sama3ervva1o7g.apps.googleusercontent.com` |
| **iOS Private Client ID** | `811573413698-h94dnjr2gmaa9pf73fvmdir3732ppg6d.apps.googleusercontent.com` |
| **Android Private Redirect** | `com.netcompany.aulanativeprivate:/googleoauth2redirect` |
| **iOS Private Redirect** | `com.netcompany.aula-native:/googleoauth2redirect` |

Staff app uses a different Google project (client IDs starting with `839639645203-`).

### OneDrive Configuration

| Parameter | Value |
|---|---|
| **Auth URL** | `https://login.microsoftonline.com/common/oauth2/v2.0/authorize` |
| **Token URL** | `https://login.microsoftonline.com/common/oauth2/v2.0/token` |
| **Issuer** | `https://login.microsoftonline.com/common/v2.0` |
| **Scope** | `https://graph.microsoft.com/files.Read https://graph.microsoft.com/files.Read.all https://graph.microsoft.com/Sites.Read.all` |
| **Client ID** (both private and staff) | `47984900-bb20-4659-9f0d-700f5ab91571` |
| **Android Private Redirect** | `com.netcompany.aulanativeprivate://onedrive2redirect` |
| **iOS Private Redirect** | `com.netcompany.aula-native://onedrive2redirect` |

Note: OneDrive uses the same client ID (`47984900-bb20-4659-9f0d-700f5ab91571`) for both private and staff apps. The `secret` field in `CloudConf` is an empty string (public client).

### CloudStorageAuthInterceptor (Android)

The `CloudStorageAuthInterceptor` activity handles OAuth callbacks for cloud storage on Android:

```java
@IntentFilter(action = "android.intent.action.VIEW",
    categories = {"DEFAULT", "BROWSABLE"},
    DataScheme = "com.netcompany.aulanativeprivate",
    DataPaths = {"//onedrive2redirect", "/googleoauth2redirect"},
    AutoVerify = true)
```

It extends `WebAuthenticatorCallbackActivity` and handles both OneDrive (`//onedrive2redirect`) and Google Drive (`/googleoauth2redirect`) callbacks via the app's custom URI scheme.

### Token Caching

Cloud storage tokens are cached in-memory in `AuthenticationState`:
- `AuthenticationState.AccountGoogleDrive` / `AuthenticationState.AccountOneDrive`
- `AuthenticationState.ExpirationDate` = `DateTime.Now + 2 minutes`
- `GetAccountRequestNewToken()` returns cached token if within the 2-minute window
- Cloud storage tokens are NOT persisted to SecureStorage (only held in memory)

### Service Names

The `CloudStorageAuthenticatorManager` sets service names for internal tracking:
- Google Drive: `"GoogleDriveOAuth"`
- OneDrive: `"OneDriveOAuth"`

## 12. Certificate Pinning

The app implements certificate pinning via `Conf.GetCertPublicKeys()`:

| Domain Pattern | Public Key Pins (SHA-256 SPKI) |
|---|---|
| `*.ncaula.com` (non-prod) | `ejsQt33CcKZWEoO/ym2mcdSynXrVfK1o6QbTI868tDE=`, `PfUUWB6dvdMA9exWlx0W+6lKT540ElcRWUERcBRtP6o=`, `CC09RfvRZQ1z+bj1VeJ/jrYOeH3D0epyQR+FEXLddF8=` |
| `*.aula.dk` (production) | `/P3+fgXhRH6jPoKBMmAKWRrtjDoEZf4ySjxLoQuqsYc=`, `eLCo7AWQ2P88/2FQfow993oOhcjXal2sS/e2mZgJLJE=`, `9XtneGQWNOLQFi0f8LEJ62bt1f/pVrCb4ytT66RcurA=` |

## 13. Summary for Rust Library Implementation

To implement the auth flow in Rust:

1. **OIDC Authorization Code + PKCE flow** against SimpleSAMLphp endpoints
2. **Authority**: `https://login.aula.dk/`
3. **Authorize**: `https://login.aula.dk/simplesaml/module.php/oidc/authorize.php`
4. **Token**: `https://login.aula.dk/simplesaml/module.php/oidc/token.php`
5. **Client ID (level 2)**: `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6` (scope: `aula`)
6. **Client ID (level 3)**: `_99949a54b8b65423862aac1bf629599ed64231607a` (scope: `aula-sensitive`)
7. **Redirect URI**: `https://app-private.aula.dk`
8. **No JWT validation needed** on client side (server handles this)
9. **Store**: access_token, refresh_token, expiration
10. **Refresh**: POST to token endpoint with refresh_token grant
11. **CSRF**: Extract `Csrfp-Token` from response cookies, send as `csrfp-token` header
12. **API calls**: `https://www.aula.dk/api/v23/...` with cookies + CSRF header
13. **Keep-alive**: Hit `/api/v23/{SESSION}keepAlive` periodically to maintain session
14. **Logout**: GET `https://login.aula.dk/auth/logout.php?code=logout&returnUri={encoded_redirect}`
15. **Session timeout**: 60 minutes from last activity, with 2-minute token refresh buffer
16. **Retry logic**: Retry auth once on first failure

## 14. Step-Up Authentication: Triggers, Affected Operations, and Flow

The step-up mechanism elevates a user from Level 2 (standard UniLogin, scope `aula`) to Level 3 (MitID/NemID, scope `aula-sensitive`). This section documents exactly when step-up is triggered, what operations require it, and how the server signals the requirement.

### 14.1 When Step-Up Is Triggered at Login

Step-up is determined during the `FetchProfileValidateLogin` flow (in `LoginPageViewModel`) and `CheckForLoginConflicts` (in `PinCodeViewModel`). The level is derived from the server-returned profile: `ProfileManager.Instance.Profile.IsSteppedUp` maps to level 3 if true, level 2 if false.

The triggers differ by user role:

**Guardians and Employees (mandatory step-up)**:
- If a guardian or employee somehow logged in at Level 2, they are **forced** to step up. The app calls `WarningStepupIsNeeded()` which presents a non-optional dialog. The log message is explicit: "The user might experience a login loop because we ask them to stepup instantly."
- This means guardians and employees effectively always need Level 3 / MitID authentication.

**Children older than 15 at Level 2 (conditional step-up)**:
- If the child has pending onboarding (unanswered consents or additional masterdata), `WarningStepupIsNeeded()` is called -- mandatory step-up.
- If no pending onboarding, `WarningSuggestStepUp()` is called -- voluntary/suggested step-up. The user can decline and continue at Level 2.

**Children younger than 15 at Level 2**:
- Level 2 is accepted. No step-up prompt.

**Any user at Level 3**:
- No step-up needed. The `CheckForLoginConflicts` method returns `ShowConfirmBox = false` immediately.

### 14.2 Step-Up Dialog Types

The `IAuthUIController` interface defines two distinct step-up methods:

| Method | Behavior | When Used |
|---|---|---|
| `WarningStepupIsNeeded(title, message, ok, cancel)` | Mandatory. User must re-authenticate at Level 3 or log out. On OK, calls `StepUp()`. | Guardians/employees at L2; children with pending onboarding |
| `WarningSuggestStepUp(title, message, ok, cancel)` | Optional. User can decline and continue at Level 2. On OK, calls `StepUp()`. | Children >=15 without pending onboarding |

### 14.3 The StepUp() Method

When the user accepts step-up (from either dialog), `LoginActivity.StepUp()` is called:

1. Logs "StepUp: PortalRole {role}"
2. Calls `AuthenticationManager.ResetData()` (clears current session, logs out)
3. The user is then taken back to the login flow, but this time the auth level is set to 3
4. The OIDC client is constructed with the Level 3 client ID and `aula-sensitive` scope
5. The SimpleSAMLphp server presents MitID/NemID authentication (instead of UniLogin)

### 14.4 Operations That Require Step-Up (Level 3 / IsSteppedUp)

The `Profile.IsSteppedUp` boolean property controls access to the following features. This value comes from the backend API in the user profile response.

#### Secure Documents ("Sikre Filer")

All secure document operations gate on `IsSteppedUp` via `PermissionManager.HasSecureFilePermission()`:

```
HasSecureFilePermission(PermissionEnum permission):
    if profile != null AND profile.IsSteppedUp:
        return PermissionUtils.HasPermissionsOnAnyInstitution(permission)
    return false
```

Affected permissions (all require IsSteppedUp = true AND the permission on the institution):
- `ACCESS_SECURE_FILESHARING` (26) -- `CanAccessSecureFilesAndSharing()`: full access to secure file sharing
- `READ_SECURE_FILES` (116) -- `CanViewSecureFiles()`: read-only access to secure files

Additionally, `CanHandleSecureFiles()` has its own step-up check:
```
CanHandleSecureFiles(institutionCode):
    flag = NOT IsChild() OR IsSteppedUp
    return HasPermission(HANDLE_SECURE_FILES) AND flag
```

When a user navigates to the Document Overview and `IsStepUpRequired` is true (i.e., `!CanAccessSecureFilesAndSharing()`), the `DocumentOverviewPageViewModel.InformAboutStepUpIsNeeded()` method shows a dialog. The dialog message differs by age:
- Users >= 15: "DOCUMENTS_SECURE_FILES_STEP_UP_IS_NEEDED_DESCRIPTION" (suggests re-login with MitID)
- Users < 15: "DOCUMENTS_SECURE_FILES_UNDER_15_CAN_NOT_ACCESS_DESCRIPTION" (cannot access at all)

Related permissions that also require step-up context:
- `SHARE_SECURE_FILES` (48) -- sharing secure files
- `EXPORT_SECURE_FILES` (115) -- exporting secure files
- `HANDLE_SECURE_FILES` (27) -- editing/managing secure files
- `HANDLE_SECURE_FILES_LIMITED` (28) -- limited handling
- `SECURE_DOCUMENTS_ACCESS_ALL` (101) -- deleting secure documents

#### Sensitive Messages

Message threads have a `SensitivityLevel` enum and a `RequiredStepUp` boolean:

```csharp
enum SensitivityLevel { Level1 = 1, Level2 = 2, Level3 = 3 }
```

- **Marking messages as sensitive**: `MessageFormViewModel.AllowToChooseSensitive` returns `ProfileManager.Instance.Profile.IsSteppedUp`. Only stepped-up users can mark a message thread as sensitive.
- **Setting sensitivity level API**: `SetSensitivityLevelRequest(threadId, isSensitive, bundleId)` sets `SensitivityLevel = isSensitive ? 3 : 2`. The API endpoint is `{BackendUrlApi}messaging/setSensitivityLevel`.
- **Viewing sensitive threads**: `ThreadSubscriptionInfoViewModel.SurpassedSensitiveLevel` returns `true` if the thread is not sensitive, or if it is sensitive AND the user is stepped up. Non-stepped-up users cannot view sensitive thread content.
- **Step-up dialog in messaging**: `ThreadSubscriptionInfoViewModel.ShowStepUpDialog()` is called when users try to interact with sensitive messages without being stepped up. This is triggered from both `MessagesOverviewFragment` (thread list) and `MessageServiceManager` (when opening/replying to sensitive threads).

The `MessageThread` model from the API contains:
- `RequiredStepUp: bool` -- whether this thread requires elevated auth
- `SensitivityLevel: SensitivityLevel` -- the sensitivity classification of the thread

#### Aula Document Links

`AulaDocumentLinkRenderingHandler` checks `IsSteppedUp` before rendering links to secure Aula documents. If the user is not stepped up, the link handler returns `LinkHandlerResponsibleType.Ignore`, hiding the document link entirely.

#### Consents

`ConsentViewModel` gates consent editing on `IsSteppedUp`. The method that checks whether consent modification is allowed returns `ProfileManager.Instance.Profile.IsSteppedUp`.

### 14.5 Server-Side Step-Up Enforcement

The backend also enforces step-up requirements via HTTP responses:

**Sub-code signaling**: `WebResponseStatusSubCodeConstants.AUTHORIZATION_STEP_UP_REQUIRED = 8`. When the server returns HTTP 401 with this sub-code, it means the operation requires elevated authentication.

**Error handler**: `StepUpNeededErrorHandler` catches HTTP 401 responses and shows a localized error:
- Title: `MOBILE_STEP_UP_ERROR_TITLE`
- Message: `MOBILE_STEP_UP_ERROR_MESSAGE`
- Does NOT log to backend (`ShouldLogToBackend() = false`)

This error handler is registered in `ErrorHandlerManagerFactory.FullServiceManager` alongside other error handlers (session expired, unauthorized, maintenance, etc.). It is NOT present in the `SilentFailServiceHandlerManager`, meaning step-up errors are always shown to the user.

### 14.6 Permission Model and Step-Up

The `Permission` class from the API includes a `StepUp: bool` field, indicating whether a specific permission requires step-up authentication at the institution level. This means the backend can configure per-permission step-up requirements on a per-institution basis, not just globally.

### 14.7 Step-Up Flow Sequence Diagram

```
User logged in at Level 2 (UniLogin, scope "aula")
    |
    v
Attempts operation requiring Level 3
    |
    +-- Client-side check: IsSteppedUp == false
    |   |
    |   v
    |   Shows step-up dialog (varies by context):
    |     - Secure docs: SecureDocumentAccessUtils.ShowStepUpDialogIfRequired()
    |     - Messages: ThreadSubscriptionInfoViewModel.ShowStepUpDialog()
    |     - Login: WarningSuggestStepUp() or WarningStepupIsNeeded()
    |
    +-- Server-side check: returns HTTP 401 + sub-code 8
        |
        v
        StepUpNeededErrorHandler shows error dialog

If user accepts step-up:
    |
    v
LoginActivity.StepUp()
    -> AuthenticationManager.ResetData() (logout)
    -> Re-enter login flow with authenticator level = 3
    -> CreateOidcClient() uses:
         ClientId = _99949a54b8b65423862aac1bf629599ed64231607a
         Scope = "aula-sensitive"
    -> SimpleSAMLphp presents MitID/NemID instead of UniLogin
    -> On success: Profile.IsSteppedUp = true (from backend)
    -> SaveSensitivityLevel(3) -> persists as LoginAuthenticationMethod.Level3NemId
```

### 14.8 Summary Table: What Requires Step-Up

| Feature | Check Method | Behavior Without Step-Up |
|---|---|---|
| View secure documents | `CanAccessSecureFilesAndSharing()` | Hidden from document list; dialog shown |
| Read secure files | `CanViewSecureFiles()` | Not accessible |
| Handle/edit secure files | `CanHandleSecureFiles()` | Blocked (children only; adults always stepped up) |
| Share/export secure files | `CanShareSecureFiles()`, `CanExportSecureFiles()` | Permission denied |
| Mark message as sensitive | `AllowToChooseSensitive` | Option not available |
| View sensitive message content | `SurpassedSensitiveLevel` | Content hidden; step-up dialog shown |
| Open/reply to sensitive thread | `MessageServiceManager` calls | Step-up dialog shown |
| Render Aula document links | `AulaDocumentLinkRenderingHandler` | Link ignored/hidden |
| Edit consents | `ConsentViewModel` | Not allowed |
| Guardian/employee login | `FetchProfileValidateLogin` | Forced step-up at login |
| Child >=15 with pending onboarding | `FetchProfileValidateLogin` | Forced step-up at login |

## Appendix A: Source File Index

| File | Namespace | Purpose |
|---|---|---|
| `AuthenticationManager.cs` | `AulaNative.OAuth` | Main auth orchestration |
| `LoginData.cs` | `AulaNative.OAuth` | Token data model |
| `AuthenticationState.cs` | `AulaNative.OAuth` | Static auth state + cloud storage token cache |
| `OAuth2Authenticator.cs` | `AulaNative.OAuth` | Generic OAuth2 authenticator (for cloud storage) |
| `LoginPageViewModel.cs` | `AulaNative.ViewModels.Login` | Login UI logic |
| `WebAuthenticatorBrowser.cs` | `AulaNative.ViewModels.Login` | MAUI WebAuthenticator IBrowser bridge |
| `Conf.cs` | `AulaNative.Configuration` | Client IDs, scopes, timeouts |
| `CloudConf.cs` | `AulaNative.Configuration` | Cloud storage OAuth configuration |
| `Urls.cs` | `AulaNative.Configuration` | All endpoint URLs as AulaUrl objects |
| `SecureStorageManager.cs` | `AulaNative.ServiceManagers` | Secure key-value storage API |
| `CloudStorageAuthenticatorManager.cs` | `AulaNative.OAuth.OAuthCloudStorage` | Cloud storage OAuth flow |
| `CloudStorageAuthenticatorInfoModel.cs` | `AulaNative.OAuth.OAuthCloudStorage` | Cloud storage config model |
| `CloudStorageAuthInterceptor.cs` | `AulaNative.Droid.OAuth` | Android cloud storage callback |
| `WebAuthenticationCallbackActivity.cs` | `AulaNative.Droid.Activities.Login` | Android OIDC callback |
| `LoginActivity.cs` | `AulaNative.Droid.Activities.Login` | Android login screen |
| `LoginWithPinActivity.cs` | `AulaNative.Droid.Activities.Login` | Android PIN entry |
| `SessionPromptManager.cs` | `AulaNative.Services.Singleton` | Session timeout management |
| `LoginAuthenticationMethod.cs` | `AulaNative.Enums` | Auth level enum |
| `SensitivityLevel.cs` | `AulaNative.Enums` | Message sensitivity levels (1-3) |
| `Permission.cs` | `AulaNative.Models.Institutions` | Permission model with StepUp flag |
| `PermissionEnum.cs` | `AulaNative.Models.Institutions` | All permission constants |
| `PermissionManager.cs` | `AulaNative.Services.Singleton` | Permission checks including step-up gates |
| `Profile.cs` | `AulaNative.Models.ProfileModels` | User profile with IsSteppedUp property |
| `SecureDocumentAccessUtils.cs` | `AulaNative.Utils` | Step-up dialog for secure documents |
| `StepUpNeededErrorHandler.cs` | `AulaNative.ServiceManagers.ServiceHandling.ErrorHandlers` | HTTP 401 step-up error handler |
| `ErrorHandlerManagerFactory.cs` | `AulaNative.ServiceManagers.ServiceHandling` | Error handler registration |
| `WebResponseStatusSubCodeConstants.cs` | `AulaNative.Models.Web` | HTTP sub-codes including AUTHORIZATION_STEP_UP_REQUIRED (8) |
| `MessageThread.cs` | `AulaNative.Models.MessageThreads` | Message thread with RequiredStepUp and SensitivityLevel |
| `SetSensitivityLevelRequest.cs` | `AulaNative.Models.MessageThreads.Argument` | Set sensitivity level API request |
| `ThreadSubscriptionInfoViewModel.cs` | `AulaNative.ViewModels.MessageThreads` | Thread view model with step-up dialog |
| `MessageFormViewModel.cs` | `AulaNative.ViewModels.Message` | Message form with AllowToChooseSensitive |
| `DocumentOverviewPageViewModel.cs` | `AulaNative.ViewModels.Document.Overview` | Document overview with step-up check |
| `ConsentViewModel.cs` | `AulaNative.ViewModels.Consent` | Consent editing step-up check |
| `PinCodeViewModel.cs` | `AulaNative.ViewModels` | PIN login with CheckForLoginConflicts step-up logic |
| `IAuthUIController.cs` | `AulaNative.OAuth` | Auth UI interface with step-up dialog methods |
| `GoogleDriveViewModel.cs` | `AulaNative.ViewModels.CloudStorageIntegration.GoogleDrive` | Google Drive auth info provider |
| `OneDriveViewModel.cs` | `AulaNative.ViewModels.CloudStorageIntegration.OneDrive` | OneDrive auth info provider |
