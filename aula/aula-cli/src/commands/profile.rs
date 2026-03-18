//! Profile subcommands: show current user, master data.

use clap::Subcommand;

use aula_api::services::profiles;

use crate::output::{bold, dim, print_json};
use crate::session_util::build_session;

/// View and manage user profiles.
#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    /// Show current user's profile(s) from login data.
    Me,
    /// Show profile master data (contact details).
    MasterData,
}

// ---------------------------------------------------------------------------
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &ProfileCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        ProfileCommand::Me => handle_me(json, env_override).await,
        ProfileCommand::MasterData => handle_master_data(json, env_override).await,
    }
}

// ---------------------------------------------------------------------------
// Me (profiles by login)
// ---------------------------------------------------------------------------

async fn handle_me(json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match profiles::get_profiles_by_login(&mut session).await {
        Ok(profile_list) => {
            if json {
                print_json(&profile_list);
            } else if profile_list.is_empty() {
                println!("No profiles found.");
            } else {
                for (i, profile) in profile_list.iter().enumerate() {
                    if i > 0 {
                        println!("{}", dim(&"-".repeat(50)));
                    }

                    let first = profile.first_name.as_deref().unwrap_or("");
                    let last = profile.last_name.as_deref().unwrap_or("");
                    let role = profile.portal_role.as_deref().unwrap_or("(unknown)");
                    let user_id = profile.user_id.as_deref().unwrap_or("");

                    println!("{}", bold(&format!("Profile #{}", i + 1)));
                    println!("  Name: {first} {last}");
                    println!("  Role: {role}");
                    if !user_id.is_empty() {
                        println!("  User ID: {user_id}");
                    }
                    if let Some(ref email) = profile.external_email {
                        println!("  Email: {email}");
                    }
                    if let Some(ref phone) = profile.phonenumber {
                        println!("  Phone: {phone}");
                    }
                    if let Some(ref mobile) = profile.mobile_phonenumber {
                        println!("  Mobile: {mobile}");
                    }

                    if let Some(ref inst) = profile.institution_profile {
                        println!("  Institution profile ID: {}", inst.institution_profile_id);
                        if let Some(ref institution) = inst.institution {
                            if let Some(ref code) = institution.institution_code {
                                println!("  Institution code: {code}");
                            }
                            if let Some(ref name) = institution.institution_name {
                                println!("  Institution: {name}");
                            }
                        }
                    }

                    if let Some(ref groups) = profile.groups {
                        if !groups.is_empty() {
                            let group_names: Vec<&str> =
                                groups.iter().filter_map(|g| g.name.as_deref()).collect();
                            println!("  Groups ({}): {}", groups.len(), group_names.join(", "));
                        }
                    }
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
// Master data
// ---------------------------------------------------------------------------

async fn handle_master_data(json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match profiles::get_profile_master_data(&mut session).await {
        Ok(profile) => {
            if json {
                print_json(&profile);
            } else {
                let first = profile.first_name.as_deref().unwrap_or("");
                let last = profile.last_name.as_deref().unwrap_or("");
                let role = profile.portal_role.as_deref().unwrap_or("(unknown)");

                println!("{}", bold("Master Data"));
                println!("  Name: {first} {last}");
                println!("  Role: {role}");
                if let Some(ref email) = profile.external_email {
                    println!("  Email: {email}");
                }
                if let Some(ref phone) = profile.phonenumber {
                    println!("  Phone: {phone}");
                }
                if let Some(ref mobile) = profile.mobile_phonenumber {
                    println!("  Mobile: {mobile}");
                }
                if let Some(ref work) = profile.work_phonenumber {
                    println!("  Work phone: {work}");
                }
                if let Some(ref home) = profile.home_phonenumber {
                    println!("  Home phone: {home}");
                }
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
