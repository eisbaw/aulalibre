# AulaNative Full Decompilation Analysis

Produced by ILSpy (ilspycmd 9.1) decompilation of `assembly_187_AulaNative.dll` (4.3MB).
Decompiled to 1825 C# source files across all namespaces. Droid assembly decompiled to 778 files.

## Environment Configuration

Production backend: `https://www.aula.dk/`
Production auth: `https://login.aula.dk/`
API base URL pattern: `https://www.aula.dk/api/v23/`
Data host (private app): `app-private.aula.dk`

Test environments include: preprod, hotfix, test1, test3, dev1, dev3, dev11, dev21, dev22, dev31, dev32, CI
Test hosts follow pattern: `www1-{env}.ncaula.com`

App ID: `com.netcompany.aulanativeprivate`
API Version: 23
Default timeout: 1 minute

### Authentication

OAuth2 via SimpleSAMLphp/OIDC:
- Authorize URL: `https://login.aula.dk/simplesaml/module.php/oidc/authorize.php`
- Token URL: `https://login.aula.dk/simplesaml/module.php/oidc/token.php`
- Logout URL: `https://login.aula.dk/auth/logout.php`

OAuth2 Client IDs:
- Step level 2 (scope `aula`): `_742adb5e2759028d86dbadf4af44ef70e8b1f407a6`
- Step level 3 (scope `aula-sensitive`): `_99949a54b8b65423862aac1bf629599ed64231607a`

Session: 60 minutes expiry, warning at 55 minutes.

### Certificate Pinning

For `aula.dk`:
- `/P3+fgXhRH6jPoKBMmAKWRrtjDoEZf4ySjxLoQuqsYc=`
- `eLCo7AWQ2P88/2FQfow993oOhcjXal2sS/e2mZgJLJE=`
- `9XtneGQWNOLQFi0f8LEJ62bt1f/pVrCb4ytT66RcurA=`

For `ncaula.com` (test):
- `ejsQt33CcKZWEoO/ym2mcdSynXrVfK1o6QbTI868tDE=`
- `PfUUWB6dvdMA9exWlx0W+6lKT540ElcRWUERcBRtP6o=`
- `CC09RfvRZQ1z+bj1VeJ/jrYOeH3D0epyQR+FEXLddF8=`

---

## Complete REST API Endpoint Map

All endpoints use the base URL `https://www.aula.dk/api/v23/`.
The API uses `?method=` query parameter routing (not path-based REST).
All responses wrap data in `{ "data": ..., "status": ... }` JSON envelope.

### Session (method=session.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| KEEP_ALIVE | POST | `?method=session.keepAlive` | bool |

### Central Configuration (method=centralConfiguration.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_MAX_FILE_SIZE | GET | `?method=centralConfiguration.getMaxFileSize` | - |
| GET_AUTHORIZED_FILE_FORMATS | GET | `?method=centralConfiguration.getauthorizedfileformats` | List\<AuthorizedFileFormat\> |
| GET_DATA_POLICY | GET | `?method=centralConfiguration.getDataPolicy` | PrivacyPolicyResultModel |
| IS_APP_DEPRECATED | GET | `?method=centralConfiguration.isAppVersionDeprecated` | - |
| LOGIN_GET_IMPORTANT_INFO | GET | `?method=centralConfiguration.getLoginImportantInformation` | - |

### Municipal Configuration (method=municipalConfiguration.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_ADMIN_AUTHORITIES | GET | `?method=municipalConfiguration.getSameAdministrativeAuthorityInstitutions` | - |
| GET_BLOCKED_COMMUNICATION | POST | `?method=municipalConfiguration.getBlockedCommunicationInstitutionProfilesAndGroups` | List\<BlockedUsersByChannelDTO\> |
| GET_CALENDAR_FEED_ENABLED | GET | `?method=MunicipalConfiguration.getCalendarFeedEnabled` | List\<CalendarSynchronisationMunicipalityFeedModel\> |

