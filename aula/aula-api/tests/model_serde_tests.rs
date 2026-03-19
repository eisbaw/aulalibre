//! Integration tests for model serialization/deserialization.
//!
//! These tests verify that realistic JSON payloads (modeled after actual Aula API
//! responses) can be deserialized into Rust structs and round-tripped back to JSON.
//! All data is synthetic — no real PII.

use aula_api::models::documents::{GetSecureDocumentsResult, SecureDocumentDto};
use aula_api::models::gallery::AlbumDto;
use aula_api::models::messaging::{
    MessageDto, MessageThreadSubscription, MessageThreadSubscriptionList, MessagesInThreadDto,
};
use aula_api::models::notifications::NotificationItemDto;
use aula_api::models::posts::{GetPostApiResult, PostApiDto};
use aula_api::models::presence::PresenceRegistrationResult;
use aula_api::models::profiles::Profile;
use aula_api::response::{AulaServiceResponse, WebResponseStatus, WebResponseStatusSubCode};

use aula_api::enums::calendar::{EventType, ParticipantRole, RepeatType, ResponseType};
use aula_api::enums::documents::JournalingStatusEnum;
use aula_api::enums::messaging::{FolderType, SubscriptionType, ThreadType};
use aula_api::enums::notifications::{NotificationArea, NotificationEventType, NotificationType};
use aula_api::enums::presence::{ActivityTypeEnum, PresenceStatusEnum};
use aula_api::enums::profiles::{InstitutionRole, PortalRole};

/// Helper: load fixture file relative to the test crate root.
fn fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"))
}

// =========================================================================
// Response envelope tests
// =========================================================================

mod response_envelope {
    use super::*;

    #[test]
    fn deserialize_success_envelope() {
        let json = fixture("messaging_thread_list.json");
        let resp: AulaServiceResponse<MessageThreadSubscriptionList> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(resp.status.http_code, 200);
        assert_eq!(resp.status.backend_error_code, 0);
        assert!(resp.status.message.is_none());
        assert!(resp.status.sub_code.is_none());
    }

    #[test]
    fn deserialize_error_envelope() {
        let json = fixture("error_response.json");
        let resp: AulaServiceResponse<Option<serde_json::Value>> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(resp.status.http_code, 403);
        assert_eq!(resp.status.backend_error_code, 100);
        assert_eq!(resp.status.message.as_deref(), Some("ACCESS_DENIED"));
        assert_eq!(
            resp.status.presented_message.as_deref(),
            Some("Du har ikke adgang til denne funktion.")
        );
        assert_eq!(resp.status.sub_code, Some(5));
        assert_eq!(
            WebResponseStatusSubCode::from_code(5),
            Some(WebResponseStatusSubCode::AuthorizationDeniedBlockedCommunication)
        );
        assert!(resp.data.is_none());
    }

    #[test]
    fn status_roundtrip() {
        let status = WebResponseStatus {
            http_code: 200,
            backend_error_code: 0,
            message: None,
            presented_message: None,
            sub_code: None,
            html_content_if_error: None,
        };
        // WebResponseStatus only has Deserialize, not Serialize, so we test
        // deserialization from manually constructed JSON
        let json = r#"{"httpCode":200}"#;
        let parsed: WebResponseStatus = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.http_code, status.http_code);
    }
}

// =========================================================================
// Messaging model tests
// =========================================================================

mod messaging {
    use super::*;

    #[test]
    fn deserialize_thread_list_from_fixture() {
        let json = fixture("messaging_thread_list.json");
        let resp: AulaServiceResponse<MessageThreadSubscriptionList> =
            serde_json::from_str(&json).unwrap();
        let data = resp.data;

        assert_eq!(data.page, Some(1));
        assert!(data.more_messages_exist);
        assert!(data.bundle_id.is_none());

        let threads = data.threads.unwrap();
        assert_eq!(threads.len(), 2);

        // First thread: normal unread marked thread
        let t1 = &threads[0];
        assert_eq!(t1.id, Some(48291));
        assert!(t1.marked);
        assert!(!t1.read);
        assert!(!t1.muted);
        assert!(!t1.sensitive);
        assert_eq!(t1.subject.as_deref(), Some("Matematik-fremskridt"));
        assert_eq!(t1.institution_code.as_deref(), Some("280371"));
        assert_eq!(t1.subscription_type, Some(SubscriptionType::Unbundled));
        assert!(!t1.is_thread_or_subscription_deleted);

        // Creator with profile picture
        let creator = t1.creator.as_ref().unwrap();
        assert_eq!(creator.full_name.as_deref(), Some("Mette Frederiksen"));
        assert_eq!(creator.short_name.as_deref(), Some("MF"));
        let owner = creator.mail_box_owner.as_ref().unwrap();
        assert_eq!(owner.portal_role, Some(PortalRole::Employee));
        assert!(!owner.is_deactivated);
        let pic = creator.profile_picture.as_ref().unwrap();
        assert!(pic.url.is_some());

        // Folder
        let folder = t1.current_folder.as_ref().unwrap();
        assert_eq!(folder.name.as_deref(), Some("Indbakke"));
        assert_eq!(folder.folder_type, Some(FolderType::Normal));

        // Latest message
        let latest = t1.latest_message.as_ref().unwrap();
        assert_eq!(latest.id.as_deref(), Some("msg-33015"));
        assert!(!latest.has_attachments);
        assert!(latest.text.as_ref().unwrap().html.is_some());

        // Regarding children
        let children = t1.regarding_children.as_ref().unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].display_name.as_deref(), Some("Emma Jensen"));

        // Second thread: bundled, muted, sensitive, with draft and entity link
        let t2 = &threads[1];
        assert!(t2.muted);
        assert!(t2.read);
        assert!(t2.sensitive);
        assert_eq!(t2.subscription_type, Some(SubscriptionType::Bundle));
        assert_eq!(t2.number_of_bundle_items, Some(3));
        assert_eq!(t2.bundle_id, Some(7701));
        assert_eq!(t2.primary_subscription_id, Some(91020));

        // Draft
        let draft = t2.message_draft.as_ref().unwrap();
        assert!(draft.text.is_some());
        assert_eq!(draft.attachment_ids.as_ref().unwrap(), &[501, 502]);

        // Thread entity link
        let link = t2.thread_entity_link_dto.as_ref().unwrap();
        assert_eq!(link.entity_id.as_deref(), Some("event-4455"));
        assert_eq!(link.thread_type, Some(ThreadType::EventReminder));

        // Deleted folder
        let folder2 = t2.current_folder.as_ref().unwrap();
        assert_eq!(folder2.folder_type, Some(FolderType::Deleted));
    }

    #[test]
    fn deserialize_thread_detail_from_fixture() {
        let json = fixture("messaging_thread_detail.json");
        let resp: AulaServiceResponse<MessagesInThreadDto> = serde_json::from_str(&json).unwrap();
        let data = resp.data;

        assert_eq!(data.id, Some(48291));
        assert!(data.is_marked);
        assert!(!data.more_messages_exist);
        assert_eq!(data.total_message_count, Some(3));
        assert_eq!(data.page, Some(1));
        assert_eq!(data.folder_name.as_deref(), Some("Indbakke"));
        assert!(!data.is_thread_forwarded);
        assert!(!data.sensitive);
        assert!(!data.has_secure_documents);

        // First message
        let first = data.first_message.as_ref().unwrap();
        assert_eq!(first.id.as_deref(), Some("msg-33010"));
        assert_eq!(first.message_type.as_deref(), Some("Message"));
        assert!(first.can_reply_to_message);
        let sender = first.sender.as_ref().unwrap();
        assert_eq!(sender.full_name.as_deref(), Some("Mette Frederiksen"));
        assert_eq!(
            sender.answer_directly_name.as_deref(),
            Some("Mette Frederiksen (Klasselærer 3.A)")
        );

        // Messages array
        let msgs = data.messages.as_ref().unwrap();
        assert_eq!(msgs.len(), 2);

        // System message (RecipientAdded)
        let sys_msg = &msgs[1];
        assert_eq!(sys_msg.message_type.as_deref(), Some("RecipientAdded"));
        assert!(!sys_msg.can_reply_to_message);
        assert!(sys_msg.text.is_none());
        assert!(sys_msg.sender.is_none());
        let new_recip = sys_msg.new_recipient.as_ref().unwrap();
        assert_eq!(new_recip.full_name.as_deref(), Some("Karen Jensen"));
        assert_eq!(sys_msg.inviter_name.as_deref(), Some("Mette Frederiksen"));

        // Thread creator
        let tc = data.thread_creator.as_ref().unwrap();
        assert_eq!(tc.full_name.as_deref(), Some("Mette Frederiksen"));
    }

    #[test]
    fn subscription_roundtrip() {
        let json = fixture("messaging_thread_list.json");
        let resp: AulaServiceResponse<MessageThreadSubscriptionList> =
            serde_json::from_str(&json).unwrap();
        let thread = &resp.data.threads.as_ref().unwrap()[0];

        // Serialize back to JSON
        let serialized = serde_json::to_string(thread).unwrap();
        // Deserialize again
        let roundtripped: MessageThreadSubscription = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, thread.id);
        assert_eq!(roundtripped.subject, thread.subject);
        assert_eq!(roundtripped.marked, thread.marked);
    }

    #[test]
    fn message_with_all_optional_fields_null() {
        let json = r#"{
            "id": null,
            "messageType": null,
            "sendDateTime": null,
            "text": null,
            "sender": null,
            "canReplyToMessage": false,
            "attachments": null,
            "newRecipient": null,
            "newRecipients": null,
            "originalRecipients": null,
            "leaverName": null,
            "inviterName": null,
            "leaverNames": null
        }"#;
        let msg: MessageDto = serde_json::from_str(json).unwrap();
        assert!(msg.id.is_none());
        assert!(msg.text.is_none());
        assert!(msg.sender.is_none());
        assert!(!msg.can_reply_to_message);
    }

    #[test]
    fn subscription_with_empty_arrays() {
        let json = r#"{
            "id": 1,
            "muted": false,
            "marked": false,
            "read": false,
            "sensitive": false,
            "recipients": [],
            "regardingChildren": [],
            "isThreadOrSubscriptionDeleted": false
        }"#;
        let sub: MessageThreadSubscription = serde_json::from_str(json).unwrap();
        assert_eq!(sub.id, Some(1));
        assert_eq!(sub.recipients.as_ref().unwrap().len(), 0);
        assert_eq!(sub.regarding_children.as_ref().unwrap().len(), 0);
    }
}

