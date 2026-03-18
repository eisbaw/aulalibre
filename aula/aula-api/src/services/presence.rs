//! Presence (ComeGo) service.
//!
//! Maps to `AulaNative.Services.Web.PresenceWebService` (40+ methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.6.
//!
//! ## PresenceWebService (Section 3.6)
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_childrens_state` | GET | `/presence/children/state` |
//! | `get_presence_registrations` | GET | `/presence/registrations` |
//! | `get_presence_registration_detail` | GET | `/presence/registrations/{id}` |
//! | `update_presence_registration` | PUT | `/presence/registrations/{id}` |
//! | `update_status_by_presence_registration_ids` | PUT | `/presence/registrations/status` |
//! | `update_status_by_institution_profile_ids` | PUT | `/presence/status/byProfiles` |
//! | `get_presence_schedules` | GET | `/presence/schedules` |
//! | `get_presence_week_overview` | GET | `/presence/weekOverview` |
//! | `update_one_day_presence` | PUT | `/presence/oneDay` |
//! | `get_template_for_date` | GET | `/presence/templates/{date}` |
//! | `delete_repeated_presence_template` | DELETE | `/presence/templates/repeated/{id}` |
//! | `get_overlapping_presence_templates` | GET | `/presence/templates/overlapping` |
//! | `get_suggestions_for_pickup` | GET | `/presence/pickup/suggestions` |
//! | `update_suggestions_for_pickup` | PUT | `/presence/pickup/suggestions` |
//! | `get_pickup_responsibles` | GET | `/presence/pickup/responsibles` |
//! | `delete_pickup_responsible` | DELETE | `/presence/pickup/responsibles/{id}` |
//! | `get_child_go_home_with` | GET | `/presence/children/{id}/goHomeWith` |
//! | `add_sleep_intervals` | POST | `/presence/sleep` |
//! | `update_sleep_interval` | PUT | `/presence/sleep/{id}` |
//! | `delete_sleep_intervals` | DELETE | `/presence/sleep` |
//! | `get_activity_list` | GET | `/presence/activities` |
//! | `get_activity_filter` | GET | `/presence/activities/filter` |
//! | `get_daily_overview` | GET | `/presence/daily/overview` |
//! | `get_available_locations` | GET | `/presence/locations` |
//! | `update_location` | PUT | `/presence/location` |
//! | `add_vacation` | POST | `/presence/vacation` |
//! | `get_children_vacation` | GET | `/presence/children/vacation` |
//! | `get_vacation_announcements_by_children` | GET | `/presence/vacation/announcements` |
//! | `get_vacation_registration_overview` | GET | `/presence/vacation/registrations/overview` |
//! | `get_vacation_registrations_by_children` | GET | `/presence/vacation/registrations/children` |
//! | `get_existing_vacation_registration_response` | GET | `/presence/vacation/registrations/existing` |
//! | `get_presence_configuration` | GET | `/presence/configuration` |
//! | `get_presence_configuration_by_children_ids` | GET | `/presence/configuration/children` |
//! | `get_presence_filter` | GET | `/presence/filter` |
//! | `get_presence_filters` | GET | `/presence/filters` |
//! | `get_closed_days` | GET | `/presence/closedDays` |
//! | `get_general_opening_hours` | GET | `/presence/openingHours` |
//! | `get_opening_hours_by_institution_codes` | GET | `/presence/openingHours/institution` |
//! | `get_specific_opening_hour_overview` | GET | `/presence/openingHours/specific` |
//! | `get_available_presence_statuses` | GET | `/presence/statuses` |
//! | `get_institution_with_presence_states` | GET | `/presence/institution/states` |
//! | `get_presence_children_distribution` | GET | `/presence/children/distribution` |

use crate::models::presence::{
    ActivityFilterResult, ActivityListRequest, ActivityListResult, AddSleepIntervalsRequest,
    BulkUpdatePresenceStatusRequest, ChildStatus, ChildrenVacationRequest, ChildrenVacationResult,
    ComeGoExitWithSuggestionRequest, ComeGoGetVacationRegistrationOverviewRequest,
    ComeGoGetWeekOverviewRequest, DeletePickupResponsibleRequest, DeletePresenceTemplateRequest,
    GetAvailableStatusesResult, GetChildGoHomeWithResult, GetClosedDaysResult,
    GetDayTemplateResult, GetExitWithSuggestionsResult, GetGeneralOpeningHoursResult,
    GetOpeningHoursByInstitutionCodesRequest, GetOpeningHoursByInstitutionCodesResult,
    GetOverlappingPresenceTemplatesRequest, GetPickupResponsibleRequest,
    GetPickupResponsibleResult, GetPresenceOverview, GetSpecificOpeningHourOverviewResult,
    GetVacationRegistrationOverview, InstitutionWithPresenceStates, ParentsDailyOverviewResult,
    PresenceChildrenDistribution, PresenceChildrenDistributionRequestDto,
    PresenceConfigurationChildResult, PresenceConfigurationResult, PresenceFilterResult,
    PresenceFiltersRequest, PresenceLocation, PresenceRegistrationResult, PresenceSchedulesRequest,
    SavePickupNameRequest, UpdateLocationRequest, UpdatePickUpResponsibleResult,
    UpdatePresenceDayRequest, UpdatePresenceRegistrationRequest, UpdateSleepIntervalsDto,
    UpdateStatusByInstitutionProfileIds, VacationAnnouncementsByChildren, VacationEntry,
    VacationRegistrationResponseForGuardian, VacationRegistrationsByChildren,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Response from presence registration mutation operations.
pub type PresenceRegistrationMutationResponse = serde_json::Value;

/// Response from status update operations.
pub type StatusUpdateResponse = serde_json::Value;

/// Response from template mutation operations.
pub type TemplateMutationResponse = serde_json::Value;

/// Response from sleep interval mutation operations.
pub type SleepMutationResponse = serde_json::Value;

/// Response from location update operations.
pub type LocationUpdateResponse = serde_json::Value;

/// Response from vacation mutation operations.
pub type VacationMutationResponse = serde_json::Value;

/// Response from pickup suggestion update.
pub type PickupSuggestionUpdateResponse = serde_json::Value;

/// Response from pickup responsible deletion.
pub type PickupResponsibleDeleteResponse = serde_json::Value;

// ===========================================================================
// Child state and registration (AC #1)
// ===========================================================================

/// Get children's current presence state.
///
/// Maps to `PresenceWebService.GetChildrensState()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/children/state`
pub async fn get_childrens_state(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ChildStatus>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/children/state".to_string()
    } else {
        format!("presence/children/state?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence registrations (list).
///
/// Maps to `PresenceWebService.GetPresenceRegistrations()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/registrations`
pub async fn get_presence_registrations(
    session: &mut Session,
    inst_profile_ids: &[i64],
    date: Option<&str>,
) -> crate::Result<Vec<PresenceRegistrationResult>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    if let Some(d) = date {
        query.push(format!("date={d}"));
    }
    let path = if query.is_empty() {
        "presence/registrations".to_string()
    } else {
        format!("presence/registrations?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence registration detail by ID.
///
/// Maps to `PresenceWebService.GetPresenceRegistrationDetail()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/registrations/{registration_id}`
pub async fn get_presence_registration_detail(
    session: &mut Session,
    registration_id: i64,
) -> crate::Result<PresenceRegistrationResult> {
    session
        .get(&format!("presence/registrations/{registration_id}"))
        .await
}

/// Update a presence registration (checkout details).
///
/// Maps to `PresenceWebService.UpdatePresenceRegistration()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/registrations/{registration_id}`
pub async fn update_presence_registration(
    session: &mut Session,
    args: &UpdatePresenceRegistrationRequest,
) -> crate::Result<PresenceRegistrationMutationResponse> {
    let registration_id = args.registration_id;
    session
        .put(&format!("presence/registrations/{registration_id}"), args)
        .await
}

/// Bulk update presence status by registration IDs.
///
/// Maps to `PresenceWebService.UpdateStatusByPresenceRegistrationIds()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/registrations/status`
pub async fn update_status_by_presence_registration_ids(
    session: &mut Session,
    args: &BulkUpdatePresenceStatusRequest,
) -> crate::Result<StatusUpdateResponse> {
    session.put("presence/registrations/status", args).await
}

/// Update presence status by institution profile IDs.
///
/// Maps to `PresenceWebService.UpdateStatusByInstitutionProfileIds()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/status/byProfiles`
pub async fn update_status_by_institution_profile_ids(
    session: &mut Session,
    args: &UpdateStatusByInstitutionProfileIds,
) -> crate::Result<StatusUpdateResponse> {
    session.put("presence/status/byProfiles", args).await
}

// ===========================================================================
// Schedule and templates (AC #2)
// ===========================================================================

/// Get presence schedules for children within a date range.
///
/// Maps to `PresenceWebService.GetPresenceSchedules()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/schedules`
pub async fn get_presence_schedules(
    session: &mut Session,
    args: &PresenceSchedulesRequest,
) -> crate::Result<Vec<serde_json::Value>> {
    let mut query = Vec::new();
    if let Some(ref ids) = args.filter_institution_profile_ids {
        for id in ids {
            query.push(format!("filterInstitutionProfileIds={id}"));
        }
    }
    if let Some(ref from) = args.from_date {
        query.push(format!("fromDate={from}"));
    }
    if let Some(ref to) = args.to_date {
        query.push(format!("toDate={to}"));
    }
    let path = if query.is_empty() {
        "presence/schedules".to_string()
    } else {
        format!("presence/schedules?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence week overview (employee view).
///
/// Maps to `PresenceWebService.GetPresenceWeekOverview()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/weekOverview`
pub async fn get_presence_week_overview(
    session: &mut Session,
    args: &ComeGoGetWeekOverviewRequest,
) -> crate::Result<GetPresenceOverview> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref ids) = args.group_ids {
        for id in ids {
            query.push(format!("groupIds={id}"));
        }
    }
    if let Some(ref filters) = args.status_filters {
        for f in filters {
            query.push(format!("statusFilters={f}"));
        }
    }
    if let Some(ref start) = args.start_date {
        query.push(format!("startDate={start}"));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={end}"));
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!("presence/weekOverview?{}", query.join("&"));
    session.get(&path).await
}

/// Update presence for a single day (template editing).
///
/// Maps to `PresenceWebService.UpdateOneDayPresence()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/oneDay`
pub async fn update_one_day_presence(
    session: &mut Session,
    args: &UpdatePresenceDayRequest,
) -> crate::Result<TemplateMutationResponse> {
    session.put("presence/oneDay", args).await
}

/// Get the presence template for a specific date.
///
/// Maps to `PresenceWebService.GetTemplateForDate()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/templates/{date}`
pub async fn get_template_for_date(
    session: &mut Session,
    date: &str,
    institution_profile_id: i64,
) -> crate::Result<GetDayTemplateResult> {
    session
        .get(&format!(
            "presence/templates/{date}?institutionProfileId={institution_profile_id}"
        ))
        .await
}

/// Delete a repeated presence template.
///
/// Maps to `PresenceWebService.DeleteRepeatedPresenceTemplate()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /presence/templates/repeated/{template_id}`
pub async fn delete_repeated_presence_template(
    session: &mut Session,
    args: &DeletePresenceTemplateRequest,
) -> crate::Result<TemplateMutationResponse> {
    let template_id = args.present_template_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(ref day) = args.delete_from_day {
        query.push(format!("deleteFromDay={day}"));
    }
    let path = if query.is_empty() {
        format!("presence/templates/repeated/{template_id}")
    } else {
        format!(
            "presence/templates/repeated/{template_id}?{}",
            query.join("&")
        )
    };
    session.delete(&path).await
}

/// Get overlapping presence templates.
///
/// Maps to `PresenceWebService.GetOverlappingPresenceTemplates()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/templates/overlapping`
pub async fn get_overlapping_presence_templates(
    session: &mut Session,
    args: &GetOverlappingPresenceTemplatesRequest,
) -> crate::Result<Vec<serde_json::Value>> {
    let mut query = Vec::new();
    query.push(format!(
        "institutionProfileId={}",
        args.institution_profile_id
    ));
    if let Some(ref start) = args.start_date {
        query.push(format!("startDate={start}"));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={end}"));
    }
    if let Some(ref pattern) = args.repeat_pattern {
        query.push(format!("repeatPattern={pattern:?}"));
    }
    let path = format!("presence/templates/overlapping?{}", query.join("&"));
    session.get(&path).await
}

// ===========================================================================
// Pickup management (AC #3)
// ===========================================================================

/// Get pickup suggestions (exit-with suggestions).
///
/// Maps to `PresenceWebService.GetSuggestionsForPickUp()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/pickup/suggestions`
pub async fn get_suggestions_for_pickup(
    session: &mut Session,
    args: &ComeGoExitWithSuggestionRequest,
) -> crate::Result<GetExitWithSuggestionsResult> {
    let mut query = Vec::new();
    if let Some(ref name) = args.pickup_name {
        query.push(format!("pickupName={name}"));
    }
    if let Some(ref ids) = args.uni_student_ids {
        for id in ids {
            query.push(format!("uniStudentIds={id}"));
        }
    }
    let path = if query.is_empty() {
        "presence/pickup/suggestions".to_string()
    } else {
        format!("presence/pickup/suggestions?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Update pickup suggestions (save pickup name).
///
/// Maps to `PresenceWebService.UpdateSuggestionsForPickup()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/pickup/suggestions`
pub async fn update_suggestions_for_pickup(
    session: &mut Session,
    args: &SavePickupNameRequest,
) -> crate::Result<UpdatePickUpResponsibleResult> {
    session.put("presence/pickup/suggestions", args).await
}

/// Get pickup responsibles for children.
///
/// Maps to `PresenceWebService.GetPickupResponsibles()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/pickup/responsibles`
pub async fn get_pickup_responsibles(
    session: &mut Session,
    args: &GetPickupResponsibleRequest,
) -> crate::Result<GetPickupResponsibleResult> {
    let mut query = Vec::new();
    if let Some(ref ids) = args.uni_student_ids {
        for id in ids {
            query.push(format!("uniStudentIds={id}"));
        }
    }
    let path = if query.is_empty() {
        "presence/pickup/responsibles".to_string()
    } else {
        format!("presence/pickup/responsibles?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Delete a pickup responsible entry.
///
/// Maps to `PresenceWebService.DeletePickupResponsible()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /presence/pickup/responsibles/{suggestion_id}`
pub async fn delete_pickup_responsible(
    session: &mut Session,
    args: &DeletePickupResponsibleRequest,
) -> crate::Result<PickupResponsibleDeleteResponse> {
    session
        .delete(&format!(
            "presence/pickup/responsibles/{}",
            args.presence_pickup_suggestion_id
        ))
        .await
}

/// Get children that a child can go home with.
///
/// Maps to `PresenceWebService.GetChildGoHomeWith()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/children/{child_id}/goHomeWith`
pub async fn get_child_go_home_with(
    session: &mut Session,
    child_id: i64,
) -> crate::Result<GetChildGoHomeWithResult> {
    session
        .get(&format!("presence/children/{child_id}/goHomeWith"))
        .await
}

// ===========================================================================
// Sleep intervals and activity (AC #4)
// ===========================================================================

/// Add sleep intervals for children.
///
/// Maps to `PresenceWebService.AddSleepIntervals()`.
///
/// # Endpoint (inferred)
///
/// `POST /presence/sleep`
pub async fn add_sleep_intervals(
    session: &mut Session,
    args: &AddSleepIntervalsRequest,
) -> crate::Result<SleepMutationResponse> {
    session.post("presence/sleep", args).await
}

/// Update a sleep interval.
///
/// Maps to `PresenceWebService.UpdateSleepInterval()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/sleep/{interval_id}`
pub async fn update_sleep_interval(
    session: &mut Session,
    args: &UpdateSleepIntervalsDto,
) -> crate::Result<SleepMutationResponse> {
    let interval_id = args.id;
    session
        .put(&format!("presence/sleep/{interval_id}"), args)
        .await
}

/// Delete sleep intervals.
///
/// Maps to `PresenceWebService.DeleteSleepIntervals()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /presence/sleep`
///
/// NOTE: DELETE with body is unusual; the API may accept IDs as query
/// parameters. Using `delete_with_body` to match the decompiled signature.
pub async fn delete_sleep_intervals(
    session: &mut Session,
    sleep_interval_ids: &[i64],
) -> crate::Result<SleepMutationResponse> {
    let mut query = Vec::new();
    for id in sleep_interval_ids {
        query.push(format!("sleepIntervalIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/sleep".to_string()
    } else {
        format!("presence/sleep?{}", query.join("&"))
    };
    session.delete(&path).await
}

/// Get the activity list for a department.
///
/// Maps to `PresenceWebService.GetActivityList()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/activities`
pub async fn get_activity_list(
    session: &mut Session,
    args: &ActivityListRequest,
) -> crate::Result<ActivityListResult> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref ids) = args.group_ids {
        for id in ids {
            query.push(format!("groupIds={id}"));
        }
    }
    if let Some(limit) = args.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(offset) = args.offset {
        query.push(format!("offset={offset}"));
    }
    if let Some(ref states) = args.states {
        for s in states {
            query.push(format!("states={s:?}"));
        }
    }
    if let Some(ref next) = args.next_activity {
        query.push(format!("nextActivity={next:?}"));
    }
    if let Some(ref ids) = args.location_ids {
        for id in ids {
            query.push(format!("locationIds={id}"));
        }
    }
    if let Some(ref sort) = args.sort_on {
        query.push(format!("sortOn={sort}"));
    }
    let path = format!("presence/activities?{}", query.join("&"));
    session.get(&path).await
}

/// Get activity filter options for a department.
///
/// Maps to `PresenceWebService.GetActivityFilter()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/activities/filter`
pub async fn get_activity_filter(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<ActivityFilterResult> {
    session
        .get(&format!(
            "presence/activities/filter?institutionCode={institution_code}"
        ))
        .await
}

/// Get daily presence overview (parent view).
///
/// Maps to `PresenceWebService.GetDailyOverview()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/daily/overview`
pub async fn get_daily_overview(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ParentsDailyOverviewResult>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/daily/overview".to_string()
    } else {
        format!("presence/daily/overview?{}", query.join("&"))
    };
    session.get(&path).await
}

// ===========================================================================
// Location and vacation management (AC #5)
// ===========================================================================

/// Get available locations for presence tracking.
///
/// Maps to `PresenceWebService.GetAvailableLocations()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/locations`
pub async fn get_available_locations(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<Vec<PresenceLocation>> {
    session
        .get(&format!(
            "presence/locations?institutionCode={institution_code}"
        ))
        .await
}

/// Update location for children.
///
/// Maps to `PresenceWebService.UpdateLocation()`.
///
/// # Endpoint (inferred)
///
/// `PUT /presence/location`
pub async fn update_location(
    session: &mut Session,
    args: &UpdateLocationRequest,
) -> crate::Result<LocationUpdateResponse> {
    session.put("presence/location", args).await
}

/// Add a vacation entry for children.
///
/// Maps to `PresenceWebService.AddVacation()`.
///
/// # Endpoint (inferred)
///
/// `POST /presence/vacation`
pub async fn add_vacation(
    session: &mut Session,
    args: &VacationEntry,
) -> crate::Result<VacationMutationResponse> {
    session.post("presence/vacation", args).await
}

/// Get children vacation overview (employee view).
///
/// Maps to `PresenceWebService.GetChildrenVacation()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/children/vacation`
pub async fn get_children_vacation(
    session: &mut Session,
    args: &ChildrenVacationRequest,
) -> crate::Result<ChildrenVacationResult> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref ids) = args.group_ids {
        for id in ids {
            query.push(format!("groupIds={id}"));
        }
    }
    if let Some(ref date) = args.date {
        query.push(format!("date={date}"));
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!("presence/children/vacation?{}", query.join("&"));
    session.get(&path).await
}

/// Get vacation announcements grouped by children.
///
/// Maps to `PresenceWebService.GetVacationAnnouncementsByChildren()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/vacation/announcements`
pub async fn get_vacation_announcements_by_children(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<VacationAnnouncementsByChildren>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/vacation/announcements".to_string()
    } else {
        format!("presence/vacation/announcements?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get vacation registration overview (employee view).
///
/// Maps to `PresenceWebService.GetVacationRegistrationOverview()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/vacation/registrations/overview`
pub async fn get_vacation_registration_overview(
    session: &mut Session,
    args: &ComeGoGetVacationRegistrationOverviewRequest,
) -> crate::Result<GetVacationRegistrationOverview> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref groups) = args.filter_groups {
        for g in groups {
            query.push(format!("filterGroups={g}"));
        }
    }
    if let Some(ref filters) = args.status_filters {
        for f in filters {
            query.push(format!("statusFilters={f}"));
        }
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!(
        "presence/vacation/registrations/overview?{}",
        query.join("&")
    );
    session.get(&path).await
}

/// Get vacation registrations grouped by children.
///
/// Maps to `PresenceWebService.GetVacationRegistrationsByChildren()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/vacation/registrations/children`
pub async fn get_vacation_registrations_by_children(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<VacationRegistrationsByChildren>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/vacation/registrations/children".to_string()
    } else {
        format!(
            "presence/vacation/registrations/children?{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get existing vacation registration response for a child.
///
/// Maps to `PresenceWebService.GetExistingVacationRegistrationResponse()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/vacation/registrations/existing`
pub async fn get_existing_vacation_registration_response(
    session: &mut Session,
    child_id: i64,
    vacation_registration_id: i64,
) -> crate::Result<VacationRegistrationResponseForGuardian> {
    session
        .get(&format!(
            "presence/vacation/registrations/existing?childId={child_id}&vacationRegistrationId={vacation_registration_id}"
        ))
        .await
}

// ===========================================================================
// Configuration (AC #6)
// ===========================================================================

/// Get presence configuration.
///
/// Maps to `PresenceWebService.GetPresenceConfiguration()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/configuration`
pub async fn get_presence_configuration(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<PresenceConfigurationResult> {
    session
        .get(&format!(
            "presence/configuration?institutionCode={institution_code}"
        ))
        .await
}

/// Get presence configuration by children IDs.
///
/// Maps to `PresenceWebService.GetPresenceConfigurationByChildrenIds()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/configuration/children`
pub async fn get_presence_configuration_by_children_ids(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<PresenceConfigurationChildResult>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "presence/configuration/children".to_string()
    } else {
        format!("presence/configuration/children?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get a single presence filter.
///
/// Maps to `PresenceWebService.GetPresenceFilter()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/filter`
pub async fn get_presence_filter(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<PresenceFilterResult> {
    session
        .get(&format!(
            "presence/filter?institutionCode={institution_code}"
        ))
        .await
}

/// Get multiple presence filters.
///
/// Maps to `PresenceWebService.GetPresenceFilters()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/filters`
pub async fn get_presence_filters(
    session: &mut Session,
    args: &PresenceFiltersRequest,
) -> crate::Result<Vec<PresenceFilterResult>> {
    let mut query = Vec::new();
    if let Some(ref institutions) = args.institutions {
        for inst in institutions {
            query.push(format!("institutions={inst}"));
        }
    }
    let path = if query.is_empty() {
        "presence/filters".to_string()
    } else {
        format!("presence/filters?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get closed days.
///
/// Maps to `PresenceWebService.GetClosedDays()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/closedDays`
pub async fn get_closed_days(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetClosedDaysResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={code}"));
    }
    let path = if query.is_empty() {
        "presence/closedDays".to_string()
    } else {
        format!("presence/closedDays?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get general opening hours.
///
/// Maps to `PresenceWebService.GetGeneralOpeningHours()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/openingHours`
pub async fn get_general_opening_hours(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetGeneralOpeningHoursResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={code}"));
    }
    let path = if query.is_empty() {
        "presence/openingHours".to_string()
    } else {
        format!("presence/openingHours?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get opening hours by institution codes within a date range.
///
/// Maps to `PresenceWebService.GetOpeningHoursByInstitutionCodes()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/openingHours/institution`
pub async fn get_opening_hours_by_institution_codes(
    session: &mut Session,
    args: &GetOpeningHoursByInstitutionCodesRequest,
) -> crate::Result<GetOpeningHoursByInstitutionCodesResult> {
    let mut query = Vec::new();
    if let Some(ref codes) = args.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={code}"));
        }
    }
    if let Some(ref start) = args.start_date {
        query.push(format!("startDate={start}"));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={end}"));
    }
    let path = if query.is_empty() {
        "presence/openingHours/institution".to_string()
    } else {
        format!("presence/openingHours/institution?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get specific opening hour overview.
///
/// Maps to `PresenceWebService.GetSpecificOpeningHourOverview()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/openingHours/specific`
pub async fn get_specific_opening_hour_overview(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetSpecificOpeningHourOverviewResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={code}"));
    }
    let path = if query.is_empty() {
        "presence/openingHours/specific".to_string()
    } else {
        format!("presence/openingHours/specific?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get available presence statuses.
///
/// Maps to `PresenceWebService.GetAvailablePresenceStatuses()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/statuses`
pub async fn get_available_presence_statuses(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<GetAvailableStatusesResult> {
    session
        .get(&format!(
            "presence/statuses?institutionCode={institution_code}"
        ))
        .await
}

/// Get institutions with presence states.
///
/// Maps to `PresenceWebService.GetInstitutionWithPresenceStates()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/institution/states`
pub async fn get_institution_with_presence_states(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<Vec<InstitutionWithPresenceStates>> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={code}"));
    }
    let path = if query.is_empty() {
        "presence/institution/states".to_string()
    } else {
        format!("presence/institution/states?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence children distribution.
///
/// Maps to `PresenceWebService.GetPresenceChildrenDistribution()`.
///
/// # Endpoint (inferred)
///
/// `GET /presence/children/distribution`
pub async fn get_presence_children_distribution(
    session: &mut Session,
    args: &PresenceChildrenDistributionRequestDto,
) -> crate::Result<PresenceChildrenDistribution> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref date) = args.date {
        query.push(format!("date={date}"));
    }
    if let Some(ref ids) = args.group_ids {
        for id in ids {
            query.push(format!("groupIds={id}"));
        }
    }
    if let Some(ref filters) = args.status_filters {
        for f in filters {
            query.push(format!("statusFilters={f}"));
        }
    }
    let path = format!("presence/children/distribution?{}", query.join("&"));
    session.get(&path).await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::presence::*;

    #[test]
    fn bulk_update_status_request_serializes() {
        let req = BulkUpdatePresenceStatusRequest {
            presence_registration_ids: Some(vec![1, 2, 3]),
            status: Some(crate::enums::presence::PresenceStatusEnum::Present),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json["presenceRegistrationIds"],
            serde_json::json!([1, 2, 3])
        );
        assert_eq!(json["status"], "Present");
    }

    #[test]
    fn update_presence_registration_request_serializes() {
        let req = UpdatePresenceRegistrationRequest {
            registration_id: 42,
            checkout_type: Some(crate::enums::presence::ActivityTypeEnum::PickedUpBy),
            pickup_by: Some(UpdateCheckoutPickedUpActivity {
                exit_time: Some("15:00".to_string()),
                exit_with: Some("Mor".to_string()),
            }),
            self_decider: None,
            send_home: None,
            go_home_with: None,
            entry_time: Some("07:30".to_string()),
            remark: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["registrationId"], 42);
        assert_eq!(json["checkoutType"], "PICKED_UP_BY");
        assert_eq!(json["pickupBy"]["exitWith"], "Mor");
    }

    #[test]
    fn update_presence_day_request_serializes() {
        let req = UpdatePresenceDayRequest {
            institution_profile_id: 100,
            id: Some(5),
            day_of_week: 1,
            by_date: Some("2026-03-18".to_string()),
            comment: Some("Test".to_string()),
            spare_time_activity: None,
            presence_activity: None,
            repeat_pattern: None,
            expires_at: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["institutionProfileId"], 100);
        assert_eq!(json["dayOfWeek"], 1);
        assert_eq!(json["byDate"], "2026-03-18");
    }

    #[test]
    fn add_sleep_intervals_request_serializes() {
        let req = AddSleepIntervalsRequest {
            child_ids: Some(vec![10, 20]),
            start: Some("12:00".to_string()),
            end: Some("13:00".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["childIds"], serde_json::json!([10, 20]));
        assert_eq!(json["start"], "12:00");
    }

    #[test]
    fn update_sleep_interval_dto_serializes() {
        let req = UpdateSleepIntervalsDto {
            presence_registration_id: 42,
            id: 5,
            start: Some("12:30".to_string()),
            end: Some("13:30".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["presenceRegistrationId"], 42);
        assert_eq!(json["id"], 5);
    }

    #[test]
    fn activity_list_request_serializes() {
        let req = ActivityListRequest {
            department_id: 10,
            group_ids: Some(vec![1, 2]),
            limit: Some(50),
            offset: Some(0),
            states: None,
            next_activity: None,
            location_ids: None,
            sort_on: Some("name".to_string()),
            daily_note: None,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["departmentId"], 10);
        assert_eq!(json["groupIds"], serde_json::json!([1, 2]));
        assert_eq!(json["sortOn"], "name");
    }

    #[test]
    fn update_location_request_serializes() {
        let req = UpdateLocationRequest {
            child_ids: Some(vec![1, 2, 3]),
            location_id: Some(42),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["childIds"], serde_json::json!([1, 2, 3]));
        assert_eq!(json["locationId"], 42);
    }

    #[test]
    fn vacation_entry_serializes() {
        let req = VacationEntry {
            child_ids: Some(vec![10]),
            intervals: Some(vec![VacationIntervals {
                start_date: Some("2026-07-01".to_string()),
                end_date: Some("2026-07-14".to_string()),
            }]),
            comment: Some("Sommerferie".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["childIds"], serde_json::json!([10]));
        let intervals = json["intervals"].as_array().unwrap();
        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0]["startDate"], "2026-07-01");
    }

    #[test]
    fn children_vacation_request_serializes() {
        let req = ChildrenVacationRequest {
            department_id: 5,
            group_ids: Some(vec![1]),
            date: Some("2026-07-01".to_string()),
            offset: 0,
            limit: 25,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["departmentId"], 5);
        assert_eq!(json["limit"], 25);
    }

    #[test]
    fn presence_schedules_request_serializes() {
        let req = PresenceSchedulesRequest {
            filter_institution_profile_ids: Some(vec![100, 200]),
            from_date: Some("2026-03-01".to_string()),
            to_date: Some("2026-03-31".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(
            json["filterInstitutionProfileIds"],
            serde_json::json!([100, 200])
        );
    }

    #[test]
    fn get_opening_hours_request_serializes() {
        let req = GetOpeningHoursByInstitutionCodesRequest {
            institution_codes: Some(vec!["101001".to_string()]),
            start_date: Some("2026-03-01".to_string()),
            end_date: Some("2026-03-31".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["institutionCodes"], serde_json::json!(["101001"]));
    }

    #[test]
    fn delete_presence_template_request_serializes() {
        let req = DeletePresenceTemplateRequest {
            delete_from_day: Some("2026-03-18".to_string()),
            present_template_id: Some(42),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["presentTemplateId"], 42);
        assert_eq!(json["deleteFromDay"], "2026-03-18");
    }

    #[test]
    fn save_pickup_name_request_serializes() {
        let req = SavePickupNameRequest {
            id: 99,
            name: Some("Farmor".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["id"], 99);
        assert_eq!(json["name"], "Farmor");
    }

    #[test]
    fn presence_children_distribution_request_serializes() {
        let req = PresenceChildrenDistributionRequestDto {
            department_id: 10,
            date: Some("2026-03-18".to_string()),
            group_ids: Some(vec![1, 2]),
            status_filters: Some(vec!["Present".to_string()]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["departmentId"], 10);
        assert_eq!(json["statusFilters"], serde_json::json!(["Present"]));
    }

    #[test]
    fn week_overview_request_serializes() {
        let req = ComeGoGetWeekOverviewRequest {
            department_id: 5,
            group_ids: Some(vec![1]),
            status_filters: None,
            start_date: Some("2026-03-16".to_string()),
            end_date: Some("2026-03-22".to_string()),
            offset: 0,
            limit: 50,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["departmentId"], 5);
        assert_eq!(json["startDate"], "2026-03-16");
    }

    #[test]
    fn vacation_registration_overview_request_serializes() {
        let req = ComeGoGetVacationRegistrationOverviewRequest {
            department_id: 10,
            filter_groups: Some(vec![1, 2]),
            status_filters: None,
            offset: 0,
            limit: 25,
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["departmentId"], 10);
        assert_eq!(json["filterGroups"], serde_json::json!([1, 2]));
    }
}
