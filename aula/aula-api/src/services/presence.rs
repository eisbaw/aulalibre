//! Presence (ComeGo) service.
//!
//! Maps to `AulaNative.Services.Web.PresenceWebService` (40+ methods) from the APK.
//!
//! # Endpoint paths
//!
//! All endpoints use RPC-style URLs from `Urls.cs` in the decompiled assembly.
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_PRESENCE_STATES` | `presence.getPresenceStates` |
//! | `GET_PRESENCE_REGISTRATIONS` | `presence.getPresenceRegistrations` |
//! | `GET_PRESENCE_REGISTRATION_BY_IDS` | `presence.getPresenceRegistrationsByIds` |
//! | `GET_PRESENCE_DETAIL` | `presence.getPresenceRegistrationDetail` |
//! | `UPDATE_PRESENCE_REGISTRATION` | `presence.updatePresenceRegistration` |
//! | `UPDATE_STATUSES_BY_PRESENCE_REGISTRATION_IDS` | `presence.bulkUpdatePresenceStatus` |
//! | `UPDATE_STATUS_BY_PROFILE_ID` | `presence.updateStatusByInstitutionProfileIds` |
//! | `GET_PRESENCE_TEMPLATES` | `presence.getPresenceTemplates` |
//! | `GET_WEEK_OVERVIEW` | `presence.getActivityOverview` |
//! | `UPDATE_PRESENCE_TEMPLATES` | `presence.updatePresenceTemplate` |
//! | `GET_PRESENCE_REGISTRATION_FOR_TODAY` | `presence.getTemplateForDate` |
//! | `DELETE_REPEATED_PRESENCE_TEMPLATE` | `presence.deleteRepeatingPresenceTemplate` |
//! | `GET_OVERLAPPING_PRESENCE_TEMPLATES` | `presence.getOverlappingPresenceTemplates` |
//! | `GET_SUGGESTIONS_PICK_UP` | `presence.getSuggestedNamesForPickupChild` |
//! | `UPDATE_SUGGESTIONS_PICK_UP` | `presence.savePickupNames` |
//! | `GET_PICK_UP_RESPONSIBLES` | `presence.getPickupResponsibles` |
//! | `DELETE_PICKUP_RESPONSIBLE` | `presence.deletePickupResponsible` |
//! | `GET_CHILD_GO_HOME_WITH` | `presence.getGoHomeWithList` |
//! | `ADD_SLEEP_INTERVALS` | `presence.addSleepIntervals` |
//! | `UPDATE_SLEEP_INTERVALS` | `presence.updateSleepInterval` |
//! | `DELETE_SLEEP_INTERVALS` | `presence.deleteSleepIntervals` |
//! | `GET_ACTIVITY_LIST` | `presence.getActivityList` |
//! | `GET_ACTIVITY_LIST_EDIT_OPTIONS` | `presence.getActivityListEditOptions` |
//! | `GET_DAILY_OVERVIEW` | `presence.getDailyOverview` |
//! | `GET_AVAILABLE_LOCATIONS` | `presence.getAvailablePresenceLocations` |
//! | `UPDATE_LOCATION` | `presence.updateLocation` |
//! | `UPDATE_CHECKOUT_ACTIVITY` | `presence.updateCheckoutActivity` |
//! | `ADD_VACATION` | `calendar.addVacation` |
//! | `GET_CHILDREN_VACATION_LIST` | `presence.getChildVacationList` |
//! | `GET_VACATION_ANNOUNCEMENTS_BY_CHILDREN` | `presence.getVacationAnnouncementsByChildren` |
//! | `GET_VACATION_REGISTRATION_OVERVIEW` | `presence.getVacationRegistrations` |
//! | `GET_VACATION_REGISTRATIONS_BY_CHILDREN` | `presence.getVacationRegistrationsByChildren` |
//! | `GET_EXISTING_VACATION_REGISTRATION_RESPONSE` | `presence.getVacationRegistrationResponse` |
//! | `GET_PRESENCE_CONFIGURATION` | `presence.getPresenceConfiguration` |
//! | `GET_PRESENCE_CONFIGURATION_BY_CHILD_IDS` | `presence.getPresenceConfigurationByChildIds` |
//! | `GET_PRESENCE_FILTER` | `presence.getPresenceFilters` |
//! | `GET_CHILDREN_VACATION_LIST` | `presence.getChildVacationList` |
//! | `GET_CLOSED_DAYS_OVERVIEW` | `presence.getClosedDays` |
//! | `GET_GENERAL_OPENING_HOURS` | `presence.getGeneralOpeningHours` |
//! | `GET_OPENING_HOURS_BY_INSTITUTION_CODES` | `presence.getOpeningHoursByInstitutionCodes` |
//! | `GET_SPECIFIC_OPENING_HOUR_OVERVIEW` | `presence.getSpecificOpeningHourOverview` |
//! | `GET_AVAILABLE_STATUSES` | `presence.getPresenceStates` |
//! | `GET_NUMBER_CHILDREN_PRESENCE` | `presence.getPresenceDistribution` |

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
use crate::services::query::encode_value;
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
/// # Endpoint
///
/// `GET ?method=presence.getPresenceStates`
pub async fn get_childrens_state(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ChildStatus>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceStates".to_string()
    } else {
        format!("?method=presence.getPresenceStates&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence registrations (list).
///
/// Maps to `PresenceWebService.GetPresenceRegistrations()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceRegistrations`
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
        query.push(format!("date={}", encode_value(d)));
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceRegistrations".to_string()
    } else {
        format!(
            "?method=presence.getPresenceRegistrations&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get presence registration detail by ID.
///
/// Maps to `PresenceWebService.GetPresenceRegistrationDetail()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceRegistrationDetail`
pub async fn get_presence_registration_detail(
    session: &mut Session,
    registration_id: i64,
) -> crate::Result<PresenceRegistrationResult> {
    session
        .get(&format!(
            "?method=presence.getPresenceRegistrationsByIds&registrationId={registration_id}"
        ))
        .await
}

/// Update a presence registration (checkout details).
///
/// Maps to `PresenceWebService.UpdatePresenceRegistration()`.
///
/// # Endpoint
///
/// `POST ?method=presence.updatePresenceRegistration`
pub async fn update_presence_registration(
    session: &mut Session,
    args: &UpdatePresenceRegistrationRequest,
) -> crate::Result<PresenceRegistrationMutationResponse> {
    let _registration_id = args.registration_id;
    session
        .post("?method=presence.updatePresenceRegistration", args)
        .await
}

/// Bulk update presence status by registration IDs.
///
/// Maps to `PresenceWebService.UpdateStatusByPresenceRegistrationIds()`.
///
/// # Endpoint
///
/// `POST ?method=presence.bulkUpdatePresenceStatus`
pub async fn update_status_by_presence_registration_ids(
    session: &mut Session,
    args: &BulkUpdatePresenceStatusRequest,
) -> crate::Result<StatusUpdateResponse> {
    session
        .post("?method=presence.bulkUpdatePresenceStatus", args)
        .await
}

/// Update presence status by institution profile IDs.
///
/// Maps to `PresenceWebService.UpdateStatusByInstitutionProfileIds()`.
///
/// # Endpoint
///
/// `POST ?method=presence.updateStatusByInstitutionProfileIds`
pub async fn update_status_by_institution_profile_ids(
    session: &mut Session,
    args: &UpdateStatusByInstitutionProfileIds,
) -> crate::Result<StatusUpdateResponse> {
    session
        .post("?method=presence.updateStatusByInstitutionProfileIds", args)
        .await
}

// ===========================================================================
// Schedule and templates (AC #2)
// ===========================================================================

/// Get presence schedules for children within a date range.
///
/// Maps to `PresenceWebService.GetPresenceSchedules()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceTemplates`
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
        query.push(format!("fromDate={}", encode_value(from)));
    }
    if let Some(ref to) = args.to_date {
        query.push(format!("toDate={}", encode_value(to)));
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceTemplates".to_string()
    } else {
        format!("?method=presence.getPresenceTemplates&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence week overview (employee view).
///
/// Maps to `PresenceWebService.GetPresenceWeekOverview()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getActivityOverview`
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
            query.push(format!("statusFilters={}", encode_value(f)));
        }
    }
    if let Some(ref start) = args.start_date {
        query.push(format!("startDate={}", encode_value(start)));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={}", encode_value(end)));
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!("?method=presence.getActivityOverview&{}", query.join("&"));
    session.get(&path).await
}

/// Update presence for a single day (template editing).
///
/// Maps to `PresenceWebService.UpdateOneDayPresence()`.
///
/// # Endpoint
///
/// `POST ?method=presence.updatePresenceTemplate`
pub async fn update_one_day_presence(
    session: &mut Session,
    args: &UpdatePresenceDayRequest,
) -> crate::Result<TemplateMutationResponse> {
    session
        .post("?method=presence.updatePresenceTemplate", args)
        .await
}

/// Get the presence template for a specific date.
///
/// Maps to `PresenceWebService.GetTemplateForDate()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getTemplateForDate`
pub async fn get_template_for_date(
    session: &mut Session,
    date: &str,
    institution_profile_id: i64,
) -> crate::Result<GetDayTemplateResult> {
    session
        .get(&format!(
            "?method=presence.getTemplateForDate&date={date}&institutionProfileId={institution_profile_id}"
        ))
        .await
}

/// Delete a repeated presence template.
///
/// Maps to `PresenceWebService.DeleteRepeatedPresenceTemplate()`.
///
/// # Endpoint
///
/// `POST ?method=presence.deleteRepeatingPresenceTemplate`
pub async fn delete_repeated_presence_template(
    session: &mut Session,
    args: &DeletePresenceTemplateRequest,
) -> crate::Result<TemplateMutationResponse> {
    let template_id = args.present_template_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(ref day) = args.delete_from_day {
        query.push(format!("deleteFromDay={}", encode_value(day)));
    }
    let path = if query.is_empty() {
        format!("?method=presence.deleteRepeatingPresenceTemplate&templateId={template_id}")
    } else {
        format!(
            "?method=presence.deleteRepeatingPresenceTemplate&templateId={template_id}&{}",
            query.join("&")
        )
    };
    session.post_empty(&path).await
}

/// Get overlapping presence templates.
///
/// Maps to `PresenceWebService.GetOverlappingPresenceTemplates()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getOverlappingPresenceTemplates`
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
        query.push(format!("startDate={}", encode_value(start)));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={}", encode_value(end)));
    }
    if let Some(ref pattern) = args.repeat_pattern {
        query.push(format!("repeatPattern={pattern:?}"));
    }
    let path = format!(
        "?method=presence.getOverlappingPresenceTemplates&{}",
        query.join("&")
    );
    session.get(&path).await
}

// ===========================================================================
// Pickup management (AC #3)
// ===========================================================================

/// Get pickup suggestions (exit-with suggestions).
///
/// Maps to `PresenceWebService.GetSuggestionsForPickUp()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getSuggestedNamesForPickupChild`
pub async fn get_suggestions_for_pickup(
    session: &mut Session,
    args: &ComeGoExitWithSuggestionRequest,
) -> crate::Result<GetExitWithSuggestionsResult> {
    let mut query = Vec::new();
    if let Some(ref name) = args.pickup_name {
        query.push(format!("pickupName={}", encode_value(name)));
    }
    if let Some(ref ids) = args.uni_student_ids {
        for id in ids {
            query.push(format!("uniStudentIds={id}"));
        }
    }
    let path = if query.is_empty() {
        "?method=presence.getSuggestedNamesForPickupChild".to_string()
    } else {
        format!(
            "?method=presence.getSuggestedNamesForPickupChild&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Update pickup suggestions (save pickup name).
///
/// Maps to `PresenceWebService.UpdateSuggestionsForPickup()`.
///
/// # Endpoint
///
/// `POST ?method=presence.savePickupNames`
pub async fn update_suggestions_for_pickup(
    session: &mut Session,
    args: &SavePickupNameRequest,
) -> crate::Result<UpdatePickUpResponsibleResult> {
    session.post("?method=presence.savePickupNames", args).await
}

/// Get pickup responsibles for children.
///
/// Maps to `PresenceWebService.GetPickupResponsibles()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPickupResponsibles`
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
        "?method=presence.getPickupResponsibles".to_string()
    } else {
        format!("?method=presence.getPickupResponsibles&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Delete a pickup responsible entry.
///
/// Maps to `PresenceWebService.DeletePickupResponsible()`.
///
/// # Endpoint
///
/// `POST ?method=presence.deletePickupResponsible`
pub async fn delete_pickup_responsible(
    session: &mut Session,
    args: &DeletePickupResponsibleRequest,
) -> crate::Result<PickupResponsibleDeleteResponse> {
    session
        .post(
            "?method=presence.deletePickupResponsible",
            &serde_json::json!({"presencePickupSuggestionId": args.presence_pickup_suggestion_id}),
        )
        .await
}

/// Get children that a child can go home with.
///
/// Maps to `PresenceWebService.GetChildGoHomeWith()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getGoHomeWithList`
pub async fn get_child_go_home_with(
    session: &mut Session,
    child_id: i64,
) -> crate::Result<GetChildGoHomeWithResult> {
    session
        .get(&format!(
            "?method=presence.getGoHomeWithList&childId={child_id}"
        ))
        .await
}

// ===========================================================================
// Sleep intervals and activity (AC #4)
// ===========================================================================

/// Add sleep intervals for children.
///
/// Maps to `PresenceWebService.AddSleepIntervals()`.
///
/// # Endpoint
///
/// `POST ?method=presence.addSleepIntervals`
pub async fn add_sleep_intervals(
    session: &mut Session,
    args: &AddSleepIntervalsRequest,
) -> crate::Result<SleepMutationResponse> {
    session
        .post("?method=presence.addSleepIntervals", args)
        .await
}

/// Update a sleep interval.
///
/// Maps to `PresenceWebService.UpdateSleepInterval()`.
///
/// # Endpoint
///
/// `POST ?method=presence.updateSleepInterval`
pub async fn update_sleep_interval(
    session: &mut Session,
    args: &UpdateSleepIntervalsDto,
) -> crate::Result<SleepMutationResponse> {
    let _interval_id = args.id;
    session
        .post("?method=presence.updateSleepInterval", args)
        .await
}

/// Delete sleep intervals.
///
/// Maps to `PresenceWebService.DeleteSleepIntervals()`.
///
/// # Endpoint
///
/// `POST ?method=presence.deleteSleepIntervals`
pub async fn delete_sleep_intervals(
    session: &mut Session,
    sleep_interval_ids: &[i64],
) -> crate::Result<SleepMutationResponse> {
    session
        .post(
            "?method=presence.deleteSleepIntervals",
            &serde_json::json!({"sleepIntervalIds": sleep_interval_ids}),
        )
        .await
}

/// Get the activity list for a department.
///
/// Maps to `PresenceWebService.GetActivityList()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getActivityList`
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
        query.push(format!("sortOn={}", encode_value(sort)));
    }
    let path = format!("?method=presence.getActivityList&{}", query.join("&"));
    session.get(&path).await
}

/// Get activity filter options for a department.
///
/// Maps to `PresenceWebService.GetActivityFilter()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getActivityListEditOptions`
pub async fn get_activity_filter(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<ActivityFilterResult> {
    session
        .get(&format!(
            "?method=presence.getActivityListEditOptions&institutionCode={institution_code}"
        ))
        .await
}

/// Get daily presence overview (parent view).
///
/// Maps to `PresenceWebService.GetDailyOverview()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getDailyOverview`
pub async fn get_daily_overview(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ParentsDailyOverviewResult>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "?method=presence.getDailyOverview".to_string()
    } else {
        format!("?method=presence.getDailyOverview&{}", query.join("&"))
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
/// # Endpoint
///
/// `GET ?method=presence.getAvailablePresenceLocations`
pub async fn get_available_locations(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<Vec<PresenceLocation>> {
    session
        .get(&format!(
            "?method=presence.getAvailablePresenceLocations&institutionCode={institution_code}"
        ))
        .await
}

/// Update location for children.
///
/// Maps to `PresenceWebService.UpdateLocation()`.
///
/// # Endpoint
///
/// `POST ?method=presence.updateLocation`
pub async fn update_location(
    session: &mut Session,
    args: &UpdateLocationRequest,
) -> crate::Result<LocationUpdateResponse> {
    session.post("?method=presence.updateLocation", args).await
}

/// Add a vacation entry for children.
///
/// Maps to `PresenceWebService.AddVacation()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.addVacation`
pub async fn add_vacation(
    session: &mut Session,
    args: &VacationEntry,
) -> crate::Result<VacationMutationResponse> {
    session.post("?method=calendar.addVacation", args).await
}

/// Get children vacation overview (employee view).
///
/// Maps to `PresenceWebService.GetChildrenVacation()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getChildVacationList`
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
        query.push(format!("date={}", encode_value(date)));
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!("?method=presence.getChildVacationList&{}", query.join("&"));
    session.get(&path).await
}

/// Get vacation announcements grouped by children.
///
/// Maps to `PresenceWebService.GetVacationAnnouncementsByChildren()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getVacationAnnouncementsByChildren`
pub async fn get_vacation_announcements_by_children(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<VacationAnnouncementsByChildren>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "?method=presence.getVacationAnnouncementsByChildren".to_string()
    } else {
        format!(
            "?method=presence.getVacationAnnouncementsByChildren&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get vacation registration overview (employee view).
///
/// Maps to `PresenceWebService.GetVacationRegistrationOverview()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getVacationRegistrations`
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
            query.push(format!("statusFilters={}", encode_value(f)));
        }
    }
    query.push(format!("offset={}", args.offset));
    query.push(format!("limit={}", args.limit));
    let path = format!(
        "?method=presence.getVacationRegistrations&{}",
        query.join("&")
    );
    session.get(&path).await
}

/// Get vacation registrations grouped by children.
///
/// Maps to `PresenceWebService.GetVacationRegistrationsByChildren()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getVacationRegistrationsByChildren`
pub async fn get_vacation_registrations_by_children(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<VacationRegistrationsByChildren>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "?method=presence.getVacationRegistrationsByChildren".to_string()
    } else {
        format!(
            "?method=presence.getVacationRegistrationsByChildren&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get existing vacation registration response for a child.
///
/// Maps to `PresenceWebService.GetExistingVacationRegistrationResponse()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getVacationRegistrationResponse`
pub async fn get_existing_vacation_registration_response(
    session: &mut Session,
    child_id: i64,
    vacation_registration_id: i64,
) -> crate::Result<VacationRegistrationResponseForGuardian> {
    session
        .get(&format!(
            "?method=presence.getVacationRegistrationResponse&childId={child_id}&vacationRegistrationId={vacation_registration_id}"
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
/// # Endpoint
///
/// `GET ?method=presence.getPresenceConfiguration`
pub async fn get_presence_configuration(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<PresenceConfigurationResult> {
    session
        .get(&format!(
            "?method=presence.getPresenceConfiguration&institutionCode={institution_code}"
        ))
        .await
}

/// Get presence configuration by children IDs.
///
/// Maps to `PresenceWebService.GetPresenceConfigurationByChildrenIds()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceConfigurationByChildIds`
pub async fn get_presence_configuration_by_children_ids(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<PresenceConfigurationChildResult>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceConfigurationByChildIds".to_string()
    } else {
        format!(
            "?method=presence.getPresenceConfigurationByChildIds&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get a single presence filter.
///
/// Maps to `PresenceWebService.GetPresenceFilter()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceFilters`
pub async fn get_presence_filter(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<PresenceFilterResult> {
    session
        .get(&format!(
            "?method=presence.getPresenceFilters&institutionCode={institution_code}"
        ))
        .await
}

/// Get multiple presence filters.
///
/// Maps to `PresenceWebService.GetPresenceFilters()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceFilters` (list)
pub async fn get_presence_filters(
    session: &mut Session,
    args: &PresenceFiltersRequest,
) -> crate::Result<Vec<PresenceFilterResult>> {
    let mut query = Vec::new();
    if let Some(ref institutions) = args.institutions {
        for inst in institutions {
            query.push(format!("institutions={}", encode_value(inst)));
        }
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceFilters".to_string()
    } else {
        format!("?method=presence.getPresenceFilters&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get closed days.
///
/// Maps to `PresenceWebService.GetClosedDays()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getClosedDays`
pub async fn get_closed_days(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetClosedDaysResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={}", encode_value(code)));
    }
    let path = if query.is_empty() {
        "?method=presence.getClosedDays".to_string()
    } else {
        format!("?method=presence.getClosedDays&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get general opening hours.
///
/// Maps to `PresenceWebService.GetGeneralOpeningHours()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getGeneralOpeningHours`
pub async fn get_general_opening_hours(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetGeneralOpeningHoursResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={}", encode_value(code)));
    }
    let path = if query.is_empty() {
        "?method=presence.getGeneralOpeningHours".to_string()
    } else {
        format!(
            "?method=presence.getGeneralOpeningHours&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get opening hours by institution codes within a date range.
///
/// Maps to `PresenceWebService.GetOpeningHoursByInstitutionCodes()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getOpeningHoursByInstitutionCodes`
pub async fn get_opening_hours_by_institution_codes(
    session: &mut Session,
    args: &GetOpeningHoursByInstitutionCodesRequest,
) -> crate::Result<GetOpeningHoursByInstitutionCodesResult> {
    let mut query = Vec::new();
    if let Some(ref codes) = args.institution_codes {
        for code in codes {
            query.push(format!("institutionCodes={}", encode_value(code)));
        }
    }
    if let Some(ref start) = args.start_date {
        query.push(format!("startDate={}", encode_value(start)));
    }
    if let Some(ref end) = args.end_date {
        query.push(format!("endDate={}", encode_value(end)));
    }
    let path = if query.is_empty() {
        "?method=presence.getOpeningHoursByInstitutionCodes".to_string()
    } else {
        format!(
            "?method=presence.getOpeningHoursByInstitutionCodes&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get specific opening hour overview.
///
/// Maps to `PresenceWebService.GetSpecificOpeningHourOverview()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getSpecificOpeningHourOverview`
pub async fn get_specific_opening_hour_overview(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<GetSpecificOpeningHourOverviewResult> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={}", encode_value(code)));
    }
    let path = if query.is_empty() {
        "?method=presence.getSpecificOpeningHourOverview".to_string()
    } else {
        format!(
            "?method=presence.getSpecificOpeningHourOverview&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get available presence statuses.
///
/// Maps to `PresenceWebService.GetAvailablePresenceStatuses()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceStates`
pub async fn get_available_presence_statuses(
    session: &mut Session,
    institution_code: &str,
) -> crate::Result<GetAvailableStatusesResult> {
    session
        .get(&format!(
            "?method=presence.getPresenceStates&institutionCode={institution_code}"
        ))
        .await
}

/// Get institutions with presence states.
///
/// Maps to `PresenceWebService.GetInstitutionWithPresenceStates()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceStates` (institution)
pub async fn get_institution_with_presence_states(
    session: &mut Session,
    institution_codes: &[String],
) -> crate::Result<Vec<InstitutionWithPresenceStates>> {
    let mut query = Vec::new();
    for code in institution_codes {
        query.push(format!("institutionCodes={}", encode_value(code)));
    }
    let path = if query.is_empty() {
        "?method=presence.getPresenceStates".to_string()
    } else {
        format!("?method=presence.getPresenceStates&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get presence children distribution.
///
/// Maps to `PresenceWebService.GetPresenceChildrenDistribution()`.
///
/// # Endpoint
///
/// `GET ?method=presence.getPresenceDistribution`
pub async fn get_presence_children_distribution(
    session: &mut Session,
    args: &PresenceChildrenDistributionRequestDto,
) -> crate::Result<PresenceChildrenDistribution> {
    let mut query = Vec::new();
    query.push(format!("departmentId={}", args.department_id));
    if let Some(ref date) = args.date {
        query.push(format!("date={}", encode_value(date)));
    }
    if let Some(ref ids) = args.group_ids {
        for id in ids {
            query.push(format!("groupIds={id}"));
        }
    }
    if let Some(ref filters) = args.status_filters {
        for f in filters {
            query.push(format!("statusFilters={}", encode_value(f)));
        }
    }
    let path = format!(
        "?method=presence.getPresenceDistribution&{}",
        query.join("&")
    );
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
