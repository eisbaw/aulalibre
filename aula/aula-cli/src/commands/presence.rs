//! Presence/attendance subcommands: status, schedule, report-absence, history.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::models::presence::PresenceSchedulesRequest;
use aula_api::services::presence;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage child presence (attendance).
#[derive(Debug, Subcommand)]
pub enum PresenceCommand {
    /// Show current presence status for children.
    Status {
        /// Institution profile IDs to query (comma-separated).
        #[arg(long, value_delimiter = ',')]
        children: Vec<i64>,
    },
    /// Show presence registrations for a date.
    Registrations {
        /// Institution profile IDs (comma-separated).
        #[arg(long, value_delimiter = ',')]
        children: Vec<i64>,
        /// Date to query (YYYY-MM-DD). Defaults to today.
        #[arg(long)]
        date: Option<String>,
    },
    /// Show weekly presence schedule.
    Schedule {
        /// Institution profile IDs (comma-separated).
        #[arg(long, value_delimiter = ',')]
        children: Vec<i64>,
        /// Start date (YYYY-MM-DD).
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD).
        #[arg(long)]
        to: Option<String>,
    },
    /// Report a child as sick or absent by institution profile ID.
    ReportStatus {
        /// Child institution profile IDs (comma-separated).
        #[arg(long, value_delimiter = ',')]
        children: Vec<i64>,
        /// Status code: 0=NotPresent, 1=Sick, 2=ReportedAbsence, 3=Present.
        #[arg(long)]
        status: i32,
    },
}

// ---------------------------------------------------------------------------
// Session helper (same pattern as messages/calendar)
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
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &PresenceCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        PresenceCommand::Status { children } => {
            handle_status(children, json, env_override).await;
        }
        PresenceCommand::Registrations { children, date } => {
            handle_registrations(children, date.as_deref(), json, env_override).await;
        }
        PresenceCommand::Schedule { children, from, to } => {
            handle_schedule(children, from.as_deref(), to.as_deref(), json, env_override).await;
        }
        PresenceCommand::ReportStatus { children, status } => {
            handle_report_status(children, *status, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------

async fn handle_status(children: &[i64], json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match presence::get_childrens_state(&mut session, children).await {
        Ok(states) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&states).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if states.is_empty() {
                println!("No presence status found.");
            } else {
                println!("{:<12} {:<15} {:<20}", "PROFILE ID", "STATUS", "NAME");
                println!("{}", "-".repeat(50));
                for s in &states {
                    let status = s
                        .state
                        .as_ref()
                        .map(|st| format!("{st:?}"))
                        .unwrap_or_else(|| "(unknown)".to_string());
                    let name = s
                        .uni_student
                        .as_ref()
                        .and_then(|u| u.name.as_deref())
                        .unwrap_or("(unknown)");
                    println!(
                        "{:<12} {:<15} {:<20}",
                        s.institution_profile_id, status, name
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Registrations
// ---------------------------------------------------------------------------

async fn handle_registrations(
    children: &[i64],
    date: Option<&str>,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    match presence::get_presence_registrations(&mut session, children, date).await {
        Ok(regs) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&regs).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if regs.is_empty() {
                println!("No presence registrations found.");
            } else {
                println!(
                    "{:<8} {:<15} {:<12} {:<12} {:<10}",
                    "ID", "STATUS", "CHECK-IN", "CHECK-OUT", "COMMENT"
                );
                println!("{}", "-".repeat(60));
                for r in &regs {
                    let status = r
                        .status
                        .as_ref()
                        .map(|s| format!("{s:?}"))
                        .unwrap_or_default();
                    let checkin = r
                        .check_in_time
                        .as_deref()
                        .map(truncate_time)
                        .unwrap_or_default();
                    let checkout = r
                        .check_out_time
                        .as_deref()
                        .map(truncate_time)
                        .unwrap_or_default();
                    let comment = r.comment.as_deref().unwrap_or("");
                    println!(
                        "{:<8} {:<15} {:<12} {:<12} {:<10}",
                        r.id,
                        status,
                        checkin,
                        checkout,
                        truncate(comment, 10)
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Schedule
// ---------------------------------------------------------------------------

async fn handle_schedule(
    children: &[i64],
    from: Option<&str>,
    to: Option<&str>,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let args = PresenceSchedulesRequest {
        filter_institution_profile_ids: if children.is_empty() {
            None
        } else {
            Some(children.to_vec())
        },
        from_date: from.map(|s| s.to_string()),
        to_date: to.map(|s| s.to_string()),
    };

    match presence::get_presence_schedules(&mut session, &args).await {
        Ok(schedules) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&schedules).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if schedules.is_empty() {
                println!("No schedules found.");
            } else {
                // Schedules come as generic JSON since the model is Vec<serde_json::Value>.
                for (i, sched) in schedules.iter().enumerate() {
                    println!("Schedule #{}", i + 1);
                    println!(
                        "{}",
                        serde_json::to_string_pretty(sched).unwrap_or_else(|_| "(error)".into())
                    );
                    println!();
                }
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Report Status (sick/absent)
// ---------------------------------------------------------------------------

async fn handle_report_status(
    children: &[i64],
    status: i32,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    use aula_api::models::presence::UpdateStatusByInstitutionProfileIds;

    let args = UpdateStatusByInstitutionProfileIds {
        institution_profile_ids: Some(children.to_vec()),
        status,
    };

    match presence::update_status_by_institution_profile_ids(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                let status_name = match status {
                    0 => "NotPresent",
                    1 => "Sick",
                    2 => "ReportedAbsence",
                    3 => "Present",
                    _ => "Unknown",
                };
                println!(
                    "Status updated to {status_name} for {} profile(s).",
                    children.len()
                );
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

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

fn truncate_time(s: &str) -> String {
    // Extract HH:MM from datetime strings like "2024-01-15T08:30:00"
    if let Some(t_pos) = s.find('T') {
        let time_part = &s[t_pos + 1..];
        if time_part.len() >= 5 {
            return time_part[..5].to_string();
        }
    }
    s.to_string()
}
