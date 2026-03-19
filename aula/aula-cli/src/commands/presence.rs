//! Presence/attendance subcommands: status, schedule, report-absence, history.

use chrono::Datelike;
use clap::Subcommand;

use aula_api::models::presence::PresenceSchedulesRequest;
use aula_api::services::presence;

use crate::output::{color_presence_status, extract_time, print_json, truncate, Column, Table};
use crate::session_util::build_session;

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

    // Auto-populate children IDs from session profile data if not provided.
    let ids: Vec<i64>;
    let children = if children.is_empty() {
        // Ensure context is initialized so profile data is available.
        if let Err(e) = session.ensure_context_initialized().await {
            eprintln!("error: failed to initialize session: {e}");
            std::process::exit(1);
        }
        ids = session.children_inst_profile_ids();
        if ids.is_empty() {
            eprintln!("error: no children found in profile; specify --children explicitly");
            std::process::exit(1);
        }
        &ids
    } else {
        children
    };

    match presence::get_childrens_state(&mut session, children).await {
        Ok(states) => {
            if json {
                print_json(&states);
            } else if states.is_empty() {
                println!("No presence status found.");
            } else {
                let table = Table::new(vec![
                    Column::new("PROFILE ID", 12),
                    Column::new("STATUS", 15),
                    Column::new("NAME", 20),
                ]);
                table.print_header();
                for s in &states {
                    let status_raw = s
                        .state
                        .as_ref()
                        .map(|st| format!("{st:?}"))
                        .unwrap_or_else(|| "(unknown)".to_string());
                    let status_display = color_presence_status(&status_raw);
                    let name = s
                        .uni_student
                        .as_ref()
                        .and_then(|u| u.name.as_deref())
                        .unwrap_or("(unknown)");
                    let profile_id = s.uni_student_id.to_string();
                    table.print_colored_row(
                        &[&profile_id, &status_raw, name],
                        &[&profile_id, &status_display, name],
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

    // Auto-populate children IDs from session profile data if not provided.
    let ids: Vec<i64>;
    let children = if children.is_empty() {
        if let Err(e) = session.ensure_context_initialized().await {
            eprintln!("error: failed to initialize session: {e}");
            std::process::exit(1);
        }
        ids = session.children_inst_profile_ids();
        if ids.is_empty() {
            eprintln!("error: no children found in profile; specify --children explicitly");
            std::process::exit(1);
        }
        &ids
    } else {
        children
    };

    // Note: the --date parameter is accepted but the getDailyOverview endpoint
    // always returns today's overview. A future improvement could use a
    // different endpoint for historical dates.
    let _ = date;

    match presence::get_daily_overview(&mut session, children).await {
        Ok(regs) => {
            if json {
                print_json(&regs);
            } else if regs.is_empty() {
                println!("No presence registrations found.");
            } else {
                let table = Table::new(vec![
                    Column::new("CHILD", 20),
                    Column::new("STATUS", 15),
                    Column::new("CHECK-IN", 12),
                    Column::new("CHECK-OUT", 12),
                    Column::new("LOCATION", 15),
                ]);
                table.print_header();
                for r in &regs {
                    let status_raw = r
                        .status
                        .as_ref()
                        .map(|s| format!("{s:?}"))
                        .unwrap_or_default();
                    let status_display = color_presence_status(&status_raw);
                    let checkin = r
                        .check_in_time
                        .as_deref()
                        .map(extract_time)
                        .unwrap_or_default();
                    let checkout = r
                        .check_out_time
                        .as_deref()
                        .map(extract_time)
                        .unwrap_or_default();
                    let child_name = r
                        .institution_profile
                        .as_ref()
                        .and_then(|p| p.name.as_deref())
                        .unwrap_or("(unknown)");
                    let location = r
                        .location
                        .as_ref()
                        .and_then(|l| l.name.as_deref())
                        .unwrap_or("");
                    let location_trunc = truncate(location, 15);
                    table.print_colored_row(
                        &[
                            child_name,
                            &status_raw,
                            &checkin,
                            &checkout,
                            &location_trunc,
                        ],
                        &[
                            child_name,
                            &status_display,
                            &checkin,
                            &checkout,
                            &location_trunc,
                        ],
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

    // Auto-populate children IDs from session profile data if not provided.
    let ids: Vec<i64>;
    let children = if children.is_empty() {
        if let Err(e) = session.ensure_context_initialized().await {
            eprintln!("error: failed to initialize session: {e}");
            std::process::exit(1);
        }
        ids = session.children_inst_profile_ids();
        if ids.is_empty() {
            eprintln!("error: no children found in profile; specify --children explicitly");
            std::process::exit(1);
        }
        &ids
    } else {
        children
    };

    // Default date range to current week (Mon-Sun) like the C# app does.
    let today = chrono::Local::now().date_naive();
    let from_date = from.map(|s| s.to_string()).unwrap_or_else(|| {
        let weekday = today.weekday().num_days_from_monday();
        let monday = today - chrono::Duration::days(weekday as i64);
        monday.format("%Y-%m-%d").to_string()
    });
    let to_date = to.map(|s| s.to_string()).unwrap_or_else(|| {
        let weekday = today.weekday().num_days_from_monday();
        let sunday = today + chrono::Duration::days(6 - weekday as i64);
        sunday.format("%Y-%m-%d").to_string()
    });

    let args = PresenceSchedulesRequest {
        filter_institution_profile_ids: if children.is_empty() {
            None
        } else {
            Some(children.to_vec())
        },
        from_date: Some(from_date),
        to_date: Some(to_date),
    };

    match presence::get_presence_schedules(&mut session, &args).await {
        Ok(schedule) => {
            if json {
                print_json(&schedule);
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&schedule).unwrap_or_else(|_| "(error)".into())
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
                print_json(&result);
            } else {
                let status_name = match status {
                    0 => "NotPresent",
                    1 => "Sick",
                    2 => "ReportedAbsence",
                    3 => "Present",
                    _ => "Unknown",
                };
                println!(
                    "Status updated to {} for {} profile(s).",
                    color_presence_status(status_name),
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
