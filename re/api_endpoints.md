# Aula REST API Endpoints Map

**App**: com.netcompany.aulanativeprivate v2.15.4
**Framework**: .NET MAUI / Xamarin (Mono runtime, .NET 9.0)
**Analysis method**: LZ4 decompression of XALZ-packed .NET assemblies from `libassemblies.x86_64.blob.so`, `monodis` type extraction, and string analysis of decompiled IL metadata.

---

## 1. API Base URLs and Domains

### Known Domains (from AndroidManifest.xml and assembly strings)

| Domain | Purpose | Source |
|--------|---------|--------|
| `app-private.aula.dk` | OAuth callback / deep link (production) | AndroidManifest.xml intent filter |
| `*.aula.dk` | Production domain (wildcard query intent) | AndroidManifest.xml |
| `*.ncaula.com` | Staging/test environment (Netcompany Aula) | AndroidManifest.xml |
| `aula-private.firebaseio.com` | Firebase Realtime Database | strings.xml |
| `aula-private.appspot.com` | Firebase Storage bucket | strings.xml |

### URL Configuration Constants (from AulaNative.dll)

The actual base URLs are configured at runtime via these properties (values are set during app initialization, not hardcoded as string literals in the assembly):

| Constant / Property | Purpose |
|---------------------|---------|
| `BackendUrl` | Main API backend base URL |
| `BackendUrlApi` | API-specific backend URL (likely `{BackendUrl}/api/v{version}`) |
| `AuthBackendUrl` | Authentication backend URL |
| `AuthorizeUrl` / `AUTHORISE_URL` | OAuth authorization endpoint |
| `AccessTokenUrl` / `ACCESS_TOKEN_URL` | OAuth token endpoint |
| `LogoutUrl` / `LOGOUT_URL` | Logout endpoint |
| `LOGOUT_DEV_URL` | Development logout URL |
| `ALL_PROFILES_URL` | All profiles endpoint |
| `AulaUrl` | General Aula web URL |
| `PortalUrl` | Aula web portal URL |
| `API_VERSION` | API version string |
| `ApiVersion` | Runtime API version property |

**Note:** The actual URL values (e.g., `https://api.aula.dk/api/v7/...`) are not embedded as string literals in the decompiled assemblies. They are likely injected at app startup from a configuration endpoint, environment-specific config, or constructed dynamically. The `Api6to7` migration helper confirms the API transitioned from v6 to v7.

### Cloud Storage OAuth URLs

| Constant | Purpose |
|----------|---------|
| `GoogleAccesssTokenUrl` | Google OAuth token URL |
| `GoogleAuthUrl` | Google OAuth authorization URL |
| `GOOGLE_API_URL` | Google Drive API base URL |
| `OneDriveAccesssTokenUrl` | OneDrive OAuth token URL |
| `OneDriveAuthUrl` | OneDrive authorization URL |
| `ONEDRIVE_API_URL` | OneDrive API base URL |
| `OneDriveAndroidPrivateRedirectUrl` | OneDrive redirect for private app |
| `OneDriveAndroidStaffRedirectUrl` | OneDrive redirect for staff app |

### OAuth Redirect URIs (from AndroidManifest.xml)

| URI | Handler |
|-----|---------|
| `https://app-private.aula.dk` | `WebAuthenticationCallbackActivity` (OIDC callback) |
| `com.netcompany.aulanativeprivate://onedrive2redirect` | `CloudStorageAuthInterceptor` (OneDrive OAuth) |
| `com.netcompany.aulanativeprivate:/googleoauth2redirect` | `CloudStorageAuthInterceptor` (Google Drive OAuth) |

---

## 2. Authentication & Authorization

### OIDC / OAuth2 Stack

| Component | Detail |
|-----------|--------|
| Library | `IdentityModel.OidcClient.dll` (standard .NET OIDC library) |
| Authentication class | `AulaNative.OAuth.AuthenticationManager` |
| OAuth2 authenticator | `AulaNative.OAuth.OAuth2Authenticator` |
| Cloud storage auth | `AulaNative.OAuth.OAuthCloudStorage.CloudStorageAuthenticatorManager` |
| Token refresh | `RequestRefreshTokenAsync` |
| Token request | `AulaRequestAccessTokenAsync` |
| Token revocation | `RevokeTokenAsync` |
| Account management | `GetValidAccount`, `GetAccountRequestNewToken` |
| Session delete | `DeleteAccount` |
| Biometric auth | `AulaNative.Enums.BioAuth`, `AuthenticateBiometric` |

### Authentication Flow

