# Milestone 2: AndroidManifest.xml Analysis

**APK**: com.netcompany.aulanativeprivate v2.15.4 (version code 602799)
**Compiled SDK**: 35 (Android 15)
**Min SDK**: 29 (Android 10)
**Target SDK**: 35 (Android 15)
**App Framework**: .NET MAUI / Xamarin (confirmed by Mono runtime provider and CRC-prefixed class names)

---

## 1. Launcher Activity (Main Entry Point)

The main launcher activity is:

```
crc64727613c41f254141.LauncherActivity
```

Intent filter:
- `android.intent.action.MAIN`
- `android.intent.category.LAUNCHER`

Theme: `AulaDetailsPageMaterialTheme.Launcher`

This is the app's entry point. The `crc64` prefix is characteristic of Xamarin/.NET MAUI -- these are CRC64 hashes of the .NET namespace, used as Java-side wrappers for .NET classes.

---

## 2. Activities (55 total)

All class names use Xamarin CRC64 hash prefixes (e.g., `crc64727613c41f254141`), meaning the actual .NET class names are obfuscated at the Android layer. The hash maps to a .NET namespace/class.

### 2.1 Exported Activities (with intent filters)

| Activity | Intent Filters / Notes |
|----------|----------------------|
| `crc64727613c41f254141.LauncherActivity` | MAIN/LAUNCHER -- app entry point |
| `crc649881f3fa1611df58.MainActivity` | Exported, no intent filter -- main app shell |
| `crc64d48df833906fa8fe.CloudStorageAuthInterceptor` | Deep link handler: `com.netcompany.aulanativeprivate://onedrive2redirect` and `com.netcompany.aulanativeprivate:/googleoauth2redirect` (autoVerify=true) |
| `crc64727613c41f254141.WebAuthenticationCallbackActivity` | Deep link: `https://app-private.aula.dk` (autoVerify=true, singleTop, noHistory) |
| `crc64a53fe42ad64be0cd.MoreMenuActivity` | Exported, no intent filter |
| `crc64a53fe42ad64be0cd.SettingsEditShortcutsActivity` | Exported, no intent filter |
| `crc64a53fe42ad64be0cd.SettingsOverviewActivity` | Exported, no intent filter |

### 2.2 Authentication & Login Flow

| Activity | Label / Purpose |
|----------|----------------|
| `LauncherActivity` | App entry / splash |
| `LoginActivity` | "Login skaerm" (Login screen) |
| `LoginWithPinActivity` | "Login med Pinkode skaerm" (Login with PIN) |
| `OTPSelectionActivity` | OTP selection for 2FA |
| `WebAuthenticationCallbackActivity` | OAuth callback from `app-private.aula.dk` |
| `ForceUpdateActivity` | Force update screen |
| `OnboardingActivity` | First-time user onboarding |

### 2.3 Messaging Activities

| Activity | Purpose |
|----------|---------|
| `CreateMessageActivity` | Compose new message |
| `MessageThreadActivity` | View message thread (adjustResize for keyboard) |
| `MessageFolderActivity` | Message folders |
| `AdvancedSearchActivity` | Advanced message search |
| `AutoReplyActivity` | Auto-reply settings |

### 2.4 Posts / Feed Activities

| Activity | Label / Purpose |
|----------|----------------|
| `EditPostActivity` | "Opret opslag" (Create post) |
| `ViewPostActivity` | "Opslag" (View post) |
| `PostsAllNotificationActivity` | Post notifications |
| `CommentsActivity` | Comments on posts |

### 2.5 Calendar Activities

| Activity | Purpose |
|----------|---------|
| `CalendarMenuActivity` | Calendar menu |
| `ShareCalendarActivity` | Share calendar |
| `EditLessonActivity` | Edit lesson |
| `EditMeetingActivity` | Edit meeting |
| `EventDetailsActivity` | Event details |
| `EventEditFormActivity` | Create/edit event |
| `EventTypeActivity` | Select event type |
| `RepeatTypeActivity` | Repeat settings |
| `RespondConversationMeetingActivity` | Respond to meeting |
| `RespondConversationMeetingParticipantSelectionActivity` | Select meeting participants |
| `SelectOtherPeopleActivity` | Select attendees |
| `SendEventReminderActivity` | Send event reminders |
| `ViewLessonActivity` | View lesson (singleTop) |
| `OverlappingActivity` | Handle scheduling conflicts |

