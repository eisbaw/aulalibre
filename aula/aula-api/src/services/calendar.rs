//! Calendar service.
//!
//! Maps to `AulaNative.Services.Web.CalendarWebService` (37 methods) from the APK.
//!
//! # Endpoint paths
//!
//! Endpoint paths are **inferred** from method names in the decompiled
//! assembly; they have not been verified against live traffic. See
//! `api_endpoints.md` Section 3.5.
//!
//! ## CalendarWebService (Section 3.5)
//!
//! | Method | HTTP | Path (inferred) |
//! |--------|------|-----------------|
//! | `get_daily_aggregated_events` | GET | `/calendar/events/daily` |
//! | `get_daily_group_event_count` | GET | `/calendar/events/dailyGroupCount` |
//! | `get_events` | GET | `/calendar/events` |
//! | `get_event_detail` | GET | `/calendar/events/{id}` |
//! | `get_event_types` | GET | `/calendar/eventTypes` |
//! | `get_event_types_for_calendar_feed` | GET | `/calendar/eventTypes/feed` |
//! | `get_event_for_group` | GET | `/calendar/events/group/{id}` |
//! | `get_school_events` | GET | `/calendar/schoolEvents` |
//! | `get_top_important_date` | GET | `/calendar/importantDates/top` |
//! | `respond_simple_event` | POST | `/calendar/events/{id}/respond` |
//! | `respond_timeslot_event` | POST | `/calendar/timeslots/{id}/respond` |
//! | `update_lesson_event` | PUT | `/calendar/lessons/{id}` |
//! | `edit_timeslot_event` | PUT | `/calendar/timeslots/{id}` |
//! | `block_time_slot` | POST | `/calendar/timeslots/block` |
//! | `delete_time_slot` | DELETE | `/calendar/timeslots/{id}` |
//! | `delete_event` | DELETE | `/calendar/events/{id}` |
//! | `check_conflict_event_for_attendees` | POST | `/calendar/events/conflicts` |
//! | `get_birthdays_for_group` | GET | `/calendar/birthdays/group/{id}` |
//! | `get_birthdays_for_institution` | GET | `/calendar/birthdays/institution/{id}` |
//! | `add_vacation` | POST | `/calendar/vacations` |
//! | `get_vacation` | GET | `/calendar/vacations/{id}` |
//! | `delete_vacation` | DELETE | `/calendar/vacations/{id}` |
//! | `get_future_vacation_request` | GET | `/calendar/vacations/future` |
//! | `get_vacation_request_response` | GET | `/calendar/vacations/{id}/response` |
//! | `respond_to_vacation_registration_request` | POST | `/calendar/vacations/{id}/respond` |
//! | `get_delegated_accesses` | GET | `/calendar/delegatedAccesses` |
//! | `set_delegated_accesses` | POST | `/calendar/delegatedAccesses` |
//! | `get_institution_profiles_with_delegated_accesses` | GET | `/calendar/delegatedAccesses/profiles` |
//! | `get_calendar_synchronisation_configurations` | GET | `/calendar/sync/configurations` |
//! | `create_calendar_synchronisation_configuration` | POST | `/calendar/sync/configurations` |
//! | `update_calendar_synchronisation_configuration` | PUT | `/calendar/sync/configurations/{id}` |
//! | `delete_calendar_synchronisation_configuration` | DELETE | `/calendar/sync/configurations/{id}` |
//! | `get_calendar_synchronisation_consent` | GET | `/calendar/sync/consent` |
//! | `update_calendar_synchronisation_consent` | PUT | `/calendar/sync/consent` |
//! | `get_is_calendar_feed_enabled_for_municipality` | GET | `/calendar/feed/municipality/{id}/enabled` |
//! | `get_feed_configuration_by_id` | GET | `/calendar/feed/configuration/{id}` |