1. `PrepareLoginAsync` -> initiate login
2. `AuthorizeUrl` -> redirect to OIDC provider
3. `WebAuthenticationCallbackActivity` -> handles callback from `app-private.aula.dk`
4. `RequestAccessTokenAsync` -> exchange code for tokens
5. `GetValidAccount` -> validate and store account
6. Token refresh via `RequestRefreshTokenAsync`
7. `OpenLogoutAndReturnToAppWithUniversalLink` -> logout flow
8. `CreateEndSessionUrl` -> OIDC end session

### Token Handling

| Property | Purpose |
|----------|---------|
| `AUTH_TOKEN_EXPIRATION` | Token expiry tracking |
| `AddAuthToClient` | Attaches auth token to HTTP client |
| `ExecuteAsyncWithToken` | Executes requests with bearer token |
| `TryGetResponseWithToken` | File download with authentication |
| `InvalidAccessTokenError` | Error handler for expired tokens |
| `SessionExpiredErrorHandler` | Session expiry handling |
| `StepUpNeededErrorHandler` | Step-up auth required |

---

## 3. REST API Endpoints by Service

All services are in the `AulaNative.Services.Web` namespace and extend a common base class. HTTP requests go through `SimpleService` (base utility class) which provides `Get<T>()`, `Post<T>()`, `SimplePost()`, `GenericGetRequest<T>()`, `ExecuteAsyncWithToken()`, and `CheckIfAulaIsAlive<T>()`.

Response wrappers: `AulaServiceResponse<T>`, `DataArrayResponse<T>`, `AulaErrorResponseWrapper<T>`.

### 3.1 IsAlive Service

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `AulaIsAlive` | GET | `/isAlive` or similar | Health check |

### 3.2 Configuration Service

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetMaxFileSize` | GET | `/configuration/maxFileSize` | Maximum upload file size |
| `GetAuthorizedFileFormats` | GET | `/configuration/authorizedFileFormats` | Allowed file types |
| `IsAppDeprecated` | GET | `/configuration/isAppDeprecated` | Force update check |
| `GetPrivacyPolicy` | GET | `/configuration/privacyPolicy` | Privacy policy content |
| `GetAdministrativeAuthority` | GET | `/configuration/administrativeAuthority` | Admin authority info |
| `GetLoginImportantInformation` | GET | `/configuration/loginImportantInformation` | Login info banner |

### 3.3 Profile / Master Data Service

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetOnboardingMasterData` | GET | `/masterdata/onboarding` | Onboarding profile data |
| `GetProfileMasterData` | GET | `/masterdata/profile` | Profile master data |
| `PostMasterData` | POST | `/masterdata` | Update master data |
| `PostUpdateProfilePicture` | POST | `/masterdata/profilePicture` | Update profile picture |
| `KeepAlive` | POST | `/profiles/keepAlive` | Session keep-alive |

