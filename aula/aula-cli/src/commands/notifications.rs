//! Notification subcommands: list, delete.

use clap::Subcommand;

use aula_api::services::notifications;

use crate::output::{print_json, truncate, Column, Table};
use crate::session_util::build_session;

/// View and manage notifications.
#[derive(Debug, Subcommand)]
pub enum NotificationsCommand {
    /// List recent notifications.
    List {
        /// Maximum number of notifications to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Show all notifications (ignore limit).
        #[arg(long)]
        all: bool,
    },
    /// Delete all notifications.
    DeleteAll,
    /// Delete notifications for a specific child.
    DeleteChild {
        /// Child institution profile ID.
        child_id: i64,
    },
}

// ---------------------------------------------------------------------------
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &NotificationsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        NotificationsCommand::List { limit, all } => {
            handle_list(if *all { u32::MAX } else { *limit }, json, env_override).await;
        }
        NotificationsCommand::DeleteAll => {
            handle_delete_all(json, env_override).await;
        }
        NotificationsCommand::DeleteChild { child_id } => {
            handle_delete_child(*child_id, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// List
// ---------------------------------------------------------------------------

async fn handle_list(limit: u32, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    // Initialize context to get children IDs and institution codes.
    if let Err(e) = session.ensure_context_initialized().await {
        eprintln!("error: failed to initialize session: {e}");
        std::process::exit(1);
    }
    let children_ids = session.children_inst_profile_ids();
    let institution_codes = session.children_institution_codes();

    match notifications::get_notifications(&mut session, &children_ids, &institution_codes).await {
        Ok(items) => {
            if json {
                print_json(&items);
            } else if items.is_empty() {
                println!("No notifications.");
            } else {
                let table = Table::new(vec![
                    Column::new("TYPE", 20),
                    Column::new("AREA", 15),
                    Column::new("TITLE", 40),
                ]);
                table.print_header();
                for (i, item) in items.iter().enumerate() {
                    if i >= limit as usize {
                        break;
                    }
                    let event_type = item
                        .notification_event_type
                        .as_ref()
                        .map(|t| format!("{t:?}"))
                        .unwrap_or_default();
                    let area = item
                        .notification_area
                        .as_ref()
                        .map(|a| format!("{a:?}"))
                        .unwrap_or_default();
                    let title = item.title.as_deref().unwrap_or("(no title)");

                    table.print_row(&[
                        &truncate(&event_type, 20),
                        &truncate(&area, 15),
                        &truncate(title, 40),
                    ]);
                }

                if items.len() > limit as usize {
                    eprintln!("\n(showing {limit} of {} notifications)", items.len());
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
// Delete all
// ---------------------------------------------------------------------------

async fn handle_delete_all(json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match notifications::delete_notifications(&mut session).await {
        Ok(result) => {
            if json {
                print_json(&result);
            } else {
                println!("All notifications deleted.");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Delete for child
// ---------------------------------------------------------------------------

async fn handle_delete_child(child_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match notifications::delete_notification_for_child(&mut session, child_id).await {
        Ok(result) => {
            if json {
                print_json(&result);
            } else {
                println!("Notifications for child {child_id} deleted.");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
