# Aula API Data Models & DTOs

**Source**: AulaNative.dll (assembly_187) from com.netcompany.aulanativeprivate v2.15.4
**Framework**: .NET 9.0 / Xamarin / MAUI
**Serialization**: Newtonsoft.Json v13.0.0 (JsonProperty attributes present but not fully decodable by monodis)
**Object mapping**: AutoMapper (DTO <-> domain model conversion)
**Analysis method**: monodis --typedef, --fields, --customattr on extracted .NET assemblies

## Summary

- **604 model/DTO classes** across 20+ domain namespaces
- **136 enum types** with catalogued values
- **109 fields** with explicit `[JsonProperty]` annotations (see Section 4)
- Key domains: Messages, Calendar/Events, ComeGo (presence/attendance), Posts, Gallery, Documents, Groups, Profiles, Notifications, Search

## Table of Contents

1. [Enum Types](#1-enum-types) -- All enum types with values
2. [Model & DTO Classes](#2-model--dto-classes) -- All classes with field names and types
3. [Model Relationships](#model-relationships) -- Entity relationship graph
4. [Serialization Annotations](#serialization-annotations) -- JsonProperty and serialization details

## Namespace Index (Models)

| Namespace | Domain | Key Classes |
|-----------|--------|-------------|
| Models.Calendar | Calendar, events, timeslots, vacations, lessons | EventBaseClass, EventDetailsDto, TimeSlot, VacationDetailsDto |
| Models.ComeGo | Presence/attendance (Come & Go) | ChildStatus, ActivityListResultModel, PresenceConfigurationResultModel |
| Models.MessageThreads | Message threading | MessageThread, MessageThreadSubscription, MessagesInThreadDto, Folder |
| Models.Messages | Individual messages | MessageDto, MessageRecipient, MessageFileUrl |
| Models.Posts.Api | Posts/feed | PostApiDto, CreatePostApiParameter, GetPostApiResult |
| Models.ProfileModels | User profiles | Profile, InstitutionProfile, ChildProfile, Address |
| Models.Groups | Groups/classes | Group, GroupMembership, SimpleGroupDto |
| Models.Document | Secure documents | SecureDocumentDto, DocumentRevisionDto, CommonFileDto |
| Models.Gallery | Photo/media gallery | AlbumDto, MediaListDto, MediaCreatorModel |
| Models.Common.Api.Files | File handling/uploads | AulaFileResultDto, AulaFileContent, UploadFileInfo |
| Models.Search | Global search | SearchResponse, SearchResultItem, various result types |
| Models.Institutions | School/institution data | Institution, Permission, InstitutionIdentity |
| Models.Notification.Api | Notifications | GetNotificationsApiParameter, DeleteNotificationsDto |
| Models.RemoteNotifications | Push notifications | NotificationSettings, DeviceModel, RemoteNotification |
| Models.Consents | GDPR/consent | ConsentResponsesDTO, InstitutionProfileConsentDTO |
| Models.Web | API response wrappers | AulaServiceResponse<T>, DataArrayResponse<T>, AulaErrorResponseWrapper<T> |
| DTOs.ComeGo | ComeGo DTOs | PresenceRegistrationResultModel, ChildStatusDTO |
| DTOs.MasterData | Profile master data | MasterDataDTO, PostMasterDataDto |
| DTOs.Logging | Error logging | LogErrorAdditionalParameterDto, LogErrorExceptionDto |

---

## 1. Enum Types

### DTOs.Calendar.Vacation

**VacationResponseStatusEnum**: IsComing, IsNotComing, PendingAnswer


### DTOs.ComeGo.PresenceDay

**PresenceTemplateRepeatPattern**: Never, Weekly, Every2Weeks


### Enums

**AdditionalMasterDataResponseAnswerTypeEnum**: YesNo, PhoneNumber, Text

**AppTypeEnum**: Staff, Private, Unknown

**AssociationModeEnum**: None, Select, Confirm

**AulaFilePickerEnum**: Files, MediaLibrary, GoogleDrive, OneDrive, PhotoCamera, VideoCamera, AulaGallery, Document, DownloadMediaGoogleDrive, DownloadMediaOneDrive, FilesForMedia, AttachFileGoogleDrive, AttachFileOneDrive, All

**CacheType**: SMALL, LARGE

**FilterAndSortType**: FilterAll, FilterUnread, FilterMarked, FilterDraft, SortDate, SortSubject, SortCreatedDate, SortMediaCreatedDate, SortMediaCreatedAt, FilterMyAlbums, FilterMyMedia, SortAlbumName

**GroupRole**: Unknown, Member, Indirect, Applied, Removed, Rejected, Inactive

**GroupStatus**: Unidentified, Active, Inactive

**InstitutionRole**: Unknown, Guardian, Daycare, Leader, PreschoolTeacher, Teacher, EarlyStudent, MiddleLateStudent, Child, Other

**InstitutionTypeEnum**: Unknown, School, Daycare, Municipality, Central

**LoadingType**: LoadMore, Action, Refresh

**LogLevel**: All, Trace, Debug, Info, Warning, Error, Fatal

**LoginAuthenticationMethod**: Unknown, Level2, Level3NemId, Level3Employees

**MessageThreadClickType**: ItemClick, Move, Mark, Delete, MultiMove, MultiMark, MultiDelete

**MessageType**: AllMessageRelatedType, Message, RecipientAdded, RecipientRemoved, AutoReply, SystemForward, SystemReply, Forward, Other, RecipientsAdded, RecipientsRemoved, MessageDeleted, MessageEdited, SystemForwardSingleMessage

**Platform**: Android, iOS, Unknown

**PortalRole**: Other, Employee, Child, Guardian, Otp

**RecipientType**: Profile, Group, CommonInbox, Unknown

**RecipientsTarget**: MessageRecipients, MessageBccRecipients, CalendarEvent, SecureDocument, Post

**ReportEnum**: Post, Media, Comments, Unknown

**ResourceType**: Location, Other, ExtraLocation, Electronics, Stationery

**RotatingEnum**: Rotating0, Rotating90, Rotating180, Rotating270

**SensitivityLevel**: Level1, Level2, Level3

**SortOrderEnum**: Unknown, Ascending, Descending

**SubscriptionStatus**: Read, Unread

**ThumbnailSizeEnum**: XS, S, M, L, Full

**TimePeriod**: None, TwoWeeks, OneMonth, ThreeMonths, SixMonths, OneYear

**UserRelationType**: Others, Child, Guardian, Otp, Teacher

**WeekDay**: Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday


### Enums.App

**BottomBarLongPressOption**: EditShortcuts

**FrontPageSettingConfigurationEnum**: ActivityFeed, Messages, CalendarOverview, ImportantDates, Document, ComeGo, Gallery, ContactList, PersonalReferenceData


### Enums.BioAuth

**BioAuthStatus**: CanTryAgain, CanNotTryAgain, Canceled, Accepted


### Enums.BlockedCommunication

**BlockedLevel**: Central, Municipal, Institution, Unknown


### Enums.Calendar

**EventClass**: Basic, Series, Timeslot, Lesson, Unknown

**EventPlacementComparedToDateTime**: NotOnTheDate, StartAndEndOnDate, StartOnDateButEndAfter, StartBeforeDateButEndOn, StartBeforeAndEndAfterDate

**EventPortraitType**: Event, Birthday, AllDay

**EventType**: Event, Holiday, PresenceHoliday, VacationRegistration, Birthday, Meeting, Other, Excursion, SchoolHomeMeeting, ClassMeeting, ParentalMeeting, PerformanceMeeting, Lesson, Unknown

**LessonStatus**: Cancelled, Normal, Absent, Substitute, ToBeDeleted, WillBeUpdated, StatusNotFound

**ParticipantRole**: PrimaryTeacher, SubstituteTeacher, HelpTeacher, Pedagogue, NotChosen

**RepeatType**: Never, Daily, Weekly, Monthly

**RepeatingEventDropdownEnum**: ForSeries, ForSingleOccurrence

**ResponseType**: Waiting, Declined, Accepted, Tentative

**TimeslotResponseType**: Blocked, NotBooked, AlreadyBooked

**VacationRegistrationResponseStatus**: Answered, Unanswered


### Enums.CloudStorageIntegration

**CloudStorageFileType**: Folder, Docs, Sheets, Excel, Slides, PowerPoint, Video, Sound, File, Pdf, Image, Unknown

**CloudStorageService**: Unknown, GoogleDrive, OneDrive


### Enums.ComeGo

**ActivityTypeEnum**: PICKED_UP_BY, SELF_DECIDER, SEND_HOME, GO_HOME_WITH, DROP_OFF_TIME, SPARE_TIME, CHECK_IN, CHECK_OUT, SLEEPING, All

**ComeGoNotificationEnum**: ALERT_RESPONSE_NOTIFICATION, ALERT_INVITE_NOTIFICATION, VACATION_RESPONSE_NOTIFICATION

**ComeGoStaffTabEnum**: ActivityList, WeekOverview, VacationRegistrationOverview, OpeningHoursAndClosedDays

**ComeGoTabEnum**: AbsenceTab, TimeTab, DailyOverview, PickupResponsible, OpeningHoursAndClosedDaysInstitutionListPage, OpeningHoursAndClosedDays, PlanningPage

**DepartureTypeEnum**: GoGomeWith, RetrieveResponsible

**OpeningHoursType**: SpecificOpeningHours, GeneralOpeningHours, DefaultOpeningHours, ClosedDay

**PresenceDayOfWeek**: Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday

**PresenceModuleSettingsDashboardContext**: EmployeeDashboardSettings, CheckinDashboardSettings, GuardianDashboardSettings

**PresenceModuleSettingsModule**: DropOffTime, Location, Sleep, FieldTrip, PickupType, PickupTimes, DailyMessage, Vacation, ReportSick, SpareTimeActivity

**PresenceModuleSettingsPermission**: Editable, Deactivated, Readable

**PresenceStatusEnum**: NotPresent, Sick, ReportedAbsence, Present, FieldTrip, Sleeping, SpareTimeActivity, PhysicalPlacement, CheckedOut, NotArrived, All

**PresenceTemplateEditingOption**: EditSingleDay, EditWholeTemplate, Delete

**SpareTimeActivityAction**: Edit, Delete

**TokenStatusEnum**: Used, NotUsed, Expired


### Enums.ComeGo.EmployeeWeekOverview

**ComGoEmployeeWeekOverviewTenseEnum**: past, present, notSpecified, missingCheckout

**ComeGoEmployeeWeekOverviewFilterOptionEnum**: Present, Vacation, Sick, NotArrived

**ComeGoEmployeeWeekOverviewPresenceTypeEnum**: Present, Vacation, Sick, NotArrived, None


### Enums.Comments

**CommentDropDownEnumeration**: Delete, Edit, Report

**CommentType**: Comment, Media, Post, Unknown


### Enums.Consents

**Consent**: SHARE_CONTACT_INFORMATION_PARENT, SHARE_CONTACT_INFORMATION_CHILD, OTHERS

**ConsentAnswerEnum**: Accepted, Declined, Class, Year, Institution, NotAtAll, Other

**Status**: Active, Deactive, Pending


### Enums.Document

**CommonFileSortEnum**: Title, UpdatedTime

**DocumentCategoryEnum**: Agenda, AgendaAllUser, PlanOfAction, Setting, ForCableSchedule, Observation, EducationalNote, Summary, SummaryAllUser, ScratchScheme, OpenTemplate, OpenTemplateAllUser, Note, Unknown

**DocumentFilterEnum**: All, Unread, Locked, Published, PublishInProgress, PublishFailed

**DocumentOverviewPageTypeEnum**: SecureDocument, CommonFiles, OneDrive, GoogleDrive

**DocumentTypeEnum**: Unknown, External, Internal, Note, Richdocument

**ImplicitSharingPermissionOverride**: Read, Write, NoAccess

**JournalingStatusEnum**: NotProcessed, InProgress, Failed, Completed

**RevisionChangeTypeEnum**: Unshared, Shared, Unlocked, Locked, Edited, Exported, Created, PermissionAdded, PermissionRemoved, ImplicitUnshared, Deleted, Restored, JournalizedToESDH, SentToESDH, EsdhJournalizationFailed, ResentToESDH, ManuallyJournalizedToESDH, MarkForManualJournalize

**SecureDocumentExportStatus**: Created, Processing, Failed, Completed, Unknown

**SecureDocumentSelectingMode**: Attachment, SecureDocumentExport, Normal, RemoveAssociation

**SecureDocumentSortEnum**: Unknown, Title, UpdatedAtDate


### Enums.File

**FileAttachmentErrorType**: UnsupportedFileType, FileSizeTooBig, MediaSizeTooBig, GenericError

**FileScanningStatus**: Available, Blocked, Processing, Bypassed

**FileStatusEnum**: Available, Pending, Unavailable, Unknown


### Enums.Gallery

**ConversionStatusEnum**: Completed, Processing, Failed

**DocumentChangeType**: Create, Update, Delete

**GalleryDropDownEnumeration**: Download, Delete, Report, EditTags, ViewInfo

**ImageSizeEnum**: Original, Max200, Max400

**MediaCellType**: TagCell, TaggedCell, NonTagCell, EmptyCell

**MediaSelectManyAction**: AddTags, Download, Delete, EditInfo, RotateRight


### Enums.Groups

**GroupActionType**: Leave, Join

**GroupMembershipRole**: Other, Applied, Member, Removed, Application_Removed, Indirect

**GroupTypeEnum**: Unknown, Institutional, Municipal, Cross_institutional, Other

**GroupsAccesType**: Other, Closed, Open, Application


### Enums.Login

**UpdateProfileInformationReturnCodeEnum**: Success, Error, ErrorUserDeactivated, ErrorUserAccessDenied, WrongUserTypeLoggedInAsEmployee, WrongUserTypeLoggedInAsGuardian, WrongUserTypeLoggedInAsChild, WrongUserTypeLoggedInAsOTP


### Enums.Messages

**BundledMessageType**: IsRegularMessage, FirstMessage, MiddleMessage, LastMessage, PrimaryMessage, SecondaryMessage, LastOfSecondaryMessage

**CommonInboxType**: Institutional, CrossInstitutional

**MessageFormType**: StartNewThread, ReplyInThread, Forward, StartNewThreadWithUser, ReplyInThreadFromAnswerOptionsButton, ForwardSingleMessage

**MessageThreadCellMoreMenuActionEnum**: MoveToFolder, MarkAsImportant, Forward

**SendMessageButton**: REPLY_SINGLE, REPLY_ALL

**SubscriptionType**: Bundle, BundleItem, Unbundled


### Enums.Messages.ThreadDetails

**DropdownActionEnum**: AddRecipient, Forwarding, ToggleMute, Leave, ToggleSensitive, ExportThreadToDocument, MarkAsImportant, MoveToFolder, Delete, ToggleReadStatus, CreateDocument

**MessageMoreOption**: Edit, Delete, Forward


### Enums.Notifications

**NotificationArea**: Unknown, Messages, Calendar, Posts, Schedule, Administration, Gallery, Documents, Album, Presence, Widget, FileScanning

**NotificationEventType**: Other, NewMessagePrivateInbox, NewSensitiveMessagePrivateInbox, InvitedToEventNoResponseRequired, InvitedToEventResponseRequired, EventChangedResponseRequired, EventChangedNoResponseRequired, EventCancelledNoReason, InvitedToParentalMeeting, InvitedToSchoolHomeMeeting, InvitedToPerformanceMeeting, GrantedRightsForOtherCalendar, RemovedRightsForOthersCalendar, LostRoomBecauseOfExternalScheduling, SomeoneElseRemovedYourResponseToEvent, SomeoneElseRespondedToEvent, PostSharedWithMe, PostWasRemovedFromGroupByAdmin, PostWasDeleted, SubstituteAdded, DashboardWasUpdated, NewMedia, TaggedInMedia, TaggedInMediaWithoutPushNotification, MediaAddedToAlbum, AlbumShared, NewOrUpdatedSecureDocument, NewCommonFile, VacationResponseRequired, InvitedToRepeatingEvent, PresenceRegistrationUpdatedExitWith, UpdatedSharingCommonFile, NewOrUpdatedCommonFile, VacationRequestCancelledNoReason, NewMessageCommonInbox, NewSensitiveMessageCommonInbox, AddedToCommonInbox, RemovedFromCommonInbox, LessonNoteChanged, EventChangedBySomeoneElse, EventCancelledBySomeoneElse, WidgetNotification, InvitedToExceptionEvent, InvitedToSingleOccurrenceOfEvent, SingleOccurrenceEventCancelledNoReason, InvitedToSurvey, DeletionRulesInfo, ResourceReservationDeclined, GeneralInformationSent, NewPostComment, OSOutdated, FileScanFailedPost, FileScanFailedEvent, FileScanFailedPrivateInboxMessage, FileScanFailedCommonInboxMessage, FileScanFailedInternalSecureDocument, FileScanFailedExternalSecureFile, FileScanFailedAlbum, FileScanFailedProfilePicture, FileScanFailedCommonFile

**NotificationEventTypeCategories**: AlertInviteWithResponse, AlertChangedWithResponse, AlertChangedWithoutResponse, AlertInviteAssignedAsSubstituteTeacher, AlertInviteGoToAfterClickAndShowCross, AlertInvite, NotificationWithoutAlert, Posts, Messages, Schedule, Other, AlertSurveyInvite, AlertDeletionRulesInfo, AlertResourceReservationDeclined, AlertGeneralInfo, AlertOSOutdated, FileScanFailed

**NotificationSettingsEnum**: ContentSettingsSection, PlatformSettingsSection, MessageSubsection, CalendarSubsection, MediaSubsection, PostsSubsection, ComeGoSubsection, WidgetsSubsection, EmployeeAllowed, GuardianAllowed, ChildAllowed, CalendarAllowed, NotifySubstituteTeacherAllowed, NewMediaAllowed, NewPostsAllowed, VacationRegistrationRequestAllow, PickupActivityAllow, NotifyLessonsChangeAllowed, WidgetsAllowed, MobileNotAvailableText, MobileNotAvailableInfo, MobileSubsection, EmailSubsection, ConsentSubsection, MobileAllowed, EmailAllowed, Instant, DatePicker, AllDay, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday, SetTime, SetDateInWeek, EmailViews

**NotificationType**: Badge, Alert, Irrelevant, Unknown

**RemoteNotificationType**: PostSharedWithMe, NewMessagePrivateInbox, NewSensitiveMessagePrivateInbox, InvitedToEventNoResponseRequired, InvitedToEventResponseRequired, InvitedToRepeatingEventResponseRequired, EventChangedResponseRequired, EventChangedNoResponseRequired, EventCancelledNoReason, InvitedToParentalMeeting, InvitedToSchoolHomeMeeting, InvitedToPerformanceMeeting, GrantedRightsForOtherCalendar, RemovedRightsForOthersCalendar, LostRoomBecauseOfExternalScheduling, SomeoneElseRespondedToEvent, SomeoneElseRemovedYourResponseToEvent, PostWasRemovedFromGroupByAdmin, PostWasDeleted, SubstituteAdded, DashboardWasUpdated, AlbumShared, MediaAddedToAlbum, TaggedInMedia, NewMedia, NewCommonFile, NewOrUpdatedSecureDocument, PresenceRegistrationUpdatedExitWith, VacationResponseRequired, VacationResponseCancelledNoResponse, NewMessageCommonInbox, NewSensitiveMessageCommonInbox, Unknown, LessonNoteChanged, EventChangedBySomeoneElse, EventCancelledBySomeoneElse, WidgetPushNotification, InvitedToExceptionEvent, InvitedToSingleOccurrenceOfEvent, InvitedToSurvey, GeneralInformation, NewPostComment, FileScanFailedPost, FileScanFailedEvent, FileScanFailedPrivateInboxMessage, FileScanFailedCommonInboxMessage, FileScanFailedInternalSecureDocument, FileScanFailedExternalSecureFile, FileScanFailedAlbum, FileScanFailedProfilePicture


### Enums.Onboarding

**OnboardingStep**: AppOnboarding, PolicyAcceptance, MasterData, Consents, AdditionalMasterData, NotificationSettings


### Enums.Posts

**PostDetailMoreMenuEnum**: ReportPost

**PostFilterTypeEnum**: All, Unread, IsImportant, FromStaff, FromParents, FromStudents, OwnPost, Bookmarked


### Enums.Profile

**ContactListFilteringProfileTypeEnum**: AllChildren, Boy, Girl, Employee, Guardian

**ProfileRoleGenderEnum**: Boy, Girl, Unknown


### Enums.Profile.ContactList

**GetProfileContactSortOrderFieldEnum**: Birthday, Name


### Enums.Profile.MasterData

**GetPersonalReferenceDataOrderFieldEnum**: Answers, DisplayName


### Enums.Search

**SearchProfileDocTypeEnum**: Profile, Group, CommonInbox, All

**SearchProfilePortalRoleEnum**: Employee, Child

**SearchResultItemType**: None, Group, Profile, Child, Employee, Guardian, InternalSecureFile, ExternalSecureFile, CommonFile, Event, Post, CommonInbox, Message, Thread, ThreadMessage, Media


### Enums.Widget

**WidgetPlacementEnum**: OwnPage, RightOfOverview, RightOfCalendar, BelowCalendar, OnOverview, OnCalendar


### Models.Calendar

**RelationMode**: ChildMode, Institution


### Models.Calendar.Event

**ItemType**: Event, Title, Birthday


### Models.Calendar.MyCalendar

**MyCalendarItemType**: Body, Title


### Models.Common.Api.Files.Enums

**FileTypeEnum**: Unknown, Media, File, ExternalFile

**MediaTypeEnum**: Unknown, Image, Video, Sound, MediaWithDuration, Media


### Models.Institutions

**PermissionEnum**: ADMIN_MODULE, SEARCH_ACCESS_PROFILES, SEARCH_ACCESS_GROUPS, HANDLE_GROUP, HANDLE_INTERINSTITUTIONAL_GROUPS, HANDLE_USER_ROLES, HANDLE_REPORTS_OF_POSTS, DELETE_POSTS_MEDIA_COMMENTS, HANDLE_CALL_TIMES, HANDLE_RESOURCES_INSTITUTION, HANDLE_ADDITIONAL_MASTER_DATA, HANDLE_PHYSICAL_LOCATION, HANDLE_IMPORTANT_FILES, HANDLE_SHARED_INBOX, HANDLE_USER_DATA, HANDLE_ALLOWED_RECIPIENTS, HANDLE_COMMUNICATION_CHANNELS_MUNICIPALITY, HANDLE_ADMINISTRATIVE_AUTHORITY, HANDLE_GROUPINGS_OF_INSTITUTIONS, HANDLE_RIGHTS_TO_PHYSICAL_LOCATION, HANDLE_RESOURCE_CATEGORY, HANDLE_ADDITIONAL_MASTER_DATA_BRUTTO_LIST, HANDLE_CONSENTS, HANDLE_LESSON_IMPORT_TIME, HANDLE_MAX_FILESIZE, HANDLE_FILEFORMATS, ACCESS_SECURE_FILESHARING, HANDLE_SECURE_FILES, HANDLE_SECURE_FILES_LIMITED, ACCESS_IMPORTANT_FILES, ACCESS_SHARED_INBOX, READ_MESSAGE, WRITE_MESSAGE, READ_POST, WRITE_POST, SHARE_MEDIA, SEE_MEDIA, HANDLE_GROUP_APPLICATION, WRITE_COMMENTS, SEE_CALENDAR, READ_EVENTS, HANDLE_EVENTS, INVITE_GROUP_TO_EVENT, HANDLE_PARENTAL_MEETING_SCHOOL, HANDLE_PERFORMANCE_MEETING, BOOK_RESOURCES, INVITE_TO_EVENT, ANSWER_EVENT_WITH_EXTENDED_ANSWER, SHARE_SECURE_FILES, HANDLE_COMMUNICATION_CHANNELS_CENTRAL, HANDLE_TRANSITION_YEAR, HANDLE_RESOURCES_MUNICIPALITY, SEND_SMS, WRITE_INFO_PROFILE, MESSAGE_ATTACH_BCC_RECIPIENTS, HANDLE_PARENTAL_MEETING_DAYCARE, INBOX_SET_PERSONAL_AUTOREPLY, INBOX_FOLDERS, MESSAGE_SEE_SUBSCRIBERS_LASTREAD, HANDLE_CONSENT_AGE, HANDLE_DASHBOARD, REPORT_POSTS_MEDIA_COMMENTS, SEE_GUARDIAN_CHILD_CONTACT_INFORMATION, SEE_EMPLOYEE_CONTACT_INFORMATION, SEE_GUARDIAN_CHILD_LAST_LOGIN, SEE_EMPLOYEE_LAST_LOGIN, HANDLE_CONTACTS, VIEW_USERS_ADDITIONAL_DATA, HANDLE_SIGNATURE, VIEW_MEDIA_REGARDLESS_OF_CONSENT, VIEW_CONTACT_INFORMATION_REGARDLESS_OF_CONSENT, HANDLE_MAX_IMAGE_RESOLUTION, HANDLE_MAX_VIDEO_LENGTH, VIEW_USERS_CONSENTS, TAG_OTHER_USERS_ON_OTHER_MEDIA, ATTACH_GOOGLE_DRIVE_FILE, IMPORT_MEDIA_FROM_GOOGLE_DRIVE, ATTACH_ONEDRIVE_FILE, IMPORT_MEDIA_FROM_ONEDRIVE, SEE_CONTACT_PARENTS_CONTACT_INFO, HANDLE_MEDIA, IMPERSONATE_USER, VIEW_EMPLOYEES_ADDITIONAL_DATA, HANDLE_SERVICE_MESSAGES, PAIRING_INSTITUTION_AND_DEVICE, VIEW_PRESENCE_STATISTICS, HANDLE_VACATION_REQUESTS, HANDLE_OPTIONS_PRESENCE_DASHBOARD, USE_GROUPS_AS_DISTRIBUTION_LISTS, VIEW_NAME_PROTECTION, VIEW_CUSTODY, ACCESS_SKOLEINTRA_ARCHIVE, SKOLEINTRA_ADMIN, SECURE_DOCUMENTS_ACCESS_ALL, CREATE_EVENTS_IN_INSTITUTION_CALENDAR, CREATE_EVENTS_ONLY_IN_INSTITUTION_CALENDAR, VIEW_PERSONAL_REFERENCE_DATA_FOR_ALL_CHILDREN_AND_GUARDIAN, VIEW_CONTACT_INFORMATION_ALL, HANDLE_GROUP_TEMPLATES, HANDLE_NOTICE_BOARDS, HANDLE_ACCESS_CONTACT_INFO, HANDLE_DATA_POLICY, HANDLE_SERVICE_REQUEST, HANDLE_CALENDAR_FEED_MUNICIPALITY, EXPORT_PRESENCE_STATISTICS, EXPORT_SECURE_FILES, READ_SECURE_FILES, EDIT_PRESENCE_TEMPLATES, HANDLE_EVENT_CO_ORGANIZER, HANDLE_OTHERS_EVENTS, EDIT_SHARED_ALBUMS, EDIT_SHARED_MEDIA, JOURNALING_TO_ESDH


### Models.MessageThreads

**ThreadType**: Thread, EventReminder, VacationRequestReminder


### Models.MessageThreads.Argument

**RecipientApiType**: Unknown, InstitutionProfile, CommonInbox, OtpInbox


### Models.MessageThreads.Folders

**FolderType**: Normal, Deleted, ButtonCell


### Models.RemoteNotifications

**ComeGoType**: PickupActivity, VacationRegistrationRequest


### Models.Search.RequestModels

**SearchRecipientParameters/GroupSearchScopeEnum**: Institutional, AdministrativeAuthority, CrossInstitutional, Municipal

**SearchRecipientParameters/SearchRecipientDocTypeEnum**: Profile, Group, CommonInbox, All

**SearchRecipientParameters/SearchRecipientMailBoxOwnerType**: InstitutionProfile, CommonInbox, OtpInbox

**SearchRecipientParameters/SearchRecipientModuleEnum**: Event, Messages, Overview, Gallery, SecureDocument, PersonalReferenceData, Contacts

**SearchRecipientParameters/SearchRecipientPortalRoleEnum**: Employee, Child, Guardian, Otp, All

**SearchResourceParameters/SearchResouceTypeEnum**: Location


---

## 2. Model & DTO Classes

### DTOs

#### DynamicContractResolver : 0x195

*(no public properties detected)*

#### GetGroupMembershipDto

| Property | Type |
|----------|------|
| GroupId | `int64` |
| Memberships | `List<GroupMembership>` |

#### LogErrorParameterDto

| Property | Type |
|----------|------|
| Platform | `string` |
| Message | `string` |
| LogLevel | `int32` |
| AdditionalData | `string` |

#### MessageSeenBy

| Property | Type |
|----------|------|
| MessageId | `string` |
| ShowSeenBy | `bool` |
| LongString | `List<string>` |
| IsHidden | `bool` |
| NumberOfRecipients | `int32` |
| NumberOfUsersAbleToSeeMessage | `int32` |
| NumberOfSeenUsers | `int32` |

#### People

| Property | Type |
|----------|------|
| people | `List<Person>` |

#### PossibleSender

| Property | Type |
|----------|------|
| Address | `string` |
| DisplayName | `string` |
| Relation | `string` |
| ShortName | `string` |

#### ProfilePictureDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Key | `string` |
| Bucket | `string` |
| IsImageScalingPending | `bool` |
| Url | `string` |
| ScanningStatus | `FileScanningStatus` |

#### RegisterDeviceDto

| Property | Type |
|----------|------|
| DeviceId | `string` |
| PushNotificationToken | `string` |
| Os | `string` |
| NotificationsActive | `bool` |
| AppType | `string` |
| Description | `string` |

#### UnregisterDeviceDto

| Property | Type |
|----------|------|
| DeviceId | `string` |

### DTOs.AddRecipientToThreadDTOs

#### Recipient

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Role | `string` |
| Email | `string` |
| InstitutionCode | `string` |
| DisplayName | `string` |
| InstitutionProfileId | `int64` |
| Relations | `List<RecipientRelation>` |

### DTOs.AdditionalMasterData

#### PostInstitutionProfileAdditionalMasterDataDTO

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| AdditionalDataResponses | `List<PostResponsAdditionalMasterDataDTO>` |

#### PostResponsAdditionalMasterDataDTO

| Property | Type |
|----------|------|
| AdditionalDataId | `Nullable<int32>` |
| AdditionalDataResponseId | `Nullable<int32>` |
| Response | `string` |
| YesNoResponse | `Nullable<bool>` |

### DTOs.App

#### FrontPageSettingsDbModel

| Property | Type |
|----------|------|
| Messages | `bool` |

### DTOs.BlockedCommunication

#### BlockedUsersByChannelDTO

| Property | Type |
|----------|------|
| PortalRole | `PortalRole` |
| BlockingLevel | `BlockedLevel` |
| InstitutionProfile | `BlockedCommunicationInstitutionProfile` |
| OtpInbox | `BlockedCommunicationOtpInbox` |
| BlockedGroup | `BlockedCommunicationGroup` |

#### SharedWithGroupRequest

| Property | Type |
|----------|------|
| PortalRoles | `List<PortalRole>` |
| GroupId | `int64` |

### DTOs.Calendar

#### DelegatedContextResultModel

| Property | Type |
|----------|------|
| FullName | `string` |
| NullableInstitutionProfileId | `Nullable<int32>` |
| InstitutionCode | `string` |
| ProfilePicture | `ProfilePictureDto` |
| Role | `PortalRole` |

#### GetEventTypeRequestDto

| Property | Type |
|----------|------|
| FilterInstitutionCodes | `string[]` |

#### HtmlDto

| Property | Type |
|----------|------|
| Html | `string` |

#### SetDelegatedContextRequestModel

| Property | Type |
|----------|------|
| DelegatedInstProfileId | `Nullable<int64>` |

### DTOs.Calendar.Vacation

#### CheckVacationRequestAnsweredRequestModel

| Property | Type |
|----------|------|
| VacationRegistrationResponseId | `int64` |

#### GetVacationRequestResponseRequestModel

| Property | Type |
|----------|------|
| VacationRequestId | `int64` |
| FilterDepartmentGroupIds | `int64[]` |
| FilterDepartmentFilteringGroupIds | `int64[]` |

#### RespondToVacationRegistrationRequestRequestDto

| Property | Type |
|----------|------|
| ChildId | `Nullable<int64>` |
| VacationRegistrationResponseId | `Nullable<int64>` |
| Days | `GuardianRegisterVacationIntervals[]` |
| Comment | `string` |

#### RespondToVacationRegistrationRequestRequestReceivedDto

| Property | Type |
|----------|------|
| ChildId | `Nullable<int64>` |
| VacationRegistrationResponseId | `Nullable<int64>` |
| Days | `GuardianRegisterVacationIntervalsReceiveDto[]` |
| Comment | `string` |

#### VacationChildrenDto

| Property | Type |
|----------|------|
| Child | `ChildMetadata` |
| Status | `VacationResponseStatusEnum` |
| VacationRegistrationResponseId | `int64` |

#### VacationDayDto

| Property | Type |
|----------|------|
| Date | `DateTime` |
| Children | `List<VacationChildrenDto>` |

#### VacationOverviewListItemResultDto

| Property | Type |
|----------|------|
| Title | `string` |
| Id | `int64` |
| InstitutionName | `string` |

#### VacationOverviewListRequestDto

| Property | Type |
|----------|------|
| FilterInstitutionCalendarCodes | `List<string>` |

#### VacationRegistrationChildrenCountByDates

| Property | Type |
|----------|------|
| Date | `DateTime` |
| ChildrenAreComing | `int32` |
| Total | `int32` |

#### VacationRegistrationDetailsResultDto

| Property | Type |
|----------|------|
| ChildrenTotal | `int32` |
| ChildrenPendingAnswers | `List<ChildMetadata>` |
| VacationChildrenCountByDates | `List<VacationRegistrationChildrenCountByDates>` |
| NoteToGuardians | `string` |
| Departments | `List<PresenceFilterDepartmentModel>` |

#### VacationWeekResultDto

| Property | Type |
|----------|------|
| FromDate | `DateTime` |
| ToDate | `DateTime` |
| WeekNumber | `int32` |
| VacationDays | `List<VacationDayDto>` |

### DTOs.CloudStorage

#### GoogleDriveGetDTO

| Property | Type |
|----------|------|
| NextPageToken | `string` |
| Files | `List<GoogleDriveGetFileDTO>` |

#### GoogleDriveGetFileDTO

| Property | Type |
|----------|------|
| Id | `string` |
| Name | `string` |
| MimeType | `string` |
| WebViewLink | `string` |
| AccessToken | `string` |

#### OneDriveGetChildDTO

| Property | Type |
|----------|------|
| Id | `string` |
| Name | `string` |
| AccessToken | `string` |
| File | `OneDriveGetFileDTO` |

#### OneDriveGetDTO

| Property | Type |
|----------|------|
| NextPageUrl | `string` |
| Children | `List<OneDriveGetChildDTO>` |

#### OneDriveGetFileDTO

| Property | Type |
|----------|------|
| MimeType | `string` |

### DTOs.ComeGo

#### BulkUpdatePresenceStatusRequestDto

| Property | Type |
|----------|------|
| PresenceRegistrationIds | `int64[]` |
| Status | `Nullable<PresenceStatusEnum>` |

#### ChildStatusDTO

| Property | Type |
|----------|------|
| UniStudentId | `int64` |
| UniStudent | `ComeGoUniStudentProfile` |
| State | `PresenceStatusEnum` |

#### ComeGoExitWithSuggestionModel

| Property | Type |
|----------|------|
| PickupName | `string` |
| UniStudentId | `int64` |

#### ComeGoExitWithSuggestionRequestModel

| Property | Type |
|----------|------|
| PickupName | `string` |
| UniStudentIds | `int64[]` |

#### ComeGoLocationResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Symbol | `string` |

#### DateTimePeriodDto

| Property | Type |
|----------|------|
| StartDate | `DateTime` |
| EndDate | `Nullable<DateTime>` |

#### GetAvailableStatusesResultModel

| Property | Type |
|----------|------|
| AvailableStatus | `List<AvailablePresenceStatusViewModel>` |

#### GetDayTemplateResultModel

| Property | Type |
|----------|------|
| CurrentDate | `DateTime` |
| PresenceWeekTemplates | `List<ChildComeGoInfoViewModel>` |

#### GetExitWithSuggestionsResultModel

| Property | Type |
|----------|------|
| Suggestions | `List<ComeGoExitWithSuggestionModel>` |

#### GetGeneralOpeningHoursDto

| Property | Type |
|----------|------|
| InstitutionOpeningHours | `List<InstitutionOpeningHours>` |

#### GetSpecificOpeningHourOverviewDto

| Property | Type |
|----------|------|
| SpecificOpeningHoursWithInstitutions | `List<SpecificOpeningHourWithInstitutionDto>` |

#### IPresenceDto

| Property | Type |
|----------|------|
| Id | `Nullable<int64>` |
| EntryTime | `Nullable<DateTime>` |
| ExitTime | `Nullable<DateTime>` |
| ExitWith | `string` |
| ByDate | `DateTime` |
| Comment | `string` |
| IsDefaultEntryTime | `bool` |
| IsDefaultExitTime | `bool` |
| ActivityType | `Nullable<ActivityTypeEnum>` |
| SelfDeciderStartTime | `Nullable<DateTime>` |
| SelfDeciderEndTime | `Nullable<DateTime>` |
| SpareTimeActivity | `SpareTimeActivityDTO` |

#### InstitutionOpeningHours

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| OpeningHours | `List<OpeningHours>` |

#### InstitutionWithPresenceStatesResponseDto

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| PresenceStates | `PresenceStatusEnum[]` |

#### OpeningHours

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| DayOfWeek | `PresenceDayOfWeek` |
| OpenTime | `string` |
| CloseTime | `string` |

#### ParentDailyOverviewInstitutionProfileDto

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Id | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Role | `PortalRole` |
| Name | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |
| MainGroup | `MainGroup` |
| ShortName | `string` |
| InstitutionRole | `InstitutionRole` |
| Metadata | `string` |

#### PhysicalLocationResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

#### PresenceChildrenDistributionRequestDto

| Property | Type |
|----------|------|
| DepartmentId | `int64` |
| Date | `string` |
| GroupIds | `int64[]` |
| StatusFilters | `string[]` |

#### PresenceFiltersRequestDto

| Property | Type |
|----------|------|
| Institutions | `List<string>` |

#### PresenceLocationDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Description | `string` |
| Symbol | `string` |

#### PresenceRegistrationResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionProfile | `SimpleInstitutionProfile` |
| Status | `PresenceStatusEnum` |
| ActivityType | `Nullable<ActivityTypeEnum>` |
| Location | `ComeGoLocationResultModel` |
| SleepIntervals | `SleepIntervalResultModel[]` |
| EditablePresenceStates | `PresenceStatusEnum[]` |
| CheckInTime | `string` |
| CheckOutTime | `string` |
| SelfDeciderStartTime | `string` |
| SelfDeciderEndTime | `string` |
| EntryTime | `string` |
| ExitTime | `string` |
| ExitWith | `string` |
| IsDefaultEntryTime | `bool` |
| IsDefaultExitTime | `bool` |
| Comment | `string` |
| SpareTimeActivity | `SpareTimeActivityDTO` |
| VacationNote | `string` |

#### SleepIntervalResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| StartTime | `string` |
| EndTime | `string` |

#### SpecificOpeningHourDto

| Property | Type |
|----------|------|
| Id | `int64` |
| StartDate | `DateTime` |
| EndDate | `DateTime` |
| OpenTime | `string` |
| CloseTime | `string` |

#### SpecificOpeningHourWithInstitutionDto

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| SpecificOpeningHours | `List<SpecificOpeningHourDto>` |

#### UpdatePresenceDayActivityRequestModel

| Property | Type |
|----------|------|
| ActivityType | `Nullable<ActivityTypeEnum>` |
| Pickup | `UpdatePresenceDayPickedUpActivityRequestModel` |
| SelfDecider | `UpdatePresenceDaySelfDeciderActivityRequestModel` |
| SendHome | `UpdatePresenceDaySendHomeActivityRequestModel` |
| GoHomeWith | `UpdatePresenceDayGoHomeWithActivityRequestModel` |
| EntryTime | `string` |
| ExitTime | `string` |

#### UpdatePresenceDayGoHomeWithActivityRequestModel

| Property | Type |
|----------|------|
| ExitWith | `string` |
| EntryTime | `string` |
| ExitTime | `string` |

#### UpdatePresenceDayPickedUpActivityRequestModel

| Property | Type |
|----------|------|
| EntryTime | `string` |
| ExitTime | `string` |
| ExitWith | `string` |

#### UpdatePresenceDayRequestModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| Id | `Nullable<int64>` |
| DayOfWeek | `int32` |
| ByDate | `string` |
| Comment | `string` |
| SpareTimeActivity | `SpareTimeActivityRequestModel` |
| PresenceActivity | `UpdatePresenceDayActivityRequestModel` |
| RepeatPattern | `PresenceTemplateRepeatPattern` |
| ExpiresAt | `Nullable<DateTime>` |

#### UpdatePresenceDaySelfDeciderActivityRequestModel

| Property | Type |
|----------|------|
| EntryTime | `string` |
| ExitStartTime | `string` |
| ExitEndTime | `string` |

#### UpdatePresenceDaySendHomeActivityRequestModel

| Property | Type |
|----------|------|
| EntryTime | `string` |
| ExitTime | `string` |

#### UpdateSleepIntervalsDTO

| Property | Type |
|----------|------|
| PresenceRegistrationId | `int64` |
| Id | `int64` |
| Start | `string` |
| End | `string` |

#### UpdateStatusByInstitutionProfileIdsDTO

| Property | Type |
|----------|------|
| InstitutionProfileIds | `List<int64>` |
| Status | `int32` |

#### UpdateStatusDTO

| Property | Type |
|----------|------|
| Ids | `List<int64>` |
| Status | `int32` |

#### VacationAnnouncementDto

| Property | Type |
|----------|------|
| VacationId | `int64` |
| StartDate | `DateTime` |
| EndDate | `DateTime` |
| Description | `string` |
| IsEditable | `bool` |

#### VacationAnnouncementsByChildrenDto

| Property | Type |
|----------|------|
| Child | `ParentDailyOverviewInstitutionProfileDto` |
| VacationAnnouncements | `List<VacationAnnouncementDto>` |

#### VacationEntryDto

| Property | Type |
|----------|------|
| ChildIds | `List<int64>` |
| Intervals | `List<VacationIntervals>` |
| Comment | `string` |

#### VacationRegistrationDto

| Property | Type |
|----------|------|
| VacationRegistrationId | `int64` |
| StartDate | `DateTime` |
| EndDate | `DateTime` |
| Title | `string` |
| NoteToGuardian | `string` |
| ResponseId | `int64` |
| ResponseDeadline | `Nullable<DateTime>` |
| IsEditable | `bool` |
| IsMissingAnswer | `bool` |
| IsPresenceTimesRequired | `bool` |

#### VacationRegistrationResponseForGuardianDto

| Property | Type |
|----------|------|
| VacationRegistration | `VacationRegistrationDto` |
| VacationRegistrationResponse | `RespondToVacationRegistrationRequestRequestReceivedDto` |

#### VacationRegistrationsByChildrenDto

| Property | Type |
|----------|------|
| Child | `ParentDailyOverviewInstitutionProfileDto` |
| VacationRegistrations | `List<VacationRegistrationDto>` |

### DTOs.ComeGo.EmployeeWeekOverview

#### ComeGoEmployeeWeekOverviewPresenceDetailsDto

| Property | Type |
|----------|------|
| StartTime | `ComeGoPresenceTimeWithTense` |
| EndTime | `ComeGoPresenceTimeWithTense` |

#### ComeGoEmployeeWeekOverviewVacationDetailsDto

| Property | Type |
|----------|------|
| StartTime | `string` |
| EndTime | `string` |

#### ComeGoGetVacationRegistrationOverviewRequestDto

| Property | Type |
|----------|------|
| DepartmentId | `int64` |
| FilterGroups | `List<int64>` |
| StatusFilters | `string[]` |
| Offset | `int32` |
| Limit | `int32` |

#### ComeGoGetWeekOverviewRequestDto

| Property | Type |
|----------|------|
| DepartmentId | `int64` |
| GroupIds | `List<int64>` |
| StatusFilters | `string[]` |
| StartDate | `string` |
| EndDate | `string` |
| Offset | `int32` |
| Limit | `int32` |

#### ComeGoPresenceTimeWithTense

| Property | Type |
|----------|------|
| Timestamp | `string` |
| Tense | `ComGoEmployeeWeekOverviewTenseEnum` |

#### DescriptionDto

| Property | Type |
|----------|------|
| Html | `string` |

#### EmployeeWeekOverviewActivitiesDto

| Property | Type |
|----------|------|
| Date | `DateTime` |
| Type | `ComeGoEmployeeWeekOverviewPresenceTypeEnum` |
| presenceDetails | `ComeGoEmployeeWeekOverviewPresenceDetailsDto` |
| vacationDetails | `ComeGoEmployeeWeekOverviewVacationDetailsDto` |

#### EmployeeWeekOverviewChildActivitiesDto

| Property | Type |
|----------|------|
| Child | `ActivityListChildResultModel` |
| Activities | `List<EmployeeWeekOverviewActivitiesDto>` |
| PresenceRegistrationId | `Nullable<int64>` |
| PresenceRegistrationIsDefaultEntryTime | `Nullable<bool>` |
| PresenceRegistrationIsDefaultExitTime | `Nullable<bool>` |

#### EmployeeWeekOverviewPresenceDto

| Property | Type |
|----------|------|
| ActivityType | `Nullable<ActivityTypeEnum>` |
| ByDate | `DateTime` |
| Comment | `string` |
| DayOfWeek | `PresenceDayOfWeek` |
| EntryTime | `Nullable<DateTime>` |
| ExitTime | `Nullable<DateTime>` |
| ExitWith | `string` |
| Id | `Nullable<int64>` |
| IsOnVacation | `bool` |
| IsRepeating | `bool` |
| RepeatFromDate | `Nullable<DateTime>` |
| RepeatToDate | `Nullable<DateTime>` |
| SelfDeciderEndTime | `Nullable<DateTime>` |
| SelfDeciderStartTime | `Nullable<DateTime>` |
| SpareTimeActivity | `SpareTimeActivityDTO` |
| Vacation | `EmployeeWeekOverviewPresenceVacationDto` |

#### EmployeeWeekOverviewPresenceVacationDto

| Property | Type |
|----------|------|
| CreatedDateTime | `DateTime` |
| CreatorInstProfileId | `int64` |
| CreatorName | `string` |
| Description | `DescriptionDto` |
| EndDateTime | `DateTime` |
| Id | `int64` |
| IsAllDayEvent | `bool` |
| IsDeleted | `bool` |
| StartDateTime | `DateTime` |
| Title | `string` |
| Type | `EventType` |

#### GetPresenceOverviewDto

| Property | Type |
|----------|------|
| WeekNumber | `int32` |
| PresenceDays | `List<WeekOverviewPresenceDaysDto>` |
| ChildActivities | `List<EmployeeWeekOverviewChildActivitiesDto>` |

#### GetVacationRegistrationOverviewDto

| Property | Type |
|----------|------|
| TotalNumber | `int32` |
| VacationRegistrations | `List<VacationRegistrationsDto>` |

#### PresenceChildrenDistributionDto

| Property | Type |
|----------|------|
| NumberPresent | `int32` |
| NumberOnVacation | `int32` |
| NumberSick | `int32` |
| NumberNotArrived | `int32` |
| Intervals | `List<PresenceIntervalModel>` |
| IsDistributionEnabled | `bool` |

#### VacationRegistrationsDto

| Property | Type |
|----------|------|
| VacationRegistrationId | `int32` |
| Title | `string` |
| StartDate | `Nullable<DateTime>` |
| EndDate | `Nullable<DateTime>` |
| Deadline | `Nullable<DateTime>` |
| RegardingDepartmentAndGroupsText | `List<string>` |
| Subtitle | `string` |
| ShortName | `string` |

#### WeekOverviewPresenceDaysDto

| Property | Type |
|----------|------|
| Date | `string` |
| NumberOfChildren | `int32` |
| TotalNumberOfChildren | `int32` |

### DTOs.ComeGo.OpeningHoursAndClosedDays

#### ClosedDaysDTO

| Property | Type |
|----------|------|
| Id | `int64` |
| StartDate | `DateTime` |
| EndDate | `DateTime` |
| Name | `string` |

#### ClosedDaysOverviewDTO

| Property | Type |
|----------|------|
| ClosedDays | `List<ClosedDaysDTO>` |

#### GetClosedDaysDTO

| Property | Type |
|----------|------|
| InstitutionClosedDays | `List<InstitutionClosedDaysDTO>` |

#### GetOpeningHoursByByInstitutionCodesRequestModel

| Property | Type |
|----------|------|
| InstitutionCodes | `string[]` |
| StartDate | `string` |
| EndDate | `string` |

#### GetOpeningHoursByInstitutionCodesDto

| Property | Type |
|----------|------|
| OpeningHoursOverviewDto | `List<OpeningHoursOverviewDto>` |

#### InstitutionClosedDaysDTO

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| ClosedDaysOverview | `ClosedDaysOverviewDTO` |

#### OpeningHoursDto

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| Date | `DateTime` |
| OpenTime | `string` |
| CloseTime | `string` |
| Name | `string` |
| Type | `OpeningHoursType` |

#### OpeningHoursOverviewDto

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| OpeningHoursDto | `List<OpeningHoursDto>` |

### DTOs.ComeGo.PresenceDataViewModels

#### PresenceDaySchedule : IPresenceDto

| Property | Type |
|----------|------|
| DayOfWeek | `PresenceDayOfWeek` |
| FullName | `string` |
| DayText | `string` |
| RepeatPattern | `PresenceTemplateRepeatPattern` |
| RepeatFromDate | `DateTime` |
| RepeatToDate | `Nullable<DateTime>` |
| IsOnVacation | `bool` |
| IsPlannedTimesOutsideOpeningHours | `bool` |

### DTOs.ComeGo.UpdatePresenceDay

#### SpareTimeActivityDTO

| Property | Type |
|----------|------|
| StartTime | `DateTime` |
| EndTime | `DateTime` |
| Comment | `string` |

#### SpareTimeActivityRequestModel

| Property | Type |
|----------|------|
| StartDate | `string` |
| EndDate | `string` |
| Comment | `string` |

### DTOs.ComeGo.VacationRegistration

#### CreateVacationRegistrationRequest

| Property | Type |
|----------|------|
| StartDateTime | `string` |
| EndDateTime | `string` |
| ResponseDeadline | `string` |
| CreatorInstProfileId | `int64` |
| Title | `string` |
| Departments | `DepartmentIdsSimpleRequest[]` |
| NoteToGuardians | `string` |
| IsPresenceTimesRequired | `bool` |

#### DepartmentIdsSimpleRequest

| Property | Type |
|----------|------|
| GroupId | `int64` |
| FilteringGroups | `int64[]` |

#### UpdateVacationRegistrationRequest

| Property | Type |
|----------|------|
| Id | `int64` |
| ResponseDeadline | `string` |

### DTOs.Consent

#### ConsentUpdateDTO

| Property | Type |
|----------|------|
| ConsentId | `int64` |
| Answer | `string` |

#### ProfileConsentUpdatesDTO

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| InstitutionProfileConsentUpdates | `List<ConsentUpdateDTO>` |

### DTOs.Group.RequestModel

#### LinkedGroupRequestModel

| Property | Type |
|----------|------|
| GroupId | `int64` |
| PortalRolesEnum | `List<PortalRole>` |

### DTOs.Group.ResultModel

#### LightMembershipResultModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `Nullable<int64>` |
| FullName | `string` |
| Metadata | `string` |
| ShortName | `string` |
| ProfilePictureUrl | `string` |
| ProfileId | `Nullable<int64>` |
| OtpInboxId | `Nullable<int64>` |

#### MembershipCountResultModel

| Property | Type |
|----------|------|
| Employees | `int64` |
| Children | `int64` |
| Guardians | `int64` |
| Total | `int64` |

#### MembershipLightPortalRoleGroupResultModel

| Property | Type |
|----------|------|
| InstitutionProfiles | `List<LightMembershipResultModel>` |
| PortalRole | `PortalRole` |

### DTOs.Logging

#### LogErrorAdditionalParameterDto

| Property | Type |
|----------|------|
| Release | `string` |
| BuildNumber | `string` |
| OSVersion | `string` |
| DeviceDescription | `string` |
| ApiVersion | `string` |
| ApiTransactionTrace | `string` |
| LogLevel | `LogLevel` |
| Time | `DateTime` |
| Message | `string` |
| AdditionalInfo | `string` |
| AdditionalInfoObj | `object` |
| Exception | `LogErrorExceptionDto` |
| ServerResponse | `LogErrorServerResponseDto` |
| ProfileData | `LogErrorProfileDto` |
| DeviceDbID | `Nullable<int32>` |

#### LogErrorExceptionDto

| Property | Type |
|----------|------|
| Type | `string` |
| Message | `string` |
| Stacktrace | `string` |
| InnerExceptionType | `string` |
| InnerExceptionMessage | `string` |
| InnerExceptionStacktrace | `string` |

#### LogErrorProfileDto

| Property | Type |
|----------|------|
| ChildAge | `string` |
| ChildCount | `string` |
| PortalRole | `string` |
| InstitutionRole | `string` |
| InstitutionCount | `string` |

#### LogErrorServerResponseDto

| Property | Type |
|----------|------|
| Url | `string` |
| UrlMethod | `string` |
| UrlCode | `string` |
| HttpCode | `string` |
| ServerCode | `string` |
| ServerSubCode | `string` |
| ResultBody | `string` |

### DTOs.MasterData

#### MasterDataDTO

| Property | Type |
|----------|------|
| Profiles | `List<MasterDataModelViewModel>` |

#### PostInstitutionProfileMasterDataDto

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| InstitutionProfileId | `Nullable<int64>` |
| Email | `string` |
| HomePhoneNumber | `string` |
| MobilePhoneNumber | `string` |
| WorkPhoneNumber | `string` |
| Description | `string` |

#### PostMasterDataDto

| Property | Type |
|----------|------|
| InstitutionProfilesMasterData | `List<PostInstitutionProfileMasterDataDto>` |
| IsChangedPrimaryInstProfile | `Nullable<bool>` |
| PrimaryInstitutionProfileId | `Nullable<int64>` |

#### ProfilePictureWithUrlDto

| Property | Type |
|----------|------|
| Url | `string` |

#### UpdateProfilePictureRequestModel

| Property | Type |
|----------|------|
| ApplyToAllInstitutionProfiles | `bool` |
| InstitutionProfileId | `int64` |
| MediaId | `Nullable<int64>` |

### DTOs.Onboarding

#### OnboardingProfileDto

| Property | Type |
|----------|------|
| IsLatestDataPolicyAccepted | `bool` |
| PortalRole | `PortalRole` |
| Children | `List<StubbedChild>` |
| InstitutionProfiles | `List<StubbedInstitutionProfile>` |
| OverConsentAge | `Nullable<bool>` |
| ContactInfoEditable | `Nullable<bool>` |

#### OnboardingResponseDto

| Property | Type |
|----------|------|
| Profiles | `List<OnboardingProfileDto>` |

#### StubbedChild

| Property | Type |
|----------|------|
| InstitutionProfile | `InstitutionProfileChild` |

#### StubbedInstitutionProfile

| Property | Type |
|----------|------|
| NewInstitutionProfile | `bool` |
| Id | `int64` |
| ProfileId | `int64` |
| ProfilePicture | `AulaFileContent` |

### DTOs.SearchRecipientsDtos

#### Person

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| AulaEmail | `string` |
| Role | `string` |
| InstitutionCode | `string` |

### Enums

#### DefaultUnknownEnumConverter : 0x379

*(no public properties detected)*

#### PortalRoleHelpers

| Property | Type |
|----------|------|
| (const) AllPortalRolesExceptOtpCount | `int32` |

### Enums.ComeGo

#### ActivityTypeEnumelpers

*(no public properties detected)*

### Enums.Groups

#### GroupTypeEnumelpers

*(no public properties detected)*

### Models

#### ObjectWithId

| Property | Type |
|----------|------|
| Id | `int64` |

#### RichTextWrapperDto

| Property | Type |
|----------|------|
| Html | `string` |

### Models.BlockedCommunication

#### BaseBlockedGenericCommunicationResult`1

| Property | Type |
|----------|------|
| FinalRecipients | `IEnumerable<!0>` |
| BlockedRecipients | `IEnumerable<!0>` |
| UnblockedRecipients | `IEnumerable<!0>` |
| WarningTexts | `ConfirmBoxTexts` |

#### BlockedCommunicationGroup

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

#### BlockedCommunicationInstitutionProfile

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| Name | `string` |
| Role | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |

#### BlockedCommunicationMeetingResult

| Property | Type |
|----------|------|
| WarningTexts | `ConfirmBoxTexts` |
| FinalInstRecipientIds | `int64[]` |

#### BlockedCommunicationModel

*(no public properties detected)*

#### BlockedCommunicationModel.SubGroupIdentifier

| Property | Type |
|----------|------|
| Name | `string` |
| Id | `int64` |
| PortalRole | `PortalRole` |

#### BlockedCommunicationOtpInbox

| Property | Type |
|----------|------|
| Id | `int64` |
| DisplayName | `string` |

#### BlockedGroupCommunicationResult : 0x1ee

*(no public properties detected)*

#### BlockedProfileCommunicationResult : 0x1f2

*(no public properties detected)*

### Models.Calendar

#### BaseTimeSlotAnswer

| Property | Type |
|----------|------|
| Id | `int32` |
| SelectedTimeSlotIndex | `int32` |

#### BaseTimeSlotDto

| Property | Type |
|----------|------|
| EndDate | `DateTime` |
| StartDate | `DateTime` |
| Id | `int64` |
| TimeSlotIndexes | `List<TimeSlotIndex>` |

#### BirthdayEventDto

| Property | Type |
|----------|------|
| Birthday | `DateTime` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionProfileId | `int64` |
| MainGroupName | `string` |

#### CalendarResourceConflict

| Property | Type |
|----------|------|
| UnavailableResourceIds | `int64[]` |

#### CreatorInstProfileId

*(no public properties detected)*

#### GetEventsForInstitutionRequestModel

| Property | Type |
|----------|------|
| Start | `DateTime` |
| End | `DateTime` |
| InstCodes | `string[]` |

#### GetEventsParameters

| Property | Type |
|----------|------|
| InstProfileIds | `IEnumerable<int64>` |
| ResourceIds | `int64[]` |
| Start | `DateTime` |
| End | `DateTime` |
| SpecificTypes | `string[]` |
| SchoolCalendarInstitutionCodes | `string[]` |

#### RelationsMessage

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| IsSelected | `bool` |
| RelationMode | `RelationMode` |

#### SendEventReminderRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| Message | `string` |

#### TimeSlot : BaseTimeSlotDto

| Property | Type |
|----------|------|
| Answers | `List<TimeSlotAnswer>` |
| PrimaryResource | `Resource` |
| PrimaryResourceText | `string` |

#### TimeSlotAnswer : BaseTimeSlotAnswer

| Property | Type |
|----------|------|
| ConcerningProfile | `EventProfile/EventProfileDetails` |
| InstProfile | `EventProfile/EventProfileDetails` |
| CanRemoveBlockingOrResponseForTimeSlot | `bool` |

#### TimeSlotAnswerSimpleDto : BaseTimeSlotAnswer

| Property | Type |
|----------|------|
| ConcerningProfileId | `Nullable<int32>` |
| InstProfileId | `Nullable<int32>` |
| CanRemoveBlockingOrResponseForTimeSlot | `bool` |

#### TimeSlotIndex

| Property | Type |
|----------|------|
| StartTime | `DateTime` |
| EndTime | `DateTime` |

#### TimeSlotProfile

*(no public properties detected)*

#### TimeSlotSimpleDto : BaseTimeSlotDto

| Property | Type |
|----------|------|
| Answers | `List<TimeSlotAnswerSimpleDto>` |
| BelongsToResource | `Nullable<int64>` |

### Models.Calendar.Birthday

#### GetBirthdayEvents

| Property | Type |
|----------|------|
| Start | `DateTime` |
| End | `DateTime` |
| InstCodes | `string[]` |

### Models.Calendar.CalendarSynchronisation

#### CalendarSynchronisationConfigurationItem

| Property | Type |
|----------|------|
| InstitutionProfileId | `int32` |
| Id | `int32` |
| Calendarfeedconfigurationid | `int32` |
| OwnerId | `int32` |
| RegardingId | `int32` |
| OneWeekFeed | `string` |
| OneYearFeed | `string` |
| Weekly | `bool` |
| Filters | `List<string>` |
| feedStatus | `string` |

#### CalendarSynchronisationModel

| Property | Type |
|----------|------|
| PolicyAccepted | `bool` |

#### CalendarSynchronisationMunicipalityFeedModel

| Property | Type |
|----------|------|
| MunicipalityCode | `string` |
| CalendarFeedEnabled | `bool` |

#### CreateCalendarSynchronizationConfigurationRequest

| Property | Type |
|----------|------|
| Filters | `List<EventType>` |
| Weekly | `bool` |
| InstitutionProfileId | `int64` |

#### GetEventTypesByPortalRoleResultModel

| Property | Type |
|----------|------|
| EventTypes | `List<string>` |

#### UpdateCalendarSynchronizationConfigurationRequest

| Property | Type |
|----------|------|
| Filters | `List<EventType>` |
| CalendarFeedConfigurationId | `int64` |

### Models.Calendar.CalenderNotification

#### NotificationItemDto

| Property | Type |
|----------|------|
| NotificationId | `string` |
| GeneralInformationId | `Nullable<int64>` |
| InstitutionProfileId | `Nullable<int64>` |
| NotificationEventType | `NotificationEventType` |
| NotificationArea | `NotificationArea` |
| NotificationType | `NotificationType` |
| InstitutionCode | `string` |
| Expires | `Nullable<DateTime>` |
| ResponseDeadline | `Nullable<DateTime>` |
| Triggered | `Nullable<DateTime>` |
| Url | `string` |
| Content | `HtmlDto` |
| RelatedChildInstitutionProfileId | `Nullable<int64>` |
| RelatedChildName | `string` |
| Title | `string` |
| OriginalTitle | `string` |
| EventId | `Nullable<int64>` |
| StartTime | `Nullable<DateTime>` |
| EndTime | `Nullable<DateTime>` |
| StartDate | `Nullable<DateTime>` |
| EndDate | `Nullable<DateTime>` |
| OtherCalendarPersonName | `string` |
| OtherCalendarInstitutionProfileId | `Nullable<int32>` |
| ResponderName | `string` |
| SenderName | `string` |
| MessageText | `string` |
| RelatedInstitution | `string` |
| FolderId | `Nullable<int64>` |
| ThreadId | `int64` |
| PostTitle | `string` |
| PostId | `Nullable<int64>` |
| GroupName | `string` |
| GroupId | `Nullable<int64>` |
| AlbumId | `Nullable<int64>` |
| AlbumName | `string` |
| MediaId | `Nullable<int64>` |
| MediaIds | `int64[]` |
| DocumentId | `Nullable<int64>` |
| CommonFileId | `Nullable<int64>` |
| RoomName | `string` |
| EventStartTime | `Nullable<DateTime>` |
| EventEndTime | `Nullable<DateTime>` |
| VacationRegistrationResponseId | `Nullable<int64>` |
| CommonInboxId | `Nullable<int64>` |
| CommonInboxName | `string` |
| NoteToGuardians | `string` |
| IsPresenceTimesRequired | `bool` |
| VacationRequestName | `string` |
| NotificationMessage | `string` |
| OccurrenceDateTime | `Nullable<DateTime>` |
| CancelledBy | `string` |
| WidgetId | `Nullable<int32>` |
| WidgetName | `string` |
| Message | `string` |
| ResourceName | `string` |
| OccurrenceDate | `Nullable<DateTime>` |
| ExceptionEventId | `Nullable<int64>` |
| CommentId | `Nullable<int64>` |
| ProfilePictureInstitutionProfileId | `Nullable<int64>` |

### Models.Calendar.ConversationMeeting

#### ConversationMeetingUpdateInput

| Property | Type |
|----------|------|
| EventId | `int64` |
| ChildId | `int64` |

#### CreateConversationMeetingProcessedResult

| Property | Type |
|----------|------|
| EventId | `int64` |
| ResourceConflict | `CalendarResourceConflict` |

### Models.Calendar.CreateEvent

#### CreateBaseEventRequest

| Property | Type |
|----------|------|
| EventId | `Nullable<int64>` |
| FromInstProfileId | `Nullable<int64>` |
| Title | `string` |
| EventTypeEnum | `EventType` |
| Description | `string` |
| InviteeIds | `int64[]` |
| InviteeGroups | `List<InviteeGroupRequest>` |
| InvitedGroupIds | `int64[]` |
| CoOrganizerIds | `int64[]` |
| InvitedOtpInboxIds | `List<int64>` |
| AttachmentIds | `int64[]` |
| HideInOwnCalendar | `bool` |
| ResponseDeadline | `string` |
| InstitutionCode | `string` |

#### InviteeGroupRequest

| Property | Type |
|----------|------|
| GroupId | `int64` |
| PortalRoles | `List<string>` |

### Models.Calendar.CreateEvent.Lesson

#### UpdateLessonRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| InstitutionProfileId | `int64` |
| NoteToClass | `string` |
| NoteToTeacher | `string` |
| NoteToSubstitute | `string` |
| AdditionalResourceIds | `int64[]` |
| AdditionalResourceText | `string` |
| AttachmentIds | `int64[]` |

### Models.Calendar.CreateEvent.RepeatingEvent

#### CreateRepeatingEventRequest : CreateSimpleEventRequest

| Property | Type |
|----------|------|
| OccurenceLimit | `Nullable<int32>` |
| WeekdayMask | `bool[]` |
| DayInMonth | `Nullable<int32>` |
| RepeatTypeEnum | `RepeatType` |
| Interval | `int32` |
| MaxDate | `Nullable<DateTime>` |
| OccurrenceDateTime | `string` |

### Models.Calendar.CreateEvent.Resources

#### CreateEventLocationResource : CreateEventResource

*(no public properties detected)*

#### CreateEventResource

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Description | `string` |
| Name | `string` |
| ShortName | `string` |
| DisplayName | `string` |
| ResourceCategory | `ResourceCategory` |
| NumberOfAvailableOccurrences | `Nullable<int32>` |
| NumberOfOccurrences | `Nullable<int32>` |

#### CreateEventResourceProfile

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Name | `string` |
| InstitutionCode | `string` |

### Models.Calendar.CreateEvent.SimpleEvent

#### CreateSimpleEventRequest : CreateBaseEventRequest

| Property | Type |
|----------|------|
| StartDateTime | `string` |
| EndDateTime | `string` |
| AllDay | `bool` |
| Private | `bool` |
| ResponseRequired | `bool` |
| PrimaryResourceId | `Nullable<int64>` |
| PrimaryResourceText | `string` |
| AdditionalResourceIds | `int64[]` |
| AdditionalResourceText | `string` |
| AddToInstitutionCalendar | `bool` |
| AddedToInstitutionCalendar | `bool` |
| MaximumNumberOfParticipants | `Nullable<int64>` |
| DoRequestNumberOfParticipants | `bool` |

### Models.Calendar.CreateEvent.TimeslotEvent

#### CreateTimeslotEventRequest : CreateBaseEventRequest

| Property | Type |
|----------|------|
| TimeSlots | `CreateTimeslotEventTimeSlotDto[]` |
| BreakLength | `int32` |
| MeetingDuration | `int32` |
| ChildRequired | `bool` |
| MeetingsBetweenBreaks | `int32` |
| AddToInstitutionCalendar | `bool` |
| NumberOfParticipantsPerTimeSlot | `Nullable<int32>` |

#### CreateTimeslotEventTimeSlotDto

| Property | Type |
|----------|------|
| Id | `Nullable<int64>` |
| PrimaryResourceId | `Nullable<int64>` |
| PrimaryResourceText | `string` |
| StartDate | `DateTime` |
| EndDate | `DateTime` |

### Models.Calendar.DailyAggregatedEvents

#### AggregatedEventsGroupByType

| Property | Type |
|----------|------|
| Type | `string` |
| Count | `int32` |

#### DailyAggregatedEventsResultModel

| Property | Type |
|----------|------|
| Date | `DateTime` |
| AggregatedEvents | `List<AggregatedEventsGroupByType>` |

#### DailyEventCountResultModel

| Property | Type |
|----------|------|
| Date | `DateTime` |
| Count | `int32` |

#### DailyGroupEventCountRequestModel

| Property | Type |
|----------|------|
| GroupId | `int64` |
| Start | `DateTime` |
| End | `DateTime` |

### Models.Calendar.Event

#### BaseTimeslotEventDto

| Property | Type |
|----------|------|
| ChildRequired | `Nullable<bool>` |

#### CheckEventConflictInput

| Property | Type |
|----------|------|
| Start | `DateTime` |
| End | `DateTime` |
| AllDay | `bool` |
| InstitutionProfileIds | `int64[]` |
| ExcludeEventId | `Nullable<int64>` |

#### CommunicationEventSpinnerItem

| Property | Type |
|----------|------|
| DisplayName | `string` |
| Value | `object` |

#### ConflictEventItem

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |

#### DelegateAccessInstitution

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| InstitutionName | `string` |

#### DelegateAccesses

| Property | Type |
|----------|------|
| OwnerInstProfileId | `int64` |
| DelegatedInstProfiles | `List<DelegateAccessesItem>` |

#### DelegateAccessesInput

| Property | Type |
|----------|------|
| OwnerInstProfileId | `int64` |
| DelegatedInstProfileIds | `int64[]` |

#### DelegateAccessesItem

| Property | Type |
|----------|------|
| InstProfileId | `int64` |
| ProfileId | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MetaData | `string` |

#### EventBaseClass

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| AllDay | `Nullable<bool>` |
| OldAllDay | `Nullable<bool>` |
| AddedToInstitutionCalendar | `bool` |
| HideInOwnCalendar | `bool` |
| ResponseDeadline | `Nullable<DateTime>` |
| IsDeadlineExceeded | `bool` |
| EndDateTime | `DateTime` |
| StartDateTime | `DateTime` |
| Title | `string` |
| Type | `string` |
| Private | `Nullable<bool>` |
| ResponseRequired | `Nullable<bool>` |
| BelongsToProfiles | `IEnumerable<int64>` |
| BelongsToResources | `IEnumerable<int64>` |
| Id | `int32` |
| SecurityLevel | `Nullable<int32>` |
| IsDeleted | `bool` |
| OldEndDateTime | `Nullable<DateTime>` |
| OldStartDateTime | `Nullable<DateTime>` |
| InviteeGroups | `List<EventGroupWithRolesDTO>` |
| InvitedGroups | `List<SimpleGroupDto>` |
| PrimaryResourceText | `string` |
| PrimaryResource | `EventBaseClass/EventResource` |
| AdditionalResources | `List<EventBaseClass/EventResource>` |
| AdditionalResourceText | `string` |
| Repeating | `RepeatingEventDto` |
| ResponseStatus | `Nullable<ResponseType>` |
| DirectlyRelated | `bool` |
| MaximumNumberOfParticipants | `Nullable<int64>` |
| ActualNumberOfParticipants | `Nullable<int64>` |
| OccurrenceDateTime | `Nullable<DateTime>` |

#### EventBaseClass.EventResource

| Property | Type |
|----------|------|
| Name | `string` |
| Label | `string` |
| Value | `string` |
| Category | `EventBaseClass/EventResource/ResourceCategory` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Id | `int32` |
| ShortName | `string` |

#### EventBaseClass.EventResource.ResourceCategory

| Property | Type |
|----------|------|
| ResourceType | `ResourceType` |

#### EventDetailsCreatorDto

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| ProfileId | `int64` |
| Name | `string` |
| ShortName | `string` |
| Metadata | `string` |
| Role | `PortalRole` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| InstitutionRole | `InstitutionRole` |

#### EventDetailsDto : EventBaseClass

| Property | Type |
|----------|------|
| Attachments | `List<AulaFileResultDto>` |
| Invitees | `List<EventProfile>` |
| CoOrganizers | `IEnumerable<EventProfile>` |
| InvitedGroupHomeChildren | `IEnumerable<InvitedGroupHome>` |
| Description | `RichTextWrapperDto` |
| Lesson | `Lesson` |
| Creator | `EventDetailsCreatorDto` |
| VacationRegistration | `VacationRegistrationDetailsResultDto` |
| TimeSlot | `TimeslotEventDto` |
| CanEditStartDate | `Nullable<bool>` |
| CanAnswerForSeries | `Nullable<bool>` |
| DoRequestNumberOfParticipants | `bool` |
| LastReminderDateTime | `Nullable<DateTime>` |

#### EventGroup

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Description | `string` |
| Type | `string` |
| Access | `string` |
| Status | `string` |
| DashboardEnabled | `bool` |
| InstitutionCode | `string` |

#### EventGroupWithRolesDTO

| Property | Type |
|----------|------|
| Group | `EventGroup` |
| InvitedPortalRoles | `List<string>` |

#### EventItem

| Property | Type |
|----------|------|
| Type | `ItemType` |
| eventViewModel | `EventViewModel` |
| Title | `string` |
| Id | `int64` |
| DateTime | `DateTime` |

#### EventProfile

| Property | Type |
|----------|------|
| InstProfile | `EventProfile/EventProfileDetails` |
| ResponseType | `ResponseType` |
| ResponseDateTime | `Nullable<DateTime>` |
| NumberOfAdultParticipants | `Nullable<int32>` |
| NumberOfChildParticipants | `Nullable<int32>` |

#### EventProfile.EventProfileDetails

| Property | Type |
|----------|------|
| Email | `string` |
| Administrator | `object` |
| FirstName | `string` |
| LastName | `string` |
| FullName | `string` |
| ShortName | `string` |
| Metadata | `string` |
| Role | `string` |
| Phone | `string` |
| CanRemoveBlockingOrResponseForTimeSlot | `bool` |
| ProfileId | `int32` |
| InstitutionProfileId | `int64` |
| ProfilePictureUrl | `string` |

#### EventSimpleDto : EventBaseClass

| Property | Type |
|----------|------|
| HasAttachments | `Nullable<bool>` |
| Lesson | `LessonSimple` |
| VacationChildrenCountByDates | `List<VacationRegistrationChildrenCountByDates>` |
| CreatorAulaName | `string` |
| CreatorProfileId | `Nullable<int64>` |
| CreatorInstProfileId | `Nullable<int64>` |
| TimeSlot | `TimeslotEventSimpleDto` |

#### InstitutionDelegateAccessesItem

| Property | Type |
|----------|------|
| InstProfileId | `int64` |
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |
| MetaData | `string` |
| Institution | `DelegateAccessInstitution` |

#### InvitedGroupHome

| Property | Type |
|----------|------|
| otpInboxId | `int64` |
| groupHomeId | `int64` |
| regardingChildId | `int64` |
| regardingChildDisplayName | `string` |
| regardingChildMetaData | `string` |
| groupHomeName | `string` |
| responseType | `ResponseType` |
| responseDateTime | `Nullable<DateTime>` |
| NumberOfAdultParticipants | `Nullable<int32>` |
| NumberOfChildParticipants | `Nullable<int32>` |

#### Lesson : LessonBase

| Property | Type |
|----------|------|
| Participants | `List<LessonParticipant>` |
| NoteToClass | `HtmlDto` |
| NoteToSubstitute | `HtmlDto` |
| NoteToTeacher | `HtmlDto` |

#### LessonBase

| Property | Type |
|----------|------|
| LessonId | `string` |
| LessonStatus | `string` |

#### LessonParticipant

| Property | Type |
|----------|------|
| ParticipantProfile | `InstitutionProfile` |
| ParticipantRole | `ParticipantRole` |
| TeacherName | `string` |
| TeacherInitials | `string` |

#### LessonSimple : LessonBase

| Property | Type |
|----------|------|
| HasRelevantNote | `bool` |
| Participants | `List<ParticipantSimple>` |

#### ParticipantSimple

| Property | Type |
|----------|------|
| TeacherInitials | `string` |
| TeacherName | `string` |
| ParticipantRole | `ParticipantRole` |

#### RepeatingEventDto

| Property | Type |
|----------|------|
| Pattern | `string` |
| OccurenceLimit | `Nullable<int32>` |
| Interval | `Nullable<int32>` |
| DayInMonth | `Nullable<int32>` |
| MaxDate | `Nullable<DateTime>` |
| WeekdayMask | `bool[]` |
| OriginalStartDateTime | `Nullable<DateTime>` |
| OriginalEndDateTime | `Nullable<DateTime>` |
| LastOccurrenceDate | `Nullable<DateTime>` |

#### SimpleGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MainGroup | `bool` |
| UniGroupType | `string` |
| IsDeactivated | `bool` |
| AllowMembersToBeShown | `bool` |

#### SimpleGroupWithRolesModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| PortalRoles | `System.Collections.Generic.HashSet`1<PortalRole>` |

#### TimeslotEventDto : BaseTimeslotEventDto

| Property | Type |
|----------|------|
| MeetingsBetweenBreaks | `Nullable<int32>` |
| BreakLength | `Nullable<int32>` |
| MeetingDuration | `Nullable<int32>` |
| CanUpdateResponseToEvent | `bool` |
| TimeSlots | `IEnumerable<TimeSlot>` |
| NumberOfParticipantsPerTimeSlot | `Nullable<int32>` |

#### TimeslotEventSimpleDto : BaseTimeslotEventDto

| Property | Type |
|----------|------|
| TimeSlots | `IEnumerable<TimeSlotSimpleDto>` |

### Models.Calendar.ImportantDate

#### ImportantDateItem

| Property | Type |
|----------|------|
| Id | `int64` |
| EndDateTime | `DateTime` |
| StartDateTime | `DateTime` |
| Title | `string` |
| Type | `string` |
| Invitees | `ImportantDateItemInvitee[]` |
| InstitutionName | `string` |
| AllDay | `bool` |

#### ImportantDateItemInvitee

| Property | Type |
|----------|------|
| InstProfile | `ImportantDateItemProfile` |
| ResponseType | `string` |

#### ImportantDateItemProfile

| Property | Type |
|----------|------|
| Id | `int64` |
| ProfileId | `int32` |
| Role | `string` |
| Relations | `ImportantDateItemProfile[]` |

### Models.Calendar.MyCalendar

#### MyCalendarItem

| Property | Type |
|----------|------|
| Type | `MyCalendarItemType` |
| MyCalendarViewModel | `MyCalendarViewModel` |
| Title | `string` |
| Id | `int64` |
| BirthDay | `DateTime` |
| Name | `string` |
| GroupName | `string` |

### Models.Calendar.RespondEvent

#### BlockTimeSlotRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| TimeSlotId | `int64` |
| TimeSlotIndex | `int32` |

#### DeleteTimeslotRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| TimeSlotId | `int64` |
| TimeSlotIndex | `int32` |
| ConcerningInstitutionProfileId | `Nullable<int32>` |

#### RespondSimpleEventRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| InstitutionProfileId | `int64` |
| InvitedInstProfileId | `Nullable<int64>` |
| ResponseType | `ResponseType` |
| OccurrenceDateTime | `string` |
| NumberOfAdultParticipants | `Nullable<int32>` |
| NumberOfChildParticipants | `Nullable<int32>` |

#### RespondTimeslotEventRequest

| Property | Type |
|----------|------|
| EventId | `int64` |
| ResponseTypeEnum | `ResponseType` |
| TimeSlotId | `int64` |
| TimeSlotIndex | `int32` |
| InstitutionProfileId | `int64` |
| ConcerningInstProfileId | `int64` |
| OnBehalfOf | `bool` |

### Models.Calendar.Vacation

#### VacationDetailsDto : EventDetailsDto

| Property | Type |
|----------|------|
| IsVacationCreatedFromVacationRequest | `bool` |

### Models.ComeGo

#### ChildStatus

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| State | `PresenceStatusEnum` |
| UniStudent | `ComeGoUniStudentProfile` |

#### ComeGoCommonText

*(no public properties detected)*

#### DeletePresenceTemplateRequestModel

| Property | Type |
|----------|------|
| DeleteFromDay | `string` |
| PresentTemplateId | `Nullable<int64>` |

#### GetOverlappingPresenceTemplatesRequest

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| StartDate | `string` |
| EndDate | `string` |
| RepeatPattern | `PresenceTemplateRepeatPattern` |

#### GetPickupResponsibleRequestModel

| Property | Type |
|----------|------|
| UniStudentIds | `int64[]` |

#### GuardianRegisterVacationIntervals

| Property | Type |
|----------|------|
| Date | `DateTime` |
| EntryTime | `Nullable<DateTime>` |
| ExitTime | `Nullable<DateTime>` |
| IsComing | `bool` |

#### GuardianRegisterVacationIntervalsReceiveDto

| Property | Type |
|----------|------|
| Date | `DateTime` |
| EntryTime | `Nullable<DateTime>` |
| ExitTime | `Nullable<DateTime>` |
| IsComing | `bool` |

#### ParentsDailyOverviewResultModel : IPresenceDto

| Property | Type |
|----------|------|
| InstitutionProfile | `ParentDailyOverviewInstitutionProfileDto` |
| MainGroup | `MainGroup` |
| Status | `PresenceStatusEnum` |
| SleepIntervals | `List<SleepIntervalsViewModel>` |
| CheckInTime | `Nullable<DateTime>` |
| CheckOutTime | `Nullable<DateTime>` |
| Location | `PresenceLocationDto` |
| IsDefaultEntryTime | `bool` |
| IsDefaultExitTime | `bool` |
| IsPlannedTimesOutsideOpeningHours | `bool` |

#### PresenceFilterDepartmentModel : SelectionControl

| Property | Type |
|----------|------|
| Id | `int64` |
| _filteringGroups | `PresenceFilterGroupModel[]` |
| MainGroup | `MainGroup` |

#### PresenceFilterGroupModel : SelectionControl

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

#### PresenceFilterResultModel

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Departments | `PresenceFilterDepartmentModel[]` |

#### PresenceSchedulesRequestModel

| Property | Type |
|----------|------|
| FilterInstitutionProfileIds | `List<int64>` |
| FromDate | `string` |
| ToDate | `string` |

#### VacationIntervals

| Property | Type |
|----------|------|
| StartDate | `DateTime` |
| EndDate | `DateTime` |

### Models.ComeGo.ActivityList

#### ActivityFilterResultModel : PresenceFilterResultModel

| Property | Type |
|----------|------|
| PresenceStates | `PresenceStatusEnum[]` |
| PresenceNextActivities | `ActivityTypeEnum[]` |
| Locations | `PhysicalLocationResultModel[]` |

#### ActivityListChildPresenceResultModel

| Property | Type |
|----------|------|
| PresenceRegistrationId | `int64` |
| UniStudent | `ActivityListChildResultModel` |
| PresenceState | `PresenceStatusEnum` |
| Comment | `string` |
| Note | `string` |
| Location | `PresenceLocationDto` |
| EditablePresenceStates | `PresenceStatusEnum[]` |
| PastActivities | `List<PastPresenceActivityResultModel>` |
| FutureActivities | `List<FuturePresenceActivityResultModel>` |
| IsEmphasized | `bool` |
| IsDefaultEntryTimes | `bool` |
| IsDefaultExitTimes | `bool` |

#### ActivityListChildResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionCode | `string` |
| ProfileId | `int64` |
| Role | `PortalRole` |
| ShortName | `string` |
| Metadata | `string` |
| Name | `string` |
| MainGroup | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

#### ActivityListRequestModel

| Property | Type |
|----------|------|
| DepartmentId | `int64` |
| GroupIds | `int64[]` |
| Limit | `Nullable<int32>` |
| Offset | `Nullable<int32>` |
| States | `PresenceStatusEnum[]` |
| NextActivity | `Nullable<ActivityTypeEnum>` |
| LocationIds | `int64[]` |
| SortOn | `ActivityListSortingEnum` |
| DailyNote | `ActivityListNoteEnum` |

#### ActivityListResultModel

| Property | Type |
|----------|------|
| TotalNumberOfChildren | `int32` |
| NumberOfChildrenPresent | `int32` |
| Activities | `List<ActivityListChildPresenceResultModel>` |

#### FuturePresenceActivityResultModel

| Property | Type |
|----------|------|
| ActivityType | `ActivityTypeEnum` |
| EntryTime | `string` |
| ExitTime | `string` |
| ExitWith | `string` |
| SelfDeciderStartTime | `string` |
| SelfDeciderEndTime | `string` |
| StartTime | `string` |
| EndTime | `string` |

#### InstitutionWithPresenceStatesResultViewModel

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| EditablePresenceStates | `PresenceStatusEnum[]` |

#### PastPresenceActivityResultModel

| Property | Type |
|----------|------|
| CheckInTime | `string` |
| CheckoutTime | `string` |
| StartTime | `string` |
| EndTime | `string` |
| ActivityType | `ActivityTypeEnum` |

#### UpdateCheckoutDayGoHomeWithActivityRequest

| Property | Type |
|----------|------|
| ExitWith | `string` |
| ExitTime | `string` |

#### UpdateCheckoutDaySendHomeActivityRequest

| Property | Type |
|----------|------|
| ExitTime | `string` |

#### UpdateCheckoutPickedUpActivityRequest

| Property | Type |
|----------|------|
| ExitTime | `string` |
| ExitWith | `string` |

#### UpdateCheckoutSelfDeciderActivityRequest

| Property | Type |
|----------|------|
| SelfDeciderStartTime | `string` |
| SelfDeciderEndTime | `string` |

#### UpdatePickUpResponsibleDataModel

| Property | Type |
|----------|------|
| Result | `bool` |
| HasWhiteSpaceError | `bool` |

#### UpdatePresenceRegistrationRequest

| Property | Type |
|----------|------|
| RegistrationId | `int64` |
| CheckoutType | `Nullable<ActivityTypeEnum>` |
| PickupBy | `UpdateCheckoutPickedUpActivityRequest` |
| SelfDecider | `UpdateCheckoutSelfDeciderActivityRequest` |
| SendHome | `UpdateCheckoutDaySendHomeActivityRequest` |
| GoHomeWith | `UpdateCheckoutDayGoHomeWithActivityRequest` |
| EntryTime | `string` |
| Remark | `string` |

### Models.ComeGo.AddSleepIntervals

#### AddSleepIntervalsRequestModel

| Property | Type |
|----------|------|
| ChildIds | `int64[]` |
| Start | `string` |
| End | `string` |

#### UpdateSleepIntervals

| Property | Type |
|----------|------|
| SleepIntervalIds | `int64[]` |
| Start | `string` |
| End | `string` |

### Models.ComeGo.ChildrenVacation

#### ChildrenVacationChildProfileResultModel

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Id | `int64` |
| ShortName | `string` |
| Name | `string` |
| Metadata | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

#### ChildrenVacationChildResultModel

| Property | Type |
|----------|------|
| Child | `ChildrenVacationChildProfileResultModel` |
| Note | `string` |

#### ChildrenVacationRequestModel

| Property | Type |
|----------|------|
| DepartmentId | `int64` |
| GroupIds | `int64[]` |
| Date | `DateTime` |
| Offset | `int32` |
| Limit | `int32` |

#### ChildrenVacationResultModel

| Property | Type |
|----------|------|
| Count | `int32` |
| Children | `List<ChildrenVacationChildResultModel>` |

#### VacationFilterResultModel : PresenceFilterResultModel

*(no public properties detected)*

### Models.ComeGo.EmployeeWeekOverview

#### ChildrenPresenceDistributionModel

| Property | Type |
|----------|------|
| ShowingDate | `DateTime` |
| Dto | `PresenceChildrenDistributionRequestDto` |

#### OverallItemModel

| Property | Type |
|----------|------|
| LeftText | `string` |
| LeftTextAccessibility | `string` |
| RightText | `string` |

#### PresenceIntervalModel

| Property | Type |
|----------|------|
| StartTime | `string` |
| EndTime | `string` |
| NumberOfChildren | `string` |
| IsCurrent | `bool` |

#### PresenceRegistrationRequestModel

| Property | Type |
|----------|------|
| ChildId | `int64` |
| Date | `string` |

#### PresenceRegistrationTodayRequestModel

| Property | Type |
|----------|------|
| PresenceRegistrationIds | `int64[]` |
| DepartmentId | `string` |

#### WeekOverviewFutureDateModel

| Property | Type |
|----------|------|
| EntryTime | `Nullable<DateTime>` |
| ExitTime | `Nullable<DateTime>` |
| SelfDeciderStartTime | `Nullable<DateTime>` |
| SelfDeciderEndTime | `Nullable<DateTime>` |
| ExitWith | `string` |
| ActivityType | `Nullable<ActivityTypeEnum>` |

### Models.ComeGo.ParentsRegisterActivity

#### ChildGoHomeWithResultModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `int32` |
| FullName | `string` |
| MainGroup | `string` |

#### GetChildGoHomeWithResultModel

| Property | Type |
|----------|------|
| Children | `List<ChildGoHomeWithResultModel>` |

### Models.ComeGo.PickupResponsible

#### DeletePickupResponsibleRequestModel

| Property | Type |
|----------|------|
| PresencePickupSuggestionId | `int64` |

#### GetPickupResponsibleChildResultModel

| Property | Type |
|----------|------|
| UniStudentId | `int64` |
| RelatedPersons | `PresenceRelatedPersonPickResponsibleResutModel[]` |
| PickupSuggestions | `PresencePickupSuggestionResultModel[]` |

#### GetPickupResponsibleResultModel

| Property | Type |
|----------|------|
| Children | `List<GetPickupResponsibleChildResultModel>` |

#### PresencePickupSuggestionResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| UniStudentId | `int64` |
| PickUpName | `string` |

#### PresenceRelatedPersonPickResponsibleResutModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `Nullable<int64>` |
| Name | `string` |
| Relation | `string` |

#### SavePickupNameRequestModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

### Models.ComeGo.PresenceConfiguration

#### PresenceConfigurationChildResultModel

| Property | Type |
|----------|------|
| UniStudentId | `int64` |
| PresenceConfiguration | `PresenceConfigurationResultModel` |

#### PresenceConfigurationDepartment

| Property | Type |
|----------|------|
| Group | `PresenceConfigurationGroup` |
| FilteringGroups | `PresenceConfigurationGroup[]` |

#### PresenceConfigurationGroup

| Property | Type |
|----------|------|
| Id | `int32` |
| Name | `string` |

#### PresenceConfigurationInstitution

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| Name | `string` |

#### PresenceConfigurationResultModel

| Property | Type |
|----------|------|
| SelfDecider | `bool` |
| GoHomeWith | `bool` |
| SendHome | `bool` |
| PickUp | `bool` |
| Institution | `PresenceConfigurationInstitution` |
| Departments | `PresenceConfigurationDepartment[]` |
| DashboardModuleSettings | `List<PresenceModuleSettings>` |

#### PresenceModule

| Property | Type |
|----------|------|
| ModuleType | `PresenceModuleSettingsModule` |
| Permission | `PresenceModuleSettingsPermission` |

#### PresenceModuleSettings

| Property | Type |
|----------|------|
| PresenceDashboardContext | `PresenceModuleSettingsDashboardContext` |
| PresenceModules | `List<PresenceModule>` |

### Models.ComeGo.UpdateLocation

#### UpdateLocationRequestModel

| Property | Type |
|----------|------|
| ChildIds | `int64[]` |
| LocationId | `Nullable<int64>` |

### Models.Comments.DTO

#### CommentResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Creator | `SimpleInstitutionProfile` |
| Content | `string` |
| CreatedAt | `DateTime` |
| UpdatedAt | `Nullable<DateTime>` |
| Comments | `List<CommentResultModel>` |
| CommentCount | `int32` |
| CanDelete | `bool` |
| CanReport | `bool` |
| IsDeleted | `bool` |
| IsReported | `bool` |

#### PagedCommentList

| Property | Type |
|----------|------|
| StartIndex | `int32` |
| Limit | `int32` |
| TotalResultCount | `int32` |
| Comments | `List<CommentResultModel>` |
| CommentableInstitutionProfiles | `List<CommentableInstitutionProfile>` |

### Models.Comments.Parameter

#### CommentItem

| Property | Type |
|----------|------|
| Type | `CommentType` |
| Id | `int64` |

#### DeleteCommentRequestModel

| Property | Type |
|----------|------|
| CommentId | `int64` |
| ParentType | `CommentType` |

#### ReportCommentApiParameters

| Property | Type |
|----------|------|
| CommentId | `int64` |
| ReportReason | `string` |

#### UpdateCommentRequestModel

| Property | Type |
|----------|------|
| CommentId | `int64` |
| Content | `string` |

### Models.Common

#### ConfirmBoxTexts

| Property | Type |
|----------|------|
| Title | `string` |
| Description | `string` |
| OKBtnTxt | `string` |
| CancelBtnTxt | `string` |
| IsHtmlText | `bool` |
| ShowConfirmBox | `bool` |
| ShouldDismissOnOutsideClick | `bool` |
| ShowCloseButton | `bool` |
| ShowLogo | `bool` |
| ShowTextFields | `bool` |
| FirstTextFieldLabel | `string` |
| SecondTextFieldLabel | `string` |
| GetTextFieldsValueHandler | `System.Action`2<string,string>` |
| ShouldCloseWorkAsCancel | `bool` |

### Models.Common.Api

#### ProfileApiDto

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |
| FullName | `string` |
| Role | `string` |
| ShortName | `string` |
| MainGroupName | `string` |
| Metadata | `string` |
| Institution | `Institution` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

### Models.Common.Api.Files

#### FileConnectionResult

| Property | Type |
|----------|------|
| FileName | `string` |
| Mime | `string` |
| FilePath | `string` |
| Length | `int64` |

### Models.Common.Api.Files.Enums

#### MediaTypeEnumExtension

*(no public properties detected)*

### Models.Common.Api.Files.Parameters

#### AddOrRemoveTagArguments

| Property | Type |
|----------|------|
| InstProfileId | `int64` |
| MediaId | `int64` |

#### CompleteMultipartUploadPartRequest

| Property | Type |
|----------|------|
| ETag | `string` |
| PartNumber | `string` |

#### CompleteMultipartUploadingRequest

| Property | Type |
|----------|------|
| FileId | `int64` |
| Parts | `CompleteMultipartUploadPartRequest[]` |

#### CreateAttachmentsArguments

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| OwnerInstitutionProfileId | `int64` |
| Media | `List<AttachmentMediaFileUploadArguments>` |
| Links | `List<AttachmentLinkUploadArguments>` |
| Files | `List<AttachmentFileUploadArguments>` |
| AttachedSecureDocumentIds | `List<int64>` |

#### DownloadFileFromAulaArguments

| Property | Type |
|----------|------|
| Name | `string` |
| Url | `string` |

#### FileResultDto : BaseResultDto

| Property | Type |
|----------|------|
| Name | `string` |
| MultipartFileUploadInformation | `FileUploadInformation` |
| UploadId | `string` |

#### GetUploadLinksArguments

| Property | Type |
|----------|------|
| UploadNames | `string[]` |
| InstitutionCode | `string` |

#### UploadFileContentArguments : UploadFileKeyInfo

| Property | Type |
|----------|------|
| Name | `string` |
| Id | `Nullable<int64>` |

#### UploadFileToAmazonArguments : UploadFileData

| Property | Type |
|----------|------|
| Url | `string` |
| File | `System.IO.Stream` |

#### UploadFileToAulaArguments

| Property | Type |
|----------|------|
| Size | `Nullable<float32>` |
| Creator | `ObjectWithId` |
| File | `UploadFileContentArguments` |
| Media | `UploadMediaContentArguments` |
| Link | `UploadLinkContentArguments` |
| Id | `Nullable<int64>` |
| Name | `string` |
| IsLoading | `bool` |

#### UploadLinkContentArguments

| Property | Type |
|----------|------|
| ExternalFileId | `string` |
| AccessToken | `string` |
| Service | `string` |

#### UploadMediaContentArguments

| Property | Type |
|----------|------|
| Duration | `Nullable<float64>` |
| Tags | `List<ObjectWithId>` |
| Title | `string` |
| Description | `string` |
| MediaType | `string` |
| File | `UploadFileContentArguments` |

### Models.Common.Api.Files.Parameters.AttachmentFeatureV2

#### AttachmentFileUploadArguments : BaseFileUploadArguments

| Property | Type |
|----------|------|
| Name | `string` |

#### AttachmentLinkUploadArguments

| Property | Type |
|----------|------|
| ExternalFileId | `string` |
| AccessToken | `string` |
| Service | `string` |

#### AttachmentMediaFileUploadArguments : BaseFileUploadArguments

| Property | Type |
|----------|------|
| Id | `Nullable<int64>` |
| AlbumId | `Nullable<int64>` |
| Name | `string` |
| MediaType | `string` |
| Tags | `List<int64>` |
| Title | `string` |
| Description | `string` |

#### BaseFileUploadArguments : BaseMetadataFileUploadArguments

| Property | Type |
|----------|------|
| UploadId | `string` |
| MultipartUploadingInfo | `MultipartUploadingInfoArguments` |

#### BaseMetadataFileUploadArguments

*(no public properties detected)*

#### MultipartUploadingInfoArguments

| Property | Type |
|----------|------|
| NumberOfPart | `int32` |

#### UpdateAttachmentsArguments

| Property | Type |
|----------|------|
| Media | `List<AttachmentMediaFileUploadArguments>` |

### Models.Common.Api.Files.Result

#### AulaDocumentLinkContent

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| CanAccess | `bool` |
| DocumentType | `string` |
| IsDeleted | `bool` |

#### AulaFileAlbumDto

| Property | Type |
|----------|------|
| Name | `string` |
| SharedGroups | `List<ShareWithGroupDto>` |

#### AulaFileContent

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Url | `string` |
| Bucket | `string` |
| Key | `string` |
| Created | `DateTime` |
| ScanningStatus | `FileScanningStatus` |

#### AulaFileResultDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Creator | `AulaFileResultProfileDto` |
| File | `AulaFileContent` |
| Media | `AulaMediaFileContent` |
| Link | `AulaLinkContent` |
| Document | `AulaDocumentLinkContent` |
| Status | `FileStatusEnum` |

#### AulaFileResultProfileDto

| Property | Type |
|----------|------|
| InstProfileId | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Role | `string` |
| ProfileId | `int64` |
| Name | `string` |
| ShortName | `string` |
| Metadata | `string` |
| ProfilePicture | `AulaFileContent` |

#### AulaGalleryMediaFileResultDto : AulaMediaFileContent

| Property | Type |
|----------|------|
| Creator | `AulaFileResultProfileDto` |
| Id | `int64` |
| CommentCount | `int32` |

#### AulaLinkContent

| Property | Type |
|----------|------|
| Service | `string` |
| Name | `string` |
| Url | `string` |

#### AulaMediaFileContent

| Property | Type |
|----------|------|
| Title | `string` |
| Description | `string` |
| AllowsComments | `bool` |
| CanViewComments | `bool` |
| File | `AulaFileContent` |
| MediaType | `string` |
| Tags | `List<AulaFileResultProfileDto>` |
| DurationNumber | `Nullable<float64>` |
| Album | `AulaFileAlbumDto` |
| ThumbnailUrl | `string` |
| LargeThumbnailUrl | `string` |
| MediumThumbnailUrl | `string` |
| SmallThumbnailUrl | `string` |
| ExtraSmallThumbnailUrl | `string` |
| HasVideoThumbnail | `bool` |
| CurrentUserCanDelete | `bool` |
| CurrentUserCanEditMetadata | `bool` |
| CurrentUserCanReport | `bool` |
| CurrentUserCanEditTags | `bool` |
| IsUploadingPending | `bool` |
| ConversionStatus | `ConversionStatusEnum` |

#### AuthorizedFileFormat

| Property | Type |
|----------|------|
| Id | `int64` |
| FileFormat | `string` |
| Name | `string` |

#### BaseResultDto

| Property | Type |
|----------|------|
| Id | `int64` |

#### CreateAttachmentsResult

| Property | Type |
|----------|------|
| Media | `List<FileResultDto>` |
| Files | `List<FileResultDto>` |
| Documents | `List<FileResultDto>` |
| Links | `List<LinkResultDto>` |
| IsAllConsentsValid | `bool` |

#### CreateMediaResult

| Property | Type |
|----------|------|
| AllImagesHasValidConsents | `bool` |
| Media | `List<AulaFileResultDto>` |

#### DeleteMediaParameters

| Property | Type |
|----------|------|
| MediaIds | `int64[]` |

#### DocumentLinkResult

| Property | Type |
|----------|------|
| Id | `int64` |
| DocumentId | `int64` |

#### FilePartUploadInformation

| Property | Type |
|----------|------|
| PartIndex | `string` |
| PreSignedUrl | `string` |

#### FileUploadInformation

| Property | Type |
|----------|------|
| Parts | `FilePartUploadInformation[]` |
| AwsUploadId | `string` |

#### LinkResultDto : BaseResultDto

| Property | Type |
|----------|------|
| Service | `string` |
| Name | `string` |
| Url | `string` |

#### MediaTagConsentsResult

| Property | Type |
|----------|------|
| IsAllConsentsValid | `bool` |

#### ShareWithGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| PortalRoles | `PortalRole[]` |
| Name | `string` |
| ShortName | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MembershipCount | `MembershipCountResultModel` |
| AllowMembersToBeShown | `bool` |

#### UploadFileData

| Property | Type |
|----------|------|
| Policy | `string` |
| AmzAlgorithm | `string` |
| AmzCredential | `string` |
| AmzDate | `string` |
| AmzSecurityToken | `string` |
| AmzSignature | `string` |
| Acl | `string` |
| Key | `string` |
| Bucket | `string` |
| CacheControl | `string` |

#### UploadFileInfo

| Property | Type |
|----------|------|
| Key | `UploadFileKeyInfo` |

#### UploadFileKeyInfo

| Property | Type |
|----------|------|
| Key | `string` |
| Bucket | `string` |

#### UploadLink

| Property | Type |
|----------|------|
| Action | `string` |
| File | `UploadFileInfo` |
| Data | `UploadFileData` |

#### UploadToAwsServiceResultDto

| Property | Type |
|----------|------|
| UploadFileToAulaContentArguments | `UploadFileContentArguments` |
| Exception | `UploadFileException` |

#### UploadToAwsServiceResultDtoV2

| Property | Type |
|----------|------|
| Exception | `UploadFileException` |

### Models.Common.CommonFile

#### CommonFileModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| File | `DownloadFileFromAulaArguments` |

### Models.Configuration

#### LoginImportantInformationRequestModel

| Property | Type |
|----------|------|
| Platform | `string` |

#### PrivacyPolicyResultModel

| Property | Type |
|----------|------|
| Id | `int64` |
| Content | `RichTextWrapperDto` |
| ChangesDescription | `RichTextWrapperDto` |

### Models.Consents

#### ConsentResponsesDTO

| Property | Type |
|----------|------|
| id | `int64` |
| ConsentId | `int32` |
| AllowedAnswers | `string[]` |
| ConsentDescription | `string` |
| ConsentResponseAnswer | `string` |
| ConsentResponseStatus | `string` |
| Editable | `bool` |
| ViewOnlyDependency | `Nullable<int32>` |
| ViewOrder | `int32` |
| FromDate | `Nullable<DateTime>` |
| ToDate | `Nullable<DateTime>` |

#### InstitutionProfileConsentDTO

| Property | Type |
|----------|------|
| InstitutionProfile | `InstitutionProfileConsent` |
| ConsentResponses | `List<ConsentResponsesDTO>` |

### Models.Document

#### CommonFileDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Attachment | `AulaFileResultDto` |
| Created | `DateTime` |
| Institution | `CommonFileDto/CommonFileInstitutionDto` |
| IsDataPolicy | `bool` |
| Title | `string` |
| ProfileTypeRestrictions | `List<PortalRole>` |
| GroupRestrictions | `List<CommonFileDto/CommonFileGroupRestrictionDto>` |

#### CommonFileDto.CommonFileGroupRestrictionDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCode | `string` |

#### CommonFileDto.CommonFileInstitutionDto

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| Name | `string` |

#### DocumentRevisionDto

| Property | Type |
|----------|------|
| Id | `int64` |
| CreatedBy | `string` |
| CreatedAt | `DateTime` |
| Title | `string` |
| ChangeType | `RevisionChangeTypeEnum` |
| SharedWith | `List<SimpleInstitutionProfile>` |
| UnsharedWith | `List<SimpleInstitutionProfile>` |
| IsAvailable | `bool` |
| RecipientName | `string` |
| ChildrenNames | `string[]` |

#### DocumentRevisionPageDto

| Property | Type |
|----------|------|
| TotalCount | `int32` |
| DocumentRevisionDtos | `List<DocumentRevisionDto>` |

#### ExternalSecureDocumentDetailsDto : SecureDocumentDto

| Property | Type |
|----------|------|
| Attachment | `AulaFileResultDto` |

#### GetImplicitSharingsDto

| Property | Type |
|----------|------|
| ImplicitSharings | `List<ImplicitSharingProfileDto>` |

#### ImplicitSharingOverride

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| PermissionOverrideEnum | `ImplicitSharingPermissionOverride` |

#### ImplicitSharingProfileDto

| Property | Type |
|----------|------|
| SimpleInstitutionProfileDto | `SecureDocumentShareWithInstitutionProfileDto` |
| PermissionOverrideEnum | `ImplicitSharingPermissionOverride` |

#### InternalSecureDocumentDetailsDto : SecureDocumentDto

| Property | Type |
|----------|------|
| Attachments | `AulaFileResultDto[]` |
| Content | `RichTextWrapperDto` |

#### SecureDocumentAssociateGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Name | `string` |
| MainGroup | `bool` |
| MembershipCount | `MembershipCountResultModel` |

#### SecureDocumentAssociateInstitutionProfileDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Alias | `bool` |
| Metadata | `string` |

#### SecureDocumentCreatorDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Alias | `bool` |
| Metadata | `string` |

#### SecureDocumentDto

| Property | Type |
|----------|------|
| Id | `int64` |
| HasMedia | `bool` |
| CanEdit | `bool` |
| CanEditLockedStatus | `bool` |
| IsLocked | `bool` |
| JournalingStatus | `JournalingStatusEnum` |
| Category | `string` |
| DocumentTemplateTitle | `string` |
| InstitutionCode | `string` |
| DocumentType | `string` |
| AssociatedInstitutionProfiles | `List<SecureDocumentAssociateInstitutionProfileDto>` |
| SharedWithGroups | `List<SecureDocumentShareWithGroupDto>` |
| SharedWithInstitutionProfiles | `List<SecureDocumentShareWithInstitutionProfileDto>` |
| ImplicitSharings | `List<ImplicitSharingProfileDto>` |
| Creator | `SecureDocumentCreatorDto` |
| CreatedAt | `Nullable<DateTime>` |
| Title | `string` |
| UpdatedAt | `Nullable<DateTime>` |
| UpdatedBy | `string` |
| Version | `Nullable<int32>` |
| Description | `string` |
| IsSharedWithGuardian | `bool` |
| IsShareable | `bool` |
| ShareableGuardianIds | `List<int64>` |
| TemplateTitle | `string` |

#### SecureDocumentExportDto

| Property | Type |
|----------|------|
| RequestExportJobId | `int64` |
| Status | `SecureDocumentExportStatus` |
| Progress | `float32` |
| FileUrl | `string` |
| FileName | `string` |

#### SecureDocumentShareWithGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| CanEdit | `bool` |
| AllowMembersToBeShown | `bool` |
| MembershipCount | `MembershipCountResultModel` |

#### SecureDocumentShareWithInstitutionProfileDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionName | `string` |
| CanEdit | `bool` |
| Alias | `bool` |
| Metadata | `string` |
| Role | `PortalRole` |

### Models.Document.Arguments

#### CreateDocumentArguments

| Property | Type |
|----------|------|
| Id | `int64` |
| Category | `string` |
| CreatorInstitutionProfileId | `int64` |
| RegardingInstitutionProfileIds | `List<int64>` |
| SharedWithGroups | `List<CreateDocumentArguments/CreateDocumentShareGroupArguments>` |
| SharedWithInstitutionProfiles | `List<CreateDocumentArguments/CreateDocumentSharedProfileArguments>` |
| Title | `string` |
| Version | `Nullable<int32>` |
| ForceUpdate | `Nullable<bool>` |
| AttachedThread | `AttachMessagesToSecureDocumentRequest` |

#### CreateDocumentArguments.CreateDocumentShareGroupArguments

| Property | Type |
|----------|------|
| GroupId | `int64` |
| CanEdit | `bool` |

#### CreateDocumentArguments.CreateDocumentSharedProfileArguments

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| CanEdit | `bool` |

#### CreateExternalDocumentArguments : CreateDocumentArguments

| Property | Type |
|----------|------|
| ExternalFile | `UploadFileToAulaArguments` |

#### CreateInternalDocumentArguments : CreateDocumentArguments

| Property | Type |
|----------|------|
| Content | `string` |
| AttachmentIds | `int64[]` |
| ImplicitSharingOverrides | `List<ImplicitSharingOverride>` |

#### GetCommonFilesArguments

| Property | Type |
|----------|------|
| Page | `int32` |
| SortType | `CommonFileSortEnum` |
| SortOrder | `SortOrderEnum` |

#### GetSecureDocumentsArguments

| Property | Type |
|----------|------|
| FilterInstitutionProfileIds | `int64[]` |
| FilterRegardingGroupIds | `int64[]` |
| FilterUnread | `Nullable<bool>` |
| FilterLocked | `Nullable<bool>` |
| FilterJournalingStatus | `Nullable<JournalingStatusEnum>` |
| FilterEditable | `bool` |
| DocumentType | `Nullable<DocumentTypeEnum>` |
| Sortings | `List<SortingModel>` |
| Index | `int32` |
| Limit | `int32` |
| FilterRegardingStudentIds | `List<int64>` |
| FilterDocumentCategories | `List<DocumentCategoryEnum>` |

#### GetShareableSecureDocumentsArguments : GetSecureDocumentsArguments

| Property | Type |
|----------|------|
| ShareToInstitutionProfileIds | `int64[]` |

#### RemoveSharingArguments

| Property | Type |
|----------|------|
| DocumentIds | `List<int64>` |

#### SortingModel

| Property | Type |
|----------|------|
| Field | `SecureDocumentSortEnum` |
| Order | `SortOrderEnum` |

#### UpdateSharingArguments

| Property | Type |
|----------|------|
| DocumentIds | `int64[]` |
| ResetSharings | `bool` |
| SharedGroups | `List<UpdateSharingArguments/UpdateSharingArgumentsGroup>` |
| SharedInstitutionProfiles | `List<UpdateSharingArguments/UpdateSharingArgumentsInstProfile>` |

#### UpdateSharingArguments.UpdateSharingArgumentsGroup

| Property | Type |
|----------|------|
| GroupId | `int64` |
| CanEdit | `bool` |

#### UpdateSharingArguments.UpdateSharingArgumentsInstProfile

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| CanEdit | `bool` |

### Models.Document.Requests

#### CreateExportForMultipleSecureDocumentsRequest

| Property | Type |
|----------|------|
| SecureDocumentIds | `IEnumerable<int64>` |

#### CreatePDFForSingleDocumentRequest

| Property | Type |
|----------|------|
| SecureDocumentId | `int64` |

#### TrackCreatePDFForSingleDocumentRequest

| Property | Type |
|----------|------|
| RequestId | `int64` |

#### TrackExportForMultipleSecureDocumentsRequest

| Property | Type |
|----------|------|
| RequestId | `int64` |

### Models.Document.Results

#### GetCommonFilesResult

| Property | Type |
|----------|------|
| CommonFiles | `List<CommonFileDto>` |
| TotalAmount | `int32` |

#### GetSecureDocumentsResult

| Property | Type |
|----------|------|
| Documents | `List<SecureDocumentDto>` |
| Filters | `GetSecureDocumentsResult/GetSecureDocumentsFilter` |
| TotalCount | `int32` |

#### GetSecureDocumentsResult.GetSecureDocumentsFilter

| Property | Type |
|----------|------|
| RegardingGroups | `List<SecureDocumentAssociateGroupDto>` |
| RegardingInstitutionProfiles | `List<GetSecureDocumentsResult/GetSecureDocumentsRegardingInstitutionProfile>` |
| DocumentCategories | `List<string>` |
| SharedGroups | `List<SecureDocumentAssociateGroupDto>` |
| SharedInstitutionProfiles | `List<GetSecureDocumentsResult/GetSecureDocumentsRegardingInstitutionProfile>` |

#### GetSecureDocumentsResult.GetSecureDocumentsRegardingInstitutionProfile

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

### Models.Extensions

#### DateTimeExtensions

*(no public properties detected)*

### Models.Files

#### UploadAttachmentServiceResult

| Property | Type |
|----------|------|
| MediaIds | `int64[]` |
| AllConsentIsValid | `bool` |
| IsSuccess | `bool` |

### Models.Gallery

#### AlbumDto

| Property | Type |
|----------|------|
| Id | `Nullable<int64>` |
| Title | `string` |
| Name | `string` |
| Creator | `AlbumCreatorDto` |
| CreationDate | `DateTime` |
| TotalSize | `int32` |
| Size | `int32` |
| From | `int32` |
| Description | `string` |
| SharedWithGroups | `List<ShareWithGroupDto>` |
| ThumbnailsUrls | `List<string>` |
| CurrentUserCanEdit | `bool` |
| CurrentUserCanDelete | `bool` |
| CurrentUserCanAddMedia | `bool` |

#### MediaCreatorModel

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Role | `string` |
| ProfileId | `int64` |
| Name | `string` |
| ShortName | `string` |
| Metadata | `string` |

#### MediaListDto

| Property | Type |
|----------|------|
| Results | `List<AulaFileResultDto>` |
| Album | `AlbumDto` |

#### MediasInAlbumDto

| Property | Type |
|----------|------|
| Results | `List<AulaGalleryMediaFileResultDto>` |
| Album | `AlbumDto` |
| MediaCount | `int32` |

### Models.Gallery.GalleryDto

#### AlbumCreatorDto

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Name | `string` |
| ShortName | `string` |
| Metadata | `string` |
| ProfileId | `int64` |
| Role | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

#### AlbumGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Role | `string` |
| MainGroup | `Nullable<bool>` |

### Models.Gallery.GalleryParameters

#### CreateAlbumParameters

| Property | Type |
|----------|------|
| Title | `string` |
| AlbumId | `Nullable<int64>` |
| CreatorInstitutionProfileId | `int64` |
| SharedWithGroups | `LinkedGroupRequestModel[]` |
| Description | `string` |

#### DeleteAlbumParameters

| Property | Type |
|----------|------|
| AlbumIds | `int64[]` |

#### GalleryViewFilter

| Property | Type |
|----------|------|
| SelectedInstitutionCodeForFilter | `string` |
| AlbumId | `Nullable<int64>` |
| UserSpecificAlbum | `Nullable<bool>` |
| Limit | `int32` |
| Index | `int32` |
| SortOn | `string` |
| OrderDirection | `string` |
| FilterBy | `string` |

#### GetMediaInAlbumFilter

| Property | Type |
|----------|------|
| AlbumId | `Nullable<int64>` |
| UserSpecificAlbum | `Nullable<bool>` |
| Limit | `int32` |
| Index | `int32` |
| SortOn | `string` |
| OrderDirection | `string` |
| FilterBy | `string` |
| IsSelectionMode | `bool` |
| SelectedInstitutionCode | `string` |

### Models.Groups

#### GetGroupsResponse

| Property | Type |
|----------|------|
| Status | `WebResponseStatus` |
| Groups | `List<Group>` |

#### Group

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Description | `string` |
| MembershipType | `object` |
| MembershipInstitutions | `string[]` |
| Access | `string` |
| CurrentUserCanAccessGroupDashBoard | `bool` |
| Status | `string` |
| Role | `GroupRole` |
| DashboardEnabled | `bool` |
| InstitutionCode | `string` |
| Type | `GroupTypeEnum` |
| ValidGroupModules | `List<GroupModule>` |
| AllowMembersToBeShown | `bool` |
| ValidGroupWidgets | `List<GroupWidget>` |
| Memberships | `List<GroupMembership>` |
| EndTime | `Nullable<DateTime>` |

#### GroupMemberGroup : StubbedGroup

| Property | Type |
|----------|------|
| InstitutionCode | `string` |

#### GroupMembership

| Property | Type |
|----------|------|
| Id | `int64` |
| GroupRole | `GroupRole` |
| InactiveDate | `Nullable<DateTime>` |
| InstitutionProfile | `GroupMembershipInstitutionProfile` |
| GroupId | `int64` |
| MemberGroup | `GroupMemberGroup` |
| InstitutionRole | `InstitutionRole` |

#### GroupMembershipGroupingByProfileTypeViewModel

| Property | Type |
|----------|------|
| Role | `PortalRole` |
| Members | `List<GenericProfileItem>` |

#### GroupMembershipInstitutionProfile : InstitutionProfileBase

| Property | Type |
|----------|------|
| MainGroupName | `string` |
| Relations | `List<RecipientRelation>` |

#### RecipientRelation

| Property | Type |
|----------|------|
| InstProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |
| FullName | `string` |
| ShortName | `string` |
| Metadata | `string` |
| Role | `string` |
| MainGroupName | `string` |

#### SimpleGroupDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |

#### StubbedGroup

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

### Models.Groups.GetGroupByProfileContext

#### GroupByContextDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| ShowAsDefault | `bool` |

#### GroupByContextRequestModel

| Property | Type |
|----------|------|
| InstitutionCodes | `string[]` |
| ChildInstitutionProfileIds | `int64[]` |

#### GroupByContextResultModel

| Property | Type |
|----------|------|
| ProfilePicture | `DownloadFileFromAulaArguments` |
| DisplayName | `string` |
| ProfileId | `int64` |
| Groups | `List<GroupByContextDto>` |

### Models.Groups.GetMemberships

#### GetMembershipsRequestModel

| Property | Type |
|----------|------|
| GroupId | `int64` |
| Limit | `Nullable<int32>` |
| FilterOnlyMember | `bool` |
| PortalRoles | `PortalRole[]` |

### Models.Institutions

#### AdministrativeAuthority

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCodes | `string[]` |

#### Institution

| Property | Type |
|----------|------|
| Children | `List<ChildProfile>` |
| InstitutionProfileId | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionType | `Nullable<InstitutionTypeEnum>` |
| MunicipalityCode | `string` |
| InstitutionRole | `InstitutionRole` |
| Permissions | `List<Permission>` |
| Groups | `List<Group>` |
| AdministrativeAuthority | `AdministrativeAuthority` |
| CommunicationBlock | `bool` |
| Selected | `bool` |
| MailboxId | `Nullable<int32>` |
| ShortName | `string` |

#### InstitutionIdentity

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MunicipalityCode | `string` |
| MunicipalityName | `string` |
| AdministrativeAuthority | `AdministrativeAuthority` |

#### Permission

| Property | Type |
|----------|------|
| PermissionId | `PermissionEnum` |
| StepUp | `bool` |
| GroupScopes | `List<int32>` |
| InstitutionScope | `bool` |

#### SimpleInstitution

| Property | Type |
|----------|------|
| InstitutionName | `string` |
| InstitutionCode | `string` |

### Models.MessageThreads

#### GetMessageInfoLightDto

| Property | Type |
|----------|------|
| ThreadId | `int32` |
| Subject | `string` |
| IsSensitive | `bool` |
| Message | `MessageDto` |

#### GetMessageThreadSubscriptionsResponse

| Property | Type |
|----------|------|
| Status | `WebResponseStatus` |
| Data | `MessageThreadSubscriptionList` |

#### MailBox

| Property | Type |
|----------|------|
| Id | `int64` |
| Email | `string` |
| DisplayName | `string` |
| Relation | `string` |
| ShortName | `object` |

#### MessageParticipantDto

| Property | Type |
|----------|------|
| MailBoxOwner | `RecipientApiModel` |
| FullName | `string` |
| Metadata | `string` |
| LastReadMessageId | `string` |
| LastReadMessageTimestamp | `Nullable<DateTime>` |
| ShortName | `string` |
| ProfilePicture | `ProfilePictureDto` |

#### MessageParticipantRelationDto

| Property | Type |
|----------|------|
| Type | `InstitutionRole` |
| InstitutioName | `string` |
| Class | `string` |
| Children | `List<MessagesStubbedChild>` |

#### MessageRegardingChildren

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| DisplayName | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

#### MessageThread

| Property | Type |
|----------|------|
| StartedDateTime | `DateTime` |
| Subject | `string` |
| RequiredStepUp | `bool` |
| SensitivityLevel | `SensitivityLevel` |
| Creator | `User` |
| OtherRecipients | `IList<MessageThread/SimpleMessageThreadSubscription>` |
| ThreadId | `string` |
| IsForwarded | `bool` |

#### MessageThread.SimpleMessageThreadSubscription

| Property | Type |
|----------|------|
| DisplayName | `string` |
| Relation | `string` |
| ShortName | `string` |

#### MessageThreadLatestMessage

| Property | Type |
|----------|------|
| Id | `string` |
| ThreadId | `int64` |
| SendDateTime | `DateTime` |
| Text | `RichTextWrapperDto` |
| Sender | `MailBox` |
| NewRecipient | `MailBox` |
| HasAttachments | `bool` |
| PendingMedia | `bool` |

#### MessageThreadSubscription

| Property | Type |
|----------|------|
| Id | `int64` |
| LeaveTime | `Nullable<DateTime>` |
| Muted | `bool` |
| Marked | `bool` |
| Read | `bool` |
| Sensitive | `bool` |
| LastReadMessageId | `string` |
| InstitutionCode | `string` |
| Creator | `MessageParticipantDto` |
| Recipients | `MessageParticipantDto[]` |
| RegardingChildren | `List<MessageRegardingChildren>` |
| LatestMessage | `MessageThreadLatestMessage` |
| Subject | `string` |
| MessageDraft | `MessageDraft` |
| MailBoxOwner | `RecipientApiModel` |
| CurrentFolder | `IMessageFolderListItem` |
| SubscriptionId | `int64` |
| IsThreadOrSubscriptionDeleted | `bool` |
| SubscriptionType | `SubscriptionType` |
| NumberOfBundleItems | `Nullable<int64>` |
| ExtraRecipientsCount | `Nullable<int64>` |
| BundleId | `Nullable<int64>` |
| ThreadEntityLinkDto | `ThreadEntityLinkDto` |
| PrimarySubscriptionId | `Nullable<int64>` |

#### MessageThreadSubscriptionList

| Property | Type |
|----------|------|
| Threads | `List<MessageThreadSubscription>` |
| Page | `int32` |
| BundleId | `int64` |
| MoreMessagesExist | `bool` |

#### MessageThreadSubscriptionRelatedChild

| Property | Type |
|----------|------|
| Id | `int64` |
| DisplayName | `string` |

#### MessageThreadSubscriptionRelatedInstitution

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| Name | `string` |

#### ThreadEntityLinkDto

| Property | Type |
|----------|------|
| EntityId | `string` |
| ThreadType | `ThreadType` |

### Models.MessageThreads.Argument

#### AddRecipientArguments

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| Recipients | `List<RecipientApiModel>` |
| CommonInboxId | `Nullable<int64>` |

#### DeleteMessageRequest

| Property | Type |
|----------|------|
| MessageId | `string` |
| ThreadId | `int64` |

#### DeleteThreadArguments

| Property | Type |
|----------|------|
| SubscriptionIds | `int64[]` |
| ThreadIds | `int64[]` |
| CommonInboxId | `Nullable<int64>` |

#### EditMessageRequest : ReplyMessageArgument

| Property | Type |
|----------|------|
| MessageId | `string` |
| MessageRequest | `MessageContentRequest` |

#### GetMessageInfoLightRequest

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| MessageId | `string` |
| CommonInboxId | `Nullable<int64>` |
| OtpInboxId | `Nullable<int64>` |

#### LeaveThreadArguments

| Property | Type |
|----------|------|
| ThreadId | `int64` |

#### LeaveThreadsRequest

| Property | Type |
|----------|------|
| SubscriptionIds | `int64[]` |

#### MarkThreadsRequest

| Property | Type |
|----------|------|
| Marked | `bool` |
| ThreadIds | `int64[]` |
| SubscriptionIds | `int64[]` |
| CommonInboxId | `Nullable<int64>` |

#### MuteThreadRequestArguments

| Property | Type |
|----------|------|
| Muted | `bool` |
| Owner | `RecipientApiModel` |
| SubscriptionIds | `int64[]` |
| CommonInboxId | `Nullable<int64>` |
| ThreadIds | `int64[]` |

#### RecipientApiModel

| Property | Type |
|----------|------|
| Id | `int64` |
| OtpInboxId | `int64` |
| MailBoxOwnerType | `string` |
| ProfileId | `Nullable<int64>` |
| IsDeactivated | `bool` |
| IsDeleted | `bool` |
| PortalRole | `Nullable<PortalRole>` |

#### ReplyMessageArgument

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| Message | `MessageContentRequest` |
| CommonInboxId | `Nullable<int64>` |
| BundleId | `Nullable<int64>` |

#### SetLastMessageRequestArguments

| Property | Type |
|----------|------|
| MessageId | `string` |
| ThreadId | `int64` |
| CommonInboxId | `Nullable<int64>` |

#### SetSensitivityLevelRequest

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| SensitivityLevel | `int32` |
| CommonInboxId | `Nullable<int64>` |
| BundleId | `Nullable<int64>` |

#### ThreadChangeMessagingCenterArguments

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| ThreadIds | `IEnumerable<int64>` |
| ShouldRemoteRefresh | `bool` |
| MessageDraftUpdated | `bool` |
| LastestMessage | `string` |
| BundleId | `Nullable<int64>` |
| SingleMessageId | `string` |

#### UpdateMessageThreadsSubscriptionStatusRequest

| Property | Type |
|----------|------|
| SubscriptionIds | `List<int64>` |
| IsRead | `bool` |

### Models.MessageThreads.Argument.Folder

#### CreateFolderArguments

| Property | Type |
|----------|------|
| FolderName | `string` |
| CommonInboxId | `Nullable<int64>` |

#### GetCommonInboxesArguments

| Property | Type |
|----------|------|
| InstitutionProfileIds | `int64[]` |
| ShouldIncludeProfilePictureUrl | `bool` |

#### GetFoldersArguments

| Property | Type |
|----------|------|
| IncludeDeletedFolders | `bool` |
| CommonInboxId | `Nullable<int64>` |

#### MoveThreadsToFolderRequestArguments

| Property | Type |
|----------|------|
| ThreadIds | `int64[]` |
| SubscriptionIds | `int64[]` |
| FolderId | `Nullable<int64>` |
| CommonInboxId | `Nullable<int64>` |

#### UpdateFolderArguments

| Property | Type |
|----------|------|
| FolderId | `int64` |
| FolderName | `string` |

### Models.MessageThreads.Argument.GetMessages

#### GetMessagesForThreadArguments

| Property | Type |
|----------|------|
| ThreadId | `int64` |
| Page | `int32` |
| CommonInboxId | `Nullable<int64>` |

### Models.MessageThreads.Argument.StartNewThread

#### ForwardInfoRequestArguments

| Property | Type |
|----------|------|
| ForwardedThreadId | `int64` |
| ForwardedMessageIds | `string[]` |
| DirectReply | `bool` |
| ForwardSingleMessage | `bool` |
| DirectReplyToCreator | `bool` |

#### ForwardThreadRequestArguments : StartNewThreadRequestArguments

| Property | Type |
|----------|------|
| ForwardInfo | `ForwardInfoRequestArguments` |

#### MessageContentRequest

| Property | Type |
|----------|------|
| AttachmentIds | `int64[]` |
| Text | `string` |

#### StartNewThreadRequestArguments

| Property | Type |
|----------|------|
| Message | `MessageContentRequest` |
| Subject | `string` |
| Recipients | `RecipientApiModel[]` |
| BccRecipients | `RecipientApiModel[]` |
| Sensitive | `bool` |
| Creator | `RecipientApiModel` |

### Models.MessageThreads.AutoReply.Argument

#### SetAutoReplyArguments

| Property | Type |
|----------|------|
| ReplyText | `string` |
| EndDateTime | `Nullable<DateTime>` |
| StartDateTime | `DateTime` |

### Models.MessageThreads.AutoReply.Result

#### MessageAutoReplyResult

| Property | Type |
|----------|------|
| Id | `int64` |
| ReplyText | `RichTextWrapperDto` |
| EndDateTime | `Nullable<DateTime>` |
| StartDateTime | `DateTime` |

### Models.MessageThreads.Folders

#### Folder

| Property | Type |
|----------|------|
| Id | `Nullable<int32>` |
| Name | `string` |
| Type | `FolderType` |

#### GetFoldersResponse

| Property | Type |
|----------|------|
| Status | `WebResponseStatus` |
| data | `IList<Folder>` |

#### MessagingParticipantDto

| Property | Type |
|----------|------|
| AnswerDirectlyName | `string` |
| ProfilePicture | `ProfilePictureDto` |
| MailBoxOwner | `RecipientApiModel` |
| FullName | `string` |
| Metadata | `string` |

### Models.MessageThreads.Messages

#### DeleteMessageDto

| Property | Type |
|----------|------|
| DeletedAt | `DateTime` |

#### MessageRecipientRelationDto

| Property | Type |
|----------|------|
| Type | `string` |
| Class | `string` |
| Children | `List<MessagesStubbedChild>` |
| InstitutionName | `string` |

#### MessagesInThreadDto

| Property | Type |
|----------|------|
| Id | `int64` |
| FirstMessage | `MessageDto` |
| Messages | `List<MessageDto>` |
| IsMarked | `bool` |
| ThreadCreator | `MessagesInThreadDto/RecipientDto` |
| ThreadStartedDateTime | `DateTime` |
| Recipients | `IEnumerable<MessagesInThreadDto/RecipientDto>` |
| MoreMessagesExist | `bool` |
| TotalMessageCount | `int32` |
| Page | `int32` |
| Subject | `string` |
| Muted | `bool` |
| Marked | `bool` |
| IsThreadForwarded | `bool` |
| Sensitive | `bool` |
| HasSecureDocuments | `bool` |
| MailboxOwner | `RecipientApiModel` |
| ThreadEntityLinkDto | `ThreadEntityLinkDto` |
| FolderName | `string` |

#### MessagesInThreadDto.RecipientDto

| Property | Type |
|----------|------|
| LastReadMessageId | `string` |
| LastReadTimeStamp | `Nullable<DateTime>` |
| LeaveTime | `Nullable<DateTime>` |
| DeletedAt | `Nullable<DateTime>` |
| FullName | `string` |
| ShortName | `string` |
| MailBoxOwner | `RecipientApiModel` |
| Metadata | `string` |
| ProfilePicture | `ProfilePictureDto` |

#### MessagesStubbedChild

| Property | Type |
|----------|------|
| Id | `int64` |
| DisplayName | `string` |
| Class | `string` |
| InstitutionName | `string` |

### Models.MessageThreads.Result

#### CommonInboxesDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| Address | `string` |
| Folders | `IList<Folder>` |
| Participants | `IList<MessagingParticipantDto>` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| CommonInboxType | `CommonInboxType` |

#### GetThreadListArguments

| Property | Type |
|----------|------|
| FolderId | `Nullable<int64>` |
| FilterType | `FilterAndSortType` |
| SortType | `FilterAndSortType` |
| SortOrder | `SortOrderEnum` |
| Page | `int32` |
| ThreadIds | `int64[]` |
| MailBoxOwnerType | `Nullable<RecipientApiType>` |
| MailBoxOwners | `List<int64>` |
| ActiveChildren | `List<int64>` |

#### GetThreadsInBundleArguments

| Property | Type |
|----------|------|
| BundleId | `int64` |

### Models.Messages

#### MessageDto

| Property | Type |
|----------|------|
| Id | `string` |
| MessageType | `string` |
| SendDateTime | `DateTime` |
| Text | `RichTextWrapperDto` |
| Sender | `MessageRecipient` |
| CanReplyToMessage | `bool` |
| Attachments | `List<AulaFileResultDto>` |
| NewRecipient | `MessageRecipient` |
| NewRecipients | `List<MessageRecipient>` |
| OriginalRecipients | `List<MessageRecipient>` |
| LeaverName | `string` |
| InviterName | `string` |
| LeaverNames | `List<string>` |

#### MessageFileUrl

| Property | Type |
|----------|------|
| Url | `string` |

#### MessageRecipient

| Property | Type |
|----------|------|
| ShortName | `string` |
| FullName | `string` |
| AnswerDirectlyName | `string` |
| MailBoxOwner | `RecipientApiModel` |
| ProfilePicture | `DownloadFileFromAulaArguments` |
| Metadata | `string` |

#### UpdateBundleMessageDto

| Property | Type |
|----------|------|
| IsMarked | `bool` |
| IsSensitive | `bool` |
| IsUnread | `bool` |
| Thread | `MessageThreadSubscription` |
| LastestMessageDate | `DateTime` |
| IsMuted | `bool` |

### Models.Messages.AttachMessagesToSecureDocument

#### AttachMessagesToSecureDocumentRequest

| Property | Type |
|----------|------|
| SecureDocumentId | `Nullable<int64>` |
| MessageIds | `string[]` |
| ThreadId | `int64` |
| CommonInboxId | `Nullable<int64>` |

### Models.Modules

#### GroupModule

| Property | Type |
|----------|------|
| Id | `int64` |
| ShowOnDashboard | `bool` |
| Module | `ModuleDto` |

### Models.Notification.Api

#### DeleteNotificationForRelatedChildRequestModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `Nullable<int64>` |
| RelatedChildInstitutionProfileId | `Nullable<int64>` |
| NotificationEventType | `NotificationEventType` |
| NotificationArea | `NotificationArea` |
| NotificationType | `NotificationType` |
| StartDate | `Nullable<DateTime>` |
| EndDate | `Nullable<DateTime>` |

#### DeleteNotificationParameter

| Property | Type |
|----------|------|
| NotificationId | `string` |
| InstitutionProfileId | `int64` |
| VacationRegistrationResponseId | `int64` |

#### DeleteNotificationsDto

| Property | Type |
|----------|------|
| Notifications | `DeleteNotificationParameter[]` |

#### GetNotificationsApiParameter

| Property | Type |
|----------|------|
| ActiveChildrenIds | `int64[]` |
| ActiveInstitutionCodes | `string[]` |
| OtherCalendarsOwnerProfileId | `Nullable<int64>` |

### Models.PersonalReferenceData

#### GetPersonalReferenceDataAnswerRequestModel

| Property | Type |
|----------|------|
| InstitutionProfileId | `Nullable<int64>` |
| GroupId | `Nullable<int64>` |
| Order | `SortOrderEnum` |
| SortField | `GetPersonalReferenceDataOrderFieldEnum` |
| Filter | `Nullable<ContactListFilteringProfileTypeEnum>` |
| IsConsent | `bool` |
| AdditionalDataId | `Nullable<int64>` |
| ConsentId | `Nullable<int64>` |
| Page | `int32` |

#### GetPersonalReferenceDataQuestionRequestModel

| Property | Type |
|----------|------|
| GroupId | `Nullable<int64>` |
| InstitutionProfileId | `Nullable<int64>` |

#### PersonalReferenceDataAnswerResultModel

| Property | Type |
|----------|------|
| ProfilePicture | `DownloadFileFromAulaArguments` |
| Name | `string` |
| Response | `string` |
| YesNoResponse | `Nullable<bool>` |
| ShortName | `string` |
| Alias | `bool` |
| MissingResponse | `bool` |
| ShouldAnswer | `bool` |
| InstitutionProfileId | `int64` |
| Metadata | `string` |

#### PersonalReferenceDataQuestionResultModel

| Property | Type |
|----------|------|
| CombinedAdditionalDataList | `PersonalReferenceQuestionAdditionalDataDto[]` |
| ConsentList | `PersonalReferenceQuestionConsentDto[]` |

#### PersonalReferenceQuestionAdditionalDataDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Permission | `bool` |
| Question | `string` |
| OptionalQuestionForNo | `string` |
| OptionalQuestionForYes | `string` |

#### PersonalReferenceQuestionConsentDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |

### Models.Posts.Api

#### CreatePostApiParameter

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| Content | `string` |
| InstitutionCode | `string` |
| CreatorInstitutionProfileId | `Nullable<int64>` |
| AllowComments | `bool` |
| IsImportant | `bool` |
| ImportantFrom | `string` |
| ImportantTo | `string` |
| SharedWithGroups | `LinkedGroupRequestModel[]` |
| AttachmentIds | `int64[]` |
| PublishAt | `DateTime` |
| ExpireAt | `DateTime` |

#### CreatePostResult

| Property | Type |
|----------|------|
| AllImagesHasValidConsents | `bool` |

#### GetPostApiParameters

| Property | Type |
|----------|------|
| GroupId | `Nullable<int64>` |
| IsImportant | `Nullable<bool>` |
| CreatorPortalRole | `string` |
| InstitutionProfileIds | `int64[]` |
| RelatedInstitutions | `string[]` |
| OwnPost | `bool` |
| IsUnread | `bool` |
| IsBookmarked | `bool` |
| Limit | `int32` |
| Index | `int32` |

#### GetPostApiResult

| Property | Type |
|----------|------|
| HasMorePosts | `bool` |
| PaginationStart | `DateTime` |
| PaginationEnd | `DateTime` |
| Page | `int32` |
| Posts | `IEnumerable<PostApiDto>` |

#### PostApiDto

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| Content | `RichTextWrapperDto` |
| CommentCount | `int32` |
| TimeStamp | `DateTime` |
| OwnerProfile | `ProfileApiDto` |
| AllowComments | `bool` |
| IsImportant | `bool` |
| ImportantFrom | `Nullable<DateTime>` |
| ImportantTo | `Nullable<DateTime>` |
| RelatedProfiles | `IEnumerable<ProfileApiDto>` |
| SharedWithGroups | `IEnumerable<ShareWithGroupDto>` |
| Attachments | `List<AulaFileResultDto>` |
| CanCurrentUserReport | `bool` |
| CanCurrentUserDelete | `bool` |
| CanCurrentUserComment | `bool` |
| PublishAt | `DateTime` |
| ExpireAt | `DateTime` |
| EditedAt | `Nullable<DateTime>` |
| IsBookmarked | `bool` |

#### ReportApiParameter

| Property | Type |
|----------|------|
| Id | `int64` |
| ReportReason | `string` |

### Models.ProfileModels

#### Address

| Property | Type |
|----------|------|
| Street | `string` |
| PostalCode | `string` |
| PostalDistrict | `string` |

#### BlockedCommunication

| Property | Type |
|----------|------|
| Child | `bool` |
| Employee | `bool` |
| Guardian | `bool` |
| IsBlockedAllProfileTypes | `bool` |

#### ChildProfile

| Property | Type |
|----------|------|
| InstCode | `string` |
| Name | `string` |
| ShortName | `string` |
| InstProfileId | `int64` |
| ProfileId | `int64` |
| ProfilePicture | `ProfilePictureDto` |
| UserId | `string` |
| HasCustodyOrExtendedAccess | `bool` |
| Selected | `bool` |

#### ChildRelationsProfile

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |
| InstitutionCode | `string` |
| Role | `string` |
| AulaEmail | `string` |

#### EditorPluginDetail

| Property | Type |
|----------|------|
| Name | `string` |
| MunicipalCode | `string` |
| InstitutionType | `InstitutionTypeEnum` |

#### InstitutionProfile : InstitutionProfileBase

| Property | Type |
|----------|------|
| InstitutionRole | `InstitutionRole` |
| CommunicationBlock | `bool` |
| UploadBlock | `bool` |
| Email | `string` |
| Phone | `string` |
| Address | `Address` |
| Birthday | `Nullable<DateTime>` |
| Relations | `List<RelationProfile>` |
| Alias | `bool` |
| GroupHomeId | `Nullable<int64>` |
| Institution | `InstitutionIdentity` |

#### InstitutionProfileBase

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| ProfileId | `int64` |
| UniPersonId | `int64` |
| MailBoxId | `Nullable<int64>` |
| FirstName | `string` |
| LastName | `string` |
| FullName | `string` |
| ShortName | `string` |
| Metadata | `string` |
| Role | `string` |
| EncryptionKey | `string` |
| ProfilePicture | `ProfilePictureDto` |
| MainGroup | `string` |

#### InstitutionProfileChild : InstitutionProfileBase

| Property | Type |
|----------|------|
| InstitutionRole | `InstitutionRole` |
| CommunicationBlock | `bool` |
| UploadBlock | `bool` |
| Email | `string` |
| Phone | `string` |
| Address | `Address` |
| Birthday | `Nullable<DateTime>` |
| Relations | `List<RelationProfile>` |
| Alias | `bool` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MunicipalityCode | `string` |
| MunicipalityName | `string` |

#### InstitutionProfileConsent : InstitutionProfileBase

| Property | Type |
|----------|------|
| Institution | `InstitutionIdentity` |

#### MainGroup

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstitutionCode | `string` |
| IsMainGroup | `bool` |

#### ModuleConfigurationDto

| Property | Type |
|----------|------|
| Id | `int32` |
| Module | `ModuleDto` |
| Order | `int32` |
| AggregatedDisplayMode | `string` |

#### ModuleDto

| Property | Type |
|----------|------|
| Id | `int32` |
| Name | `string` |
| Icon | `string` |
| Url | `string` |
| Type | `string` |
| Ordering | `int32` |
| CanBePlacedOnGroup | `bool` |

#### ModuleType

| Property | Type |
|----------|------|
| (const) TYPE_OVERVIEW | `string` |
| (const) TYPE_CALENDAR | `string` |
| (const) TYPE_MESSAGE | `string` |
| (const) TYPE_FILE_DOCUMENT | `string` |
| (const) TYPE_COME_GO | `string` |
| (const) TYPE_GALLERY | `string` |
| (const) TYPE_MORE | `string` |
| (const) TYPE_PERSONAL_REFERENCE_DATA | `string` |
| (const) TYPE_CONTACT_LIST | `string` |
| (const) TYPE_ACTIVITY_LIST | `string` |
| (const) TYPE_GROUPS | `string` |

#### PageConfiguration

| Property | Type |
|----------|------|
| WidgetConfigurations | `List<WidgetConfigurationDto>` |
| EditorPluginDetails | `List<EditorPluginDetail>` |

#### Profile

| Property | Type |
|----------|------|
| Id | `int64` |
| InstitutionProfile | `InstitutionProfile` |
| Groups | `List<Group>` |
| MunicipalGroups | `List<Group>` |
| Phonenumber | `string` |
| ExternalEmail | `string` |
| WorkPhonenumber | `string` |
| HomePhonenumber | `string` |
| MobilePhonenumber | `string` |
| Administrator | `object` |
| FirstName | `string` |
| LastName | `string` |
| UserId | `string` |
| PortalRole | `string` |
| IsSteppedUp | `bool` |
| GroupHomes | `List<object>` |
| IsGroupHomeAdmin | `bool` |
| PageConfiguration | `PageConfiguration` |

#### RelationProfile

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| ProfileId | `int64` |
| FirstName | `string` |
| FullName | `string` |
| LastName | `string` |
| MailBoxId | `int64` |
| ShortName | `string` |
| MainGroupName | `string` |
| Metadata | `string` |
| ProfilePicture | `AulaFileContent` |
| Institution | `Institution` |
| Role | `PortalRole` |

#### RoleDefinition

| Property | Type |
|----------|------|
| Id | `int32` |
| RoleName | `string` |

#### WidgetConfigurationDto

| Property | Type |
|----------|------|
| Id | `int32` |
| Widget | `WidgetDto` |
| Placement | `WidgetPlacementEnum` |
| AggregatedDisplayMode | `string` |
| Order | `int32` |

#### WidgetDto

| Property | Type |
|----------|------|
| Id | `int32` |
| Name | `string` |
| Icon | `string` |
| IconEmployee | `string` |
| IconHover | `string` |
| Url | `string` |
| Type | `string` |
| UsableForGroups | `bool` |
| Ordering | `int32` |
| WidgetId | `string` |
| WidgetVersion | `string` |
| CanAccessOnMobile | `bool` |

### Models.ProfileModels.Argument

#### InstitutionProfilesMasterDataRequestModel

| Property | Type |
|----------|------|
| InstitutionRoles | `string[]` |
| IsResponded | `bool` |

#### ProfileMasterDataArgument

| Property | Type |
|----------|------|
| InstProfileIds | `List<int64>` |
| FromAdministration | `bool` |

#### SetUserSessionArgument

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| AssuranceLevel | `int32` |

### Models.ProfileModels.Contact

#### ContactListInstitutionProfileResultModel : InstitutionProfile

| Property | Type |
|----------|------|
| ProfilePictureUrl | `string` |
| UserHasGivenConsentToShowContactInformation | `bool` |
| CurrentUserCanViewContactInformation | `bool` |

#### GetBaseProfileContactListRequestModel

| Property | Type |
|----------|------|
| Order | `SortOrderEnum` |
| Field | `GetProfileContactSortOrderFieldEnum` |
| Filter | `ContactListFilteringProfileTypeEnum` |
| Page | `int32` |

#### GetProfileContactListForContactParentRequestModel : GetBaseProfileContactListRequestModel

*(no public properties detected)*

#### GetProfileContactListRequestModel : GetBaseProfileContactListRequestModel

| Property | Type |
|----------|------|
| GroupId | `int64` |

### Models.ProfileModels.StubbedUsers

#### ChildMetadata

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Name | `string` |
| Id | `int64` |
| Metadata | `string` |
| ProfilePicture | `AulaFileContent` |

#### ComeGoInstitutionProfile

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| InstitutionProfileId | `int64` |
| Name | `string` |
| Role | `PortalRole` |
| ProfilePicture | `DownloadFileFromAulaArguments` |
| ShortName | `string` |
| InstitutionCode | `string` |

#### EmployeeMetadata

| Property | Type |
|----------|------|
| Name | `string` |
| InstitutionProfileId | `int64` |
| InstitutionRole | `InstitutionRole` |

#### SimpleInstitutionProfile

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| InstitutionProfileId | `int64` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Name | `string` |
| Role | `PortalRole` |
| MainGroup | `string` |
| ProfilePicture | `DownloadFileFromAulaArguments` |
| ShortName | `string` |
| Metadata | `string` |

#### StubbedUser

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| Name | `string` |
| PortalRole | `string` |
| UserId | `string` |

### Models.RemoteNotifications

#### ComeGoNotificationSettings

| Property | Type |
|----------|------|
| ComeGoType | `string` |
| Activated | `bool` |

#### ConfigureDeviceModel

| Property | Type |
|----------|------|
| CurrentToken | `string` |
| DeviceId | `string` |
| DeviceDescription | `string` |
| DeviceAccessGranted | `bool` |
| Platform | `Platform` |

#### DeviceModel : SimpleDevice

| Property | Type |
|----------|------|
| NotificationsActive | `bool` |
| LastActivity | `DateTime` |
| Description | `string` |
| AppType | `AppTypeEnum` |

#### DevicesModel

| Property | Type |
|----------|------|
| DeviceList | `List<DeviceModel>` |

#### NotificationSettings

| Property | Type |
|----------|------|
| ScheduledTime | `string` |
| Instant | `bool` |
| Monday | `bool` |
| Tuesday | `bool` |
| Wednesday | `bool` |
| Thursday | `bool` |
| Friday | `bool` |
| Saturday | `bool` |
| Sunday | `bool` |
| NotifyMessages | `bool` |
| NotifyMessagesFromEmployees | `bool` |
| NotifyMessagesFromChildren | `bool` |
| NotifyMessagesFromGuardians | `bool` |
| NotifyCalendar | `bool` |
| NotifyGallery | `bool` |
| NotifyPosts | `bool` |
| EmailDisabled | `bool` |
| EmailAvailable | `bool` |
| AppDisabled | `bool` |
| AppAvailable | `bool` |
| NotifyAssignedAsSubstituteTeacher | `bool` |
| NotifyLessonNoteChanged | `bool` |
| ComeGoNotificationSettings | `ComeGoNotificationSettings[]` |
| DeviceList | `List<SimpleDevice>` |
| WidgetSettings | `List<WidgetNotificationSettings>` |

#### RemoteNotification

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| ElementId | `string` |
| Id | `string` |
| Type | `string` |
| RelatedChildInstProfileId | `int64` |
| CommonInboxId | `Nullable<int64>` |
| CommentId | `Nullable<int32>` |
| OccurrenceDateTime | `Nullable<DateTime>` |
| ProfilePictureInstitutionProfileId | `Nullable<int64>` |

#### SimpleDevice

| Property | Type |
|----------|------|
| DeviceId | `string` |

#### WidgetNotificationSettings

| Property | Type |
|----------|------|
| Title | `string` |
| WidgetId | `int32` |
| IsActive | `bool` |

### Models.Resources

#### Resource

| Property | Type |
|----------|------|
| Id | `int32` |
| Name | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Category | `ResourceCategory` |
| Quantity | `int32` |
| Status | `int32` |
| Description | `string` |
| InactiveReason | `string` |
| Location | `string` |

#### ResourceCategory

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| ResourceType | `ResourceType` |

### Models.Resources.RequestModels

#### GetCalendarResourcesParameters

| Property | Type |
|----------|------|
| EventId | `Nullable<int64>` |
| Query | `string` |
| InstitutionProfileId | `int64` |
| InstitutionCode | `string` |
| Start | `Nullable<DateTime>` |
| End | `Nullable<DateTime>` |
| ResourceTypes | `string[]` |
| ExcludeResourceTypes | `string[]` |
| AllDay | `bool` |
| OccurenceLimit | `Nullable<int32>` |
| WeekdayMask | `bool[]` |
| DayInMonth | `Nullable<int32>` |
| RepeatTypeEnum | `Nullable<RepeatType>` |
| Interval | `Nullable<int32>` |
| MaxDate | `Nullable<DateTime>` |

#### GetResourceParameters

| Property | Type |
|----------|------|
| Query | `string` |
| InstitutionProfileId | `int64` |
| InstitutionCode | `string` |
| Start | `Nullable<DateTime>` |
| End | `Nullable<DateTime>` |
| ResourceCategories | `List<string>` |

### Models.Search

#### ChildRelationsResponse

| Property | Type |
|----------|------|
| ChildRelationsProfileList | `List<ChildRelationsProfile>` |
| SearchRecipientGroupList | `List<SearchResultGroupItem>` |

#### Creator

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |

#### MainGroup

| Property | Type |
|----------|------|
| Id | `string` |
| Name | `string` |

#### RelatedGroup

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |

#### RelatedProfile

| Property | Type |
|----------|------|
| FirstName | `string` |
| LastName | `string` |
| ProfileId | `int64` |
| InstitutionProfileId | `int64` |
| RelationType | `string` |
| AulaEmail | `string` |
| MainGroup | `MainGroup` |
| Metadata | `string` |

#### SearchGroupHome

| Property | Type |
|----------|------|
| Name | `string` |
| OtpInboxId | `int64` |
| Id | `int64` |

#### SearchResponse

| Property | Type |
|----------|------|
| TotalSize | `int32` |
| DocTypeCount | `List<SearchResultCountItem>` |
| GroupTypeCount | `List<SearchResultGroupCountItem>` |
| Results | `List<SearchResultItem>` |

#### SearchResultCommonFile : SearchResultItem

| Property | Type |
|----------|------|
| Title | `string` |
| Created | `DateTime` |
| FileKey | `string` |
| FileBucket | `string` |
| Url | `string` |
| FileName | `string` |
| ScanningStatus | `FileScanningStatus` |

#### SearchResultCommonInboxItem : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Score | `float32` |
| AulaEmail | `string` |

#### SearchResultCountItem

| Property | Type |
|----------|------|
| Name | `string` |
| Count | `int32` |

#### SearchResultEventItem : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| StartDateTime | `DateTime` |
| EndDateTime | `DateTime` |
| CreatorAulaName | `string` |
| Location | `string` |
| Type | `string` |

#### SearchResultGroupCountItem

| Property | Type |
|----------|------|
| Name | `string` |
| Count | `int32` |
| Key | `GroupTypeEnum` |

#### SearchResultGroupItem : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Status | `string` |
| Access | `string` |
| DashboardEnabled | `Nullable<bool>` |
| CurrentUserCanAccessGroupDashboard | `bool` |
| MembershipRole | `string` |
| Type | `GroupTypeEnum` |
| IsGroupMember | `Nullable<bool>` |
| ShortName | `string` |
| MembershipCount | `MembershipCountResultModel` |
| AllowMembersToBeShown | `bool` |
| Admins | `SearchResultGroupItem/SearchResultGroupAdmin[]` |

#### SearchResultGroupItem.SearchResultGroupAdmin

| Property | Type |
|----------|------|
| InstitutionProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |
| FullName | `string` |

#### SearchResultHighlight

| Property | Type |
|----------|------|
| Property | `string` |
| Fragment | `string` |

#### SearchResultItem

| Property | Type |
|----------|------|
| DocId | `string` |
| DocType | `string` |
| InstitutionCode | `string` |
| InstitutionName | `string` |
| MunicipalityCode | `string` |
| MunicipalityName | `string` |
| Name | `string` |
| Description | `string` |

#### SearchResultMediaItem : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Creator | `AulaFileResultProfileDto` |
| Tags | `List<AulaFileResultProfileDto>` |
| Title | `string` |
| AlbumTitle | `string` |
| AlbumDescription | `string` |
| ThumbnailUrl | `string` |
| LargeThumbnailUrl | `string` |
| MediumThumbnailUrl | `string` |
| SmallThumbnailUrl | `string` |
| HasVideoThumbnail | `bool` |
| ExtraSmallThumbnailUrl | `string` |
| MediaType | `string` |
| File | `AulaFileContent` |
| CurrentUserCanDelete | `Nullable<bool>` |
| CanComment | `bool` |
| CommentCount | `int32` |
| ConversionStatus | `ConversionStatusEnum` |

#### SearchResultPostItem : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Title | `string` |
| Content | `string` |
| Timestamp | `DateTime` |
| PublishAt | `DateTime` |
| EditedAt | `Nullable<DateTime>` |
| ReceiverGroups | `string[]` |
| Creator | `Creator` |

#### SearchResultProfileItemBase : SearchResultItem

| Property | Type |
|----------|------|
| ProfileId | `Nullable<int64>` |
| Address | `Address` |
| PortalRole | `string` |
| InstitutionRole | `Nullable<InstitutionRole>` |
| InstitutionProfileId | `int64` |
| HomePhoneNumber | `string` |
| MobilePhoneNumber | `string` |
| WorkPhoneNumber | `string` |
| FirstName | `string` |
| LastName | `string` |
| Gender | `string` |
| AulaEmail | `string` |
| ExternalEmail | `string` |
| RoleDefinitions | `List<RoleDefinition>` |
| MainGroup | `MainGroup` |
| ShortName | `string` |
| Metadata | `string` |
| RelatedGroups | `List<RelatedGroup>` |
| ProfilePicture | `DownloadFileFromAulaArguments` |

#### SearchResultProfileItemFindRecipients : SearchResultProfileItemBase

| Property | Type |
|----------|------|
| RelatedProfiles | `List<RelatedProfile>` |
| GroupHomes | `List<SearchGroupHome>` |

#### SearchResultProfileItemGlobalSearch : SearchResultProfileItemBase

| Property | Type |
|----------|------|
| Relations | `List<RelatedProfile>` |

#### SearchResultSecureFile : SearchResultItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Category | `string` |
| ChildAssociations | `List<SearchResultSecureFileChildAssociation>` |
| GroupAssociations | `List<SearchResultSecureFile/SearchResultSecureFileGroupAssociation>` |
| Created | `Nullable<DateTime>` |
| Edited | `Nullable<DateTime>` |
| CreatorName | `string` |
| Metada | `string` |
| Title | `string` |

#### SearchResultSecureFile.SearchResultSecureFileGroupAssociation

| Property | Type |
|----------|------|
| Id | `int64` |
| Name | `string` |
| InstituionCode | `string` |

#### SearchResultSecureFileChildAssociation

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| FirstName | `string` |
| LastName | `string` |

### Models.Search.RequestModels

#### GlobalSearchParameters

| Property | Type |
|----------|------|
| Text | `string` |
| PageLimit | `int32` |
| PageNumber | `int32` |
| GroupId | `Nullable<int64>` |
| DocTypeCount | `bool` |
| DocType | `SearchResultItemType` |
| GroupTypes | `GroupTypeEnum[]` |

#### SearchForAssociateSecureDocumentsParameter

| Property | Type |
|----------|------|
| InstitutionCodes | `string[]` |
| Text | `string` |
| FilterDelegate | `SearchRecipientsFilterDelegate` |

#### SearchForProfilesAndGroupsParameters

| Property | Type |
|----------|------|
| OnlyProfiles | `bool` |
| Text | `string` |
| PortalRoles | `SearchProfilePortalRoleEnum` |
| Typeahead | `bool` |
| Limit | `int32` |
| FilterDelegate | `SearchRecipientsFilterDelegate` |

#### SearchGroupRequestModel

| Property | Type |
|----------|------|
| Text | `string` |
| InstitutionCodes | `string[]` |
| Limit | `int32` |
| Offset | `int32` |
| FromModuleValue | `Nullable<SearchRecipientParameters/SearchRecipientModuleEnum>` |

#### SearchMessageRequestModel

| Property | Type |
|----------|------|
| (const) Limit | `int32` |
| Keyword | `string` |
| ThreadSubject | `string` |
| MessageContent | `string` |
| HasAttachments | `Nullable<bool>` |
| FromDate | `Nullable<DateTime>` |
| ToDate | `Nullable<DateTime>` |
| ThreadCreators | `List<RecipientViewModel>` |
| Participants | `List<RecipientViewModel>` |
| Page | `int32` |
| CommonInboxId | `Nullable<int64>` |
| FolderId | `Nullable<int64>` |
| Filter | `FilterAndSortType` |
| SortType | `FilterAndSortType` |
| SortOrder | `SortOrderEnum` |

#### SearchRecipientParameters

| Property | Type |
|----------|------|
| Text | `string` |
| FromModule | `Nullable<SearchRecipientParameters/SearchRecipientModuleEnum>` |
| DocTypes | `SearchRecipientParameters/SearchRecipientDocTypeEnum` |
| PortalRoles | `Nullable<SearchRecipientParameters/SearchRecipientPortalRoleEnum>` |
| GroupSearchScope | `Nullable<SearchRecipientParameters/GroupSearchScopeEnum>` |
| Limit | `int32` |
| ScopeEmployeesToInstitution | `Nullable<bool>` |
| GroupId | `Nullable<int32>` |
| InstCode | `string` |
| InstitutionCodes | `string[]` |
| RegardingChildren | `int64[]` |
| FilterDelegate | `SearchRecipientsFilterDelegate` |
| MailBoxOwnerType | `Nullable<SearchRecipientParameters/SearchRecipientMailBoxOwnerType>` |
| MailBoxOwnerId | `Nullable<int64>` |

#### SearchRecipientsFilterDelegate

*(no public properties detected)*

#### SearchResourceParameters

| Property | Type |
|----------|------|
| Query | `string` |
| InstitutionCode | `List<string>` |
| ExcludeTypes | `List<SearchResourceParameters/SearchResouceTypeEnum>` |
| IncludeTypes | `List<SearchResourceParameters/SearchResouceTypeEnum>` |

### Models.Search.SearchResultItems

#### SearchGroupItemResultModel

| Property | Type |
|----------|------|
| InstitutionCode | `string` |
| InstitutionName | `string` |
| Name | `string` |
| Id | `int64` |

#### SearchGroupResultModel

| Property | Type |
|----------|------|
| Results | `SearchGroupItemResultModel[]` |

### Models.Search.SearchResultItems.Messages

#### BaseSearchResultMessageItem : SearchResultItem

| Property | Type |
|----------|------|
| Marked | `bool` |
| Muted | `bool` |
| Thread | `SearchResultMessageThreadItem` |
| LeaveTime | `Nullable<DateTime>` |
| SensitivityLevel | `int32` |
| Read | `bool` |
| SelectedInMultiEditMode | `bool` |
| MessageDraft | `MessageDraft` |
| MailBoxOwner | `RecipientApiModel` |
| FolderId | `Nullable<int64>` |
| FolderName | `string` |
| SubscriptionId | `int64` |
| RegardingChildren | `List<MessageRegardingChildren>` |

#### SearchResultMessage

| Property | Type |
|----------|------|
| Id | `string` |
| Text | `RichTextWrapperDto` |
| SendDateTime | `Nullable<DateTime>` |
| SenderEmail | `string` |
| SenderDisplayName | `string` |
| MessageType | `string` |
| Unread | `bool` |

#### SearchResultMessageGlobalSearchItem : BaseSearchResultMessageItem

| Property | Type |
|----------|------|
| Message | `SearchResultMessage` |

#### SearchResultMessageItemSimple : SearchResultItem

| Property | Type |
|----------|------|
| MessageId | `string` |
| Message | `string` |
| SubscriptionId | `int64` |
| Author | `string` |
| Metadata | `string` |
| ThreadId | `int64` |
| Title | `string` |
| StepUpRequired | `bool` |
| LatestMessageSendTime | `Nullable<DateTime>` |
| MailBoxOwner | `RecipientApiModel` |

#### SearchResultMessageMessageModuleItem : BaseSearchResultMessageItem

| Property | Type |
|----------|------|
| SearchMessage | `SearchResultMessage` |
| Recipients | `MessageParticipantDto[]` |
| Creator | `MessageParticipantDto` |
| ExtraRecipientsCount | `Nullable<int64>` |

#### SearchResultMessageThreadItem

| Property | Type |
|----------|------|
| Id | `int64` |
| Subject | `string` |
| SensitivityLevel | `SensitivityLevel` |
| IsForwarded | `bool` |
| ThreadType | `ThreadType` |

#### SearchResultMessagesResponse

| Property | Type |
|----------|------|
| TotalHits | `int32` |
| Results | `List<SearchResultMessageMessageModuleItem>` |

### Models.SearchRecipients

#### SearchRecipientResponse

| Property | Type |
|----------|------|
| TotalHits | `int32` |
| Results | `List<SearchResultItem>` |

### Models.Users

#### ProfileContext

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| PortalRole | `string` |
| Institutions | `List<Institution>` |

#### User

| Property | Type |
|----------|------|
| Address | `string` |
| DisplayName | `string` |
| Email | `string` |
| FirstName | `string` |
| LastName | `string` |
| Highlights | `IList<SearchResultHighlight>` |

#### UserRelationship

| Property | Type |
|----------|------|
| ProfileId | `int64` |
| ChildRelationships | `List<string>` |
| InstitutionRelationships | `List<string>` |

### Models.Web

#### AulaErrorResponseWrapperStatus`1

| Property | Type |
|----------|------|
| ErrorInformation | `!0` |

#### AulaErrorResponseWrapper`1

| Property | Type |
|----------|------|
| Status | `AulaErrorResponseWrapperStatus`1<!0>` |

#### AulaServiceResponse

| Property | Type |
|----------|------|
| Url | `AulaUrl` |
| HttpMethod | `string` |
| RequestBody | `string` |
| ResultBody | `string` |
| ApiTransactionTrace | `string` |
| (const) ManualClientErrorCode | `int32` |

#### AulaServiceResponse`1 : AulaServiceResponse

| Property | Type |
|----------|------|
| ErrorContainer | `List<System.Exception>` |
| Data | `!0` |

#### AulaUrl

| Property | Type |
|----------|------|
| BaseUrl | `string` |
| Url | `string` |
| UrlCode | `int32` |

#### DataArrayResponse`1

| Property | Type |
|----------|------|
| TotalHits | `int32` |
| Results | `IEnumerable<!0>` |

#### PostResponse

| Property | Type |
|----------|------|
| Status | `WebResponseStatus` |

#### WebResponseStatus

| Property | Type |
|----------|------|
| HttpCode | `int32` |
| BackendErrorCode | `int32` |
| Message | `string` |
| PresentedMessage | `string` |
| SubCode | `Nullable<int32>` |
| HtmlContentIfError | `string` |
| ErrorInformation | `object` |
| Exception | `System.Exception` |

#### WebResponseStatusSubCodeConstants

| Property | Type |
|----------|------|
| (const) AUTHORIZATION_DENIED_ANY_SCOPE | `int32` |
| (const) AUTHORIZATION_DENIED_INSTITUTION_SCOPE | `int32` |
| (const) AUTHORIZATION_DENIED_GROUP_SCOPE | `int32` |
| (const) AUTHORIZATION_DENIED_PROFILE_SCOPE | `int32` |
| (const) AUTHORIZATION_DENIED_BLOCKED_COMMUNICATION | `int32` |
| (const) AUTHORIZATION_DENIED_ACCESS_NOT_GRANTED | `int32` |
| (const) AUTHORIZATION_DENIED_USER_DEACTIVATED | `int32` |
| (const) AUTHORIZATION_STEP_UP_REQUIRED | `int32` |
| (const) INVALID_TOKEN | `int32` |
| (const) OUT_OF_SYNC_PRESENCE_CONFIGURATION | `int32` |
| (const) UNREGISTER_DEVICE_FAILED | `int32` |
| (const) CROSS_MUNICIPALITY_TAGGING | `int32` |
| (const) SESSION_EXPIRED | `int32` |
| (const) EXCEEDING_MAXIMUM_PARTICIPANTS | `int32` |
| (const) DATE_ALREADY_HAS_OCCURRENCE_FROM_THE_SAME_SERIES | `int32` |
| (const) FIRST_REPEATING_EVENT_EXCEPTION_OUT_OF_RANGE | `int32` |
| (const) LAST_REPEATING_EVENT_EXCEPTION_OUT_OF_RANGE | `int32` |
| (const) DEACTIVATED_INSTITUTION_PROFILE | `int32` |
| (const) SECURE_DOCS_ONLY_SHARE_WITHIN_ONE_MUNICIPALITY | `int32` |

### Models.Widgets

#### GroupWidget

| Property | Type |
|----------|------|
| Id | `int64` |
| ShowOnDashboard | `bool` |
| Widget | `WidgetDto` |

#### MobileWidgetArguments

| Property | Type |
|----------|------|
| Group | `Nullable<int64>` |
| InstitutionFilter | `string[]` |
| ChildFilter | `string[]` |

#### SsoWidgetDirectLinkArguments

| Property | Type |
|----------|------|
| SessionUuid | `string` |
| IsMobileApp | `bool` |
| AulaToken | `string` |
| AssuranceLevel | `string` |
| UserProfile | `string` |
| ChildFilter | `string` |
| InstitutionFilter | `string` |
| Group | `Nullable<int64>` |
| CurrentWeekNumber | `string` |
| CsrfpToken | `string` |

### Services.Models.Messages

#### CreateThreadServiceResult

| Property | Type |
|----------|------|
| ThreadId | `Nullable<int32>` |
| Success | `bool` |

### Services.Web

#### AddCommentRequestModel

| Property | Type |
|----------|------|
| CommentableItem | `CommentItem` |
| Content | `string` |
| CreatorInstProfileId | `int64` |

#### AdditionalMasterDataService : RequestCachingService

*(no public properties detected)*

#### CalendarService : SimpleService

*(no public properties detected)*

#### CloudStorageOAuthService : CloudService

*(no public properties detected)*

#### ComeGoService : SimpleService

*(no public properties detected)*

#### CommentService : SimpleService

*(no public properties detected)*

#### CommentableInstitutionProfile : SimpleInstitutionProfile

| Property | Type |
|----------|------|
| Metadata | `string` |
| IsSelected | `bool` |

#### ConfigurationService : RequestCachingService

*(no public properties detected)*

#### ConsentService : SimpleService

*(no public properties detected)*

#### DocumentService : SimpleService

*(no public properties detected)*

#### FileDownloadException : 0xb1

*(no public properties detected)*

#### FileService : SimpleService

| Property | Type |
|----------|------|
| (const) MAXIMUM_PART_UPLOAD_LENGTH | `int32` |
| (const) BUFFER_SIZE | `int32` |

#### FolderService : SimpleService

*(no public properties detected)*

#### GalleryService : RequestCachingService

*(no public properties detected)*

#### GetCommmentsRequestModel

| Property | Type |
|----------|------|
| ParentType | `CommentType` |
| ParentId | `int64` |
| StartIndex | `int32` |
| Limit | `int32` |

#### GroupService : SimpleService

*(no public properties detected)*

#### IdWrapperResultModel

| Property | Type |
|----------|------|
| Data | `int32` |

#### IsAliveService : SimpleService

*(no public properties detected)*

#### LoggingService : SimpleService

*(no public properties detected)*

#### MasterDataService : SimpleService

*(no public properties detected)*

#### MessageService : SimpleService

*(no public properties detected)*

#### NotificationService : SimpleService

*(no public properties detected)*

#### OnboardingService : SimpleService

*(no public properties detected)*

#### PersonalReferenceDataService : SimpleService

*(no public properties detected)*

#### PostService : SimpleService

*(no public properties detected)*

#### RemoteNotificationService : RequestCachingService

*(no public properties detected)*

#### SearchService : SimpleService

*(no public properties detected)*

#### UpdateDocumentStatusRequestModel

| Property | Type |
|----------|------|
| DocumentId | `int64` |
| IsLocked | `bool` |

#### WidgetService : SimpleService

*(no public properties detected)*

## Model Relationships

### Core Entity Relationships

```
Profile
  |-- InstitutionProfile (via profile.InstitutionProfile)
  |-- Group[] (via profile.Groups)
  |-- PageConfiguration
  '-- User (identity)

User
  |-- UserRelationship[]
  '-- ProfileContext

MessageThread
  |-- User (creator)
  |-- SensitivityLevel (enum)
  |-- SimpleMessageThreadSubscription[] (recipients)
  '-- MessageThreadLatestMessage
       |-- RichTextWrapperDto (text)
       '-- MailBox (sender)

MessageThreadSubscription
  |-- MessageThreadSubscriptionRelatedChild[]
  |-- MessageThreadSubscriptionRelatedInstitution[]
  '-- MessageThread

MessagesInThreadDto
  |-- MessageDto[] (messages)
  |-- RecipientDto[] (recipients)
  '-- AulaFileResultDto[] (attachments)

MessageDto
  |-- RichTextWrapperDto (text)
  |-- AulaFileResultDto[] (attachments)
  '-- MessageRecipient[] (recipients)

PostApiDto
  |-- RichTextWrapperDto (content)
  |-- ProfileApiDto (owner)
  |-- ProfileApiDto[] (relatedProfiles)
  |-- ShareWithGroupDto[] (sharedWithGroups)
  '-- AulaFileResultDto[] (attachments)

EventBaseClass (calendar events)
  |-- EventGroup[] (inviteeGroups)
  |-- EventProfile[] (participants)
  |-- Resource (primaryResource)
  |-- Resource[] (additionalResources)
  |-- RepeatingEventDto (repeating)
  '-- ResponseType (enum)
  EventDetailsDto : EventBaseClass
  EventSimpleDto : EventBaseClass

Group
  |-- GroupMembership[]
  |-- RecipientRelation[]
  |-- Institution
  '-- MainGroup

Institution
  |-- InstitutionIdentity
  |-- Permission[]
  |-- ModuleDto[]
  |-- WidgetDto[]
  '-- ChildProfile[] (children)

SecureDocumentDto
  |-- SecureDocumentCreatorDto
  |-- SecureDocumentAssociateGroupDto[]
  |-- SecureDocumentAssociateInstitutionProfileDto[]
  |-- AulaFileResultDto[]
  '-- DocumentRevisionDto[]

AlbumDto (gallery)
  |-- AlbumCreatorDto
  |-- AlbumGroupDto[]
  '-- MediaListDto[] (media)

AulaFileResultDto
  |-- AulaFileContent
  |-- AulaMediaFileContent
  |-- AulaDocumentLinkContent
  '-- AulaFileResultProfileDto

NotificationSettings
  |-- WidgetNotificationSettings
  '-- ComeGoNotificationSettings

PresenceRegistration (ComeGo)
  |-- ActivityListChildPresenceResultModel
  |-- PresenceConfigurationResultModel
  |-- ComeGoLocationResultModel
  '-- PresenceStatusEnum
```

### Most Referenced Model Types

| Type | Referenced By (count) |
|------|----------------------|
| Enums.PortalRole | 19 |
| Models.Common.Api.Files.Parameters.DownloadFileFromAulaArguments | 13 |
| Models.MessageThreads.Argument.RecipientApiModel | 10 |
| Enums.ComeGo.PresenceStatusEnum | 10 |
| Enums.ComeGo.ActivityTypeEnum | 10 |
| Enums.InstitutionRole | 9 |
| Models.RichTextWrapperDto | 8 |
| Models.Common.Api.Files.Result.AulaFileResultDto | 8 |
| Models.Common.Api.Files.Result.AulaFileContent | 7 |
| Enums.SortOrderEnum | 6 |
| Models.Web.WebResponseStatus | 5 |
| DTOs.ProfilePictureDto | 5 |
| Enums.Calendar.ResponseType | 5 |
| Models.Institutions.Institution | 4 |
| Enums.Groups.GroupTypeEnum | 4 |
| DTOs.Group.ResultModel.MembershipCountResultModel | 4 |
| Models.Common.Api.Files.Result.AulaFileResultProfileDto | 4 |
| Enums.Calendar.EventType | 4 |
| Enums.File.FileScanningStatus | 3 |
| Models.ProfileModels.Address | 3 |
| Models.Search.RequestModels.SearchRecipientsFilterDelegate | 3 |
| Models.Common.Api.Files.Result.ShareWithGroupDto | 3 |
| Models.MessageThreads.Argument.StartNewThread.MessageContentRequest | 3 |
| Models.Groups.Group | 3 |
| Models.ProfileModels.StubbedUsers.SimpleInstitutionProfile | 3 |
| Models.Common.Api.Files.Parameters.UploadFileContentArguments | 3 |
| Models.Common.Api.Files.Parameters.AttachmentFeatureV | 3 |
| Models.ProfileModels.MainGroup | 3 |
| DTOs.ComeGo.ParentDailyOverviewInstitutionProfileDto | 3 |
| DTOs.ComeGo.PresenceDay.PresenceTemplateRepeatPattern | 3 |

## Serialization Annotations

The app uses **Newtonsoft.Json** (v13.0.0) for JSON serialization. The following fields have
`[JsonProperty]` annotations, meaning their serialized JSON field names may differ from the
C# property names. monodis could not decode the attribute values (missing Newtonsoft.Json
assembly dependency), so the actual JSON field names are unknown from static analysis alone.

A `DefaultUnknownEnumConverter` class exists in `AulaNative.Enums`, suggesting enum values
are serialized as strings with a fallback to a default/unknown value.

The `DynamicContractResolver` class in `AulaNative.DTOs` (extending `DefaultContractResolver`)
indicates dynamic JSON contract resolution is used for some DTOs.

### Fields with [JsonProperty] Annotations

| Class | Fields with [JsonProperty] |
|-------|---------------------------|
| DTOs.Calendar.Vacation.VacationRegistrationDetailsResultDto | ChildrenPendingAnswers |
| DTOs.ComeGo.EmployeeWeekOverview.ComeGoGetVacationRegistrationOverviewRequestDto | FilterGroups, Limit |
| DTOs.ComeGo.EmployeeWeekOverview.ComeGoGetWeekOverviewRequestDto | StartDate |
| DTOs.ComeGo.EmployeeWeekOverview.VacationRegistrationsDto | Subtitle |
| DTOs.ComeGo.OpeningHoursAndClosedDays.InstitutionClosedDaysDTO | ClosedDaysOverview |
| DTOs.ComeGo.ParentDailyOverviewInstitutionProfileDto | MainGroup |
| DTOs.ComeGo.PresenceRegistrationResultModel | Location, SleepIntervals |
| DTOs.ComeGo.UpdatePresenceDayRequestModel | Comment |
| DTOs.Logging.LogErrorAdditionalParameterDto | AdditionalInfoObj |
| DTOs.Logging.LogErrorServerResponseDto | ServerSubCode |
| DTOs.Onboarding.StubbedChild | InstitutionProfile |
| DTOs.SearchRecipientsDtos.Person | ProfileId |
| Enums.ComeGo.OpeningHoursType | GeneralOpeningHours, DefaultOpeningHours, ClosedDay |
| Enums.ComeGo.PresenceModuleSettingsModule | ReportSick, SpareTimeActivity |
| Enums.Notifications.NotificationType | Badge |
| Enums.ResourceType | Location |
| Models.Calendar.CalendarResourceConflict | UnavailableResourceIds |
| Models.Calendar.CalenderNotification.NotificationItemDto | EventEndTime |
| Models.Calendar.CreateEvent.Lesson.UpdateLessonRequest | NoteToClass |
| Models.Calendar.CreateEvent.Resources.CreateEventResource | Id, InstitutionCode, InstitutionName |
| Models.Calendar.CreateEvent.SimpleEvent.CreateSimpleEventRequest | AddedToInstitutionCalendar, MaximumNumberOfParticipants, DoRequestNumberOfParticipants |
| Models.Calendar.Event.CheckEventConflictInput | ExcludeEventId |
| Models.Calendar.Event.DelegateAccesses | OwnerInstProfileId |
| Models.Calendar.Event.EventBaseClass | InviteeGroups, InvitedGroups, PrimaryResource, AdditionalResources, AdditionalResourceText, Repeating, ResponseStatus, DirectlyRelated, MaximumNumberOfParticipants |
| Models.Calendar.ImportantDate.ImportantDateItem | StartDateTime |
| Models.ComeGo.ActivityList.ActivityListChildPresenceResultModel | IsDefaultExitTimes |
| Models.ComeGo.GuardianRegisterVacationIntervals | Date |
| Models.Comments.Parameter.UpdateCommentRequestModel | CommentId |
| Models.Common.Api.Files.Enums.MediaTypeEnum | Video |
| Models.Common.Api.Files.Parameters.UploadFileToAulaArguments | IsLoading |
| Models.Document.Arguments.GetSecureDocumentsArguments | FilterRegardingStudentIds |
| Models.Groups.GetGroupByProfileContext.GroupByContextRequestModel | InstitutionCodes |
| Models.Groups.GetMemberships.GetMembershipsRequestModel | Limit |
| Models.Institutions.Institution | Children |
| Models.RemoteNotifications.RemoteNotification | ProfileId, ElementId |
| Models.Search.SearchResultProfileItemBase | HomePhoneNumber |

**Note**: Most model properties follow standard .NET PascalCase naming. By default,
Newtonsoft.Json serializes using the property name as-is unless `[JsonProperty]` specifies
a different name. The Aula API likely uses camelCase JSON field names (standard for REST APIs),
which may be handled by a global `CamelCasePropertyNamesContractResolver` rather than per-field
`[JsonProperty]` annotations. The 109 annotated fields above are likely exceptions to this
default naming strategy (e.g., snake_case names, abbreviated names, or backward-compatible aliases).