### 3.4 Additional Master Data Service

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetAdditionalMasterData` | GET | `/additionalMasterData` | Additional profile data |
| `GetAdditionalMasterDataByInstitutionProfileId` | GET | `/additionalMasterData/{instProfileId}` | Additional data by institution profile |
| `PostAdditionalMasterData` | POST | `/additionalMasterData` | Update additional data |
| `PostAdditionalMasterDataEmployee` | POST | `/additionalMasterData/employee` | Update employee data |

### 3.5 Calendar Service (largest service, 40+ methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetDailyAggregatedEvents` | GET | `/calendar/events/daily` | Daily aggregated events |
| `GetDailyGroupEventCount` | GET | `/calendar/events/dailyGroupCount` | Daily event count per group |
| `GetEvents` | GET | `/calendar/events` | Get calendar events |
| `GetEventDetail` | GET | `/calendar/events/{id}` | Event detail |
| `GetEventTypes` | GET | `/calendar/eventTypes` | Available event types |
| `GetEventTypesForCalendarFeed` | GET | `/calendar/eventTypes/feed` | Event types for feed |
| `GetEventForGroup` | GET | `/calendar/events/group/{id}` | Events for a group |
| `GetSchoolEvents` | GET | `/calendar/schoolEvents` | School-wide events |
| `GetTopImportantDate` | GET | `/calendar/importantDates/top` | Top important dates |
| `RespondSimpleEvent` | POST | `/calendar/events/{id}/respond` | Respond to event |
| `RespondTimeslotEvent` | POST | `/calendar/timeslots/{id}/respond` | Respond to timeslot |
| `UpdateLessonEvent` | PUT | `/calendar/lessons/{id}` | Update lesson |
| `EditTimeslotEvent` | PUT | `/calendar/timeslots/{id}` | Edit timeslot |
| `BlockTimeSlot` | POST | `/calendar/timeslots/block` | Block a timeslot |
| `DeleteTimeSlot` | DELETE | `/calendar/timeslots/{id}` | Delete timeslot |
| `DeleteEvent` | DELETE | `/calendar/events/{id}` | Delete event |
| `CheckConflictEventForAttendees` | POST | `/calendar/events/conflicts` | Check scheduling conflicts |
| `GetBirthdaysForGroup` | GET | `/calendar/birthdays/group/{id}` | Birthdays in a group |
| `GetBirthdaysForInstitution` | GET | `/calendar/birthdays/institution/{id}` | Birthdays in institution |
| `AddVacation` | POST | `/calendar/vacations` | Add vacation |
| `GetVacation` | GET | `/calendar/vacations/{id}` | Get vacation |
| `DeleteVacation` | DELETE | `/calendar/vacations/{id}` | Delete vacation |
| `GetFutureVacationRequest` | GET | `/calendar/vacations/future` | Future vacation requests |
| `GetVacationRequestResponse` | GET | `/calendar/vacations/{id}/response` | Vacation request response |
| `RespondToVacationRegistrationRequest` | POST | `/calendar/vacations/{id}/respond` | Respond to vacation request |
| `GetDelegatedAccesses` | GET | `/calendar/delegatedAccesses` | Delegated calendar access |
| `SetDelegatedAccesses` | POST | `/calendar/delegatedAccesses` | Set delegated access |
| `GetInstitutionProfilesWithDelegatedAccesses` | GET | `/calendar/delegatedAccesses/profiles` | Profiles with delegated access |
| `GetCalendarSynchronisationConfigurations` | GET | `/calendar/sync/configurations` | Calendar sync configs |
| `CreateCalendarSynchronisationConfiguration` | POST | `/calendar/sync/configurations` | Create sync config |
| `UpdateCalendarSynchronisationConfiguration` | PUT | `/calendar/sync/configurations/{id}` | Update sync config |
| `DeleteCalendarSynchronisationConfiguration` | DELETE | `/calendar/sync/configurations/{id}` | Delete sync config |
| `GetCalendarSynchronisationConsent` | GET | `/calendar/sync/consent` | Sync consent status |
| `UpdateCalendarSynchronisationConsent` | PUT | `/calendar/sync/consent` | Update sync consent |
| `GetIsCalendarFeedEnabledForMunicipality` | GET | `/calendar/feed/municipality/{id}/enabled` | Feed enabled check |
| `GetFeedConfigurationById` | GET | `/calendar/feed/configuration/{id}` | Feed configuration |

### 3.6 ComeGo Service (Presence/Attendance — 40+ methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetPresenceSchedules` | GET | `/presence/schedules` | Presence schedules |
| `GetPresenceWeekOverview` | GET | `/presence/weekOverview` | Weekly presence overview |
| `GetChildrensState` | GET | `/presence/children/state` | Children's current state |
| `UpdateOneDayPresence` | PUT | `/presence/oneDay` | Update single day presence |
| `GetPresenceConfiguration` | GET | `/presence/configuration` | Presence config |
| `GetPresenceConfigurationByChildrenIds` | GET | `/presence/configuration/children` | Config by children |
| `GetSuggestionsForPickUp` | GET | `/presence/pickup/suggestions` | Pickup suggestions |
| `UpdateSuggestionsForPickup` | PUT | `/presence/pickup/suggestions` | Update pickup suggestions |
| `GetPickupResponsibles` | GET | `/presence/pickup/responsibles` | Pickup responsibles |
| `DeletePickupResponsible` | DELETE | `/presence/pickup/responsibles/{id}` | Delete pickup responsible |
| `GetActivityList` | GET | `/presence/activities` | Activity list |
| `GetActivityFilter` | GET | `/presence/activities/filter` | Activity filter options |
| `GetChildGoHomeWith` | GET | `/presence/children/{id}/goHomeWith` | Who child goes home with |
| `GetPresenceFilter` | GET | `/presence/filter` | Presence filter |
| `GetPresenceFilters` | GET | `/presence/filters` | Available filters |
| `GetChildrenVacation` | GET | `/presence/children/vacation` | Children vacation |
| `GetDailyOverview` | GET | `/presence/daily/overview` | Daily presence overview |
| `GetAvailableLocations` | GET | `/presence/locations` | Available locations |
| `UpdateLocation` | PUT | `/presence/location` | Update location |
| `AddSleepIntervals` | POST | `/presence/sleep` | Add sleep intervals |
| `UpdateSleepInterval` | PUT | `/presence/sleep/{id}` | Update sleep interval |
| `DeleteSleepIntervals` | DELETE | `/presence/sleep` | Delete sleep intervals |
| `AddVacation` | POST | `/presence/vacation` | Add vacation |
| `GetPresenceRegistrations` | GET | `/presence/registrations` | Presence registrations |
| `GetPresenceRegistrationDetail` | GET | `/presence/registrations/{id}` | Registration detail |
| `GetPresenceRegistrationsByIds` | GET | `/presence/registrations/byIds` | Registrations by IDs |
| `UpdatePresenceRegistration` | PUT | `/presence/registrations/{id}` | Update registration |
| `UpdateStatusByPresenceRegistrationIds` | PUT | `/presence/registrations/status` | Bulk status update |
| `UpdateStatusByInstitutionProfileIds` | PUT | `/presence/status/byProfiles` | Status by profiles |
| `GetPresenceChildrenDistribution` | GET | `/presence/children/distribution` | Children distribution |
| `GetTemplateForDate` | GET | `/presence/templates/{date}` | Template for date |
| `DeleteRepeatedPresenceTemplate` | DELETE | `/presence/templates/repeated/{id}` | Delete repeated template |
| `GetOverlappingPresenceTemplates` | GET | `/presence/templates/overlapping` | Overlapping templates |
| `GetClosedDays` | GET | `/presence/closedDays` | Closed days |
| `GetGeneralOpeningHours` | GET | `/presence/openingHours` | General opening hours |
| `GetOpeningHoursByInstitutionCodes` | GET | `/presence/openingHours/institution` | Opening hours by institution |
| `GetSpecificOpeningHourOverview` | GET | `/presence/openingHours/specific` | Specific opening hours |
| `GetAvailablePresenceStatuses` | GET | `/presence/statuses` | Available statuses |
| `GetInstitutionWithPresenceStates` | GET | `/presence/institution/states` | Institution presence states |
| `GetVacationAnnouncementsByChildren` | GET | `/presence/vacation/announcements` | Vacation announcements |
| `GetVacationRegistrationOverview` | GET | `/presence/vacation/registrations/overview` | Vacation reg overview |
| `GetVacationRegistrationsByChildren` | GET | `/presence/vacation/registrations/children` | Registrations by children |
| `GetExistingVacationRegistrationResponse` | GET | `/presence/vacation/registrations/existing` | Existing vacation reg |

