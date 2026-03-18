//! Calendar and event domain models.
//!
//! These types represent the calendar system in Aula, covering events (basic,
//! series, timeslot, lesson), repeating events, timeslot booking for
//! parent-teacher meetings, vacation registrations, calendar sync, delegated
//! access, and important dates.
//!
//! See `data_models.md` Models.Calendar namespace and `domain_concepts.md`
//! Section 1.4.

use serde::{Deserialize, Serialize};

use crate::enums::calendar::{
    CalendarItemType, EventType, MyCalendarItemType, ParticipantRole, RelationMode, RepeatType,
    ResponseType, VacationResponseStatusEnum,
};
use crate::enums::common::ResourceType;
use crate::enums::profiles::{InstitutionRole, PortalRole};

use super::groups::SimpleGroupDto;
use super::messaging::RichTextWrapperDto;
use super::profiles::{ChildMetadata, InstitutionProfile, ProfilePictureDto};

// ---------------------------------------------------------------------------
// Shared / HTML value types
// ---------------------------------------------------------------------------

/// Simple HTML content wrapper (used for lesson notes, descriptions).
///
/// Maps to `DTOs.Calendar.HtmlDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlDto {
    pub html: Option<String>,
}

/// File attachment result DTO (placeholder for full file model).
///
/// Maps to `Models.Common.Api.Files.AulaFileResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AulaFileResultDto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub url: Option<String>,
}

// ---------------------------------------------------------------------------
// Core event types
// ---------------------------------------------------------------------------

/// Resource category for event resources.
///
/// Maps to `EventBaseClass.EventResource.ResourceCategory`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventResourceCategory {
    pub resource_type: Option<ResourceType>,
}

/// Resource attached to a calendar event (room, equipment, etc.).
///
/// Maps to `EventBaseClass.EventResource`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventResource {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub category: Option<EventResourceCategory>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub short_name: Option<String>,
}

/// Group with invited portal roles on an event.
///
/// Maps to `Models.Calendar.Event.EventGroupWithRolesDTO`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventGroupWithRolesDto {
    pub group: Option<EventGroup>,
    pub invited_portal_roles: Option<Vec<String>>,
}

/// Repeating event pattern information.
///
/// Maps to `Models.Calendar.Event.RepeatingEventDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepeatingEventDto {
    pub pattern: Option<String>,
    pub occurence_limit: Option<i32>,
    pub interval: Option<i32>,
    pub day_in_month: Option<i32>,
    pub max_date: Option<String>,
    pub weekday_mask: Option<Vec<bool>>,
    pub original_start_date_time: Option<String>,
    pub original_end_date_time: Option<String>,
    pub last_occurrence_date: Option<String>,
}

/// Base class for all calendar events.
///
/// Maps to `Models.Calendar.Event.EventBaseClass`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBaseClass {
    pub id: Option<i32>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub institution_code: Option<String>,
    pub all_day: Option<bool>,
    pub old_all_day: Option<bool>,
    #[serde(default)]
    pub added_to_institution_calendar: bool,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    #[serde(default)]
    pub is_deadline_exceeded: bool,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub private: Option<bool>,
    pub response_required: Option<bool>,
    pub belongs_to_profiles: Option<Vec<i64>>,
    pub belongs_to_resources: Option<Vec<i64>>,
    pub security_level: Option<i32>,
    #[serde(default)]
    pub is_deleted: bool,
    pub old_start_date_time: Option<String>,
    pub old_end_date_time: Option<String>,
    pub invitee_groups: Option<Vec<EventGroupWithRolesDto>>,
    pub invited_groups: Option<Vec<SimpleGroupDto>>,
    pub primary_resource_text: Option<String>,
    pub primary_resource: Option<EventResource>,
    pub additional_resources: Option<Vec<EventResource>>,
    pub additional_resource_text: Option<String>,
    pub repeating: Option<RepeatingEventDto>,
    pub response_status: Option<ResponseType>,
    #[serde(default)]
    pub directly_related: bool,
    pub maximum_number_of_participants: Option<i64>,
    pub actual_number_of_participants: Option<i64>,
    pub occurrence_date_time: Option<String>,
}

/// Creator information on an event detail view.
///
/// Maps to `Models.Calendar.Event.EventDetailsCreatorDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDetailsCreatorDto {
    pub institution_profile_id: Option<i64>,
    pub profile_id: Option<i64>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<PortalRole>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub institution_role: Option<InstitutionRole>,
}

/// Detailed event view (extends EventBaseClass conceptually).
///
/// Maps to `Models.Calendar.Event.EventDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDetailsDto {
    // -- base fields (from EventBaseClass) --
    pub id: Option<i32>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub institution_code: Option<String>,
    pub all_day: Option<bool>,
    pub old_all_day: Option<bool>,
    #[serde(default)]
    pub added_to_institution_calendar: bool,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    #[serde(default)]
    pub is_deadline_exceeded: bool,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub private: Option<bool>,
    pub response_required: Option<bool>,
    pub belongs_to_profiles: Option<Vec<i64>>,
    pub belongs_to_resources: Option<Vec<i64>>,
    pub security_level: Option<i32>,
    #[serde(default)]
    pub is_deleted: bool,
    pub old_start_date_time: Option<String>,
    pub old_end_date_time: Option<String>,
    pub invitee_groups: Option<Vec<EventGroupWithRolesDto>>,
    pub invited_groups: Option<Vec<SimpleGroupDto>>,
    pub primary_resource_text: Option<String>,
    pub primary_resource: Option<EventResource>,
    pub additional_resources: Option<Vec<EventResource>>,
    pub additional_resource_text: Option<String>,
    pub repeating: Option<RepeatingEventDto>,
    pub response_status: Option<ResponseType>,
    #[serde(default)]
    pub directly_related: bool,
    pub maximum_number_of_participants: Option<i64>,
    pub actual_number_of_participants: Option<i64>,
    pub occurrence_date_time: Option<String>,

    // -- detail-specific fields --
    pub attachments: Option<Vec<AulaFileResultDto>>,
    pub invitees: Option<Vec<EventProfile>>,
    pub co_organizers: Option<Vec<EventProfile>>,
    pub invited_group_home_children: Option<Vec<InvitedGroupHome>>,
    pub description: Option<RichTextWrapperDto>,
    pub lesson: Option<Lesson>,
    pub creator: Option<EventDetailsCreatorDto>,
    pub vacation_registration: Option<VacationRegistrationDetailsResultDto>,
    pub time_slot: Option<TimeslotEventDto>,
    pub can_edit_start_date: Option<bool>,
    pub can_answer_for_series: Option<bool>,
    #[serde(default)]
    pub do_request_number_of_participants: bool,
    pub last_reminder_date_time: Option<String>,
}

