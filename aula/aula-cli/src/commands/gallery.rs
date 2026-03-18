//! Gallery/media subcommands: list albums, show album contents.

use clap::Subcommand;

use aula_api::models::gallery::{GalleryViewFilter, GetMediaInAlbumFilter};
use aula_api::services::gallery;

use crate::output::{bold, dim, format_datetime, print_json, Column, Table};
use crate::session_util::build_session;

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
                print_json(&albums);
            } else if albums.is_empty() {
                println!("No albums found.");
            } else {
                let table = Table::new(vec![
                    Column::new("ID", 8),
                    Column::new("TITLE", 30),
                    Column::new("CREATOR", 20),
                    Column::new("ITEMS", 8),
                    Column::new("CREATED", 16),
                ]);
                table.print_header();
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

                    table.print_row(&[&id, title, creator, &size, &format_datetime(date)]);
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
                print_json(&result);
            } else {
                if let Some(ref album) = result.album {
                    let title = album
                        .title
                        .as_deref()
                        .or(album.name.as_deref())
                        .unwrap_or("(untitled)");
                    println!("{}", bold(&format!("Album: {title}")));
                    if let Some(ref desc) = album.description {
                        if !desc.is_empty() {
                            println!("  {desc}");
                        }
                    }
                }
                if let Some(count) = result.media_count {
                    println!("  Total media: {count}");
                }
                println!("{}", dim(&"=".repeat(72)));

                match result.results.as_ref() {
                    Some(media) if !media.is_empty() => {
                        let table = Table::new(vec![
                            Column::new("TITLE", 30),
                            Column::new("TYPE", 10),
                            Column::new("URL", 40),
                        ]);
                        table.print_header();
                        for m in media {
                            let title = m.title.as_deref().unwrap_or("(untitled)");
                            let media_type = m.media_type.as_deref().unwrap_or("");
                            let url = m
                                .thumbnail_url
                                .as_deref()
                                .or(m.file.as_ref().and_then(|f| f.url.as_deref()))
                                .unwrap_or("");
                            table.print_row(&[title, media_type, url]);
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
