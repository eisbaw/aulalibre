//! Presence / Come & Go domain models.
//!
//! Covers child check-in/check-out, schedules, pickup authorization,
//! opening hours, sleep tracking, vacation, and configuration.
//!
//! See `data_models.md` sections Models.ComeGo.* and DTOs.ComeGo.*.

use serde::{Deserialize, Serialize};

use crate::enums::presence::{
    ActivityTypeEnum, ComeGoEmployeeWeekOverviewPresenceTypeEnum,
    ComeGoEmployeeWeekOverviewTenseEnum, OpeningHoursType, PresenceDayOfWeek,
    PresenceModuleSettingsDashboardContext, PresenceModuleSettingsModule,
    PresenceModuleSettingsPermission, PresenceStatusEnum, PresenceTemplateRepeatPattern,
};
use crate::enums::profiles::PortalRole;

use super::messaging::DownloadFileFromAulaArguments;
use super::profiles::{MainGroup, SimpleInstitutionProfile};

// ===========================================================================
// Core presence types (DTOs.ComeGo)
// ===========================================================================

/// Presence registration result for a child.
///
/// Maps to `DTOs.ComeGo.PresenceRegistrationResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRegistrationResult {
    pub id: i64,
    pub institution_profile: Option<SimpleInstitutionProfile>,
    pub status: Option<PresenceStatusEnum>,
    pub activity_type: Option<ActivityTypeEnum>,
    pub location: Option<ComeGoLocation>,
    pub sleep_intervals: Option<Vec<SleepIntervalResult>>,
    pub editable_presence_states: Option<Vec<PresenceStatusEnum>>,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
    #[serde(default)]
    pub is_default_entry_time: bool,
    #[serde(default)]
    pub is_default_exit_time: bool,
    pub comment: Option<String>,
    pub spare_time_activity: Option<SpareTimeActivity>,
    pub vacation_note: Option<String>,
}

/// Child status in the ComeGo system.
///
/// Maps to `Models.ComeGo.ChildStatus`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildStatus {
    pub institution_profile_id: i64,
    pub state: Option<PresenceStatusEnum>,
    pub uni_student: Option<ComeGoUniStudentProfile>,
}

/// Child status DTO (includes UniStudentId).
///
/// Maps to `DTOs.ComeGo.ChildStatusDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildStatusDto {
    pub uni_student_id: i64,
    pub uni_student: Option<ComeGoUniStudentProfile>,
    pub state: Option<PresenceStatusEnum>,
}

/// ComeGo-specific student profile (referenced by ChildStatus).
///
/// Not explicitly defined in data_models.md; inferred from ComeGoInstitutionProfile pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoUniStudentProfile {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Location result in ComeGo.
///
/// Maps to `DTOs.ComeGo.ComeGoLocationResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoLocation {
    pub id: i64,
    pub name: Option<String>,
    pub symbol: Option<String>,
}

/// Presence location with description.
///
/// Maps to `DTOs.ComeGo.PresenceLocationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceLocation {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub symbol: Option<String>,
}

/// Physical location in activity list.
///
/// Maps to `DTOs.ComeGo.PhysicalLocationResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalLocation {
    pub id: i64,
    pub name: Option<String>,
}

/// Sleep interval result.
///
/// Maps to `DTOs.ComeGo.SleepIntervalResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepIntervalResult {
    pub id: i64,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Spare time activity details.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDay.SpareTimeActivityDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpareTimeActivity {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub comment: Option<String>,
}

/// Institution with available presence states.
///
/// Maps to `DTOs.ComeGo.InstitutionWithPresenceStatesResponseDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionWithPresenceStates {
    pub institution_code: Option<String>,
    pub presence_states: Option<Vec<PresenceStatusEnum>>,
}

/// Date/time period.
///
/// Maps to `DTOs.ComeGo.DateTimePeriodDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimePeriod {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ===========================================================================
// Presence schedule / template types (DTOs.ComeGo.PresenceDataViewModels)
// ===========================================================================

/// Interface-like base for presence day data.
///
/// Maps to `DTOs.ComeGo.IPresenceDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceDay {
    pub id: Option<i64>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
    pub by_date: Option<String>,
    pub comment: Option<String>,
    #[serde(default)]
    pub is_default_entry_time: bool,
    #[serde(default)]
    pub is_default_exit_time: bool,
    pub activity_type: Option<ActivityTypeEnum>,
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub spare_time_activity: Option<SpareTimeActivity>,
}

/// Presence day schedule (a single day within a template).
///
/// Maps to `DTOs.ComeGo.PresenceDataViewModels.PresenceDaySchedule`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceDaySchedule {
    // IPresenceDto fields
    pub id: Option<i64>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
    pub by_date: Option<String>,
    pub comment: Option<String>,
    #[serde(default)]
    pub is_default_entry_time: bool,
    #[serde(default)]
    pub is_default_exit_time: bool,
    pub activity_type: Option<ActivityTypeEnum>,
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub spare_time_activity: Option<SpareTimeActivity>,
    // PresenceDaySchedule-specific fields
    pub day_of_week: Option<PresenceDayOfWeek>,
    pub full_name: Option<String>,
    pub day_text: Option<String>,
    pub repeat_pattern: Option<PresenceTemplateRepeatPattern>,
    pub repeat_from_date: Option<String>,
    pub repeat_to_date: Option<String>,
    #[serde(default)]
    pub is_on_vacation: bool,
    #[serde(default)]
    pub is_planned_times_outside_opening_hours: bool,
}

/// Day template result (schedule for a child's week).
///
/// Maps to `DTOs.ComeGo.GetDayTemplateResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDayTemplateResult {
    pub current_date: Option<String>,
    pub presence_week_templates: Option<Vec<serde_json::Value>>,
}

/// Available presence statuses result.
///
/// Maps to `DTOs.ComeGo.GetAvailableStatusesResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAvailableStatusesResult {
    pub available_status: Option<Vec<serde_json::Value>>,
}

// ===========================================================================
// Activity list types (Models.ComeGo.ActivityList)
// ===========================================================================