### Messaging (method=messaging.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| START_NEW_THREAD | POST | `?method=messaging.startNewThread` | IdWrapperResultModel |
| REPLY_TO_THREAD | POST | `?method=messaging.reply` | List\<MessageDto\> |
| GET_THREADS | GET | `?method=messaging.getThreads` | MessageThreadSubscriptionList |
| GET_MESSAGES_FOR_THREAD | GET | `?method=messaging.getMessagesForThread` | MessagesInThreadDto |
| SET_LAST_READ_MESSAGE | POST | `?method=messaging.setLastReadMessage` | bool |
| DELETE_THREAD | POST | `?method=messaging.deleteThreads` | bool |
| LEAVE_THREAD | POST | `?method=messaging.leaveThread` | bool |
| LEAVE_THREADS | POST | `?method=messaging.leaveThreads` | bool |
| SET_THREAD_MUTED | POST | `?method=messaging.setThreadsMuted` | bool |
| SET_THREAD_MARKED | POST | `?method=messaging.setThreadsMarked` | bool |
| SET_AUTOREPLY | POST | `?method=messaging.setAutoReply` | bool |
| GET_AUTOREPLY | GET | `?method=messaging.getAutoReply` | MessageAutoReplyResult |
| DELETE_AUTOREPLY | POST | `?method=messaging.deleteAutoReply` | bool |
| SET_SENSITIVITY_LEVEL | POST | `?method=messaging.setSensitivityLevel` | bool |
| GET_FOLDERS | GET | `?method=messaging.getFolders` | List\<Folder\> |
| CREATE_FOLDER | POST | `?method=messaging.createFolder` | IdWrapperResultModel |
| DELETE_FOLDER | POST | `?method=messaging.deletefolder` | bool |
| UPDATE_FOLDER | POST | `?method=messaging.updateFolder` | bool |
| MOVE_THREADS_TO_FOLDER | POST | `?method=messaging.moveThreadsToFolder` | bool |
| ADD_RECIPIENTS | POST | `?method=messaging.addRecipients` | bool |
| ATTACH_TO_SECURE_DOC | POST | `?method=messaging.attachMessagesToSecureDocument` | bool |
| GET_COMMON_INBOXES | GET | `?method=messaging.getCommonInboxes` | List\<CommonInboxesDto\> |
| UPDATE_SUBSCRIPTION_STATUS | POST | `?method=messaging.updateSubscriptionStatus` | bool |
| GET_THREADS_IN_BUNDLE | GET | `?method=messaging.getThreadsInBundle` | MessageThreadSubscriptionList |
| DELETE_MESSAGE | POST | `?method=messaging.deleteMessage` | DeleteMessageDto |
| EDIT_MESSAGE | POST | `?method=messaging.editMessage` | MessageDto |
| SEND_EVENT_REMINDER | POST | `?method=messaging.sendEventReminder` | IdWrapperResultModel |
| GET_MESSAGE_INFO_LIGHT | GET | `?method=messaging.getMessageInfoLight` | GetMessageInfoLightDto |

### Profiles (method=profiles.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| ALL_PROFILES | GET | `?method=profiles.getAllProfiles` | - |
| GET_PROFILE_CONTEXT | GET | `?method=profiles.getProfileContext` | - |
| GET_PROFILE_TYPES | GET | `?method=profiles.getProfileTypesByLogin` | - |
| GET_ADDITIONAL_DATA_FOR_OWNER | GET | `?method=profiles.getAdditionalDataResponsesForOwner` | List\<InstitutionProfileAdditionalMasterDataViewModel\> |
| GET_ADDITIONAL_DATA_BY_ID | GET | `?method=profiles.getAdditionalDataResponsesByInstitutionProfileIds` | List\<InstitutionProfileAdditionalMasterDataViewModel\> |
| UPDATE_ADDITIONAL_DATA | POST | `?method=profiles.updateAdditionalDataResponses` | - |
| GET_MASTER_DATA | GET | `?method=profiles.getProfileMasterData` | MasterDataModelViewModel |
| GET_PROFILES_BY_LOGIN | GET | `?method=profiles.getProfilesByLogin` | MasterDataDTO |
| UPDATE_MASTER_DATA | POST | `?method=profiles.updateProfileMasterData` | NullServiceObject |
| GET_PROFILE_BY_LOGIN | GET | `?method=profiles.getprofilesbylogin` | - |
| MARK_ONBOARDING_COMPLETED | GET | `?method=profiles.markOnboardingCompleted` | NullServiceObject |
| UPDATE_PROFILE_PICTURE | POST | `?method=profiles.updateProfilePicture` | ProfilePictureDto |
| GET_CONTACT_LIST | GET | `?method=profiles.getContactList` | - |
| UPDATE_ADDITIONAL_DATA_EMPLOYEE | POST | `?method=profiles.updateAdditionalDataResponsesEmployee` | - |
| GET_CONTACT_PARENTS | GET | `?method=profiles.getContactParents` | - |
| GET_POLICY_LINKS | GET | `?method=CommonFiles.getPersonalDataPolicies` | List\<PolicyLinksViewModel\> |

