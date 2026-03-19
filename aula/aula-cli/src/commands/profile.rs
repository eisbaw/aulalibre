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
        Ok(resp) => {
            if json {
                print_json(&resp);
            } else if resp.profiles.is_empty() {
                println!("No profiles found.");
            } else {
                for (i, profile) in resp.profiles.iter().enumerate() {
                    if i > 0 {
                        println!("{}", dim(&"-".repeat(50)));
                    }

                    let name = profile.display_name.as_deref().unwrap_or("(unknown)");
                    let role = profile.portal_role.as_deref().unwrap_or("(unknown)");

                    println!("{}", bold(&format!("Profile #{}", i + 1)));
                    println!("  Name: {name}");
                    println!("  Role: {role}");
                    if let Some(pid) = profile.profile_id {
                        println!("  Profile ID: {pid}");
                    }

                    // Institution profiles
                    if let Some(ref ips) = profile.institution_profiles {
                        for ip in ips {
                            println!();
                            println!(
                                "  {}",
                                bold(&format!(
                                    "Institution: {}",
                                    ip.institution_name.as_deref().unwrap_or("(unknown)")
                                ))
                            );
                            println!("    Institution profile ID: {}", ip.id);
                            if let Some(ref code) = ip.institution_code {
                                println!("    Institution code: {code}");
                            }
                            if let Some(ref mun) = ip.municipality_name {
                                println!("    Municipality: {mun}");
                            }
                            if let Some(ref email) = ip.email {
                                println!("    Email: {email}");
                            }
                            if let Some(ref phone) = ip.mobile_phone_number {
                                println!("    Mobile: {phone}");
                            }
                            if let Some(ref addr) = ip.address {
                                let street = addr.street.as_deref().unwrap_or("");
                                let postal = addr.postal_code.as_deref().unwrap_or("");
                                let district = addr.postal_district.as_deref().unwrap_or("");
                                if !street.is_empty() {
                                    println!("    Address: {street}, {postal} {district}");
                                }
                            }
                        }
                    }

                    // Children
                    let kids = profile.children_names();
                    if !kids.is_empty() {
                        println!();
                        println!("  Children ({}): {}", kids.len(), kids.join(", "));
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
