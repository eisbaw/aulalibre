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

### Environment Configuration

The app supports multiple environments, selected via `AulaNative.Configuration.EnvironmentFactory`:

| Environment | Backend Host | Auth Host |
|---|---|---|
| **Production** | `www.aula.dk` | `login.aula.dk` |
| **Pre-prod** | `www1-preprod.aula.dk` | `login-preprod.aula.dk` |
| **Hotfix** | `www1-hotfix.aula.dk` | `login-hotfix.aula.dk` |
| **Test1** | `www1-test1.ncaula.com` | `www1-test1.ncaula.com` |
| **Test3** | `www1-test3.ncaula.com` | `www1-test3.ncaula.com` |
| **Dev1** | `www1-dev1.ncaula.com` | `www1-dev1.ncaula.com` |
| **Dev3** | `www1-dev3.ncaula.com` | `www1-dev3.ncaula.com` |
| **Dev11** | `www1-dev11.ncaula.com` | `www1-dev11.ncaula.com` |

Non-production environments use basic auth credentials: `aula-user:Aula-1337`.

## 2. Step-Up Authentication (Levels 2 and 3)

The app supports two authentication levels for accessing different sensitivity tiers:

| Level | Client ID | Scope | Purpose |
|---|---|---|---|
| **Level 2** | `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6` | `aula` | Standard login (UniLogin) |
| **Level 3** | `_99949a54b8b65423862aac1bf629599ed64231607a` | `aula-sensitive` | Elevated access (MitID/NemID) for secure documents |

The level is stored in `SecureStorageManager` under key `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY`. Step-up is required for operations like viewing/editing secure documents ("sikre filer") and modifying consents.

The authentication level is determined by `SecureStorageManager.GetAuthenticatorLevelDuringLogin()` and selected dynamically when `CreateOidcClient()` is called.

## 3. Login Flow (Step by Step)

### 3.1 UI Entry Point

```
LoginActivity (Android)
  -> LoginPageViewModel.ClickedLoginButton()
    -> LoginPageViewModel.InitLogin()
      -> AuthenticationManager.AulaRequestAccessTokenAsync()
```

### 3.2 OIDC Client Construction

`AuthenticationManager.CreateOidcClient()` builds the OIDC client:

1. Reads current auth level from `SecureStorageManager.GetAuthenticatorLevelDuringLogin()`
2. Selects scope: level 2 = `"aula"`, default/level 3 = `"aula-sensitive"`
3. Selects client ID: level 2 = `"_742adb5e..."`, level 3 = `"_99949a54..."`
4. Configures `OidcClientOptions`:
   - Authority = `AUTHORISE_URL` (the SimpleSAMLphp authorize endpoint)
   - ClientId = selected client ID
   - Scope = selected scope
   - RedirectUri = `"https://app-private.aula.dk"`
   - Browser = `WebAuthenticatorBrowser` (custom IBrowser implementation)
   - ProviderInformation (manually set, no discovery):
     - IssuerName = `Conf.AuthBackendUrl` (`https://login.aula.dk/`)
     - AuthorizeEndpoint = `AUTHORISE_URL`
     - TokenEndpoint = `ACCESS_TOKEN_URL` (the token.php endpoint)
     - KeySet = empty `JsonWebKeySet` (no client-side JWT validation)
   - LoadProfile = false

### 3.3 Token Acquisition

`AuthenticationManager.AulaRequestAccessTokenAsync()`:

1. **Check for persisted account**: Calls `GetPersistedAccountAndCalculateExpirationTime()`
   - Loads `LoginData` from `SecureStorageHelper` using `ServiceName` key
   - If found and token is expired, attempts refresh
2. **If token expired**: Calls `OidcClient.RefreshTokenAsync(refreshToken)` to get new tokens
3. **If no persisted account or fresh login**: Calls `OidcClient.LoginAsync(new LoginRequest())`
   - This opens a browser (WebAuthenticatorBrowser) showing the SimpleSAMLphp login page
   - User authenticates via UniLogin (level 2) or MitID/NemID (level 3)
   - Browser redirects back to `https://app-private.aula.dk` with auth code
   - OidcClient exchanges auth code for tokens (PKCE flow)
4. **On success**: Creates `LoginData` from `LoginResult`, calls `ResetData(false)`, logs "User successfully logged in"
5. **Error handling**: Catches `TimeoutException`, `TaskCanceledException`, generic `Exception`

### 3.4 Browser Authentication

`WebAuthenticatorBrowser` implements `IdentityModel.OidcClient.Browser.IBrowser`:
- Opens the authorize URL in a system browser or custom WebView (`LoginWebViewActivity`)
- Intercepts the redirect to `https://app-private.aula.dk` or `com.netcompany.aulanativeprivate://` scheme
- Returns the authorization code to the OIDC client
- `WebAuthenticationCallbackActivity` handles the callback on Android