/// Summary event DTO (list views). Extends EventBaseClass conceptually.
///
/// Maps to `Models.Calendar.Event.EventSimpleDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSimpleDto {
    // -- base fields (from EventBaseClass) --
    pub id: Option<i32>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub institution_code: Option<String>,
    pub all_day: Option<bool>,
    #[serde(default)]
    pub added_to_institution_calendar: bool,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    #[serde(default)]
    pub is_deadline_exceeded: bool,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub private: Option<bool>,
    pub response_required: Option<bool>,
    pub belongs_to_profiles: Option<Vec<i64>>,
    pub belongs_to_resources: Option<Vec<i64>>,
    pub security_level: Option<i32>,
    #[serde(default)]
    pub is_deleted: bool,
    pub old_start_date_time: Option<String>,
    pub old_end_date_time: Option<String>,
    pub invitee_groups: Option<Vec<EventGroupWithRolesDto>>,
    pub invited_groups: Option<Vec<SimpleGroupDto>>,
    pub primary_resource_text: Option<String>,
    pub primary_resource: Option<EventResource>,
    pub additional_resources: Option<Vec<EventResource>>,
    pub additional_resource_text: Option<String>,
    pub repeating: Option<RepeatingEventDto>,
    pub response_status: Option<ResponseType>,
    #[serde(default)]
    pub directly_related: bool,
    pub maximum_number_of_participants: Option<i64>,
    pub actual_number_of_participants: Option<i64>,
    pub occurrence_date_time: Option<String>,

    // -- simple-specific fields --
    pub has_attachments: Option<bool>,
    pub lesson: Option<LessonSimple>,
    pub vacation_children_count_by_dates: Option<Vec<VacationRegistrationChildrenCountByDates>>,
    pub creator_aula_name: Option<String>,
    pub creator_profile_id: Option<i64>,
    pub creator_inst_profile_id: Option<i64>,
    pub time_slot: Option<TimeslotEventSimpleDto>,
}

// ---------------------------------------------------------------------------
// Event profile types
// ---------------------------------------------------------------------------

/// Profile details within an event context.
///
/// Maps to `Models.Calendar.Event.EventProfile.EventProfileDetails`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventProfileDetails {
    pub email: Option<String>,
    pub administrator: Option<serde_json::Value>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub metadata: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    #[serde(default)]
    pub can_remove_blocking_or_response_for_time_slot: bool,
    pub profile_id: Option<i32>,
    pub institution_profile_id: Option<i64>,
    pub profile_picture_url: Option<String>,
}

/// Participant profile on an event (with response info).
///
/// Maps to `Models.Calendar.Event.EventProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventProfile {
    pub inst_profile: Option<EventProfileDetails>,
    pub response_type: Option<ResponseType>,
    pub response_date_time: Option<String>,
    pub number_of_adult_participants: Option<i32>,
    pub number_of_child_participants: Option<i32>,
}

/// Group associated with an event.
///
/// Maps to `Models.Calendar.Event.EventGroup`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventGroup {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub group_type: Option<String>,
    pub access: Option<String>,
    pub status: Option<String>,
    #[serde(default)]
    pub dashboard_enabled: bool,
    pub institution_code: Option<String>,
}

/// Group-home child invited to an event.
///
/// Maps to `Models.Calendar.Event.InvitedGroupHome`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitedGroupHome {
    pub otp_inbox_id: Option<i64>,
    pub group_home_id: Option<i64>,
    pub regarding_child_id: Option<i64>,
    pub regarding_child_display_name: Option<String>,
    pub regarding_child_meta_data: Option<String>,
    pub group_home_name: Option<String>,
    pub response_type: Option<ResponseType>,
    pub response_date_time: Option<String>,
    pub number_of_adult_participants: Option<i32>,
    pub number_of_child_participants: Option<i32>,
}

// ---------------------------------------------------------------------------
// Timeslot types (parent-teacher meeting booking)
// ---------------------------------------------------------------------------

/// Base timeslot event DTO.
///
/// Maps to `Models.Calendar.Event.BaseTimeslotEventDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseTimeslotEventDto {
    pub child_required: Option<bool>,
}

/// Full timeslot event (detail view). Extends BaseTimeslotEventDto.
///
/// Maps to `Models.Calendar.Event.TimeslotEventDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeslotEventDto {
    // -- base --
    pub child_required: Option<bool>,

    // -- extended --
    pub meetings_between_breaks: Option<i32>,
    pub break_length: Option<i32>,
    pub meeting_duration: Option<i32>,
    #[serde(default)]
    pub can_update_response_to_event: bool,
    pub time_slots: Option<Vec<TimeSlot>>,
    pub number_of_participants_per_time_slot: Option<i32>,
}

/// Simplified timeslot event (list view). Extends BaseTimeslotEventDto.
///
/// Maps to `Models.Calendar.Event.TimeslotEventSimpleDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeslotEventSimpleDto {
    // -- base --
    pub child_required: Option<bool>,

    // -- extended --
    pub time_slots: Option<Vec<TimeSlotSimpleDto>>,
}

/// Time index within a timeslot (a specific bookable slot).
///
/// Maps to `Models.Calendar.TimeSlotIndex`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSlotIndex {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// Base time slot DTO (shared between TimeSlot and TimeSlotSimpleDto).
///
/// Maps to `Models.Calendar.BaseTimeSlotDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseTimeSlotDto {
    pub id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub time_slot_indexes: Option<Vec<TimeSlotIndex>>,
}

