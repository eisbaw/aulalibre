//! Calendar service.
//!
//! Maps to `AulaNative.Services.Web.CalendarWebService` (37 methods) from the APK.
//!
//! # Endpoint paths
//!
//! | Urls.cs constant | RPC method |
//! |------------------|------------|
//! | `GET_EVENTS` | `calendar.getEventsByProfileIdsAndResourceIds` |
//! | `GET_EVENT_BY_ID` | `calendar.getEventById` |
//! | `GET_DAILY_AGGREGATED_EVENT` | `calendar.getDailyAggregatedEvents` |
//! | `GET_DAILY_GROUP_EVENT_COUNT` | `calendar.getDailyEventCountForGroup` |
//! | `GET_EVENT_FOR_GROUP` | `calendar.geteventsbygroupid` |
//! | `GET_EVENTS_FOR_INSTITUTION` | `calendar.getEventsForInstitutions` |
//! | `GET_IMPORTANT_DATE` | `calendar.getImportantDates` |
//! | `GET_EVENT_TYPES` | `calendar.getEventTypes` |
//! | `DELETE_EVENT` | `calendar.deleteEvent` |
//! | `RESPOND_SIMPLE_EVENT` | `calendar.respondToSimpleEvent` |
//! | `RESPOND_TIMESLOT_EVENT` | `calendar.respondToTimeSlotEvent` |
//! | `RESPOND_EVENT` | `calendar.respondToEvent` |
//! | `EDIT_TIMESLOT_EVENT` | `calendar.updateResponseToTimeSlotEvent` |
//! | `BLOCK_TIMES_SLOT` | `calendar.blockTimeSlot` |
//! | `DELETE_TIMES_SLOT` | `calendar.removeBlockingOrResponseToTimeSlot` |
//! | `UPDATE_LESSON_EVENT` | `calendar.updateLessonEvent` |
//! | `CHECK_EVENT_CONFLICT` | `calendar.checkConflictEventForAttendees` |
//! | `GET_BIRTHDAY_EVENT_FOR_GROUP` | `calendar.getBirthdayEventsForGroup` |
//! | `GET_BIRTHDAY_EVENT_FOR_INSTITUTION` | `calendar.getBirthdayEventsForInstitutions` |
//! | `ADD_VACATION` | `calendar.addVacation` |
//! | `GET_VACATION_BY_ID` | `calendar.getVacationById` |
//! | `DELETE_VACATION` | `calendar.deleteVacation` |
//! | `GET_ALL_FURURE_VACATION_REQUESTS` | `calendar.getFutureVacationRequests` |
//! | `GET_VACATION_REQUEST_RESPONSES` | `calendar.getVacationRequestResponses` |
//! | `RESPOND_TO_VACATION_REGISTRATION_REQUEST` | `calendar.respondToVacationRegistrationRequest` |
//! | `CREATE_VACATION_REGISTRATION` | `calendar.createVacationRequest` |
//! | `UPDATE_VACATION_REQUEST` | `calendar.updateVacationRequest` |
//! | `GET_DELEGATED_ACCESSES` | `calendar.getDelegatedAccesses` |
//! | `SET_DELEGATED_ACCESS` | `calendar.setDelegatedAccesses` |
//! | `GET_INSTITUTION_PROFILES_DELEGATED_ACCESSES` | `calendar.getInstitutionProfilesWithDelegatedAccess` |
//! | `SET_DELEGATED_CONTEXT` | `calendar.setDelegatedContext` |
//! | `GET_DELEGATED_CONTEXT` | `calendar.getDelegatedContext` |
//! | `GET_CALENDAR_SYNCHRONISATION_CONSENT` | `CalendarFeed.getPolicyAnswer` |
//! | `DELETE_CALENDAR_SYNCHRONISATION_CONSENT` | `CalendarFeed.setPolicyAnswer` |
//! | `GET_CALENDAR_SYNCHRONISATION_CONFIGURATIONS` | `CalendarFeed.getFeedConfigurations` |
//! | `CREATE_CALENDAR_SYNCHRONISATION_CONFIGURATION` | `CalendarFeed.createFeedConfiguration` |
//! | `UPDATE_CALENDAR_SYNCHRONISATION_CONFIGURATION` | `CalendarFeed.updateFeedConfiguration` |
//! | `DELETE_CALENDAR_SYNCHRONISATION_CONFIGURATION` | `CalendarFeed.removeFeedConfiguration` |
//! | `GET_EVENT_TYPE_FOR_PORTAL_ROLE` | `CalendarFeed.getEventTypesRelevantForPortalRole` |
//! | `GET_CALENDAR_SYNCHRONISATION_CONFIGURATIONS_BY_ID` | `CalendarFeed.getFeedConfigurationById` |
//! | `GET_CALENDAR_SYNCHRONISATION_MUNICIPALITY_FEED_ENABLED` | `MunicipalConfiguration.getCalendarFeedEnabled` |

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
use crate::services::query::{encode_value, param_num};
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
/// # Endpoint
///
/// `GET ?method=calendar.getEventsByProfileIdsAndResourceIds`
pub async fn get_events(
    session: &mut Session,
    params: &GetEventsParameters,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(ref ids) = params.inst_profile_ids {
        for id in ids {
            query.push(param_num("instProfileIds", id));
        }
    }
    if let Some(ref ids) = params.resource_ids {
        for id in ids {
            query.push(param_num("resourceIds", id));
        }
    }
    if let Some(ref start) = params.start {
        query.push(format!("start={}", encode_value(start)));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={}", encode_value(end)));
    }
    if let Some(ref types) = params.specific_types {
        for t in types {
            query.push(format!("specificTypes={}", encode_value(t)));
        }
    }
    if let Some(ref codes) = params.school_calendar_institution_codes {
        for c in codes {
            query.push(format!(
                "schoolCalendarInstitutionCodes={}",
                encode_value(c)
            ));
        }
    }
    let path = if query.is_empty() {
        "?method=calendar.getEventsByProfileIdsAndResourceIds".to_string()
    } else {
        format!(
            "?method=calendar.getEventsByProfileIdsAndResourceIds&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get event detail by ID.
///
/// Maps to `CalendarWebService.GetEventDetail()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getEventById&eventId={event_id}`
pub async fn get_event_detail(
    session: &mut Session,
    event_id: i64,
) -> crate::Result<EventDetailsDto> {
    session
        .get(&format!("?method=calendar.getEventById&eventId={event_id}"))
        .await
}

/// Get daily aggregated events (event counts per day per type).
///
/// Maps to `CalendarWebService.GetDailyAggregatedEvents()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getDailyAggregatedEvents`
pub async fn get_daily_aggregated_events(
    session: &mut Session,
    params: &GetEventsParameters,
) -> crate::Result<Vec<DailyAggregatedEventsResultModel>> {
    let mut query = Vec::new();
    if let Some(ref ids) = params.inst_profile_ids {
        for id in ids {
            query.push(param_num("instProfileIds", id));
        }
    }
    if let Some(ref start) = params.start {
        query.push(format!("start={}", encode_value(start)));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={}", encode_value(end)));
    }
    if let Some(ref types) = params.specific_types {
        for t in types {
            query.push(format!("specificTypes={}", encode_value(t)));
        }
    }
    if let Some(ref codes) = params.school_calendar_institution_codes {
        for c in codes {
            query.push(format!(
                "schoolCalendarInstitutionCodes={}",
                encode_value(c)
            ));
        }
    }
    let path = if query.is_empty() {
        "?method=calendar.getDailyAggregatedEvents".to_string()
    } else {
        format!(
            "?method=calendar.getDailyAggregatedEvents&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get daily event count per group.
///
/// Maps to `CalendarWebService.GetDailyGroupEventCount()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getDailyEventCountForGroup`
pub async fn get_daily_group_event_count(
    session: &mut Session,
    group_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<DailyEventCountResultModel>> {
    session
        .get(&format!(
            "?method=calendar.getDailyEventCountForGroup&groupId={group_id}&start={}&end={}",
            encode_value(start),
            encode_value(end)
        ))
        .await
}

/// Get events for a specific group.
///
/// Maps to `CalendarWebService.GetEventForGroup()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.geteventsbygroupid&groupId={group_id}`
pub async fn get_event_for_group(
    session: &mut Session,
    group_id: i64,
    start: Option<&str>,
    end: Option<&str>,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(s) = start {
        query.push(format!("start={}", encode_value(s)));
    }
    if let Some(e) = end {
        query.push(format!("end={}", encode_value(e)));
    }
    let path = if query.is_empty() {
        format!("?method=calendar.geteventsbygroupid&groupId={group_id}")
    } else {
        format!(
            "?method=calendar.geteventsbygroupid&groupId={group_id}&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get school-wide events.
///
/// Maps to `CalendarWebService.GetSchoolEvents()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getEventsForInstitutions`
pub async fn get_school_events(
    session: &mut Session,
    params: &GetEventsForInstitutionRequestModel,
) -> crate::Result<Vec<EventSimpleDto>> {
    let mut query = Vec::new();
    if let Some(ref start) = params.start {
        query.push(format!("start={}", encode_value(start)));
    }
    if let Some(ref end) = params.end {
        query.push(format!("end={}", encode_value(end)));
    }
    if let Some(ref codes) = params.inst_codes {
        for c in codes {
            query.push(format!("instCodes={}", encode_value(c)));
        }
    }
    let path = if query.is_empty() {
        "?method=calendar.getEventsForInstitutions".to_string()
    } else {
        format!(
            "?method=calendar.getEventsForInstitutions&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get available event types for filtering.
///
/// Maps to `CalendarWebService.GetEventTypes()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getEventTypes`
pub async fn get_event_types(
    session: &mut Session,
    filter_institution_codes: &[String],
) -> crate::Result<GetEventTypesByPortalRoleResultModel> {
    let mut query = Vec::new();
    for code in filter_institution_codes {
        query.push(format!("filterInstitutionCodes={}", encode_value(code)));
    }
    let path = if query.is_empty() {
        "?method=calendar.getEventTypes".to_string()
    } else {
        format!("?method=calendar.getEventTypes&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Get event types available for calendar feed configuration.
///
/// Maps to `CalendarWebService.GetEventTypesForCalendarFeed()`.
///
/// # Endpoint
///
/// `GET ?method=CalendarFeed.getEventTypesRelevantForPortalRole`
pub async fn get_event_types_for_calendar_feed(
    session: &mut Session,
) -> crate::Result<GetEventTypesByPortalRoleResultModel> {
    session
        .get("?method=CalendarFeed.getEventTypesRelevantForPortalRole")
        .await
}

/// Delete a calendar event.
///
/// Maps to `CalendarWebService.DeleteEvent()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.deleteEvent`
pub async fn delete_event(
    session: &mut Session,
    event_id: i64,
) -> crate::Result<DeleteEventResponse> {
    session
        .post(
            "?method=calendar.deleteEvent",
            &serde_json::json!({"eventId": event_id}),
        )
        .await
}

// ===========================================================================
// Event responses (AC #2)
// ===========================================================================

/// Respond to a simple event invitation (accept/decline).
///
/// Maps to `CalendarWebService.RespondSimpleEvent()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.respondToSimpleEvent`
pub async fn respond_simple_event(
    session: &mut Session,
    args: &RespondSimpleEventRequest,
) -> crate::Result<EventRespondResponse> {
    session
        .post("?method=calendar.respondToSimpleEvent", args)
        .await
}

/// Respond to a timeslot event (book a timeslot).
///
/// Maps to `CalendarWebService.RespondTimeslotEvent()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.respondToTimeSlotEvent`
pub async fn respond_timeslot_event(
    session: &mut Session,
    args: &RespondTimeslotEventRequest,
) -> crate::Result<EventRespondResponse> {
    session
        .post("?method=calendar.respondToTimeSlotEvent", args)
        .await
}

// ===========================================================================
// Timeslot and lesson management (AC #3)
// ===========================================================================

/// Edit a timeslot event.
///
/// Maps to `CalendarWebService.EditTimeslotEvent()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.updateResponseToTimeSlotEvent`
pub async fn edit_timeslot_event(
    session: &mut Session,
    args: &CreateTimeslotEventRequest,
) -> crate::Result<EditTimeslotResponse> {
    session
        .post("?method=calendar.updateResponseToTimeSlotEvent", args)
        .await
}

/// Block a timeslot to prevent booking.
///
/// Maps to `CalendarWebService.BlockTimeSlot()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.blockTimeSlot`
pub async fn block_time_slot(
    session: &mut Session,
    args: &BlockTimeSlotRequest,
) -> crate::Result<BlockTimeSlotResponse> {
    session.post("?method=calendar.blockTimeSlot", args).await
}

/// Delete a timeslot booking.
///
/// Maps to `CalendarWebService.DeleteTimeSlot()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.removeBlockingOrResponseToTimeSlot`
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
        format!("?method=calendar.removeBlockingOrResponseToTimeSlot&eventId={event_id}")
    } else {
        format!(
            "?method=calendar.removeBlockingOrResponseToTimeSlot&eventId={event_id}&{}",
            query.join("&")
        )
    };
    session.post_empty(&path).await
}

/// Update a lesson event (notes, resources, attachments).
///
/// Maps to `CalendarWebService.UpdateLessonEvent()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.updateLessonEvent`
pub async fn update_lesson_event(
    session: &mut Session,
    args: &UpdateLessonRequest,
) -> crate::Result<UpdateLessonResponse> {
    session
        .post("?method=calendar.updateLessonEvent", args)
        .await
}

// ===========================================================================
// Vacation registration (AC #4)
// ===========================================================================

/// Add a vacation registration event.
///
/// Maps to `CalendarWebService.AddVacation()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.addVacation`
///
/// NOTE: The request body likely uses a `CreateSimpleEventRequest`-derived
/// structure for the vacation event. Using the simple event request type
/// as the closest match.
pub async fn add_vacation(
    session: &mut Session,
    args: &CreateSimpleEventRequest,
) -> crate::Result<AddVacationResponse> {
    session.post("?method=calendar.addVacation", args).await
}

/// Get a vacation by ID.
///
/// Maps to `CalendarWebService.GetVacation()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getVacationById&vacationId={vacation_id}`
pub async fn get_vacation(
    session: &mut Session,
    vacation_id: i64,
) -> crate::Result<EventDetailsDto> {
    session
        .get(&format!(
            "?method=calendar.getVacationById&vacationId={vacation_id}"
        ))
        .await
}

/// Delete a vacation.
///
/// Maps to `CalendarWebService.DeleteVacation()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.deleteVacation`
pub async fn delete_vacation(
    session: &mut Session,
    vacation_id: i64,
) -> crate::Result<DeleteVacationResponse> {
    session
        .post(
            "?method=calendar.deleteVacation",
            &serde_json::json!({"vacationId": vacation_id}),
        )
        .await
}

/// Get future vacation requests.
///
/// Maps to `CalendarWebService.GetFutureVacationRequest()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getFutureVacationRequests`
pub async fn get_future_vacation_request(
    session: &mut Session,
    filter_institution_codes: &[String],
) -> crate::Result<Vec<VacationOverviewListItemResultDto>> {
    let mut query = Vec::new();
    for code in filter_institution_codes {
        query.push(format!(
            "filterInstitutionCalendarCodes={}",
            encode_value(code)
        ));
    }
    let path = if query.is_empty() {
        "?method=calendar.getFutureVacationRequests".to_string()
    } else {
        format!(
            "?method=calendar.getFutureVacationRequests&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Get the response details for a vacation request.
///
/// Maps to `CalendarWebService.GetVacationRequestResponse()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getVacationRequestResponses`
pub async fn get_vacation_request_response(
    session: &mut Session,
    args: &GetVacationRequestResponseRequestModel,
) -> crate::Result<Vec<VacationWeekResultDto>> {
    let vacation_id = args.vacation_request_id.unwrap_or(0);
    let mut query = Vec::new();
    if let Some(ref ids) = args.filter_department_group_ids {
        for id in ids {
            query.push(param_num("filterDepartmentGroupIds", id));
        }
    }
    if let Some(ref ids) = args.filter_department_filtering_group_ids {
        for id in ids {
            query.push(param_num("filterDepartmentFilteringGroupIds", id));
        }
    }
    let path = if query.is_empty() {
        format!("?method=calendar.getVacationRequestResponses&vacationRequestId={vacation_id}")
    } else {
        format!(
            "?method=calendar.getVacationRequestResponses&vacationRequestId={vacation_id}&{}",
            query.join("&")
        )
    };
    session.get(&path).await
}

/// Respond to a vacation registration request.
///
/// Maps to `CalendarWebService.RespondToVacationRegistrationRequest()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.respondToVacationRegistrationRequest`
pub async fn respond_to_vacation_registration_request(
    session: &mut Session,
    args: &RespondToVacationRegistrationRequestDto,
) -> crate::Result<RespondVacationResponse> {
    session
        .post(
            "?method=calendar.respondToVacationRegistrationRequest",
            args,
        )
        .await
}

// ===========================================================================
// Calendar sync configuration and consent (AC #5)
// ===========================================================================

/// Get all calendar synchronisation configurations.
///
/// Maps to `CalendarWebService.GetCalendarSynchronisationConfigurations()`.
///
/// # Endpoint
///
/// `GET ?method=CalendarFeed.getFeedConfigurations`
pub async fn get_calendar_synchronisation_configurations(
    session: &mut Session,
) -> crate::Result<Vec<CalendarSynchronisationConfigurationItem>> {
    session
        .get("?method=CalendarFeed.getFeedConfigurations")
        .await
}

/// Create a new calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.CreateCalendarSynchronisationConfiguration()`.
///
/// # Endpoint
///
/// `POST ?method=CalendarFeed.createFeedConfiguration`
pub async fn create_calendar_synchronisation_configuration(
    session: &mut Session,
    args: &CreateCalendarSynchronizationConfigurationRequest,
) -> crate::Result<SyncConfigMutationResponse> {
    session
        .post("?method=CalendarFeed.createFeedConfiguration", args)
        .await
}

/// Update an existing calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.UpdateCalendarSynchronisationConfiguration()`.
///
/// # Endpoint
///
/// `POST ?method=CalendarFeed.updateFeedConfiguration`
pub async fn update_calendar_synchronisation_configuration(
    session: &mut Session,
    args: &UpdateCalendarSynchronizationConfigurationRequest,
) -> crate::Result<SyncConfigMutationResponse> {
    session
        .post("?method=CalendarFeed.updateFeedConfiguration", args)
        .await
}

/// Delete a calendar synchronisation configuration.
///
/// Maps to `CalendarWebService.DeleteCalendarSynchronisationConfiguration()`.
///
/// # Endpoint
///
/// `POST ?method=CalendarFeed.removeFeedConfiguration`
pub async fn delete_calendar_synchronisation_configuration(
    session: &mut Session,
    config_id: i64,
) -> crate::Result<SyncConfigMutationResponse> {
    session
        .post(
            "?method=CalendarFeed.removeFeedConfiguration",
            &serde_json::json!({"configId": config_id}),
        )
        .await
}

/// Get the current calendar synchronisation consent status.
///
/// Maps to `CalendarWebService.GetCalendarSynchronisationConsent()`.
///
/// # Endpoint
///
/// `GET ?method=CalendarFeed.getPolicyAnswer`
pub async fn get_calendar_synchronisation_consent(
    session: &mut Session,
) -> crate::Result<CalendarSynchronisationModel> {
    session.get("?method=CalendarFeed.getPolicyAnswer").await
}

/// Update (accept/revoke) calendar synchronisation consent.
///
/// Maps to `CalendarWebService.UpdateCalendarSynchronisationConsent()`.
///
/// # Endpoint
///
/// `POST ?method=CalendarFeed.setPolicyAnswer`
pub async fn update_calendar_synchronisation_consent(
    session: &mut Session,
    args: &CalendarSynchronisationModel,
) -> crate::Result<UpdateSyncConsentResponse> {
    session
        .post("?method=CalendarFeed.setPolicyAnswer", args)
        .await
}

// ===========================================================================
// Delegated access, birthdays, important dates, conflict check (AC #6)
// ===========================================================================

/// Get delegated calendar accesses for the current user.
///
/// Maps to `CalendarWebService.GetDelegatedAccesses()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getDelegatedAccesses`
pub async fn get_delegated_accesses(
    session: &mut Session,
    inst_profile_id: Option<i64>,
) -> crate::Result<Vec<DelegateAccesses>> {
    let path = match inst_profile_id {
        Some(id) => format!("?method=calendar.getDelegatedAccesses&instProfileId={id}"),
        None => "?method=calendar.getDelegatedAccesses".to_string(),
    };
    session.get(&path).await
}

/// Set delegated calendar accesses.
///
/// Maps to `CalendarWebService.SetDelegatedAccesses()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.setDelegatedAccesses`
pub async fn set_delegated_accesses(
    session: &mut Session,
    args: &DelegateAccessesInput,
) -> crate::Result<SetDelegatedAccessesResponse> {
    session
        .post("?method=calendar.setDelegatedAccesses", args)
        .await
}

/// Get institution profiles that have delegated calendar access.
///
/// Maps to `CalendarWebService.GetInstitutionProfilesWithDelegatedAccesses()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getInstitutionProfilesWithDelegatedAccess`
pub async fn get_institution_profiles_with_delegated_accesses(
    session: &mut Session,
    inst_profile_id: Option<i64>,
) -> crate::Result<Vec<InstitutionDelegateAccessesItem>> {
    let path = match inst_profile_id {
        Some(id) => {
            format!("?method=calendar.getInstitutionProfilesWithDelegatedAccess&instProfileId={id}")
        }
        None => "?method=calendar.getInstitutionProfilesWithDelegatedAccess".to_string(),
    };
    session.get(&path).await
}

/// Get birthdays for a group.
///
/// Maps to `CalendarWebService.GetBirthdaysForGroup()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getBirthdayEventsForGroup`
pub async fn get_birthdays_for_group(
    session: &mut Session,
    group_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<BirthdayEventDto>> {
    session
        .get(&format!(
            "?method=calendar.getBirthdayEventsForGroup&groupId={group_id}&start={}&end={}",
            encode_value(start),
            encode_value(end)
        ))
        .await
}

/// Get birthdays for an institution.
///
/// Maps to `CalendarWebService.GetBirthdaysForInstitution()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getBirthdayEventsForInstitutions`
pub async fn get_birthdays_for_institution(
    session: &mut Session,
    institution_id: i64,
    start: &str,
    end: &str,
) -> crate::Result<Vec<BirthdayEventDto>> {
    session
        .get(&format!(
            "?method=calendar.getBirthdayEventsForInstitutions&institutionId={institution_id}&start={}&end={}",
            encode_value(start),
            encode_value(end)
        ))
        .await
}

/// Get the top important dates (shown on dashboard).
///
/// Maps to `CalendarWebService.GetTopImportantDate()`.
///
/// # Endpoint
///
/// `GET ?method=calendar.getImportantDates`
pub async fn get_top_important_date(
    session: &mut Session,
    inst_profile_ids: &[i64],
) -> crate::Result<Vec<ImportantDateItem>> {
    let mut query = Vec::new();
    for id in inst_profile_ids {
        query.push(param_num("instProfileIds", id));
    }
    let path = if query.is_empty() {
        "?method=calendar.getImportantDates".to_string()
    } else {
        format!("?method=calendar.getImportantDates&{}", query.join("&"))
    };
    session.get(&path).await
}

/// Check scheduling conflicts for attendees.
///
/// Maps to `CalendarWebService.CheckConflictEventForAttendees()`.
///
/// # Endpoint
///
/// `POST ?method=calendar.checkConflictEventForAttendees`
pub async fn check_conflict_event_for_attendees(
    session: &mut Session,
    args: &CheckEventConflictInput,
) -> crate::Result<Vec<ConflictEventItem>> {
    session
        .post("?method=calendar.checkConflictEventForAttendees", args)
        .await
}

/// Check whether calendar feed is enabled for a municipality.
///
/// Maps to `CalendarWebService.GetIsCalendarFeedEnabledForMunicipality()`.
///
/// # Endpoint
///
/// `GET ?method=MunicipalConfiguration.getCalendarFeedEnabled`
pub async fn get_is_calendar_feed_enabled_for_municipality(
    session: &mut Session,
    municipality_id: i64,
) -> crate::Result<CalendarSynchronisationMunicipalityFeedModel> {
    session
        .get(&format!(
            "?method=MunicipalConfiguration.getCalendarFeedEnabled&municipalityId={municipality_id}"
        ))
        .await
}

/// Get a feed configuration by ID.
///
/// Maps to `CalendarWebService.GetFeedConfigurationById()`.
///
/// # Endpoint
///
/// `GET ?method=CalendarFeed.getFeedConfigurationById`
pub async fn get_feed_configuration_by_id(
    session: &mut Session,
    config_id: i64,
) -> crate::Result<CalendarSynchronisationConfigurationItem> {
    session
        .get(&format!(
            "?method=CalendarFeed.getFeedConfigurationById&configId={config_id}"
        ))
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
