# Firebase Integration Analysis - Aula Private App

## Firebase Project Configuration

Extracted from `apktool_out/res/values/strings.xml`:

| Key | Value |
|-----|-------|
| `project_id` | `aula-private` |
| `google_app_id` | `1:811573413698:android:4b7d0241cc907d17` |
| `gcm_defaultSenderId` | `811573413698` |
| `google_api_key` | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` |
| `firebase_database_url` | `https://aula-private.firebaseio.com` |
| `google_storage_bucket` | `aula-private.appspot.com` |

## 1. Firebase Services Used by the App

### Firebase Cloud Messaging (FCM) - ACTIVELY USED

FCM is the **only** Firebase service with active code-level integration. It serves as the push notification delivery channel.

**Code evidence:**
- `AulaFirebaseMessagingService` (Android Service, `AulaNative.Droid.FireBase`) - registered for `com.google.firebase.MESSAGING_EVENT`
- `FirebaseNotificationParser` (shared code, `AulaNative.Utils.Notifications`) - parses incoming FCM payloads
- `DroidConfigureDeviceService` - retrieves FCM token via `FirebaseMessaging.Instance.GetToken()`
- Assembly reference: `Xamarin.Firebase.Messaging` in the Droid project

**Device registration flow:**
1. App calls `FirebaseMessaging.Instance.GetToken()` to get the FCM registration token
2. Token is sent to the Aula backend via `Urls.REGISTER_DEVICE` (`?method=notifications.registerDevice`)
3. Backend uses this token to send push notifications back through FCM
4. The `RegisterDeviceDto` includes: `DeviceId`, `PushNotificationToken`, `Os`, `NotificationsActive`, `AppType` ("Private"), `Description`

### Firebase Realtime Database - NOT USED IN CODE

Despite being configured (`https://aula-private.firebaseio.com`), there are **zero references** to Firebase Realtime Database APIs in the decompiled source:
- No `FirebaseDatabase`, `DatabaseReference`, or `ValueEventListener` usage found
- No code reads from or writes to the Realtime Database
- The database URL is present only because it is part of the standard Firebase config (`google-services.json` -> `strings.xml`)

The Realtime Database URL is likely a default configuration artifact from the Firebase project setup, not an actively used service.

### Firebase Storage - NOT USED IN CODE

Despite being configured (`aula-private.appspot.com`), there are **zero references** to Firebase Storage APIs in the decompiled source:
- No `FirebaseStorage`, `StorageReference`, or `UploadTask` usage found
- File uploads/downloads go through the Aula backend API (`?method=files.getDownloadUrl`, etc.), not Firebase Storage directly

## 2. FCM Push Notification Payload Structure

### Data Payload Fields

The FCM payload uses the **data message** pattern (not notification-only). The app extracts the following fields:

**From `FirebaseNotificationParser.Parse()` (foreground handling):**
- `elementId` (string, required) - ID of the content that triggered the notification
- `type` (string, required) - notification type string (e.g., "NewMessagePrivateInboxNotification")

If either `elementId` or `type` is missing, the notification is silently discarded.

**From `RemoteNotificationsUtils.HandleClickedNotification()` (background/clicked handling):**
The intent extras contain additional fields beyond the two above:
- `elementId` (string) - same as above
- `type` (string) - same as above
- `id` (string) - notification ID
- `relatedChildInstitutionProfileId` (long) - profile ID of the related child
- `commonInboxId` (long) - ID of the common inbox (if message notification)
- `commentId` (int) - ID of the comment (if comment notification)
- `occurrenceDateTime` (string, parsed as DateTime) - when the event occurred
- `profilePictureInstitutionProfileId` (long) - profile ID for avatar lookup

**Note on the `notification.Body` field:**
`AulaFirebaseMessagingService.NotifyNotificationReceived()` also reads `message.GetNotification().Body` - this is the human-readable notification text displayed in the system notification tray. This body text is logged but not stored in `RemoteNotification`.