/// Resource attached to a timeslot.
///
/// Reuses `EventResource` — referenced as `Resource` in the .NET model.
pub type TimeSlotResource = EventResource;

/// Base answer to a timeslot booking.
///
/// Maps to `Models.Calendar.BaseTimeSlotAnswer`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseTimeSlotAnswer {
    pub id: Option<i32>,
    pub selected_time_slot_index: Option<i32>,
}

/// Full timeslot answer (with profile details).
///
/// Maps to `Models.Calendar.TimeSlotAnswer`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSlotAnswer {
    // -- base --
    pub id: Option<i32>,
    pub selected_time_slot_index: Option<i32>,

    // -- extended --
    pub concerning_profile: Option<EventProfileDetails>,
    pub inst_profile: Option<EventProfileDetails>,
    #[serde(default)]
    pub can_remove_blocking_or_response_for_time_slot: bool,
}

/// Simplified timeslot answer (IDs only).
///
/// Maps to `Models.Calendar.TimeSlotAnswerSimpleDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSlotAnswerSimpleDto {
    // -- base --
    pub id: Option<i32>,
    pub selected_time_slot_index: Option<i32>,

    // -- extended --
    pub concerning_profile_id: Option<i32>,
    pub inst_profile_id: Option<i32>,
    #[serde(default)]
    pub can_remove_blocking_or_response_for_time_slot: bool,
}

/// Full timeslot (detail view). Extends BaseTimeSlotDto.
///
/// Maps to `Models.Calendar.TimeSlot`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSlot {
    // -- base --
    pub id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub time_slot_indexes: Option<Vec<TimeSlotIndex>>,

    // -- extended --
    pub answers: Option<Vec<TimeSlotAnswer>>,
    pub primary_resource: Option<TimeSlotResource>,
    pub primary_resource_text: Option<String>,
}

/// Simplified timeslot (list view). Extends BaseTimeSlotDto.
///
/// Maps to `Models.Calendar.TimeSlotSimpleDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSlotSimpleDto {
    // -- base --
    pub id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub time_slot_indexes: Option<Vec<TimeSlotIndex>>,

    // -- extended --
    pub answers: Option<Vec<TimeSlotAnswerSimpleDto>>,
    pub belongs_to_resource: Option<i64>,
}

// ---------------------------------------------------------------------------
// Lesson types
// ---------------------------------------------------------------------------

/// Base lesson fields shared between detail and simple views.
///
/// Maps to `Models.Calendar.Event.LessonBase`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonBase {
    pub lesson_id: Option<String>,
    pub lesson_status: Option<String>,
}

/// Lesson participant with role and teacher info.
///
/// Maps to `Models.Calendar.Event.LessonParticipant`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonParticipant {
    pub participant_profile: Option<InstitutionProfile>,
    pub participant_role: Option<ParticipantRole>,
    pub teacher_name: Option<String>,
    pub teacher_initials: Option<String>,
}

/// Full lesson detail (with notes and full participant info).
///
/// Maps to `Models.Calendar.Event.Lesson`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lesson {
    // -- base --
    pub lesson_id: Option<String>,
    pub lesson_status: Option<String>,

    // -- extended --
    pub participants: Option<Vec<LessonParticipant>>,
    pub note_to_class: Option<HtmlDto>,
    pub note_to_substitute: Option<HtmlDto>,
    pub note_to_teacher: Option<HtmlDto>,
}

/// Simplified participant info (list views).
///
/// Maps to `Models.Calendar.Event.ParticipantSimple`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantSimple {
    pub teacher_initials: Option<String>,
    pub teacher_name: Option<String>,
    pub participant_role: Option<ParticipantRole>,
}

/// Simplified lesson (list views).
///
/// Maps to `Models.Calendar.Event.LessonSimple`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonSimple {
    // -- base --
    pub lesson_id: Option<String>,
    pub lesson_status: Option<String>,

    // -- extended --
    #[serde(default)]
    pub has_relevant_note: bool,
    pub participants: Option<Vec<ParticipantSimple>>,
}

// ---------------------------------------------------------------------------
// Delegated access
// ---------------------------------------------------------------------------

/// Institution info for delegate access context.
///
/// Maps to `Models.Calendar.Event.DelegateAccessInstitution`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateAccessInstitution {
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
}

/// A single delegate access item (who has been given access).
///
/// Maps to `Models.Calendar.Event.DelegateAccessesItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateAccessesItem {
    pub inst_profile_id: Option<i64>,
    pub profile_id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub meta_data: Option<String>,
}

/// Delegated calendar access configuration.
///
/// Maps to `Models.Calendar.Event.DelegateAccesses`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateAccesses {
    pub owner_inst_profile_id: Option<i64>,
    pub delegated_inst_profiles: Option<Vec<DelegateAccessesItem>>,
}

/// Input for setting delegated access.
///
/// Maps to `Models.Calendar.Event.DelegateAccessesInput`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateAccessesInput {
    pub owner_inst_profile_id: Option<i64>,
    pub delegated_inst_profile_ids: Option<Vec<i64>>,
}

/// Delegate access item with full institution info.
///
/// Maps to `Models.Calendar.Event.InstitutionDelegateAccessesItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionDelegateAccessesItem {
    pub inst_profile_id: Option<i64>,
    pub profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub meta_data: Option<String>,
    pub institution: Option<DelegateAccessInstitution>,
}

/// Delegated context result (from DTOs.Calendar).
///
/// Maps to `DTOs.Calendar.DelegatedContextResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegatedContextResultModel {
    pub full_name: Option<String>,
    pub nullable_institution_profile_id: Option<i32>,
    pub institution_code: Option<String>,
    pub profile_picture: Option<ProfilePictureDto>,
    pub role: Option<PortalRole>,
}

/// Set delegated context request.
///
/// Maps to `DTOs.Calendar.SetDelegatedContextRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDelegatedContextRequestModel {
    pub delegated_inst_profile_id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Calendar sync configuration
// ---------------------------------------------------------------------------