### Consents (method=consents.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_CONSENTS | GET | `?method=consents.getConsentResponses` | List\<InstitutionProfileConsentDTO\> |
| POST_CONSENTS | POST | `?method=consents.updateConsentResponses` | WebResponseStatus |

### Search (method=search.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| FIND_RECIPIENTS | GET | `?method=search.findRecipients` | SearchRecipientResponse |
| GLOBAL_SEARCH | GET | `?method=search.findGeneric` | SearchResponse |
| FIND_PROFILES_AND_GROUPS | GET | `?method=search.findProfilesAndGroups` | SearchRecipientResponse |
| FIND_PROFILES | GET | `?method=search.findProfiles` | SearchRecipientResponse |
| FIND_RECIPIENTS_PERSONAL_REF | GET | `?method=search.findRecipientsPersonalReferenceData` | SearchRecipientResponse |
| FIND_RECIPIENTS_SECURED_DOC | GET | `?method=search.findProfilesAndGroupsToShareDocument` | SearchRecipientResponse |
| FIND_GROUPS | GET | `?method=search.findGroups` | SearchGroupResultModel |
| FIND_PROFILES_ASSOCIATE_DOC | GET | `?method=search.findProfilesAndGroupsToAssociateDocument` | SearchRecipientResponse |
| SEARCH_IN_MESSAGES | POST | `?method=search.findMessage` | SearchResultMessagesResponse |

### Groups (method=groups.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_GROUP_BY_ID | GET | `?method=groups.getGroupById&groupId={id}` | Group |
| JOIN_OR_LEAVE_GROUP | POST | `?method=groups.joinOrLeaveGroup` | bool |
| GET_MEMBERSHIPS_LIGHT | GET | `?method=groups.getMembershipsLight` | List\<MembershipLightPortalRoleGroupResultModel\> |
| GET_GROUPS_BY_CONTEXT | GET | `?method=groups.getGroupsByContext` | GroupByContextResultModel[] |

### Posts (method=posts.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_ALL_POSTS | GET | `?method=posts.getAllPosts` | GetPostApiResult |
| GET_POST_BY_ID | GET | `?method=posts.getById` | PostApiDto |
| CREATE_POST | POST | `?method=posts.createPost` | CreatePostResult |
| UPDATE_POST | POST | `?method=posts.updatePost` | CreatePostResult |
| DELETE_POST | POST | `?method=posts.deletePost` | bool |
| REPORT_POST | POST | `?method=posts.reportPost` | bool |
| BOOKMARK | POST | `?method=posts.bookmark` | bool |
| UNBOOKMARK | POST | `?method=posts.unbookmark` | bool |

