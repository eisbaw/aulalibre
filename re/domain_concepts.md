# Aula Domain Concepts

**Source**: Reverse-engineered from `com.netcompany.aulanativeprivate` v2.15.4
**Context**: Aula is Denmark's mandatory school communication platform, operated by Netcompany A/S. It connects parents, teachers, students, and administrators across Danish municipalities, schools, and daycares.

---

## 1. Core Domain Entities

### 1.1 Organizational Hierarchy

#### Municipality (Kommune)
The top-level administrative unit. Danish municipalities manage public schools and daycares. Each municipality has a `MunicipalityCode` and `MunicipalityName`. Municipalities define communication policies, consent rules, and can create cross-institutional groups.

#### Administrative Authority (Administrativ myndighed)
An intermediate organizational layer between municipality and institution. Has an `Id`, `Name`, and a set of `InstitutionCodes` it governs. Used to scope permissions and group searches (`GroupSearchScopeEnum: AdministrativeAuthority`).

#### Institution (Institution)
A school or daycare. Central entity in Aula's data model.

| Property | Description |
|----------|-------------|
| InstitutionCode | Unique identifier string |
| Name | Institution name |
| InstitutionType | School, Daycare, Municipality, Central, or Unknown |
| MunicipalityCode | Parent municipality |
| Groups | Groups belonging to this institution |
| Children | Child profiles enrolled |
| Permissions | Role-based permissions granted at this institution |
| MailboxId | Institution-level mailbox for common inbox |
| CommunicationBlock | Whether communication is blocked |
| AdministrativeAuthority | Governing authority |

#### Group (Gruppe)
A collection of users within or across institutions. Groups are the primary unit for content sharing (posts, media, events, messages).

| Property | Description |
|----------|-------------|
| Id | Unique identifier |
| Name | Group name (e.g., class name "3.A") |
| Type | Institutional, Municipal, Cross-institutional, Other, Unknown |
| Access | Closed, Open, or Application-based |
| Status | Active or Inactive |
| InstitutionCode | Owning institution |
| Memberships | List of GroupMembership entries |
| DashboardEnabled | Whether group has its own dashboard |
| ValidGroupModules | Modules available on this group's page |
| ValidGroupWidgets | Widgets enabled for this group |

Groups have a `MainGroup` concept -- a child's primary class/group, identified by `IsMainGroup: true`. This is the default context group for that child.

**Group types** (`GroupTypeEnum`):
- **Institutional**: Within a single institution (e.g., a class)
- **Municipal**: Spans institutions in a municipality
- **Cross-institutional**: Spans institutions across municipalities
- **Other**: Custom groupings

### 1.2 People and Profiles

Aula has a layered identity model: a **User** has one **Profile**, which has one or more **InstitutionProfiles** (one per institution the user is associated with).

#### User
The authenticated identity. Holds `UserRelationship[]` and a `ProfileContext`.

#### Profile
The user's overall Aula identity.

| Property | Description |
|----------|-------------|
| Id | Unique profile ID |
| PortalRole | Employee, Child, Guardian, Otp, Other |
| InstitutionProfile | The currently active institution profile |
| Groups | Groups the user belongs to |
| PageConfiguration | UI module/widget configuration |
| IsSteppedUp | Whether user has elevated auth (MitID) |
| IsGroupHomeAdmin | Whether user administers a group home |

#### InstitutionProfile
A user's role and identity at a specific institution. This is the pivotal entity -- most API operations reference `InstitutionProfileId` rather than bare profile IDs.

| Property | Description |
|----------|-------------|
| InstitutionProfileId | Unique ID for this user-at-institution |
| ProfileId | Parent profile ID |
| UniPersonId | Universal person identifier (from UNI-Login) |
| InstitutionRole | Guardian, Teacher, Leader, Daycare, PreschoolTeacher, Child, EarlyStudent, MiddleLateStudent, Other, Unknown |
| Institution | The institution this profile belongs to |
| Relations | Related profiles (e.g., a child's guardians, or a guardian's children) |
| MainGroup | The primary group (class) |
| MailBoxId | Messaging mailbox ID |
| CommunicationBlock | Whether messaging is blocked |
| UploadBlock | Whether file upload is blocked |
| Birthday | Date of birth |
| Address | Postal address |

