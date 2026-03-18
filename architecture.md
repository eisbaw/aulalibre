# Aula Native App Architecture

Analysis of the decompiled Aula Android app (com.netcompany.aulanativeprivate, v2.15.4).
Built by Netcompany A/S as a Xamarin.Android application (not .NET MAUI despite some MAUI namespace references).

## Technology Stack

| Component | Technology |
|-----------|-----------|
| Runtime | Mono/.NET (libmonodroid.so, libmonosgen-2.0.so) |
| UI Framework | Xamarin.Android (native Activities/Fragments, not Xamarin.Forms) |
| DI Container | Unity Container + CommonServiceLocator |
| HTTP | System.Net.Http.HttpClient with certificate pinning |
| Auth | OIDC via IdentityModel.OidcClient (login.aula.dk) |
| JSON | Newtonsoft.Json |
| Object Mapping | AutoMapper |
| Local Storage | SQLite (via custom SQLiteHelper) |
| Localization | I18NPortable (Danish locale, dk.json) |
| Push Notifications | Firebase Cloud Messaging (FCM) |
| Encryption | Custom EncryptionManager + SecureStorage |
| Platform Compat | Microsoft.Maui.Essentials (partial migration; UI remains Xamarin.Android) |

## Assembly Structure

Two main assemblies extracted from the APK:

- **AulaNative** (assembly_187) -- shared/portable code: ViewModels, Services, Models, DTOs, Mappers, DomainServices, Interfaces, Utils, SQLiteData, OAuth, Configuration, Plugins
- **AulaNative.Droid** (assembly_0) -- Android platform-specific: Activities, Fragments, CrossPlatformServices, Renderers, Utils

## Dependency Injection

### Container Setup

The app uses **Unity Container** with **CommonServiceLocator** as the service resolution pattern. Initialization happens in `MainApplication.Init()`:

```
MainApplication.Init()
  -> AndroidDependencyInjectionConfig.Init()
       -> new UnityContainer()
       -> Register 16 Android-specific services (transient + singletons)
       -> CommonDependencyInjectionConfig.Init(container)
            -> RegisterLocalStorageAndCachingManagers()  (10 singletons)
            -> RegisterServiceAndDataManagers()          (9 registrations)
            -> RegisterViewModels()                      (9 ComeGo-related VMs)
       -> ServiceLocator.SetLocatorProvider(AulaServiceLocator)
```

### Registration Categories

**Android Platform Services** (registered in `AndroidDependencyInjectionConfig`):
- `ICrossPlatformDialogService` -> `AndroidDialogService`
- `ICrossPlatformDynamicTextDialogService` -> `AndroidCountDownDialogService`
- `IDeviceInfo` -> `DeviceInfoImplementation`
- `ICookieCrossService` -> `AndroidCookieService`
- `IConfigureDeviceService` -> `DroidConfigureDeviceService`
- `ICrossPlatformUIThreadService` -> `AndroidUIThreadService`
- `ICrossPlatformKeyboardService` -> `AndroidKeyboardService`
- `ICrossPlatformLogOutService` -> `AndroidLogOutService`
- `IUniversalLinkOpener` -> `AndroidUniversalLinkOpener`
- `ICrossPlatformOptionListBottomSheetService` -> `AndroidOptionListBottomSheetService`
- `ISimpleInfoPage` -> `DroidSimpleInfoPageService`
- `ISecureStorageHelper` -> `DroidSecureStorageHelper`
- `IBadgeUpdate` -> `DroidBadgeUpdateService`
- `ICrossPlatformNotificationPermissionService` -> `AndroidNotificationPermissionService`
- `ICrossPlatformNativeUrlLauncherService` -> `AndroidNativeUrlLauncherService`
- `ImageCachingManager` -> `AndroidImageCachingManager` (singleton)
- `ImageCachingManager["large"]` -> `AndroidLargeImageCachingManager` (named singleton)
- `IEmailNotificationsHandler` -> `AndroidEmailNotificationsHandler` (singleton)

**Shared Services** (registered in `CommonDependencyInjectionConfig`):
- SQLite managers (10 singletons): AppSettings, DeviceSettings, Logging, CachingSettings, ImageCaching, PostDraft, MessageDraft, MessageThreadDraft, DelegatedContext, LoginSettings
- Service managers (singletons): FileServiceManager, NotificationServiceManager, ComeGoServiceManager, NotificationDataManager, RemoteNotificationServiceManager, MasterDataServiceManager, ComeGoConfigurationDataManager, MessageFolderDataManager
- `IDateTimeProvider` -> `SystemDateTimeProvider` (transient)

