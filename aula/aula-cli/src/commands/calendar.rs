//! Calendar subcommands: list events, show details, today, week, respond, birthdays.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::enums::calendar::ResponseType;
use aula_api::models::calendar::{
    EventDetailsDto, EventSimpleDto, GetEventsParameters, RespondSimpleEventRequest,
};
use aula_api::services::calendar;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage calendar events.
#[derive(Debug, Subcommand)]
pub enum CalendarCommand {
    /// List calendar events for a date range.
    List {
        /// Start date (YYYY-MM-DD). Defaults to today.
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD). Defaults to 7 days from start.
        #[arg(long)]
        to: Option<String>,
        /// Filter by group ID.
        #[arg(long)]
        group: Option<i64>,
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Show details for a single event.
    Show {
        /// Event ID.
        event_id: u64,
    },
    /// Show today's events (shortcut for list --from today --to today).
    Today,
    /// Show this week's events (shortcut for list --from today --to +7 days).
    Week,
    /// Respond to an event invitation.
    Respond {
        /// Event ID.
        event_id: u64,
        /// Accept the invitation.
        #[arg(long, group = "response")]
        accept: bool,
        /// Decline the invitation.
        #[arg(long, group = "response")]
        decline: bool,
        /// Respond tentatively.
        #[arg(long, group = "response")]
        tentative: bool,
        /// Institution profile ID (required for response).
        #[arg(long)]
        profile: Option<i64>,
    },
    /// Show birthdays for a group or institution.
    Birthdays {
        /// Group ID to show birthdays for.
        #[arg(long)]
        group: Option<i64>,
        /// Institution ID to show birthdays for.
        #[arg(long)]
        institution: Option<i64>,
        /// Start date (YYYY-MM-DD). Defaults to today.
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD). Defaults to 30 days from start.
        #[arg(long)]
        to: Option<String>,
    },
}

// ---------------------------------------------------------------------------
// Session helper (same pattern as messages.rs)
// ---------------------------------------------------------------------------

fn resolve_environment(env: Option<&str>) -> aula_api::client::Environment {
    match env {
        Some("preprod") => aula_api::client::Environment::Preprod,
        Some("hotfix") => aula_api::client::Environment::Hotfix,
        Some("test1") => aula_api::client::Environment::Test1,
        Some("test3") => aula_api::client::Environment::Test3,
        Some("dev1") => aula_api::client::Environment::Dev1,
        Some("dev3") => aula_api::client::Environment::Dev3,
        Some("dev11") => aula_api::client::Environment::Dev11,
        _ => aula_api::client::Environment::Production,
    }
}

fn token_store() -> TokenStore {
    TokenStore::default_location().unwrap_or_else(|| {
        eprintln!("warning: could not determine data directory, using ./aula-data");
        TokenStore::new("./aula-data")
    })
}

