//! Extended integration tests for all API service modules using wiremock.
//!
//! Complements `mock_api_tests.rs` (TASK-66) by covering every service module
//! with at least basic request/response validation.

use wiremock::matchers::{body_json_string, header, method, path, path_regex, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use aula_api::client::AulaClient;
use aula_api::error::AulaError;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

// ---------------------------------------------------------------------------
// Test helpers (same pattern as mock_api_tests.rs)
// ---------------------------------------------------------------------------

fn mock_session(base_url: &str) -> Session {
    let client = mock_client(base_url);
    client.set_cookie("Csrfp-Token=test-csrf-token-42; Path=/");

    let dir = std::env::temp_dir().join(format!(
        "aula_svc_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let store = TokenStore::new(&dir);
    Session::new(client, store, SessionConfig::default()).expect("session")
}

fn mock_client(base_url: &str) -> AulaClient {
    AulaClient::with_base_url(&format!("{base_url}/api/v19/")).expect("client with base URL")
}

fn aula_envelope(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": {
            "httpCode": 200,
            "backendErrorCode": 0,
            "message": null,
            "presentedMessage": null,
            "subCode": null,
            "htmlContentIfError": null
        },
        "data": data
    })
}

fn aula_error_envelope(sub_code: i32) -> serde_json::Value {
    serde_json::json!({
        "status": {
            "httpCode": 200,
            "backendErrorCode": 0,
            "subCode": sub_code
        },
        "data": {}
    })
}

fn fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"))
}

fn json_response(data: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_json(aula_envelope(data))
        .insert_header("content-type", "application/json")
}

fn fixture_response(name: &str) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_string(fixture(name))
        .insert_header("content-type", "application/json")
}

// ===========================================================================
// AC #1: Profile / Configuration service integration tests
// ===========================================================================

mod profile_configuration {
    use super::*;
    use aula_api::services::{configuration, profiles};

    #[tokio::test]
    async fn get_profile_master_data() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "id": 12055,
            "institutionProfiles": [],
            "firstName": "Henrik",
            "lastName": "Jensen",
            "profilePicture": null,
            "isMunicipality": false
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/masterdata/profile"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = profiles::get_profile_master_data(&mut session).await;
        let profile = result.expect("should deserialize profile master data");
        assert_eq!(profile.first_name.as_deref(), Some("Henrik"));
    }

    #[tokio::test]
    async fn post_master_data_sends_json_body() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/masterdata"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = profiles::UpdateMasterDataRequest {
            external_email: Some("test@example.dk".into()),
            phonenumber: Some("12345678".into()),
            work_phonenumber: None,
            home_phonenumber: None,
            mobile_phonenumber: None,
        };
        profiles::post_master_data(&mut session, &req)
            .await
            .expect("post_master_data should succeed");
    }

    #[tokio::test]
    async fn keep_alive_sends_post() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/profiles/keepAlive"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        profiles::keep_alive(&mut session)
            .await
            .expect("keep_alive should succeed");
    }

    #[tokio::test]
    async fn get_max_file_size() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/configuration/maxFileSize"))
            .respond_with(json_response(serde_json::json!(52428800)))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let size = configuration::get_max_file_size(&mut session)
            .await
            .expect("should deserialize max file size");
        assert_eq!(size, 52428800);
    }

    #[tokio::test]
    async fn is_app_deprecated() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "isDeprecated": false,
            "message": null
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/configuration/isAppDeprecated"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let resp = configuration::is_app_deprecated(&mut session)
            .await
            .expect("should deserialize app deprecated response");
        assert!(!resp.is_deprecated);
    }

    #[tokio::test]
    async fn get_authorized_file_formats() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            { "extension": "pdf", "mimeType": "application/pdf" },
            { "extension": "jpg", "mimeType": "image/jpeg" }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/configuration/authorizedFileFormats"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let formats = configuration::get_authorized_file_formats(&mut session)
            .await
            .expect("should deserialize file formats");
        assert_eq!(formats.len(), 2);
    }

    #[tokio::test]
    async fn get_privacy_policy() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "content": "<p>Privacy policy text</p>",
            "version": "3.1"
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/configuration/privacyPolicy"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let policy = configuration::get_privacy_policy(&mut session)
            .await
            .expect("should deserialize privacy policy");
        assert_eq!(policy.version.as_deref(), Some("3.1"));
    }

    #[tokio::test]
    async fn get_login_important_information() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "content": "System maintenance tonight",
            "show": true
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/configuration/loginImportantInformation"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let info = configuration::get_login_important_information(&mut session)
            .await
            .expect("should deserialize login info");
        assert!(info.show);
        assert_eq!(info.content.as_deref(), Some("System maintenance tonight"));
    }

    #[tokio::test]
    async fn update_profile_picture() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/masterdata/profilePicture"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = profiles::UpdateProfilePictureRequest {
            institution_profile_id: 42,
            key: "photos/42/pic.jpg".into(),
            bucket: "aula-prod".into(),
        };
        profiles::update_profile_picture(&mut session, &req)
            .await
            .expect("update_profile_picture should succeed");
    }
}

