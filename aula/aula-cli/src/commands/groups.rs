//! Groups subcommands: list (by context), show, members.

use clap::Subcommand;

use aula_api::services::groups;

use crate::output::{bold, print_json, truncate, Column, Table};
use crate::session_util::build_session;

/// View and manage groups.
#[derive(Debug, Subcommand)]
pub enum GroupsCommand {
    /// List groups for an institution profile (context).
    List {
        /// Institution profile ID to list groups for.
        #[arg(long)]
        inst_profile: i64,
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
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &GroupsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        GroupsCommand::List { inst_profile } => {
            handle_list(*inst_profile, json, env_override).await
        }
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
                print_json(&group_list);
            } else if group_list.is_empty() {
                println!("No groups found.");
            } else {
                let table = Table::new(vec![
                    Column::new("ID", 8),
                    Column::new("NAME", 30),
                    Column::new("DEFAULT", 10),
                ]);
                table.print_header();
                for g in &group_list {
                    let id = g.id.map(|id| id.to_string()).unwrap_or_default();
                    let name = g.name.as_deref().unwrap_or("(unnamed)");
                    let default = if g.show_as_default { "yes" } else { "no" };
                    table.print_row(&[&id, &truncate(name, 30), default]);
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
                print_json(&group);
            } else {
                let name = group.name.as_deref().unwrap_or("(unnamed)");
                println!("{}", bold(&format!("Group: {name}")));
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
                print_json(&members);
            } else if members.is_empty() {
                println!("No members found.");
            } else {
                let table = Table::new(vec![
                    Column::new("ID", 8),
                    Column::new("ROLE", 15),
                    Column::new("INST. ROLE", 15),
                ]);
                table.print_header();
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
                    table.print_row(&[&id, &role, &inst_role]);
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