#### ChildProfile
A lightweight reference to a child, used in institution and presence contexts.

| Property | Description |
|----------|-------------|
| InstProfileId | Institution profile ID |
| ProfileId | Profile ID |
| InstCode | Institution code |
| Name / ShortName | Display names |
| UserId | UNI-Login user ID |
| HasCustodyOrExtendedAccess | Legal custody flag |

#### RelationProfile
Represents a relationship between two people (e.g., parent-child). Contains the related person's `InstitutionProfileId`, name, `MailBoxId`, `MainGroupName`, institution, and `PortalRole`.

### 1.3 Communication

#### Message Thread (Beskedtrad)
Threaded conversations between users. The messaging system is the largest communication feature.

| Property | Description |
|----------|-------------|
| ThreadId | Unique thread identifier |
| Subject | Thread subject line |
| Creator | The user who started the thread |
| SensitivityLevel | Level1, Level2, or Level3 (Level3 requires MitID step-up) |
| RequiredStepUp | Whether elevated auth is needed to view |
| OtherRecipients | List of thread participants |
| StartedDateTime | When thread was created |

**Thread features**:
- **Folders**: Users can organize threads into folders (Normal, Deleted)
- **Muting**: Suppress notifications for a thread
- **Marking**: Flag threads as important
- **Bundling**: Related threads can be grouped (`SubscriptionType: Bundle, BundleItem, Unbundled`)
- **Sensitivity levels**: Messages can be marked sensitive, requiring step-up authentication
- **Common Inbox**: Shared institutional mailboxes (`CommonInboxType: Institutional, CrossInstitutional`)
- **Auto-reply**: Users can set auto-reply messages
- **BCC recipients**: Supported for messages
- **Regarding Children**: Threads can be tagged as concerning specific children

**Thread types** (`ThreadType`): Thread, EventReminder, VacationRequestReminder

#### Message (Besked)
An individual message within a thread.

| Property | Description |
|----------|-------------|
| Id | Message ID |
| Text | Rich text content (`RichTextWrapperDto`) |
| Sender | MailBox of the sender |
| SendDateTime | When sent |
| Attachments | File attachments (`AulaFileResultDto[]`) |
| Recipients | Message recipients |

**Message types** (`MessageType`): Message, RecipientAdded, RecipientRemoved, AutoReply, SystemForward, SystemReply, Forward, MessageDeleted, MessageEdited, etc.

#### Post (Opslag)
A broadcast-style message shared with groups (similar to a social media post).

| Property | Description |
|----------|-------------|
| Owner | The author (`ProfileApiDto`) |
| Content | Rich text (`RichTextWrapperDto`) |
| SharedWithGroups | Groups the post is shared with |
| RelatedProfiles | Tagged/related people |
| Attachments | File attachments |

**Post filters** (`PostFilterTypeEnum`): All, Unread, IsImportant, FromStaff, FromParents, FromStudents, OwnPost, Bookmarked

#### Comment (Kommentar)
Comments on posts. Can be reported, edited, or deleted. Types: Comment, Media, Post, Unknown.

### 1.4 Calendar and Events

#### Event (Begivenhed)
Calendar events with rich scheduling and response tracking.

| Property | Description |
|----------|-------------|
| InviteeGroups / InvitedGroups | Groups invited to the event |
| Participants | Individual event profiles |
| PrimaryResource / AdditionalResources | Booked resources (rooms, equipment) |
| Repeating | Recurrence configuration |
| ResponseStatus | Per-user response tracking |
| MaximumNumberOfParticipants | Capacity limit |

**Event classes** (`EventClass`): Basic, Series, Timeslot, Lesson, Unknown

**Event types** (`EventType`): Event, Holiday, PresenceHoliday, VacationRegistration, Birthday, Meeting, Excursion, SchoolHomeMeeting (skole-hjem-samtale), ClassMeeting, ParentalMeeting (foraldremade), PerformanceMeeting, Lesson, Other, Unknown

**Response types** (`ResponseType`): Waiting, Declined, Accepted, Tentative

**Timeslot events**: Parent-teacher meetings are organized as timeslots that parents can book. Timeslots can be blocked or already booked (`TimeslotResponseType`).