### Calendar (method=calendar.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_EVENTS | POST | `?method=calendar.getEventsByProfileIdsAndResourceIds` | List\<EventSimpleDto\> |
| GET_IMPORTANT_DATES | GET | `?method=calendar.getImportantDates` | List\<EventSimpleDto\> |
| GET_EVENT_BY_ID | GET | `?method=calendar.getEventById` | EventDetailsDto |
| DELETE_EVENT | POST | `?method=calendar.deleteEvent` | bool |
| GET_DELEGATED_ACCESSES | GET | `?method=calendar.getDelegatedAccesses` | List\<DelegateAccesses\> |
| GET_INST_PROF_DELEGATED | GET | `?method=calendar.getInstitutionProfilesWithDelegatedAccess` | List\<InstitutionDelegateAccessesItem\> |
| SET_DELEGATED_ACCESSES | POST | `?method=calendar.setDelegatedAccesses` | bool |
| CHECK_EVENT_CONFLICT | POST | `?method=calendar.checkConflictEventForAttendees` | List\<ConflictEventItem\> |
| RESPOND_TO_EVENT | POST | `?method=calendar.respondToEvent` | bool |
| GET_EVENTS_BY_GROUP | GET | `?method=calendar.geteventsbygroupid` | List\<EventSimpleDto\> |
| CREATE_SIMPLE_EVENT | POST | `?method=calendar.createSimpleEvent` | - |
| CREATE_REPEATING_EVENT | POST | `?method=calendar.createRepeatingEvent` | - |
| CREATE_TIMESLOT_EVENT | POST | `?method=calendar.createTimeSlotEvent` | - |
| UPDATE_SIMPLE_EVENT | POST | `?method=calendar.updateSimpleEvent` | - |
| UPDATE_REPEATING_EVENT | POST | `?method=calendar.updateRepeatingEvent` | - |
| UPDATE_TIMESLOT_EVENT | POST | `?method=calendar.updateTimeSlotEvent` | - |
| UPDATE_LESSON_EVENT | POST | `?method=calendar.updateLessonEvent` | - |
| RESPOND_SIMPLE_EVENT | POST | `?method=calendar.respondToSimpleEvent` | - |
| RESPOND_TIMESLOT_EVENT | POST | `?method=calendar.respondToTimeSlotEvent` | - |
| BLOCK_TIMESLOT | POST | `?method=calendar.blockTimeSlot` | - |
| DELETE_TIMESLOT | POST | `?method=calendar.removeBlockingOrResponseToTimeSlot` | - |
| EDIT_TIMESLOT | POST | `?method=calendar.updateResponseToTimeSlotEvent` | - |
| SET_DELEGATED_CONTEXT | POST | `?method=calendar.setDelegatedContext` | bool |
| GET_DELEGATED_CONTEXT | GET | `?method=calendar.getDelegatedContext` | - |
| GET_DAILY_AGGREGATED | GET | `?method=calendar.getDailyAggregatedEvents` | List\<DailyAggregatedEventsResultModel\> |
| GET_DAILY_GROUP_COUNT | GET | `?method=calendar.getDailyEventCountForGroup` | List\<DailyAggregatedEventsResultModel\> |
| GET_BIRTHDAY_INSTITUTION | GET | `?method=calendar.getBirthdayEventsForInstitutions` | List\<BirthdayEventDto\> |
| GET_BIRTHDAY_GROUP | GET | `?method=calendar.getBirthdayEventsForGroup` | List\<BirthdayEventDto\> |
| GET_EVENTS_FOR_INSTITUTION | GET | `?method=calendar.getEventsForInstitutions` | List\<EventSimpleDto\> |
| RESPOND_VACATION_REG | POST | `?method=calendar.respondToVacationRegistrationRequest` | - |
| GET_FUTURE_VACATION_REQ | GET | `?method=calendar.getFutureVacationRequests` | VacationOverviewListItemResultDto[] |
| GET_EVENT_TYPES | GET | `?method=calendar.getEventTypes` | string[] |
| CREATE_VACATION_REQ | POST | `?method=calendar.createVacationRequest` | - |
| UPDATE_VACATION_REQ | POST | `?method=calendar.updateVacationRequest` | - |
| GET_VACATION_RESPONSES | GET | `?method=calendar.getVacationRequestResponses` | VacationWeekResultDto[] |
| ADD_VACATION | POST | `?method=calendar.addVacation` | - |
| DELETE_VACATION | POST | `?method=calendar.deleteVacation` | - |
| GET_VACATION_BY_ID | GET | `?method=calendar.getVacationById` | VacationDetailsDto |