### 3.7 Message Service (26 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetThreadList` | GET | `/messaging/threads` | List message threads |
| `GetThreadById` | GET | `/messaging/threads/{id}` | Get specific thread |
| `DeleteThreads` | DELETE | `/messaging/threads` | Delete threads |
| `LeaveThread` | POST | `/messaging/threads/{id}/leave` | Leave a thread |
| `LeaveThreads` | POST | `/messaging/threads/leave` | Leave multiple threads |
| `SetThreadMuted` | PUT | `/messaging/threads/{id}/muted` | Mute thread |
| `SetSensitiveLevel` | PUT | `/messaging/threads/{id}/sensitive` | Set sensitivity level |
| `SetThreadMarked` | PUT | `/messaging/threads/{id}/marked` | Mark thread |
| `GetMessageList` | GET | `/messaging/threads/{id}/messages` | Messages in thread |
| `GetMessageInfoLight` | GET | `/messaging/messages/{id}/info` | Message info (light) |
| `StartNewThread` | POST | `/messaging/threads` | Start new thread |
| `ReplyInNewThread` | POST | `/messaging/threads/{id}/replyNew` | Reply in new thread |
| `ForwardThread` | POST | `/messaging/threads/{id}/forward` | Forward thread |
| `AddRecipientsToThread` | POST | `/messaging/threads/{id}/recipients` | Add recipients |
| `ReplyToThread` | POST | `/messaging/threads/{id}/reply` | Reply to thread |
| `SetLastReadMessage` | PUT | `/messaging/threads/{id}/lastRead` | Mark as read |
| `SetAutoReply` | POST | `/messaging/autoReply` | Set auto-reply |
| `GetAutoReply` | GET | `/messaging/autoReply` | Get auto-reply |
| `DeleteAutoReply` | DELETE | `/messaging/autoReply` | Delete auto-reply |
| `DeleteMessage` | DELETE | `/messaging/messages/{id}` | Delete message |
| `EditMessage` | PUT | `/messaging/messages/{id}` | Edit message |
| `SendEventReminder` | POST | `/messaging/eventReminder` | Send event reminder |
| `GetThreadsInBundleList` | GET | `/messaging/threads/bundle` | Thread bundles |
| `SetMessageThreadsSubscriptionStatus` | PUT | `/messaging/threads/subscription` | Subscription status |
| `CheckRecipientsForBlockedChannels` | POST | `/messaging/recipients/blocked` | Check blocked recipients |
| `AttachMessagesToSecureDocument` | POST | `/messaging/messages/attachToDocument` | Attach to secure doc |