#### Lesson (Lektion)
A scheduled class/lesson, imported from external scheduling systems. Has statuses: Normal, Cancelled, Absent, Substitute, ToBeDeleted, WillBeUpdated.

**Participant roles** (`ParticipantRole`): PrimaryTeacher, SubstituteTeacher, HelpTeacher, Pedagogue, NotChosen

#### Vacation Registration (Ferieregistrering)
Institutions can create vacation registration requests that guardians must respond to (IsComing, IsNotComing, PendingAnswer). Includes response deadlines and notes to guardians.

#### Calendar Synchronization
Calendar events can be synchronized to external calendars (e.g., Google Calendar, iCal) via feed configurations, with municipality-level controls.

#### Delegated Calendar Access
Users can delegate calendar access to others (`DelegatedAccesses`), allowing colleagues to view/manage their calendar.

### 1.5 Presence / Attendance (Komme/Ga -- "ComeGo")

The ComeGo module tracks children's attendance at daycare and schools. This is one of Aula's largest subsystems with 40+ API methods.

#### Presence Registration
Tracks a child's daily attendance.

| Property | Description |
|----------|-------------|
| Status | NotPresent, Sick, ReportedAbsence, Present, FieldTrip, Sleeping, SpareTimeActivity, PhysicalPlacement, CheckedOut, NotArrived |
| EntryTime / ExitTime | Check-in and check-out times |
| ExitWith | Who the child leaves with |
| Location | Physical location within the institution |
| SleepIntervals | Nap times (for daycare) |
| ActivityType | PICKED_UP_BY, SELF_DECIDER, SEND_HOME, GO_HOME_WITH, DROP_OFF_TIME, SPARE_TIME, CHECK_IN, CHECK_OUT, SLEEPING |
| Comment | Daily notes |
| SpareTimeActivity | After-school activity details |

#### Presence Schedule / Template
Repeating weekly schedules for drop-off/pickup times. Templates have a `RepeatPattern` (Never, Weekly, Every2Weeks) and can be set for specific date ranges.

#### Pickup Responsible (Afhentningsansvarlig)
Authorized people who can pick up a child. Guardians manage a list of pickup responsibles and suggestions.

#### Opening Hours (Abningstider)
Institution opening/closing times. Types: GeneralOpeningHours (weekly defaults), SpecificOpeningHours (date-specific overrides), DefaultOpeningHours, ClosedDay.

#### Closed Days (Lukkedage)
Days when the institution is closed (holidays, teacher planning days).

#### Vacation (Ferie)
Parent-reported absences for children. Includes intervals with start/end dates and comments.

#### Module Settings
ComeGo features are configurable per institution. Each module (DropOffTime, Location, Sleep, FieldTrip, PickupType, PickupTimes, DailyMessage, Vacation, ReportSick, SpareTimeActivity) can be Editable, Deactivated, or Readable per dashboard context (Employee, Checkin, Guardian).

### 1.6 Documents

#### Secure Document (Sikkert dokument / Sikre filer)
Confidential documents about children, requiring Level 3 (MitID) authentication to access.

| Property | Description |
|----------|-------------|
| Creator | Document author |
| AssociatedGroups | Groups with access |
| AssociatedProfiles | Individual profiles with access |
| Revisions | Version history |
| Attachments | File attachments |
| Category | Agenda, PlanOfAction, Observation, EducationalNote, Summary, Note, etc. |
| JournalingStatus | Integration with ESDH (electronic records management) |
| LockedStatus | Whether the document is locked for editing |

**Document types** (`DocumentTypeEnum`): External, Internal, Note, Richdocument, Unknown

**Sharing model**: Documents have explicit sharings (manually shared), implicit sharings (derived from group membership), and permission overrides (Read, Write, NoAccess).

**ESDH Integration**: Documents can be journalized (exported) to municipal ESDH systems. Statuses: NotProcessed, InProgress, Failed, Completed.

#### Common File (Falles fil)
Non-sensitive shared files at the institution level. Can be sorted by Title or UpdatedTime.

### 1.7 Gallery (Galleri)

#### Album
Photo/video collections shared with groups.