fn build_session(env_override: Option<&str>) -> Session {
    let environment = resolve_environment(env_override);
    let store = token_store();

    if !store.exists() {
        eprintln!("Not logged in. Run 'aula auth login' first.");
        std::process::exit(1);
    }

    let client = match AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 19,
    }) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to create client: {e}");
            std::process::exit(1);
        }
    };

    match Session::new(client, store, SessionConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to create session: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Date helpers
// ---------------------------------------------------------------------------

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn days_from_today(days: i64) -> String {
    (chrono::Local::now() + chrono::Duration::days(days))
        .format("%Y-%m-%d")
        .to_string()
}

// ---------------------------------------------------------------------------
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &CalendarCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        CalendarCommand::List {
            from,
            to,
            group,
            institution,
        } => {
            let start = from.clone().unwrap_or_else(today);
            let end = to.clone().unwrap_or_else(|| days_from_today(7));
            if let Some(gid) = group {
                handle_list_group(*gid, &start, &end, json, env_override).await;
            } else {
                handle_list(
                    &start,
                    &end,
                    institution.map(|id| id as i64),
                    json,
                    env_override,
                )
                .await;
            }
        }
        CalendarCommand::Show { event_id } => {
            handle_show(*event_id as i64, json, env_override).await
        }
        CalendarCommand::Today => {
            let start = today();
            let end = days_from_today(1);
            handle_list(&start, &end, None, json, env_override).await;
        }
        CalendarCommand::Week => {
            let start = today();
            let end = days_from_today(7);
            handle_list(&start, &end, None, json, env_override).await;
        }
        CalendarCommand::Respond {
            event_id,
            accept,
            decline,
            tentative,
            profile,
        } => {
            handle_respond(
                *event_id as i64,
                *accept,
                *decline,
                *tentative,
                *profile,
                json,
                env_override,
            )
            .await
        }
        CalendarCommand::Birthdays {
            group,
            institution,
            from,
            to,
        } => {
            let start = from.clone().unwrap_or_else(today);
            let end = to.clone().unwrap_or_else(|| days_from_today(30));
            handle_birthdays(*group, *institution, &start, &end, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// List events
// ---------------------------------------------------------------------------

async fn handle_list(
    start: &str,
    end: &str,
    inst_profile_id: Option<i64>,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let params = GetEventsParameters {
        inst_profile_ids: inst_profile_id.map(|id| vec![id]),
        resource_ids: None,
        start: Some(start.to_string()),
        end: Some(end.to_string()),
        specific_types: None,
        school_calendar_institution_codes: None,
    };

    match calendar::get_events(&mut session, &params).await {
        Ok(events) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&events).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_event_list(&events);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

async fn handle_list_group(
    group_id: i64,
    start: &str,
    end: &str,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    match calendar::get_event_for_group(&mut session, group_id, Some(start), Some(end)).await {
        Ok(events) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&events).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_event_list(&events);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_event_list(events: &[EventSimpleDto]) {
    if events.is_empty() {
        println!("No events found.");
        return;
    }

    println!(
        "{:<8} {:<12} {:<6} {:<30} {:<14} {:<10}",
        "ID", "DATE", "TIME", "TITLE", "TYPE", "RESPONSE"
    );
    println!("{}", "-".repeat(82));

    for event in events {
        let id = event
            .id
            .map(|id| id.to_string())
            .unwrap_or_else(|| "-".to_string());

        let (date, time) = extract_date_time(event.start_date_time.as_deref());

        let all_day = event.all_day.unwrap_or(false);
        let time_display = if all_day { "all-day".to_string() } else { time };

        let title = truncate(event.title.as_deref().unwrap_or("(untitled)"), 30);
        let event_type = event.event_type.as_deref().unwrap_or("");
        let response = event
            .response_status
            .as_ref()
            .map(|r| format!("{r:?}"))
            .unwrap_or_default();

        println!(
            "{:<8} {:<12} {:<6} {:<30} {:<14} {:<10}",
            id, date, time_display, title, event_type, response
        );
    }

    println!("\n{} event(s) total.", events.len());
}

// ---------------------------------------------------------------------------
// Show event detail
// ---------------------------------------------------------------------------

async fn handle_show(event_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match calendar::get_event_detail(&mut session, event_id).await {
        Ok(detail) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&detail).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_event_detail(&detail);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_event_detail(detail: &EventDetailsDto) {
    let title = detail.title.as_deref().unwrap_or("(untitled)");
    let event_type = detail.event_type.as_deref().unwrap_or("unknown");

    println!("Event: {title}");
    println!("  Type: {event_type}");

    if let Some(id) = detail.id {
        println!("  ID: {id}");
    }

    // Date/time
    if let Some(ref start) = detail.start_date_time {
        print!("  Start: {start}");
    }
    if let Some(ref end) = detail.end_date_time {
        print!("  End: {end}");
    }
    if detail.all_day == Some(true) {
        print!("  (all day)");
    }
    println!();

    // Location / resource
    if let Some(ref text) = detail.primary_resource_text {
        println!("  Location: {text}");
    } else if let Some(ref res) = detail.primary_resource {
        if let Some(ref name) = res.name {
            println!("  Resource: {name}");
        }
    }

    // Institution
    if let Some(ref code) = detail.institution_code {
        println!("  Institution: {code}");
    }

    // Response status
    if let Some(ref status) = detail.response_status {
        println!("  Your response: {status:?}");
    }

    if detail.response_required == Some(true) {
        print!("  [Response required]");
        if let Some(ref deadline) = detail.response_deadline {
            print!("  Deadline: {deadline}");
        }
        println!();
    }

    // Creator
    if let Some(ref creator) = detail.creator {
        if let Some(ref name) = creator.name {
            println!("  Created by: {name}");
        }
    }

    // Description
    if let Some(ref desc) = detail.description {
        if let Some(ref html) = desc.html {
            let plain = strip_html_tags(html);
            if !plain.trim().is_empty() {
                println!();
                println!("Description:");
                println!("{}", plain.trim());
            }
        }
    }

    // Invited groups
    if let Some(ref groups) = detail.invited_groups {
        if !groups.is_empty() {
            println!();
            println!("Invited groups:");
            for g in groups {
                let name = g.name.as_deref().unwrap_or("(unnamed)");
                println!("  - {name}");
            }
        }
    }

    // Invitees
    if let Some(ref invitees) = detail.invitees {
        if !invitees.is_empty() {
            println!();
            println!("Invitees ({}):", invitees.len());
            for inv in invitees {
                let name = inv
                    .inst_profile
                    .as_ref()
                    .and_then(|p| p.full_name.as_deref())
                    .unwrap_or("(unknown)");
                let response = inv
                    .response_type
                    .as_ref()
                    .map(|r| format!("{r:?}"))
                    .unwrap_or_else(|| "N/A".to_string());
                println!("  - {name} [{response}]");
            }
        }
    }

    // Attachments
    if let Some(ref attachments) = detail.attachments {
        if !attachments.is_empty() {
            println!();
            println!("Attachments:");
            for att in attachments {
                let name = att.name.as_deref().unwrap_or("(unnamed)");
                println!("  - {name}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Respond to event
// ---------------------------------------------------------------------------

async fn handle_respond(
    event_id: i64,
    accept: bool,
    decline: bool,
    tentative: bool,
    profile_id: Option<i64>,
    json: bool,
    env_override: Option<&str>,
) {
    let response_type = if accept {
        ResponseType::Accepted
    } else if decline {
        ResponseType::Declined
    } else if tentative {
        ResponseType::Tentative
    } else {
        eprintln!("error: specify one of --accept, --decline, or --tentative");
        std::process::exit(1);
    };

    let mut session = build_session(env_override);

    let args = RespondSimpleEventRequest {
        event_id: Some(event_id),
        institution_profile_id: profile_id,
        invited_inst_profile_id: profile_id,
        response_type: Some(response_type),
        occurrence_date_time: None,
        number_of_adult_participants: None,
        number_of_child_participants: None,
    };

    match calendar::respond_simple_event(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Responded to event {event_id}: {response_type:?}");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Birthdays
// ---------------------------------------------------------------------------

async fn handle_birthdays(
    group_id: Option<i64>,
    institution_id: Option<i64>,
    start: &str,
    end: &str,
    json: bool,
    env_override: Option<&str>,
) {
    if group_id.is_none() && institution_id.is_none() {
        eprintln!("error: specify --group <id> or --institution <id>");
        std::process::exit(1);
    }

    let mut session = build_session(env_override);

    let result = if let Some(gid) = group_id {
        calendar::get_birthdays_for_group(&mut session, gid, start, end).await
    } else {
        calendar::get_birthdays_for_institution(&mut session, institution_id.unwrap(), start, end)
            .await
    };

    match result {
        Ok(birthdays) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&birthdays).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if birthdays.is_empty() {
                println!("No birthdays found.");
            } else {
                println!("{:<12} {:<25} {:<20}", "DATE", "NAME", "GROUP");
                println!("{}", "-".repeat(58));
                for bday in &birthdays {
                    let date = bday.birthday.as_deref().unwrap_or("");
                    let name = bday.name.as_deref().unwrap_or("(unknown)");
                    let group = bday.main_group_name.as_deref().unwrap_or("");
                    println!("{:<12} {:<25} {:<20}", date, truncate(name, 25), group);
                }
                println!("\n{} birthday(s) total.", birthdays.len());
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract date and time portions from an ISO datetime string.
fn extract_date_time(datetime: Option<&str>) -> (String, String) {
    match datetime {
        Some(dt) if dt.len() >= 16 => {
            let date = &dt[..10];
            let time = &dt[11..16];
            (date.to_string(), time.to_string())
        }
        Some(dt) if dt.len() >= 10 => (dt[..10].to_string(), String::new()),
        Some(dt) => (dt.to_string(), String::new()),
        None => (String::new(), String::new()),
    }
}

/// Truncate a string to at most `max` characters, appending "..." if needed.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

/// Very basic HTML tag stripping for terminal display.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}
