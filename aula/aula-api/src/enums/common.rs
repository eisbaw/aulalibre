//! Common/general-purpose enums used across multiple domains.

use serde::{Deserialize, Serialize};

/// Platform identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    Android,
    #[serde(rename = "iOS")]
    Ios,
    Unknown,
}

/// Day of the week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Sort order direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortOrderEnum {
    Unknown,
    Ascending,
    Descending,
}

/// Answer type for additional master data responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AdditionalMasterDataResponseAnswerTypeEnum {
    YesNo,
    PhoneNumber,
    Text,
}

/// Application type (Staff vs Private vs Unknown).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppTypeEnum {
    Staff,
    Private,
    Unknown,
}

/// Association mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssociationModeEnum {
    None,
    Select,
    Confirm,
}

/// Source for file picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AulaFilePickerEnum {
    Files,
    MediaLibrary,
    GoogleDrive,
    OneDrive,
    PhotoCamera,
    VideoCamera,
    AulaGallery,
    Document,
    DownloadMediaGoogleDrive,
    DownloadMediaOneDrive,
    FilesForMedia,
    AttachFileGoogleDrive,
    AttachFileOneDrive,
    All,
}

/// Cache size classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CacheType {
    Small,
    Large,
}

/// Filter and sort type for lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterAndSortType {
    FilterAll,
    FilterUnread,
    FilterMarked,
    FilterDraft,
    SortDate,
    SortSubject,
    SortCreatedDate,
    SortMediaCreatedDate,
    SortMediaCreatedAt,
    FilterMyAlbums,
    FilterMyMedia,
    SortAlbumName,
}

/// Loading/pagination type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LoadingType {
    LoadMore,
    Action,
    Refresh,
}

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    All,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

/// Report target type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportEnum {
    Post,
    Media,
    Comments,
    Unknown,
}

/// Resource type for calendar resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceType {
    Location,
    Other,
    ExtraLocation,
    Electronics,
    Stationery,
}

/// Time period selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimePeriod {
    None,
    TwoWeeks,
    OneMonth,
    ThreeMonths,
    SixMonths,
    OneYear,
}

/// Bottom bar long-press option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BottomBarLongPressOption {
    EditShortcuts,
}

/// Front page configuration setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FrontPageSettingConfigurationEnum {
    ActivityFeed,
    Messages,
    CalendarOverview,
    ImportantDates,
    Document,
    ComeGo,
    Gallery,
    ContactList,
    PersonalReferenceData,
}

/// Biometric authentication status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BioAuthStatus {
    CanTryAgain,
    CanNotTryAgain,
    Canceled,
    Accepted,
}

/// Blocked communication level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockedLevel {
    Central,
    Municipal,
    Institution,
    Unknown,
}

/// Consent type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Consent {
    ShareContactInformationParent,
    ShareContactInformationChild,
    Others,
}

/// Consent answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConsentAnswerEnum {
    Accepted,
    Declined,
    Class,
    Year,
    Institution,
    NotAtAll,
    Other,
}

/// Consent status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConsentStatus {
    Active,
    Deactive,
    Pending,
}

/// Comment dropdown action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommentDropDownEnumeration {
    Delete,
    Edit,
    Report,
}

/// Comment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommentType {
    Comment,
    Media,
    Post,
    Unknown,
}

/// Post detail more-menu action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostDetailMoreMenuEnum {
    ReportPost,
}

/// Post filter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostFilterTypeEnum {
    All,
    Unread,
    IsImportant,
    FromStaff,
    FromParents,
    FromStudents,
    OwnPost,
    Bookmarked,
}

/// Widget placement location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WidgetPlacementEnum {
    OwnPage,
    RightOfOverview,
    RightOfCalendar,
    BelowCalendar,
    OnOverview,
    OnCalendar,
}

