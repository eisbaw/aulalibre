//! Search subcommands: global search across Aula content.

use clap::Args;

use aula_api::models::search::GlobalSearchParameters;
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
                print_json(&result);
            } else {
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
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
