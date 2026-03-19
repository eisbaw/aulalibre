//! Integration tests using wiremock to simulate the Aula API.
//!
//! These tests exercise the full HTTP client -> service -> deserialization
//! pipeline without requiring real credentials or network access.

use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use aula_api::client::AulaClient;
use aula_api::error::AulaError;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Build a `Session` whose `AulaClient` points at the given mock server.
///
/// The session has no stored tokens, so automatic token refresh is skipped.
/// The CSRF cookie is pre-set so the client sends the `csrfp-token` header.
fn mock_session(base_url: &str) -> Session {
    let client = mock_client(base_url);
    // Simulate the server having set the CSRF cookie after login.
    client.set_cookie("Csrfp-Token=test-csrf-token-42; Path=/");

    let dir = std::env::temp_dir().join(format!(
        "aula_mock_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let store = TokenStore::new(&dir);
    Session::new(client, store, SessionConfig::default()).expect("session")
}

/// Build an `AulaClient` pointing at the given mock server (no CSRF cookie).
fn mock_client(base_url: &str) -> AulaClient {
    AulaClient::with_base_url(&format!("{base_url}/api/v23/")).expect("client with base URL")
}

/// Wrap a JSON payload in the standard Aula API response envelope.
fn aula_envelope(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": {
            "httpCode": 200,
            "code": 0,
            "message": null,
            "presentedMessage": null,
            "subCode": null,
            "htmlContentIfError": null
        },
        "data": data
    })
}

/// Build an Aula error envelope with the given sub-code.
///
/// Uses `"data": {}` (empty object) so it deserializes as `serde_json::Value`
/// without type errors. The error is signaled via the sub-code.
fn aula_error_envelope(sub_code: i32) -> serde_json::Value {
    serde_json::json!({
        "status": {
            "httpCode": 200,
            "code": 0,
            "subCode": sub_code
        },
        "data": {}
    })
}

/// Load a fixture file from the tests/fixtures directory.
fn fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"))
}

// ===========================================================================
// AC #1: Canned responses for key API endpoints
// ===========================================================================

mod profiles {
    use super::*;
    use aula_api::services::profiles;

    #[tokio::test]
    async fn get_profiles_by_login_returns_profiles() {
        let server = MockServer::start().await;

        // Serve the full profiles fixture (already wrapped in envelope).
        let body = fixture("profiles_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = profiles::get_profiles_by_login(&mut session).await;
        let resp = result.expect("should deserialize profiles");

        assert_eq!(resp.profiles.len(), 1);
        assert_eq!(resp.profiles[0].first_name.as_deref(), Some("Henrik"));
        assert_eq!(resp.profiles[0].last_name.as_deref(), Some("Jensen"));
    }
}

mod messaging {
    use super::*;
    use aula_api::models::messaging::GetThreadListArguments;
    use aula_api::services::messaging;

    #[tokio::test]
    async fn get_thread_list_returns_threads() {
        let server = MockServer::start().await;

        let body = fixture("messaging_thread_list.json");

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "messaging.getThreads"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let args = GetThreadListArguments {
            page: None,
            folder_id: None,
            filter_type: None,
            sort_type: None,
            sort_order: None,
            mail_box_owner_type: None,
            mail_box_owners: None,
            active_children: None,
            thread_ids: None,
        };
        let result = messaging::get_thread_list(&mut session, &args).await;
        let thread_list = result.expect("should deserialize thread list");

        let threads = thread_list.threads.expect("threads should be present");
        assert_eq!(threads.len(), 2);
        assert!(thread_list.more_messages_exist);
    }
}

mod calendar {
    use super::*;
    use aula_api::models::calendar::GetEventsParameters;
    use aula_api::services::calendar;