/// Activity list result with child counts.
///
/// Maps to `Models.ComeGo.ActivityList.ActivityListResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityListResult {
    pub total_number_of_children: i32,
    pub number_of_children_present: i32,
    pub activities: Option<Vec<ActivityListChildPresence>>,
}

/// Single child's presence in the activity list.
///
/// Maps to `Models.ComeGo.ActivityList.ActivityListChildPresenceResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityListChildPresence {
    pub presence_registration_id: i64,
    pub uni_student: Option<ActivityListChild>,
    pub presence_state: Option<PresenceStatusEnum>,
    pub comment: Option<String>,
    pub note: Option<String>,
    pub location: Option<PresenceLocation>,
    pub editable_presence_states: Option<Vec<PresenceStatusEnum>>,
    pub past_activities: Option<Vec<PastPresenceActivity>>,
    pub future_activities: Option<Vec<FuturePresenceActivity>>,
    #[serde(default)]
    pub is_emphasized: bool,
    #[serde(default)]
    pub is_default_entry_times: bool,
    #[serde(default)]
    pub is_default_exit_times: bool,
}

/// Child profile within the activity list.
///
/// Maps to `Models.ComeGo.ActivityList.ActivityListChildResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityListChild {
    pub id: i64,
    pub institution_code: Option<String>,
    pub profile_id: i64,
    pub role: Option<PortalRole>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub name: Option<String>,
    pub main_group: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Past presence activity record.
///
/// Maps to `Models.ComeGo.ActivityList.PastPresenceActivityResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastPresenceActivity {
    pub check_in_time: Option<String>,
    pub checkout_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub activity_type: Option<ActivityTypeEnum>,
}

/// Future presence activity record.
///
/// Maps to `Models.ComeGo.ActivityList.FuturePresenceActivityResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturePresenceActivity {
    pub activity_type: Option<ActivityTypeEnum>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Activity list request parameters.
///
/// Maps to `Models.ComeGo.ActivityList.ActivityListRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityListRequest {
    pub department_id: i64,
    pub group_ids: Option<Vec<i64>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub states: Option<Vec<PresenceStatusEnum>>,
    pub next_activity: Option<ActivityTypeEnum>,
    pub location_ids: Option<Vec<i64>>,
    pub sort_on: Option<String>,
    pub daily_note: Option<String>,
}

/// Activity filter (extends PresenceFilter with presence states).
///
/// Maps to `Models.ComeGo.ActivityList.ActivityFilterResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFilterResult {
    // PresenceFilterResultModel fields
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub departments: Option<Vec<PresenceFilterDepartment>>,
    // ActivityFilterResultModel-specific
    pub presence_states: Option<Vec<PresenceStatusEnum>>,
    pub presence_next_activities: Option<Vec<ActivityTypeEnum>>,
    pub locations: Option<Vec<PhysicalLocation>>,
}

/// Institution with presence states (view model variant).
///
/// Maps to `Models.ComeGo.ActivityList.InstitutionWithPresenceStatesResultViewModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionWithPresenceStatesResult {
    pub institution_code: Option<String>,
    pub editable_presence_states: Option<Vec<PresenceStatusEnum>>,
}

// ===========================================================================
// Filter types (Models.ComeGo)
// ===========================================================================

/// Presence filter result (per institution).
///
/// Maps to `Models.ComeGo.PresenceFilterResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceFilterResult {
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub departments: Option<Vec<PresenceFilterDepartment>>,
}

/// Filter department within a presence filter.
///
/// Maps to `Models.ComeGo.PresenceFilterDepartmentModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceFilterDepartment {
    pub id: i64,
    pub filtering_groups: Option<Vec<PresenceFilterGroup>>,
    pub main_group: Option<MainGroup>,
    // SelectionControl inherited fields
    pub name: Option<String>,
    #[serde(default)]
    pub is_selected: bool,
}

/// Filter group within a department.
///
/// Maps to `Models.ComeGo.PresenceFilterGroupModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceFilterGroup {
    pub id: i64,
    pub name: Option<String>,
    // SelectionControl inherited fields
    #[serde(default)]
    pub is_selected: bool,
}

/// Presence filters request.
///
/// Maps to `DTOs.ComeGo.PresenceFiltersRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceFiltersRequest {
    pub institutions: Option<Vec<String>>,
}

/// Request for presence schedules.
///
/// Maps to `Models.ComeGo.PresenceSchedulesRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceSchedulesRequest {
    pub filter_institution_profile_ids: Option<Vec<i64>>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

// ===========================================================================
// Pickup responsible types (Models.ComeGo.PickupResponsible)
// ===========================================================================

/// Result for a child's pickup responsible persons.
///
/// Maps to `Models.ComeGo.PickupResponsible.GetPickupResponsibleChildResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPickupResponsibleChildResult {
    pub uni_student_id: i64,
    pub related_persons: Option<Vec<PresenceRelatedPersonPickResponsible>>,
    pub pickup_suggestions: Option<Vec<PresencePickupSuggestion>>,
}

/// Top-level pickup responsible result (wraps children).
///
/// Maps to `Models.ComeGo.PickupResponsible.GetPickupResponsibleResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPickupResponsibleResult {
    pub children: Option<Vec<GetPickupResponsibleChildResult>>,
}

/// Related person who can pick up a child.
///
/// Maps to `Models.ComeGo.PickupResponsible.PresenceRelatedPersonPickResponsibleResutModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRelatedPersonPickResponsible {
    pub institution_profile_id: Option<i64>,
    pub name: Option<String>,
    pub relation: Option<String>,
}

/// Pickup name suggestion.
///
/// Maps to `Models.ComeGo.PickupResponsible.PresencePickupSuggestionResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresencePickupSuggestion {
    pub id: i64,
    pub uni_student_id: i64,
    pub pick_up_name: Option<String>,
}

/// Request to get pickup responsible.
///
/// Maps to `Models.ComeGo.GetPickupResponsibleRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPickupResponsibleRequest {
    pub uni_student_ids: Option<Vec<i64>>,
}