/// Institution permission.
///
/// This is a large enum covering all granular permissions in the Aula platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PermissionEnum {
    AdminModule,
    SearchAccessProfiles,
    SearchAccessGroups,
    HandleGroup,
    HandleInterinstitutionalGroups,
    HandleUserRoles,
    HandleReportsOfPosts,
    DeletePostsMediaComments,
    HandleCallTimes,
    HandleResourcesInstitution,
    HandleAdditionalMasterData,
    HandlePhysicalLocation,
    HandleImportantFiles,
    HandleSharedInbox,
    HandleUserData,
    HandleAllowedRecipients,
    HandleCommunicationChannelsMunicipality,
    HandleAdministrativeAuthority,
    HandleGroupingsOfInstitutions,
    HandleRightsToPhysicalLocation,
    HandleResourceCategory,
    HandleAdditionalMasterDataBruttoList,
    HandleConsents,
    HandleLessonImportTime,
    HandleMaxFilesize,
    HandleFileformats,
    AccessSecureFilesharing,
    HandleSecureFiles,
    HandleSecureFilesLimited,
    AccessImportantFiles,
    AccessSharedInbox,
    ReadMessage,
    WriteMessage,
    ReadPost,
    WritePost,
    ShareMedia,
    SeeMedia,
    HandleGroupApplication,
    WriteComments,
    SeeCalendar,
    ReadEvents,
    HandleEvents,
    InviteGroupToEvent,
    HandleParentalMeetingSchool,
    HandlePerformanceMeeting,
    BookResources,
    InviteToEvent,
    AnswerEventWithExtendedAnswer,
    ShareSecureFiles,
    HandleCommunicationChannelsCentral,
    HandleTransitionYear,
    HandleResourcesMunicipality,
    SendSms,
    WriteInfoProfile,
    MessageAttachBccRecipients,
    HandleParentalMeetingDaycare,
    InboxSetPersonalAutoreply,
    InboxFolders,
    MessageSeeSubscribersLastread,
    HandleConsentAge,
    HandleDashboard,
    ReportPostsMediaComments,
    SeeGuardianChildContactInformation,
    SeeEmployeeContactInformation,
    SeeGuardianChildLastLogin,
    SeeEmployeeLastLogin,
    HandleContacts,
    ViewUsersAdditionalData,
    HandleSignature,
    ViewMediaRegardlessOfConsent,
    ViewContactInformationRegardlessOfConsent,
    HandleMaxImageResolution,
    HandleMaxVideoLength,
    ViewUsersConsents,
    TagOtherUsersOnOtherMedia,
    AttachGoogleDriveFile,
    ImportMediaFromGoogleDrive,
    AttachOnedriveFile,
    ImportMediaFromOnedrive,
    SeeContactParentsContactInfo,
    HandleMedia,
    ImpersonateUser,
    ViewEmployeesAdditionalData,
    HandleServiceMessages,
    PairingInstitutionAndDevice,
    ViewPresenceStatistics,
    HandleVacationRequests,
    HandleOptionsPresenceDashboard,
    UseGroupsAsDistributionLists,
    ViewNameProtection,
    ViewCustody,
    AccessSkoleintraArchive,
    SkoleintraAdmin,
    SecureDocumentsAccessAll,
    CreateEventsInInstitutionCalendar,
    CreateEventsOnlyInInstitutionCalendar,
    ViewPersonalReferenceDataForAllChildrenAndGuardian,
    ViewContactInformationAll,
    HandleGroupTemplates,
    HandleNoticeBoards,
    HandleAccessContactInfo,
    HandleDataPolicy,
    HandleServiceRequest,
    HandleCalendarFeedMunicipality,
    ExportPresenceStatistics,
    ExportSecureFiles,
    ReadSecureFiles,
    EditPresenceTemplates,
    HandleEventCoOrganizer,
    HandleOthersEvents,
    EditSharedAlbums,
    EditSharedMedia,
    JournalingToEsdh,
}

/// Search profile document type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchProfileDocTypeEnum {
    Profile,
    Group,
    CommonInbox,
    All,
}

/// Search profile portal role filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchProfilePortalRoleEnum {
    Employee,
    Child,
}

/// Search result item type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchResultItemType {
    None,
    Group,
    Profile,
    Child,
    Employee,
    Guardian,
    InternalSecureFile,
    ExternalSecureFile,
    CommonFile,
    Event,
    Post,
    CommonInbox,
    Message,
    Thread,
    ThreadMessage,
    Media,
}

/// Group search scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GroupSearchScopeEnum {
    Institutional,
    AdministrativeAuthority,
    CrossInstitutional,
    Municipal,
}

/// Search recipient document type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchRecipientDocTypeEnum {
    Profile,
    Group,
    CommonInbox,
    All,
}

/// Search recipient mailbox owner type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchRecipientMailBoxOwnerType {
    InstitutionProfile,
    CommonInbox,
    OtpInbox,
}

/// Search recipient module context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchRecipientModuleEnum {
    Event,
    Messages,
    Overview,
    Gallery,
    SecureDocument,
    PersonalReferenceData,
    Contacts,
}

/// Search recipient portal role filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchRecipientPortalRoleEnum {
    Employee,
    Child,
    Guardian,
    Otp,
    All,
}