### 3.8 Post Service (8 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetPosts` | GET | `/posts` | Get posts feed |
| `GetPostById` | GET | `/posts/{id}` | Get specific post |
| `CreatePost` | POST | `/posts` | Create new post |
| `EditPost` | PUT | `/posts/{id}` | Edit post |
| `DeletePost` | DELETE | `/posts/{id}` | Delete post |
| `ReportPost` | POST | `/posts/{id}/report` | Report post |
| `BookmarkPost` | POST | `/posts/{id}/bookmark` | Bookmark post |
| `UnbookmarkPost` | DELETE | `/posts/{id}/bookmark` | Remove bookmark |

### 3.9 Comment Service (5 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `AddComment` | POST | `/comments` | Add comment |
| `UpdateComment` | PUT | `/comments/{id}` | Update comment |
| `GetComments` | GET | `/comments` | Get comments |
| `ReportComment` | POST | `/comments/{id}/report` | Report comment |
| `DeleteComment` | DELETE | `/comments/{id}` | Delete comment |

### 3.10 Gallery Service (12 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetAlbums` | GET | `/gallery/albums` | Get albums |
| `GetAlbumsCached` | GET | `/gallery/albums` (cached) | Albums (with cache) |
| `GetMediasInAlbum` | GET | `/gallery/albums/{id}/media` | Media in album |
| `GetMediasInAlbumCached` | GET | `/gallery/albums/{id}/media` (cached) | Media (with cache) |
| `GetMediaById` | GET | `/gallery/media/{id}` | Get media by ID |
| `CreateAlbum` | POST | `/gallery/albums` | Create album |
| `UpdateAlbum` | PUT | `/gallery/albums/{id}` | Update album |
| `DeleteAlbum` | DELETE | `/gallery/albums/{id}` | Delete album |
| `DeleteMedia` | DELETE | `/gallery/media/{id}` | Delete media |
| `AddTag` | POST | `/gallery/media/{id}/tags` | Tag media |
| `RemoveTag` | DELETE | `/gallery/media/{id}/tags/{tagId}` | Remove tag |
| `ReportMedia` | POST | `/gallery/media/{id}/report` | Report media |

### 3.11 Document Service (Secure Documents — 18 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetSecureDocuments` | GET | `/documents/secure` | List secure documents |
| `GetCommonFiles` | GET | `/documents/common` | Common files |
| `UpdateSharings` | PUT | `/documents/{id}/sharings` | Update sharings |
| `RemoveOwnSharings` | DELETE | `/documents/{id}/sharings/own` | Remove own sharings |
| `GetImplicitSharings` | GET | `/documents/{id}/implicitSharings` | Implicit sharings |
| `GetDocumentRevisions` | GET | `/documents/{id}/revisions` | Document revisions |
| `GetExternalDocumentDetails` | GET | `/documents/external/{id}` | External doc details |
| `GetExternalSecureDocumentRevision` | GET | `/documents/external/{id}/revision` | External doc revision |
| `GetInternalDocumentDetails` | GET | `/documents/internal/{id}` | Internal doc details |
| `GetInternalSecureDocumentRevision` | GET | `/documents/internal/{id}/revision` | Internal doc revision |
| `CreateInternalSecureDocument` | POST | `/documents/internal` | Create internal doc |
| `UpdateInternalSecureDocument` | PUT | `/documents/internal/{id}` | Update internal doc |
| `UpdateDocumentLockedStatus` | PUT | `/documents/{id}/locked` | Lock/unlock doc |
| `SoftDeleteSecureDocument` | DELETE | `/documents/{id}` | Soft delete doc |
| `GetShareableSecureDocuments` | GET | `/documents/shareable` | Shareable documents |
| `GetMaxDocumentsPerExport` | GET | `/documents/export/maxCount` | Export limit |
| `CreateExportForMultipleSecureDocuments` | POST | `/documents/export` | Create export |
| `TrackExportForMultipleSecureDocuments` | GET | `/documents/export/{id}/status` | Track export status |
| `CreatePDFForSingleDocument` | POST | `/documents/{id}/pdf` | Generate PDF |
| `TrackCreatePDFForSingleDocument` | GET | `/documents/{id}/pdf/status` | Track PDF generation |

### 3.12 File Service (8 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `CreateDocumentLinks` | POST | `/files/documentLinks` | Create document links |
| `UploadFileToAws` | PUT | (pre-signed AWS URL) | Upload file to S3 |
| `UploadPartToAws` | PUT | (pre-signed AWS URL) | Multipart S3 upload |
| `FetchHttpResponse` | GET | (dynamic URL) | Fetch file response |
| `TryGetResponseWithToken` | GET | (authenticated URL) | Authenticated file download |
| `DownloadFileWithProgress` | GET | (dynamic URL) | Download with progress |
| `StoreDownloadedFile` | - | (local) | Store downloaded file |
| `SaveImageFileWithProgress` | - | (local) | Save image file |

