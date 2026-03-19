# KeepAlive, Polling Intervals, and Notification Refresh Analysis

Analysis of session keepalive mechanisms, polling intervals, and notification refresh
patterns in the Aula Android app (v2.15.4).

## Session KeepAlive

### Mechanism

The KeepAlive is an **on-demand HTTP GET** call, not a periodic timer-based poll.

- **Endpoint**: `?method=session.keepAlive` (URL ID 1501)
- **Full URL**: `https://app-private.aula.dk/api/v23/?method=session.keepAlive`
- **Call chain**: `ProfileService.KeepAlive()` -> `ProfileServiceManager.KeepAlive()` -> callers

### When KeepAlive Is Called

KeepAlive is called **reactively**, not proactively:

1. **Device unregistration** (`RemoteNotificationServiceManager.UnregisterDevice`):
   Before unregistering a device from push notifications, if no CSRF token exists,
   KeepAlive is called to refresh the session and obtain a fresh CSRF token.

2. **No periodic timer**: There is **no background timer** that calls KeepAlive on a
   fixed interval. The app does not poll the server to keep the session alive. Instead,
   the session is maintained by normal user-initiated API activity.

### Session Expiration

The session expiration is managed client-side by `SessionPromptManager`:

| Constant | Value | Purpose |
|----------|-------|---------|
| `MINUTES_BEFORE_SESSION_EXPIRES` | **60 minutes** | Session timeout from last activity |
| `MINUTES_BEFORE_WARNING_SESSION_WILL_EXPIRE` | **55 minutes** | Show warning dialog |
| `BufferOnTokenExpiration` | **2 minutes** | Buffer before token refresh |

The `SessionPromptManager` runs a **1-second polling loop** (`Task.Delay(1000)`) that:
- Tracks `LastActive` timestamp (UTC)
- After 55 minutes of inactivity: shows a countdown warning dialog
- After 60 minutes of inactivity: navigates to PIN/login screen
- Resets `LastActive` on user interaction
- Persists timing state across background/foreground transitions via `Preferences`

## Notification Refresh

### Push-Based (Primary Mechanism)

The app uses **Firebase Cloud Messaging (FCM)** as the primary notification delivery:

- `AulaFirebaseMessagingService` extends `FirebaseMessagingService`
- On message received: parses the notification, broadcasts via `PushNotificationMessagingCenter`
- Calls `NotificationMessagingCenter.NotifyNotificationListChanged()` to invalidate caches
- **No periodic polling for notifications** -- they arrive via push

### Notification Cache

Notification data is cached with a very short lifetime:

| Parameter | Value | Purpose |
|-----------|-------|---------|
| `NotificationCacheLifetime` | **0.3 seconds** | Cache lifetime for notification data |

The `NotificationCacheManager` uses a `System.Threading.Timer` that fires every 0.3s
to delete expired caches. The `ShouldFetchNew` property returns `true` if the cached
notifications are older than 0.3s or null -- effectively meaning every API call
re-fetches fresh data.

- **Endpoint**: `?method=notifications.getNotificationsForActiveProfile` (URL ID 805)
- **Trigger**: View navigation, push notification receipt, or MessagingCenter events
- **Not timer-polled**: Fetched on demand when the UI needs badge counts

### Badge Updates

Badge counts are computed from notification data:
- `NotificationDataManager.GetBadgeViewModelFromAllModules()` fetches notifications
  and processes them into badge counts per area (Posts, Messages, Gallery, Calendar, etc.)
- `IBadgeUpdate.UpdateBadgeCountOnHomeScreen()` updates the app icon badge
- Updated when: push notification arrives, user navigates between tabs, or
  notification data is explicitly refreshed

## Other Auto-Refresh Timers

### Gallery Auto-Refresh

The only `AutoRefreshManager` usage found is for gallery content:

| Component | Interval | Timer Tick | Purpose |
|-----------|----------|------------|---------|
| `GalleryFragment` | **15 minutes** (`GALLERY_CACHE_TIMEOUT`) | 1 second | Refresh gallery list |
| `AlbumDetailsFragment` | **15 minutes** (`GALLERY_CACHE_TIMEOUT`) | 1 second | Refresh album details |

`AutoRefreshManager` works by:
- Running a 1-second `System.Timers.Timer`
- On each tick, checking if `TimeSpan` since last refresh exceeds the configured interval
- If yes, invoking the `OnRefresh` callback
- Started/stopped/resumed with fragment lifecycle

### AulaTaskScheduler Instances

| Component | Action Interval | Check Interval | Purpose |
|-----------|----------------|----------------|---------|
| `SecureDocumentOverviewPageViewModel` (export tracking) | 1 second | 1 second | Poll export job status |
| `SecureDocumentOverviewPageViewModel` (document refresh) | 5 minutes | 30 seconds | Refresh document overview |

### BaseDataManager Throttling

`BaseDataManager<T>` throttles repeated API calls with `_minimumIntervalBetweenApiCalls`:

| DataManager | Minimum Interval | Purpose |
|-------------|-----------------|---------|
| Default (`BaseDataManager`) | **1 minute** | Throttle repeated fetches |
| `MessageFolderDataManager` | **1 minute** | Message folder data |
| `EventTypeDataManager` | **1 hour** | Calendar event types |
| `ComeGoConfigurationDataManager` | **1 minute** | Come-and-go config |

These are **not polling intervals** -- they are minimum delays between consecutive
fetches. The data is fetched on demand (e.g., when navigating to a screen), not on
a timer.

## Background Processing

- **No WorkManager usage**: No Android WorkManager, JobScheduler, or AlarmManager
  patterns were found in the decompiled code.
- **No background polling**: The app does not poll for data in the background.
- **FCM only**: Background notification delivery relies entirely on Firebase push
  notifications.
- **Session does not persist in background**: The `SessionPromptManager` dismisses
  its counting thread when the app moves to background and recalculates on foreground
  resume.

## Summary of Timing Constants

| Timer/Interval | Value | Type |
|----------------|-------|------|
| Session timeout | 60 min | Client-side idle timeout |
| Session warning | 55 min | Warning dialog before timeout |
| Token expiry buffer | 2 min | Pre-emptive token refresh |
| Notification cache | 0.3 sec | Cache invalidation (effectively no cache) |
| Gallery auto-refresh | 15 min | Periodic UI refresh |
| SecureDoc export poll | 1 sec | Job status polling |
| SecureDoc overview refresh | 5 min | Periodic UI refresh |
| BaseDataManager throttle | 1 min | API call deduplication |
| EventType throttle | 1 hour | Rarely-changing data |
| Session countdown tick | 1 sec | Client-side timer resolution |
| AutoRefreshManager tick | 1 sec | Timer check resolution |
| HTTP timeout (default) | 1 min | Request timeout |
| HTTP timeout (cloud) | 1 hour | Upload/download timeout |

## Key Findings

1. **KeepAlive is not periodically polled.** It is only called reactively before device
   unregistration when no CSRF token exists. The session is kept alive implicitly by
   normal API calls during active use.

2. **No background polling exists.** The app relies entirely on FCM push notifications
   for background updates. There are no WorkManager jobs, AlarmManager alarms, or
   background timers.

3. **Notification data has an extremely short cache (0.3s)**, meaning it is effectively
   re-fetched from the server on every UI request. This is event-driven, not polled.

4. **Gallery is the only auto-refreshing UI component**, refreshing every 15 minutes
   via `AutoRefreshManager`.

5. **Session management is entirely client-side.** A 1-second timer checks idle time
   and shows a warning at 55 minutes, forcing re-login at 60 minutes.
