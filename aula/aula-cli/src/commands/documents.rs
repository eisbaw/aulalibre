//! Document subcommands: list secure documents, show details.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::models::documents::GetSecureDocumentsArguments;
use aula_api::services::documents;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

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

    let args = GetSecureDocumentsArguments {
        filter_institution_profile_ids: if profiles.is_empty() {
            None
        } else {
            Some(profiles.to_vec())
        },
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
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                let docs = result.documents.as_ref();
                let total = result.total_count.unwrap_or(0);

                match docs {
                    Some(docs) if !docs.is_empty() => {
                        println!(
                            "{:<8} {:<30} {:<15} {:<10} {:<16}",
                            "ID", "TITLE", "CATEGORY", "LOCKED", "UPDATED"
                        );
                        println!("{}", "-".repeat(82));
                        for doc in docs {
                            let id = doc.id.map(|id| id.to_string()).unwrap_or_default();
                            let title = doc.title.as_deref().unwrap_or("(untitled)");
                            let category = doc.category.as_deref().unwrap_or("");
                            let locked = if doc.is_locked { "yes" } else { "no" };
                            let updated = doc.updated_at.as_deref().unwrap_or("");

                            println!(
                                "{:<8} {:<30} {:<15} {:<10} {:<16}",
                                id,
                                truncate(title, 30),
                                truncate(category, 15),
                                locked,
                                truncate_date(updated)
                            );
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
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&doc).unwrap_or_else(|e| format!(
                            "{{\"error\": \"serialization failed: {e}\"}}"
                        ))
                    );
                } else {
                    println!("External Document #{document_id}");
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
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&doc).unwrap_or_else(|e| format!(
                            "{{\"error\": \"serialization failed: {e}\"}}"
                        ))
                    );
                } else {
                    // Display structured output for internal documents
                    let title = doc.title.as_deref().unwrap_or("(untitled)");
                    println!("Document: {title}");
                    if let Some(ref category) = doc.category {
                        println!("  Category: {category}");
                    }
                    if let Some(ref creator) = doc.creator {
                        if let Some(ref name) = creator.name {
                            println!("  Creator: {name}");
                        }
                    }
                    if let Some(ref created) = doc.created_at {
                        println!("  Created: {created}");
                    }
                    if let Some(ref updated) = doc.updated_at {
                        println!("  Updated: {updated}");
                    }
                    if let Some(version) = doc.version {
                        println!("  Version: {version}");
                    }
                    println!("{}", "=".repeat(72));
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

fn truncate_date(s: &str) -> String {
    if s.len() >= 16 {
        s[..16].replace('T', " ")
    } else {
        s.to_string()
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}