    #[tokio::test]
    async fn get_events_returns_events() {
        let server = MockServer::start().await;

        // Use a simplified canned response that matches EventSimpleDto fields.
        let data = serde_json::json!([
            {
                "id": 22150,
                "title": "Forældremøde 3.A",
                "type": "ParentalMeeting",
                "institutionCode": "280371",
                "allDay": false,
                "addedToInstitutionCalendar": false,
                "isPrivate": false,
                "startDateTime": "2026-04-10T19:00:00",
                "endDateTime": "2026-04-10T21:00:00",
                "responseRequired": true,
                "responseType": "Waiting"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param(
                "method",
                "calendar.getEventsByProfileIdsAndResourceIds",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_envelope(data))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let params = GetEventsParameters {
            inst_profile_ids: None,
            resource_ids: None,
            start: None,
            end: None,
            specific_types: None,
            school_calendar_institution_codes: None,
        };
        let result = calendar::get_events(&mut session, &params).await;
        let events = result.expect("should deserialize calendar events");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title.as_deref(), Some("Forældremøde 3.A"));
    }
}

mod presence {
    use super::*;
    use aula_api::services::presence;

    #[tokio::test]
    async fn get_childrens_state_returns_statuses() {
        let server = MockServer::start().await;

        let data = serde_json::json!([
            {
                "institutionProfileId": 14201,
                "state": "Present",
                "uniStudent": {
                    "id": 8801,
                    "name": "Emma Jensen",
                    "shortName": "EJ",
                    "profilePicture": null
                }
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "presence.getPresenceStates"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_envelope(data))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = presence::get_childrens_state(&mut session, &[14201]).await;
        let statuses = result.expect("should deserialize children state");

        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].institution_profile_id, 14201);
    }
}

// ===========================================================================
// AC #2: Request validation (methods, paths, headers)
// ===========================================================================

mod request_validation {
    use super::*;
    use aula_api::services::profiles;

    #[tokio::test]
    async fn sends_csrf_header_on_get() {
        let server = MockServer::start().await;

        let body = fixture("profiles_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .and(header("csrfp-token", "test-csrf-token-42"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let result = profiles::get_profiles_by_login(&mut session).await;
        result.expect("request should include csrfp-token header");

        // wiremock's expect(1) assertion verifies the request matched
        // including the csrf header matcher.
    }

    #[tokio::test]
    async fn sends_correct_user_agent() {
        let server = MockServer::start().await;

        let body = fixture("profiles_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .and(header("user-agent", "Android"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        profiles::get_profiles_by_login(&mut session)
            .await
            .expect("request should include user-agent");
    }

    #[tokio::test]
    async fn sends_accept_json_header() {
        let server = MockServer::start().await;

        let body = fixture("profiles_response.json");

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .and(header("accept", "application/json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        profiles::get_profiles_by_login(&mut session)
            .await
            .expect("request should include accept header");
    }

    #[tokio::test]
    async fn post_sends_json_content_type() {
        let server = MockServer::start().await;
        use aula_api::services::profiles::{post_master_data, UpdateMasterDataRequest};

        let response_data = aula_envelope(serde_json::json!({}));

        Mock::given(method("POST"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.updateProfileMasterData"))
            .and(header("content-type", "application/json"))
            .and(header("csrfp-token", "test-csrf-token-42"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(response_data)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let req = UpdateMasterDataRequest {
            external_email: Some("test@test.dk".to_string()),
            phonenumber: None,
            work_phonenumber: None,
            home_phonenumber: None,
            mobile_phonenumber: None,
        };
        post_master_data(&mut session, &req)
            .await
            .expect("POST should have correct headers");
    }
}

// ===========================================================================
// AC #3: CSRF flow simulation
// ===========================================================================

mod csrf_flow {
    use super::*;
    use aula_api::services::profiles;

    #[tokio::test]
    async fn csrf_cookie_is_echoed_as_header() {
        let server = MockServer::start().await;

        let body = fixture("profiles_response.json");

        // The mock requires the csrfp-token header to match the cookie value.
        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .and(header("csrfp-token", "test-csrf-token-42"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        profiles::get_profiles_by_login(&mut session)
            .await
            .expect("CSRF token from cookie should be sent as header");
    }

    #[tokio::test]
    async fn request_without_csrf_cookie_has_no_csrf_header() {
        let server = MockServer::start().await;

        let body = fixture("profiles_response.json");

        // This mock requires the csrfp-token header and should NOT match
        // when the client has no CSRF cookie.
        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .and(header("csrfp-token", "test-csrf-token-42"))
            .respond_with(ResponseTemplate::new(200).set_body_string(&body))
            .expect(0) // Should NOT be called
            .named("with-csrf")
            .mount(&server)
            .await;

        // Mount a fallback that matches without the csrf header constraint.
        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .named("without-csrf")
            .mount(&server)
            .await;

        // Build a session WITHOUT setting the CSRF cookie.
        let client = mock_client(&server.uri());
        // No set_cookie call here.
        let dir = std::env::temp_dir().join(format!("aula_mock_no_csrf_{}", std::process::id()));
        let store = TokenStore::new(&dir);
        let mut session = Session::new(client, store, SessionConfig::default()).unwrap();

        profiles::get_profiles_by_login(&mut session)
            .await
            .expect("should work without CSRF (though real API would reject it)");
    }
}

// ===========================================================================
// AC #4: Error condition simulation
// ===========================================================================

mod error_conditions {
    use super::*;

    /// Test 401 Unauthorized via AulaClient directly (Session retries on 401).
    #[tokio::test]
    async fn unauthorized_401() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(ResponseTemplate::new(401))
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::Unauthorized),
            "expected Unauthorized, got {err:?}"
        );
    }

    #[tokio::test]
    async fn maintenance_503() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::Maintenance),
            "expected Maintenance, got {err:?}"
        );
    }

    #[tokio::test]
    async fn session_expired_subcode_13() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(aula_error_envelope(13))
                    .insert_header("content-type", "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = mock_client(&server.uri());
        let err = client
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::SessionExpired),
            "expected SessionExpired, got {err:?}"
        );
    }

    #[tokio::test]
    async fn invalid_token_subcode_9() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
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
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::InvalidAccessToken),
            "expected InvalidAccessToken, got {err:?}"
        );
    }

    #[tokio::test]
    async fn user_deactivated_subcode_7() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
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
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::UserDeactivated),
            "expected UserDeactivated, got {err:?}"
        );
    }

    #[tokio::test]
    async fn backend_error_code_nonzero() {
        let server = MockServer::start().await;

        let body = serde_json::json!({
            "status": {
                "httpCode": 200,
                "code": 99,
                "message": "internal server error"
            },
            "data": {}
        });

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
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
            .get::<serde_json::Value>("?method=profiles.getprofilesbylogin")
            .await
            .unwrap_err();

        match err {
            AulaError::Api { message, .. } => {
                assert_eq!(message, "internal server error");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }

    /// Test that Session surfaces maintenance errors (503 bypasses retry logic).
    #[tokio::test]
    async fn session_surfaces_maintenance() {
        let server = MockServer::start().await;
        use aula_api::services::profiles;

        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;

        let mut session = mock_session(&server.uri());
        let err = profiles::get_profiles_by_login(&mut session)
            .await
            .unwrap_err();

        assert!(
            matches!(err, AulaError::Maintenance),
            "expected Maintenance through Session, got {err:?}"
        );
    }
}

// ===========================================================================
// AC #5 & #6: Helper functions and reusability demonstration
// ===========================================================================

mod helpers_and_reuse {
    use super::*;
    use aula_api::models::messaging::GetThreadListArguments;
    use aula_api::services::{messaging, profiles};

    /// Demonstrates that mock_session and aula_envelope helpers can be reused
    /// across different service modules in a single test.
    #[tokio::test]
    async fn mock_helpers_work_across_services() {
        let server = MockServer::start().await;

        // Mount mocks for two different services on the same server.
        let profiles_body = fixture("profiles_response.json");
        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "profiles.getprofilesbylogin"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(profiles_body)
                    .insert_header("content-type", "application/json"),
            )
            .mount(&server)
            .await;

        let threads_body = fixture("messaging_thread_list.json");
        Mock::given(method("GET"))
            .and(path("/api/v23/"))
            .and(query_param("method", "messaging.getThreads"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(threads_body)
                    .insert_header("content-type", "application/json"),
            )
            .mount(&server)
            .await;

        // One session, multiple service calls.
        let mut session = mock_session(&server.uri());

        let profs = profiles::get_profiles_by_login(&mut session)
            .await
            .expect("profiles should work");
        assert!(!profs.profiles.is_empty());

        let args = GetThreadListArguments {
            page: None,
            folder_id: None,
            filter_type: None,
            sort_type: None,
            sort_order: None,
            mail_box_owner_type: None,
            mail_box_owners: None,
            active_children: None,
            thread_ids: None,
        };
        let threads = messaging::get_thread_list(&mut session, &args)
            .await
            .expect("messaging should work");
        assert!(!threads.threads.expect("threads").is_empty());
    }
}