| Property | Description |
|----------|-------------|
| Creator | Album creator |
| Groups | Groups with access |
| Media | List of media items |

#### Media
Individual photos or videos within albums. Can be tagged with people, reported, downloaded, or rotated. Media has a `ConversionStatus` (Completed, Processing, Failed) for server-side processing.

**Media types** (`MediaTypeEnum`): Image, Video, Sound, MediaWithDuration, Media

**Consent**: Media visibility is governed by consent settings. Employees with specific permissions can `VIEW_MEDIA_REGARDLESS_OF_CONSENT`.

### 1.8 Notifications (Notifikationer)

#### Notification
In-app notifications for various events.

**Notification areas** (`NotificationArea`): Messages, Calendar, Posts, Schedule, Administration, Gallery, Documents, Album, Presence, Widget, FileScanning

**Notification types** (`NotificationType`): Badge (counter), Alert (push/banner), Irrelevant, Unknown

The notification event type enum (`NotificationEventType`) has 60+ values covering every significant action in the system -- from new messages and event invitations to file scan failures and OS deprecation warnings.

#### Push Notification Settings
Per-user notification preferences, configurable per platform (mobile, email), per module (messages, calendar, media, posts, ComeGo, widgets), and per day of week. Settings are role-aware (Employee, Guardian, Child).

### 1.9 Search

Global search spans multiple entity types: Profile, Group, Child, Employee, Guardian, InternalSecureFile, ExternalSecureFile, CommonFile, Event, Post, CommonInbox, Message, Thread, ThreadMessage, Media.

Search can be scoped by institution, administrative authority, municipality, or cross-institutional.

### 1.10 Widgets

Third-party or municipality-specific widgets integrated into Aula.

| Property | Description |
|----------|-------------|
| WidgetId | External widget identifier |
| Url | Widget URL |
| Placement | OwnPage, RightOfOverview, RightOfCalendar, BelowCalendar, OnOverview, OnCalendar |
| CanAccessOnMobile | Whether the widget works on mobile |
| UsableForGroups | Whether it can be placed on group pages |

Widget access is authenticated via `GetAulaToken` (SSO token for widgets).

### 1.11 Consents (Samtykke)

GDPR-related consent management.

**Consent types** (`Consent`): SHARE_CONTACT_INFORMATION_PARENT, SHARE_CONTACT_INFORMATION_CHILD, OTHERS

**Consent answers** (`ConsentAnswerEnum`): Accepted, Declined, Class, Year, Institution, NotAtAll, Other

Consents are managed per InstitutionProfile and determine what contact information and media is visible to other users. Consent status can be Active, Deactive, or Pending.

### 1.12 Personal Reference Data (Personlige referencedata)

Structured data collection about children, likely for pedagogical or administrative purposes. Includes questions (`GetPersonalReferenceQuestionData`), consent answers, and additional answers. Accessible to employees with `VIEW_PERSONAL_REFERENCE_DATA_FOR_ALL_CHILDREN_AND_GUARDIAN` permission.

### 1.13 Onboarding

New users go through a structured onboarding flow:
1. AppOnboarding (app introduction)
2. PolicyAcceptance (privacy policy)
3. MasterData (profile information)
4. Consents (GDPR consent)
5. AdditionalMasterData (extra profile data)
6. NotificationSettings (notification preferences)

### 1.14 Files and Attachments

Files flow through several processing stages:
- **Upload**: Files are uploaded to AWS S3 via pre-signed URLs (supports multipart upload)
- **Scanning**: `FileScanningStatus` (Available, Blocked, Processing, Bypassed) -- files are scanned for malware/policy violations
- **Status**: `FileStatusEnum` (Available, Pending, Unavailable, Unknown)
- **Types**: Media, File, ExternalFile (from cloud storage)
- **Cloud integration**: Google Drive and OneDrive files can be attached or imported

**Profile pictures** are stored in S3 (identified by Key and Bucket) with an `IsImageScalingPending` flag.

---

## 2. Entity Relationships

### 2.1 Organizational Structure

```
Municipality (MunicipalityCode)
  |-- AdministrativeAuthority
  |     '-- Institution[] (by InstitutionCodes)
  |
  '-- Institution (InstitutionCode)
        |-- Groups[] (Institutional groups)
        |-- ChildProfile[] (enrolled children)
        |-- Permission[] (role-based permissions)
        |-- ModuleDto[] (enabled modules)
        |-- WidgetDto[] (enabled widgets)
        '-- MailboxId (common inbox)
```