### 3.5 Post-Login

After successful token acquisition:
```
LoginPageViewModel.SaveNewAccountAndFetchProfile()
  -> Persists the LoginData (tokens) to SecureStorage
  -> LoginPageViewModel.FetchProfileValidateLogin()
    -> ProfileServiceManager.GetProfilesByLogin()
      -> Fetches user profiles from API
  -> LoginPageViewModel.UserCanLogin() - validates the user can proceed
```

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

The `LoginData` can be constructed from either:
- `IdentityModel.OidcClient.LoginResult` (fresh login)
- `IdentityModel.OidcClient.Results.RefreshTokenResult` (token refresh)

### Token Characteristics

- **No client-side JWT validation**: The `ProviderInformation.KeySet` is set to an empty `JsonWebKeySet`, and `LoadProfile` is false. The app trusts the token endpoint implicitly.
- **PKCE is used**: The `IdentityModel.OidcClient` library includes `CryptoHelper.Pkce` for code challenge generation (standard authorization code + PKCE flow).
- **Authorization Code flow**: Standard OIDC authorization code grant with PKCE.

## 5. Token Storage

### Storage Mechanism

Tokens are persisted using a layered storage approach:

```
SecureStorageManager (high-level key-value API)
  -> SecureStorageHelper.Save<T>(value, key) / FindValueForKey<T>(key)
    -> ISecureStorageHelper (platform-specific implementation)
      -> DroidSecureStorageHelper (Android)
        -> Microsoft.Maui.Essentials SecureStorage (Android Keystore-backed)
```

### Stored Keys

| Key | Description | Storage Class |
|---|---|---|
| `ServiceName` (field on `AuthenticationManager`) | Full `LoginData` object (serialized JSON via Newtonsoft.Json) | `SecureStorageHelper` |
| `PRIVATE_KEYSTORE_PINCODE_KEY` | User's PIN code (for PIN login) | `SecureStorageManager` |
| `PRIVATE_PORTAL_AUTHENTICATOR_LEVEL_KEY` | Auth level (2 or 3) during login | `SecureStorageManager` |
| `PRIVATE_BIO_AUTH_ACTIVATED_KEY` | Whether biometric auth is enabled ("1"/"0") | `SecureStorageManager` |
| `PRIVATE_PORTAL_ROLE_KEY` | User's portal role | `SecureStorageManager` |
| `PRIVATE_SENSITIVITY_LEVEL_KEY` | Persisted auth type (LoginAuthenticationMethod enum) | `SecureStorageManager` |

### Android Implementation

On Android, `DroidSecureStorageHelper` wraps **Microsoft.Maui.Essentials SecureStorage**, which on Android:
- Uses the **Android Keystore** for encryption key storage
- Stores encrypted values in **SharedPreferences** (EncryptedSharedPreferences on API 23+)
- Values are serialized to JSON via `Newtonsoft.Json.JsonConvert.SerializeObject/DeserializeObject`

## 6. Token Refresh Mechanism

### Automatic Refresh

In `AulaRequestAccessTokenAsync()`:
1. Loads persisted `LoginData`
2. Checks `LoginData.IsExpired` (compares `AccessTokenExpiration` to current time)
3. If expired, calls `OidcClient.RefreshTokenAsync(refreshToken)` with the stored refresh token
4. Creates new `LoginData` from `RefreshTokenResult`
5. The new tokens (including possibly a new refresh token) are persisted

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

`Conf.BufferOnTokenExpiration` is a `TimeSpan` field that provides a time buffer before actual token expiration, ensuring proactive refresh.

## 7. Session Management

### Session Lifetime

| Parameter | Description |
|---|---|
| `MINUTES_BEFORE_SESSION_EXPIRES` | Session timeout (minutes), stored in `Conf` |
| `MINUTES_BEFORE_WARNING_SESSION_WILL_EXPIRE` | Warning shown before expiry |

The user sees: "Du bliver logget ud om: {0} min. Vil du forblive logget ind?" (You will be logged out in: {0} min. Do you want to stay logged in?)

### SessionPromptManager

`AulaNative.Services.Singleton.SessionPromptManager` manages session lifecycle:
- **Countdown timer**: `ExecuteCountingThread()` runs a background timer
- **Warning prompt**: Shows dialog before session expires
- **Keep-alive**: `KEEP_ALIVE` URL endpoint to extend the session
- **Session state tracking**: `IsSessionActiveAndNotLoggingOut`, `IsSessionStillActiveForPinLogin`
- **Re-auth on expiry**: `SessionExpiredErrorHandler` forces re-authentication

### CSRF Protection

The app implements CSRF protection for API calls:

| Component | Value |
|---|---|
| **Cookie Name** | `Csrfp-Token` |
| **Header Name** | `csrfp-token` |
| **Manager** | `HttpClientManager.SetCsrfpToken()` / `CSRFTokenExists()` |