// ===========================================================================
// AC #2: Messaging service integration tests
// ===========================================================================

mod messaging_extended {
    use super::*;
    use aula_api::models::messaging::*;
    use aula_api::services::messaging;

    #[tokio::test]
    async fn start_new_thread() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/messaging/threads"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({"threadId": 999})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = StartNewThreadRequestArguments {
            subject: Some("Test subject".into()),
            message: Some(MessageContentRequest {
                text: Some("Hello world".into()),
                attachment_ids: Some(vec![]),
            }),
            recipients: None,
            bcc_recipients: None,
            sensitive: false,
            creator: None,
        };
        let result = messaging::start_new_thread(&mut session, &args).await;
        result.expect("start_new_thread should succeed");
    }

    #[tokio::test]
    async fn reply_to_thread() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/messaging/threads/42/reply"))
            .respond_with(json_response(serde_json::json!({"messageId": "msg-100"})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = ReplyMessageArgument {
            thread_id: Some(42),
            message: Some(MessageContentRequest {
                text: Some("Reply text".into()),
                attachment_ids: Some(vec![]),
            }),
            common_inbox_id: None,
            bundle_id: None,
        };
        messaging::reply_to_thread(&mut session, &args)
            .await
            .expect("reply should succeed");
    }

    #[tokio::test]
    async fn get_thread_by_id() {
        let server = MockServer::start().await;

        let body = fixture("messaging_thread_detail.json");

        Mock::given(method("GET"))
            .and(path("/api/v19/messaging/threads/42/messages"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = GetMessagesForThreadArguments {
            thread_id: Some(42),
            page: None,
            common_inbox_id: None,
        };
        let result = messaging::get_thread_by_id(&mut session, &args).await;
        result.expect("should deserialize thread detail");
    }

    #[tokio::test]
    async fn delete_threads() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/messaging/threads"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = DeleteThreadArguments {
            subscription_ids: Some(vec![1, 2]),
            thread_ids: Some(vec![10, 20]),
            common_inbox_id: None,
        };
        messaging::delete_threads(&mut session, &args)
            .await
            .expect("delete_threads should succeed");
    }

    #[tokio::test]
    async fn get_folders() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            { "id": 1, "name": "Indbakke", "isDeleted": false },
            { "id": 2, "name": "Arkiv", "isDeleted": false }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/messaging/folders"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = GetFoldersArguments {
            include_deleted_folders: false,
            common_inbox_id: None,
        };
        let folders = messaging::get_folders(&mut session, &args)
            .await
            .expect("should deserialize folders");
        assert_eq!(folders.len(), 2);
        assert_eq!(folders[0].name.as_deref(), Some("Indbakke"));
    }

    #[tokio::test]
    async fn create_folder() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/messaging/folders"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({"id": 5})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = CreateFolderArguments {
            folder_name: Some("Work".into()),
            common_inbox_id: None,
        };
        messaging::create_folder(&mut session, &args)
            .await
            .expect("create_folder should succeed");
    }

    #[tokio::test]
    async fn get_auto_reply() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "replyText": { "html": "Out of office" },
            "startDateTime": "2026-03-20T00:00:00",
            "endDateTime": "2026-03-25T00:00:00"
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/messaging/autoReply"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = messaging::get_auto_reply(&mut session).await;
        let auto_reply = result.expect("should deserialize auto reply");
        let reply_text = auto_reply.reply_text.expect("reply_text should be present");
        assert_eq!(reply_text.html.as_deref(), Some("Out of office"));
    }

    #[tokio::test]
    async fn set_auto_reply() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/messaging/autoReply"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({
                "replyText": { "html": "Away" },
                "startDateTime": "2026-04-01T00:00:00",
                "endDateTime": "2026-04-10T00:00:00"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = SetAutoReplyArguments {
            reply_text: Some("Away".into()),
            start_date_time: Some("2026-04-01T00:00:00".into()),
            end_date_time: Some("2026-04-10T00:00:00".into()),
        };
        messaging::set_auto_reply(&mut session, &args)
            .await
            .expect("set_auto_reply should succeed");
    }

    #[tokio::test]
    async fn delete_auto_reply() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/messaging/autoReply"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        messaging::delete_auto_reply(&mut session)
            .await
            .expect("delete_auto_reply should succeed");
    }

    #[tokio::test]
    async fn set_thread_muted() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v19/messaging/threads/muted"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = MuteThreadRequestArguments {
            muted: true,
            owner: None,
            subscription_ids: Some(vec![5]),
            common_inbox_id: None,
            thread_ids: Some(vec![10]),
        };
        messaging::set_thread_muted(&mut session, &args)
            .await
            .expect("set_thread_muted should succeed");
    }

    #[tokio::test]
    async fn set_thread_marked() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v19/messaging/threads/marked"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = MarkThreadsRequest {
            marked: true,
            thread_ids: Some(vec![1, 2]),
            subscription_ids: None,
            common_inbox_id: None,
        };
        messaging::set_thread_marked(&mut session, &args)
            .await
            .expect("set_thread_marked should succeed");
    }

    #[tokio::test]
    async fn get_common_inboxes() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "id": 100,
                "name": "SFO Postkasse",
                "institutionCode": "280371"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/messaging/commonInboxes"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let inboxes = messaging::get_common_inboxes(&mut session, &[14201], false)
            .await
            .expect("should deserialize common inboxes");
        assert_eq!(inboxes.len(), 1);
        assert_eq!(inboxes[0].name.as_deref(), Some("SFO Postkasse"));
    }

    #[tokio::test]
    async fn get_message_info_light() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "threadId": 42,
            "threadSubject": "Test besked",
            "isRead": false
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/messaging/messages/msg-99/info"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let info = messaging::get_message_info_light(&mut session, "msg-99", None, None)
            .await
            .expect("should deserialize message info light");
        assert_eq!(info.thread_id, Some(42));
    }
}