### 2.6 Calendar Synchronisation

| Activity | Purpose |
|----------|---------|
| `CalendarSynchronisationCreationActivity` | Create calendar sync |
| `CalendarSynchronisationDetailedOverviewActivity` | Detailed sync overview |
| `CalendarSynchronisationOverviewActivity` | Sync overview |
| `CalendarSynchronisationActivity` | Calendar sync settings |

### 2.7 Come & Go (Attendance / Presence)

| Activity | Purpose |
|----------|---------|
| `PresenceChildrenDistributionActivity` | Children distribution |
| `ComeGoEmployeeWeekViewEditTimesActivity` | Employee week view |
| `ComeGoActivityEditFormActivity` | Edit come/go activity |
| `ComeGoAllNotificationActivity` | "Notifikationer" |
| `PlanningSubPageContainerActivity` | Planning sub-page |
| `ComeGoViewTimesActivity` | View times |
| `ComeGoDetailTimesActivity` | Detail times form |
| `ComeGoPickupResponsibleEditFormActivity` | Pickup responsible form |
| `ComeGoOpeningHoursAndClosedDaysOverviewPageActivity` | Opening hours & closed days |
| `GuardianRegisterVacationOnEmployeeRequestActivity` | Guardian vacation registration |
| `ComeGoPickUpInfoFormActivity` | Pickup info form |
| `SpareTimeActivityFormActivity` | Spare time activity form |
| `ActivityListGeneralFilteringActivity` | Activity list filtering |
| `ActivityListDepartmentAndGroupsFilteringActivity` | Filter by department/group |

### 2.8 Vacation Management

| Activity | Purpose |
|----------|---------|
| `AbsenceVacationActivity` | Absence/vacation form |
| `RequestRegisterVacationActivity` | Request vacation registration |
| `EditVacationRequestActivity` | Edit vacation request |
| `VacationDetailsActivity` | Vacation details |
| `VacationRegistrationOverviewActivity` | Vacation overview |
| `VacationRegistrationOverviewDayActivity` | Daily vacation overview |

### 2.9 Profile & Settings

| Activity | Purpose |
|----------|---------|
| `ProfileActivity` | "Profil menu" (Profile menu) |
| `MasterDataActivity` | Master data management |
| `ProfilePictureValidateAndRotateActivity` | "Profile picture" |
| `ProfileGeneralTermsActivity` | General terms |
| `ProfileDeviceSettingsActivity` | Device settings |
| `ConsentActivity` | Consent management |
| `ProfileAdditionalMasterdataActivity` | Additional master data |
| `AdditionalDataRevisionPageActivity` | Data revision |
| `ViewOtherMasterDataActivity` | View other's master data |

### 2.10 Notifications

| Activity | Purpose |
|----------|---------|
| `AllNotificationActivity` | "Notifikationer" (all notifications) |
| `ClearNotificationActivity` | Clear notifications |
| `NotificationSettingsActivity` | Notification settings |
| `NotificationSettingsEditActivity` | Edit notification settings |

### 2.11 Secure Documents

| Activity | Purpose |
|----------|---------|
| `SecureDocumentViewRevisionActivity` | View document revision |
| `SecureDocumentsOverviewSelectModeActivity` | Document overview / select |
| `InternalSecureDocumentFormActivity` | Internal document form |
| `SecureDocumentShareWithFormActivity` | Share document |
| `SecureDocumentDetailsActivity` | Document details |

### 2.12 Albums / Gallery / Media