/// Configuration item for calendar feed sync.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.CalendarSynchronisationConfigurationItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSynchronisationConfigurationItem {
    pub institution_profile_id: Option<i32>,
    pub id: Option<i32>,
    pub calendarfeedconfigurationid: Option<i32>,
    pub owner_id: Option<i32>,
    pub regarding_id: Option<i32>,
    pub one_week_feed: Option<String>,
    pub one_year_feed: Option<String>,
    #[serde(default)]
    pub weekly: bool,
    pub filters: Option<Vec<String>>,
    pub feed_status: Option<String>,
}

/// Calendar sync policy acceptance.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.CalendarSynchronisationModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSynchronisationModel {
    #[serde(default)]
    pub policy_accepted: bool,
}

/// Municipality-level calendar feed configuration.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.CalendarSynchronisationMunicipalityFeedModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSynchronisationMunicipalityFeedModel {
    pub municipality_code: Option<String>,
    #[serde(default)]
    pub calendar_feed_enabled: bool,
}

/// Create a new calendar sync configuration.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.CreateCalendarSynchronizationConfigurationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCalendarSynchronizationConfigurationRequest {
    pub filters: Option<Vec<EventType>>,
    #[serde(default)]
    pub weekly: bool,
    pub institution_profile_id: Option<i64>,
}

/// Update an existing calendar sync configuration.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.UpdateCalendarSynchronizationConfigurationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCalendarSynchronizationConfigurationRequest {
    pub filters: Option<Vec<EventType>>,
    pub calendar_feed_configuration_id: Option<i64>,
}

/// Event types available by portal role.
///
/// Maps to `Models.Calendar.CalendarSynchronisation.GetEventTypesByPortalRoleResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventTypesByPortalRoleResultModel {
    pub event_types: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Birthday events
// ---------------------------------------------------------------------------

/// Birthday event DTO.
///
/// Maps to `Models.Calendar.BirthdayEventDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthdayEventDto {
    pub birthday: Option<String>,
    pub name: Option<String>,
    pub institution_code: Option<String>,
    pub institution_profile_id: Option<i64>,
    pub main_group_name: Option<String>,
}

/// Request to get birthday events.
///
/// Maps to `Models.Calendar.Birthday.GetBirthdayEvents`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBirthdayEvents {
    pub start: Option<String>,
    pub end: Option<String>,
    pub inst_codes: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Important dates
// ---------------------------------------------------------------------------

/// Profile on an important date invitee.
///
/// Maps to `Models.Calendar.ImportantDate.ImportantDateItemProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportantDateItemProfile {
    pub id: Option<i64>,
    pub profile_id: Option<i32>,
    pub role: Option<String>,
    pub relations: Option<Vec<ImportantDateItemProfile>>,
}

/// Invitee on an important date.
///
/// Maps to `Models.Calendar.ImportantDate.ImportantDateItemInvitee`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportantDateItemInvitee {
    pub inst_profile: Option<ImportantDateItemProfile>,
    pub response_type: Option<String>,
}

/// An important date item (upcoming events shown on dashboard).
///
/// Maps to `Models.Calendar.ImportantDate.ImportantDateItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportantDateItem {
    pub id: Option<i64>,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub invitees: Option<Vec<ImportantDateItemInvitee>>,
    pub institution_name: Option<String>,
    #[serde(default)]
    pub all_day: bool,
}

// ---------------------------------------------------------------------------
// My Calendar
// ---------------------------------------------------------------------------

/// My calendar item.
///
/// Maps to `Models.Calendar.MyCalendar.MyCalendarItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyCalendarItem {
    #[serde(rename = "type")]
    pub item_type: Option<MyCalendarItemType>,
    pub my_calendar_view_model: Option<serde_json::Value>,
    pub title: Option<String>,
    pub id: Option<i64>,
    pub birth_day: Option<String>,
    pub name: Option<String>,
    pub group_name: Option<String>,
}

/// Calendar event item (event list / calendar views).
///
/// Maps to `Models.Calendar.Event.EventItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventItem {
    #[serde(rename = "type")]
    pub item_type: Option<CalendarItemType>,
    pub event_view_model: Option<serde_json::Value>,
    pub title: Option<String>,
    pub id: Option<i64>,
    pub date_time: Option<String>,
}

// ---------------------------------------------------------------------------
// Daily aggregated events
// ---------------------------------------------------------------------------

/// Aggregated event count by type.
///
/// Maps to `Models.Calendar.DailyAggregatedEvents.AggregatedEventsGroupByType`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedEventsGroupByType {
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub count: Option<i32>,
}

/// Daily aggregated events result.
///
/// Maps to `Models.Calendar.DailyAggregatedEvents.DailyAggregatedEventsResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyAggregatedEventsResultModel {
    pub date: Option<String>,
    pub aggregated_events: Option<Vec<AggregatedEventsGroupByType>>,
}

/// Daily event count result.
///
/// Maps to `Models.Calendar.DailyAggregatedEvents.DailyEventCountResultModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyEventCountResultModel {
    pub date: Option<String>,
    pub count: Option<i32>,
}

/// Request for daily group event counts.
///
/// Maps to `Models.Calendar.DailyAggregatedEvents.DailyGroupEventCountRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyGroupEventCountRequestModel {
    pub group_id: Option<i64>,
    pub start: Option<String>,
    pub end: Option<String>,
}

// ---------------------------------------------------------------------------
// Conflict checking
// ---------------------------------------------------------------------------

/// Input for checking event time conflicts.
///
/// Maps to `Models.Calendar.Event.CheckEventConflictInput`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckEventConflictInput {
    pub start: Option<String>,
    pub end: Option<String>,
    #[serde(default)]
    pub all_day: bool,
    pub institution_profile_ids: Option<Vec<i64>>,
    pub exclude_event_id: Option<i64>,
}

/// A profile that conflicts with a proposed event time.
///
/// Maps to `Models.Calendar.Event.ConflictEventItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConflictEventItem {
    pub profile_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Resource conflict result.
///
/// Maps to `Models.Calendar.CalendarResourceConflict`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarResourceConflict {
    pub unavailable_resource_ids: Option<Vec<i64>>,
}