/// Request to save a pickup name.
///
/// Maps to `Models.ComeGo.PickupResponsible.SavePickupNameRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavePickupNameRequest {
    pub id: i64,
    pub name: Option<String>,
}

/// Request to delete a pickup responsible entry.
///
/// Maps to `Models.ComeGo.PickupResponsible.DeletePickupResponsibleRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePickupResponsibleRequest {
    pub presence_pickup_suggestion_id: i64,
}

/// Exit-with suggestion (who will pick up a child).
///
/// Maps to `DTOs.ComeGo.ComeGoExitWithSuggestionModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoExitWithSuggestion {
    pub pickup_name: Option<String>,
    pub uni_student_id: i64,
}

/// Request for exit-with suggestions.
///
/// Maps to `DTOs.ComeGo.ComeGoExitWithSuggestionRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoExitWithSuggestionRequest {
    pub pickup_name: Option<String>,
    pub uni_student_ids: Option<Vec<i64>>,
}

/// Result wrapper for exit-with suggestions.
///
/// Maps to `DTOs.ComeGo.GetExitWithSuggestionsResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExitWithSuggestionsResult {
    pub suggestions: Option<Vec<ComeGoExitWithSuggestion>>,
}

// ===========================================================================
// Sleep interval types (Models.ComeGo.AddSleepIntervals)
// ===========================================================================

/// Request to add sleep intervals.
///
/// Maps to `Models.ComeGo.AddSleepIntervals.AddSleepIntervalsRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSleepIntervalsRequest {
    pub child_ids: Option<Vec<i64>>,
    pub start: Option<String>,
    pub end: Option<String>,
}

/// Request to update sleep intervals.
///
/// Maps to `Models.ComeGo.AddSleepIntervals.UpdateSleepIntervals`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSleepIntervals {
    pub sleep_interval_ids: Option<Vec<i64>>,
    pub start: Option<String>,
    pub end: Option<String>,
}

/// DTO for updating sleep intervals (with registration context).
///
/// Maps to `DTOs.ComeGo.UpdateSleepIntervalsDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSleepIntervalsDto {
    pub presence_registration_id: i64,
    pub id: i64,
    pub start: Option<String>,
    pub end: Option<String>,
}

// ===========================================================================
// Opening hours & closed days (DTOs.ComeGo.OpeningHoursAndClosedDays)
// ===========================================================================

/// General opening hours for a day of the week.
///
/// Maps to `DTOs.ComeGo.OpeningHours`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHours {
    pub institution_code: Option<String>,
    pub day_of_week: Option<PresenceDayOfWeek>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}

/// Institution opening hours wrapper.
///
/// Maps to `DTOs.ComeGo.InstitutionOpeningHours`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionOpeningHours {
    pub institution_code: Option<String>,
    pub opening_hours: Option<Vec<OpeningHours>>,
}

/// General opening hours result.
///
/// Maps to `DTOs.ComeGo.GetGeneralOpeningHoursDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGeneralOpeningHoursResult {
    pub institution_opening_hours: Option<Vec<InstitutionOpeningHours>>,
}

/// Opening hours for a specific date/period.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.OpeningHoursDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHoursDto {
    pub institution_code: Option<String>,
    pub date: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub opening_hours_type: Option<OpeningHoursType>,
}

/// Opening hours overview per institution.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.OpeningHoursOverviewDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningHoursOverview {
    pub institution_code: Option<String>,
    pub opening_hours_dto: Option<Vec<OpeningHoursDto>>,
}

/// Request for opening hours by institution codes.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.GetOpeningHoursByByInstitutionCodesRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpeningHoursByInstitutionCodesRequest {
    pub institution_codes: Option<Vec<String>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Result for opening hours by institution codes.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.GetOpeningHoursByInstitutionCodesDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpeningHoursByInstitutionCodesResult {
    pub opening_hours_overview_dto: Option<Vec<OpeningHoursOverview>>,
}

/// Specific opening hour entry.
///
/// Maps to `DTOs.ComeGo.SpecificOpeningHourDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecificOpeningHour {
    pub id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}

/// Specific opening hours per institution.
///
/// Maps to `DTOs.ComeGo.SpecificOpeningHourWithInstitutionDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecificOpeningHourWithInstitution {
    pub institution_code: Option<String>,
    pub specific_opening_hours: Option<Vec<SpecificOpeningHour>>,
}

/// Result for specific opening hours overview.
///
/// Maps to `DTOs.ComeGo.GetSpecificOpeningHourOverviewDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpecificOpeningHourOverviewResult {
    pub specific_opening_hours_with_institutions: Option<Vec<SpecificOpeningHourWithInstitution>>,
}

/// Closed day entry.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.ClosedDaysDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedDay {
    pub id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub name: Option<String>,
}

/// Closed days overview.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.ClosedDaysOverviewDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedDaysOverview {
    pub closed_days: Option<Vec<ClosedDay>>,
}

/// Institution closed days.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.InstitutionClosedDaysDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionClosedDays {
    pub institution_code: Option<String>,
    pub closed_days_overview: Option<ClosedDaysOverview>,
}

/// Result for closed days query.
///
/// Maps to `DTOs.ComeGo.OpeningHoursAndClosedDays.GetClosedDaysDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedDaysResult {
    pub institution_closed_days: Option<Vec<InstitutionClosedDays>>,
}

// ===========================================================================
// Presence configuration (Models.ComeGo.PresenceConfiguration)
// ===========================================================================

/// Configuration result for a child's presence.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceConfigurationChildResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceConfigurationChildResult {
    pub uni_student_id: i64,
    pub presence_configuration: Option<PresenceConfigurationResult>,
}

/// Full presence configuration result.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceConfigurationResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceConfigurationResult {
    #[serde(default)]
    pub self_decider: bool,
    #[serde(default)]
    pub go_home_with: bool,
    #[serde(default)]
    pub send_home: bool,
    #[serde(default)]
    pub pick_up: bool,
    pub institution: Option<PresenceConfigurationInstitution>,
    pub departments: Option<Vec<PresenceConfigurationDepartment>>,
    pub dashboard_module_settings: Option<Vec<PresenceModuleSettings>>,
}