### 2.2 Identity and Relationships

```
User (authenticated identity)
  '-- Profile (ProfileId)
        |-- PortalRole (Employee | Child | Guardian | Otp | Other)
        |-- InstitutionProfile[] (one per institution)
        |     |-- InstitutionProfileId (primary key for API operations)
        |     |-- InstitutionRole (Guardian | Teacher | Leader | Daycare | Child | ...)
        |     |-- Institution (the institution)
        |     |-- MainGroup (primary class/group)
        |     |-- Relations[] -> RelationProfile (links to children/guardians)
        |     |-- MailBoxId (for messaging)
        |     '-- Permissions (via institution)
        |
        '-- Groups[] (all groups across institutions)
```

### 2.3 Content Ownership and Sharing

```
Post --> Owner (ProfileApiDto)
     --> SharedWithGroups[] (GroupDto)
     --> Comments[]
     --> Attachments[] (AulaFileResultDto)

MessageThread --> Creator (User)
             --> Recipients[] (InstitutionProfile | CommonInbox | OtpInbox)
             --> Messages[]
                   --> Sender (MailBox)
                   --> Attachments[]
             --> RegardingChildren[] (ChildProfile)

Event --> InviteeGroups[] (EventGroup)
      --> Participants[] (EventProfile)
      --> Resources[] (rooms, equipment)
      --> Repeating (recurrence)

Album --> Creator
      --> Groups[] (shared with)
      --> Media[]
            --> Tags[] (tagged people)

SecureDocument --> Creator
               --> AssociatedGroups[]
               --> AssociatedProfiles[]
               --> Revisions[]
               --> Attachments[]
```

### 2.4 Presence (ComeGo) Relationships

```
Child (InstitutionProfile)
  |-- PresenceRegistration (daily)
  |     |-- Status (Present, Sick, Absent, etc.)
  |     |-- EntryTime / ExitTime
  |     |-- Location (physical room/area)
  |     |-- SleepIntervals[] (nap times)
  |     '-- SpareTimeActivity
  |
  |-- PresenceSchedule (weekly template)
  |     |-- RepeatPattern (Weekly, Every2Weeks)
  |     '-- PresenceDay[] (per day-of-week)
  |
  |-- PickupResponsibles[] (authorized adults)
  |-- VacationRegistrations[] (parent-reported absences)
  '-- VacationAnnouncements[] (institution-created vacation requests)
```

---

## 3. Domain Terminology (Danish to English)

### Organizational Terms

| Danish | English | Context |
|--------|---------|---------|
| Kommune | Municipality | Top-level administrative unit |
| Institution | Institution | School or daycare |
| Skole | School | `InstitutionTypeEnum.School` |
| Daginstitution / Dagpleje | Daycare | `InstitutionTypeEnum.Daycare` |
| Gruppe | Group | Class, team, or organizational group |
| Klasse | Class | A school class (typically a group) |
| Stue | Room/Department | Daycare group room |

### People and Roles

| Danish | English | Context |
|--------|---------|---------|
| Foralder / Vaerge | Parent / Guardian | `PortalRole.Guardian`, `InstitutionRole.Guardian` |
| Laerer | Teacher | `InstitutionRole.Teacher` |
| Padagog | Pedagogue / Educator | `ParticipantRole.Pedagogue` |
| Leder | Leader / Principal | `InstitutionRole.Leader` |
| Elev / Barn | Student / Child | `PortalRole.Child`, `InstitutionRole.Child` |
| Medarbejder | Employee / Staff | `PortalRole.Employee` |
| Vikar | Substitute teacher | `ParticipantRole.SubstituteTeacher`, `LessonStatus.Substitute` |
| Kontaktperson | Contact person | Primary teacher for a class |

### Communication