// ===========================================================================
// AC #3: Calendar service integration tests
// ===========================================================================

mod calendar_extended {
    use super::*;
    use aula_api::services::calendar;

    #[tokio::test]
    async fn get_event_detail() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "id": 22150,
            "title": "Forældremøde 3.A",
            "type": "ParentalMeeting",
            "allDay": false,
            "startDateTime": "2026-04-10T19:00:00",
            "endDateTime": "2026-04-10T21:00:00"
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/events/22150"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let detail = calendar::get_event_detail(&mut session, 22150)
            .await
            .expect("should deserialize event detail");
        assert_eq!(detail.title.as_deref(), Some("Forældremøde 3.A"));
    }

    #[tokio::test]
    async fn get_birthdays_for_group() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "birthdayDate": "2026-04-15",
                "name": "Emma Jensen",
                "institutionProfileId": 14201
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/birthdays/group/1501"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let birthdays =
            calendar::get_birthdays_for_group(&mut session, 1501, "2026-04-01", "2026-04-30")
                .await
                .expect("should deserialize birthdays");
        assert_eq!(birthdays.len(), 1);
    }

    #[tokio::test]
    async fn get_future_vacation_request() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "id": 501,
                "title": "Sommerferie 2026",
                "startDateTime": "2026-06-27",
                "endDateTime": "2026-08-09"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/vacations/future"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let vacations = calendar::get_future_vacation_request(&mut session, &[])
            .await
            .expect("should deserialize future vacations");
        assert_eq!(vacations.len(), 1);
    }

    #[tokio::test]
    async fn delete_event() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/calendar/events/22150"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        calendar::delete_event(&mut session, 22150)
            .await
            .expect("delete_event should succeed");
    }

    #[tokio::test]
    async fn get_event_types() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "eventTypes": ["Event", "Meeting", "ParentalMeeting"]
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/eventTypes"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = calendar::get_event_types(&mut session, &[])
            .await
            .expect("should deserialize event types");
        assert!(result.event_types.is_some());
    }

    #[tokio::test]
    async fn get_top_important_date() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "date": "2026-04-10",
                "title": "Forældremøde"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/importantDates/top"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let dates = calendar::get_top_important_date(&mut session, &[14201])
            .await
            .expect("should deserialize important dates");
        assert_eq!(dates.len(), 1);
    }

    #[tokio::test]
    async fn get_calendar_sync_consent() {
        let server = MockServer::start().await;

        let data = serde_json::json!({ "policyAccepted": true });

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/sync/consent"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let consent = calendar::get_calendar_synchronisation_consent(&mut session)
            .await
            .expect("should deserialize sync consent");
        assert!(consent.policy_accepted);
    }

    #[tokio::test]
    async fn get_delegated_accesses() {
        let server = MockServer::start().await;

        let data = serde_json::json!([]);

        Mock::given(method("GET"))
            .and(path("/api/v19/calendar/delegatedAccesses"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let accesses = calendar::get_delegated_accesses(&mut session, None)
            .await
            .expect("should deserialize delegated accesses");
        assert!(accesses.is_empty());
    }
}

// ===========================================================================
// AC #4: Presence service integration tests
// ===========================================================================

mod presence_extended {
    use super::*;
    use aula_api::services::presence;

    #[tokio::test]
    async fn get_presence_registrations() {
        let server = MockServer::start().await;

        let body = fixture("presence_registrations.json");

        Mock::given(method("GET"))
            .and(path("/api/v19/presence/registrations"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let regs = presence::get_presence_registrations(&mut session, &[14201], Some("2026-03-18"))
            .await
            .expect("should deserialize presence registrations");
        assert_eq!(regs.len(), 2);
        assert_eq!(regs[0].id, 55001);
    }

    #[tokio::test]
    async fn get_presence_schedules() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "institutionProfileId": 14201,
                "scheduleEntries": []
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/presence/schedules"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = aula_api::models::presence::PresenceSchedulesRequest {
            filter_institution_profile_ids: Some(vec![14201]),
            from_date: Some("2026-03-18".into()),
            to_date: Some("2026-03-24".into()),
        };
        let schedules = presence::get_presence_schedules(&mut session, &args)
            .await
            .expect("should deserialize presence schedules");
        assert_eq!(schedules.len(), 1);
    }

    #[tokio::test]
    async fn get_presence_week_overview() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "weekNumber": 12,
            "presenceDays": [],
            "childActivities": []
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/presence/weekOverview"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = aula_api::models::presence::ComeGoGetWeekOverviewRequest {
            department_id: 100,
            group_ids: None,
            status_filters: None,
            start_date: Some("2026-03-18".into()),
            end_date: Some("2026-03-24".into()),
            offset: 0,
            limit: 50,
        };
        let overview = presence::get_presence_week_overview(&mut session, &args)
            .await
            .expect("should deserialize week overview");
        assert_eq!(overview.week_number, 12);
    }

    #[tokio::test]
    async fn get_pickup_responsibles() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "children": []
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/presence/pickup/responsibles"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = aula_api::models::presence::GetPickupResponsibleRequest {
            uni_student_ids: Some(vec![14201]),
        };
        let result = presence::get_pickup_responsibles(&mut session, &args)
            .await
            .expect("should deserialize pickup responsibles");
        assert!(result.children.unwrap_or_default().is_empty());
    }
}

