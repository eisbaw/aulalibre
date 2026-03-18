//! Posts/feed subcommands: list, show, create.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::models::posts::{CreatePostApiParameter, GetPostApiParameters, PostApiDto};
use aula_api::services::posts;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// View and manage posts in the institution feed.
#[derive(Debug, Subcommand)]
pub enum PostsCommand {
    /// List recent posts.
    List {
        /// Maximum number of posts to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Filter by group ID.
        #[arg(long)]
        group: Option<i64>,
        /// Show only important posts.
        #[arg(long)]
        important: bool,
        /// Show only unread posts.
        #[arg(long)]
        unread: bool,
        /// Show only bookmarked posts.
        #[arg(long)]
        bookmarked: bool,
    },
    /// Show a single post by ID.
    Show {
        /// Post ID.
        post_id: i64,
    },
    /// Create a new post.
    Create {
        /// Post title.
        #[arg(short, long)]
        title: String,
        /// Post body (HTML or plain text).
        #[arg(short, long)]
        body: String,
        /// Institution code.
        #[arg(long)]
        institution_code: String,
        /// Creator institution profile ID.
        #[arg(long)]
        profile: i64,
        /// Allow comments on the post.
        #[arg(long, default_value = "true")]
        allow_comments: bool,
        /// Mark post as important.
        #[arg(long)]
        important: bool,
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

pub async fn handle(cmd: &PostsCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        PostsCommand::List {
            limit,
            group,
            important,
            unread,
            bookmarked,
        } => {
            handle_list(
                *limit,
                *group,
                *important,
                *unread,
                *bookmarked,
                json,
                env_override,
            )
            .await;
        }
        PostsCommand::Show { post_id } => {
            handle_show(*post_id, json, env_override).await;
        }
        PostsCommand::Create {
            title,
            body,
            institution_code,
            profile,
            allow_comments,
            important,
        } => {
            handle_create(
                title,
                body,
                institution_code,
                *profile,
                *allow_comments,
                *important,
                json,
                env_override,
            )
            .await;
        }
    }
}

// ---------------------------------------------------------------------------
// List posts
// ---------------------------------------------------------------------------

async fn handle_list(
    limit: u32,
    group: Option<i64>,
    important: bool,
    unread: bool,
    bookmarked: bool,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let params = GetPostApiParameters {
        group_id: group,
        is_important: if important { Some(true) } else { None },
        creator_portal_role: None,
        institution_profile_ids: None,
        related_institutions: None,
        own_post: false,
        is_unread: unread,
        is_bookmarked: bookmarked,
        limit: Some(limit as i32),
        index: Some(0),
    };

    match posts::get_posts(&mut session, &params).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_post_list(&result, limit);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_post_list(result: &aula_api::models::posts::GetPostApiResult, limit: u32) {
    let posts = match result.posts.as_ref() {
        Some(p) => p,
        None => {
            println!("No posts found.");
            return;
        }
    };

    if posts.is_empty() {
        println!("No posts found.");
        return;
    }

    println!(
        "{:<8} {:<30} {:<20} {:<20}",
        "ID", "TITLE", "AUTHOR", "DATE"
    );
    println!("{}", "-".repeat(80));

    for (i, post) in posts.iter().enumerate() {
        if i >= limit as usize {
            break;
        }
        let id = post.id.map(|id| id.to_string()).unwrap_or_default();
        let title = post.title.as_deref().unwrap_or("(no title)");
        let author = post
            .owner_profile
            .as_ref()
            .and_then(|p| p.full_name.as_deref())
            .unwrap_or("(unknown)");
        let date = post.time_stamp.as_deref().unwrap_or("");
        let date_display = truncate_date(date);

        let flags = format!(
            "{}{}",
            if post.is_important { "!" } else { "" },
            if post.is_bookmarked { "*" } else { "" }
        );

        println!(
            "{:<8} {:<30} {:<20} {:<20}",
            format!("{id}{flags}"),
            truncate(title, 30),
            truncate(author, 20),
            date_display
        );
    }

    if result.has_more_posts {
        eprintln!("\n(more posts available)");
    }
}

// ---------------------------------------------------------------------------
// Show post
// ---------------------------------------------------------------------------

async fn handle_show(post_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    match posts::get_post_by_id(&mut session, post_id).await {
        Ok(post) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&post).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_post_detail(&post);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_post_detail(post: &PostApiDto) {
    let title = post.title.as_deref().unwrap_or("(no title)");
    let author = post
        .owner_profile
        .as_ref()
        .and_then(|p| p.full_name.as_deref())
        .unwrap_or("(unknown)");
    let date = post.time_stamp.as_deref().unwrap_or("");

    println!("Post: {title}");
    println!("  Author: {author}");
    println!("  Date: {date}");

    if post.is_important {
        print!("  [IMPORTANT]");
    }
    if post.is_bookmarked {
        print!("  [BOOKMARKED]");
    }
    println!();

    if let Some(ref groups) = post.shared_with_groups {
        if !groups.is_empty() {
            let names: Vec<&str> = groups.iter().filter_map(|g| g.name.as_deref()).collect();
            println!("  Groups: {}", names.join(", "));
        }
    }

    println!("{}", "=".repeat(72));

    if let Some(ref content) = post.content {
        if let Some(ref html) = content.html {
            let plain = strip_html_tags(html);
            println!("{plain}");
        }
    }

    if let Some(ref attachments) = post.attachments {
        if !attachments.is_empty() {
            println!("\n  [{} attachment(s)]", attachments.len());
        }
    }

    if let Some(count) = post.comment_count {
        if count > 0 {
            println!("\n  [{count} comment(s)]");
        }
    }
}

// ---------------------------------------------------------------------------
// Create post
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
async fn handle_create(
    title: &str,
    body: &str,
    institution_code: &str,
    profile: i64,
    allow_comments: bool,
    important: bool,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let params = CreatePostApiParameter {
        id: None,
        title: Some(title.to_string()),
        content: Some(body.to_string()),
        institution_code: Some(institution_code.to_string()),
        creator_institution_profile_id: Some(profile),
        allow_comments,
        is_important: important,
        important_from: None,
        important_to: None,
        shared_with_groups: None,
        attachment_ids: None,
        publish_at: None,
        expire_at: None,
    };

    match posts::create_post(&mut session, &params).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Post created.");
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