/// Communication event spinner item (for event type selection UI).
///
/// Maps to `Models.Calendar.Event.CommunicationEventSpinnerItem`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationEventSpinnerItem {
    pub display_name: Option<String>,
    pub value: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Conversation meeting
// ---------------------------------------------------------------------------

/// Update input for a conversation meeting.
///
/// Maps to `Models.Calendar.ConversationMeeting.ConversationMeetingUpdateInput`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationMeetingUpdateInput {
    pub event_id: Option<i64>,
    pub child_id: Option<i64>,
}

/// Result after creating a conversation meeting.
///
/// Maps to `Models.Calendar.ConversationMeeting.CreateConversationMeetingProcessedResult`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConversationMeetingProcessedResult {
    pub event_id: Option<i64>,
    pub resource_conflict: Option<CalendarResourceConflict>,
}

// ---------------------------------------------------------------------------
// Event CRUD request types
// ---------------------------------------------------------------------------

/// Group invitation request (group ID + portal roles).
///
/// Maps to `Models.Calendar.CreateEvent.InviteeGroupRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteeGroupRequest {
    pub group_id: Option<i64>,
    pub portal_roles: Option<Vec<String>>,
}

/// Base fields for event creation requests.
///
/// Maps to `Models.Calendar.CreateEvent.CreateBaseEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBaseEventRequest {
    pub event_id: Option<i64>,
    pub from_inst_profile_id: Option<i64>,
    pub title: Option<String>,
    pub event_type_enum: Option<EventType>,
    pub description: Option<String>,
    pub invitee_ids: Option<Vec<i64>>,
    pub invitee_groups: Option<Vec<InviteeGroupRequest>>,
    pub invited_group_ids: Option<Vec<i64>>,
    pub co_organizer_ids: Option<Vec<i64>>,
    pub invited_otp_inbox_ids: Option<Vec<i64>>,
    pub attachment_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    pub institution_code: Option<String>,
}

/// Create a simple (non-repeating, non-timeslot) event.
///
/// Maps to `Models.Calendar.CreateEvent.SimpleEvent.CreateSimpleEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSimpleEventRequest {
    // -- base (from CreateBaseEventRequest) --
    pub event_id: Option<i64>,
    pub from_inst_profile_id: Option<i64>,
    pub title: Option<String>,
    pub event_type_enum: Option<EventType>,
    pub description: Option<String>,
    pub invitee_ids: Option<Vec<i64>>,
    pub invitee_groups: Option<Vec<InviteeGroupRequest>>,
    pub invited_group_ids: Option<Vec<i64>>,
    pub co_organizer_ids: Option<Vec<i64>>,
    pub invited_otp_inbox_ids: Option<Vec<i64>>,
    pub attachment_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    pub institution_code: Option<String>,

    // -- simple event fields --
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    #[serde(default)]
    pub all_day: bool,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub response_required: bool,
    pub primary_resource_id: Option<i64>,
    pub primary_resource_text: Option<String>,
    pub additional_resource_ids: Option<Vec<i64>>,
    pub additional_resource_text: Option<String>,
    #[serde(default)]
    pub add_to_institution_calendar: bool,
    #[serde(default)]
    pub added_to_institution_calendar: bool,
    pub maximum_number_of_participants: Option<i64>,
    #[serde(default)]
    pub do_request_number_of_participants: bool,
}

/// Create a repeating event.
///
/// Maps to `Models.Calendar.CreateEvent.RepeatingEvent.CreateRepeatingEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRepeatingEventRequest {
    // -- base (from CreateSimpleEventRequest / CreateBaseEventRequest) --
    pub event_id: Option<i64>,
    pub from_inst_profile_id: Option<i64>,
    pub title: Option<String>,
    pub event_type_enum: Option<EventType>,
    pub description: Option<String>,
    pub invitee_ids: Option<Vec<i64>>,
    pub invitee_groups: Option<Vec<InviteeGroupRequest>>,
    pub invited_group_ids: Option<Vec<i64>>,
    pub co_organizer_ids: Option<Vec<i64>>,
    pub invited_otp_inbox_ids: Option<Vec<i64>>,
    pub attachment_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    pub institution_code: Option<String>,
    pub start_date_time: Option<String>,
    pub end_date_time: Option<String>,
    #[serde(default)]
    pub all_day: bool,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub response_required: bool,
    pub primary_resource_id: Option<i64>,
    pub primary_resource_text: Option<String>,
    pub additional_resource_ids: Option<Vec<i64>>,
    pub additional_resource_text: Option<String>,
    #[serde(default)]
    pub add_to_institution_calendar: bool,
    #[serde(default)]
    pub added_to_institution_calendar: bool,
    pub maximum_number_of_participants: Option<i64>,
    #[serde(default)]
    pub do_request_number_of_participants: bool,

    // -- repeating fields --
    pub occurence_limit: Option<i32>,
    pub weekday_mask: Option<Vec<bool>>,
    pub day_in_month: Option<i32>,
    pub repeat_type_enum: Option<RepeatType>,
    pub interval: Option<i32>,
    pub max_date: Option<String>,
    pub occurrence_date_time: Option<String>,
}

/// Timeslot DTO for timeslot event creation.
///
/// Maps to `Models.Calendar.CreateEvent.TimeslotEvent.CreateTimeslotEventTimeSlotDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeslotEventTimeSlotDto {
    pub id: Option<i64>,
    pub primary_resource_id: Option<i64>,
    pub primary_resource_text: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Create a timeslot event (parent-teacher meeting).
///
/// Maps to `Models.Calendar.CreateEvent.TimeslotEvent.CreateTimeslotEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimeslotEventRequest {
    // -- base (from CreateBaseEventRequest) --
    pub event_id: Option<i64>,
    pub from_inst_profile_id: Option<i64>,
    pub title: Option<String>,
    pub event_type_enum: Option<EventType>,
    pub description: Option<String>,
    pub invitee_ids: Option<Vec<i64>>,
    pub invitee_groups: Option<Vec<InviteeGroupRequest>>,
    pub invited_group_ids: Option<Vec<i64>>,
    pub co_organizer_ids: Option<Vec<i64>>,
    pub invited_otp_inbox_ids: Option<Vec<i64>>,
    pub attachment_ids: Option<Vec<i64>>,
    #[serde(default)]
    pub hide_in_own_calendar: bool,
    pub response_deadline: Option<String>,
    pub institution_code: Option<String>,

    // -- timeslot fields --
    pub time_slots: Option<Vec<CreateTimeslotEventTimeSlotDto>>,
    pub break_length: Option<i32>,
    pub meeting_duration: Option<i32>,
    #[serde(default)]
    pub child_required: bool,
    pub meetings_between_breaks: Option<i32>,
    #[serde(default)]
    pub add_to_institution_calendar: bool,
    pub number_of_participants_per_time_slot: Option<i32>,
}