/// Institution identity within presence configuration.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceConfigurationInstitution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceConfigurationInstitution {
    pub institution_code: Option<String>,
    pub name: Option<String>,
}

/// Department within presence configuration.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceConfigurationDepartment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceConfigurationDepartment {
    pub group: Option<PresenceConfigurationGroup>,
    pub filtering_groups: Option<Vec<PresenceConfigurationGroup>>,
}

/// Group within presence configuration.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceConfigurationGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceConfigurationGroup {
    pub id: i32,
    pub name: Option<String>,
}

/// Presence module with type and permission.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceModule`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceModule {
    pub module_type: Option<PresenceModuleSettingsModule>,
    pub permission: Option<PresenceModuleSettingsPermission>,
}

/// Presence module settings per dashboard context.
///
/// Maps to `Models.ComeGo.PresenceConfiguration.PresenceModuleSettings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceModuleSettings {
    pub presence_dashboard_context: Option<PresenceModuleSettingsDashboardContext>,
    pub presence_modules: Option<Vec<PresenceModule>>,
}

// ===========================================================================
// Daily overview (Models.ComeGo)
// ===========================================================================

/// Parent daily overview institution profile.
///
/// Maps to `DTOs.ComeGo.ParentDailyOverviewInstitutionProfileDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDailyOverviewInstitutionProfile {
    pub profile_id: i64,
    pub id: i64,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub role: Option<PortalRole>,
    pub name: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
    pub main_group: Option<MainGroup>,
    pub short_name: Option<String>,
    pub institution_role: Option<String>,
    pub metadata: Option<String>,
}

/// Parent daily overview result for a child.
///
/// Maps to `Models.ComeGo.ParentsDailyOverviewResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentsDailyOverviewResult {
    pub institution_profile: Option<ParentDailyOverviewInstitutionProfile>,
    pub main_group: Option<MainGroup>,
    pub status: Option<PresenceStatusEnum>,
    pub sleep_intervals: Option<Vec<SleepIntervalResult>>,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub location: Option<PresenceLocation>,
    #[serde(default)]
    pub is_default_entry_time: bool,
    #[serde(default)]
    pub is_default_exit_time: bool,
    #[serde(default)]
    pub is_planned_times_outside_opening_hours: bool,
}

// ===========================================================================
// Vacation types (DTOs.ComeGo.VacationRegistration + Models.ComeGo.ChildrenVacation)
// ===========================================================================

/// Vacation announcement.
///
/// Maps to `DTOs.ComeGo.VacationAnnouncementDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationAnnouncement {
    pub vacation_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub is_editable: bool,
}

/// Vacation announcements grouped by child.
///
/// Maps to `DTOs.ComeGo.VacationAnnouncementsByChildrenDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationAnnouncementsByChildren {
    pub child: Option<ParentDailyOverviewInstitutionProfile>,
    pub vacation_announcements: Option<Vec<VacationAnnouncement>>,
}

/// Vacation registration (staff-created).
///
/// Maps to `DTOs.ComeGo.VacationRegistrationDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistration {
    pub vacation_registration_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub title: Option<String>,
    pub note_to_guardian: Option<String>,
    pub response_id: i64,
    pub response_deadline: Option<String>,
    #[serde(default)]
    pub is_editable: bool,
    #[serde(default)]
    pub is_missing_answer: bool,
    #[serde(default)]
    pub is_presence_times_required: bool,
}

/// Vacation registrations grouped by child.
///
/// Maps to `DTOs.ComeGo.VacationRegistrationsByChildrenDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistrationsByChildren {
    pub child: Option<ParentDailyOverviewInstitutionProfile>,
    pub vacation_registrations: Option<Vec<VacationRegistration>>,
}

/// Vacation entry (guardian submits intervals).
///
/// Maps to `DTOs.ComeGo.VacationEntryDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationEntry {
    pub child_ids: Option<Vec<i64>>,
    pub intervals: Option<Vec<VacationIntervals>>,
    pub comment: Option<String>,
}

/// Vacation interval (date range).
///
/// Maps to `Models.ComeGo.VacationIntervals`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationIntervals {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Guardian vacation registration intervals.
///
/// Maps to `Models.ComeGo.GuardianRegisterVacationIntervals`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardianRegisterVacationIntervals {
    pub date: Option<String>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    #[serde(default)]
    pub is_coming: bool,
}

/// Children vacation request parameters.
///
/// Maps to `Models.ComeGo.ChildrenVacation.ChildrenVacationRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenVacationRequest {
    pub department_id: i64,
    pub group_ids: Option<Vec<i64>>,
    pub date: Option<String>,
    pub offset: i32,
    pub limit: i32,
}

/// Children vacation result (paginated).
///
/// Maps to `Models.ComeGo.ChildrenVacation.ChildrenVacationResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenVacationResult {
    pub count: i32,
    pub children: Option<Vec<ChildrenVacationChild>>,
}

/// Child within vacation result.
///
/// Maps to `Models.ComeGo.ChildrenVacation.ChildrenVacationChildResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenVacationChild {
    pub child: Option<ChildrenVacationChildProfile>,
    pub note: Option<String>,
}

/// Child profile within vacation context.
///
/// Maps to `Models.ComeGo.ChildrenVacation.ChildrenVacationChildProfileResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenVacationChildProfile {
    pub profile_id: i64,
    pub id: i64,
    pub short_name: Option<String>,
    pub name: Option<String>,
    pub metadata: Option<String>,
    pub profile_picture: Option<DownloadFileFromAulaArguments>,
}

/// Vacation filter (extends PresenceFilterResult).
///
/// Maps to `Models.ComeGo.ChildrenVacation.VacationFilterResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationFilterResult {
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub departments: Option<Vec<PresenceFilterDepartment>>,
}

