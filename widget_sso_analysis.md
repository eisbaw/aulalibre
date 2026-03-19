# Widget SSO Token Mechanism Analysis

Analysis of how the Aula app authenticates third-party widgets via SSO tokens.

## Overview

Aula supports embedding third-party widgets that need access to user context. The app
uses a dedicated `aulaToken.getAulaToken` API endpoint to obtain a per-widget SSO token,
then passes that token (along with user context data) to the widget via URL query parameters.

There are two distinct rendering paths depending on widget type:
1. **In-app WebView** -- non-SSO widgets rendered inside `AulaPortalWebView`
2. **External browser** -- SSO-type widgets opened via `Browser.OpenAsync`

## Token Acquisition Flow

### API Endpoint

```
GET {BackendUrlApi}?method=aulaToken.getAulaToken&WidgetId={widgetId}
```

- URL code: `1601` (internal request ID in `Urls.cs`)
- Returns: `AulaServiceResponse<string>` containing the token as a plain string

### Call Chain

```
WidgetDto.GetSsoWidgetUrlAsync()
  -> WidgetServiceManager.Instance.GetAulaToken(widgetId)
       -> WidgetService.GetAulaToken(widgetId)
            -> SimpleService.Get<string>(Urls.GET_AULA_TOKEN(widgetId))
                 -> HTTP GET to backend API
```

The `WidgetServiceManager` is a singleton that wraps `WidgetService` and calls
`HandleResponseShowError()` on the response before returning `token.Data`.

## Token Injection Method

The token is injected into the widget URL as **query parameters**. The
`SsoWidgetDirectLinkArguments` class is serialized to a query string and appended
to the widget's base URL.

### SSO Widget URL Construction (WidgetDto.GetSsoWidgetUrlAsync)

```csharp
string token = await WidgetServiceManager.Instance.GetAulaToken(WidgetId);
string url = Url.Contains("?")
    ? Url + "&" + ConvertObjectToQueryUrl(new SsoWidgetDirectLinkArguments(token))
    : Url + "?" + ConvertObjectToQueryUrl(new SsoWidgetDirectLinkArguments(token));
url = url.Replace("csrfpToken", "Csrfp-Token");  // rename parameter
```

The final URL is opened in the **external system browser** via `Browser.OpenAsync(url)`.

### Parameters Passed to SSO Widgets (SsoWidgetDirectLinkArguments)

| Parameter | Value | Source |
|-----------|-------|--------|
| `sessionUuid` | Current user's ID | `ProfileManager.Instance.UserId` |
| `isMobileApp` | `true` (hardcoded) | Constant |
| `aulaToken` | Per-widget SSO token | `aulaToken.getAulaToken` API |
| `assuranceLevel` | `"3"` (hardcoded default) | Constant |
| `userProfile` | User's portal role (lowercase) | `ProfileManager.Instance.PortalRole` -- one of: `employee`, `child`, `guardian`, `otp`, `other` |
| `childFilter` | Comma-separated child user IDs | `ProfileManager.Instance.SelectedChildren` (filtered for non-empty UserId) |
| `institutionFilter` | Comma-separated institution codes | `ProfileManager.Instance.SelectedInstitutions` |
| `group` | Currently selected group ID | `GroupManager.Instance.ChoosingGroup?.Id` |
| `currentWeekNumber` | ISO week string e.g. `"2025-W12"` | `DateTime.Now` |
| `Csrfp-Token` | CSRF protection token | `HttpClientManager.Instance.CurrentCsrfpToken` |

Note: The `childFilter` and `institutionFilter` parameters and the `group` parameter are
only populated when no group is currently selected (`GroupManager.Instance.ChoosingGroup == null`).
When a group IS selected, only the `group` parameter is set (from the default field initializer)
and the filters remain null.

## Two Rendering Paths

### Path 1: In-App WebView (Non-SSO Widgets)

Used when `WidgetConfigurationDto.ShouldOpenBrowser` is `false` (i.e., `Widget.Type != "sso"`).