/// Search resource type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SearchResourceTypeEnum {
    Location,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! roundtrip_test {
        ($name:ident, $ty:ty, $variant:expr) => {
            #[test]
            fn $name() {
                let json = serde_json::to_string(&$variant).unwrap();
                let back: $ty = serde_json::from_str(&json).unwrap();
                assert_eq!(back, $variant);
            }
        };
    }

    roundtrip_test!(platform, Platform, Platform::Ios);
    roundtrip_test!(weekday, WeekDay, WeekDay::Friday);
    roundtrip_test!(sort_order, SortOrderEnum, SortOrderEnum::Ascending);
    roundtrip_test!(
        master_data_answer,
        AdditionalMasterDataResponseAnswerTypeEnum,
        AdditionalMasterDataResponseAnswerTypeEnum::PhoneNumber
    );
    roundtrip_test!(app_type, AppTypeEnum, AppTypeEnum::Staff);
    roundtrip_test!(
        association_mode,
        AssociationModeEnum,
        AssociationModeEnum::Confirm
    );
    roundtrip_test!(
        file_picker,
        AulaFilePickerEnum,
        AulaFilePickerEnum::AulaGallery
    );
    roundtrip_test!(cache_type, CacheType, CacheType::Small);
    roundtrip_test!(
        filter_sort,
        FilterAndSortType,
        FilterAndSortType::SortAlbumName
    );
    roundtrip_test!(loading_type, LoadingType, LoadingType::Refresh);
    roundtrip_test!(log_level, LogLevel, LogLevel::Warning);
    roundtrip_test!(report_enum, ReportEnum, ReportEnum::Media);
    roundtrip_test!(resource_type, ResourceType, ResourceType::Electronics);
    roundtrip_test!(time_period, TimePeriod, TimePeriod::ThreeMonths);
    roundtrip_test!(
        bottom_bar,
        BottomBarLongPressOption,
        BottomBarLongPressOption::EditShortcuts
    );
    roundtrip_test!(
        front_page,
        FrontPageSettingConfigurationEnum,
        FrontPageSettingConfigurationEnum::ComeGo
    );
    roundtrip_test!(bio_auth, BioAuthStatus, BioAuthStatus::Accepted);
    roundtrip_test!(blocked_level, BlockedLevel, BlockedLevel::Municipal);
    roundtrip_test!(consent, Consent, Consent::ShareContactInformationParent);
    roundtrip_test!(
        consent_answer,
        ConsentAnswerEnum,
        ConsentAnswerEnum::Institution
    );
    roundtrip_test!(consent_status, ConsentStatus, ConsentStatus::Deactive);
    roundtrip_test!(
        comment_dropdown,
        CommentDropDownEnumeration,
        CommentDropDownEnumeration::Report
    );
    roundtrip_test!(comment_type, CommentType, CommentType::Media);
    roundtrip_test!(
        post_detail_menu,
        PostDetailMoreMenuEnum,
        PostDetailMoreMenuEnum::ReportPost
    );
    roundtrip_test!(
        post_filter,
        PostFilterTypeEnum,
        PostFilterTypeEnum::Bookmarked
    );
    roundtrip_test!(
        widget_placement,
        WidgetPlacementEnum,
        WidgetPlacementEnum::BelowCalendar
    );
    roundtrip_test!(permission, PermissionEnum, PermissionEnum::HandleEvents);
    roundtrip_test!(
        search_doc_type,
        SearchProfileDocTypeEnum,
        SearchProfileDocTypeEnum::All
    );
    roundtrip_test!(
        search_portal_role,
        SearchProfilePortalRoleEnum,
        SearchProfilePortalRoleEnum::Employee
    );
    roundtrip_test!(
        search_result_item,
        SearchResultItemType,
        SearchResultItemType::ThreadMessage
    );
    roundtrip_test!(
        group_search_scope,
        GroupSearchScopeEnum,
        GroupSearchScopeEnum::Municipal
    );
    roundtrip_test!(
        search_recipient_doc,
        SearchRecipientDocTypeEnum,
        SearchRecipientDocTypeEnum::CommonInbox
    );
    roundtrip_test!(
        search_recipient_mailbox,
        SearchRecipientMailBoxOwnerType,
        SearchRecipientMailBoxOwnerType::OtpInbox
    );
    roundtrip_test!(
        search_recipient_module,
        SearchRecipientModuleEnum,
        SearchRecipientModuleEnum::Gallery
    );
    roundtrip_test!(
        search_recipient_portal,
        SearchRecipientPortalRoleEnum,
        SearchRecipientPortalRoleEnum::Otp
    );
    roundtrip_test!(
        search_resource_type,
        SearchResourceTypeEnum,
        SearchResourceTypeEnum::Location
    );

    #[test]
    fn platform_ios_rename() {
        let json = serde_json::to_string(&Platform::Ios).unwrap();
        assert_eq!(json, r#""iOS""#);
        let back: Platform = serde_json::from_str(r#""iOS""#).unwrap();
        assert_eq!(back, Platform::Ios);
    }

    #[test]
    fn cache_type_screaming() {
        let json = serde_json::to_string(&CacheType::Small).unwrap();
        assert_eq!(json, r#""SMALL""#);
    }

    #[test]
    fn consent_screaming_snake() {
        let json = serde_json::to_string(&Consent::ShareContactInformationParent).unwrap();
        assert_eq!(json, r#""SHARE_CONTACT_INFORMATION_PARENT""#);
    }

    #[test]
    fn permission_screaming_snake() {
        let json = serde_json::to_string(&PermissionEnum::AdminModule).unwrap();
        assert_eq!(json, r#""ADMIN_MODULE""#);
        let back: PermissionEnum = serde_json::from_str(r#""HANDLE_EVENTS""#).unwrap();
        assert_eq!(back, PermissionEnum::HandleEvents);
    }
}