/// Respond to vacation registration request.
///
/// Maps to `DTOs.ComeGo.RespondToVacationRegistrationRequestRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespondToVacationRegistrationRequest {
    pub child_id: Option<i64>,
    pub vacation_registration_response_id: Option<i64>,
    pub days: Option<Vec<GuardianRegisterVacationIntervals>>,
    pub comment: Option<String>,
}

/// Vacation registration response for guardian.
///
/// Maps to `DTOs.ComeGo.VacationRegistrationResponseForGuardianDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistrationResponseForGuardian {
    pub vacation_registration: Option<VacationRegistration>,
    pub vacation_registration_response: Option<RespondToVacationRegistrationRequest>,
}

/// Create vacation registration request (staff).
///
/// Maps to `DTOs.ComeGo.VacationRegistration.CreateVacationRegistrationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateVacationRegistrationRequest {
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub response_deadline: Option<String>,
    pub creator_inst_profile_id: i64,
    pub title: Option<String>,
    pub departments: Option<Vec<DepartmentIdsSimpleRequest>>,
    pub note_to_guardians: Option<String>,
    #[serde(default)]
    pub is_presence_times_required: bool,
}

/// Simple department ID reference.
///
/// Maps to `DTOs.ComeGo.VacationRegistration.DepartmentIdsSimpleRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentIdsSimpleRequest {
    pub group_id: i64,
    pub filtering_groups: Option<Vec<i64>>,
}

/// Update vacation registration request.
///
/// Maps to `DTOs.ComeGo.VacationRegistration.UpdateVacationRegistrationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVacationRegistrationRequest {
    pub id: i64,
    pub response_deadline: Option<String>,
}

// ===========================================================================
// Employee week overview (DTOs.ComeGo.EmployeeWeekOverview)
// ===========================================================================

/// Presence time with tense (past/present/future).
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.ComeGoPresenceTimeWithTense`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoPresenceTimeWithTense {
    pub timestamp: Option<String>,
    pub tense: Option<ComeGoEmployeeWeekOverviewTenseEnum>,
}

/// Presence details for employee week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.ComeGoEmployeeWeekOverviewPresenceDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeWeekOverviewPresenceDetails {
    pub start_time: Option<ComeGoPresenceTimeWithTense>,
    pub end_time: Option<ComeGoPresenceTimeWithTense>,
}

/// Vacation details for employee week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.ComeGoEmployeeWeekOverviewVacationDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeWeekOverviewVacationDetails {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Activities for a single day in week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.EmployeeWeekOverviewActivitiesDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeWeekOverviewActivities {
    pub date: Option<String>,
    #[serde(rename = "type")]
    pub presence_type: Option<ComeGoEmployeeWeekOverviewPresenceTypeEnum>,
    pub presence_details: Option<EmployeeWeekOverviewPresenceDetails>,
    pub vacation_details: Option<EmployeeWeekOverviewVacationDetails>,
}

/// Child activities in employee week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.EmployeeWeekOverviewChildActivitiesDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeWeekOverviewChildActivities {
    pub child: Option<ActivityListChild>,
    pub activities: Option<Vec<EmployeeWeekOverviewActivities>>,
    pub presence_registration_id: Option<i64>,
    pub presence_registration_is_default_entry_time: Option<bool>,
    pub presence_registration_is_default_exit_time: Option<bool>,
}

/// Presence overview result for employee week view.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.GetPresenceOverviewDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPresenceOverview {
    pub week_number: i32,
    pub presence_days: Option<Vec<WeekOverviewPresenceDays>>,
    pub child_activities: Option<Vec<EmployeeWeekOverviewChildActivities>>,
}

/// Day summary in week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.WeekOverviewPresenceDaysDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekOverviewPresenceDays {
    pub date: Option<String>,
    pub number_of_children: i32,
    pub total_number_of_children: i32,
}

/// Children distribution in a presence overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.PresenceChildrenDistributionDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceChildrenDistribution {
    pub number_present: i32,
    pub number_on_vacation: i32,
    pub number_sick: i32,
    pub number_not_arrived: i32,
    pub intervals: Option<Vec<PresenceIntervalModel>>,
    #[serde(default)]
    pub is_distribution_enabled: bool,
}

/// Presence interval (time slot with child count).
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.PresenceIntervalModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceIntervalModel {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub number_of_children: Option<String>,
    #[serde(default)]
    pub is_current: bool,
}

/// Children presence distribution request.
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.ChildrenPresenceDistributionModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenPresenceDistributionRequest {
    pub showing_date: Option<String>,
    pub dto: Option<PresenceChildrenDistributionRequestDto>,
}

/// Presence children distribution request DTO.
///
/// Maps to `DTOs.ComeGo.PresenceChildrenDistributionRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceChildrenDistributionRequestDto {
    pub department_id: i64,
    pub date: Option<String>,
    pub group_ids: Option<Vec<i64>>,
    pub status_filters: Option<Vec<String>>,
}

/// Request to get employee week overview.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.ComeGoGetWeekOverviewRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoGetWeekOverviewRequest {
    pub department_id: i64,
    pub group_ids: Option<Vec<i64>>,
    pub status_filters: Option<Vec<String>>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub offset: i32,
    pub limit: i32,
}

/// Request to get vacation registration overview (employee view).
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.ComeGoGetVacationRegistrationOverviewRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComeGoGetVacationRegistrationOverviewRequest {
    pub department_id: i64,
    pub filter_groups: Option<Vec<i64>>,
    pub status_filters: Option<Vec<String>>,
    pub offset: i32,
    pub limit: i32,
}

/// Vacation registration overview result (employee view).
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.GetVacationRegistrationOverviewDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVacationRegistrationOverview {
    pub total_number: i32,
    pub vacation_registrations: Option<Vec<VacationRegistrationsDto>>,
}

/// Vacation registration entry in overview list.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.VacationRegistrationsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistrationsDto {
    pub vacation_registration_id: i32,
    pub title: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub deadline: Option<String>,
    pub regarding_department_and_groups_text: Option<Vec<String>>,
    pub subtitle: Option<String>,
    pub short_name: Option<String>,
}

