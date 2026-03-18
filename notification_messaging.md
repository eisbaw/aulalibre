# Push Notification and Real-Time Messaging Analysis

**App**: com.netcompany.aulanativeprivate v2.15.4
**Framework**: .NET MAUI / Xamarin (Mono runtime, .NET 9.0)
**Analysis method**: `monodis` type/field extraction and string analysis of extracted .NET assemblies

---

## 1. Push Notification Provider: Firebase Cloud Messaging (FCM)

### Firebase Configuration

| Component | Value |
|-----------|-------|
| Firebase Project | `aula-private` |
| FCM Sender ID | `811573413698` |
| API Key | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` |
| Realtime Database | `aula-private.firebaseio.com` |
| Storage Bucket | `aula-private.appspot.com` |
| OAuth Web Client ID | `811573413698-mnjq3uvi6b23ajkghjp4rodbtgks0uqt.apps.googleusercontent.com` |

### Android Service Class

**`AulaNative.Droid.FireBase.AulaFirebaseMessagingService`** extends `FirebaseMessagingService` (via Xamarin.Firebase).

This class handles:
- `onMessageReceived` -- receives FCM data messages from the Aula backend
- `onNewToken` -- receives FCM token refreshes, triggers device re-registration

### Supporting Classes

| Class | Purpose |
|-------|---------|
| `AulaNative.Droid.FireBase.RemoteNotificationsUtils` | Static utility for processing incoming remote notifications on Android |
| `AulaNative.Utils.Notifications.FirebaseNotificationParser` | Static parser that extracts structured notification data from FCM data payload |
| `AulaNative.Utils.MessagingCenter.PushNotificationMessagingCenter` | Internal pub/sub bus for distributing push events within the app |
| `AulaNative.Droid.Activities.Common.Processors.AulaNotificationHandler` | Processes notifications and triggers UI updates |
| `AulaNative.Droid.Activities.Common.BaseActivities.AulaBaseAbstractPushNotificationNavigatorActivity` | Base activity that all push-navigable activities extend |
| `AulaNative.Droid.CrossPlatformServices.NotificationPermissionService.AndroidNotificationPermissionService` | Manages Android notification channel and permission requests |

---

## 2. Real-Time Update Mechanism: Polling + FCM Push (No WebSocket/SSE/SignalR)

### Finding: No persistent real-time connection

The app does **not** use WebSockets, SignalR, or Server-Sent Events. Evidence:

1. **No WebSocket usage**: `System.Net.WebSockets.Client.dll` is present as a .NET framework dependency but no references to `ClientWebSocket`, `wss://`, or `ws://` exist in the AulaNative assemblies.
2. **No SignalR**: No SignalR assemblies (`Microsoft.AspNetCore.SignalR.Client`, etc.) are present in the extracted assemblies.
3. **No SSE**: No references to `EventSource`, `text/event-stream`, or server-sent event patterns.

### Actual real-time architecture

The app uses a **two-tier approach**:

#### Tier 1: FCM Push Notifications (server-initiated)
- The Aula backend sends FCM data messages to registered devices when events occur
- The app receives them via `AulaFirebaseMessagingService.onMessageReceived`
- `FirebaseNotificationParser` extracts structured data from the FCM payload
- `PushNotificationMessagingCenter` distributes the event internally via the app's pub/sub bus
- The app then navigates to the appropriate screen or updates badge counts

#### Tier 2: API Polling (client-initiated)
- **KeepAlive**: `ProfileServiceManager.KeepAlive()` calls `POST /profiles/keepAlive` to maintain session
- **Notification fetch**: `NotificationServiceManager.GetNotificationsForActiveProfile()` calls `GET /notifications` to pull latest in-app notifications
- **Badge refresh**: `GetBadgeNotifications` and related methods fetch badge counts from the API
- **Pull-to-refresh**: `SwipeRefreshLayout` (AndroidX) is used throughout the app for manual refresh

#### Internal Event Bus (not real-time with server)
The app has a sophisticated internal `MessagingCenter` pub/sub system (20+ channels) for cross-component communication, but this is purely in-process -- it does not maintain any connection to the server. Channels include:
- `PushNotificationMessagingCenter` (push notification key, app-is-open key)
- `NotificationMessagingCenter` (notification change key)
- `SubscriptionMessagingCenter` (thread list change, thread change, thread delete, move thread folder)
- `AlbumMessagingCenter`, `CommentMessagingCenter`, `PostMessagingCenter`, `GalleryMessagingCenter`, etc.