// ===========================================================================
// AC #5: Remaining services integration tests
// ===========================================================================

mod posts_service {
    use super::*;
    use aula_api::models::posts::{CreatePostApiParameter, GetPostApiParameters};
    use aula_api::services::posts;

    #[tokio::test]
    async fn get_posts() {
        let server = MockServer::start().await;

        let body = fixture("posts_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v19/posts"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = GetPostApiParameters {
            group_id: None,
            is_important: None,
            creator_portal_role: None,
            institution_profile_ids: None,
            related_institutions: None,
            own_post: false,
            is_unread: false,
            is_bookmarked: false,
            limit: Some(20),
            index: Some(0),
        };
        let result = posts::get_posts(&mut session, &params)
            .await
            .expect("should deserialize posts");
        assert!(result.has_more_posts);
        let post_list = result.posts.expect("posts should be present");
        assert_eq!(post_list.len(), 1);
        assert_eq!(
            post_list[0].title.as_deref(),
            Some("Tur til Moesgaard Museum")
        );
    }

    #[tokio::test]
    async fn create_post() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/posts"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(
                serde_json::json!({"allImagesHasValidConsents": true}),
            ))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = CreatePostApiParameter {
            id: None,
            title: Some("Test Post".into()),
            content: Some("<p>Hello</p>".into()),
            institution_code: Some("280371".into()),
            creator_institution_profile_id: Some(12055),
            allow_comments: true,
            is_important: false,
            important_from: None,
            important_to: None,
            shared_with_groups: None,
            attachment_ids: None,
            publish_at: None,
            expire_at: None,
        };
        let _result = posts::create_post(&mut session, &params)
            .await
            .expect("create_post should succeed");
    }

    #[tokio::test]
    async fn bookmark_post() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/posts/33001/bookmark"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        posts::bookmark_post(&mut session, 33001)
            .await
            .expect("bookmark_post should succeed");
    }

    #[tokio::test]
    async fn unbookmark_post() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/posts/33001/bookmark"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        posts::unbookmark_post(&mut session, 33001)
            .await
            .expect("unbookmark_post should succeed");
    }

    #[tokio::test]
    async fn delete_post() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/posts/33001"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        posts::delete_post(&mut session, 33001)
            .await
            .expect("delete_post should succeed");
    }
}

mod gallery_service {
    use super::*;
    use aula_api::models::gallery::{CreateAlbumParameters, GalleryViewFilter};
    use aula_api::services::gallery;