### Calendar Feed (method=CalendarFeed.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_POLICY_ANSWER | GET | `?method=CalendarFeed.getPolicyAnswer` | CalendarSynchronisationModel |
| SET_POLICY_ANSWER | POST | `?method=CalendarFeed.setPolicyAnswer` | bool |
| GET_FEED_CONFIGS | GET | `?method=CalendarFeed.getFeedConfigurations` | List\<CalendarSynchronisationConfigurationItem\> |
| CREATE_FEED_CONFIG | POST | `?method=CalendarFeed.createFeedConfiguration` | - |
| UPDATE_FEED_CONFIG | POST | `?method=CalendarFeed.updateFeedConfiguration` | - |
| REMOVE_FEED_CONFIG | POST | `?method=CalendarFeed.removeFeedConfiguration` | - |
| GET_EVENT_TYPES_ROLE | GET | `?method=CalendarFeed.getEventTypesRelevantForPortalRole` | GetEventTypesByPortalRoleResultModel |
| GET_FEED_CONFIG_BY_ID | GET | `?method=CalendarFeed.getFeedConfigurationById` | CalendarSynchronisationConfigurationItem |

### Notifications (method=notifications.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_NOTIFICATION_SETTINGS | GET | `?method=notifications.getNotificationSettingsForActiveProfile&includeDeviceTokens=true` | DevicesModel / NotificationSettings |
| UPDATE_NOTIFICATION_SETTINGS | POST | `?method=notifications.setNotificationSettingsForActiveProfile` | bool |
| REGISTER_DEVICE | POST | `?method=notifications.registerDevice` | bool |
| UNREGISTER_DEVICE | POST | `?method=notifications.unregisterDevice` | bool |
| GET_NOTIFICATIONS | GET | `?method=notifications.getNotificationsForActiveProfile` | List\<NotificationItemDto\> |
| DELETE_NOTIFICATIONS | POST | `?method=notifications.deleteNotifications` | bool |
| DELETE_BY_RELATED_CHILD | POST | `?method=notifications.deleteNotificationsByRelatedChild` | bool |
| UNREGISTER_ALL_DEVICES | POST | `?method=notifications.unregisterAllDevices` | bool |
| DELETE_BY_MODULE | POST | `?method=notifications.deleteBadgeNotificationByModule` | bool |

### Gallery (method=gallery.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_MEDIA | GET | `?method=gallery.getMedia` | MediasInAlbumDto |
| GET_MEDIA_BY_INST_PROFILE | GET | `?method=gallery.getMediaByInstitutionProfileId` | - |
| GET_ALBUMS | GET | `?method=gallery.getAlbums` | List\<AlbumDto\> |
| CREATE_ALBUM | POST | `?method=gallery.createAlbum` | int |
| UPDATE_ALBUM | POST | `?method=gallery.updateAlbum` | bool |
| DELETE_MEDIA | POST | `?method=gallery.deleteMedia` | bool |
| CREATE_MEDIA | POST | `?method=gallery.createMedia` | - |
| UPDATE_MEDIA | POST | `?method=gallery.updateMedia` | bool |
| ADD_TAG | POST | `?method=gallery.addTag` | bool |
| REMOVE_TAG | POST | `?method=gallery.removeTag` | bool |
| REPORT_MEDIA | POST | `?method=gallery.reportMedia` | bool |
| GET_MEDIA_BY_ID | GET | `?method=gallery.getMediaById` | AulaGalleryMediaFileResultDto |
| DELETE_ALBUMS | POST | `?method=gallery.deleteAlbums` | bool |