| Danish | English | Context |
|--------|---------|---------|
| Besked | Message | Individual message |
| Beskedtrad | Message thread | Threaded conversation |
| Opslag | Post | Broadcast-style message |
| Kommentar | Comment | Comment on a post |
| Indbakke | Inbox | Message inbox |
| Falles indbakke | Common inbox | Shared institutional inbox |
| Folsomme beskeder | Sensitive messages | Require MitID authentication |
| Automatisk svar | Auto-reply | Out-of-office reply |

### Calendar

| Danish | English | Context |
|--------|---------|---------|
| Begivenhed | Event | Calendar event |
| Lektion | Lesson | Scheduled class period |
| Skole-hjem-samtale | School-home meeting | Parent-teacher conference |
| Foraldremade | Parental meeting | Group meeting for parents |
| Ferie | Vacation / Holiday | School holidays |
| Ferieregistrering | Vacation registration | Attendance tracking during holidays |
| Udflugt | Excursion / Field trip | Off-site school activity |
| Tidspunkt / Tidsinterval | Timeslot | Bookable meeting slot |

### Presence / Attendance

| Danish | English | Context |
|--------|---------|---------|
| Komme/Ga (ComeGo) | Come and Go | Attendance/presence module |
| Fravarende | Absent | `PresenceStatusEnum.ReportedAbsence` |
| Tilstede | Present | `PresenceStatusEnum.Present` |
| Syg | Sick | `PresenceStatusEnum.Sick` |
| Sover | Sleeping | `PresenceStatusEnum.Sleeping` (daycare naps) |
| Afhentning | Pickup | Who picks up the child |
| Aflevering | Drop-off | When child is dropped off |
| Abningstider | Opening hours | Institution operating hours |
| Lukkedage | Closed days | Days institution is closed |
| Fritidsaktivitet | Spare time activity | After-school/leisure activity |

### Documents

| Danish | English | Context |
|--------|---------|---------|
| Sikre filer / Sikkert dokument | Secure files / Secure document | Confidential child documents |
| Falles filer | Common files | Shared institution files |
| Handleplan | Plan of action | `DocumentCategoryEnum.PlanOfAction` |
| Observation | Observation | Pedagogical observation note |
| Padagogisk notat | Educational note | `DocumentCategoryEnum.EducationalNote` |
| Referat | Summary / Minutes | `DocumentCategoryEnum.Summary` |
| Dagsorden | Agenda | `DocumentCategoryEnum.Agenda` |

### Other

| Danish | English | Context |
|--------|---------|---------|
| Galleri | Gallery | Photo/video sharing |
| Album | Album | Media collection |
| Medier | Media | Photos and videos |
| Notifikation | Notification | Push/in-app notification |
| Samtykke | Consent | GDPR consent |
| Kontaktoplysninger | Contact information | Phone, email, address |
| Privatlivspolitik | Privacy policy | GDPR privacy policy |
| Opslagstavle | Notice board | Bulletin board feature |

---

## 4. User Roles and Permission Model

### 4.1 Role Hierarchy

Aula has two role dimensions that operate together:

**Portal Role** (`PortalRole`) -- the user's system-wide role:
| Role | Description |
|------|-------------|
| Employee | School/daycare staff (teachers, pedagogues, leaders, administrators) |
| Guardian | Parent or legal guardian of a child |
| Child | Student (limited access, typically older students) |
| Otp | "OTP" -- likely "Other Trusted Person" or similar; an extended role for non-guardian trusted contacts |
| Other | Fallback role |

**Institution Role** (`InstitutionRole`) -- the user's specific role at an institution:
| Role | Description |
|------|-------------|
| Guardian | Parent/guardian of enrolled child |
| Teacher | School teacher (laerer) |
| PreschoolTeacher | Preschool/kindergarten teacher |
| Daycare | Daycare worker (padagog) |
| Leader | Institution leader/principal |
| Child | Enrolled child |
| EarlyStudent | Younger student (0.-3. klasse) |
| MiddleLateStudent | Older student (4.-10. klasse) |
| Other | Other staff |
| Unknown | Unclassified |

### 4.2 Authentication Levels

Authentication level determines what a user can access:

| Level | Method | Scope | Client ID |
|-------|--------|-------|-----------|
| Level 2 | UniLogin | Standard access -- messages, calendar, posts, gallery, presence | `_742adb5e...` |
| Level 3 | MitID (formerly NemID) | Elevated access -- secure documents, sensitive messages, consents | `_99949a54...` |

