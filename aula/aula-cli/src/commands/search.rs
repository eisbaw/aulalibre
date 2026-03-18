//! Search subcommands: global search across Aula content.

use clap::Args;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::models::search::GlobalSearchParameters;
use aula_api::services::search;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// Search across Aula content.
#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Search query string.
    pub query: String,
    /// Maximum number of results.
    #[arg(short = 'n', long, default_value = "20")]
    pub limit: u32,
    /// Page number.
    #[arg(long)]
    pub page: Option<i32>,
    /// Include document type counts in output.
    #[arg(long)]
    pub counts: bool,
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

pub async fn handle(cmd: &SearchCommand, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let params = GlobalSearchParameters {
        text: Some(cmd.query.clone()),
        page_limit: Some(cmd.limit as i32),
        page_number: cmd.page,
        group_id: None,
        doc_type_count: cmd.counts,
        doc_type: None,
        group_types: None,
    };

    match search::global_search(&mut session, &params).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                let total = result.total_size.unwrap_or(0);
                println!("Search results for '{}' ({total} total):", cmd.query);
                println!();

                // Show doc type counts if requested
                if cmd.counts {
                    if let Some(ref counts) = result.doc_type_count {
                        if !counts.is_empty() {
                            println!("Content types:");
                            for c in counts {
                                let name = c.name.as_deref().unwrap_or("(unknown)");
                                let count = c.count.unwrap_or(0);
                                println!("  {name}: {count}");
                            }
                            println!();
                        }
                    }
                }

                match result.results.as_ref() {
                    Some(items) if !items.is_empty() => {
                        println!("{:<10} {:<30} {:<30}", "TYPE", "NAME", "DESCRIPTION");
                        println!("{}", "-".repeat(72));
                        for item in items {
                            let doc_type = item.doc_type.as_deref().unwrap_or("");
                            let name = item.name.as_deref().unwrap_or("(unnamed)");
                            let desc = item.description.as_deref().unwrap_or("");
                            println!(
                                "{:<10} {:<30} {:<30}",
                                truncate(doc_type, 10),
                                truncate(name, 30),
                                truncate(desc, 30)
                            );
                        }
                    }
                    _ => {
                        println!("No results found.");
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
// Helpers
// ---------------------------------------------------------------------------

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}