use crate::models::calendar::{
    BirthdayEventDto, BlockTimeSlotRequest, CalendarSynchronisationConfigurationItem,
    CalendarSynchronisationModel, CalendarSynchronisationMunicipalityFeedModel,
    CheckEventConflictInput, ConflictEventItem, CreateCalendarSynchronizationConfigurationRequest,
    CreateSimpleEventRequest, CreateTimeslotEventRequest, DailyAggregatedEventsResultModel,
    DailyEventCountResultModel, DelegateAccesses, DelegateAccessesInput, DeleteTimeslotRequest,
    EventDetailsDto, EventSimpleDto, GetEventTypesByPortalRoleResultModel,
    GetEventsForInstitutionRequestModel, GetEventsParameters,
    GetVacationRequestResponseRequestModel, ImportantDateItem, InstitutionDelegateAccessesItem,
    RespondSimpleEventRequest, RespondTimeslotEventRequest,
    RespondToVacationRegistrationRequestDto, UpdateCalendarSynchronizationConfigurationRequest,
    UpdateLessonRequest, VacationOverviewListItemResultDto, VacationWeekResultDto,
};
use crate::session::Session;

// ---------------------------------------------------------------------------
// Response types specific to this service
// ---------------------------------------------------------------------------

/// Response from event mutation operations (create, update, delete).
///
/// The API likely returns a confirmation or the affected event ID.
/// Using `serde_json::Value` since the exact shape is unverified.
pub type EventMutationResponse = serde_json::Value;

/// Response from `RespondSimpleEvent` / `RespondTimeslotEvent`.
pub type EventRespondResponse = serde_json::Value;

/// Response from `UpdateLessonEvent`.
pub type UpdateLessonResponse = serde_json::Value;

/// Response from `EditTimeslotEvent`.
pub type EditTimeslotResponse = serde_json::Value;

/// Response from `BlockTimeSlot`.
pub type BlockTimeSlotResponse = serde_json::Value;

/// Response from `DeleteTimeSlot`.
pub type DeleteTimeSlotResponse = serde_json::Value;

/// Response from `DeleteEvent`.
pub type DeleteEventResponse = serde_json::Value;

/// Response from `AddVacation`.
pub type AddVacationResponse = serde_json::Value;

/// Response from `DeleteVacation`.
pub type DeleteVacationResponse = serde_json::Value;

/// Response from `RespondToVacationRegistrationRequest`.
pub type RespondVacationResponse = serde_json::Value;

/// Response from `SetDelegatedAccesses`.
pub type SetDelegatedAccessesResponse = serde_json::Value;

/// Response from calendar sync configuration mutations.
pub type SyncConfigMutationResponse = serde_json::Value;

/// Response from `UpdateCalendarSynchronisationConsent`.
pub type UpdateSyncConsentResponse = serde_json::Value;

// ===========================================================================
// Event CRUD (AC #1)
// ===========================================================================