// =========================================================================
// Calendar model tests
// =========================================================================

mod calendar {
    use super::*;

    #[test]
    fn deserialize_events_from_fixture() {
        let json = fixture("calendar_events.json");
        let resp: AulaServiceResponse<Vec<serde_json::Value>> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(resp.status.http_code, 200);
        let events = resp.data;
        assert_eq!(events.len(), 2);

        // Parse individual events as EventSimpleDto-like structures
        // (the fixture uses a simplified event format matching common fields)
        let event1: serde_json::Value = events[0].clone();
        assert_eq!(event1["id"], 22150);
        assert_eq!(event1["title"], "Forældremøde 3.A");
        assert_eq!(event1["type"], "ParentalMeeting");
        assert_eq!(event1["responseRequired"], true);
        assert_eq!(event1["repeating"], false);
        assert_eq!(event1["repeatType"], "Never");

        // Lesson event
        let event2: serde_json::Value = events[1].clone();
        assert_eq!(event2["id"], 22151);
        assert_eq!(event2["type"], "Lesson");
        assert_eq!(event2["repeating"], true);
        assert_eq!(event2["repeatType"], "Weekly");
        assert!(event2["lesson"].is_object());
    }

    #[test]
    fn event_type_enum_all_variants() {
        let variants = vec![
            ("\"Event\"", EventType::Event),
            ("\"Holiday\"", EventType::Holiday),
            ("\"PresenceHoliday\"", EventType::PresenceHoliday),
            ("\"VacationRegistration\"", EventType::VacationRegistration),
            ("\"Birthday\"", EventType::Birthday),
            ("\"Meeting\"", EventType::Meeting),
            ("\"Other\"", EventType::Other),
            ("\"Excursion\"", EventType::Excursion),
            ("\"SchoolHomeMeeting\"", EventType::SchoolHomeMeeting),
            ("\"ClassMeeting\"", EventType::ClassMeeting),
            ("\"ParentalMeeting\"", EventType::ParentalMeeting),
            ("\"PerformanceMeeting\"", EventType::PerformanceMeeting),
            ("\"Lesson\"", EventType::Lesson),
            ("\"Unknown\"", EventType::Unknown),
        ];
        for (json_str, expected) in variants {
            let parsed: EventType = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected, "Failed for {json_str}");
            let roundtripped = serde_json::to_string(&parsed).unwrap();
            assert_eq!(roundtripped, json_str);
        }
    }

    #[test]
    fn response_type_all_variants() {
        let variants = vec![
            ("\"Waiting\"", ResponseType::Waiting),
            ("\"Declined\"", ResponseType::Declined),
            ("\"Accepted\"", ResponseType::Accepted),
            ("\"Tentative\"", ResponseType::Tentative),
        ];
        for (json_str, expected) in variants {
            let parsed: ResponseType = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn repeat_type_all_variants() {
        let variants = vec![
            ("\"Never\"", RepeatType::Never),
            ("\"Daily\"", RepeatType::Daily),
            ("\"Weekly\"", RepeatType::Weekly),
            ("\"Monthly\"", RepeatType::Monthly),
        ];
        for (json_str, expected) in variants {
            let parsed: RepeatType = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn participant_role_all_variants() {
        let variants = vec![
            ("\"PrimaryTeacher\"", ParticipantRole::PrimaryTeacher),
            ("\"SubstituteTeacher\"", ParticipantRole::SubstituteTeacher),
            ("\"HelpTeacher\"", ParticipantRole::HelpTeacher),
            ("\"Pedagogue\"", ParticipantRole::Pedagogue),
            ("\"NotChosen\"", ParticipantRole::NotChosen),
        ];
        for (json_str, expected) in variants {
            let parsed: ParticipantRole = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
        }
    }
}

// =========================================================================
// Presence model tests
// =========================================================================

mod presence {
    use super::*;

    #[test]
    fn deserialize_presence_from_fixture() {
        let json = fixture("presence_registrations.json");
        let resp: AulaServiceResponse<Vec<PresenceRegistrationResult>> =
            serde_json::from_str(&json).unwrap();
        let data = resp.data;
        assert_eq!(data.len(), 2);

        // First child: present, checked in
        let p1 = &data[0];
        assert_eq!(p1.id, 55001);
        assert_eq!(p1.status, Some(PresenceStatusEnum::Present));
        assert_eq!(p1.activity_type, Some(ActivityTypeEnum::CheckIn));
        assert!(p1.check_in_time.is_some());
        assert!(p1.check_out_time.is_none());

        let profile = p1.institution_profile.as_ref().unwrap();
        assert_eq!(profile.name.as_deref(), Some("Emma Jensen"));
        assert_eq!(profile.role, Some(PortalRole::Child));
        assert_eq!(profile.main_group.as_deref(), Some("3.A"));

        let location = p1.location.as_ref().unwrap();
        assert_eq!(location.name.as_deref(), Some("SFO Lokale 1"));

        let editable = p1.editable_presence_states.as_ref().unwrap();
        assert_eq!(editable.len(), 4);
        assert!(editable.contains(&PresenceStatusEnum::Present));
        assert!(editable.contains(&PresenceStatusEnum::Sick));

        // daily_note and pick_up_time are not on PresenceRegistrationResult;
        // they exist on ActivityListRequest. The extra JSON fields are silently
        // ignored by serde, which is intentional.
        assert_eq!(p1.check_in_time.as_deref(), Some("2026-03-18T07:45:00"));

        // Second child: sick, with sleep interval
        let p2 = &data[1];
        assert_eq!(p2.status, Some(PresenceStatusEnum::Sick));
        assert!(p2.activity_type.is_none());
        assert!(p2.location.is_none());
        assert!(p2.check_in_time.is_none());

        let sleep = p2.sleep_intervals.as_ref().unwrap();
        assert_eq!(sleep.len(), 1);
        assert_eq!(sleep[0].id, 3301);
        assert_eq!(sleep[0].start_time.as_deref(), Some("12:00"));
        assert_eq!(sleep[0].end_time.as_deref(), Some("13:30"));
    }

    #[test]
    fn presence_status_all_variants() {
        let variants = vec![
            ("\"NotPresent\"", PresenceStatusEnum::NotPresent),
            ("\"Sick\"", PresenceStatusEnum::Sick),
            ("\"ReportedAbsence\"", PresenceStatusEnum::ReportedAbsence),
            ("\"Present\"", PresenceStatusEnum::Present),
            ("\"FieldTrip\"", PresenceStatusEnum::FieldTrip),
            ("\"Sleeping\"", PresenceStatusEnum::Sleeping),
            (
                "\"SpareTimeActivity\"",
                PresenceStatusEnum::SpareTimeActivity,
            ),
            (
                "\"PhysicalPlacement\"",
                PresenceStatusEnum::PhysicalPlacement,
            ),
            ("\"CheckedOut\"", PresenceStatusEnum::CheckedOut),
            ("\"NotArrived\"", PresenceStatusEnum::NotArrived),
            ("\"All\"", PresenceStatusEnum::All),
        ];
        for (json_str, expected) in variants {
            let parsed: PresenceStatusEnum = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected, "Failed for {json_str}");
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn activity_type_screaming_snake_all_variants() {
        let variants = vec![
            ("\"PICKED_UP_BY\"", ActivityTypeEnum::PickedUpBy),
            ("\"SELF_DECIDER\"", ActivityTypeEnum::SelfDecider),
            ("\"SEND_HOME\"", ActivityTypeEnum::SendHome),
            ("\"GO_HOME_WITH\"", ActivityTypeEnum::GoHomeWith),
            ("\"DROP_OFF_TIME\"", ActivityTypeEnum::DropOffTime),
            ("\"SPARE_TIME\"", ActivityTypeEnum::SpareTime),
            ("\"CHECK_IN\"", ActivityTypeEnum::CheckIn),
            ("\"CHECK_OUT\"", ActivityTypeEnum::CheckOut),
            ("\"SLEEPING\"", ActivityTypeEnum::Sleeping),
            ("\"ALL\"", ActivityTypeEnum::All),
        ];
        for (json_str, expected) in variants {
            let parsed: ActivityTypeEnum = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected, "Failed for {json_str}");
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn presence_registration_roundtrip() {
        let json = fixture("presence_registrations.json");
        let resp: AulaServiceResponse<Vec<PresenceRegistrationResult>> =
            serde_json::from_str(&json).unwrap();
        let reg = &resp.data[0];
        let serialized = serde_json::to_string(reg).unwrap();
        let roundtripped: PresenceRegistrationResult = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, reg.id);
        assert_eq!(roundtripped.status, reg.status);
    }
}

// =========================================================================
// Profile / Institution model tests
// =========================================================================

mod profiles {
    use super::*;

    #[test]
    fn deserialize_profiles_from_fixture() {
        use aula_api::services::profiles::ProfilesByLoginResponse;
        let json = fixture("profiles_response.json");
        let resp: AulaServiceResponse<ProfilesByLoginResponse> =
            serde_json::from_str(&json).unwrap();
        let profiles = resp.data.profiles;
        assert_eq!(profiles.len(), 1);

        let profile = &profiles[0];
        assert_eq!(profile.id, Some(6701));
        assert_eq!(profile.first_name.as_deref(), Some("Henrik"));
        assert_eq!(profile.last_name.as_deref(), Some("Jensen"));
        assert_eq!(profile.portal_role.as_deref(), Some("Guardian"));
        assert!(!profile.is_stepped_up);
        assert!(!profile.is_group_home_admin);

        // Institution profile
        let ip = profile.institution_profile.as_ref().unwrap();
        assert_eq!(ip.institution_profile_id, 12055);
        assert_eq!(ip.profile_id, 6701);
        assert_eq!(ip.institution_role, Some(InstitutionRole::Guardian));
        assert_eq!(ip.email.as_deref(), Some("henrik.jensen@mail.dk"));
        assert_eq!(ip.phone.as_deref(), Some("28456789"));
        assert!(!ip.communication_block);
        assert!(!ip.upload_block);
        assert!(!ip.alias);

        // Address
        let addr = ip.address.as_ref().unwrap();
        assert_eq!(addr.street.as_deref(), Some("Skovvej 15"));
        assert_eq!(addr.postal_code.as_deref(), Some("8260"));
        assert_eq!(addr.postal_district.as_deref(), Some("Viby J"));

        // Profile picture
        let pic = ip.profile_picture.as_ref().unwrap();
        assert_eq!(pic.id, Some(8811));
        assert!(pic.url.is_some());

        // Relations
        let relations = ip.relations.as_ref().unwrap();
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].role, Some(PortalRole::Child));
        assert_eq!(relations[0].full_name.as_deref(), Some("Emma Jensen"));
        let inst = relations[0].institution.as_ref().unwrap();
        assert_eq!(inst.name.as_deref(), Some("Bakkeskolen"));
        assert_eq!(inst.institution_code.as_deref(), Some("280371"));

        // Institution identity
        let identity = ip.institution.as_ref().unwrap();
        assert_eq!(identity.institution_code.as_deref(), Some("280371"));
        assert_eq!(identity.municipality_name.as_deref(), Some("Aarhus"));

        // Groups
        let groups = profile.groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name.as_deref(), Some("3.A Forældre"));

        // Page configuration
        let page_config = profile.page_configuration.as_ref().unwrap();
        let widgets = page_config.widget_configurations.as_ref().unwrap();
        assert_eq!(widgets.len(), 1);
        let widget = widgets[0].widget.as_ref().unwrap();
        assert_eq!(widget.name.as_deref(), Some("Kontaktbog"));
        assert!(widget.can_access_on_mobile);
    }

    #[test]
    fn portal_role_all_variants() {
        let variants = vec![
            ("\"Other\"", PortalRole::Other),
            ("\"Employee\"", PortalRole::Employee),
            ("\"Child\"", PortalRole::Child),
            ("\"Guardian\"", PortalRole::Guardian),
            ("\"Otp\"", PortalRole::Otp),
        ];
        for (json_str, expected) in variants {
            let parsed: PortalRole = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn institution_role_all_variants() {
        let variants = vec![
            ("\"Unknown\"", InstitutionRole::Unknown),
            ("\"Guardian\"", InstitutionRole::Guardian),
            ("\"Daycare\"", InstitutionRole::Daycare),
            ("\"Leader\"", InstitutionRole::Leader),
            ("\"PreschoolTeacher\"", InstitutionRole::PreschoolTeacher),
            ("\"Teacher\"", InstitutionRole::Teacher),
            ("\"EarlyStudent\"", InstitutionRole::EarlyStudent),
            ("\"MiddleLateStudent\"", InstitutionRole::MiddleLateStudent),
            ("\"Child\"", InstitutionRole::Child),
            ("\"Other\"", InstitutionRole::Other),
        ];
        for (json_str, expected) in variants {
            let parsed: InstitutionRole = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn profile_roundtrip() {
        use aula_api::services::profiles::ProfilesByLoginResponse;
        let json = fixture("profiles_response.json");
        let resp: AulaServiceResponse<ProfilesByLoginResponse> =
            serde_json::from_str(&json).unwrap();
        let profile = &resp.data.profiles[0];
        let serialized = serde_json::to_string(profile).unwrap();
        let roundtripped: Profile = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, profile.id);
        assert_eq!(roundtripped.first_name, profile.first_name);
    }

    #[test]
    fn profile_minimal_all_nulls() {
        let json = r#"{
            "id": null,
            "institutionProfile": null,
            "groups": null,
            "municipalGroups": null,
            "phonenumber": null,
            "email": null,
            "workPhonenumber": null,
            "homePhonenumber": null,
            "mobilePhonenumber": null,
            "administrator": null,
            "firstName": null,
            "lastName": null,
            "userId": null,
            "portalRole": null,
            "isSteppedUp": false,
            "groupHomes": null,
            "isGroupHomeAdmin": false,
            "pageConfiguration": null
        }"#;
        let profile: Profile = serde_json::from_str(json).unwrap();
        assert!(profile.id.is_none());
        assert!(profile.institution_profile.is_none());
        assert!(!profile.is_stepped_up);
    }
}

// =========================================================================
// Notification model tests
// =========================================================================

mod notifications {
    use super::*;

    #[test]
    fn deserialize_notifications_from_fixture() {
        let json = fixture("notifications_response.json");
        let resp: AulaServiceResponse<Vec<NotificationItemDto>> =
            serde_json::from_str(&json).unwrap();
        let notifs = resp.data;
        assert_eq!(notifs.len(), 2);

        // Message notification
        let n1 = &notifs[0];
        assert_eq!(n1.notification_id.as_deref(), Some("notif-88201"));
        assert_eq!(
            n1.notification_event_type,
            Some(NotificationEventType::NewMessagePrivateInbox)
        );
        assert_eq!(n1.notification_area, Some(NotificationArea::Messages));
        assert_eq!(n1.notification_type, Some(NotificationType::Alert));
        assert_eq!(n1.thread_id, Some(48291));
        assert_eq!(n1.sender_name.as_deref(), Some("Mette Frederiksen"));
        assert_eq!(n1.related_child_name.as_deref(), Some("Emma Jensen"));
        assert!(n1.content.is_none());
        assert!(!n1.is_presence_times_required);

        // Calendar notification
        let n2 = &notifs[1];
        assert_eq!(
            n2.notification_event_type,
            Some(NotificationEventType::InvitedToEventResponseRequired)
        );
        assert_eq!(n2.notification_area, Some(NotificationArea::Calendar));
        assert_eq!(n2.event_id, Some(22150));
        assert!(n2.content.is_some());
        assert!(n2.response_deadline.is_some());
        assert_eq!(n2.room_name.as_deref(), Some("Lokale 201"));
        assert_eq!(n2.media_ids, Some(vec![]));
    }

    #[test]
    fn notification_event_type_all_variants() {
        // Test a representative sample of the large NotificationEventType enum
        let variants = vec![
            (
                "\"NewMessagePrivateInbox\"",
                NotificationEventType::NewMessagePrivateInbox,
            ),
            (
                "\"InvitedToEventResponseRequired\"",
                NotificationEventType::InvitedToEventResponseRequired,
            ),
            (
                "\"PostSharedWithMe\"",
                NotificationEventType::PostSharedWithMe,
            ),
            ("\"NewMedia\"", NotificationEventType::NewMedia),
            (
                "\"NewOrUpdatedSecureDocument\"",
                NotificationEventType::NewOrUpdatedSecureDocument,
            ),
            (
                "\"VacationResponseRequired\"",
                NotificationEventType::VacationResponseRequired,
            ),
            (
                "\"LessonNoteChanged\"",
                NotificationEventType::LessonNoteChanged,
            ),
            (
                "\"WidgetNotification\"",
                NotificationEventType::WidgetNotification,
            ),
            (
                "\"FileScanFailedAlbum\"",
                NotificationEventType::FileScanFailedAlbum,
            ),
            ("\"Other\"", NotificationEventType::Other),
        ];
        for (json_str, expected) in variants {
            let parsed: NotificationEventType = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected, "Failed for {json_str}");
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn notification_area_all_variants() {
        let variants = vec![
            ("\"Unknown\"", NotificationArea::Unknown),
            ("\"Messages\"", NotificationArea::Messages),
            ("\"Calendar\"", NotificationArea::Calendar),
            ("\"Posts\"", NotificationArea::Posts),
            ("\"Schedule\"", NotificationArea::Schedule),
            ("\"Administration\"", NotificationArea::Administration),
            ("\"Gallery\"", NotificationArea::Gallery),
            ("\"Documents\"", NotificationArea::Documents),
            ("\"Album\"", NotificationArea::Album),
            ("\"Presence\"", NotificationArea::Presence),
            ("\"Widget\"", NotificationArea::Widget),
            ("\"FileScanning\"", NotificationArea::FileScanning),
        ];
        for (json_str, expected) in variants {
            let parsed: NotificationArea = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn notification_type_all_variants() {
        let variants = vec![
            ("\"Badge\"", NotificationType::Badge),
            ("\"Alert\"", NotificationType::Alert),
            ("\"Irrelevant\"", NotificationType::Irrelevant),
            ("\"Unknown\"", NotificationType::Unknown),
        ];
        for (json_str, expected) in variants {
            let parsed: NotificationType = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
        }
    }
}

// =========================================================================
// Post model tests
// =========================================================================

mod posts {
    use super::*;

    #[test]
    fn deserialize_posts_from_fixture() {
        let json = fixture("posts_response.json");
        let resp: AulaServiceResponse<GetPostApiResult> = serde_json::from_str(&json).unwrap();
        let data = resp.data;

        assert!(data.has_more_posts);
        assert_eq!(data.page, Some(1));

        let posts = data.posts.unwrap();
        assert_eq!(posts.len(), 1);

        let post = &posts[0];
        assert_eq!(post.id, Some(33001));
        assert_eq!(post.title.as_deref(), Some("Tur til Moesgaard Museum"));
        assert!(post.allow_comments);
        assert!(post.is_important);
        assert!(post.important_from.is_some());
        assert!(post.important_to.is_some());
        assert!(!post.can_current_user_report);
        assert!(!post.can_current_user_delete);
        assert!(post.can_current_user_comment);
        assert!(!post.is_bookmarked);
        assert_eq!(post.comment_count, Some(3));

        let owner = post.owner_profile.as_ref().unwrap();
        assert_eq!(owner.full_name.as_deref(), Some("Mette Frederiksen"));
        assert_eq!(owner.role.as_deref(), Some("Employee"));

        let groups = post.shared_with_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        let mc = groups[0].membership_count.as_ref().unwrap();
        assert_eq!(mc.total, Some(69));
    }

    #[test]
    fn post_roundtrip() {
        let json = fixture("posts_response.json");
        let resp: AulaServiceResponse<GetPostApiResult> = serde_json::from_str(&json).unwrap();
        let post = &resp.data.posts.unwrap()[0];
        let serialized = serde_json::to_string(post).unwrap();
        let roundtripped: PostApiDto = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, post.id);
        assert_eq!(roundtripped.title, post.title);
    }
}

// =========================================================================
// Gallery model tests
// =========================================================================

mod gallery {
    use super::*;

    #[test]
    fn deserialize_album_from_fixture() {
        let json = fixture("gallery_album.json");
        let resp: AulaServiceResponse<AlbumDto> = serde_json::from_str(&json).unwrap();
        let album = resp.data;

        assert_eq!(album.id, Some(7701));
        assert_eq!(album.title.as_deref(), Some("Tur til Tivoli"));
        assert_eq!(album.total_size, Some(45));
        assert_eq!(album.size, Some(20));
        assert!(!album.current_user_can_edit);
        assert!(!album.current_user_can_delete);
        assert!(!album.current_user_can_add_media);

        let creator = album.creator.as_ref().unwrap();
        assert_eq!(creator.name.as_deref(), Some("Mette Frederiksen"));

        let thumbs = album.thumbnails_urls.as_ref().unwrap();
        assert_eq!(thumbs.len(), 3);

        let groups = album.shared_with_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        let roles = groups[0].portal_roles.as_ref().unwrap();
        assert!(roles.contains(&PortalRole::Guardian));
        assert!(roles.contains(&PortalRole::Employee));
    }

    #[test]
    fn album_roundtrip() {
        let json = fixture("gallery_album.json");
        let resp: AulaServiceResponse<AlbumDto> = serde_json::from_str(&json).unwrap();
        let serialized = serde_json::to_string(&resp.data).unwrap();
        let roundtripped: AlbumDto = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, resp.data.id);
        assert_eq!(roundtripped.title, resp.data.title);
    }
}

// =========================================================================
// Document model tests
// =========================================================================

mod documents {
    use super::*;

    #[test]
    fn deserialize_documents_from_fixture() {
        let json = fixture("documents_response.json");
        let resp: AulaServiceResponse<GetSecureDocumentsResult> =
            serde_json::from_str(&json).unwrap();
        let data = resp.data;
        assert_eq!(data.total_count, Some(1));

        let docs = data.documents.unwrap();
        assert_eq!(docs.len(), 1);

        let doc = &docs[0];
        assert_eq!(doc.id, Some(9901));
        assert!(doc.can_edit);
        assert!(doc.can_edit_locked_status);
        assert!(!doc.is_locked);
        assert!(!doc.has_media);
        assert_eq!(
            doc.journaling_status,
            Some(JournalingStatusEnum::NotProcessed)
        );
        assert_eq!(doc.category.as_deref(), Some("Agenda"));
        assert_eq!(doc.version, Some(3));
        assert!(doc.is_shared_with_guardian);
        assert!(doc.is_shareable);

        let guardian_ids = doc.shareable_guardian_ids.as_ref().unwrap();
        assert_eq!(guardian_ids, &[12055, 12056]);

        let creator = doc.creator.as_ref().unwrap();
        assert_eq!(creator.name.as_deref(), Some("Mette Frederiksen"));
        assert!(!creator.alias);

        let shared_profiles = doc.shared_with_institution_profiles.as_ref().unwrap();
        assert_eq!(shared_profiles.len(), 1);
        assert_eq!(shared_profiles[0].role, Some(PortalRole::Guardian));
        assert!(!shared_profiles[0].can_edit);

        // Filters
        let filters = data.filters.unwrap();
        let categories = filters.document_categories.unwrap();
        assert_eq!(categories, vec!["Agenda", "Note"]);

        let regarding = filters.regarding_institution_profiles.unwrap();
        assert_eq!(regarding.len(), 1);
        assert_eq!(regarding[0].name.as_deref(), Some("Emma Jensen"));
    }

    #[test]
    fn journaling_status_all_variants() {
        let variants = vec![
            ("\"NotProcessed\"", JournalingStatusEnum::NotProcessed),
            ("\"InProgress\"", JournalingStatusEnum::InProgress),
            ("\"Failed\"", JournalingStatusEnum::Failed),
            ("\"Completed\"", JournalingStatusEnum::Completed),
        ];
        for (json_str, expected) in variants {
            let parsed: JournalingStatusEnum = serde_json::from_str(json_str).unwrap();
            assert_eq!(parsed, expected);
            let rt = serde_json::to_string(&parsed).unwrap();
            assert_eq!(rt, json_str);
        }
    }

    #[test]
    fn document_roundtrip() {
        let json = fixture("documents_response.json");
        let resp: AulaServiceResponse<GetSecureDocumentsResult> =
            serde_json::from_str(&json).unwrap();
        let doc = &resp.data.documents.unwrap()[0];
        let serialized = serde_json::to_string(doc).unwrap();
        let roundtripped: SecureDocumentDto = serde_json::from_str(&serialized).unwrap();
        assert_eq!(roundtripped.id, doc.id);
        assert_eq!(roundtripped.title, doc.title);
    }
}

// =========================================================================
// Comprehensive enum round-trip tests for ALL enum variants
// =========================================================================

mod enum_exhaustive {
    use aula_api::enums::calendar::*;
    use aula_api::enums::common::*;
    use aula_api::enums::documents::*;
    use aula_api::enums::gallery::*;
    use aula_api::enums::messaging::*;
    use aula_api::enums::notifications::*;
    use aula_api::enums::presence::*;
    use aula_api::enums::profiles::*;

    /// Macro to test all variants of an enum round-trip through JSON.
    macro_rules! test_all_variants {
        ($test_name:ident, $enum_ty:ty, [$($variant:expr),+ $(,)?]) => {
            #[test]
            fn $test_name() {
                let variants: Vec<$enum_ty> = vec![$($variant),+];
                for variant in &variants {
                    let json = serde_json::to_string(variant)
                        .unwrap_or_else(|e| panic!("Failed to serialize {:?}: {e}", variant));
                    let back: $enum_ty = serde_json::from_str(&json)
                        .unwrap_or_else(|e| panic!("Failed to deserialize {json} back to {:?}: {e}", variant));
                    assert_eq!(&back, variant, "Round-trip failed for {json}");
                }
            }
        };
    }

    // -- Calendar enums (all variants) --
    test_all_variants!(
        event_class_all,
        EventClass,
        [
            EventClass::Basic,
            EventClass::Series,
            EventClass::Timeslot,
            EventClass::Lesson,
            EventClass::Unknown
        ]
    );
    test_all_variants!(
        event_placement_all,
        EventPlacementComparedToDateTime,
        [
            EventPlacementComparedToDateTime::NotOnTheDate,
            EventPlacementComparedToDateTime::StartAndEndOnDate,
            EventPlacementComparedToDateTime::StartOnDateButEndAfter,
            EventPlacementComparedToDateTime::StartBeforeDateButEndOn,
            EventPlacementComparedToDateTime::StartBeforeAndEndAfterDate
        ]
    );
    test_all_variants!(
        event_portrait_all,
        EventPortraitType,
        [
            EventPortraitType::Event,
            EventPortraitType::Birthday,
            EventPortraitType::AllDay
        ]
    );
    test_all_variants!(
        event_type_all,
        EventType,
        [
            EventType::Event,
            EventType::Holiday,
            EventType::PresenceHoliday,
            EventType::VacationRegistration,
            EventType::Birthday,
            EventType::Meeting,
            EventType::Other,
            EventType::Excursion,
            EventType::SchoolHomeMeeting,
            EventType::ClassMeeting,
            EventType::ParentalMeeting,
            EventType::PerformanceMeeting,
            EventType::Lesson,
            EventType::Unknown
        ]
    );
    test_all_variants!(
        lesson_status_all,
        LessonStatus,
        [
            LessonStatus::Cancelled,
            LessonStatus::Normal,
            LessonStatus::Absent,
            LessonStatus::Substitute,
            LessonStatus::ToBeDeleted,
            LessonStatus::WillBeUpdated,
            LessonStatus::StatusNotFound
        ]
    );
    test_all_variants!(
        participant_role_all,
        ParticipantRole,
        [
            ParticipantRole::PrimaryTeacher,
            ParticipantRole::SubstituteTeacher,
            ParticipantRole::HelpTeacher,
            ParticipantRole::Pedagogue,
            ParticipantRole::NotChosen
        ]
    );
    test_all_variants!(
        repeat_type_all,
        RepeatType,
        [
            RepeatType::Never,
            RepeatType::Daily,
            RepeatType::Weekly,
            RepeatType::Monthly
        ]
    );
    test_all_variants!(
        repeating_dropdown_all,
        RepeatingEventDropdownEnum,
        [
            RepeatingEventDropdownEnum::ForSeries,
            RepeatingEventDropdownEnum::ForSingleOccurrence
        ]
    );
    test_all_variants!(
        response_type_all,
        ResponseType,
        [
            ResponseType::Waiting,
            ResponseType::Declined,
            ResponseType::Accepted,
            ResponseType::Tentative
        ]
    );
    test_all_variants!(
        timeslot_response_all,
        TimeslotResponseType,
        [
            TimeslotResponseType::Blocked,
            TimeslotResponseType::NotBooked,
            TimeslotResponseType::AlreadyBooked
        ]
    );
    test_all_variants!(
        vacation_reg_response_all,
        VacationRegistrationResponseStatus,
        [
            VacationRegistrationResponseStatus::Answered,
            VacationRegistrationResponseStatus::Unanswered
        ]
    );
    test_all_variants!(
        vacation_response_all,
        VacationResponseStatusEnum,
        [
            VacationResponseStatusEnum::IsComing,
            VacationResponseStatusEnum::IsNotComing,
            VacationResponseStatusEnum::PendingAnswer
        ]
    );
    test_all_variants!(
        relation_mode_all,
        RelationMode,
        [RelationMode::ChildMode, RelationMode::Institution]
    );
    test_all_variants!(
        calendar_item_type_all,
        CalendarItemType,
        [
            CalendarItemType::Event,
            CalendarItemType::Title,
            CalendarItemType::Birthday
        ]
    );
    test_all_variants!(
        my_calendar_item_type_all,
        MyCalendarItemType,
        [MyCalendarItemType::Body, MyCalendarItemType::Title]
    );

    // -- Common enums (all variants) --
    test_all_variants!(
        platform_all,
        Platform,
        [Platform::Android, Platform::Ios, Platform::Unknown]
    );
    test_all_variants!(
        weekday_all,
        WeekDay,
        [
            WeekDay::Monday,
            WeekDay::Tuesday,
            WeekDay::Wednesday,
            WeekDay::Thursday,
            WeekDay::Friday,
            WeekDay::Saturday,
            WeekDay::Sunday
        ]
    );
    test_all_variants!(
        sort_order_all,
        SortOrderEnum,
        [
            SortOrderEnum::Unknown,
            SortOrderEnum::Ascending,
            SortOrderEnum::Descending
        ]
    );
    test_all_variants!(
        app_type_all,
        AppTypeEnum,
        [
            AppTypeEnum::Staff,
            AppTypeEnum::Private,
            AppTypeEnum::Unknown
        ]
    );
    test_all_variants!(
        association_mode_all,
        AssociationModeEnum,
        [
            AssociationModeEnum::None,
            AssociationModeEnum::Select,
            AssociationModeEnum::Confirm
        ]
    );
    test_all_variants!(
        file_picker_all,
        AulaFilePickerEnum,
        [
            AulaFilePickerEnum::Files,
            AulaFilePickerEnum::MediaLibrary,
            AulaFilePickerEnum::GoogleDrive,
            AulaFilePickerEnum::OneDrive,
            AulaFilePickerEnum::PhotoCamera,
            AulaFilePickerEnum::VideoCamera,
            AulaFilePickerEnum::AulaGallery,
            AulaFilePickerEnum::Document,
            AulaFilePickerEnum::DownloadMediaGoogleDrive,
            AulaFilePickerEnum::DownloadMediaOneDrive,
            AulaFilePickerEnum::FilesForMedia,
            AulaFilePickerEnum::AttachFileGoogleDrive,
            AulaFilePickerEnum::AttachFileOneDrive,
            AulaFilePickerEnum::All
        ]
    );
    test_all_variants!(
        cache_type_all,
        CacheType,
        [CacheType::Small, CacheType::Large]
    );
    test_all_variants!(
        filter_sort_all,
        FilterAndSortType,
        [
            FilterAndSortType::FilterAll,
            FilterAndSortType::FilterUnread,
            FilterAndSortType::FilterMarked,
            FilterAndSortType::FilterDraft,
            FilterAndSortType::SortDate,
            FilterAndSortType::SortSubject,
            FilterAndSortType::SortCreatedDate,
            FilterAndSortType::SortMediaCreatedDate,
            FilterAndSortType::SortMediaCreatedAt,
            FilterAndSortType::FilterMyAlbums,
            FilterAndSortType::FilterMyMedia,
            FilterAndSortType::SortAlbumName
        ]
    );
    test_all_variants!(
        loading_type_all,
        LoadingType,
        [
            LoadingType::LoadMore,
            LoadingType::Action,
            LoadingType::Refresh
        ]
    );
    test_all_variants!(
        log_level_all,
        LogLevel,
        [
            LogLevel::All,
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warning,
            LogLevel::Error,
            LogLevel::Fatal
        ]
    );
    test_all_variants!(
        report_enum_all,
        ReportEnum,
        [
            ReportEnum::Post,
            ReportEnum::Media,
            ReportEnum::Comments,
            ReportEnum::Unknown
        ]
    );
    test_all_variants!(
        resource_type_all,
        ResourceType,
        [
            ResourceType::Location,
            ResourceType::Other,
            ResourceType::ExtraLocation,
            ResourceType::Electronics,
            ResourceType::Stationery
        ]
    );
    test_all_variants!(
        time_period_all,
        TimePeriod,
        [
            TimePeriod::None,
            TimePeriod::TwoWeeks,
            TimePeriod::OneMonth,
            TimePeriod::ThreeMonths,
            TimePeriod::SixMonths,
            TimePeriod::OneYear
        ]
    );
    test_all_variants!(
        bio_auth_all,
        BioAuthStatus,
        [
            BioAuthStatus::CanTryAgain,
            BioAuthStatus::CanNotTryAgain,
            BioAuthStatus::Canceled,
            BioAuthStatus::Accepted
        ]
    );
    test_all_variants!(
        blocked_level_all,
        BlockedLevel,
        [
            BlockedLevel::Central,
            BlockedLevel::Municipal,
            BlockedLevel::Institution,
            BlockedLevel::Unknown
        ]
    );
    test_all_variants!(
        consent_all,
        Consent,
        [
            Consent::ShareContactInformationParent,
            Consent::ShareContactInformationChild,
            Consent::Others
        ]
    );
    test_all_variants!(
        consent_answer_all,
        ConsentAnswerEnum,
        [
            ConsentAnswerEnum::Accepted,
            ConsentAnswerEnum::Declined,
            ConsentAnswerEnum::Class,
            ConsentAnswerEnum::Year,
            ConsentAnswerEnum::Institution,
            ConsentAnswerEnum::NotAtAll,
            ConsentAnswerEnum::Other
        ]
    );
    test_all_variants!(
        consent_status_all,
        ConsentStatus,
        [
            ConsentStatus::Active,
            ConsentStatus::Deactive,
            ConsentStatus::Pending
        ]
    );
    test_all_variants!(
        comment_type_all,
        CommentType,
        [
            CommentType::Comment,
            CommentType::Media,
            CommentType::Post,
            CommentType::Unknown
        ]
    );
    test_all_variants!(
        post_filter_all,
        PostFilterTypeEnum,
        [
            PostFilterTypeEnum::All,
            PostFilterTypeEnum::Unread,
            PostFilterTypeEnum::IsImportant,
            PostFilterTypeEnum::FromStaff,
            PostFilterTypeEnum::FromParents,
            PostFilterTypeEnum::FromStudents,
            PostFilterTypeEnum::OwnPost,
            PostFilterTypeEnum::Bookmarked
        ]
    );

    // -- Document enums (all variants) --
    test_all_variants!(
        doc_category_all,
        DocumentCategoryEnum,
        [
            DocumentCategoryEnum::Agenda,
            DocumentCategoryEnum::AgendaAllUser,
            DocumentCategoryEnum::PlanOfAction,
            DocumentCategoryEnum::Setting,
            DocumentCategoryEnum::ForCableSchedule,
            DocumentCategoryEnum::Observation,
            DocumentCategoryEnum::EducationalNote,
            DocumentCategoryEnum::Summary,
            DocumentCategoryEnum::SummaryAllUser,
            DocumentCategoryEnum::ScratchScheme,
            DocumentCategoryEnum::OpenTemplate,
            DocumentCategoryEnum::OpenTemplateAllUser,
            DocumentCategoryEnum::Note,
            DocumentCategoryEnum::Unknown
        ]
    );
    test_all_variants!(
        doc_type_all,
        DocumentTypeEnum,
        [
            DocumentTypeEnum::Unknown,
            DocumentTypeEnum::External,
            DocumentTypeEnum::Internal,
            DocumentTypeEnum::Note,
            DocumentTypeEnum::Richdocument
        ]
    );
    test_all_variants!(
        revision_change_all,
        RevisionChangeTypeEnum,
        [
            RevisionChangeTypeEnum::Unshared,
            RevisionChangeTypeEnum::Shared,
            RevisionChangeTypeEnum::Unlocked,
            RevisionChangeTypeEnum::Locked,
            RevisionChangeTypeEnum::Edited,
            RevisionChangeTypeEnum::Exported,
            RevisionChangeTypeEnum::Created,
            RevisionChangeTypeEnum::PermissionAdded,
            RevisionChangeTypeEnum::PermissionRemoved,
            RevisionChangeTypeEnum::ImplicitUnshared,
            RevisionChangeTypeEnum::Deleted,
            RevisionChangeTypeEnum::Restored,
            RevisionChangeTypeEnum::JournalizedToESDH,
            RevisionChangeTypeEnum::SentToESDH,
            RevisionChangeTypeEnum::EsdhJournalizationFailed,
            RevisionChangeTypeEnum::ResentToESDH,
            RevisionChangeTypeEnum::ManuallyJournalizedToESDH,
            RevisionChangeTypeEnum::MarkForManualJournalize
        ]
    );
    test_all_variants!(
        file_scanning_all,
        FileScanningStatus,
        [
            FileScanningStatus::Available,
            FileScanningStatus::Blocked,
            FileScanningStatus::Processing,
            FileScanningStatus::Bypassed
        ]
    );
    test_all_variants!(
        file_status_all,
        FileStatusEnum,
        [
            FileStatusEnum::Available,
            FileStatusEnum::Pending,
            FileStatusEnum::Unavailable,
            FileStatusEnum::Unknown
        ]
    );
    test_all_variants!(
        cloud_storage_type_all,
        CloudStorageFileType,
        [
            CloudStorageFileType::Folder,
            CloudStorageFileType::Docs,
            CloudStorageFileType::Sheets,
            CloudStorageFileType::Excel,
            CloudStorageFileType::Slides,
            CloudStorageFileType::PowerPoint,
            CloudStorageFileType::Video,
            CloudStorageFileType::Sound,
            CloudStorageFileType::File,
            CloudStorageFileType::Pdf,
            CloudStorageFileType::Image,
            CloudStorageFileType::Unknown
        ]
    );
    test_all_variants!(
        cloud_service_all,
        CloudStorageService,
        [
            CloudStorageService::Unknown,
            CloudStorageService::GoogleDrive,
            CloudStorageService::OneDrive
        ]
    );

    // -- Gallery enums (all variants) --
    test_all_variants!(
        media_type_all,
        MediaTypeEnum,
        [
            MediaTypeEnum::Unknown,
            MediaTypeEnum::Image,
            MediaTypeEnum::Video,
            MediaTypeEnum::Sound,
            MediaTypeEnum::MediaWithDuration,
            MediaTypeEnum::Media
        ]
    );
    test_all_variants!(
        conversion_status_all,
        ConversionStatusEnum,
        [
            ConversionStatusEnum::Completed,
            ConversionStatusEnum::Processing,
            ConversionStatusEnum::Failed
        ]
    );
    test_all_variants!(
        rotating_all,
        RotatingEnum,
        [
            RotatingEnum::Rotating0,
            RotatingEnum::Rotating90,
            RotatingEnum::Rotating180,
            RotatingEnum::Rotating270
        ]
    );
    test_all_variants!(
        thumbnail_size_all,
        ThumbnailSizeEnum,
        [
            ThumbnailSizeEnum::XS,
            ThumbnailSizeEnum::S,
            ThumbnailSizeEnum::M,
            ThumbnailSizeEnum::L,
            ThumbnailSizeEnum::Full
        ]
    );

    // -- Messaging enums (all variants) --
    test_all_variants!(
        message_type_all,
        MessageType,
        [
            MessageType::AllMessageRelatedType,
            MessageType::Message,
            MessageType::RecipientAdded,
            MessageType::RecipientRemoved,
            MessageType::AutoReply,
            MessageType::SystemForward,
            MessageType::SystemReply,
            MessageType::Forward,
            MessageType::Other,
            MessageType::RecipientsAdded,
            MessageType::RecipientsRemoved,
            MessageType::MessageDeleted,
            MessageType::MessageEdited,
            MessageType::SystemForwardSingleMessage
        ]
    );
    test_all_variants!(
        sensitivity_level_all,
        SensitivityLevel,
        [
            SensitivityLevel::Level1,
            SensitivityLevel::Level2,
            SensitivityLevel::Level3
        ]
    );
    test_all_variants!(
        subscription_type_all,
        SubscriptionType,
        [
            SubscriptionType::Bundle,
            SubscriptionType::BundleItem,
            SubscriptionType::Unbundled
        ]
    );
    test_all_variants!(
        thread_type_all,
        ThreadType,
        [
            ThreadType::Thread,
            ThreadType::EventReminder,
            ThreadType::VacationRequestReminder
        ]
    );
    test_all_variants!(
        folder_type_all,
        FolderType,
        [
            FolderType::Normal,
            FolderType::Deleted,
            FolderType::ButtonCell
        ]
    );
    test_all_variants!(
        common_inbox_type_all,
        CommonInboxType,
        [
            CommonInboxType::Institutional,
            CommonInboxType::CrossInstitutional
        ]
    );
    test_all_variants!(
        recipient_api_type_all,
        RecipientApiType,
        [
            RecipientApiType::Unknown,
            RecipientApiType::InstitutionProfile,
            RecipientApiType::CommonInbox,
            RecipientApiType::OtpInbox
        ]
    );
    test_all_variants!(
        send_button_all,
        SendMessageButton,
        [SendMessageButton::ReplySingle, SendMessageButton::ReplyAll]
    );
    test_all_variants!(
        dropdown_action_all,
        DropdownActionEnum,
        [
            DropdownActionEnum::AddRecipient,
            DropdownActionEnum::Forwarding,
            DropdownActionEnum::ToggleMute,
            DropdownActionEnum::Leave,
            DropdownActionEnum::ToggleSensitive,
            DropdownActionEnum::ExportThreadToDocument,
            DropdownActionEnum::MarkAsImportant,
            DropdownActionEnum::MoveToFolder,
            DropdownActionEnum::Delete,
            DropdownActionEnum::ToggleReadStatus,
            DropdownActionEnum::CreateDocument
        ]
    );

    // -- Notification enums (all variants) --
    test_all_variants!(
        notification_area_all,
        NotificationArea,
        [
            NotificationArea::Unknown,
            NotificationArea::Messages,
            NotificationArea::Calendar,
            NotificationArea::Posts,
            NotificationArea::Schedule,
            NotificationArea::Administration,
            NotificationArea::Gallery,
            NotificationArea::Documents,
            NotificationArea::Album,
            NotificationArea::Presence,
            NotificationArea::Widget,
            NotificationArea::FileScanning
        ]
    );
    test_all_variants!(
        notification_type_all,
        NotificationType,
        [
            NotificationType::Badge,
            NotificationType::Alert,
            NotificationType::Irrelevant,
            NotificationType::Unknown
        ]
    );

    // -- Presence enums (all variants) --
    test_all_variants!(
        presence_status_all,
        PresenceStatusEnum,
        [
            PresenceStatusEnum::NotPresent,
            PresenceStatusEnum::Sick,
            PresenceStatusEnum::ReportedAbsence,
            PresenceStatusEnum::Present,
            PresenceStatusEnum::FieldTrip,
            PresenceStatusEnum::Sleeping,
            PresenceStatusEnum::SpareTimeActivity,
            PresenceStatusEnum::PhysicalPlacement,
            PresenceStatusEnum::CheckedOut,
            PresenceStatusEnum::NotArrived,
            PresenceStatusEnum::All
        ]
    );
    test_all_variants!(
        activity_type_all,
        ActivityTypeEnum,
        [
            ActivityTypeEnum::PickedUpBy,
            ActivityTypeEnum::SelfDecider,
            ActivityTypeEnum::SendHome,
            ActivityTypeEnum::GoHomeWith,
            ActivityTypeEnum::DropOffTime,
            ActivityTypeEnum::SpareTime,
            ActivityTypeEnum::CheckIn,
            ActivityTypeEnum::CheckOut,
            ActivityTypeEnum::Sleeping,
            ActivityTypeEnum::All
        ]
    );
    test_all_variants!(
        opening_hours_all,
        OpeningHoursType,
        [
            OpeningHoursType::SpecificOpeningHours,
            OpeningHoursType::GeneralOpeningHours,
            OpeningHoursType::DefaultOpeningHours,
            OpeningHoursType::ClosedDay
        ]
    );
    test_all_variants!(
        presence_day_all,
        PresenceDayOfWeek,
        [
            PresenceDayOfWeek::Monday,
            PresenceDayOfWeek::Tuesday,
            PresenceDayOfWeek::Wednesday,
            PresenceDayOfWeek::Thursday,
            PresenceDayOfWeek::Friday,
            PresenceDayOfWeek::Saturday,
            PresenceDayOfWeek::Sunday
        ]
    );
    test_all_variants!(
        comego_type_all,
        ComeGoType,
        [
            ComeGoType::PickupActivity,
            ComeGoType::VacationRegistrationRequest
        ]
    );
    test_all_variants!(
        token_status_all,
        TokenStatusEnum,
        [
            TokenStatusEnum::Used,
            TokenStatusEnum::NotUsed,
            TokenStatusEnum::Expired
        ]
    );

    // -- Profile enums (all variants) --
    test_all_variants!(
        portal_role_all,
        PortalRole,
        [
            PortalRole::Other,
            PortalRole::Employee,
            PortalRole::Child,
            PortalRole::Guardian,
            PortalRole::Otp
        ]
    );
    test_all_variants!(
        institution_role_all,
        InstitutionRole,
        [
            InstitutionRole::Unknown,
            InstitutionRole::Guardian,
            InstitutionRole::Daycare,
            InstitutionRole::Leader,
            InstitutionRole::PreschoolTeacher,
            InstitutionRole::Teacher,
            InstitutionRole::EarlyStudent,
            InstitutionRole::MiddleLateStudent,
            InstitutionRole::Child,
            InstitutionRole::Other
        ]
    );
    test_all_variants!(
        institution_type_all,
        InstitutionTypeEnum,
        [
            InstitutionTypeEnum::Unknown,
            InstitutionTypeEnum::School,
            InstitutionTypeEnum::Daycare,
            InstitutionTypeEnum::Municipality,
            InstitutionTypeEnum::Central
        ]
    );
    test_all_variants!(
        group_role_all,
        GroupRole,
        [
            GroupRole::Unknown,
            GroupRole::Member,
            GroupRole::Indirect,
            GroupRole::Applied,
            GroupRole::Removed,
            GroupRole::Rejected,
            GroupRole::Inactive
        ]
    );
    test_all_variants!(
        group_status_all,
        GroupStatus,
        [
            GroupStatus::Unidentified,
            GroupStatus::Active,
            GroupStatus::Inactive
        ]
    );
    test_all_variants!(
        group_type_all,
        GroupTypeEnum,
        [
            GroupTypeEnum::Unknown,
            GroupTypeEnum::Institutional,
            GroupTypeEnum::Municipal,
            GroupTypeEnum::CrossInstitutional,
            GroupTypeEnum::Other
        ]
    );
    test_all_variants!(
        login_auth_all,
        LoginAuthenticationMethod,
        [
            LoginAuthenticationMethod::Unknown,
            LoginAuthenticationMethod::Level2,
            LoginAuthenticationMethod::Level3NemId,
            LoginAuthenticationMethod::Level3Employees
        ]
    );
    test_all_variants!(
        onboarding_step_all,
        OnboardingStep,
        [
            OnboardingStep::AppOnboarding,
            OnboardingStep::PolicyAcceptance,
            OnboardingStep::MasterData,
            OnboardingStep::Consents,
            OnboardingStep::AdditionalMasterData,
            OnboardingStep::NotificationSettings
        ]
    );
}

// =========================================================================
// Edge case tests
// =========================================================================

mod edge_cases {
    use super::*;
    use aula_api::models::messaging::{CommonInboxesDto, Folder, MailBox, RecipientApiModel};

    #[test]
    fn unknown_fields_are_ignored() {
        // Ensure forward compatibility: extra JSON fields do not cause errors
        let json = r#"{
            "id": 42,
            "email": "test@aula.dk",
            "displayName": "Test",
            "relation": null,
            "shortName": null,
            "futureField": "should be ignored",
            "anotherNewField": 123
        }"#;
        let mb: MailBox = serde_json::from_str(json).unwrap();
        assert_eq!(mb.id, Some(42));
    }

    #[test]
    fn empty_json_object_for_defaults() {
        // Models with all #[serde(default)] booleans and Options
        let json = r#"{}"#;
        let result = serde_json::from_str::<RecipientApiModel>(json);
        // This should succeed because all fields are Option or have defaults
        let r = result.unwrap();
        assert!(r.id.is_none());
        assert!(!r.is_deactivated);
        assert!(!r.is_deleted);
    }

    #[test]
    fn nested_null_objects() {
        let json = r#"{
            "id": 1,
            "name": null,
            "type": null
        }"#;
        let folder: Folder = serde_json::from_str(json).unwrap();
        assert_eq!(folder.id, Some(1));
        assert!(folder.name.is_none());
        assert!(folder.folder_type.is_none());
    }

    #[test]
    fn large_integer_ids() {
        let json = r#"{
            "id": 9223372036854775807,
            "name": "Max ID Inbox",
            "address": null,
            "folders": [],
            "participants": [],
            "institutionCode": "999999",
            "institutionName": null,
            "commonInboxType": null
        }"#;
        let inbox: CommonInboxesDto = serde_json::from_str(json).unwrap();
        assert_eq!(inbox.id, Some(i64::MAX));
    }

    #[test]
    fn html_with_special_characters() {
        let json = r#"{"html": "<p>Børn &amp; unge &lt;3&gt; år</p>"}"#;
        let rt: aula_api::models::messaging::RichTextWrapperDto =
            serde_json::from_str(json).unwrap();
        assert!(rt.html.as_deref().unwrap().contains("&amp;"));
    }

    #[test]
    fn unicode_in_names() {
        let json = r#"{
            "id": 1,
            "email": null,
            "displayName": "Søren Ægidius Ødegård",
            "relation": null,
            "shortName": "SÆØ"
        }"#;
        let mb: MailBox = serde_json::from_str(json).unwrap();
        assert_eq!(mb.display_name.as_deref(), Some("Søren Ægidius Ødegård"));
        assert_eq!(mb.short_name.as_ref().and_then(|v| v.as_str()), Some("SÆØ"));
    }

    #[test]
    fn data_array_response_empty() {
        let json = r#"{
            "totalHits": 0,
            "results": []
        }"#;
        let resp: aula_api::response::DataArrayResponse<serde_json::Value> =
            serde_json::from_str(json).unwrap();
        assert_eq!(resp.total_hits, 0);
        assert!(resp.results.is_empty());
    }

    #[test]
    fn sub_code_all_known_values() {
        for code in 1..=19 {
            let result = WebResponseStatusSubCode::from_code(code);
            assert!(result.is_some(), "SubCode {code} should be a known variant");
        }
        assert!(WebResponseStatusSubCode::from_code(0).is_none());
        assert!(WebResponseStatusSubCode::from_code(20).is_none());
        assert!(WebResponseStatusSubCode::from_code(-1).is_none());
    }
}