### Service Resolution

Services are resolved via `ServiceLocator.Current.GetInstance<T>()` throughout the codebase. This is the Xamarin-era pattern (pre-DI-injection via constructors). Most classes manually request their dependencies from the ServiceLocator rather than receiving them via constructor injection.

## Layered Architecture

```
+----------------------------------------------------------+
|  Activities & Fragments (AulaNative.Droid)                |
|  - UI rendering, Android lifecycle                       |
|  - 453 Activity/Fragment files across 40+ feature areas  |
+----------------------------------------------------------+
          |                    |
          v                    v
+---------------------+  +---------------------------+
| ViewModels           |  | MessagingCenter           |
| (AulaNative)         |  | (Event bus for cross-     |
| - 520 ViewModel files|  |  component communication) |
| - BaseViewModel base |  | - 24 domain-specific      |
| - Validation support |  |   messaging centers       |
+---------------------+  +---------------------------+
          |
          v
+----------------------------------------------------------+
|  ServiceManagers (AulaNative.ServiceManagers)            |
|  - ~30 managers wrapping web services                    |
|  - GenericServiceManager<T> base class                   |
|  - Error handling via ServiceHandlerManager              |
+----------------------------------------------------------+
          |
          v
+----------------------------------------------------------+
|  Web Services (AulaNative.Services.Web)                  |
|  - ~31 service classes extending SimpleService            |
|  - HTTP calls to Aula backend API                        |
|  - JSON serialization via Newtonsoft.Json                 |
+----------------------------------------------------------+
          |
          v
+----------------------------------------------------------+
|  HTTP Infrastructure (AulaNative.Services.WebUtils)      |
|  - SimpleService base: GET/POST/PUT/DELETE helpers       |
|  - HttpClientManager (singleton): CSRF tokens, auth      |
|  - CertificatePinningUtils: TLS pin validation           |
|  - RequestCachingService: response caching               |
+----------------------------------------------------------+
          |
          v
+----------------------------------------------------------+
|  Models & DTOs                                           |
|  - AulaNative.Models.* (~480 domain model classes)        |
|  - AulaNative.DTOs.* (transfer objects for API)          |
|  - AutoMapper profiles for DTO <-> Model mapping         |
+----------------------------------------------------------+
          |
          v
+----------------------------------------------------------+
|  Local Storage (AulaNative.SQLiteData)                   |
|  - SQLite database (auladb_13_05_2019)                   |
|  - SqliteConnectionManager (singleton)                   |
|  - Managers: drafts, settings, caching, logging          |
|  - Encrypted storage via EncryptionManager               |
+----------------------------------------------------------+
```

### DomainService Layer

Between ServiceManagers and ViewModels sits a `DomainService` layer for complex business operations:
- `MessagesDomainService`, `MessageThreadDomainService`, `MailboxIdentityDomainService`
- `CalendarDomainService` (in DomainService.Calendar)
- `ComeGoDomainService` (in DomainService.ComeGo)
- `GalleryDomainService`, `PostsDomainService`, `SearchDomainService`
- `AttachmentDomainService`, `SecureDocumentDomainService`

These orchestrate multiple ServiceManagers and encapsulate business logic that spans multiple services.

### DataManager Layer

Alongside ServiceManagers, DataManagers handle cached/local data:
- `NotificationDataManager` -- notification state management
- `MessageFolderDataManager` -- message folder state
- `ComeGoConfigurationDataManager` -- presence/attendance configuration
- `DataManager.Calendar`, `DataManager.Message` -- calendar/message data caching

## Navigation Architecture

### App Startup Flow

```
MainApplication (Android Application class)
  -> Init()
       -> LocalesService.Initialize()
       -> DeviceManager.Instance.Platform = Android
       -> SessionPromptManager.StartCountingThreadOnAppFirstLaunching()
       -> AndroidDependencyInjectionConfig.Init()
       -> AutoMapperConfig.Initialize()
       -> SQLiteHelper.InitializeSqlConnection()
       -> CertificatePinningUtils.Init()
       -> MigrationsHelper.PerformMigrations()
  -> OnCreate()
       -> RegisterActivityLifecycleCallbacks()
       -> ProcessLifecycleOwner lifecycle observer
```

### Activity Hierarchy

```
AppCompatActivity (AndroidX)
  -> AulaBaseAppCompatActivity
       -> AulaBaseFragmentActivity
            -> AulaBaseAbstractPushNotificationNavigatorActivity
                 -> MainActivity (main app shell, tab-based)
       -> AulaBaseAppCompatActivity<T1> (generic data-passing variant)
```