### RemoteNotification Model

The parsed notification is stored as a `RemoteNotification` object with these properties:
- `ProfileId` (long)
- `ElementId` (string)
- `Id` (string)
- `Type` (string)
- `RelatedChildInstProfileId` (long)
- `CommonInboxId` (long?)
- `CommentId` (int?)
- `OccurrenceDateTime` (DateTime?)
- `ProfilePictureInstitutionProfileId` (long?)

### Notification Types (47 distinct types)

The `Type` field maps to the `RemoteNotificationType` enum (47 values). Categories:

**Messages:** NewMessagePrivateInbox, NewSensitiveMessagePrivateInbox, NewMessageCommonInbox, NewSensitiveMessageCommonInbox
**Calendar/Events:** InvitedToEventNoResponseRequired, InvitedToEventResponseRequired, InvitedToRepeatingEventResponseRequired, EventChangedResponseRequired, EventChangedNoResponseRequired, EventCancelledNoReason, EventChangedBySomeoneElse, EventCancelledBySomeoneElse, InvitedToExceptionEvent, InvitedToSingleOccurrenceOfEvent, SomeoneElseRespondedToEvent, SomeoneElseRemovedYourResponseToEvent, LostRoomBecauseOfExternalScheduling
**Meetings:** InvitedToParentalMeeting, InvitedToSchoolHomeMeeting, InvitedToPerformanceMeeting
**Posts:** PostSharedWithMe, PostWasRemovedFromGroupByAdmin, PostWasDeleted, NewPostComment
**Gallery/Media:** AlbumShared, MediaAddedToAlbum, TaggedInMedia, NewMedia, NewCommonFile
**Presence/ComeGo:** PresenceRegistrationUpdatedExitWith
**Other:** SubstituteAdded, DashboardWasUpdated, LessonNoteChanged, NewOrUpdatedSecureDocument, VacationResponseRequired, VacationResponseCancelledNoResponse, GrantedRightsForOtherCalendar, RemovedRightsForOthersCalendar, WidgetPushNotification, InvitedToSurvey, GeneralInformation
**File Scan Failures:** FileScanFailedPost, FileScanFailedEvent, FileScanFailedPrivateInboxMessage, FileScanFailedCommonInboxMessage, FileScanFailedInternalSecureDocument, FileScanFailedExternalSecureFile, FileScanFailedAlbum, FileScanFailedProfilePicture

### Privacy Assessment of FCM Payloads

The FCM data payload contains **only IDs and type strings** - no message content, names, or PII. This is a privacy-conscious design:
- `elementId` is an opaque numeric ID
- `type` is a fixed enum string
- Additional fields (`relatedChildInstitutionProfileId`, `commonInboxId`, etc.) are also numeric IDs
- The notification body text (`notification.Body`) is the only potentially sensitive field, as it may contain a preview of the message or event name - this is standard Android notification behavior and is not stored in the app's data model

The app must then fetch the actual content from the Aula API using the `elementId` and `type` to navigate to the correct screen.

## 3. Firebase Security Rules Probing

### Realtime Database

All paths return HTTP 401 with `{"error": "Permission denied"}`:

| Path | Status |
|------|--------|
| `/.json` | 401 - Permission denied |
| `/users.json` | 401 - Permission denied |
| `/messages.json` | 401 - Permission denied |
| `/notifications.json` | 401 - Permission denied |
| `/profiles.json` | 401 - Permission denied |
| `/schools.json` | 401 - Permission denied |

**Assessment:** The Realtime Database requires authentication for all reads. No public data exposure. Since the app does not actually use the RTDB, this is likely the default security rules (deny all) applied to an unused database.

### Firebase Storage

| Endpoint | Status |
|----------|--------|
| `https://aula-private.appspot.com/` | 404 |
| `https://firebasestorage.googleapis.com/v0/b/aula-private.appspot.com/o` | 412 - Service account permissions error |
| `https://firebasestorage.googleapis.com/v0/b/aula-private.appspot.com/o?maxResults=1` | 412 |