/// Resource for event creation.
///
/// Maps to `Models.Calendar.CreateEvent.Resources.CreateEventResource`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResource {
    pub id: Option<i64>,
    pub institution_code: Option<String>,
    pub institution_name: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub display_name: Option<String>,
    pub resource_category: Option<String>,
    pub number_of_available_occurrences: Option<i32>,
    pub number_of_occurrences: Option<i32>,
}

/// Profile for event resource creation.
///
/// Maps to `Models.Calendar.CreateEvent.Resources.CreateEventResourceProfile`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResourceProfile {
    pub profile_id: Option<i64>,
    pub name: Option<String>,
    pub institution_code: Option<String>,
}

/// Update a lesson.
///
/// Maps to `Models.Calendar.CreateEvent.Lesson.UpdateLessonRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLessonRequest {
    pub event_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub note_to_class: Option<String>,
    pub note_to_teacher: Option<String>,
    pub note_to_substitute: Option<String>,
    pub additional_resource_ids: Option<Vec<i64>>,
    pub additional_resource_text: Option<String>,
    pub attachment_ids: Option<Vec<i64>>,
}

// ---------------------------------------------------------------------------
// Event response/respond types
// ---------------------------------------------------------------------------

/// Respond to a simple event invitation.
///
/// Maps to `Models.Calendar.RespondEvent.RespondSimpleEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespondSimpleEventRequest {
    pub event_id: Option<i64>,
    pub institution_profile_id: Option<i64>,
    pub invited_inst_profile_id: Option<i64>,
    pub response_type: Option<ResponseType>,
    pub occurrence_date_time: Option<String>,
    pub number_of_adult_participants: Option<i32>,
    pub number_of_child_participants: Option<i32>,
}

/// Respond to a timeslot event (book a timeslot).
///
/// Maps to `Models.Calendar.RespondEvent.RespondTimeslotEventRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespondTimeslotEventRequest {
    pub event_id: Option<i64>,
    pub response_type_enum: Option<ResponseType>,
    pub time_slot_id: Option<i64>,
    pub time_slot_index: Option<i32>,
    pub institution_profile_id: Option<i64>,
    pub concerning_inst_profile_id: Option<i64>,
    #[serde(default)]
    pub on_behalf_of: bool,
}

/// Block a timeslot (prevent booking).
///
/// Maps to `Models.Calendar.RespondEvent.BlockTimeSlotRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTimeSlotRequest {
    pub event_id: Option<i64>,
    pub time_slot_id: Option<i64>,
    pub time_slot_index: Option<i32>,
}

/// Delete a timeslot booking.
///
/// Maps to `Models.Calendar.RespondEvent.DeleteTimeslotRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTimeslotRequest {
    pub event_id: Option<i64>,
    pub time_slot_id: Option<i64>,
    pub time_slot_index: Option<i32>,
    pub concerning_institution_profile_id: Option<i32>,
}

/// Send an event reminder.
///
/// Maps to `Models.Calendar.SendEventReminderRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEventReminderRequest {
    pub event_id: Option<i64>,
    pub message: Option<String>,
}

/// Relations message for calendar event views.
///
/// Maps to `Models.Calendar.RelationsMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationsMessage {
    pub profile_id: Option<i64>,
    #[serde(default)]
    pub is_selected: bool,
    pub relation_mode: Option<RelationMode>,
}

// ---------------------------------------------------------------------------
// Event query parameters
// ---------------------------------------------------------------------------

/// Parameters for getting events.
///
/// Maps to `Models.Calendar.GetEventsParameters`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsParameters {
    pub inst_profile_ids: Option<Vec<i64>>,
    pub resource_ids: Option<Vec<i64>>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub specific_types: Option<Vec<String>>,
    pub school_calendar_institution_codes: Option<Vec<String>>,
}

/// Parameters for getting institution events.
///
/// Maps to `Models.Calendar.GetEventsForInstitutionRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsForInstitutionRequestModel {
    pub start: Option<String>,
    pub end: Option<String>,
    pub inst_codes: Option<Vec<String>>,
}

/// Request to get event types for filtering.
///
/// Maps to `DTOs.Calendar.GetEventTypeRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventTypeRequestDto {
    pub filter_institution_codes: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Vacation types
// ---------------------------------------------------------------------------

/// Children count by date for vacation registration overview.
///
/// Maps to `DTOs.Calendar.Vacation.VacationRegistrationChildrenCountByDates`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistrationChildrenCountByDates {
    pub date: Option<String>,
    pub children_are_coming: Option<i32>,
    pub total: Option<i32>,
}

/// Vacation registration details (embedded in EventDetailsDto).
///
/// Maps to `DTOs.Calendar.Vacation.VacationRegistrationDetailsResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationRegistrationDetailsResultDto {
    pub children_total: Option<i32>,
    pub children_pending_answers: Option<Vec<ChildMetadata>>,
    pub vacation_children_count_by_dates: Option<Vec<VacationRegistrationChildrenCountByDates>>,
    pub note_to_guardians: Option<String>,
    // Departments use a presence filter model -- represented as generic JSON
    // until the presence domain models are implemented.
    pub departments: Option<Vec<serde_json::Value>>,
}

/// Child info in a vacation day.
///
/// Maps to `DTOs.Calendar.Vacation.VacationChildrenDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationChildrenDto {
    pub child: Option<ChildMetadata>,
    pub status: Option<VacationResponseStatusEnum>,
    pub vacation_registration_response_id: Option<i64>,
}