| Activity | Purpose |
|----------|---------|
| `AlbumDetailsActivity` | Album details |
| `AlbumSelectedActivity` | Album selection |
| `MediaTaggingPageActivity` | Media tagging |
| `RecipientsViewActivity` | Recipients view |
| `SelectUserToTagActivity` | Select user to tag |
| `AlbumEditFormActivity` | Edit album |
| `FilePickerActivity` | File picker |
| `GalleryPickerActivity` | Gallery picker |
| `ImportMediaContainerActivity` | Import media |
| `MediaOverviewActivity` | Media overview |
| `FullscreenMediaPlayerActivity` | Fullscreen media player |
| `MediaTagActivity` | Media tag |
| `LinkRenderingMediaPlayerActivity` | Link rendering media player |
| `MediaPickerActivity` | Media picker |
| `FileViewActivity` | File viewer |

### 2.13 Cloud Storage & Reporting

| Activity | Purpose |
|----------|---------|
| `CloudIntegrationActivity` | Cloud storage integration (OneDrive, Google) |
| `ReportActivity` | Report form |

### 2.14 Groups

| Activity | Purpose |
|----------|---------|
| `GroupDashboardActivity` | Group dashboard |
| `GroupMembershipActivity` | Group membership |

### 2.15 Other

| Activity | Purpose |
|----------|---------|
| `GenericView` | Generic view container |
| `AulaPortalWebViewActivity` | WebView for Aula portal |
| `AulaSelectionActivity` | Selection UI |
| `MultipleAulaSelectionActivity` | Multiple selection UI |
| `SelectionPageWithAddMoreOptionActivity` | Selection with "add more" |
| `AutoCompleteControlSelectionPageActivity` | Autocomplete selection |
| `GlobalSearchActivity` | "Global soegning" (Global search) |
| `TestingActivity` | Testing/debug activity |
| `RequestPermissionActivity` | Permission request dialog |
| `AulaBaseAppCompatActivity` | Base activity class (no config) |
| `AulaBaseFragmentActivity` | Base fragment activity |
| `WebAuthenticatorIntermediateActivity` | Web auth intermediate |
| `IntermediateActivity` | MAUI Essentials intermediate |
| `GoogleApiActivity` | Google API activity (GMS) |

---

## 3. Services (5 total)