Login flow Activities:
- `LauncherActivity` -- app entry point
- `LoginActivity` -- OIDC web-based login
- `LoginWithPinActivity` -- PIN-based session resumption
- `ForceUpdateActivity` -- mandatory app update screen
- `OnboardingActivity` -- first-time user onboarding
- `OTPSelectionActivity` -- MitID/NemID OTP selection
- `WebAuthenticationCallbackActivity` -- OAuth callback handler

### Main App Navigation

The `MainActivity` uses a **ViewPager** with a bottom tab bar. Tab items are dynamically configured based on the user's profile and module permissions from the server.

Bottom bar supports 4 items on phones, 5 on tablets. Modules are server-driven via `Profile.PageConfiguration.ModuleConfigurations` and `WidgetConfigurations`.

Key feature areas (each with their own Activity/Fragment hierarchy):
- Overview (dashboard/feed)
- Messages (threads, folders, auto-reply)
- Calendar (events, birthdays, important dates, vacation)
- ComeGo / Presence (attendance tracking, check-in/out)
- Gallery / Albums
- Documents / Secure Documents / Cloud Storage
- Groups
- Contacts List
- Notifications
- Profile / MasterData
- Settings
- Global Search

### Navigation Patterns

1. **Activity-to-Activity**: Via `Intent` with static data passing (`AulaBaseAppCompatActivity<T1>.NewIntent()` stores data in a static field, read in `OnCreate`)
2. **Fragment transactions**: Within Activities for sub-navigation
3. **MessagingCenter**: Event-bus pattern for cross-component communication (24 domain-specific messaging centers covering login, posts, notifications, galleries, documents, etc.)
4. **Push notification deep-linking**: `AulaBaseAbstractPushNotificationNavigatorActivity` handles routing from push notifications to specific screens

## Singleton Managers

The app uses many singleton managers (manual singleton pattern, not DI-scoped):

| Manager | Responsibility |
|---------|---------------|
| `ProfileManager.Instance` | Current user profile, children, institutions, portal role |
| `HttpClientManager.Instance` | HTTP client with auth tokens, CSRF management |
| `DeviceManager.Instance` | Platform identification, device info |
| `SessionPromptManager.Instance` | Session timeout tracking, re-auth prompts |
| `FileManager.Instance` | File operations and temp directory management |
| `PermissionManager.Instance` | Android permission management |
| `GroupManager.Instance` | Group data caching |
| `EncryptionManager.Instance` | Data encryption for local storage |
| `DelegateAccessManager.Instance` | Delegate/proxy access management |
| `ViewModelUtils.Instance` | Shared VM utilities (file upload, profile data) |
| `SqliteConnectionManager.Instance` | SQLite database connections |

On logout (`ProfileManager.ResetProfileDataOnDevice()`), all singletons are explicitly reset via `MakeNewInstance()` or `NewInstance()` calls.

## Authentication

### OIDC Flow

- Uses `IdentityModel.OidcClient` library
- Authority: `login.aula.dk` (production)
- Two scope levels:
  - Level 2 (standard): scope `aula`, clientId `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6`
  - Level 3 (sensitive): scope `aula-sensitive`, clientId `_99949a54b8b65423862aac1bf629599ed64231607a`
- Token refresh via `RefreshTokenResult`
- 60-minute session timeout, warning at 55 minutes
- PIN-based session resumption (`LoginWithPinActivity`)

### CSRF Protection

- CSRF token managed by `HttpClientManager`
- Token stored as HTTP header on `HttpClient.DefaultRequestHeaders`
- Thread-safe access via `CsrfpHttpHeaderSemaphore`

### Certificate Pinning

Public key pins configured per domain:
- `aula.dk`: 3 pinned keys
- `ncaula.com` (test environments): 3 pinned keys

## Configuration & Environments

Production URL: `https://www.aula.dk/api/v23/`
Auth URL: `https://login.aula.dk/`
Data host: `app-private.aula.dk`

13 configured environments:
- **Production**: PROD, PREPROD, HOTFIX (isProduction=true)
- **Test/Dev**: TEST1, TEST3, DEV1, DEV3, DEV11, DEV21, DEV22, DEV31, DEV32, CI

EndOfSupportDate is set to June 15, 2025 in `Conf.cs`, indicating a planned end-of-support milestone for this app version.

Test environments use basic auth fallback: `aula-user` / `Aula-1337`

API version: 23