### Presence (method=presence.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_ACTIVITY_OVERVIEW | GET | `?method=presence.getActivityOverview` | GetPresenceOverviewDto |
| GET_PRESENCE_CONFIGURATION | GET | `?method=presence.getPresenceConfiguration` | PresenceConfigurationResultModel |
| GET_PRESENCE_TEMPLATES | GET | `?method=presence.getPresenceTemplates` | GetDayTemplateResultModel |
| UPDATE_PRESENCE_TEMPLATE | POST | `?method=presence.updatePresenceTemplate` | bool |
| GET_PRESENCE_STATES | GET | `?method=presence.getPresenceStates` | GetAvailableStatusesResultModel / List\<ChildStatusDTO\> |
| GET_PRESENCE_DETAIL | GET | `?method=presence.getPresenceRegistrationDetail` | PresenceRegistrationDetailViewModel |
| UPDATE_STATUS_BY_PROFILE_IDS | POST | `?method=presence.updateStatusByInstitutionProfileIds` | bool |
| GET_SUGGESTIONS_PICKUP | GET | `?method=presence.getSuggestedNamesForPickupChild` | List\<ComeGoExitWithSuggestionModel\> |
| SAVE_PICKUP_NAMES | POST | `?method=presence.savePickupNames` | bool |
| GET_PICKUP_RESPONSIBLES | GET | `?method=presence.getPickupResponsibles` | List\<GetPickupResponsibleChildResultModel\> |
| DELETE_PICKUP_RESPONSIBLE | POST | `?method=presence.deletePickupResponsible` | bool |
| GET_ACTIVITY_LIST | POST | `?method=presence.getActivityList` | ActivityListResultModel |
| GET_PRESENCE_CONFIG_BY_CHILD | GET | `?method=presence.getPresenceConfigurationByChildIds` | List\<PresenceConfigurationChildResultModel\> |
| GET_GO_HOME_WITH_LIST | GET | `?method=presence.getGoHomeWithList` | ChildGoHomeWithResultModel[] |
| GET_ACTIVITY_LIST_EDIT_OPTS | GET | `?method=presence.getActivityListEditOptions` | List\<InstitutionWithPresenceStatesResponseDto\> |
| BULK_UPDATE_STATUS | POST | `?method=presence.bulkUpdatePresenceStatus` | bool |
| GET_PRESENCE_FILTERS | GET | `?method=presence.getPresenceFilters` | ActivityFilterResultModel[] / PresenceFilterResultModel[] |
| GET_CHILD_VACATION_LIST | GET | `?method=presence.getChildVacationList` | ChildrenVacationResultModel |
| GET_DAILY_OVERVIEW | GET | `?method=presence.getDailyOverview` | List\<ParentsDailyOverviewResultModel\> |
| DELETE_REPEATING_TEMPLATE | POST | `?method=presence.deleteRepeatingPresenceTemplate` | bool |
| GET_AVAILABLE_LOCATIONS | GET | `?method=presence.getAvailablePresenceLocations` | ComeGoLocationResultModel[] |
| GET_REGISTRATIONS_BY_IDS | GET | `?method=presence.getPresenceRegistrationsByIds` | PresenceRegistrationResultModel[] |
| ADD_SLEEP_INTERVALS | POST | `?method=presence.addSleepIntervals` | long[] |
| DELETE_SLEEP_INTERVALS | POST | `?method=presence.deleteSleepIntervals` | bool |
| UPDATE_LOCATION | POST | `?method=presence.updateLocation` | bool |
| UPDATE_CHECKOUT_ACTIVITY | POST | `?method=presence.updateCheckoutActivity` | bool |
| UPDATE_SLEEP_INTERVAL | POST | `?method=presence.updateSleepInterval` | bool |
| GET_PRESENCE_DISTRIBUTION | GET | `?method=presence.getPresenceDistribution` | PresenceChildrenDistributionDto |
| GET_PRESENCE_REGISTRATIONS | GET | `?method=presence.getPresenceRegistrations` | PresenceRegistrationResultModel[] |
| GET_TEMPLATE_FOR_DATE | GET | `?method=presence.getTemplateForDate` | EmployeeWeekOverviewPresenceDto |
| GET_VACATION_ANNOUNCEMENTS | GET | `?method=presence.getVacationAnnouncementsByChildren` | List\<VacationAnnouncementsByChildrenDto\> |
| GET_VACATION_REGISTRATIONS | GET | `?method=presence.getVacationRegistrationsByChildren` | List\<VacationRegistrationsByChildrenDto\> |
| GET_VACATION_REG_RESPONSE | GET | `?method=presence.getVacationRegistrationResponse` | VacationRegistrationResponseForGuardianDto |
| GET_VACATION_REGISTRATIONS_OV | GET | `?method=presence.getVacationRegistrations` | GetVacationRegistrationOverviewDto |
| GET_OPENING_HOURS | GET | `?method=presence.getOpeningHoursByInstitutionCodes` | GetOpeningHoursByInstitutionCodesDto |
| GET_SPECIFIC_OPENING_HOURS | GET | `?method=presence.getSpecificOpeningHourOverview` | GetSpecificOpeningHourOverviewDto |
| GET_GENERAL_OPENING_HOURS | GET | `?method=presence.getGeneralOpeningHours` | GetGeneralOpeningHoursDto |
| GET_CLOSED_DAYS | GET | `?method=presence.getClosedDays` | GetClosedDaysDTO |
| UPDATE_PRESENCE_REGISTRATION | POST | `?method=presence.updatePresenceRegistration` | bool |
| GET_OVERLAPPING_TEMPLATES | GET | `?method=presence.getOverlappingPresenceTemplates` | DateTimePeriodDto |