/// Overall item in employee week overview.
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.OverallItemModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverallItem {
    pub left_text: Option<String>,
    pub left_text_accessibility: Option<String>,
    pub right_text: Option<String>,
}

/// Employee week overview presence record.
///
/// Maps to `DTOs.ComeGo.EmployeeWeekOverview.EmployeeWeekOverviewPresenceDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeWeekOverviewPresence {
    pub activity_type: Option<ActivityTypeEnum>,
    pub by_date: Option<String>,
    pub comment: Option<String>,
    pub day_of_week: Option<PresenceDayOfWeek>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
    pub id: Option<i64>,
    #[serde(default)]
    pub is_on_vacation: bool,
    #[serde(default)]
    pub is_repeating: bool,
    pub repeat_from_date: Option<String>,
    pub repeat_to_date: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub self_decider_start_time: Option<String>,
    pub spare_time_activity: Option<SpareTimeActivity>,
    pub vacation: Option<serde_json::Value>,
}

/// Week overview future date model.
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.WeekOverviewFutureDateModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekOverviewFutureDate {
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
    pub exit_with: Option<String>,
    pub activity_type: Option<ActivityTypeEnum>,
}

/// Presence registration request (by child and date).
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.PresenceRegistrationRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRegistrationRequest {
    pub child_id: i64,
    pub date: Option<String>,
}

/// Presence registration today request (by registration IDs).
///
/// Maps to `Models.ComeGo.EmployeeWeekOverview.PresenceRegistrationTodayRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceRegistrationTodayRequest {
    pub presence_registration_ids: Option<Vec<i64>>,
    pub department_id: Option<String>,
}

// ===========================================================================
// Request types for presence updates
// ===========================================================================

/// Bulk update presence status request.
///
/// Maps to `DTOs.ComeGo.BulkUpdatePresenceStatusRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkUpdatePresenceStatusRequest {
    pub presence_registration_ids: Option<Vec<i64>>,
    pub status: Option<PresenceStatusEnum>,
}

/// Update status by institution profile IDs.
///
/// Maps to `DTOs.ComeGo.UpdateStatusByInstitutionProfileIdsDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatusByInstitutionProfileIds {
    pub institution_profile_ids: Option<Vec<i64>>,
    pub status: i32,
}

/// Update status by IDs.
///
/// Maps to `DTOs.ComeGo.UpdateStatusDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub ids: Option<Vec<i64>>,
    pub status: i32,
}

/// Update presence registration request (checkout details).
///
/// Maps to `Models.ComeGo.ActivityList.UpdatePresenceRegistrationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceRegistrationRequest {
    pub registration_id: i64,
    pub checkout_type: Option<ActivityTypeEnum>,
    pub pickup_by: Option<UpdateCheckoutPickedUpActivity>,
    pub self_decider: Option<UpdateCheckoutSelfDeciderActivity>,
    pub send_home: Option<UpdateCheckoutSendHomeActivity>,
    pub go_home_with: Option<UpdateCheckoutGoHomeWithActivity>,
    pub entry_time: Option<String>,
    pub remark: Option<String>,
}

/// Checkout picked-up activity.
///
/// Maps to `Models.ComeGo.ActivityList.UpdateCheckoutPickedUpActivityRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckoutPickedUpActivity {
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
}

/// Checkout self-decider activity.
///
/// Maps to `Models.ComeGo.ActivityList.UpdateCheckoutSelfDeciderActivityRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckoutSelfDeciderActivity {
    pub self_decider_start_time: Option<String>,
    pub self_decider_end_time: Option<String>,
}

/// Checkout send-home activity.
///
/// Maps to `Models.ComeGo.ActivityList.UpdateCheckoutDaySendHomeActivityRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckoutSendHomeActivity {
    pub exit_time: Option<String>,
}

/// Checkout go-home-with activity.
///
/// Maps to `Models.ComeGo.ActivityList.UpdateCheckoutDayGoHomeWithActivityRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheckoutGoHomeWithActivity {
    pub exit_with: Option<String>,
    pub exit_time: Option<String>,
}

/// Update presence day request (template editing).
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDayRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDayRequest {
    pub institution_profile_id: i64,
    pub id: Option<i64>,
    pub day_of_week: i32,
    pub by_date: Option<String>,
    pub comment: Option<String>,
    pub spare_time_activity: Option<SpareTimeActivityRequest>,
    pub presence_activity: Option<UpdatePresenceDayActivity>,
    pub repeat_pattern: Option<PresenceTemplateRepeatPattern>,
    pub expires_at: Option<String>,
}

/// Spare time activity request model.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDay.SpareTimeActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpareTimeActivityRequest {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub comment: Option<String>,
}

/// Update presence day activity (within template).
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDayActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDayActivity {
    pub activity_type: Option<ActivityTypeEnum>,
    pub pickup: Option<UpdatePresenceDayPickedUpActivity>,
    pub self_decider: Option<UpdatePresenceDaySelfDeciderActivity>,
    pub send_home: Option<UpdatePresenceDaySendHomeActivity>,
    pub go_home_with: Option<UpdatePresenceDayGoHomeWithActivity>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
}

/// Picked-up activity in presence day update.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDayPickedUpActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDayPickedUpActivity {
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
    pub exit_with: Option<String>,
}

/// Self-decider activity in presence day update.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDaySelfDeciderActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDaySelfDeciderActivity {
    pub entry_time: Option<String>,
    pub exit_start_time: Option<String>,
    pub exit_end_time: Option<String>,
}

/// Send-home activity in presence day update.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDaySendHomeActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDaySendHomeActivity {
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
}

/// Go-home-with activity in presence day update.
///
/// Maps to `DTOs.ComeGo.UpdatePresenceDayGoHomeWithActivityRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePresenceDayGoHomeWithActivity {
    pub exit_with: Option<String>,
    pub entry_time: Option<String>,
    pub exit_time: Option<String>,
}

