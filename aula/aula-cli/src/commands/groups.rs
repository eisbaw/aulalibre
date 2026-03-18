//! Groups subcommands: list (by context), show, members.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::services::groups;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage groups.
#[derive(Debug, Subcommand)]
pub enum GroupsCommand {
    /// List groups for an institution profile (context).
    List {
        /// Institution profile ID to list groups for.
        #[arg(long)]
        profile: i64,
    },
    /// Show group details.
    Show {
        /// Group ID.
        group_id: i64,
    },
    /// List members of a group.
    Members {
        /// Group ID.
        group_id: i64,
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

pub async fn handle(cmd: &GroupsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        GroupsCommand::List { profile } => handle_list(*profile, json, env_override).await,
        GroupsCommand::Show { group_id } => handle_show(*group_id, json, env_override).await,
        GroupsCommand::Members { group_id } => {
            handle_members(*group_id, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// List groups by context
// ---------------------------------------------------------------------------

async fn handle_list(profile_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match groups::get_group_by_context(&mut session, profile_id).await {
        Ok(group_list) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&group_list).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if group_list.is_empty() {
                println!("No groups found.");
            } else {
                println!("{:<8} {:<30} {:<10}", "ID", "NAME", "DEFAULT");
                println!("{}", "-".repeat(50));
                for g in &group_list {
                    let id = g.id.map(|id| id.to_string()).unwrap_or_default();
                    let name = g.name.as_deref().unwrap_or("(unnamed)");
                    let default = if g.show_as_default { "yes" } else { "no" };
                    println!("{:<8} {:<30} {:<10}", id, truncate(name, 30), default);
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
// Show group
// ---------------------------------------------------------------------------

async fn handle_show(group_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match groups::get_group(&mut session, group_id).await {
        Ok(group) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&group).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                let name = group.name.as_deref().unwrap_or("(unnamed)");
                println!("Group: {name}");
                if let Some(ref desc) = group.description {
                    if !desc.is_empty() {
                        println!("  Description: {desc}");
                    }
                }
                if let Some(ref group_type) = group.group_type {
                    println!("  Type: {group_type:?}");
                }
                if let Some(ref status) = group.status {
                    println!("  Status: {status}");
                }
                if let Some(ref access) = group.access {
                    println!("  Access: {access}");
                }
                if let Some(ref role) = group.role {
                    println!("  Your role: {role:?}");
                }
                if let Some(ref code) = group.institution_code {
                    println!("  Institution: {code}");
                }
                if group.dashboard_enabled {
                    println!("  Dashboard: enabled");
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
// Members
// ---------------------------------------------------------------------------

async fn handle_members(group_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match groups::get_memberships_light(&mut session, group_id).await {
        Ok(members) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&members).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if members.is_empty() {
                println!("No members found.");
            } else {
                println!("{:<8} {:<15} {:<15}", "ID", "ROLE", "INST. ROLE");
                println!("{}", "-".repeat(40));
                for m in &members {
                    let id = m.id.map(|id| id.to_string()).unwrap_or_default();
                    let role = m
                        .group_role
                        .as_ref()
                        .map(|r| format!("{r:?}"))
                        .unwrap_or_default();
                    let inst_role = m
                        .institution_role
                        .as_ref()
                        .map(|r| format!("{r:?}"))
                        .unwrap_or_default();
                    println!("{:<8} {:<15} {:<15}", id, role, inst_role);
                }
                println!("\n{} member(s)", members.len());
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