### Files (method=files.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_DOWNLOAD_URL | GET | `?method=files.getDownloadUrl` | - |
| CREATE_ATTACHMENTS | POST | `?method=files.createAttachments` | - |
| UPDATE_ATTACHMENTS | POST | `?method=files.updateAttachments` | - |
| COMPLETE_MULTIPART | POST | `?method=files.completeMultipartUploading` | - |
| CREATE_DOCUMENT_LINKS | POST | `?method=files.createDocumentLinks` | List\<DocumentLinkResult\> |

### Documents (method=documents.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_SECURE_DOCUMENTS | POST | `?method=documents.getSecureDocuments` | GetSecureDocumentsResult |
| GET_INTERNAL_SECURE_DOC | GET | `?method=documents.getInternalSecureDocument` | InternalSecureDocumentDetailsDto |
| GET_EXTERNAL_SECURE_FILE | GET | `?method=documents.getExternalSecureFile` | ExternalSecureDocumentDetailsDto |
| UPDATE_SHARINGS | POST | `?method=documents.updateSharings` | bool |
| GET_DOCUMENT_REVISIONS | GET | `?method=documents.getDocumentRevisions` | DocumentRevisionPageDto |
| GET_COMMON_FILES | GET | `?method=commonFiles.getCommonFiles` | GetCommonFilesResult |
| REMOVE_OWN_SHARINGS | POST | `?method=documents.removeOwnSharings` | bool |
| CREATE_INTERNAL_SECURE | POST | `?method=documents.createInternalSecureDocument` | int? |
| UPDATE_INTERNAL_SECURE | POST | `?method=documents.updateInternalSecureDocument` | bool? |
| UPDATE_LOCKED_STATUS | POST | `?method=documents.updateLockedStatus` | bool |
| CREATE_ARCHIVE_MULTIPLE | POST | `?method=documents.createArchiveForMultipleSecureDocuments` | long |
| TRACK_ARCHIVE_MULTIPLE | POST | `?method=documents.trackCreateArchiveForMultipleSecureDocumentsRequest` | SecureDocumentExportDto |
| GET_SHAREABLE_SECURE_DOCS | POST | `?method=documents.getShareableSecureDocuments` | GetSecureDocumentsResult |
| GET_DOCUMENT_REVISION | GET | `?method=documents.getDocumentRevision` | ExternalSecureDocumentDetailsDto / InternalSecureDocumentDetailsDto |
| GET_IMPLICIT_SHARINGS | POST | `?method=documents.getImplicitSharings` | GetImplicitSharingsDto |
| DELETE_DOCUMENT | POST | `?method=documents.deleteDocument` | bool |
| CREATE_PDF_SINGLE | POST | `?method=documents.createPDFForSingleDocument` | long |
| TRACK_PDF_SINGLE | POST | `?method=documents.trackCreatePDFForSingleDocument` | SecureDocumentExportDto |
| GET_MAX_DOCS_PER_EXPORT | GET | `?method=documents.getMaxDocumentsPerExport` | int |

### Personal Reference Data (method=personalReferenceData.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_QUESTION | GET | `?method=personalReferenceData.getPersonalReferenceDataQuestion` | PersonalReferenceDataQuestionResultModel |
| GET_ANSWER | GET | `?method=personalReferenceData.getPersonalReferenceDataAdditionalDataAnswer` | PersonalReferenceDataAnswerResultModel[] |
| GET_CONSENT_ANSWER | GET | `?method=personalReferenceData.getPersonalReferenceDataConsentAnswer` | PersonalReferenceDataAnswerResultModel[] |

