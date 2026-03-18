//! Notification subcommands: list, delete.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::services::notifications;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage notifications.
#[derive(Debug, Subcommand)]
pub enum NotificationsCommand {
    /// List recent notifications.
    List {
        /// Maximum number of notifications to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
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
// Session helper
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

pub async fn handle(cmd: &NotificationsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        NotificationsCommand::List { limit } => {
            handle_list(*limit, json, env_override).await;
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

    match notifications::get_notifications(&mut session).await {
        Ok(items) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&items).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if items.is_empty() {
                println!("No notifications.");
            } else {
                println!("{:<20} {:<15} {:<40}", "TYPE", "AREA", "TITLE");
                println!("{}", "-".repeat(78));
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

                    println!(
                        "{:<20} {:<15} {:<40}",
                        truncate(&event_type, 20),
                        truncate(&area, 15),
                        truncate(title, 40)
                    );
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
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