The CSRF token is read from a cookie named `Csrfp-Token` and sent as the `csrfp-token` HTTP header on subsequent requests.

### LoginSetting (SQLite)

Login settings are persisted in a local SQLite database via `SQLiteLoginSettingManager`:
- `LoginSetting` model in `AulaNative.SQLiteData.Models`
- Managed by `SQLiteDeviceSettingManager.InsertOrUpdateLoginSetting()`

## 8. Biometric Authentication (PIN & Fingerprint)

### PIN Login

- After initial OIDC login, the user can set a PIN code
- PIN is stored in SecureStorage under `PRIVATE_KEYSTORE_PINCODE_KEY`
- `LoginWithPinActivity` / `PinCodeViewModel` handles PIN entry
- PIN login uses persisted tokens (no OIDC round-trip), just validates the PIN locally

### Biometric (Fingerprint)

- Uses `Plugin.Fingerprint` library (via `BiometricPrompt`)
- `BiometricUtils.AuthenticateBiometric()`, `IsBiometricAuthPossible()`, `GetBiometricAuthenticationType()`
- State stored in `PRIVATE_BIO_AUTH_ACTIVATED_KEY`
- If biometric succeeds, uses persisted tokens (same as PIN login)
- Falls back to PIN or full OIDC login on failure

### Login Authentication Methods

`LoginAuthenticationMethod` enum defines available methods:
- Standard OIDC (UniLogin / MitID)
- PIN code
- Biometric (fingerprint/face)

### OTP Selection

`OTPSelectionActivity` / `OTPSelectionViewModel` supports one-time password selection during login (likely for MitID/NemID two-factor).

## 9. Logout Flow

```
AuthenticationManager.OpenLogoutAndReturnToAppWithUniversalLink(baseLogoutUrl)
  -> Opens LOGOUT_URL: https://login.aula.dk/auth/logout.php
  -> AuthenticationManager.ResetData()
    -> Clears persisted LoginData
    -> Clears cookies (via ICookieCrossService)
  -> SecureStorageManager.DeleteLoginSettings()
    -> Removes all auth-related secure storage keys
  -> ProfileUtils.Logout() (Android-specific cleanup)
```

The logout URL is `{AuthBackendUrl}/auth/logout.php`, which terminates the SSO session on the SimpleSAMLphp side.

A "wipe all sessions" feature (`DeleteAllSessions` / `DELETE_ALL_DEVICES`) allows the user to log out from all registered devices.

## 10. API Authentication

Once authenticated, API calls use:

1. **Cookies**: The browser-based login sets session cookies which are forwarded
2. **CSRF Token**: `Csrfp-Token` cookie value sent as `csrfp-token` header
3. **Token-based requests**: Some services use `ExecuteAsyncWithToken` which includes the access token
4. **Bearer token**: The access token is sent as a Bearer token in the Authorization header for API calls

### API Base URL Pattern

```
https://www.aula.dk/api/v{API_VERSION}/
```

Where `API_VERSION` is a static int on the `Conf` class.

## 11. Cloud Storage OAuth (Google Drive / OneDrive)

Separate OAuth flows exist for cloud storage integration:

| Provider | Client ID Field | Redirect URL Field | Scope Field |
|---|---|---|---|
| **Google Drive** | `CloudConf.AndroidReleasePrivateClientId` | `/googleoauth2redirect` | `GoogleScope` |
| **OneDrive** | `OneDriveAndroidPrivateRedirectUrl` | `/onedrive2redirect` | `OneDriveScope` |

These use `CloudStorageAuthenticatorManager` / `OAuth2Authenticator` and are separate from the main Aula login flow.

## 12. Summary for Rust Library Implementation

To implement the auth flow in Rust:

1. **OIDC Authorization Code + PKCE flow** against SimpleSAMLphp endpoints
2. **Authority**: `https://login.aula.dk/`
3. **Authorize**: `https://login.aula.dk/simplesaml/module.php/oidc/authorize.php`
4. **Token**: `https://login.aula.dk/simplesaml/module.php/oidc/token.php`
5. **Client ID (level 3)**: `_99949a54b8b65423862aac1bf629599ed64231607a`
6. **Scope (level 3)**: `aula-sensitive`
7. **Redirect URI**: `https://app-private.aula.dk`
8. **No JWT validation needed** on client side (server handles this)
9. **Store**: access_token, refresh_token, expiration
10. **Refresh**: POST to token endpoint with refresh_token grant
11. **CSRF**: Extract `Csrfp-Token` from response cookies, send as `csrfp-token` header
12. **API calls**: `https://www.aula.dk/api/v{VERSION}/...` with cookies + CSRF header
13. **Keep-alive**: Hit the keep-alive endpoint periodically to maintain session