    #[tokio::test]
    async fn get_albums() {
        let server = MockServer::start().await;

        // The fixture is a single album wrapped in envelope.data (not array).
        // The service expects Vec<AlbumDto>, so we wrap in array.
        let data = serde_json::json!([
            {
                "id": 7701,
                "title": "Tur til Tivoli",
                "name": "Tur til Tivoli",
                "creator": {
                    "id": 4401,
                    "institutionCode": "280371",
                    "institutionName": "Bakkeskolen",
                    "name": "Mette Frederiksen",
                    "shortName": "MF"
                },
                "totalSize": 45,
                "size": 20,
                "from": 0,
                "description": "Billeder fra klassens tur",
                "currentUserCanEdit": false,
                "currentUserCanDelete": false,
                "currentUserCanAddMedia": false
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/gallery/albums"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let filter = GalleryViewFilter {
            selected_institution_code_for_filter: None,
            album_id: None,
            user_specific_album: None,
            limit: Some(20),
            index: Some(0),
            sort_on: None,
            order_direction: None,
            filter_by: None,
        };
        let albums = gallery::get_albums(&mut session, &filter)
            .await
            .expect("should deserialize albums");
        assert_eq!(albums.len(), 1);
        assert_eq!(albums[0].title.as_deref(), Some("Tur til Tivoli"));
    }

    #[tokio::test]
    async fn create_album() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/gallery/albums"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({"id": 7702})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = CreateAlbumParameters {
            title: Some("New Album".into()),
            album_id: None,
            creator_institution_profile_id: Some(12055),
            shared_with_groups: None,
            description: Some("Test album".into()),
        };
        gallery::create_album(&mut session, &params)
            .await
            .expect("create_album should succeed");
    }

    #[tokio::test]
    async fn delete_album() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/gallery/albums/7701"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        gallery::delete_album(&mut session, 7701)
            .await
            .expect("delete_album should succeed");
    }

    #[tokio::test]
    async fn delete_media() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/gallery/media/100"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        gallery::delete_media(&mut session, 100)
            .await
            .expect("delete_media should succeed");
    }
}

mod documents_service {
    use super::*;
    use aula_api::models::documents::GetSecureDocumentsArguments;
    use aula_api::services::documents;

    #[tokio::test]
    async fn get_secure_documents() {
        let server = MockServer::start().await;

        let body = fixture("documents_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v19/documents/secure"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = GetSecureDocumentsArguments {
            filter_institution_profile_ids: None,
            filter_regarding_group_ids: None,
            filter_unread: None,
            filter_locked: None,
            filter_journaling_status: None,
            filter_editable: false,
            document_type: None,
            sortings: None,
            index: Some(0),
            limit: Some(20),
            filter_regarding_student_ids: None,
            filter_document_categories: None,
        };
        let result = documents::get_secure_documents(&mut session, &args)
            .await
            .expect("should deserialize secure documents");
        assert_eq!(result.total_count, Some(1));
        let docs = result.documents.unwrap();
        assert_eq!(
            docs[0].title.as_deref(),
            Some("Trivselssamtale - Emma Jensen")
        );
    }

    #[tokio::test]
    async fn get_max_documents_per_export() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/documents/export/maxCount"))
            .respond_with(json_response(serde_json::json!(50)))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let max = documents::get_max_documents_per_export(&mut session)
            .await
            .expect("should deserialize max export count");
        assert_eq!(max, 50);
    }

    #[tokio::test]
    async fn soft_delete_document() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/documents/9901"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        documents::soft_delete_secure_document(&mut session, 9901)
            .await
            .expect("soft_delete should succeed");
    }

    #[tokio::test]
    async fn update_document_locked_status() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v19/documents/9901/locked"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        documents::update_document_locked_status(&mut session, 9901, true)
            .await
            .expect("update_document_locked_status should succeed");
    }

    #[tokio::test]
    async fn create_pdf_for_single() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "requestExportJobId": 42,
            "status": "Created",
            "progress": 0.0,
            "fileUrl": null,
            "fileName": null
        });

        Mock::given(method("POST"))
            .and(path("/api/v19/documents/9901/pdf"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let export = documents::create_pdf_for_single(&mut session, 9901)
            .await
            .expect("create_pdf should succeed");
        assert_eq!(export.request_export_job_id, Some(42));
    }
}

mod notifications_service {
    use super::*;
    use aula_api::services::notifications;

    #[tokio::test]
    async fn get_notifications() {
        let server = MockServer::start().await;

        let body = fixture("notifications_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v19/notifications"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let notifs = notifications::get_notifications(&mut session)
            .await
            .expect("should deserialize notifications");
        assert_eq!(notifs.len(), 2);
        assert_eq!(notifs[0].notification_id.as_deref(), Some("notif-88201"));
    }

    #[tokio::test]
    async fn delete_notifications() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/notifications"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        notifications::delete_notifications(&mut session)
            .await
            .expect("delete_notifications should succeed");
    }

