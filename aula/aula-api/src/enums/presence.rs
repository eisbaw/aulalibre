//! Presence/attendance (Come & Go) enums.

use serde::{Deserialize, Serialize};

/// Repeat pattern for presence templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceTemplateRepeatPattern {
    Never,
    Weekly,
    Every2Weeks,
}

/// Type of presence activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityTypeEnum {
    PickedUpBy,
    SelfDecider,
    SendHome,
    GoHomeWith,
    DropOffTime,
    SpareTime,
    CheckIn,
    CheckOut,
    Sleeping,
    All,
}

/// Come & Go notification type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComeGoNotificationEnum {
    AlertResponseNotification,
    AlertInviteNotification,
    VacationResponseNotification,
}

/// Staff tab in ComeGo module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComeGoStaffTabEnum {
    ActivityList,
    WeekOverview,
    VacationRegistrationOverview,
    OpeningHoursAndClosedDays,
}

/// User-facing ComeGo tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComeGoTabEnum {
    AbsenceTab,
    TimeTab,
    DailyOverview,
    PickupResponsible,
    OpeningHoursAndClosedDaysInstitutionListPage,
    OpeningHoursAndClosedDays,
    PlanningPage,
}

/// Departure type for child pickup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartureTypeEnum {
    GoGomeWith,
    RetrieveResponsible,
}

/// Type of opening hours definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpeningHoursType {
    SpecificOpeningHours,
    GeneralOpeningHours,
    DefaultOpeningHours,
    ClosedDay,
}

/// Day of week in presence context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceDayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// Dashboard context for presence module settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceModuleSettingsDashboardContext {
    EmployeeDashboardSettings,
    CheckinDashboardSettings,
    GuardianDashboardSettings,
}

/// Presence module setting type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceModuleSettingsModule {
    DropOffTime,
    Location,
    Sleep,
    FieldTrip,
    PickupType,
    PickupTimes,
    DailyMessage,
    Vacation,
    ReportSick,
    SpareTimeActivity,
}

/// Permission level for a presence module setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceModuleSettingsPermission {
    Editable,
    Deactivated,
    Readable,
}

/// Presence status of a child.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceStatusEnum {
    NotPresent,
    Sick,
    ReportedAbsence,
    Present,
    FieldTrip,
    Sleeping,
    SpareTimeActivity,
    PhysicalPlacement,
    CheckedOut,
    NotArrived,
    All,
}

/// Editing option for presence templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PresenceTemplateEditingOption {
    EditSingleDay,
    EditWholeTemplate,
    Delete,
}

/// Action on a spare time activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpareTimeActivityAction {
    Edit,
    Delete,
}

/// Token usage status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenStatusEnum {
    Used,
    NotUsed,
    Expired,
}

/// Tense of employee week overview (uses lowercase in API).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComeGoEmployeeWeekOverviewTenseEnum {
    Past,
    Present,
    NotSpecified,
    MissingCheckout,
}

/// Filter option for employee week overview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComeGoEmployeeWeekOverviewFilterOptionEnum {
    Present,
    Vacation,
    Sick,
    NotArrived,
}

/// Presence type in employee week overview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComeGoEmployeeWeekOverviewPresenceTypeEnum {
    Present,
    Vacation,
    Sick,
    NotArrived,
    None,
}

/// ComeGo type for remote notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComeGoType {
    PickupActivity,
    VacationRegistrationRequest,
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

    roundtrip_test!(
        presence_template_repeat,
        PresenceTemplateRepeatPattern,
        PresenceTemplateRepeatPattern::Weekly
    );
    roundtrip_test!(activity_type, ActivityTypeEnum, ActivityTypeEnum::CheckIn);
    roundtrip_test!(
        comego_notification,
        ComeGoNotificationEnum,
        ComeGoNotificationEnum::AlertResponseNotification
    );
    roundtrip_test!(
        comego_staff_tab,
        ComeGoStaffTabEnum,
        ComeGoStaffTabEnum::WeekOverview
    );
    roundtrip_test!(comego_tab, ComeGoTabEnum, ComeGoTabEnum::DailyOverview);
    roundtrip_test!(
        departure_type,
        DepartureTypeEnum,
        DepartureTypeEnum::GoGomeWith
    );
    roundtrip_test!(opening_hours, OpeningHoursType, OpeningHoursType::ClosedDay);
    roundtrip_test!(presence_day, PresenceDayOfWeek, PresenceDayOfWeek::Friday);
    roundtrip_test!(
        presence_dashboard,
        PresenceModuleSettingsDashboardContext,
        PresenceModuleSettingsDashboardContext::GuardianDashboardSettings
    );
    roundtrip_test!(
        presence_module,
        PresenceModuleSettingsModule,
        PresenceModuleSettingsModule::Sleep
    );
    roundtrip_test!(
        presence_permission,
        PresenceModuleSettingsPermission,
        PresenceModuleSettingsPermission::Editable
    );
    roundtrip_test!(
        presence_status,
        PresenceStatusEnum,
        PresenceStatusEnum::Present
    );
    roundtrip_test!(
        presence_template_edit,
        PresenceTemplateEditingOption,
        PresenceTemplateEditingOption::Delete
    );
    roundtrip_test!(
        spare_time_action,
        SpareTimeActivityAction,
        SpareTimeActivityAction::Edit
    );
    roundtrip_test!(token_status, TokenStatusEnum, TokenStatusEnum::Expired);
    roundtrip_test!(
        employee_tense,
        ComeGoEmployeeWeekOverviewTenseEnum,
        ComeGoEmployeeWeekOverviewTenseEnum::Past
    );
    roundtrip_test!(
        employee_filter,
        ComeGoEmployeeWeekOverviewFilterOptionEnum,
        ComeGoEmployeeWeekOverviewFilterOptionEnum::Sick
    );
    roundtrip_test!(
        employee_presence_type,
        ComeGoEmployeeWeekOverviewPresenceTypeEnum,
        ComeGoEmployeeWeekOverviewPresenceTypeEnum::None
    );
    roundtrip_test!(comego_type, ComeGoType, ComeGoType::PickupActivity);

    #[test]
    fn activity_type_screaming_snake() {
        let json = serde_json::to_string(&ActivityTypeEnum::PickedUpBy).unwrap();
        assert_eq!(json, r#""PICKED_UP_BY""#);
        let back: ActivityTypeEnum = serde_json::from_str(r#""CHECK_IN""#).unwrap();
        assert_eq!(back, ActivityTypeEnum::CheckIn);
    }

    #[test]
    fn comego_notification_screaming_snake() {
        let json =
            serde_json::to_string(&ComeGoNotificationEnum::AlertResponseNotification).unwrap();
        assert_eq!(json, r#""ALERT_RESPONSE_NOTIFICATION""#);
    }

    #[test]
    fn employee_tense_camel_case() {
        let json =
            serde_json::to_string(&ComeGoEmployeeWeekOverviewTenseEnum::NotSpecified).unwrap();
        assert_eq!(json, r#""notSpecified""#);
    }
}