### Comments (method=comments.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| ADD_COMMENT | POST | `?method=comments.addComment` | CommentResultModel |
| GET_COMMENTS | GET | `?method=comments.getComments` | PagedCommentList |
| REPORT_COMMENT | POST | `?method=comments.reportComment` | bool |
| DELETE_COMMENT | POST | `?method=comments.deleteComment` | bool |
| UPDATE_COMMENT | POST | `?method=comments.updateComment` | bool |

### Resources (method=resources.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| LIST_RESOURCES | GET | `?method=resources.listResources` | - |
| LIST_AVAILABLE | GET | `?method=resources.listAvailableResources` | - |

### App Logging (method=appLogging.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| BULK_LOG | POST | `?method=appLogging.bulkLog` | bool |

### Aula Token (method=aulaToken.)

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| GET_AULA_TOKEN | GET | `?method=aulaToken.getAulaToken&WidgetId={widgetId}` | string |

### Other

| Endpoint | HTTP Method | URL Suffix | Response Type |
|----------|-------------|-----------|---------------|
| IS_ALIVE | GET | `alivecheck/` | - |
| GOOGLE_API | GET | `https://www.googleapis.com/drive/v3/files` | - |
| ONEDRIVE_API | GET | `https://graph.microsoft.com/v1.0/me/drive` | - |

---

## Key Request/Response JSON Schemas

### StartNewThread (POST messaging.startNewThread)

```json
{
  "message": {
    "text": "string (HTML)",
    "attachmentIds": [123, 456]
  },
  "subject": "string",
  "recipients": [
    { "type": "string (institutionProfile|group|commonInbox)", "id": 123 }
  ],
  "bccRecipients": [
    { "type": "string", "id": 123 }
  ],
  "sensitive": false,
  "creator": { "type": "string", "id": 123 }
}
```

### ReplyToThread (POST messaging.reply)

```json
{
  "threadId": 123,
  "message": {
    "text": "string (HTML)",
    "attachmentIds": [123]
  },
  "commonInboxId": null,
  "bundleId": null
}
```

### CreatePost (POST posts.createPost)

```json
{
  "id": 0,
  "title": "string",
  "content": "string (HTML)",
  "institutionCode": "string",
  "creatorInstitutionProfileId": 123,
  "allowComments": true,
  "isImportant": false,
  "importantFrom": "date string",
  "importantTo": "date string",
  "sharedWithGroups": [
    { "groupId": 123, "groupName": "string" }
  ],
  "attachmentIds": [123, 456],
  "publishAt": "datetime",
  "expireAt": "datetime"
}
```

### GetPosts (GET posts.getAllPosts)

Query parameters:
```
groupId=123
isImportant=true
creatorPortalRole=employee|guardian|child
institutionProfileIds[]=123&institutionProfileIds[]=456
relatedInstitutions[]=ABC
ownPost=true
isUnread=true
isBookmarked=true
limit=10
index=0
```

---

## Architecture Notes

- **API routing**: NOT REST paths. Uses PHP-style `?method=module.action` query parameters.
- **Response envelope**: All responses have `{ "data": ..., "status": { "message": "OK", "code": ... } }`.
- **Authentication**: OAuth2 OIDC via SimpleSAMLphp. Two security levels (step-up auth for sensitive data).
- **CSRF protection**: Uses CSRF-P token, checked on POST requests.
- **HTTP methods**: Despite being an RPC-style API, GET is used for reads, POST for writes/mutations. Some exceptions (e.g., `getEventsByProfileIdsAndResourceIds` uses POST, `getSecureDocuments` uses POST).
- **File uploads**: Multi-part uploads via `files.createAttachments` + `files.completeMultipartUploading`.
- **Cloud storage**: Google Drive v3 and OneDrive Graph API integration for file access.
- **Local cache**: SQLite database (`auladb_13_05_2019`) for caching, with MonkeyCache library.
