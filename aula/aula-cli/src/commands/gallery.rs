//! Gallery/media subcommands: list albums, show album contents.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::models::gallery::{GalleryViewFilter, GetMediaInAlbumFilter};
use aula_api::services::gallery;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// Browse and download gallery media (photos, videos).
#[derive(Debug, Subcommand)]
pub enum GalleryCommand {
    /// List gallery albums.
    List {
        /// Filter by institution code.
        #[arg(long)]
        institution: Option<String>,
        /// Maximum number of albums to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
    },
    /// Show album contents (media items).
    Show {
        /// Album ID.
        album_id: i64,
        /// Maximum number of media items to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
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

pub async fn handle(cmd: &GalleryCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        GalleryCommand::List { institution, limit } => {
            handle_list(institution.as_deref(), *limit, json, env_override).await;
        }
        GalleryCommand::Show { album_id, limit } => {
            handle_show(*album_id, *limit, json, env_override).await;
        }
    }
}

// ---------------------------------------------------------------------------
// List albums
// ---------------------------------------------------------------------------

async fn handle_list(
    institution: Option<&str>,
    limit: u32,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let filter = GalleryViewFilter {
        selected_institution_code_for_filter: institution.map(|s| s.to_string()),
        album_id: None,
        user_specific_album: None,
        limit: Some(limit as i32),
        index: Some(0),
        sort_on: Some("createdAt".to_string()),
        order_direction: Some("desc".to_string()),
        filter_by: None,
    };

    match gallery::get_albums(&mut session, &filter).await {
        Ok(albums) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&albums).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if albums.is_empty() {
                println!("No albums found.");
            } else {
                println!(
                    "{:<8} {:<30} {:<20} {:<8} {:<16}",
                    "ID", "TITLE", "CREATOR", "ITEMS", "CREATED"
                );
                println!("{}", "-".repeat(85));
                for album in &albums {
                    let id = album.id.map(|id| id.to_string()).unwrap_or_default();
                    let title = album
                        .title
                        .as_deref()
                        .or(album.name.as_deref())
                        .unwrap_or("(untitled)");
                    let creator = album
                        .creator
                        .as_ref()
                        .and_then(|c| c.name.as_deref())
                        .unwrap_or("(unknown)");
                    let size = album
                        .total_size
                        .or(album.size)
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let date = album.creation_date.as_deref().unwrap_or("");

                    println!(
                        "{:<8} {:<30} {:<20} {:<8} {:<16}",
                        id,
                        truncate(title, 30),
                        truncate(creator, 20),
                        size,
                        truncate_date(date)
                    );
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
// Show album contents
// ---------------------------------------------------------------------------

async fn handle_show(album_id: i64, limit: u32, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let filter = GetMediaInAlbumFilter {
        album_id: Some(album_id),
        user_specific_album: None,
        limit: Some(limit as i32),
        index: Some(0),
        sort_on: None,
        order_direction: None,
        filter_by: None,
        is_selection_mode: false,
        selected_institution_code: None,
    };

    match gallery::get_medias_in_album(&mut session, &filter).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                // Print album header if available
                if let Some(ref album) = result.album {
                    let title = album
                        .title
                        .as_deref()
                        .or(album.name.as_deref())
                        .unwrap_or("(untitled)");
                    println!("Album: {title}");
                    if let Some(ref desc) = album.description {
                        if !desc.is_empty() {
                            println!("  {desc}");
                        }
                    }
                }
                if let Some(count) = result.media_count {
                    println!("  Total media: {count}");
                }
                println!("{}", "=".repeat(72));

                match result.results.as_ref() {
                    Some(media) if !media.is_empty() => {
                        println!("{:<30} {:<10} {:<40}", "TITLE", "TYPE", "URL");
                        println!("{}", "-".repeat(80));
                        for m in media {
                            let title = m.title.as_deref().unwrap_or("(untitled)");
                            let media_type = m.media_type.as_deref().unwrap_or("");
                            let url = m
                                .thumbnail_url
                                .as_deref()
                                .or(m.file.as_ref().and_then(|f| f.url.as_deref()))
                                .unwrap_or("");
                            println!(
                                "{:<30} {:<10} {:<40}",
                                truncate(title, 30),
                                truncate(media_type, 10),
                                truncate(url, 40)
                            );
                        }
                    }
                    _ => {
                        println!("No media items found.");
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

fn truncate_date(s: &str) -> String {
    if s.len() >= 16 {
        s[..16].replace('T', " ")
    } else {
        s.to_string()
    }
}