/// A single vacation day with children responses.
///
/// Maps to `DTOs.Calendar.Vacation.VacationDayDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationDayDto {
    pub date: Option<String>,
    pub children: Option<Vec<VacationChildrenDto>>,
}

/// Vacation week overview.
///
/// Maps to `DTOs.Calendar.Vacation.VacationWeekResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationWeekResultDto {
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub week_number: Option<i32>,
    pub vacation_days: Option<Vec<VacationDayDto>>,
}

/// Vacation overview list item.
///
/// Maps to `DTOs.Calendar.Vacation.VacationOverviewListItemResultDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationOverviewListItemResultDto {
    pub title: Option<String>,
    pub id: Option<i64>,
    pub institution_name: Option<String>,
}

/// Request to list vacation overviews.
///
/// Maps to `DTOs.Calendar.Vacation.VacationOverviewListRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationOverviewListRequestDto {
    pub filter_institution_calendar_codes: Option<Vec<String>>,
}

/// Check if vacation request has been answered.
///
/// Maps to `DTOs.Calendar.Vacation.CheckVacationRequestAnsweredRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckVacationRequestAnsweredRequestModel {
    pub vacation_registration_response_id: Option<i64>,
}

/// Get vacation request response details.
///
/// Maps to `DTOs.Calendar.Vacation.GetVacationRequestResponseRequestModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVacationRequestResponseRequestModel {
    pub vacation_request_id: Option<i64>,
    pub filter_department_group_ids: Option<Vec<i64>>,
    pub filter_department_filtering_group_ids: Option<Vec<i64>>,
}

/// Respond to a vacation registration request.
///
/// Maps to `DTOs.Calendar.Vacation.RespondToVacationRegistrationRequestRequestDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RespondToVacationRegistrationRequestDto {
    pub child_id: Option<i64>,
    pub vacation_registration_response_id: Option<i64>,
    pub days: Option<Vec<GuardianRegisterVacationIntervals>>,
    pub comment: Option<String>,
}

/// Vacation day intervals (guardian registration).
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

/// Vacation details DTO (extends EventDetailsDto conceptually).
///
/// Maps to `Models.Calendar.Vacation.VacationDetailsDto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VacationDetailsDto {
    #[serde(default)]
    pub is_vacation_created_from_vacation_request: bool,
}

