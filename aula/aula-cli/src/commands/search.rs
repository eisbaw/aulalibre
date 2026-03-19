//! Search subcommands: global search across Aula content.

use clap::Args;

use aula_api::models::search::{
    GlobalSearchParameters, SearchForProfilesAndGroupsParameters, SearchResponse,
};
use aula_api::services::search;

use crate::output::{bold, print_json, truncate, Column, Table};
use crate::session_util::build_session;

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
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &SearchCommand, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    // Try search.findGeneric first (the full global search endpoint).
    // If it fails (HTTP 500 "intern fejl" is a known server-side issue),
    // fall back to search.findProfiles which returns a compatible response.
    let result = match try_global_search(&mut session, cmd).await {
        Ok(result) => result,
        Err(global_err) => {
            // Fallback: use search.findProfiles which is more reliable.
            match try_profile_search(&mut session, cmd).await {
                Ok(result) => result,
                Err(fallback_err) => {
                    // Both failed; report the original error.
                    eprintln!("error: search.findGeneric failed: {global_err}");
                    eprintln!("error: search.findProfiles fallback also failed: {fallback_err}");
                    std::process::exit(1);
                }
            }
        }
    };

    if json {
        print_json(&result);
    } else {
        display_results(cmd, &result);
    }
}

async fn try_global_search(
    session: &mut aula_api::session::Session,
    cmd: &SearchCommand,
) -> Result<SearchResponse, aula_api::error::AulaError> {
    let params = GlobalSearchParameters {
        text: Some(cmd.query.clone()),
        page_limit: Some(cmd.limit as i32),
        page_number: cmd.page,
        group_id: None,
        doc_type_count: cmd.counts,
        doc_type: None,
        group_types: None,
    };
    search::global_search(session, &params).await
}

async fn try_profile_search(
    session: &mut aula_api::session::Session,
    cmd: &SearchCommand,
) -> Result<SearchResponse, aula_api::error::AulaError> {
    let params = SearchForProfilesAndGroupsParameters {
        text: Some(cmd.query.clone()),
        only_profiles: false,
        typeahead: false,
        limit: Some(cmd.limit as i32),
        portal_roles: None,
    };
    search::search_for_profiles(session, &params).await
}

fn display_results(cmd: &SearchCommand, result: &SearchResponse) {
    let total = result.total_size.unwrap_or(0);
    println!(
        "{}",
        bold(&format!(
            "Search results for '{}' ({total} total):",
            cmd.query
        ))
    );
    println!();

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
            let table = Table::new(vec![
                Column::new("TYPE", 10),
                Column::new("NAME", 30),
                Column::new("DESCRIPTION", 30),
            ]);
            table.print_header();
            for item in items {
                let doc_type = item.doc_type.as_deref().unwrap_or("");
                let name = item.name.as_deref().unwrap_or("(unnamed)");
                let desc = item.description.as_deref().unwrap_or("");
                table.print_row(&[
                    &truncate(doc_type, 10),
                    &truncate(name, 30),
                    &truncate(desc, 30),
                ]);
            }
        }
        _ => {
            println!("No results found.");
        }
    }
}
