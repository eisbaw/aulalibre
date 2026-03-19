//! Search subcommands: global search across Aula content.
//!
//! Strategy: try `search.findGeneric` first (comprehensive global search).
//! If it fails (known HTTP 500 server-side), fall back to a combined search
//! that merges results from `search.findProfiles` and `search.findGroups`.
//!
//! ## Endpoint status (as of 2026-03-19)
//!
//! | Endpoint                     | Status | Notes                              |
//! |------------------------------|--------|------------------------------------|
//! | `search.findGeneric`         | 500    | Server-side broken ("intern fejl") |
//! | `search.findProfiles`        | OK     | Returns profile results            |
//! | `search.findGroups`          | OK     | Returns group results              |
//! | `search.findProfilesAndGroups` | 400  | Needs context params we lack       |
//! | `search.findMessage`         | 400    | Needs context params we lack       |
//! | `search.findRecipients`      | 400    | Needs context params we lack       |

use clap::Args;

use aula_api::models::search::{
    GlobalSearchParameters, SearchForProfilesAndGroupsParameters, SearchGroupRequestModel,
    SearchMessageRequestModel, SearchResponse, SearchResultItem, SearchResultMessagesResponse,
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
    /// Probe all search endpoints and report which ones work.
    #[arg(long)]
    pub probe: bool,
}