    #[tokio::test]
    async fn delete_notification_for_child() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/notifications/child/14201"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        notifications::delete_notification_for_child(&mut session, 14201)
            .await
            .expect("delete_notification_for_child should succeed");
    }
}

mod search_service {
    use super::*;
    use aula_api::models::search::{
        GlobalSearchParameters, SearchGroupRequestModel, SearchRecipientParameters,
    };
    use aula_api::services::search;

    #[tokio::test]
    async fn global_search() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "totalSize": 3,
            "docTypeCount": [
                { "name": "Post", "count": 2 },
                { "name": "Event", "count": 1 }
            ],
            "groupTypeCount": [],
            "results": []
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/search"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = GlobalSearchParameters {
            text: Some("test".into()),
            page_limit: Some(20),
            page_number: Some(1),
            group_id: None,
            doc_type_count: true,
            doc_type: None,
            group_types: None,
        };
        let result = search::global_search(&mut session, &params)
            .await
            .expect("should deserialize search response");
        assert_eq!(result.total_size, Some(3));
    }

    #[tokio::test]
    async fn search_for_recipients() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "totalHits": 1,
            "results": [
                {
                    "docId": "p-1",
                    "docType": "Profile",
                    "institutionCode": "280371",
                    "institutionName": "Bakkeskolen",
                    "name": "Mette Frederiksen"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/search/recipients"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = SearchRecipientParameters {
            text: Some("Mette".into()),
            from_module: None,
            doc_types: None,
            portal_roles: None,
            group_search_scope: None,
            limit: Some(10),
            scope_employees_to_institution: None,
            group_id: None,
            inst_code: None,
            institution_codes: None,
            regarding_children: None,
            mail_box_owner_type: None,
            mail_box_owner_id: None,
        };
        let result = search::search_for_recipients(&mut session, &params)
            .await
            .expect("should deserialize recipient search");
        assert_eq!(result.total_hits, Some(1));
    }

    #[tokio::test]
    async fn search_groups() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "results": [
                {
                    "institutionCode": "280371",
                    "institutionName": "Bakkeskolen",
                    "name": "3.A",
                    "id": 1501
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/search/groups"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = SearchGroupRequestModel {
            text: Some("3.A".into()),
            institution_codes: None,
            limit: Some(10),
            offset: None,
            from_module_value: None,
        };
        let result = search::search_groups(&mut session, &params)
            .await
            .expect("should deserialize group search");
        assert_eq!(result.results.as_ref().unwrap().len(), 1);
    }
}

mod groups_service {
    use super::*;
    use aula_api::services::groups;

    #[tokio::test]
    async fn get_group() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "id": 1501,
            "name": "3.A",
            "description": "Class 3A",
            "status": "Active",
            "type": "Institutional",
            "institutionCode": "280371",
            "dashboardEnabled": true,
            "currentUserCanAccessGroupDashBoard": true,
            "allowMembersToBeShown": true,
            "validGroupModules": [],
            "validGroupWidgets": [],
            "memberships": []
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/groups/1501"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let group = groups::get_group(&mut session, 1501)
            .await
            .expect("should deserialize group");
        assert_eq!(group.name.as_deref(), Some("3.A"));
        assert_eq!(group.id, Some(1501));
    }

    #[tokio::test]
    async fn get_memberships_light() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "id": 10,
                "groupRole": "Member",
                "institutionProfile": null,
                "groupId": 1501,
                "institutionRole": "Guardian"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/groups/1501/memberships"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let members = groups::get_memberships_light(&mut session, 1501)
            .await
            .expect("should deserialize memberships");
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].group_id, Some(1501));
    }

    #[tokio::test]
    async fn get_group_by_context() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            { "id": 1, "name": "3.A", "showAsDefault": true },
            { "id": 2, "name": "Fritids", "showAsDefault": false }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/groups/context/12055"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let groups = groups::get_group_by_context(&mut session, 12055)
            .await
            .expect("should deserialize groups by context");
        assert_eq!(groups.len(), 2);
    }

    #[tokio::test]
    async fn join_or_leave_group() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/groups/1501/membership"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = groups::JoinOrLeaveGroupRequest {
            action: Some("join".into()),
        };
        groups::join_or_leave_group(&mut session, 1501, &req)
            .await
            .expect("join_or_leave_group should succeed");
    }
}

mod health_service {
    use super::*;
    use aula_api::services::health;

    #[tokio::test]
    async fn is_alive() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/isAlive"))
            .respond_with(json_response(serde_json::json!({"status": "ok"})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = health::is_alive(&mut session).await;
        let val = result.expect("should deserialize isAlive response");
        assert_eq!(val["status"], "ok");
    }
}

mod consent_service {
    use super::*;
    use aula_api::models::consent::{ConsentUpdateDto, ProfileConsentUpdatesDto};
    use aula_api::services::consent;