/// Delete presence template request.
///
/// Maps to `Models.ComeGo.DeletePresenceTemplateRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePresenceTemplateRequest {
    pub delete_from_day: Option<String>,
    pub present_template_id: Option<i64>,
}

/// Get overlapping presence templates request.
///
/// Maps to `Models.ComeGo.GetOverlappingPresenceTemplatesRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOverlappingPresenceTemplatesRequest {
    pub institution_profile_id: i64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub repeat_pattern: Option<PresenceTemplateRepeatPattern>,
}

/// Update location request.
///
/// Maps to `Models.ComeGo.UpdateLocation.UpdateLocationRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLocationRequest {
    pub child_ids: Option<Vec<i64>>,
    pub location_id: Option<i64>,
}

/// Update pickup responsible result.
///
/// Maps to `Models.ComeGo.ActivityList.UpdatePickUpResponsibleDataModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePickUpResponsibleResult {
    #[serde(default)]
    pub result: bool,
    #[serde(default)]
    pub has_white_space_error: bool,
}

/// Child go-home-with result (parents register activity).
///
/// Maps to `Models.ComeGo.ParentsRegisterActivity.ChildGoHomeWithResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildGoHomeWithResult {
    pub institution_profile_id: i32,
    pub full_name: Option<String>,
    pub main_group: Option<String>,
}