---

## 3. Notification Payload Format

### FCM Data Payload Keys

Based on the `FirebaseNotificationParser` and `BaseNotificationViewModel`, the FCM data message contains these keys:

| Key | Type | Description |
|-----|------|-------------|
| `notificationType` | string | Maps to `RemoteNotificationType` enum (e.g., "NewMessagePrivateInbox") |
| `notificationArea` | string | Maps to `NotificationArea` enum (e.g., "Messages", "Calendar") |
| `notificationId` | string | Unique notification identifier |
| `institutionCode` | string | Institution code for the notification context |
| `InstitutionProfileId` | int64 | Profile ID within the institution |
| `threadId` | int64 | Message thread ID (for message notifications) |
| `albumId` | int64 | Album ID (for gallery notifications) |
| `mediaId` | int64 | Media ID (for gallery notifications) |
| `widgetId` | int32 | Widget ID (for widget notifications) |

### RemoteNotificationType Enum (50 values)

These are the push notification types the app can receive from the server:

**Messages (5)**:
- `NewMessagePrivateInbox`, `NewSensitiveMessagePrivateInbox`
- `NewMessageCommonInbox`, `NewSensitiveMessageCommonInbox`
- `LessonNoteChanged`

**Calendar/Events (17)**:
- `InvitedToEventNoResponseRequired`, `InvitedToEventResponseRequired`
- `InvitedToRepeatingEventResponseRequired`
- `EventChangedResponseRequired`, `EventChangedNoResponseRequired`
- `EventCancelledNoReason`
- `InvitedToParentalMeeting`, `InvitedToSchoolHomeMeeting`, `InvitedToPerformanceMeeting`
- `GrantedRightsForOtherCalendar`, `RemovedRightsForOthersCalendar`
- `LostRoomBecauseOfExternalScheduling`
- `SomeoneElseRespondedToEvent`, `SomeoneElseRemovedYourResponseToEvent`
- `EventChangedBySomeoneElse`, `EventCancelledBySomeoneElse`
- `InvitedToExceptionEvent`, `InvitedToSingleOccurrenceOfEvent`

**Posts (4)**:
- `PostSharedWithMe`, `PostWasRemovedFromGroupByAdmin`, `PostWasDeleted`
- `NewPostComment`

**Gallery/Media (4)**:
- `AlbumShared`, `MediaAddedToAlbum`, `TaggedInMedia`, `NewMedia`

**Documents (2)**:
- `NewCommonFile`, `NewOrUpdatedSecureDocument`

**Presence/ComeGo (3)**:
- `PresenceRegistrationUpdatedExitWith`
- `VacationResponseRequired`, `VacationResponseCancelledNoResponse`

**Schedule (2)**:
- `SubstituteAdded`, `DashboardWasUpdated`

**Other (4)**:
- `WidgetPushNotification`, `InvitedToSurvey`, `GeneralInformation`, `Unknown`

**File Scan Failures (9)**:
- `FileScanFailedPost`, `FileScanFailedEvent`
- `FileScanFailedPrivateInboxMessage`, `FileScanFailedCommonInboxMessage`
- `FileScanFailedInternalSecureDocument`, `FileScanFailedExternalSecureFile`
- `FileScanFailedAlbum`, `FileScanFailedProfilePicture`

### In-App Notification DTO (from API response)

The `NotificationItemDto` represents notifications fetched from `GET /notifications`:

| Field | Type | Description |
|-------|------|-------------|
| `NotificationId` | string | Unique ID |
| `NotificationEventType` | `NotificationEventType` enum | Event type (57 values, superset of remote types) |
| `NotificationArea` | `NotificationArea` enum | Area categorization |
| `NotificationType` | `NotificationType` enum | Badge, Alert, Irrelevant, Unknown |
| `InstitutionCode` | string | Institution context |
| `InstitutionProfileId` | int64? | Profile context |
| `GeneralInformationId` | int64? | General info reference |
| `Title` | string | Notification title |
| `OriginalTitle` | string | Original title (before localization/formatting) |
| `Content` | HtmlDto | HTML content body |
| `Url` | string | Deep link URL |
| `Triggered` | DateTime? | When the event was triggered |
| `Expires` | DateTime? | Expiration time |
| `ResponseDeadline` | DateTime? | Deadline for response (events) |
| `StartTime` / `EndTime` | DateTime? | Event time range |
| `StartDate` / `EndDate` | DateTime? | Event date range |
| `EventId` | int64? | Related event ID |
| `ThreadId` | int64 | Related message thread ID |
| `FolderId` | int64? | Message folder ID |
| `RelatedChildInstitutionProfileId` | int64? | Related child profile |
| `RelatedChildName` | string | Related child name |
| `RelatedInstitution` | string | Related institution name |
| `ResponderName` | string | Name of person who responded |
| `SenderName` | string | Name of sender |
| `MessageText` | string | Message text preview |
| `OtherCalendarPersonName` | string | Name for shared calendar |
| `OtherCalendarInstitutionProfileId` | int32? | Profile ID for shared calendar |

