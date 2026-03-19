//! Document subcommands: list secure documents, show details.

use clap::Subcommand;

use aula_api::models::documents::GetSecureDocumentsArguments;
use aula_api::services::documents;

use crate::output::{
    bold, dim, format_datetime, print_json, strip_html_tags, truncate, Column, Table,
};
use crate::session_util::build_session;

/// Browse and download shared documents.
#[derive(Debug, Subcommand)]
pub enum DocumentsCommand {
    /// List secure documents.
    List {
        /// Filter by institution profile IDs (comma-separated).
        #[arg(long, value_delimiter = ',')]
        profiles: Vec<i64>,
        /// Maximum number of documents to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Show only unread documents.
        #[arg(long)]
        unread: bool,
    },
    /// Show document details by ID.
    Show {
        /// Document ID.
        document_id: i64,
        /// Document type: 'internal' (default) or 'external'.
        #[arg(long, default_value = "internal")]
        doc_type: String,
    },
}

// ---------------------------------------------------------------------------
// Top-level handler
// ---------------------------------------------------------------------------

pub async fn handle(cmd: &DocumentsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        DocumentsCommand::List {
            profiles,
            limit,
            unread,
        } => {
            handle_list(profiles, *limit, *unread, json, env_override).await;
        }
        DocumentsCommand::Show {
            document_id,
            doc_type,
        } => {
            handle_show(*document_id, doc_type, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// List documents
// ---------------------------------------------------------------------------

async fn handle_list(
    profiles: &[i64],
    limit: u32,
    unread: bool,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    // Auto-populate institution profile IDs from session when not specified.
    let profile_ids = if profiles.is_empty() {
        if let Err(e) = session.ensure_context_initialized().await {
            eprintln!("error: failed to initialize session: {e}");
            std::process::exit(1);
        }
        let ids = session.children_inst_profile_ids();
        if ids.is_empty() {
            None
        } else {
            Some(ids)
        }
    } else {
        Some(profiles.to_vec())
    };

    let args = GetSecureDocumentsArguments {
        filter_institution_profile_ids: profile_ids,
        filter_regarding_group_ids: None,
        filter_unread: if unread { Some(true) } else { None },
        filter_locked: None,
        filter_journaling_status: None,
        filter_editable: false,
        document_type: None,
        sortings: None,
        index: Some(0),
        limit: Some(limit as i32),
        filter_regarding_student_ids: None,
        filter_document_categories: None,
    };

    match documents::get_secure_documents(&mut session, &args).await {
        Ok(result) => {
            if json {
                print_json(&result);
            } else {
                let docs = result.documents.as_ref();
                let total = result.total_count.unwrap_or(0);

                match docs {
                    Some(docs) if !docs.is_empty() => {
                        let table = Table::new(vec![
                            Column::new("ID", 8),
                            Column::new("TITLE", 30),
                            Column::new("CATEGORY", 15),
                            Column::new("LOCKED", 10),
                            Column::new("UPDATED", 16),
                        ]);
                        table.print_header();
                        for doc in docs {
                            let id = doc.id.map(|id| id.to_string()).unwrap_or_default();
                            let title = doc.title.as_deref().unwrap_or("(untitled)");
                            let category = doc.category.as_deref().unwrap_or("");
                            let locked = if doc.is_locked { "yes" } else { "no" };
                            let updated = doc.updated_at.as_deref().unwrap_or("");

                            table.print_row(&[
                                &id,
                                &truncate(title, 30),
                                &truncate(category, 15),
                                locked,
                                &format_datetime(updated),
                            ]);
                        }
                        println!("\nTotal: {total} document(s)");
                    }
                    _ => {
                        println!("No documents found.");
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
// Show document
// ---------------------------------------------------------------------------

async fn handle_show(document_id: i64, doc_type: &str, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    if doc_type == "external" {
        match documents::get_external_document_details(&mut session, document_id).await {
            Ok(doc) => {
                if json {
                    print_json(&doc);
                } else {
                    println!("{}", bold(&format!("External Document #{document_id}")));
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&doc)
                            .unwrap_or_else(|_| "(display error)".into())
                    );
                }
            }
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        match documents::get_internal_document_details(&mut session, document_id).await {
            Ok(doc) => {
                if json {
                    print_json(&doc);
                } else {
                    let title = doc.title.as_deref().unwrap_or("(untitled)");
                    println!("{}", bold(&format!("Document: {title}")));
                    if let Some(ref category) = doc.category {
                        println!("  Category: {category}");
                    }
                    if let Some(ref creator) = doc.creator {
                        if let Some(ref name) = creator.name {
                            println!("  Creator: {name}");
                        }
                    }
                    if let Some(ref created) = doc.created_at {
                        println!("  Created: {}", format_datetime(created));
                    }
                    if let Some(ref updated) = doc.updated_at {
                        println!("  Updated: {}", format_datetime(updated));
                    }
                    if let Some(version) = doc.version {
                        println!("  Version: {version}");
                    }
                    println!("{}", dim(&"=".repeat(72)));
                    if let Some(ref content) = doc.content {
                        if let Some(ref html) = content.html {
                            let plain = strip_html_tags(html);
                            println!("{plain}");
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
}
