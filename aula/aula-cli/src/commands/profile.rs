//! Profile subcommands: show current user, master data.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::services::profiles;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage user profiles.
#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    /// Show current user's profile(s) from login data.
    Me,
    /// Show profile master data (contact details).
    MasterData,
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&profile_list).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if profile_list.is_empty() {
                println!("No profiles found.");
            } else {
                for (i, profile) in profile_list.iter().enumerate() {
                    if i > 0 {
                        println!("{}", "-".repeat(50));
                    }

                    let first = profile.first_name.as_deref().unwrap_or("");
                    let last = profile.last_name.as_deref().unwrap_or("");
                    let role = profile.portal_role.as_deref().unwrap_or("(unknown)");
                    let user_id = profile.user_id.as_deref().unwrap_or("");

                    println!("Profile #{}", i + 1);
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

                    // Institution profile info
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

                    // Groups
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&profile).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                let first = profile.first_name.as_deref().unwrap_or("");
                let last = profile.last_name.as_deref().unwrap_or("");
                let role = profile.portal_role.as_deref().unwrap_or("(unknown)");

                println!("Master Data");
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