---

## 4. Subscription/Channel Model

### Device Registration

The app registers devices with the Aula backend for push notification delivery via these API endpoints:

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/pushNotifications/devices` | POST | Register device (sends `ConfigureDeviceModel`) |
| `/pushNotifications/devices/{id}` | DELETE | Unregister specific device |
| `/pushNotifications/devices` | DELETE | Delete all registered devices |
| `/pushNotifications/devices` | GET | List registered devices |

**`ConfigureDeviceModel`** (sent during registration):

| Field | Type | Description |
|-------|------|-------------|
| `CurrentToken` | string | FCM device token |
| `DeviceId` | string | Unique device identifier |
| `DeviceDescription` | string | Human-readable device name |
| `DeviceAccessGranted` | bool | Whether push access is granted |
| `Platform` | Platform enum | Android/iOS |

### Notification Settings (Per-User Subscription Configuration)

Users control which push notifications they receive via `NotificationSettings`:

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/pushNotifications/settings` | GET | Fetch current settings |
| `/pushNotifications/settings` | PUT | Update settings |
| `/pushNotifications/badges/clear` | POST | Clear badge counts by module |

**`NotificationSettings`** model:

| Field | Type | Description |
|-------|------|-------------|
| **Delivery mode** | | |
| `Instant` | bool | Push immediately on event |
| `ScheduledTime` | string | Scheduled delivery time (e.g., "07:00") |
| **Day-of-week filter** | | |
| `Monday` through `Sunday` | bool | Which days to receive notifications |
| **Content channels** | | |
| `NotifyMessages` | bool | New messages |
| `NotifyMessagesFromEmployees` | bool | Messages from school employees |
| `NotifyMessagesFromChildren` | bool | Messages from children |
| `NotifyMessagesFromGuardians` | bool | Messages from other guardians |
| `NotifyCalendar` | bool | Calendar events |
| `NotifyGallery` | bool | Gallery/media updates |
| `NotifyPosts` | bool | New posts |
| `NotifyAssignedAsSubstituteTeacher` | bool | Substitute teacher assignments |
| `NotifyLessonNoteChanged` | bool | Lesson note changes |
| **Delivery platform** | | |
| `EmailDisabled` | bool | Email notifications disabled |
| `EmailAvailable` | bool | Email notifications available |
| `AppDisabled` | bool | App push notifications disabled |
| `AppAvailable` | bool | App push notifications available |
| **Special channels** | | |
| `ComeGoNotificationSettings` | ComeGoNotificationSettings[] | Presence/come-go notification settings |
| `DeviceList` | List\<SimpleDevice\> | Registered devices |
| `WidgetSettings` | List\<WidgetNotificationSettings\> | Per-widget notification settings |

**`ComeGoNotificationSettings`**:
- `ComeGoType` (string) -- type of come/go event
- `Activated` (bool) -- whether notifications are enabled for this type

**`WidgetNotificationSettings`**:
- `Title` (string) -- widget display name
- `WidgetId` (int32) -- widget identifier
- `IsActive` (bool) -- whether notifications are active

### NotificationArea Enum (Module-Level Channels)

Notifications are categorized into these areas, which correspond to the app's main modules:

| Value | Description |
|-------|-------------|
| `Unknown` | Uncategorized |
| `Messages` | Private/common inbox messages |
| `Calendar` | Calendar events, invitations, meetings |
| `Posts` | Posts and announcements |
| `Schedule` | School schedule, lesson notes |
| `Administration` | Administrative notifications |
| `Gallery` | Photos, media, albums |
| `Documents` | Secure documents, common files |
| `Album` | Album-specific notifications |
| `Presence` | Come/go, presence registration |
| `Widget` | Widget-specific notifications |
| `FileScanning` | File scan failure notifications |

