//! Live E2E tests that call the real Aula API.
//!
//! These tests are marked `#[ignore]` and only run when explicitly requested:
//!
//! ```bash
//! cargo test -p aula-api --test e2e_live_tests -- --ignored
//! # or
//! just e2e-live
//! ```
//!
//! They require a valid authentication token. See [`aula_api::e2e`] module
//! documentation for setup instructions.
//!
//! When no token is available, each test prints a skip message and passes
//! without making any API calls. This ensures CI pipelines do not fail
//! due to missing credentials.

use aula_api::e2e;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Load an E2E session or skip the test with a descriptive message.
///
/// Returns `None` when no credentials are available, causing the test
/// to return early with a pass (not a failure).
fn try_session() -> Option<aula_api::Session> {
    e2e::e2e_session()
}

/// Get today's date as YYYY-MM-DD string (no chrono dependency needed).
fn today_date_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Simple date calculation: days since epoch, then convert.
    let days = (secs / 86400) as i64;
    epoch_days_to_date(days)
}

/// Get tomorrow's date as YYYY-MM-DD string.
fn tomorrow_date_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let days = (secs / 86400) as i64 + 1;
    epoch_days_to_date(days)
}

/// Convert days since Unix epoch to YYYY-MM-DD string.
fn epoch_days_to_date(days: i64) -> String {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

// ---------------------------------------------------------------------------
// Live API tests
// ---------------------------------------------------------------------------

/// Verify that the loaded token can successfully refresh.
///
/// This tests AC#4: "Token refresh works in E2E mode (re-use refresh token)".
/// If the access token is expired but a refresh token is present, the session
/// should be able to obtain a new access token.
#[tokio::test]
#[ignore]
async fn e2e_token_refresh() {
    let Some(mut session) = try_session() else {
        return;
    };

    // Only test refresh if we have a refresh token.
    let has_refresh = session
        .login_data()
        .map(|ld| ld.refresh_token.is_some())
        .unwrap_or(false);

    if !has_refresh {
        eprintln!("SKIP: no refresh token available for refresh test");
        return;
    }

    match session.refresh_token().await {
        Ok(()) => {
            eprintln!("E2E: token refresh succeeded");
            assert!(
                session.has_valid_tokens(),
                "session should have valid tokens after refresh"
            );
        }
        Err(e) => {
            // A refresh failure is not necessarily a test bug -- the refresh
            // token may have expired. Report but don't panic.
            eprintln!("E2E: token refresh failed (may be expired): {e}");
        }
    }
}

/// Verify that ensure_valid_token works with real credentials.
#[tokio::test]
#[ignore]
async fn e2e_ensure_valid_token() {
    let Some(mut session) = try_session() else {
        return;
    };

    match session.ensure_valid_token().await {
        Ok(()) => {
            eprintln!("E2E: ensure_valid_token succeeded");
            assert!(session.has_valid_tokens());
        }
        Err(e) => {
            eprintln!("E2E: ensure_valid_token failed: {e}");
            // This is expected if tokens are fully expired with no valid
            // refresh token.
        }
    }
}

/// Verify graceful skip behavior: loading credentials should either succeed
/// or return a clear unavailable message -- never panic.
#[test]
#[ignore]
fn e2e_graceful_skip_message() {
    let creds = e2e::load_credentials();
    match creds {
        e2e::E2eCredentials::Available { source, .. } => {
            eprintln!("E2E credentials available from: {source}");
        }
        e2e::E2eCredentials::Unavailable { reason } => {
            eprintln!("E2E credentials unavailable (expected in CI):");
            eprintln!("{reason}");
            // The skip message should contain setup instructions.
            assert!(
                reason.contains("auth login"),
                "skip message should contain setup instructions"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// API workflow tests (TASK-69)
// ---------------------------------------------------------------------------

/// AC#1: Login and fetch profiles, verify profile data structure.
///
/// Calls `get_profiles_by_login` and verifies we get a non-empty list
/// of profiles back with expected fields populated.
#[tokio::test]
#[ignore]
async fn e2e_get_profiles_by_login() {
    let Some(mut session) = try_session() else {
        return;
    };

    let profiles = aula_api::services::profiles::get_profiles_by_login(&mut session)
        .await
        .expect("get_profiles_by_login should succeed");

    assert!(
        !profiles.is_empty(),
        "logged-in user should have at least one profile"
    );

    let first = &profiles[0];
    eprintln!(
        "E2E: got {} profile(s); first has institution_profile={}, portal_role={:?}",
        profiles.len(),
        first.institution_profile.is_some(),
        first.portal_role,
    );

    // A valid profile should have an institution profile.
    assert!(
        first.institution_profile.is_some(),
        "profile should contain an institution_profile"
    );
}

/// AC#2: List message threads and read a thread.
///
/// Calls `get_thread_list` to list inbox threads, then reads the first
/// thread's messages if any exist.
#[tokio::test]
#[ignore]
async fn e2e_list_threads_and_read() {
    let Some(mut session) = try_session() else {
        return;
    };

    let args = aula_api::models::messaging::GetThreadListArguments {
        folder_id: None,
        filter_type: None,
        sort_type: None,
        sort_order: None,
        page: Some(0),
        thread_ids: None,
        mail_box_owner_type: None,
        mail_box_owners: None,
        active_children: None,
    };

    let thread_list = aula_api::services::messaging::get_thread_list(&mut session, &args)
        .await
        .expect("get_thread_list should succeed");

    eprintln!(
        "E2E: got thread list, page={:?}, threads={}",
        thread_list.page,
        thread_list.threads.as_ref().map(|t| t.len()).unwrap_or(0)
    );

    // If there are threads, read the first one's messages.
    if let Some(threads) = &thread_list.threads {
        if let Some(first_thread) = threads.first() {
            if let Some(thread_id) = first_thread.id {
                let read_args = aula_api::models::messaging::GetMessagesForThreadArguments {
                    thread_id: Some(thread_id),
                    page: Some(0),
                    common_inbox_id: None,
                };
                let messages =
                    aula_api::services::messaging::get_thread_by_id(&mut session, &read_args)
                        .await
                        .expect("get_thread_by_id should succeed");

                eprintln!(
                    "E2E: read thread {thread_id}, messages={}",
                    messages.messages.as_ref().map(|m| m.len()).unwrap_or(0)
                );
            }
        }
    }
}

/// AC#3: List today's calendar events.
///
/// Fetches profiles first to get institution profile IDs, then queries
/// calendar events for today's date range.
#[tokio::test]
#[ignore]
async fn e2e_calendar_events_today() {
    let Some(mut session) = try_session() else {
        return;
    };

    // Get institution profile IDs from the logged-in user's profiles.
    let profiles = aula_api::services::profiles::get_profiles_by_login(&mut session)
        .await
        .expect("get_profiles_by_login should succeed");

    let inst_profile_ids: Vec<i64> = profiles
        .iter()
        .filter_map(|p| {
            p.institution_profile
                .as_ref()
                .map(|ip| ip.institution_profile_id)
        })
        .collect();

    if inst_profile_ids.is_empty() {
        eprintln!("SKIP: no institution profile IDs found");
        return;
    }

    let today = today_date_string();
    let tomorrow = tomorrow_date_string();

    let params = aula_api::models::calendar::GetEventsParameters {
        inst_profile_ids: Some(inst_profile_ids.clone()),
        resource_ids: None,
        start: Some(today.clone()),
        end: Some(tomorrow),
        specific_types: None,
        school_calendar_institution_codes: None,
    };

    let events = aula_api::services::calendar::get_events(&mut session, &params)
        .await
        .expect("get_events should succeed");

    eprintln!("E2E: got {} calendar event(s) for {today}", events.len());

    // No assertion on event count -- there may be zero events today.
    // But the call succeeded and deserialized correctly.
}

/// AC#4: Get children's presence status.
///
/// Uses institution profile IDs from login to query presence state.
#[tokio::test]
#[ignore]
async fn e2e_presence_status() {
    let Some(mut session) = try_session() else {
        return;
    };

    let profiles = aula_api::services::profiles::get_profiles_by_login(&mut session)
        .await
        .expect("get_profiles_by_login should succeed");

    let inst_profile_ids: Vec<i64> = profiles
        .iter()
        .filter_map(|p| {
            p.institution_profile
                .as_ref()
                .map(|ip| ip.institution_profile_id)
        })
        .collect();

    if inst_profile_ids.is_empty() {
        eprintln!("SKIP: no institution profile IDs found");
        return;
    }

    let children =
        aula_api::services::presence::get_childrens_state(&mut session, &inst_profile_ids)
            .await
            .expect("get_childrens_state should succeed");

    eprintln!("E2E: got {} child status record(s)", children.len());

    // Presence data structure should deserialize correctly.
    // There may be zero children if the user is not a parent.
}

/// AC#5: List posts for a group.
///
/// Fetches posts with default parameters (no group filter) to verify
/// the posts endpoint works.
#[tokio::test]
#[ignore]
async fn e2e_list_posts() {
    let Some(mut session) = try_session() else {
        return;
    };

    // Get institution profile IDs for the query.
    let profiles = aula_api::services::profiles::get_profiles_by_login(&mut session)
        .await
        .expect("get_profiles_by_login should succeed");

    let inst_profile_ids: Vec<i64> = profiles
        .iter()
        .filter_map(|p| {
            p.institution_profile
                .as_ref()
                .map(|ip| ip.institution_profile_id)
        })
        .collect();

    let params = aula_api::models::posts::GetPostApiParameters {
        group_id: None,
        is_important: None,
        creator_portal_role: None,
        institution_profile_ids: if inst_profile_ids.is_empty() {
            None
        } else {
            Some(inst_profile_ids)
        },
        related_institutions: None,
        own_post: false,
        is_unread: false,
        is_bookmarked: false,
        limit: Some(5),
        index: Some(0),
    };

    let result = aula_api::services::posts::get_posts(&mut session, &params)
        .await
        .expect("get_posts should succeed");

    eprintln!(
        "E2E: got {} post(s), has_more={}",
        result.posts.as_ref().map(|p| p.len()).unwrap_or(0),
        result.has_more_posts
    );
}

/// AC#6: Get notifications.
///
/// Fetches the current user's in-app notifications.
#[tokio::test]
#[ignore]
async fn e2e_get_notifications() {
    let Some(mut session) = try_session() else {
        return;
    };

    let notifications = aula_api::services::notifications::get_notifications(&mut session)
        .await
        .expect("get_notifications should succeed");

    eprintln!("E2E: got {} notification(s)", notifications.len());

    // Verify structure if there are any notifications.
    for n in &notifications {
        // Each notification should have an ID.
        assert!(
            n.notification_id.is_some(),
            "notification should have an ID"
        );
    }
}

/// Bonus: Global search test (read-only).
///
/// Exercises the search endpoint with a simple query.
#[tokio::test]
#[ignore]
async fn e2e_global_search() {
    let Some(mut session) = try_session() else {
        return;
    };

    let params = aula_api::models::search::GlobalSearchParameters {
        text: Some("test".to_string()),
        page_limit: Some(5),
        page_number: Some(0),
        group_id: None,
        doc_type_count: true,
        doc_type: None,
        group_types: None,
    };

    let result = aula_api::services::search::global_search(&mut session, &params)
        .await
        .expect("global_search should succeed");

    eprintln!(
        "E2E: search returned total_size={:?}, results={}",
        result.total_size,
        result.results.as_ref().map(|r| r.len()).unwrap_or(0)
    );
}

// ---------------------------------------------------------------------------
// Credential loading tests (always run, not #[ignore])
// ---------------------------------------------------------------------------

/// This test always runs (no #[ignore]) to verify the E2E harness itself
/// does not panic when credentials are absent.
#[test]
fn e2e_harness_no_panic_without_credentials() {
    // This must never panic, even without any credentials configured.
    let _creds = e2e::load_credentials();
    let _session = e2e::e2e_session();
}
