//! Calendar and event-related enums.

use serde::{Deserialize, Serialize};

/// Classification of a calendar event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventClass {
    Basic,
    Series,
    Timeslot,
    Lesson,
    Unknown,
}

/// How an event is placed relative to a given date/time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventPlacementComparedToDateTime {
    NotOnTheDate,
    StartAndEndOnDate,
    StartOnDateButEndAfter,
    StartBeforeDateButEndOn,
    StartBeforeAndEndAfterDate,
}

/// Portrait display type for calendar events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventPortraitType {
    Event,
    Birthday,
    AllDay,
}

/// Type of calendar event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Event,
    Holiday,
    PresenceHoliday,
    VacationRegistration,
    Birthday,
    Meeting,
    Other,
    Excursion,
    SchoolHomeMeeting,
    ClassMeeting,
    ParentalMeeting,
    PerformanceMeeting,
    Lesson,
    Unknown,
}

/// Status of a lesson in the schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LessonStatus {
    Cancelled,
    Normal,
    Absent,
    Substitute,
    ToBeDeleted,
    WillBeUpdated,
    StatusNotFound,
}

/// Role of a participant in a calendar event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParticipantRole {
    PrimaryTeacher,
    SubstituteTeacher,
    HelpTeacher,
    Pedagogue,
    NotChosen,
}

/// How an event repeats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RepeatType {
    Never,
    Daily,
    Weekly,
    Monthly,
}

/// Dropdown option when editing a repeating event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RepeatingEventDropdownEnum {
    ForSeries,
    ForSingleOccurrence,
}

/// Response to an event invitation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Waiting,
    Declined,
    Accepted,
    Tentative,
}

/// Availability status for a timeslot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeslotResponseType {
    Blocked,
    NotBooked,
    AlreadyBooked,
}

/// Vacation registration response status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VacationRegistrationResponseStatus {
    Answered,
    Unanswered,
}

/// Vacation response status (from DTOs.Calendar.Vacation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VacationResponseStatusEnum {
    IsComing,
    IsNotComing,
    PendingAnswer,
}

/// Relation mode for calendar views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RelationMode {
    ChildMode,
    Institution,
}

/// Calendar item type (event vs title vs birthday).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CalendarItemType {
    Event,
    Title,
    Birthday,
}

/// My-calendar item type (body vs title).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MyCalendarItemType {
    Body,
    Title,
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

    roundtrip_test!(event_class, EventClass, EventClass::Series);
    roundtrip_test!(
        event_placement,
        EventPlacementComparedToDateTime,
        EventPlacementComparedToDateTime::StartAndEndOnDate
    );
    roundtrip_test!(
        event_portrait,
        EventPortraitType,
        EventPortraitType::Birthday
    );
    roundtrip_test!(event_type, EventType, EventType::SchoolHomeMeeting);
    roundtrip_test!(lesson_status, LessonStatus, LessonStatus::Substitute);
    roundtrip_test!(
        participant_role,
        ParticipantRole,
        ParticipantRole::HelpTeacher
    );
    roundtrip_test!(repeat_type, RepeatType, RepeatType::Weekly);
    roundtrip_test!(
        repeating_dropdown,
        RepeatingEventDropdownEnum,
        RepeatingEventDropdownEnum::ForSeries
    );
    roundtrip_test!(response_type, ResponseType, ResponseType::Accepted);
    roundtrip_test!(
        timeslot_response,
        TimeslotResponseType,
        TimeslotResponseType::NotBooked
    );
    roundtrip_test!(
        vacation_reg_status,
        VacationRegistrationResponseStatus,
        VacationRegistrationResponseStatus::Answered
    );
    roundtrip_test!(
        vacation_response,
        VacationResponseStatusEnum,
        VacationResponseStatusEnum::IsComing
    );
    roundtrip_test!(relation_mode, RelationMode, RelationMode::ChildMode);
    roundtrip_test!(
        calendar_item_type,
        CalendarItemType,
        CalendarItemType::Event
    );
    roundtrip_test!(
        my_calendar_item_type,
        MyCalendarItemType,
        MyCalendarItemType::Body
    );
}