The `IsSteppedUp` flag on Profile and `RequiredStepUp` on content items indicate when Level 3 auth is needed. The `Permission.StepUp` boolean marks which permissions require elevation.

### 4.3 Permission Model

Permissions are defined per institution, per role. The `PermissionEnum` contains 100+ granular permissions organized by domain:

#### Administrative Permissions
- `ADMIN_MODULE` -- Access admin panel
- `HANDLE_USER_ROLES` -- Manage user roles
- `HANDLE_USER_DATA` -- Manage user data
- `IMPERSONATE_USER` -- Act as another user
- `HANDLE_REPORTS_OF_POSTS` -- Process reported content
- `HANDLE_COMMUNICATION_CHANNELS_MUNICIPALITY` / `_CENTRAL` -- Configure communication rules
- `HANDLE_CONSENTS` -- Manage consent settings
- `HANDLE_CONSENT_AGE` -- Configure consent age thresholds

#### Communication Permissions
- `READ_MESSAGE` / `WRITE_MESSAGE` -- Read/send messages
- `READ_POST` / `WRITE_POST` -- Read/write posts
- `WRITE_COMMENTS` -- Post comments
- `MESSAGE_ATTACH_BCC_RECIPIENTS` -- Use BCC in messages
- `INBOX_SET_PERSONAL_AUTOREPLY` -- Set auto-reply
- `INBOX_FOLDERS` -- Use message folders
- `MESSAGE_SEE_SUBSCRIBERS_LASTREAD` -- See who has read messages
- `HANDLE_ALLOWED_RECIPIENTS` -- Configure allowed message recipients

#### Calendar Permissions
- `SEE_CALENDAR` / `READ_EVENTS` / `HANDLE_EVENTS` -- View/manage events
- `INVITE_TO_EVENT` / `INVITE_GROUP_TO_EVENT` -- Send event invitations
- `HANDLE_PARENTAL_MEETING_SCHOOL` / `_DAYCARE` -- Manage parent meetings
- `HANDLE_PERFORMANCE_MEETING` -- Manage performance review meetings
- `BOOK_RESOURCES` -- Book rooms and equipment
- `CREATE_EVENTS_IN_INSTITUTION_CALENDAR` -- Add to institution-wide calendar
- `HANDLE_OTHERS_EVENTS` / `HANDLE_EVENT_CO_ORGANIZER` -- Manage other users' events

#### Document Permissions
- `ACCESS_SECURE_FILESHARING` -- Access secure file module
- `HANDLE_SECURE_FILES` / `HANDLE_SECURE_FILES_LIMITED` -- Full/limited secure doc access
- `SHARE_SECURE_FILES` -- Share secure documents
- `READ_SECURE_FILES` -- Read-only secure doc access
- `EXPORT_SECURE_FILES` -- Export secure documents
- `SECURE_DOCUMENTS_ACCESS_ALL` -- Override: access all secure docs
- `JOURNALING_TO_ESDH` -- Export to ESDH records system
- `ACCESS_IMPORTANT_FILES` / `HANDLE_IMPORTANT_FILES` -- Manage important files

#### Gallery/Media Permissions
- `SEE_MEDIA` / `SHARE_MEDIA` / `HANDLE_MEDIA` -- View/share/manage media
- `VIEW_MEDIA_REGARDLESS_OF_CONSENT` -- Override consent for media viewing
- `TAG_OTHER_USERS_ON_OTHER_MEDIA` -- Tag others in media
- `EDIT_SHARED_ALBUMS` / `EDIT_SHARED_MEDIA` -- Modify shared content

#### Profile/Contact Permissions
- `SEARCH_ACCESS_PROFILES` / `SEARCH_ACCESS_GROUPS` -- Search for people/groups
- `SEE_GUARDIAN_CHILD_CONTACT_INFORMATION` -- View guardian/child contacts
- `SEE_EMPLOYEE_CONTACT_INFORMATION` -- View employee contacts
- `VIEW_CONTACT_INFORMATION_REGARDLESS_OF_CONSENT` -- Override consent for contacts
- `VIEW_CONTACT_INFORMATION_ALL` -- View all contact info
- `SEE_GUARDIAN_CHILD_LAST_LOGIN` / `SEE_EMPLOYEE_LAST_LOGIN` -- See login timestamps
- `VIEW_NAME_PROTECTION` / `VIEW_CUSTODY` -- View protected personal data
- `HANDLE_CONTACTS` / `HANDLE_ACCESS_CONTACT_INFO` -- Manage contact settings