## Data Flow Example: Message Thread

```
User taps message thread in UI
  -> MessageThreadListFragment (AulaNative.Droid)
       -> MessageThreadListViewModel.LoadThreads()
            -> MessageServiceManager.GetThreadList()
                 -> GenericServiceManager<MessageService>.HandleResponse()
                      -> MessageService.GetThreadList() [extends SimpleService]
                           -> SimpleService.PostAsync() / GetAsync()
                                -> HttpClientManager.Instance.HttpClient
                                     -> POST https://www.aula.dk/api/v23/?method=messaging.getThreads
  <- AulaServiceResponse<ThreadListResult>
       <- ServiceHandlerManager.HandleErrors() (error handling chain)
            <- AutoMapper: DTO -> ViewModel mapping
                 <- UI update via Fragment/Adapter refresh
```

## API Style: RPC-over-HTTP (not REST)

The Aula backend API is **not a traditional REST API**. Instead of using resource-oriented URLs with HTTP verbs, it uses a single endpoint with a `method` query parameter to dispatch operations:

```
POST https://www.aula.dk/api/v23/?method=messaging.getThreads
POST https://www.aula.dk/api/v23/?method=messaging.sendMessage
```

All calls go through the same base URL; the `method` parameter selects the server-side procedure. This is an RPC-over-HTTP pattern. `SimpleService` base class provides `PostAsync()` / `GetAsync()` helpers, but the URL routing is method-based, not resource-based. The HTTP verb (GET vs POST) is chosen per-call but does not carry REST semantics.

## Cross-Platform Abstractions

The `AulaNative.Services.CrossPlatform` namespace defines 16 platform-agnostic interfaces, each implemented by Android-specific classes in `AulaNative.Droid.CrossPlatformServices`. This pattern suggests the codebase was designed to also target iOS (and likely has an iOS counterpart app), though only the Android implementation is present in this APK.

Key cross-platform abstractions:
- Dialog services (alerts, countdown dialogs, bottom sheets)
- Device info and configuration
- Cookie management
- Keyboard control
- Logout/cleanup
- Secure storage
- Badge updates
- Notification permissions
- URL launching and universal links
- Image caching (with size variants)

## Mapper Configuration

AutoMapper is initialized at app startup with 9 mapping profiles:
- ProfileMapper, RecipientMapper, MessagesMapper, EventMapper
- InstitutionMapper, NotificationsMapper, GlobalSearchMapper
- ConsentMapper, GalleryMapper

These map between backend DTOs and internal model/ViewModel classes.

## Validation Framework

The app has a custom validation framework in `AulaNative.Utils.ValidationPattern`:
- `BaseViewModel` includes `ValidateViewModel()` and `ValidateViewModelHeavy()` methods
- `ValidatorManager` processes validation rules and shows error popups
- `ControlValidationManager` tracks per-control validation state
- Validation is two-phase: lightweight (sync) and heavy (async, e.g., server-side checks)

## Key Architectural Observations

1. **Not MVVM in the traditional sense**: While there are ViewModels, they don't use data binding (no INotifyPropertyChanged, no commands). Activities/Fragments manually read ViewModel properties and call methods. This is more of a "ViewModel as presentation model" pattern.

2. **Heavy use of ServiceLocator anti-pattern**: The app uses a mix of `ServiceLocator.Current.GetInstance<T>()` resolution and direct singleton `.Instance` access (e.g., `ProfileManager.Instance`, `HttpClientManager.Instance`) rather than constructor injection. This makes the dependency graph implicit and testing harder.

3. **Manual singleton management**: 11+ singleton managers with manual `Instance` properties and explicit reset-on-logout. This creates tight coupling and makes lifecycle management error-prone.

4. **Event bus for decoupling**: 24 domain-specific MessagingCenter classes provide pub/sub between components. This avoids direct references but can make control flow hard to trace.

5. **Static data passing between Activities**: `AulaBaseAppCompatActivity<T1>` uses a static field to pass data between Activities, which is fragile (can be lost on process death).

6. **Server-driven module system**: The bottom tab bar and available features are dynamically configured based on the user's profile from the server (`ModuleConfigurations`, `WidgetConfigurations`). This allows Aula to control feature rollout per-institution.

7. **Dual app variant**: Constants like `IsStaffApp = false` and separate data hosts for private vs staff apps indicate there is a parallel "Aula for Medarbejdere" (staff) app sharing much of this codebase.

8. **Certificate pinning**: Production and test environments have separate pinned public keys for TLS certificate validation, providing defense against MITM attacks.