- Widget is rendered in `WidgetWebViewFragment` using `AulaPortalWebView`
- URL format: `{PortalUrl}widgetMobile/W{WidgetId}V{WidgetVersion}?{MobileWidgetArguments}`
- `MobileWidgetArguments` passes: `group`, `institutionFilter[]`, `childFilter[]`
- The WebView has JavaScript enabled and DOM storage enabled
- **BasicAuth**: `AulaWebViewClient.OnReceivedHttpAuthRequest` automatically responds
  with `Conf.BasicAuthUserName` and `Conf.BasicAuthUserPassword` -- this authenticates
  the WebView to the portal without user interaction
- URL interception: `ShouldOverrideUrlLoading` checks a whitelist and opens non-whitelisted
  URLs with user gesture in the system browser (prevents navigation away from widget)
- The whitelist (`WhiteListAulaPortalInAppConf`) only contains two data ethics policy URLs

### Path 2: External Browser (SSO Widgets)

Used when `WidgetConfigurationDto.ShouldOpenBrowser` is `true`, which requires:
- `Widget.Type == "sso"`
- `Widget.Url` is a well-formed absolute URI

- Token is fetched via `GetSsoWidgetUrlAsync()` which calls the `getAulaToken` API
- Full URL with all SSO parameters is opened in the system browser via `Browser.OpenAsync()`
- The external browser has no Aula-specific security controls

## Widget Configuration

Widgets are configured server-side and delivered as part of `PageConfiguration`:

- `WidgetDto`: Core widget data (id, name, url, type, widgetId, widgetVersion, canAccessOnMobile)
- `WidgetConfigurationDto`: Placement and display configuration wrapper around `WidgetDto`
- `WidgetPlacementEnum` (flags): `OwnPage=1`, `RightOfOverview=2`, `RightOfCalendar=4`, `BelowCalendar=8`
- Widgets can appear in bottom navigation bar, on the overview page, or on the calendar page

## Widget Notifications

- Push notification type: `RemoteNotificationType.WidgetPushNotification`
- Notification settings: `WidgetNotificationSettings` (title, widgetId, isActive)
- Notification navigation: When a widget notification arrives, the app checks if the widget
  belongs to `CalendarWidgetIds` or `OverviewWidgetIds` and navigates to the appropriate page
- The widget ID is extracted from the notification ID: `id.Substring(0, id.IndexOf('-'))`

## Security Observations

### Token Exposure via URL Query Parameters

All SSO parameters -- including the `aulaToken`, `sessionUuid`, `Csrfp-Token`, `childFilter`,
and `institutionFilter` -- are passed as **URL query parameters**. This means:

1. **Browser history**: The full URL with token is stored in browser history
2. **Server logs**: Any intermediate servers or CDNs may log the full URL
3. **Referer header**: If the widget page loads external resources, the token-bearing URL
   may leak via the HTTP `Referer` header
4. **Shoulder surfing**: The URL bar displays sensitive parameters

### CSRF Token Forwarding

The app's `CurrentCsrfpToken` is forwarded to third-party widgets. The parameter is renamed
from `csrfpToken` to `Csrfp-Token` via string replacement. This gives widgets the ability
to make CSRF-protected requests on behalf of the user if they know the API structure.

### BasicAuth Auto-Response in WebView

The `AulaPortalWebView` automatically responds to HTTP Basic Auth challenges with hardcoded
credentials (`Conf.BasicAuthUserName`, `Conf.BasicAuthUserPassword`). These credentials are
shared across all widget WebViews. This is likely for portal-level authentication rather than
user-level, but it means any in-app widget can trigger a Basic Auth challenge and receive
these credentials.

### AssuranceLevel Hardcoded

The `assuranceLevel` is hardcoded to `"3"` in the mobile app. This value likely indicates
a trust level (possibly related to the NSIS/NemID/MitID authentication assurance levels used
in Danish public sector). Hardcoding this means the widget always receives the same assurance
claim regardless of how the user actually authenticated.

### Child and Institution Data Exposure

Third-party widgets receive the user IDs of selected children and institution codes. This
constitutes PII that flows to external parties. The scope of what widgets can do with this
data depends on the backend validation of the `aulaToken`.

### No Token Scoping Visible Client-Side

The `getAulaToken` API takes only `widgetId` as a parameter. Any scoping/restrictions on what
the token authorizes must be enforced server-side. The client has no visibility into token
permissions or expiration.