### 3.13 Folder Service (6 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetFolders` | GET | `/messaging/folders` | Get folders |
| `UpdateFolder` | PUT | `/messaging/folders/{id}` | Update folder |
| `CreateFolder` | POST | `/messaging/folders` | Create folder |
| `PostDeleteFolder` | POST/DELETE | `/messaging/folders/{id}` | Delete folder |
| `MoveThreadsToFolder` | POST | `/messaging/folders/{id}/moveThreads` | Move threads to folder |
| `GetCommonInboxes` | GET | `/messaging/commonInboxes` | Common inboxes |

### 3.14 Group Service (4 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetGroup` | GET | `/groups/{id}` | Get group |
| `GetGroupByContext` | GET | `/groups/context/{id}` | Get group by context |
| `GetMembershipsLight` | GET | `/groups/{id}/memberships` | Light membership list |
| `JoinOrLeaveGroup` | POST | `/groups/{id}/membership` | Join or leave group |

### 3.15 Search Service (9 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GlobalSearch` | GET | `/search` | Global search across all content |
| `SearchForMessages` | GET | `/search/messages` | Search messages |
| `SearchForProfiles` | GET | `/search/profiles` | Search profiles |
| `SearchForProfilesAndGroups` | GET | `/search/profilesAndGroups` | Search profiles and groups |
| `SearchForRecipients` | GET | `/search/recipients` | Search message recipients |
| `SearchForRecipientsForPersonalReference` | GET | `/search/recipients/personalReference` | Recipients for personal ref |
| `SearchForRecipientsForSecureDocument` | GET | `/search/recipients/secureDocument` | Recipients for secure docs |
| `SearchForGroupsToAssociateDocument` | GET | `/search/groups/document` | Groups for doc association |
| `SearchGroups` | GET | `/search/groups` | Search groups |

### 3.16 Notification Service (3 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetNotificationsForActiveProfile` | GET | `/notifications` | Get notifications |
| `DeleteNotifications` | DELETE | `/notifications` | Delete notifications |
| `DeleteNotificationForRelatedChild` | DELETE | `/notifications/child/{id}` | Delete child notifications |

### 3.17 Remote Notification Service (Push Notifications — 7 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `RegisterDevice` | POST | `/pushNotifications/devices` | Register device for push |
| `UnregisterDevice` | DELETE | `/pushNotifications/devices/{id}` | Unregister device |
| `DeleteAllDevices` | DELETE | `/pushNotifications/devices` | Delete all devices |
| `GetDevices` | GET | `/pushNotifications/devices` | Get registered devices |
| `GetNotificationSettings` | GET | `/pushNotifications/settings` | Get notification settings |
| `UpdateNotificationSettings` | PUT | `/pushNotifications/settings` | Update notification settings |
| `ClearNotificationBadgesByModule` | POST | `/pushNotifications/badges/clear` | Clear badge counts |

### 3.18 Onboarding Service (2 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `MarkOnboardingComplete` | POST | `/onboarding/complete` | Mark onboarding complete |
| `GetPolicyLinks` | GET | `/onboarding/policyLinks` | Get policy links |

### 3.19 Consent Service (2 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetConsents` | GET | `/consents` | Get consent status |
| `PostConsents` | POST | `/consents` | Submit consent response |

### 3.20 Personal Reference Data Service (3 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetPersonalReferenceAdditionalAnswerData` | GET | `/personalReference/additionalAnswers` | Additional answer data |
| `GetPersonalReferenceConsentAnswerData` | GET | `/personalReference/consentAnswers` | Consent answer data |
| `GetPersonalReferenceQuestionData` | GET | `/personalReference/questions` | Question data |

### 3.21 Widget Service (1 method)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetAulaToken` | GET | `/widget/token` | Get widget SSO token |

### 3.22 Cloud Storage OAuth Service (2 methods)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `GetGoogleFilesFromFolder` | GET | (Google Drive API) | List Google Drive files |
| `GetOneDriveFilesFromFolder` | GET | (OneDrive API) | List OneDrive files |

### 3.23 Logging Service (1 method)

| Method | HTTP | Endpoint (inferred) | Purpose |
|--------|------|---------------------|---------|
| `LogToBackend` | POST | `/logging` | Log events to backend |

---

## 4. API Request/Response Model Classes

### Common API Models