/// Get calendar events with filtering by profile, date range, and type.
///
/// Maps to `CalendarWebService.GetEvents()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/events`
pub async fn get_events(
    session: &mut Session,
    params: &GetEventsParameters,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(ref ids) = params.inst_profile_ids {
        for id in ids {
            query.push(format!("instProfileIds={id}"));
        }
    }
    if let Some(ref ids) = params.resource_ids {
        for id in ids {
            query.push(format!("resourceIds={id}"));
        }
    }
    if let Some(ref start) = params.start {
        query.push(format!("start={start}"));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={end}"));
    }
    if let Some(ref types) = params.specific_types {
        for t in types {
            query.push(format!("specificTypes={t}"));
        }
    }
    if let Some(ref codes) = params.school_calendar_institution_codes {
        for c in codes {
            query.push(format!("schoolCalendarInstitutionCodes={c}"));
        }
    }
    let path = if query.is_empty() {
        "calendar/events".to_string()
    } else {
        format!("calendar/events?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get event detail by ID.
///
/// Maps to `CalendarWebService.GetEventDetail()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/events/{event_id}`
pub async fn get_event_detail(
    session: &mut Session,
    event_id: i64,
) -> crate::Result<EventDetailsDto> {
    session.get(&format!("calendar/events/{event_id}")).await
}

/// Get daily aggregated events (event counts per day per type).
///
/// Maps to `CalendarWebService.GetDailyAggregatedEvents()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/events/daily`
pub async fn get_daily_aggregated_events(
    session: &mut Session,
    params: &GetEventsParameters,
) -> crate::Result<Vec<DailyAggregatedEventsResultModel>> {
    let mut query = Vec::new();
    if let Some(ref ids) = params.inst_profile_ids {
        for id in ids {
            query.push(format!("instProfileIds={id}"));
        }
    }
    if let Some(ref start) = params.start {
        query.push(format!("start={start}"));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={end}"));
    }
    if let Some(ref types) = params.specific_types {
        for t in types {
            query.push(format!("specificTypes={t}"));
        }
    }
    if let Some(ref codes) = params.school_calendar_institution_codes {
        for c in codes {
            query.push(format!("schoolCalendarInstitutionCodes={c}"));
        }
    }
    let path = if query.is_empty() {
        "calendar/events/daily".to_string()
    } else {
        format!("calendar/events/daily?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get daily event count per group.
///
/// Maps to `CalendarWebService.GetDailyGroupEventCount()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/events/dailyGroupCount`
pub async fn get_daily_group_event_count(
    session: &mut Session,
    group_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<DailyEventCountResultModel>> {
    session
        .get(&format!(
            "calendar/events/dailyGroupCount?groupId={group_id}&start={start}&end={end}"
        ))
        .await
}

/// Get events for a specific group.
///
/// Maps to `CalendarWebService.GetEventForGroup()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/events/group/{group_id}`
pub async fn get_event_for_group(
    session: &mut Session,
    group_id: i64,
    start: Option<&str>,
    end: Option<&str>,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(s) = start {
        query.push(format!("start={s}"));
    }
    if let Some(e) = end {
        query.push(format!("end={e}"));
    }
    let path = if query.is_empty() {
        format!("calendar/events/group/{group_id}")
    } else {
        format!("calendar/events/group/{group_id}?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get school-wide events.
///
/// Maps to `CalendarWebService.GetSchoolEvents()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/schoolEvents`
pub async fn get_school_events(
    session: &mut Session,
    params: &GetEventsForInstitutionRequestModel,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(ref start) = params.start {
        query.push(format!("start={start}"));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={end}"));
    }
    if let Some(ref codes) = params.inst_codes {
        for c in codes {
            query.push(format!("instCodes={c}"));
        }
    }
    let path = if query.is_empty() {
        "calendar/schoolEvents".to_string()
    } else {
        format!("calendar/schoolEvents?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get available event types for filtering.
///
/// Maps to `CalendarWebService.GetEventTypes()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/eventTypes`
pub async fn get_event_types(
    session: &mut Session,
    filter_institution_codes: &[String],
) -> crate::Result<GetEventTypesByPortalRoleResultModel> {
    let mut query = Vec::new();
    for code in filter_institution_codes {
        query.push(format!("filterInstitutionCodes={code}"));
    }
    let path = if query.is_empty() {
        "calendar/eventTypes".to_string()
    } else {
        format!("calendar/eventTypes?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get event types available for calendar feed configuration.
///
/// Maps to `CalendarWebService.GetEventTypesForCalendarFeed()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/eventTypes/feed`
pub async fn get_event_types_for_calendar_feed(
    session: &mut Session,
) -> crate::Result<GetEventTypesByPortalRoleResultModel> {
    session.get("calendar/eventTypes/feed").await
}

/// Delete a calendar event.
///
/// Maps to `CalendarWebService.DeleteEvent()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /calendar/events/{event_id}`
pub async fn delete_event(
    session: &mut Session,
    event_id: i64,
) -> crate::Result<DeleteEventResponse> {
    session.delete(&format!("calendar/events/{event_id}")).await
}

// ===========================================================================
// Event responses (AC #2)
// ===========================================================================

/// Respond to a simple event invitation (accept/decline).
///
/// Maps to `CalendarWebService.RespondSimpleEvent()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/events/{event_id}/respond`
pub async fn respond_simple_event(
    session: &mut Session,
    args: &RespondSimpleEventRequest,
) -> crate::Result<EventRespondResponse> {
    let event_id = args.event_id.unwrap_or(0);
    session
        .post(&format!("calendar/events/{event_id}/respond"), args)
        .await
}

/// Respond to a timeslot event (book a timeslot).
///
/// Maps to `CalendarWebService.RespondTimeslotEvent()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/timeslots/{event_id}/respond`
pub async fn respond_timeslot_event(
    session: &mut Session,
    args: &RespondTimeslotEventRequest,
) -> crate::Result<EventRespondResponse> {
    let event_id = args.event_id.unwrap_or(0);
    session
        .post(&format!("calendar/timeslots/{event_id}/respond"), args)
        .await
}

// ===========================================================================
// Timeslot and lesson management (AC #3)
// ===========================================================================

/// Edit a timeslot event.
///
/// Maps to `CalendarWebService.EditTimeslotEvent()`.
///
/// # Endpoint (inferred)
///
/// `PUT /calendar/timeslots/{event_id}`
pub async fn edit_timeslot_event(
    session: &mut Session,
    args: &CreateTimeslotEventRequest,
) -> crate::Result<EditTimeslotResponse> {
    let event_id = args.event_id.unwrap_or(0);
    session
        .put(&format!("calendar/timeslots/{event_id}"), args)
        .await
}

/// Block a timeslot to prevent booking.
///
/// Maps to `CalendarWebService.BlockTimeSlot()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/timeslots/block`
pub async fn block_time_slot(
    session: &mut Session,
    args: &BlockTimeSlotRequest,
) -> crate::Result<BlockTimeSlotResponse> {
    session.post("calendar/timeslots/block", args).await
}

/// Delete a timeslot booking.
///
/// Maps to `CalendarWebService.DeleteTimeSlot()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /calendar/timeslots/{event_id}`
///
/// NOTE: The timeslot ID and index are likely sent as query parameters
/// or in a request body. Since DELETE with body is unusual, query
/// parameters are used here.
pub async fn delete_time_slot(
    session: &mut Session,
    args: &DeleteTimeslotRequest,
) -> crate::Result<DeleteTimeSlotResponse> {
    let event_id = args.event_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(ts_id) = args.time_slot_id {
        query.push(format!("timeSlotId={ts_id}"));
    }
    if let Some(idx) = args.time_slot_index {
        query.push(format!("timeSlotIndex={idx}"));
    }
    if let Some(pid) = args.concerning_institution_profile_id {
        query.push(format!("concerningInstitutionProfileId={pid}"));
    }
    let path = if query.is_empty() {
        format!("calendar/timeslots/{event_id}")
    } else {
        format!("calendar/timeslots/{event_id}?{}", query.join("&"))
    };
    session.delete(&path).await
}

/// Update a lesson event (notes, resources, attachments).
///
/// Maps to `CalendarWebService.UpdateLessonEvent()`.
///
/// # Endpoint (inferred)
///
/// `PUT /calendar/lessons/{event_id}`
pub async fn update_lesson_event(
    session: &mut Session,
    args: &UpdateLessonRequest,
) -> crate::Result<UpdateLessonResponse> {
    let event_id = args.event_id.unwrap_or(0);
    session
        .put(&format!("calendar/lessons/{event_id}"), args)
        .await
}

// ===========================================================================
// Vacation registration (AC #4)
// ===========================================================================

/// Add a vacation registration event.
///
/// Maps to `CalendarWebService.AddVacation()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/vacations`
///
/// NOTE: The request body likely uses a `CreateSimpleEventRequest`-derived
/// structure for the vacation event. Using the simple event request type
/// as the closest match.
pub async fn add_vacation(
    session: &mut Session,
    args: &CreateSimpleEventRequest,
) -> crate::Result<AddVacationResponse> {
    session.post("calendar/vacations", args).await
}

/// Get a vacation by ID.
///
/// Maps to `CalendarWebService.GetVacation()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/vacations/{vacation_id}`
pub async fn get_vacation(
    session: &mut Session,
    vacation_id: i64,
) -> crate::Result<EventDetailsDto> {
    session
        .get(&format!("calendar/vacations/{vacation_id}"))
        .await
}

/// Delete a vacation.
///
/// Maps to `CalendarWebService.DeleteVacation()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /calendar/vacations/{vacation_id}`
pub async fn delete_vacation(
    session: &mut Session,
    vacation_id: i64,
) -> crate::Result<DeleteVacationResponse> {
    session
        .delete(&format!("calendar/vacations/{vacation_id}"))
        .await
}

/// Get future vacation requests.
///
/// Maps to `CalendarWebService.GetFutureVacationRequest()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/vacations/future`
pub async fn get_future_vacation_request(
    session: &mut Session,
    filter_institution_codes: &[String],
) -> crate::Result<Vec<VacationOverviewListItemResultDto>> {
    let mut query = Vec::new();
    for code in filter_institution_codes {
        query.push(format!("filterInstitutionCalendarCodes={code}"));
    }
    let path = if query.is_empty() {
        "calendar/vacations/future".to_string()
    } else {
        format!("calendar/vacations/future?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get the response details for a vacation request.
///
/// Maps to `CalendarWebService.GetVacationRequestResponse()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/vacations/{vacation_id}/response`
pub async fn get_vacation_request_response(
    session: &mut Session,
    args: &GetVacationRequestResponseRequestModel,
) -> crate::Result<Vec<VacationWeekResultDto>> {
    let vacation_id = args.vacation_request_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(ref ids) = args.filter_department_group_ids {
        for id in ids {
            query.push(format!("filterDepartmentGroupIds={id}"));
        }
    }
    if let Some(ref ids) = args.filter_department_filtering_group_ids {
        for id in ids {
            query.push(format!("filterDepartmentFilteringGroupIds={id}"));
        }
    }
    let path = if query.is_empty() {
        format!("calendar/vacations/{vacation_id}/response")
    } else {
        format!(
            "calendar/vacations/{vacation_id}/response?{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Respond to a vacation registration request.
///
/// Maps to `CalendarWebService.RespondToVacationRegistrationRequest()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/vacations/{vacation_id}/respond`
pub async fn respond_to_vacation_registration_request(
    session: &mut Session,
    vacation_id: i64,
    args: &RespondToVacationRegistrationRequestDto,
) -> crate::Result<RespondVacationResponse> {
    session
        .post(&format!("calendar/vacations/{vacation_id}/respond"), args)
        .await
}

// ===========================================================================
// Calendar sync configuration and consent (AC #5)
// ===========================================================================

/// Get all calendar synchronisation configurations.
///
/// Maps to `CalendarWebService.GetCalendarSynchronisationConfigurations()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/sync/configurations`
pub async fn get_calendar_synchronisation_configurations(
    session: &mut Session,
) -> crate::Result<Vec<CalendarSynchronisationConfigurationItem>> {
    session.get("calendar/sync/configurations").await
}

/// Create a new calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.CreateCalendarSynchronisationConfiguration()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/sync/configurations`
pub async fn create_calendar_synchronisation_configuration(
    session: &mut Session,
    args: &CreateCalendarSynchronizationConfigurationRequest,
) -> crate::Result<SyncConfigMutationResponse> {
    session.post("calendar/sync/configurations", args).await
}

/// Update an existing calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.UpdateCalendarSynchronisationConfiguration()`.
///
/// # Endpoint (inferred)
///
/// `PUT /calendar/sync/configurations/{config_id}`
pub async fn update_calendar_synchronisation_configuration(
    session: &mut Session,
    config_id: i64,
    args: &UpdateCalendarSynchronizationConfigurationRequest,
) -> crate::Result<SyncConfigMutationResponse> {
    session
        .put(&format!("calendar/sync/configurations/{config_id}"), args)
        .await
}

/// Delete a calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.DeleteCalendarSynchronisationConfiguration()`.
///
/// # Endpoint (inferred)
///
/// `DELETE /calendar/sync/configurations/{config_id}`
pub async fn delete_calendar_synchronisation_configuration(
    session: &mut Session,
    config_id: i64,
) -> crate::Result<SyncConfigMutationResponse> {
    session
        .delete(&format!("calendar/sync/configurations/{config_id}"))
        .await
}

/// Get the current calendar synchronisation consent status.
///
/// Maps to `CalendarWebService.GetCalendarSynchronisationConsent()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/sync/consent`
pub async fn get_calendar_synchronisation_consent(
    session: &mut Session,
) -> crate::Result<CalendarSynchronisationModel> {
    session.get("calendar/sync/consent").await
}

/// Update (accept/revoke) calendar synchronisation consent.
///
/// Maps to `CalendarWebService.UpdateCalendarSynchronisationConsent()`.
///
/// # Endpoint (inferred)
///
/// `PUT /calendar/sync/consent`
pub async fn update_calendar_synchronisation_consent(
    session: &mut Session,
    args: &CalendarSynchronisationModel,
) -> crate::Result<UpdateSyncConsentResponse> {
    session.put("calendar/sync/consent", args).await
}

// ===========================================================================
// Delegated access, birthdays, important dates, conflict check (AC #6)
// ===========================================================================

/// Get delegated calendar accesses for the current user.
///
/// Maps to `CalendarWebService.GetDelegatedAccesses()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/delegatedAccesses`
pub async fn get_delegated_accesses(
    session: &mut Session,
    inst_profile_id: Option<i64>,
) -> crate::Result<Vec<DelegateAccesses>> {
    let path = match inst_profile_id {
        Some(id) => format!("calendar/delegatedAccesses?instProfileId={id}"),
        None => "calendar/delegatedAccesses".to_string(),
    };
    session.get(&path).await
}

/// Set delegated calendar accesses.
///
/// Maps to `CalendarWebService.SetDelegatedAccesses()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/delegatedAccesses`
pub async fn set_delegated_accesses(
    session: &mut Session,
    args: &DelegateAccessesInput,
) -> crate::Result<SetDelegatedAccessesResponse> {
    session.post("calendar/delegatedAccesses", args).await
}

/// Get institution profiles that have delegated calendar access.
///
/// Maps to `CalendarWebService.GetInstitutionProfilesWithDelegatedAccesses()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/delegatedAccesses/profiles`
pub async fn get_institution_profiles_with_delegated_accesses(
    session: &mut Session,
    inst_profile_id: Option<i64>,
) -> crate::Result<Vec<InstitutionDelegateAccessesItem>> {
    let path = match inst_profile_id {
        Some(id) => format!("calendar/delegatedAccesses/profiles?instProfileId={id}"),
        None => "calendar/delegatedAccesses/profiles".to_string(),
    };
    session.get(&path).await
}

/// Get birthdays for a group.
///
/// Maps to `CalendarWebService.GetBirthdaysForGroup()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/birthdays/group/{group_id}`
pub async fn get_birthdays_for_group(
    session: &mut Session,
    group_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<BirthdayEventDto>> {
    session
        .get(&format!(
            "calendar/birthdays/group/{group_id}?start={start}&end={end}"
        ))
        .await
}

/// Get birthdays for an institution.
///
/// Maps to `CalendarWebService.GetBirthdaysForInstitution()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/birthdays/institution/{institution_id}`
pub async fn get_birthdays_for_institution(
    session: &mut Session,
    institution_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<BirthdayEventDto>> {
    session
        .get(&format!(
            "calendar/birthdays/institution/{institution_id}?start={start}&end={end}"
        ))
        .await
}

/// Get the top important dates (shown on dashboard).
///
/// Maps to `CalendarWebService.GetTopImportantDate()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/importantDates/top`
pub async fn get_top_important_date(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ImportantDateItem>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(format!("instProfileIds={id}"));
    }
    let path = if query.is_empty() {
        "calendar/importantDates/top".to_string()
    } else {
        format!("calendar/importantDates/top?{}", query.join("&"))
    };
    session.get(&path).await
}

/// Check scheduling conflicts for attendees.
///
/// Maps to `CalendarWebService.CheckConflictEventForAttendees()`.
///
/// # Endpoint (inferred)
///
/// `POST /calendar/events/conflicts`
pub async fn check_conflict_event_for_attendees(
    session: &mut Session,
    args: &CheckEventConflictInput,
) -> crate::Result<Vec<ConflictEventItem>> {
    session.post("calendar/events/conflicts", args).await
}

/// Check whether calendar feed is enabled for a municipality.
///
/// Maps to `CalendarWebService.GetIsCalendarFeedEnabledForMunicipality()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/feed/municipality/{municipality_id}/enabled`
pub async fn get_is_calendar_feed_enabled_for_municipality(
    session: &mut Session,
    municipality_id: i64,
) -> crate::Result<CalendarSynchronisationMunicipalityFeedModel> {
    session
        .get(&format!(
            "calendar/feed/municipality/{municipality_id}/enabled"
        ))
        .await
}

/// Get a feed configuration by ID.
///
/// Maps to `CalendarWebService.GetFeedConfigurationById()`.
///
/// # Endpoint (inferred)
///
/// `GET /calendar/feed/configuration/{config_id}`
pub async fn get_feed_configuration_by_id(
    session: &mut Session,
    config_id: i64,
) -> crate::Result<CalendarSynchronisationConfigurationItem> {
    session
        .get(&format!("calendar/feed/configuration/{config_id}"))
        .await
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::models::calendar::*;

    #[test]
    fn get_events_params_serializes() {
        let params = GetEventsParameters {
            inst_profile_ids: Some(vec![100, 200]),
            resource_ids: None,
            start: Some("2024-03-01".to_string()),
            end: Some("2024-03-31".to_string()),
            specific_types: Some(vec!["Event".to_string()]),
            school_calendar_institution_codes: None,
        };
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["instProfileIds"], serde_json::json!([100, 200]));
        assert_eq!(json["start"], "2024-03-01");
    }

    #[test]
    fn respond_simple_event_request_serializes() {
        let req = RespondSimpleEventRequest {
            event_id: Some(42),
            institution_profile_id: Some(100),
            invited_inst_profile_id: Some(200),
            response_type: Some(crate::enums::calendar::ResponseType::Accepted),
            occurrence_date_time: None,
            number_of_adult_participants: Some(2),
            number_of_child_participants: Some(1),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["eventId"], 42);
        assert_eq!(json["responseType"], "Accepted");
        assert_eq!(json["numberOfAdultParticipants"], 2);
    }

    #[test]
    fn block_time_slot_request_serializes() {
        let req = BlockTimeSlotRequest {
            event_id: Some(10),
            time_slot_id: Some(5),
            time_slot_index: Some(2),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["eventId"], 10);
        assert_eq!(json["timeSlotId"], 5);
    }

    #[test]
    fn update_lesson_request_serializes() {
        let req = UpdateLessonRequest {
            event_id: Some(99),
            institution_profile_id: Some(100),
            note_to_class: Some("<p>Homework</p>".to_string()),
            note_to_teacher: None,
            note_to_substitute: None,
            additional_resource_ids: Some(vec![1, 2]),
            additional_resource_text: None,
            attachment_ids: Some(vec![]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["eventId"], 99);
        assert_eq!(json["noteToClass"], "<p>Homework</p>");
    }

    #[test]
    fn delegate_accesses_input_serializes() {
        let input = DelegateAccessesInput {
            owner_inst_profile_id: Some(100),
            delegated_inst_profile_ids: Some(vec![200, 300]),
        };
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json["ownerInstProfileId"], 100);
        assert_eq!(
            json["delegatedInstProfileIds"],
            serde_json::json!([200, 300])
        );
    }

    #[test]
    fn create_sync_config_request_serializes() {
        let req = CreateCalendarSynchronizationConfigurationRequest {
            filters: Some(vec![
                crate::enums::calendar::EventType::Event,
                crate::enums::calendar::EventType::Meeting,
            ]),
            weekly: true,
            institution_profile_id: Some(100),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert!(json["weekly"].as_bool().unwrap());
        assert_eq!(json["institutionProfileId"], 100);
    }

    #[test]
    fn calendar_sync_model_serializes() {
        let model = CalendarSynchronisationModel {
            policy_accepted: true,
        };
        let json = serde_json::to_value(&model).unwrap();
        assert!(json["policyAccepted"].as_bool().unwrap());
    }

    #[test]
    fn check_conflict_input_serializes() {
        let input = CheckEventConflictInput {
            start: Some("2024-04-01T10:00:00".to_string()),
            end: Some("2024-04-01T12:00:00".to_string()),
            all_day: false,
            institution_profile_ids: Some(vec![100, 200]),
            exclude_event_id: Some(5),
        };
        let json = serde_json::to_value(&input).unwrap();
        assert_eq!(json["start"], "2024-04-01T10:00:00");
        assert_eq!(json["institutionProfileIds"], serde_json::json!([100, 200]));
    }

    #[test]
    fn respond_vacation_request_serializes() {
        let req = RespondToVacationRegistrationRequestDto {
            child_id: Some(10),
            vacation_registration_response_id: Some(99),
            days: Some(vec![GuardianRegisterVacationIntervals {
                date: Some("2024-07-01".to_string()),
                entry_time: Some("08:00".to_string()),
                exit_time: Some("16:00".to_string()),
                is_coming: true,
            }]),
            comment: Some("See you!".to_string()),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["childId"], 10);
        let days = json["days"].as_array().unwrap();
        assert_eq!(days.len(), 1);
        assert!(days[0]["isComing"].as_bool().unwrap());
    }

    #[test]
    fn get_birthday_events_request_serializes() {
        let req = GetBirthdayEvents {
            start: Some("2024-01-01".to_string()),
            end: Some("2024-12-31".to_string()),
            inst_codes: Some(vec!["101001".to_string()]),
        };
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["start"], "2024-01-01");
        assert_eq!(json["instCodes"], serde_json::json!(["101001"]));
    }
}