// ---------------------------------------------------------------------------
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &SearchCommand, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    if cmd.probe {
        probe_endpoints(&mut session, cmd).await;
        return;
    }

    // Try search.findGeneric first (the full global search endpoint).
    // If it fails (HTTP 500 "intern fejl" is a known server-side issue),
    // fall back to combined search across multiple endpoints.
    let result = match try_global_search(&mut session, cmd).await {
        Ok(result) => {
            eprintln!("[search] findGeneric succeeded");
            result
        }
        Err(global_err) => {
            eprintln!(
                "[search] findGeneric failed ({}), falling back to combined search",
                global_err
            );
            match try_combined_search(&mut session, cmd).await {
                Ok(result) => result,
                Err(combined_err) => {
                    eprintln!("error: combined search also failed: {combined_err}");
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

// ---------------------------------------------------------------------------
// Endpoint probing (--probe flag)
// ---------------------------------------------------------------------------

/// Test each search endpoint and report results. Useful for checking whether
/// previously broken endpoints have been fixed server-side.
async fn probe_endpoints(session: &mut aula_api::session::Session, cmd: &SearchCommand) {
    println!(
        "{}",
        bold(&format!(
            "Probing search endpoints with query '{}':",
            cmd.query
        ))
    );
    println!();

    // 1. findGeneric
    print!("  search.findGeneric ... ");
    match try_global_search(session, cmd).await {
        Ok(r) => {
            let n = r.results.as_ref().map_or(0, |v| v.len());
            let total = r.total_size.unwrap_or(0);
            println!("OK ({n} results, {total} total)");
        }
        Err(e) => println!("FAIL: {e}"),
    }

    // 2. findProfiles
    print!("  search.findProfiles ... ");
    match try_profile_search(session, cmd).await {
        Ok(r) => {
            let n = r.results.as_ref().map_or(0, |v| v.len());
            let total = r.total_size.unwrap_or(0);
            println!("OK ({n} results, {total} total)");
        }
        Err(e) => println!("FAIL: {e}"),
    }

    // 3. findProfilesAndGroups
    print!("  search.findProfilesAndGroups ... ");
    match try_profiles_and_groups_search(session, cmd).await {
        Ok(r) => {
            let n = r.results.as_ref().map_or(0, |v| v.len());
            let total = r.total_size.unwrap_or(0);
            println!("OK ({n} results, {total} total)");
        }
        Err(e) => println!("FAIL: {e}"),
    }

    // 4. findMessage
    print!("  search.findMessage ... ");
    match try_message_search(session, cmd).await {
        Ok(r) => {
            let n = r.results.as_ref().map_or(0, |v| v.len());
            let total = r.total_hits.unwrap_or(0);
            println!("OK ({n} results, {total} total)");
        }
        Err(e) => println!("FAIL: {e}"),
    }

    // 5. findGroups
    print!("  search.findGroups ... ");
    match try_group_search(session, cmd).await {
        Ok(r) => {
            let n = r.len();
            println!("OK ({n} results)");
        }
        Err(e) => println!("FAIL: {e}"),
    }

    // 6. findRecipients
    print!("  search.findRecipients ... ");
    {
        use aula_api::models::search::SearchRecipientParameters;
        let params = SearchRecipientParameters {
            text: Some(cmd.query.clone()),
            from_module: None,
            doc_types: None,
            portal_roles: None,
            group_search_scope: None,
            limit: Some(cmd.limit as i32),
            scope_employees_to_institution: None,
            group_id: None,
            inst_code: None,
            institution_codes: None,
            regarding_children: None,
            mail_box_owner_type: None,
            mail_box_owner_id: None,
        };
        match search::search_for_recipients(session, &params).await {
            Ok(r) => {
                let n = r.results.as_ref().map_or(0, |v| v.len());
                let total = r.total_hits.unwrap_or(0);
                println!("OK ({n} results, {total} total)");
            }
            Err(e) => println!("FAIL: {e}"),
        }
    }

    println!();
    println!("Probe complete.");
}

// ---------------------------------------------------------------------------
// Individual endpoint callers
// ---------------------------------------------------------------------------

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

async fn try_profiles_and_groups_search(
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
    search::search_for_profiles_and_groups(session, &params).await
}

#[allow(dead_code)]
async fn try_message_search(
    session: &mut aula_api::session::Session,
    cmd: &SearchCommand,
) -> Result<SearchResultMessagesResponse, aula_api::error::AulaError> {
    let params = SearchMessageRequestModel {
        keyword: Some(cmd.query.clone()),
        thread_subject: None,
        message_content: None,
        has_attachments: None,
        from_date: None,
        to_date: None,
        thread_creators: None,
        participants: None,
        page: Some(0),
        common_inbox_id: None,
        folder_id: None,
        filter: None,
        sort_type: None,
        sort_order: None,
    };
    search::search_for_messages(session, &params).await
}

async fn try_group_search(
    session: &mut aula_api::session::Session,
    cmd: &SearchCommand,
) -> Result<Vec<SearchResultItem>, aula_api::error::AulaError> {
    let params = SearchGroupRequestModel {
        text: Some(cmd.query.clone()),
        institution_codes: None,
        limit: Some(cmd.limit as i32),
        offset: Some(0),
        from_module_value: None,
    };
    let result = search::search_groups(session, &params).await?;
    // Convert group results to SearchResultItem for unified display
    let items = result
        .results
        .unwrap_or_default()
        .into_iter()
        .map(|g| SearchResultItem {
            doc_id: g.id.map(|id| format!("g-{id}")),
            doc_type: Some("Group".to_string()),
            institution_code: g.institution_code,
            institution_name: g.institution_name,
            municipality_code: None,
            municipality_name: None,
            name: g.name,
            description: None,
        })
        .collect();
    Ok(items)
}

// ---------------------------------------------------------------------------
// Combined search (fallback when findGeneric fails)
// ---------------------------------------------------------------------------

/// Run multiple search endpoints and merge their results into a single
/// `SearchResponse`. Since `findGeneric` is broken server-side, we combine:
///
///   - `search.findProfiles` -- profiles (always works)
///   - `search.findGroups` -- groups (always works)
///
/// Note: `findProfilesAndGroups`, `findMessage`, and `findRecipients` all
/// return HTTP 400 when called without the specific context parameters that
/// the app normally provides (institution codes, module context, etc.).
/// They are excluded from the combined search for now.
async fn try_combined_search(
    session: &mut aula_api::session::Session,
    cmd: &SearchCommand,
) -> Result<SearchResponse, aula_api::error::AulaError> {
    let mut all_items: Vec<SearchResultItem> = Vec::new();
    let mut total: i32 = 0;
    let mut sources_ok = 0u32;

    // 1. Profiles (via findProfiles)
    match try_profile_search(session, cmd).await {
        Ok(r) => {
            total += r.total_size.unwrap_or(0);
            if let Some(items) = r.results {
                all_items.extend(items);
            }
            sources_ok += 1;
        }
        Err(e) => eprintln!("[search] findProfiles failed: {e}"),
    }

    // 2. Groups (via findGroups)
    match try_group_search(session, cmd).await {
        Ok(items) => {
            total += items.len() as i32;
            all_items.extend(items);
            sources_ok += 1;
        }
        Err(e) => eprintln!("[search] findGroups failed: {e}"),
    }

    if sources_ok == 0 {
        return Err(aula_api::error::AulaError::Api {
            message: "all combined search endpoints failed".to_string(),
            status: None,
        });
    }

    Ok(SearchResponse {
        total_size: Some(total),
        doc_type_count: None,
        group_type_count: None,
        results: Some(all_items),
    })
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