| Class | Purpose |
|-------|---------|
| `AulaServiceResponse` | Base response wrapper |
| `AulaServiceResponse<T>` | Generic typed response |
| `DataArrayResponse<T>` | Array data response |
| `AulaErrorResponseWrapper<T>` | Error response wrapper |
| `AulaErrorResponseWrapperStatus<T>` | Error with status |
| `WebResponseStatus` | Response status codes |
| `WebResponseStatusSubCodeConstants` | Sub-code constants |
| `AulaUrl` | URL model |
| `PostResponse` | Post response wrapper |
| `IdWrapperResultModel` | ID wrapper for create operations |

### Domain-Specific API Models

| Namespace | Key Model Classes |
|-----------|-------------------|
| `AulaNative.Models.Posts.Api` | `CreatePostApiParameter`, `CreatePostResult`, `GetPostApiParameters`, `GetPostApiResult`, `PostApiDto`, `ReportApiParameter` |
| `AulaNative.Models.Notification.Api` | `DeleteNotificationForRelatedChildRequestModel`, `DeleteNotificationParameter`, `DeleteNotificationsDto`, `GetNotificationsApiParameter` |
| `AulaNative.Models.Common.Api` | `ProfileApiDto` |
| `AulaNative.Models.Common.Api.Files` | `FileConnectionResult` |
| `AulaNative.Models.Common.Api.Files.Result` | `AulaDocumentLinkContent`, `AulaFileAlbumDto`, `AulaFileContent`, `AulaFileResultDto`, `AulaFileResultProfileDto`, `AulaGalleryMediaFileResultDto`, `AulaLinkContent`, `AulaMediaFileContent`, `AuthorizedFileFormat` |
| `AulaNative.Models.Common.Api.Files.Parameters` | `AttachmentFeatureV2` |
| `AulaNative.Models.MessageThreads.Argument` | `RecipientApiModel`, `RecipientApiType`, `RecipientApiTypeHelpers` |
| `AulaNative.Services.Web` | `AddCommentRequestModel`, `GetCommmentsRequestModel`, `UpdateDocumentStatusRequestModel` |

---

## 5. API Versioning

| Evidence | Detail |
|----------|--------|
| `API_VERSION` constant | Runtime API version identifier |
| `ApiVersion` property | Property on service base class |
| `get_BackendUrlApi` | Likely constructs `{BackendUrl}/api/v{ApiVersion}/` |
| `Api6to7DraftJsonBinder` | Migration helper from API v6 to v7 |
| `FromApi6To7Helper` | Explicit migration class |
| Current version (inferred) | **v7** (based on migration from v6 to v7) |

The API versioning appears to follow a simple numeric scheme (`v6`, `v7`) embedded in the URL path. The `BackendUrlApi` property likely resolves to something like `https://{hostname}/api/v7/`.

---

## 6. Error Handling

| Error Handler | Purpose |
|---------------|---------|
| `AbortRequestErrorHandler` | Request abortion |
| `AulaMaintenanceErrorHandler` | Aula under maintenance |
| `AulaNotRespondingErrorHandler` | Aula not responding |
| `GenericErrorHandler` | Generic API errors |
| `HeavyLoadingErrorHandler` | High load / rate limiting |
| `HttpRequestErrorHandler` | HTTP request failures |
| `InvalidAccessTokenError` | Expired/invalid access token |
| `NoNetworkErrorHandler` | No network connectivity |
| `SessionExpiredErrorHandler` | Session expired |
| `StepUpNeededErrorHandler` | Step-up authentication required |
| `ThrowExceptionErrorHandler` | Throw exception on error |
| `UnauthorizedErrorHandler` | 401 Unauthorized |
| `UserDeactivatedErrorHandler` | Deactivated user account |
| `SilentErrorHandler` | Silently handle errors |

Custom exceptions: `AulaIsDownException`, `AulaMaintenanceException`, `ApiFailWithDetailsException<T>`.

---

## 7. Firebase Cloud Messaging (Push Notifications)

| Component | Detail |
|-----------|--------|
| Service | `AulaNative.Droid.FireBase.AulaFirebaseMessagingService` |
| FCM Sender ID | `811573413698` |
| Firebase Project | `aula-private` |
| API Key | `AIzaSyBdpTl_XKcu2l2cmK79GwnHkz9GW_PoPZc` |
| OAuth Web Client ID | `811573413698-mnjq3uvi6b23ajkghjp4rodbtgks0uqt.apps.googleusercontent.com` |
| Token management | `onNewToken`, `onMessageReceived` -> native .NET handlers |

---

## 8. Information URLs (AulaInfo*)

These are help/info page URLs shown to users within the app:

| Constant | Topic |
|----------|-------|
| `AulaInfoUrl` | General Aula information |
| `AulaInfoCalendarOverviewUrl` | Calendar overview help |
| `AulaInfoCalendarLessonNotesUrl` | Lesson notes help |
| `AulaInfoCalendarLocationAndSendReminderUrl` | Location and reminders |
| `AulaInfoCalendarMultiplePeopleInSameTimeSlotUrl` | Timeslot conflicts |
| `AulaInfoCalendarSendNotificationUrl` | Calendar notifications |
| `AulaInfoComeGoGuardianOverviewUrl` | Guardian presence overview |
| `AulaInfoGalleryMediaConversionUrl` | Media conversion info |
| `AulaInfoMessageBccRecipientUrl` | BCC recipients help |
| `AulaInfoMessageOverviewUrl` | Message overview help |
| `AulaInfoSensitiveMessageUrl` | Sensitive messages info |
| `AulaInfoOnboardingPrimaryInstitutionUrl` | Primary institution onboarding |
| `AulaInfoOnboardingShareChildContactUrl` | Share child contact info |
| `AulaInfoOnboardingShareSelfContactUrl` | Share self contact info |
| `AulaInfoSecureFilesConcerningChildrenUrl` | Secure files about children |
| `AulaInfoSecureFilesOverviewUrl` | Secure files overview |
| `AulaInfoSecureFilesRemoveAssociationUrl` | Remove file association |
| `AulaInfoSecureFilesSharedWithUrl` | Shared files info |

---

## 9. Limitations and Next Steps

### What we COULD extract
- Complete .NET type hierarchy and namespace structure
- All Web Service class names and method signatures (200+ API operations)
- URL configuration constant names
- Request/response model class names
- Authentication flow components
- Error handling strategy
- API version migration evidence (v6 -> v7)

### What we could NOT extract without full IL decompilation
- **Actual URL path strings** (e.g., the literal `/api/v7/messaging/threads`) -- these are likely constructed in method bodies using string interpolation or concatenation, not stored as metadata strings
- **HTTP method assignments** (GET/POST/PUT/DELETE) -- inferred from method naming conventions but not verified from IL
- **Request parameter details** (query params, request body schemas) -- requires IL decompilation of service methods
- **Response body JSON structure** -- requires decompiling model classes with their properties and JSON attributes
- **Header names and values** -- requires IL decompilation of HTTP client setup
- **Rate limiting / pagination patterns** -- `NextPageUrl` property exists but implementation details unknown

### Recommended Follow-up Tasks
1. **Full .NET decompilation** -- Use `ilspy` or `dnSpy` (via Wine or .NET SDK) on the extracted `assembly_187_AulaNative.dll` (4.3MB) to get complete C# source code including URL paths, HTTP methods, and request/response bodies
2. **Network traffic capture** -- Use mitmproxy with the running app to capture actual API requests and validate endpoint paths
3. **API documentation generation** -- Once URLs are confirmed, generate OpenAPI/Swagger spec from the captured traffic

---

## 10. Assembly Inventory

### App-Specific Assemblies
| Assembly | Size | Purpose |
|----------|------|---------|
| `AulaNative.dll` (assembly_187) | 4.3MB | Core business logic, all services, models, view models |
| `AulaNative.Droid.dll` (assembly_0) | 2.4MB | Android-specific UI, activities, fragments |
| `AulaNative.Droid.Private.dll` (assembly_188) | 5KB | Private variant metadata (production config) |
| `AulaNative.Droid.dll` (assembly_189) | 1.0MB | Android UI resources and views |

### Key Third-Party Libraries
| Library | Purpose |
|---------|---------|
| `IdentityModel.dll` | OIDC/OAuth2 protocol handling |
| `IdentityModel.OidcClient.dll` | OIDC client implementation |
| `Newtonsoft.Json.dll` | JSON serialization |
| `AutoMapper.dll` | Object mapping (DTO <-> domain) |
| `MonkeyCache.dll` + `MonkeyCache.SQLite.dll` | HTTP response caching |
| `SQLite-net.dll` | Local SQLite database |
| `Plugin.Fingerprint.dll` | Biometric authentication |
| `Plugin.SecureStorage.dll` | Secure credential storage |
| `SixLabors.ImageSharp.dll` | Image processing |
| `I18NPortable.dll` + `I18NPortable.JsonReader.dll` | Internationalization |
| `DeviceId.dll` | Device identification |
| `Unity.Container.dll` + `Unity.Abstractions.dll` | Dependency injection |
| `CommonServiceLocator.dll` | Service locator pattern |
| `Square.OkIO.dll` | Efficient I/O (via Xamarin binding) |

### Total: 363 assemblies in blob (most are .NET framework/Xamarin bindings)