**Assessment:** Firebase Storage returns a 412 error indicating a service account configuration issue, not an authentication bypass. No public listing of files is possible. Since the app does not use Firebase Storage in code, this bucket may be unused or used only server-side.

## 4. Notification Backend Integration

All notification management goes through the Aula backend API, not Firebase directly:

| Endpoint | Purpose |
|----------|---------|
| `notifications.registerDevice` | Register FCM token with backend |
| `notifications.unregisterDevice` | Remove device from push notifications |
| `notifications.unregisterAllDevices` | Remove all devices |
| `notifications.getNotificationSettingsForActiveProfile` | Get notification preferences |
| `notifications.setNotificationSettingsForActiveProfile` | Update notification preferences |
| `notifications.getNotificationsForActiveProfile` | Fetch notification list (from API, not Firebase) |
| `notifications.deleteNotifications` | Delete notifications |
| `notifications.deleteNotificationsByRelatedChild` | Delete child-specific notifications |
| `notifications.deleteBadgeNotificationByModule` | Clear badge counts by module |

### Notification Settings Model

Users can configure:
- **Schedule:** `ScheduledTime`, day-of-week toggles (Mon-Sun), `Instant` (immediate delivery)
- **Message filters:** `NotifyMessages`, `NotifyMessagesFromEmployees`/`Children`/`Guardians`
- **Category toggles:** `NotifyCalendar`, `NotifyGallery`, `NotifyPosts`
- **Teacher-specific:** `NotifyAssignedAsSubstituteTeacher`, `NotifyLessonNoteChanged`
- **Channel toggles:** `EmailDisabled`/`EmailAvailable`, `AppDisabled`/`AppAvailable`
- **Per-widget settings:** `WidgetNotificationSettings` (via `widgetNotificationSettingDtos`)
- **ComeGo settings:** `ComeGoNotificationSettings` (arrival/departure notifications)

## 5. Data Flow Summary

```
Server-side event occurs (new message, calendar invite, etc.)
    |
    v
Aula Backend sends FCM message via Firebase Admin SDK
    |  Payload: { elementId, type, id, relatedChildInstitutionProfileId, ... }
    |  + notification.body (display text)
    v
FCM delivers to device
    |
    v
AulaFirebaseMessagingService.OnMessageReceived()
    |
    +--> FirebaseNotificationParser.Parse(data)  -->  RemoteNotification { elementId, type }
    |
    +--> PushNotificationMessagingCenter.NotifyNotificationsWhileAppIsOpen()
    |       (if app is foreground)
    |
    +--> NotificationMessagingCenter.NotifyNotificationListChanged()
    |       (triggers UI refresh of notification list)
    |
    v
On notification click (from background):
    RemoteNotificationsUtils.HandleClickedNotification()
    --> Extracts all fields from Intent extras
    --> Stores in SessionPromptManager.ActivatedRemoteNotification
    --> App navigates to appropriate screen based on NotificationType
```

## 6. Key Findings

1. **Firebase is used ONLY for FCM push delivery.** Realtime Database and Storage are configured but have zero code-level usage. The app is not a "Firebase app" - it uses Firebase purely as a push notification transport.

2. **FCM payloads are privacy-conscious.** Only opaque IDs and type strings are sent. No message content, names, or PII in the data payload. The notification body text (for system tray display) may contain preview text.

3. **All Firebase endpoints are properly secured.** RTDB returns 401 on all paths. Storage returns 412 (misconfigured service account, but not exposing data).

4. **The Aula backend is the source of truth for notifications.** The app fetches notification content via the API. FCM is just a signal that triggers the app to refresh.

5. **Token registration flow:** FCM token is obtained from Firebase SDK and sent to the Aula backend, which stores it and uses it server-side to send notifications. This is the standard server-side FCM pattern.