| Service | Exported | Purpose |
|---------|----------|---------|
| `crc645d81b9b77a3a8305.AulaFirebaseMessagingService` | No | Handles Firebase Cloud Messaging (push notifications). Filters `com.google.firebase.MESSAGING_EVENT` |
| `crc64396a3fe5f8138e3f.KeepAliveService` | No | Background keep-alive service (likely for maintaining push notification connection) |
| `com.google.firebase.messaging.FirebaseMessagingService` | No | Default Firebase messaging handler (priority -500, lower than Aula's custom one) |
| `com.google.firebase.components.ComponentDiscoveryService` | No | Firebase component discovery. Registers: FirebaseCommonKtx, DataTransport, FirebaseInstallations, FirebaseMessaging |
| `com.google.android.datatransport.runtime.scheduling.jobscheduling.JobInfoSchedulerService` | No | Google data transport scheduler (requires BIND_JOB_SERVICE) |
| `com.google.android.datatransport.runtime.backends.TransportBackendDiscovery` | No | Data transport backend discovery (CCT backend) |

---

## 4. Broadcast Receivers (7 total)

| Receiver | Exported | Purpose |
|----------|----------|---------|
| `com.google.firebase.iid.FirebaseInstanceIdInternalReceiver` | No | Internal Firebase Instance ID |
| `com.google.firebase.iid.FirebaseInstanceIdReceiver` | Yes | C2DM receive & registration (permission: `com.google.android.c2dm.permission.SEND`) |
| `crc64e53d2f592022988e.ConnectivityBroadcastReceiver` | No | MAUI Essentials: network connectivity changes |
| `crc640a8d9a12ddbf2cf2.BatteryBroadcastReceiver` | No | MAUI Essentials: battery status changes |
| `crc640a8d9a12ddbf2cf2.EnergySaverBroadcastReceiver` | No | MAUI Essentials: energy saver mode changes |
| `androidx.profileinstaller.ProfileInstallReceiver` | Yes | AndroidX profile installer (DUMP permission). Actions: INSTALL_PROFILE, SKIP_FILE, SAVE_PROFILE, BENCHMARK_OPERATION |
| `com.google.android.datatransport.runtime.scheduling.jobscheduling.AlarmManagerSchedulerBroadcastReceiver` | No | Google data transport alarm scheduling |

---

## 5. Content Providers (5 total)

| Provider | Authorities | Purpose |
|----------|-------------|---------|
| `androidx.core.content.FileProvider` | `com.netcompany.aulanativeprivate` | Core file sharing (paths from `@xml/file_path`) |
| `microsoft.maui.essentials.fileProvider` | `com.netcompany.aulanativeprivate.fileProvider` | MAUI Essentials file provider (paths from `@xml/microsoft_maui_essentials_fileprovider_file_paths`) |
| `mono.MonoRuntimeProvider` | `com.netcompany.aulanativeprivate.mono.MonoRuntimeProvider.__mono_init__` | Mono/.NET runtime initialization (initOrder=1999999999 -- very high priority) |
| `androidx.startup.InitializationProvider` | `com.netcompany.aulanativeprivate.androidx-startup` | AndroidX startup: ProfileInstaller, ProcessLifecycle, EmojiCompat |
| `com.google.firebase.provider.FirebaseInitProvider` | `com.netcompany.aulanativeprivate.firebaseinitprovider` | Firebase initialization (directBootAware, initOrder=100) |

---

## 6. Permissions

### 6.1 Requested Permissions (14 total)

| Permission | Category | Purpose |
|------------|----------|---------|
| `android.permission.INTERNET` | Network | Internet access |
| `android.permission.ACCESS_NETWORK_STATE` | Network | Check network connectivity |
| `android.permission.CAMERA` | Hardware | Camera for photos/media |
| `android.permission.READ_EXTERNAL_STORAGE` | Storage | Read files (deprecated in API 33+) |
| `android.permission.WRITE_EXTERNAL_STORAGE` | Storage | Write files (deprecated in API 33+) |
| `android.permission.USE_FINGERPRINT` | Biometric | Fingerprint auth (deprecated, replaced by USE_BIOMETRIC) |
| `android.permission.USE_BIOMETRIC` | Biometric | Biometric authentication |
| `android.permission.CALL_PHONE` | Phone | Initiate phone calls |
| `android.permission.POST_NOTIFICATIONS` | Notifications | Show notifications (Android 13+) |
| `android.permission.WAKE_LOCK` | System | Keep CPU awake for background tasks |
| `android.permission.SYSTEM_ALERT_WINDOW` | System | Draw over other apps |
| `com.google.android.c2dm.permission.RECEIVE` | Firebase | Receive push notifications via FCM |
| `com.samsung.android.providers.context.permission.WRITE_USE_APP_FEATURE_SURVEY` | Samsung | Samsung-specific usage tracking |
| `com.netcompany.aulanativeprivate.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` | Internal | Protect dynamically registered receivers |

### 6.2 Declared Permissions (1)

| Permission | Protection Level | Purpose |
|------------|-----------------|---------|
| `com.netcompany.aulanativeprivate.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` | signature | Signature-level protection for dynamic receivers |

### 6.3 Hardware Features (optional)

| Feature | Required |
|---------|----------|
| `android.hardware.screen.portrait` | No |
| `android.hardware.telephony` | No |
| `android.hardware.camera` | No |
| `android.hardware.camera.autofocus` | No |

All hardware features are declared as optional (`required=false`), meaning the app can be installed on devices without these features.

---

## 7. Deep Links & URL Handling

### 7.1 App Links (autoVerify=true)

1. **WebAuthenticationCallbackActivity**: `https://app-private.aula.dk` -- OAuth/authentication callback
2. **CloudStorageAuthInterceptor**: `com.netcompany.aulanativeprivate://onedrive2redirect` and `com.netcompany.aulanativeprivate:/googleoauth2redirect` -- Cloud storage OAuth redirects

### 7.2 Queried Intents

The app queries for the following external capabilities:
- Custom Tabs support (`android.support.customtabs.action.CustomTabsService`)
- HTTPS browsing capability
- HTTPS links to `*.ncaula.com` -- internal/staging Aula domain
- HTTPS links to `*.aula.dk` -- production Aula domain
- Google Docs (`com.google.android.apps.docs.editors.docs`)
- YouTube (`com.google.android.youtube`)

---

## 8. Application Configuration

| Property | Value |
|----------|-------|
| Application class | `crc649881f3fa1611df58.MainApplication` |
| Label | "Aula" |
| Icon | `@mipmap/ic_blue` / `@mipmap/ic_blue_round` |
| Backup allowed | false |
| Clear user data allowed | false |
| Large heap | true |
| Extract native libs | false |
| App component factory | `androidx.core.app.CoreComponentFactory` |
| Uses library | `org.apache.http.legacy` (optional) |

---

## 9. .NET MAUI / Xamarin-Specific Observations

1. **CRC64 class naming**: All app-specific classes use `crc64XXXX.ClassName` format. The CRC64 hash maps to a .NET namespace. The actual class structure is inside the .NET assemblies (libassemblies blob).

2. **Mono Runtime Provider**: `mono.MonoRuntimeProvider` initializes the .NET runtime at the highest possible priority (initOrder=1999999999), ensuring the .NET runtime starts before any other component.

3. **MAUI Essentials providers**: The app uses `microsoft.maui.essentials.fileProvider` alongside standard AndroidX FileProvider, indicating MAUI Essentials integration for cross-platform APIs (connectivity, battery, energy saver).

4. **Hybrid architecture**: While the core app is .NET MAUI, it uses standard Android components:
   - Firebase Cloud Messaging for push notifications
   - AndroidX startup for initialization
   - Google Play Services
   - Custom Tabs for external browsing

5. **Known CRC64 hash groups** (same prefix = same .NET namespace):
   - `crc64727613c41f254141`: Auth/launch activities (Launcher, Login, LoginWithPin, OTP, ForceUpdate, WebAuth)
   - `crc64a53fe42ad64be0cd`: Settings/menu (MoreMenu, SettingsEditShortcuts, SettingsOverview)
   - `crc647e759e71f16a7378`: Calendar activities (12 activities)
   - `crc64f48a125ae4e10cda`: Album activities (5 activities)
   - `crc640d97cec38b77325f`: Album edit/import (4 activities)
   - `crc6486715a2de441f74b`: Notification activities (3 activities)
   - `crc648d07b834a19501a6`: Master data / profile picture (2 activities)
   - `crc64df89d32432429ade`: Calendar sync (3 activities)
   - `crc648e6078e8b9fed96c`: Vacation (3 activities)
   - `crc6456b0136d0ce34f0b`: Selection UI (2 activities)
   - `crc64c2bf81c26ee7219c`: File/media views (2 activities)
   - `crc640a8d9a12ddbf2cf2`: MAUI Essentials receivers (Battery, EnergySaver)
   - `crc64db7f9aea8cfbb131`: Secure document forms (2 activities)
   - `crc64def7a5631a95e5c8`: Come/go forms (2 activities)

---

## 10. Config APK Manifests

The 3 config APKs contain minimal manifests:
- **config.en**: Language split (English resources), no code
- **config.mdpi**: Density split (medium DPI resources), no code
- **config.x86_64**: ABI split (x86_64 native libraries), no code

All config APKs have `hasCode=false` -- they contain only resources or native libraries.

---

## 11. Key Domains

| Domain | Usage |
|--------|-------|
| `app-private.aula.dk` | OAuth callback / authentication |
| `*.aula.dk` | Production Aula platform |
| `*.ncaula.com` | Internal / staging Aula domain (Netcompany) |
| `play.google.com` | App distribution stamp |

---

## 12. Security-Relevant Notes

- **No backup**: `allowBackup=false` prevents ADB backup of app data
- **No clear user data**: `allowClearUserData=false` prevents clearing via settings
- **Signature-protected receiver**: Dynamic receiver permission uses signature protection level
- **Biometric auth**: Both legacy fingerprint and modern biometric APIs
- **SYSTEM_ALERT_WINDOW**: Can draw overlays -- potentially for in-app popups
- **Samsung tracking permission**: Integrates with Samsung's app usage survey system
- **OAuth flows**: Uses both web-based auth (app-private.aula.dk) and custom scheme redirects for cloud storage (OneDrive, Google)