#### Presence/ComeGo Permissions
- `VIEW_PRESENCE_STATISTICS` / `EXPORT_PRESENCE_STATISTICS` -- Presence data access
- `HANDLE_VACATION_REQUESTS` -- Manage vacation registrations
- `HANDLE_OPTIONS_PRESENCE_DASHBOARD` -- Configure presence dashboard
- `EDIT_PRESENCE_TEMPLATES` -- Edit presence schedule templates

#### Group Permissions
- `HANDLE_GROUP` / `HANDLE_INTERINSTITUTIONAL_GROUPS` -- Manage groups
- `HANDLE_GROUP_APPLICATION` -- Process group join requests
- `USE_GROUPS_AS_DISTRIBUTION_LISTS` -- Use groups for bulk messaging
- `HANDLE_GROUP_TEMPLATES` -- Manage group templates

#### Cloud Storage Permissions
- `ATTACH_GOOGLE_DRIVE_FILE` / `ATTACH_ONEDRIVE_FILE` -- Attach cloud files
- `IMPORT_MEDIA_FROM_GOOGLE_DRIVE` / `IMPORT_MEDIA_FROM_ONEDRIVE` -- Import media from cloud

### 4.4 Permission Scoping

Each `Permission` has:
- `PermissionId` -- The permission enum value
- `StepUp` -- Whether Level 3 (MitID) authentication is required to exercise this permission
- `GroupScopes` -- List of group IDs this permission applies to (empty = all groups)
- `InstitutionScope` -- Whether the permission applies institution-wide

### 4.5 Communication Blocking

Communication can be blocked at multiple levels:

- **BlockedLevel**: Central, Municipal, Institution, Unknown
- **Per role type**: Blocks can target Child, Employee, or Guardian communications separately
- **IsBlockedAllProfileTypes**: Universal block

The API provides `CheckRecipientsForBlockedChannels` to verify messaging permissions before sending.

### 4.6 Typical Role Capabilities

| Capability | Guardian | Employee | Child |
|-----------|----------|----------|-------|
| Send/receive messages | Yes | Yes | Limited |
| View calendar events | Yes (own children) | Yes (institution) | Yes (own) |
| Create events | No | Yes | No |
| View/create posts | Yes | Yes | Limited |
| Manage presence (ComeGo) | Yes (own children) | Yes (all children) | No |
| Access secure documents | Yes (MitID required) | Yes (MitID required) | No |
| Upload media to gallery | Limited | Yes | Limited |
| Manage groups | No | Yes (with permission) | No |
| Admin functions | No | Yes (Leaders only) | No |
| Set notification preferences | Yes | Yes | Yes |

---

## 5. Key Architectural Observations

### 5.1 InstitutionProfileId is the Pivot
Nearly all API operations reference `InstitutionProfileId` rather than `ProfileId`. This is because a single person can have different roles and permissions at different institutions. The institution profile is the intersection of person + institution + role.

### 5.2 Content is Group-Scoped
Posts, gallery albums, events, and even messaging recipients are organized around Groups. Groups are the primary sharing/access-control unit, not individual profiles.

### 5.3 Two-Tier Authentication Maps to Data Sensitivity
The Level 2 / Level 3 authentication split cleanly maps to data sensitivity: everyday communication (messages, posts, calendar) requires only UniLogin, while legally sensitive data about children (secure documents, detailed personal data) requires the stronger MitID authentication.

### 5.4 Extensive Consent Model
Denmark's strict data protection requirements are reflected in Aula's consent system. Contact information sharing, media tagging, and personal data visibility are all governed by explicit consent, with override permissions for staff who need access regardless.

### 5.5 ComeGo is a Subsystem, Not Just a Feature
The presence/attendance module has its own configuration system, module settings, templates, vacation management, sleep tracking, activity tracking, and distribution analytics. It operates semi-independently from the rest of Aula.