/// Result wrapper for child go-home-with query.
///
/// Maps to `Models.ComeGo.ParentsRegisterActivity.GetChildGoHomeWithResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChildGoHomeWithResult {
    pub children: Option<Vec<ChildGoHomeWithResult>>,
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_presence_registration() {
        let json = r#"{
            "id": 12345,
            "institutionProfile": null,
            "status": "Present",
            "activityType": null,
            "location": {
                "id": 1,
                "name": "Stue A",
                "symbol": "house"
            },
            "sleepIntervals": [],
            "editablePresenceStates": ["Present", "Sick", "NotPresent"],
            "checkInTime": "07:30",
            "checkOutTime": null,
            "selfDeciderStartTime": null,
            "selfDeciderEndTime": null,
            "entryTime": "07:30",
            "exitTime": "15:00",
            "exitWith": null,
            "isDefaultEntryTime": false,
            "isDefaultExitTime": true,
            "comment": "Hentes tidligt",
            "spareTimeActivity": null,
            "vacationNote": null
        }"#;
        let reg: PresenceRegistrationResult = serde_json::from_str(json).unwrap();
        assert_eq!(reg.id, 12345);
        assert_eq!(reg.status, Some(PresenceStatusEnum::Present));
        assert_eq!(
            reg.location.as_ref().unwrap().name.as_deref(),
            Some("Stue A")
        );
        assert_eq!(reg.editable_presence_states.as_ref().unwrap().len(), 3);
        assert!(!reg.is_default_entry_time);
        assert!(reg.is_default_exit_time);
    }

    #[test]
    fn deserialize_child_status() {
        let json = r#"{
            "institutionProfileId": 42,
            "state": "Sick",
            "uniStudent": null
        }"#;
        let cs: ChildStatus = serde_json::from_str(json).unwrap();
        assert_eq!(cs.institution_profile_id, 42);
        assert_eq!(cs.state, Some(PresenceStatusEnum::Sick));
    }

    #[test]
    fn deserialize_activity_list_result() {
        let json = r#"{
            "totalNumberOfChildren": 25,
            "numberOfChildrenPresent": 18,
            "activities": []
        }"#;
        let result: ActivityListResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.total_number_of_children, 25);
        assert_eq!(result.number_of_children_present, 18);
        assert!(result.activities.as_ref().unwrap().is_empty());
    }

    #[test]
    fn deserialize_presence_configuration() {
        let json = r#"{
            "selfDecider": true,
            "goHomeWith": true,
            "sendHome": false,
            "pickUp": true,
            "institution": {
                "institutionCode": "101001",
                "name": "Test Skole"
            },
            "departments": [],
            "dashboardModuleSettings": [{
                "presenceDashboardContext": "GuardianDashboardSettings",
                "presenceModules": [{
                    "moduleType": "PickupType",
                    "permission": "Editable"
                }]
            }]
        }"#;
        let config: PresenceConfigurationResult = serde_json::from_str(json).unwrap();
        assert!(config.self_decider);
        assert!(config.go_home_with);
        assert!(!config.send_home);
        assert!(config.pick_up);
        assert_eq!(
            config.institution.as_ref().unwrap().name.as_deref(),
            Some("Test Skole")
        );
        let settings = config.dashboard_module_settings.as_ref().unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(
            settings[0].presence_dashboard_context,
            Some(PresenceModuleSettingsDashboardContext::GuardianDashboardSettings)
        );
    }

    #[test]
    fn deserialize_opening_hours() {
        let json = r#"{
            "institutionCode": "101001",
            "dayOfWeek": "Monday",
            "openTime": "06:30",
            "closeTime": "17:00"
        }"#;
        let oh: OpeningHours = serde_json::from_str(json).unwrap();
        assert_eq!(oh.institution_code.as_deref(), Some("101001"));
        assert_eq!(oh.day_of_week, Some(PresenceDayOfWeek::Monday));
        assert_eq!(oh.open_time.as_deref(), Some("06:30"));
    }

    #[test]
    fn deserialize_closed_day() {
        let json = r#"{
            "id": 99,
            "startDate": "2026-12-24T00:00:00",
            "endDate": "2026-12-26T00:00:00",
            "name": "Juleferie"
        }"#;
        let cd: ClosedDay = serde_json::from_str(json).unwrap();
        assert_eq!(cd.id, 99);
        assert_eq!(cd.name.as_deref(), Some("Juleferie"));
    }

    #[test]
    fn deserialize_presence_filter() {
        let json = r#"{
            "institutionCode": "101001",
            "institutionName": "Test Skole",
            "departments": [{
                "id": 5,
                "filteringGroups": [],
                "mainGroup": null,
                "name": "Afdeling A",
                "isSelected": true
            }]
        }"#;
        let pf: PresenceFilterResult = serde_json::from_str(json).unwrap();
        assert_eq!(pf.institution_code.as_deref(), Some("101001"));
        let dept = &pf.departments.as_ref().unwrap()[0];
        assert_eq!(dept.id, 5);
        assert!(dept.is_selected);
    }

    #[test]
    fn deserialize_pickup_responsible() {
        let json = r#"{
            "uniStudentId": 77,
            "relatedPersons": [{
                "institutionProfileId": 100,
                "name": "Karen Hansen",
                "relation": "Mother"
            }],
            "pickupSuggestions": [{
                "id": 1,
                "uniStudentId": 77,
                "pickUpName": "Farmor Inge"
            }]
        }"#;
        let result: GetPickupResponsibleChildResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.uni_student_id, 77);
        assert_eq!(result.related_persons.as_ref().unwrap().len(), 1);
        assert_eq!(
            result.pickup_suggestions.as_ref().unwrap()[0]
                .pick_up_name
                .as_deref(),
            Some("Farmor Inge")
        );
    }

    #[test]
    fn deserialize_vacation_registration() {
        let json = r#"{
            "vacationRegistrationId": 10,
            "startDate": "2026-07-01T00:00:00",
            "endDate": "2026-07-14T00:00:00",
            "title": "Sommerferie",
            "noteToGuardian": "Husk at svare",
            "responseId": 3,
            "responseDeadline": "2026-06-15T00:00:00",
            "isEditable": true,
            "isMissingAnswer": false,
            "isPresenceTimesRequired": true
        }"#;
        let vr: VacationRegistration = serde_json::from_str(json).unwrap();
        assert_eq!(vr.vacation_registration_id, 10);
        assert_eq!(vr.title.as_deref(), Some("Sommerferie"));
        assert!(vr.is_editable);
        assert!(vr.is_presence_times_required);
    }

    #[test]
    fn deserialize_presence_day_schedule() {
        let json = r#"{
            "id": 5,
            "entryTime": "2026-03-18T07:00:00",
            "exitTime": "2026-03-18T15:00:00",
            "exitWith": null,
            "byDate": "2026-03-18",
            "comment": null,
            "isDefaultEntryTime": false,
            "isDefaultExitTime": false,
            "activityType": "PICKED_UP_BY",
            "selfDeciderStartTime": null,
            "selfDeciderEndTime": null,
            "spareTimeActivity": null,
            "dayOfWeek": "Tuesday",
            "fullName": "Lars Hansen",
            "dayText": "Tirsdag",
            "repeatPattern": "Weekly",
            "repeatFromDate": "2026-01-06",
            "repeatToDate": null,
            "isOnVacation": false,
            "isPlannedTimesOutsideOpeningHours": false
        }"#;
        let pds: PresenceDaySchedule = serde_json::from_str(json).unwrap();
        assert_eq!(pds.id, Some(5));
        assert_eq!(pds.day_of_week, Some(PresenceDayOfWeek::Tuesday));
        assert_eq!(
            pds.repeat_pattern,
            Some(PresenceTemplateRepeatPattern::Weekly)
        );
        assert_eq!(pds.activity_type, Some(ActivityTypeEnum::PickedUpBy));
    }

    #[test]
    fn deserialize_week_overview_presence_days() {
        let json = r#"{
            "date": "2026-03-16",
            "numberOfChildren": 22,
            "totalNumberOfChildren": 25
        }"#;
        let wop: WeekOverviewPresenceDays = serde_json::from_str(json).unwrap();
        assert_eq!(wop.number_of_children, 22);
        assert_eq!(wop.total_number_of_children, 25);
    }

    #[test]
    fn deserialize_update_presence_day_request() {
        let json = r#"{
            "institutionProfileId": 42,
            "id": null,
            "dayOfWeek": 1,
            "byDate": "2026-03-18",
            "comment": "test",
            "spareTimeActivity": null,
            "presenceActivity": {
                "activityType": "PICKED_UP_BY",
                "pickup": {
                    "entryTime": "07:00",
                    "exitTime": "15:00",
                    "exitWith": "Mor"
                },
                "selfDecider": null,
                "sendHome": null,
                "goHomeWith": null,
                "entryTime": "07:00",
                "exitTime": "15:00"
            },
            "repeatPattern": "Weekly",
            "expiresAt": null
        }"#;
        let req: UpdatePresenceDayRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.institution_profile_id, 42);
        assert_eq!(req.day_of_week, 1);
        let activity = req.presence_activity.as_ref().unwrap();
        assert_eq!(activity.activity_type, Some(ActivityTypeEnum::PickedUpBy));
        assert_eq!(
            activity.pickup.as_ref().unwrap().exit_with.as_deref(),
            Some("Mor")
        );
    }

    #[test]
    fn deserialize_bulk_update_status() {
        let json = r#"{
            "presenceRegistrationIds": [1, 2, 3],
            "status": "Sick"
        }"#;
        let req: BulkUpdatePresenceStatusRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.presence_registration_ids.as_ref().unwrap().len(), 3);
        assert_eq!(req.status, Some(PresenceStatusEnum::Sick));
    }

    #[test]
    fn deserialize_parent_daily_overview() {
        let json = r#"{
            "institutionProfile": null,
            "mainGroup": null,
            "status": "Present",
            "sleepIntervals": [],
            "checkInTime": "2026-03-18T07:15:00",
            "checkOutTime": null,
            "location": {
                "id": 1,
                "name": "Stue B",
                "description": "Den store stue",
                "symbol": "star"
            },
            "isDefaultEntryTime": false,
            "isDefaultExitTime": true,
            "isPlannedTimesOutsideOpeningHours": false
        }"#;
        let pdo: ParentsDailyOverviewResult = serde_json::from_str(json).unwrap();
        assert_eq!(pdo.status, Some(PresenceStatusEnum::Present));
        assert_eq!(
            pdo.location.as_ref().unwrap().description.as_deref(),
            Some("Den store stue")
        );
        assert!(!pdo.is_planned_times_outside_opening_hours);
    }
}