### NotificationType Enum (Severity/Priority)

| Value | Description |
|-------|-------------|
| `Badge` | Silent badge count update only |
| `Alert` | Visible alert notification requiring attention |
| `Irrelevant` | No user notification needed |
| `Unknown` | Unclassified |

### Message Thread Subscription Model

For message threads, the app uses a subscription model:

**`SubscriptionType`** enum:
- `Bundle` -- bundled/grouped thread subscription
- `BundleItem` -- item within a bundled subscription
- `Unbundled` -- standalone thread subscription

**`SubscriptionStatus`** enum:
- `Read` -- user has read the latest
- `Unread` -- new content not yet seen

Thread subscription tracking via `ThreadSubscriptionInfoViewModel`:
- `SubscriptionId` (int64) -- unique subscription identifier
- `ThreadId` (int64) -- the message thread
- `BundleId` (int64?) -- optional bundle grouping
- `IsMuted` (bool) -- notifications suppressed
- `IsRead` (bool) -- read status
- `IsMarked` (bool) -- bookmarked/flagged
- `HasLeftThread` (bool) -- user left the conversation
- `LeaveTime` (DateTime?) -- when user left

---

## 5. Push Notification Navigation

When a push notification is tapped, the app navigates to the appropriate detail screen. The `NavigationDetailViewHelper` handles routing:

| Method | Target |
|--------|--------|
| `NavigateToAlbumDetailsFromPushNotification` | Album detail page |
| `NavigateToEventDetailsFromPushNotification` | Calendar event detail |
| `NavigateToMasterDataDetailsFromPushNotification` | Master data/profile detail |
| `NavigateToNewNotification` | Notification overview |
| `NavigateToSecureDocumentDetailsFromPushNotification` | Secure document viewer |
| `NavigateToThreadDetailsFromPushNotification` | Message thread detail |

Two keys in `PushNotificationMessagingCenter`:
- `_pushNotificationKey` -- standard push notification received
- `_pushNotificationAppIsOpenKey` -- push received while app is in foreground (different handling: update UI in-place vs. show banner)

---

## 6. Architecture Summary

```
                     +-----------------------+
                     |   Aula Backend API    |
                     +----------+------------+
                                |
               +----------------+----------------+
               |                                 |
      (server push)                      (client poll)
               |                                 |
    +----------v-----------+        +------------v-----------+
    |  Firebase Cloud       |        | REST API Endpoints     |
    |  Messaging (FCM)      |        | GET /notifications     |
    |  Project: aula-private|        | POST /profiles/keepAlive|
    +----------+------------+        +------------+-----------+
               |                                  |
    +----------v-----------+        +-------------v----------+
    |AulaFirebaseMessaging-|        |NotificationService-    |
    |Service               |        |Manager                 |
    | (Android receiver)   |        | (API client)           |
    +----------+-----------+        +-------------+----------+
               |                                  |
    +----------v-----------+                      |
    |FirebaseNotification- |                      |
    |Parser                |                      |
    | (payload extraction) |                      |
    +----------+-----------+                      |
               |                                  |
    +----------v----------------------------------v----------+
    |         PushNotificationMessagingCenter                 |
    |         NotificationMessagingCenter                     |
    |         (internal pub/sub event bus)                    |
    +-----------+-------------------+------------------------+
                |                   |
    +-----------v------+  +---------v-----------+
    |AulaNotification- |  |NotificationData-    |
    |Handler           |  |Manager              |
    | (UI updates)     |  | (badge counts,      |
    +------------------+  |  notification lists) |
                          +---------------------+
```

---

## 7. Limitations and Caveats

1. **monodis crashes on assembly_0**: Full IL disassembly of `AulaNative.Droid.dll` was not possible due to segfaults. Class structures were obtained from `--typedef` and `--fields` options instead.
2. **No IL method bodies**: Without full decompilation, we cannot see the exact FCM payload parsing logic in `FirebaseNotificationParser`, only the input/output types and field names.
3. **KeepAlive interval unknown**: The polling interval for `KeepAlive` and notification refresh is set in method bodies, not visible from metadata alone.
4. **Firebase Realtime Database**: The `aula-private.firebaseio.com` reference exists but its role is unclear -- it could be used for real-time features (like presence) that we cannot confirm from assembly metadata alone.