/// Simple group with portal roles (used in event group context).
///
/// Maps to `Models.Calendar.Event.SimpleGroupWithRolesModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleGroupWithRolesModel {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub portal_roles: Option<Vec<PortalRole>>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_event_base_class() {
        let json = r#"{
            "id": 42,
            "title": "Parent meeting",
            "type": "Meeting",
            "institutionCode": "101001",
            "allDay": false,
            "addedToInstitutionCalendar": true,
            "hideInOwnCalendar": false,
            "responseDeadline": "2024-03-01T23:59:00",
            "isDeadlineExceeded": false,
            "startDateTime": "2024-03-15T14:00:00",
            "endDateTime": "2024-03-15T16:00:00",
            "private": false,
            "responseRequired": true,
            "belongsToProfiles": [100, 200],
            "belongsToResources": [],
            "securityLevel": 2,
            "isDeleted": false,
            "inviteeGroups": [],
            "invitedGroups": [],
            "primaryResourceText": "Room 101",
            "primaryResource": null,
            "additionalResources": [],
            "repeating": null,
            "responseStatus": "Accepted",
            "directlyRelated": true,
            "maximumNumberOfParticipants": null,
            "actualNumberOfParticipants": null,
            "occurrenceDateTime": null
        }"#;
        let event: EventBaseClass = serde_json::from_str(json).unwrap();
        assert_eq!(event.id, Some(42));
        assert_eq!(event.title.as_deref(), Some("Parent meeting"));
        assert_eq!(event.event_type.as_deref(), Some("Meeting"));
        assert!(event.added_to_institution_calendar);
        assert_eq!(event.response_status, Some(ResponseType::Accepted));
        assert!(event.directly_related);
        assert_eq!(event.belongs_to_profiles.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_event_profile() {
        let json = r#"{
            "instProfile": {
                "email": "teacher@school.dk",
                "administrator": null,
                "firstName": "Hans",
                "lastName": "Jensen",
                "fullName": "Hans Jensen",
                "shortName": "HJ",
                "metadata": null,
                "role": "Employee",
                "phone": "12345678",
                "canRemoveBlockingOrResponseForTimeSlot": true,
                "profileId": 55,
                "institutionProfileId": 100,
                "profilePictureUrl": null
            },
            "responseType": "Accepted",
            "responseDateTime": "2024-03-10T09:00:00",
            "numberOfAdultParticipants": 2,
            "numberOfChildParticipants": 1
        }"#;
        let ep: EventProfile = serde_json::from_str(json).unwrap();
        assert_eq!(ep.response_type, Some(ResponseType::Accepted));
        let profile = ep.inst_profile.unwrap();
        assert_eq!(profile.full_name.as_deref(), Some("Hans Jensen"));
        assert!(profile.can_remove_blocking_or_response_for_time_slot);
    }

    #[test]
    fn deserialize_timeslot_event_dto() {
        let json = r#"{
            "childRequired": true,
            "meetingsBetweenBreaks": 3,
            "breakLength": 10,
            "meetingDuration": 20,
            "canUpdateResponseToEvent": true,
            "timeSlots": [],
            "numberOfParticipantsPerTimeSlot": 1
        }"#;
        let ts: TimeslotEventDto = serde_json::from_str(json).unwrap();
        assert_eq!(ts.child_required, Some(true));
        assert_eq!(ts.meeting_duration, Some(20));
        assert!(ts.can_update_response_to_event);
    }

    #[test]
    fn deserialize_lesson() {
        let json = r#"{
            "lessonId": "L-123",
            "lessonStatus": "Normal",
            "participants": [],
            "noteToClass": {"html": "<p>Bring pencils</p>"},
            "noteToSubstitute": null,
            "noteToTeacher": null
        }"#;
        let lesson: Lesson = serde_json::from_str(json).unwrap();
        assert_eq!(lesson.lesson_id.as_deref(), Some("L-123"));
        let note = lesson.note_to_class.unwrap();
        assert_eq!(note.html.as_deref(), Some("<p>Bring pencils</p>"));
    }

    #[test]
    fn deserialize_delegate_accesses() {
        let json = r#"{
            "ownerInstProfileId": 100,
            "delegatedInstProfiles": [
                {
                    "instProfileId": 200,
                    "profileId": 55,
                    "name": "Colleague A",
                    "institutionCode": "101001",
                    "institutionName": "Test School",
                    "metaData": null
                }
            ]
        }"#;
        let da: DelegateAccesses = serde_json::from_str(json).unwrap();
        assert_eq!(da.owner_inst_profile_id, Some(100));
        let delegates = da.delegated_inst_profiles.unwrap();
        assert_eq!(delegates.len(), 1);
        assert_eq!(delegates[0].name.as_deref(), Some("Colleague A"));
    }

    #[test]
    fn deserialize_calendar_sync_config() {
        let json = r#"{
            "institutionProfileId": 10,
            "id": 1,
            "calendarfeedconfigurationid": 5,
            "ownerId": 10,
            "regardingId": 20,
            "oneWeekFeed": "https://feed.aula.dk/week/abc",
            "oneYearFeed": "https://feed.aula.dk/year/abc",
            "weekly": true,
            "filters": ["Event", "Meeting"],
            "feedStatus": "Active"
        }"#;
        let cfg: CalendarSynchronisationConfigurationItem = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.id, Some(1));
        assert!(cfg.weekly);
        assert_eq!(cfg.filters.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_important_date_item() {
        let json = r#"{
            "id": 7,
            "startDateTime": "2024-06-01T00:00:00",
            "endDateTime": "2024-06-01T23:59:00",
            "title": "Sports day",
            "type": "Event",
            "invitees": [],
            "institutionName": "Test School",
            "allDay": true
        }"#;
        let item: ImportantDateItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, Some(7));
        assert_eq!(item.title.as_deref(), Some("Sports day"));
        assert!(item.all_day);
    }

    #[test]
    fn deserialize_create_simple_event_request() {
        let json = r#"{
            "eventId": null,
            "fromInstProfileId": 100,
            "title": "New event",
            "eventTypeEnum": "Event",
            "description": "Test event",
            "inviteeIds": [200, 300],
            "inviteeGroups": [],
            "invitedGroupIds": [],
            "coOrganizerIds": [],
            "invitedOtpInboxIds": [],
            "attachmentIds": [],
            "hideInOwnCalendar": false,
            "responseDeadline": null,
            "institutionCode": "101001",
            "startDateTime": "2024-04-01T10:00:00",
            "endDateTime": "2024-04-01T12:00:00",
            "allDay": false,
            "private": false,
            "responseRequired": true,
            "primaryResourceId": null,
            "primaryResourceText": null,
            "additionalResourceIds": [],
            "additionalResourceText": null,
            "addToInstitutionCalendar": false,
            "addedToInstitutionCalendar": false,
            "maximumNumberOfParticipants": null,
            "doRequestNumberOfParticipants": false
        }"#;
        let req: CreateSimpleEventRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title.as_deref(), Some("New event"));
        assert_eq!(req.event_type_enum, Some(EventType::Event));
        assert!(req.response_required);
        assert_eq!(req.invitee_ids.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn deserialize_respond_timeslot_request() {
        let json = r#"{
            "eventId": 42,
            "responseTypeEnum": "Accepted",
            "timeSlotId": 5,
            "timeSlotIndex": 2,
            "institutionProfileId": 100,
            "concerningInstProfileId": 200,
            "onBehalfOf": true
        }"#;
        let req: RespondTimeslotEventRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.event_id, Some(42));
        assert_eq!(req.response_type_enum, Some(ResponseType::Accepted));
        assert!(req.on_behalf_of);
    }

    #[test]
    fn deserialize_vacation_children_dto() {
        let json = r#"{
            "child": {
                "profileId": 10,
                "name": "Emma",
                "id": 5,
                "metadata": null,
                "profilePicture": null
            },
            "status": "IsComing",
            "vacationRegistrationResponseId": 99
        }"#;
        let vc: VacationChildrenDto = serde_json::from_str(json).unwrap();
        assert_eq!(vc.status, Some(VacationResponseStatusEnum::IsComing));
        let child = vc.child.unwrap();
        assert_eq!(child.name.as_deref(), Some("Emma"));
    }

    #[test]
    fn deserialize_birthday_event_dto() {
        let json = r#"{
            "birthday": "2015-05-20",
            "name": "Sofia",
            "institutionCode": "101001",
            "institutionProfileId": 300,
            "mainGroupName": "3A"
        }"#;
        let bd: BirthdayEventDto = serde_json::from_str(json).unwrap();
        assert_eq!(bd.name.as_deref(), Some("Sofia"));
        assert_eq!(bd.main_group_name.as_deref(), Some("3A"));
    }

    #[test]
    fn deserialize_aggregated_events() {
        let json = r#"{
            "date": "2024-03-15",
            "aggregatedEvents": [
                {"type": "Event", "count": 3},
                {"type": "Birthday", "count": 1}
            ]
        }"#;
        let result: DailyAggregatedEventsResultModel = serde_json::from_str(json).unwrap();
        let events = result.aggregated_events.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].count, Some(3));
    }

    #[test]
    fn serialize_create_base_event_request() {
        let req = CreateBaseEventRequest {
            event_id: None,
            from_inst_profile_id: Some(100),
            title: Some("Test".to_string()),
            event_type_enum: Some(EventType::Event),
            description: Some("Desc".to_string()),
            invitee_ids: Some(vec![]),
            invitee_groups: Some(vec![]),
            invited_group_ids: Some(vec![]),
            co_organizer_ids: Some(vec![]),
            invited_otp_inbox_ids: Some(vec![]),
            attachment_ids: Some(vec![]),
            hide_in_own_calendar: false,
            response_deadline: None,
            institution_code: Some("101001".to_string()),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"eventTypeEnum\":\"Event\""));
        assert!(json.contains("\"fromInstProfileId\":100"));
    }
}
