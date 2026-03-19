# MessagingCenter Event Bus Analysis

Analysis of the Xamarin.Forms MessagingCenter-based internal event bus in the Aula Android app. The app implements a custom `MessagingCenter` (re-implementation of Xamarin's) with 23 domain-specific static wrapper classes that provide type-safe publish/subscribe semantics.

## Architecture Overview

The core `MessagingCenter` class (`AulaNative.Utils.MessagingCenter.MessagingCenter`) is a singleton implementing `IMessagingCenter`. It uses a dictionary of `(message, senderType, argType)` tuples to subscriptions, with weak references for automatic cleanup.

Each domain wrapper class follows a consistent pattern:
- Static methods: `Subscribe*()`, `Unsubscribe*()`, `Notify*()` / `Send*()`
- Private string constants for message keys
- Nested argument/payload classes and enums

The general data flow is: **ServiceManagers publish** (after API calls complete) and **Fragments/ViewModels subscribe** (to refresh their UI).

## Message Catalog

### 1. Authentication & Session Domain

#### LoginMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `LogoutNeededKey` | `bool` (performUnregisterDevice) | `ProfileDeviceSettingsViewModel`, `AuthenticationManager` | `MainActivity` |
| `LogoutNeededIfPinCodeViewKey` | `bool` (performUnregisterDevice) | `InvalidAccessTokenError` | `MainActivity`, `LoginWithPinActivity` |
| `LoginScreenEncounteredKey` | none | `LoginActivity` | `MainActivity` |
| `OpenPinViewKey` | `bool` | (no publishers found in Android) | (no subscribers found outside definition) |
| `ForceUpdateKey` | `bool` | `ServiceHandlerManager`, `ConfigurationServiceManager` | `ProfileUtils` (wraps activity) |
| `ForceLogOutNeededKey` | `bool` (performUnregisterDevice, default true) | (no publishers found in Android) | (no subscribers found in Android) |

#### LoginFinishedMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `LoginFinished` | `bool` (isLogged) | `LoginWithPinActivity` (7 call sites), `MainActivity` | `AndroidEmailNotificationsHandler` (via `IsLoginAndInitialRenderingFinishedAsync`) |

This center has an unusual async/await pattern: `IsLoginAndInitialRenderingFinishedAsync` subscribes, waits for the message via a `TaskCompletionSource`, then unsubscribes. A static `Finished` flag short-circuits if login already completed.

### 2. Messaging / Threads Domain

#### SubscriptionMessagingCenter
The most heavily used messaging center -- 40+ call sites.

| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `ThreadListChange` | `int?` (responseThreadId) | `AdvancedSearchActivity`, `MessageFormViewModel` (3 sites), `MessageServiceManager` | `MessagesOverviewFragment` |
| `ThreadChange` | `ThreadChangeMessagingCenterArguments` | `SQLiteMessageDraftManager` (3 sites), `MessageServiceManager` (6 sites), `MessageViewModel`, `MessagesListViewModel` (2 sites), `ThreadSubscriptionInfoViewModel` (10 sites), `MessageOverviewPageViewModel` | `MessageThreadFragment`, `MessagesOverviewFragment` |
| `ThreadDeleteKey` | `DeleteThreadArguments` | `MessageServiceManager` | `GlobalSearchActivity`, `MessagesOverviewFragment` |
| `MoveThreadFolderKey` | `DeleteThreadArguments` | `FolderServiceManager` (2 sites) | `MessagesOverviewFragment` |

`ThreadChangeMessagingCenterArguments` carries: `ThreadId`, `ThreadIds`, `ShouldRemoteRefresh`, `MessageDraftUpdated`, `LastestMessage` [sic], `BundleId`, `SingleMessageId`. Factory methods distinguish update types: `FromDraftUpdate`, `FromRemoteUpdate`, `FromRemoteUpdateMultiple`, `FromMessageUpdate`.

### 3. Posts Domain

#### PostMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `PostChangeKey` | `PostChangeArguments` (PostId, PostChangeType: Create/Update/Delete) | `PostServiceManager` (13 call sites) | `PostListViewWithFilterFragment` |

#### CommentMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `CommentChangeKey` | `CommentChangeArguments` (Type: Create/Update/Delete/UnreadCount, ParentType) | `CommentServiceManager` (6 sites) | `PostListViewWithFilterFragment` |
| `UnreadCommentCountChange` | `CommentChangeArguments` | `CommentPagedListViewModel` | `PostListViewWithFilterFragment` |

### 4. Notifications Domain

#### NotificationMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `NotificationChangeKey` | `NotificationChangeArguments` (NotificationIds[], Type: Create/Update/Delete, NotificationType, NotificationArea) | `AulaFirebaseMessagingService`, `MessagesOverviewFragment`, `CalendarFragment`, `SecureDocumentOverviewPageViewModel` (4 sites), `NotificationServiceManager` (4 sites), `SharedNotificationDetailViewHelper` | `NotificationDataManager`, `SecureDocumentBaseFragment`, `AulaNotificationHandler` |

#### PushNotificationMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `PushNotificationKey` | `PushNotificationArguments` (RemoteNotification Data) | (no publishers found -- likely iOS only) | (no subscribers found) |
| `PushNotificationAppIsOpenKey` | `PushNotificationArguments` | `AulaFirebaseMessagingService` | `PostListViewWithFilterFragment` |

### 5. Gallery & Media Domain

#### GalleryMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `GalleryChangeKey` | `BaseGalleryChangeArguments` (polymorphic: Delete, TagChange, TagChangeMultiple, Create) | `GalleryServiceManager` (4 sites), `MediaTaggingPageViewModel` (2 sites), `ImportMediaViewModel` (2 sites) | `MediaFilesPlayerOverviewPageViewModel`, `MessageThreadFragment`, `AlbumDetailsFragment`, `GalleryFragment` |

#### AlbumMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `AlbumChangeKey` | `AlbumChangeArguments` (AlbumId, DocumentChangeType) | `GalleryServiceManager` (6 sites) | `AlbumDetailsFragment`, `GalleryFragment` |

#### GalleryImageChangedMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `ImageChangedKey` | `int` (position) | `MediaOverviewActivity` | `MediaOverviewAdapter` |

### 6. Documents Domain

#### DocumentMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `DocumentChangeKey` | `DocumentChangeArguments` (DocumentIds[], Type: Create/Update/Delete) | `DocumentServiceManager` (10 sites), `SecureDocumentDetailsActivity` | `SecureDocumentDetailsActivity`, `SecureDocumentBaseFragment` |

### 7. UI / Navigation Domain

#### TabChangeMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `TabChangeKey` | none | `AulaMainPageBottomNavigationView`, `AulaBottomBarEditShortcutsView` | (no subscribers found in Android) |
| `TabBarOrderChangeKey` | none | `SettingsEditShortcutsActivity` (3 sites) | `MainActivity` |
| `TabBarOrderUpdatedInMainViewChangeKey` | none | (never used) | (never used) |

#### ProfileBarMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `OnProfileImageChanged` | none | `MasterDataFragment`, `ProfileActivity`, `MasterDataActivity`, `MasterDataViewModel` (2 sites) | `AbstractProfileBar` |

#### FilterProfileBarMessagingCenter
The most widely subscribed-to messaging center by number of unique subscribers.

| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `OnProfileFilterChanged` | none | `AbstractProfileBar` | `PlanningPageFragment`, `ContactListOverviewFragment`, `ComeGoGuardianOverviewFragment`, `ComeGoDailyOverviewTabFragment`, `VacationRegistrationOverviewFragment`, `SecureDocumentOverviewFilteringFragment`, `PostListViewWithFilterFragment`, `SecureDocumentBaseFragment`, `ActivityListOverviewFragment`, `WeekOverviewFragment`, `CalendarLandscapeItemFragment`, `ComeGoOpeningHoursAndClosedDaysInstitutionListFragment`, `VacationListOverviewFragment`, `GalleryFragment`, `MessagesOverviewFragment`, `CommonFilesFragment`, `CalendarMainViewModel`, `ActivityListPageViewModel` |

#### FilterProfileBarForNotificationHandlerMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `FilterProfileBarForNotificationHandlerMessagingCenter` | none | `AbstractProfileBar` | `WidgetWebViewFragment`, `AulaNotificationHandler` |

#### GenericViewMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `ViewClosed` | none | `GenericView` | (no subscribers found in Android) |

### 8. Institution / Context Domain

#### InstitutionMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `InstitutionChangeKey` | `string` (institutionCode) | `EditPostViewModel`, `EditConversationMeetingViewModel`, `EventEditFormViewModel` (2 sites), `AlbumViewModel` | `AutoCompleteControlSelectionPageViewModel` |

### 9. ComeGo / Attendance Domain

#### ActivityListMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `ActivityListChangeKey` | `ActivityListChangeArgument` (IsInEditMode) | `ActivityListOverviewFragment` (2 sites: enter/exit edit mode) | `ComeGoStaffOverviewFragment` |

#### PresenceTemplateMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `PresenceTemplateChangeKey` | `PresenceTemplateChangeArguments` (Type: Update/Delete) | `ComeGoServiceManager` (4 sites) | `ComeGoViewTimesActivity` |

#### RegisteringVacationMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `RegisteringVacationChangeKey` | `RegisteringVacationChangeArgument` (Type: Update/Delete, EventId) | `CalendarServiceManager` (2 sites) | (no subscribers found in Android) |

#### DepartmentAndGroupsFilteringMessagingCenter
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `DepartmentAndGroupsFilteringMessagingCenter` | none | `ComeGoEmployeeDashboardFilteringViewModel` | `ComeGoEmployeeDashboardFilteringViewModel` (self-subscribe) |

### 10. Delegate Access Domain

#### DelegateAccessManager (uses MessagingCenter internally)
| Message Key | Payload | Publishers | Subscribers |
|---|---|---|---|
| `DelegateAccessManager` | none | `DelegateAccessManager.SetDelegate()` (implied) | `OverviewFragment`, `CalendarFragment` |

## Dead / Unused Channels (Android)

The following message keys are defined but have no subscribers (or no publishers) in the Android codebase. They likely exist for iOS parity or are vestigial:

| Channel | Issue |
|---|---|
| `LoginMessagingCenter.OpenPinViewKey` | No publishers found |
| `LoginMessagingCenter.ForceLogOutNeededKey` | No publishers or subscribers found |
| `PushNotificationMessagingCenter.PushNotificationKey` | No publishers found (background click handler) |
| `TabChangeMessagingCenter.TabChangeKey` | Published but no subscribers |
| `TabChangeMessagingCenter.TabBarOrderUpdatedInMainViewChangeKey` | Never referenced outside definition |
| `GenericViewMessagingCenter.ViewClosed` | Published but no subscribers |
| `RegisteringVacationMessagingCenter.RegisteringVacationChangeKey` | Published but no subscribers |

## Architectural Observations

### Communication Patterns

1. **Service-to-UI refresh**: The dominant pattern. ServiceManagers (PostServiceManager, MessageServiceManager, etc.) publish after successful API calls; Fragments subscribe to refresh their lists. This decouples the service layer from specific UI components.

2. **Cross-cutting profile filter**: `FilterProfileBarMessagingCenter` is the widest fan-out channel (18 subscribers). When the user switches child profile in `AbstractProfileBar`, every major screen refreshes. This is the "context switch" mechanism.

3. **Async gate pattern**: `LoginFinishedMessagingCenter` uses `TaskCompletionSource` to let `AndroidEmailNotificationsHandler` await login completion before processing push notifications.

4. **Self-subscribe**: `DepartmentAndGroupsFilteringMessagingCenter` is both published and subscribed by the same ViewModel, suggesting coordination between multiple instances of the filtering view.

### Coupling Map (Simplified)

```
AbstractProfileBar ──publishes──> FilterProfileBarMessagingCenter ──> 18 Fragment/ViewModel subscribers
                   ──publishes──> FilterProfileBarForNotificationHandlerMessagingCenter ──> 2 subscribers

ServiceManagers ──publish──> [Post|Document|Gallery|Album|Comment|Notification|Subscription]MessagingCenter
     └──> Corresponding Fragments subscribe for UI refresh

LoginWithPinActivity ──publishes──> LoginFinishedMessagingCenter ──> AndroidEmailNotificationsHandler
AuthenticationManager ──publishes──> LoginMessagingCenter ──> MainActivity (logout flow)
```

### Quantitative Summary

| Metric | Count |
|---|---|
| Wrapper classes | 23 (including DelegateAccessManager) |
| Distinct message keys | 30 |
| Active channels (both publisher and subscriber found) | 23 |
| Dead/unused channels | 7 |
| Files with MessagingCenter usage | 117 |
| Highest fan-out (most subscribers) | FilterProfileBarMessagingCenter (18) |
| Highest fan-in (most publishers) | SubscriptionMessagingCenter.ThreadChange (16+) |
| Most call sites | PostServiceManager -> PostMessagingCenter (13) |