    #[tokio::test]
    async fn get_consents() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "institutionProfile": {
                    "institutionProfileId": 42,
                    "firstName": "Anna",
                    "lastName": "Jensen",
                    "fullName": "Anna Jensen",
                    "institutionCode": "101001",
                    "institutionName": "Viby Skole"
                },
                "consentResponses": [
                    {
                        "id": 1,
                        "consentId": 10,
                        "allowedAnswers": ["Accepted", "Declined"],
                        "consentDescription": "Billeder af barnet",
                        "consentResponseAnswer": "Accepted",
                        "consentResponseStatus": "Active",
                        "editable": true,
                        "viewOrder": 1
                    }
                ]
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/consents"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let consents = consent::get_consents(&mut session)
            .await
            .expect("should deserialize consents");
        assert_eq!(consents.len(), 1);
        assert_eq!(consents[0].consent_responses.as_ref().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn post_consents() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/consents"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let updates = ProfileConsentUpdatesDto {
            institution_profile_id: Some(42),
            institution_profile_consent_updates: Some(vec![ConsentUpdateDto {
                consent_id: Some(10),
                answer: Some("Accepted".into()),
            }]),
        };
        consent::post_consents(&mut session, &updates)
            .await
            .expect("post_consents should succeed");
    }
}

mod widget_service {
    use super::*;
    use aula_api::services::widget;

    #[tokio::test]
    async fn get_aula_token() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/widget/token"))
            .respond_with(json_response(serde_json::json!({
                "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.test"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let val = widget::get_aula_token(&mut session)
            .await
            .expect("should deserialize widget token");
        assert!(val["token"].as_str().unwrap().starts_with("eyJ"));
    }
}

mod onboarding_service {
    use super::*;
    use aula_api::services::onboarding;

    #[tokio::test]
    async fn mark_onboarding_complete() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/onboarding/complete"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        onboarding::mark_onboarding_complete(&mut session)
            .await
            .expect("mark_onboarding_complete should succeed");
    }

    #[tokio::test]
    async fn get_policy_links() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            { "title": "Privatlivspolitik", "url": "https://aula.dk/privacy" },
            { "title": "Vilkår", "url": "https://aula.dk/terms" }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/onboarding/policyLinks"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let links = onboarding::get_policy_links(&mut session)
            .await
            .expect("should deserialize policy links");
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].title.as_deref(), Some("Privatlivspolitik"));
    }
}

mod push_notifications_service {
    use super::*;
    use aula_api::services::push_notifications;

    #[tokio::test]
    async fn get_devices() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            { "deviceId": "dev-001" },
            { "deviceId": "dev-002" }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v19/pushNotifications/devices"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let devices = push_notifications::get_devices(&mut session)
            .await
            .expect("should deserialize devices");
        assert_eq!(devices.len(), 2);
    }

    #[tokio::test]
    async fn get_notification_settings() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "instant": true,
            "monday": true,
            "tuesday": true,
            "wednesday": true,
            "thursday": true,
            "friday": true,
            "saturday": false,
            "sunday": false,
            "notifyMessages": true,
            "notifyMessagesFromEmployees": true,
            "notifyMessagesFromChildren": false,
            "notifyMessagesFromGuardians": false,
            "notifyCalendar": true,
            "notifyGallery": true,
            "notifyPosts": true,
            "emailDisabled": false,
            "emailAvailable": true,
            "appDisabled": false,
            "appAvailable": true,
            "notifyAssignedAsSubstituteTeacher": false,
            "notifyLessonNoteChanged": true
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/pushNotifications/settings"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let settings = push_notifications::get_notification_settings(&mut session)
            .await
            .expect("should deserialize notification settings");
        assert!(settings.instant);
        assert!(!settings.saturday);
    }

    #[tokio::test]
    async fn unregister_device() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/pushNotifications/devices/dev-001"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        push_notifications::unregister_device(&mut session, "dev-001")
            .await
            .expect("unregister_device should succeed");
    }

    #[tokio::test]
    async fn clear_notification_badges() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/pushNotifications/badges/clear"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = push_notifications::ClearBadgesRequest {
            module: Some("Messages".into()),
        };
        push_notifications::clear_notification_badges(&mut session, &req)
            .await
            .expect("clear_notification_badges should succeed");
    }
}

mod comments_service {
    use super::*;
    use aula_api::enums::common::CommentType;
    use aula_api::models::posts::CommentItem;
    use aula_api::services::comments;

    #[tokio::test]
    async fn add_comment() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v19/comments"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(serde_json::json!({"id": 501})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = comments::AddCommentRequestModel {
            commentable_item: CommentItem {
                comment_type: Some(CommentType::Post),
                id: Some(33001),
            },
            content: "Great post!".into(),
            creator_inst_profile_id: 12055,
        };
        comments::add_comment(&mut session, &req)
            .await
            .expect("add_comment should succeed");
    }

    #[tokio::test]
    async fn get_comments() {
        let server = MockServer::start().await;

        let data = serde_json::json!({
            "totalResultCount": 2,
            "startIndex": 0,
            "limit": 20,
            "comments": [
                {
                    "id": 501,
                    "text": "Godt indlæg",
                    "creatorInstitutionProfileId": 4401
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/v19/comments"))
            .and(query_param("parentType", "Post"))
            .and(query_param("parentId", "33001"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = comments::GetCommentsRequestModel {
            parent_type: CommentType::Post,
            parent_id: 33001,
            start_index: Some(0),
            limit: Some(20),
        };
        let result = comments::get_comments(&mut session, &params)
            .await
            .expect("should deserialize paged comment list");
        assert_eq!(result.total_result_count, Some(2));
    }

    #[tokio::test]
    async fn delete_comment() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/comments/501"))
            .respond_with(json_response(serde_json::json!({})))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = aula_api::models::posts::DeleteCommentRequestModel {
            comment_id: Some(501),
            parent_type: Some(CommentType::Post),
        };
        comments::delete_comment(&mut session, 501, &req)
            .await
            .expect("delete_comment should succeed");
    }
}

mod files_service {
    use super::*;
    use aula_api::models::files::GetUploadLinksArguments;
    use aula_api::services::files;

    #[tokio::test]
    async fn get_upload_links() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "key": "uploads/12345/file.pdf",
                "bucket": "aula-upload",
                "url": "https://s3.amazonaws.com/aula-upload/uploads/12345/file.pdf?sig=abc"
            }
        ]);

        Mock::given(method("POST"))
            .and(path("/api/v19/files/uploadLinks"))
            .and(header("content-type", "application/json"))
            .respond_with(json_response(data))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = GetUploadLinksArguments {
            upload_names: Some(vec!["file.pdf".into()]),
            institution_code: None,
        };
        let links = files::get_upload_links(&mut session, &args)
            .await
            .expect("should deserialize upload links");
        assert_eq!(links.len(), 1);
    }
}

// ===========================================================================
// AC #6: Error handling tests
// ===========================================================================

mod error_handling_extended {
    use super::*;
    use aula_api::services::notifications;

    /// Verify that maintenance (503) propagates through service calls.
    #[tokio::test]
    async fn service_call_returns_maintenance_on_503() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/notifications"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let err = notifications::get_notifications(&mut session)
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::Maintenance),
            "expected Maintenance, got {err:?}"
        );
    }

    /// Verify that session expired (subcode 13) propagates through service calls.
    #[tokio::test]
    async fn service_call_returns_session_expired() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/notifications"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_error_envelope(13))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        // Use the raw client (no stored tokens) so Session doesn't try to refresh.
        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("notifications")
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::SessionExpired),
            "expected SessionExpired, got {err:?}"
        );
    }

    /// Verify that invalid token (subcode 9) propagates through service calls.
    #[tokio::test]
    async fn service_call_returns_invalid_token() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/notifications"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_error_envelope(9))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("notifications")
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::InvalidAccessToken),
            "expected InvalidAccessToken, got {err:?}"
        );
    }

    /// Verify that user deactivated (subcode 7) is detected on any service.
    #[tokio::test]
    async fn service_call_returns_user_deactivated() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/groups/1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_error_envelope(7))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("groups/1")
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::UserDeactivated),
            "expected UserDeactivated, got {err:?}"
        );
    }

    /// Verify that POST-based services also handle error envelopes.
    #[tokio::test]
    async fn post_service_handles_backend_error() {
        let server = MockServer::start().await;

        let body = serde_json::json!({
            "status": {
                "httpCode": 200,
                "backendErrorCode": 42,
                "message": "forbidden action"
            },
            "data": {}
        });

        Mock::given(method("POST"))
            .and(path("/api/v19/comments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .post::<serde_json::Value, _>("comments", &serde_json::json!({"content": "test"}))
            .await
            .unwrap_err();

        match err {
            AulaError::Api { message, .. } => {
                assert_eq!(message, "forbidden action");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    /// Verify 401 on DELETE requests.
    #[tokio::test]
    async fn delete_service_handles_401() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v19/notifications"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .delete::<serde_json::Value>("notifications")
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::Unauthorized),
            "expected Unauthorized, got {err:?}"
        );
    }

    /// Verify step-up required (subcode 8) is detected.
    #[tokio::test]
    async fn service_call_returns_step_up_required() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v19/documents/secure"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_error_envelope(8))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("documents/secure")
            .await
            .unwrap_err();
        assert!(
            matches!(err, AulaError::StepUpRequired),
            "expected StepUpRequired, got {err:?}"
        );
    }
}
