//! Messaging subcommands: list threads, read, send, reply, delete, folders, move.

use clap::Subcommand;

use aula_api::client::{AulaClient, AulaClientConfig};
use aula_api::enums::common::FilterAndSortType;
use aula_api::models::messaging::{
    DeleteThreadArguments, GetFoldersArguments, GetMessagesForThreadArguments,
    GetThreadListArguments, MessageContentRequest, MoveThreadsToFolderRequestArguments,
    RecipientApiModel, ReplyMessageArgument, SetLastMessageRequestArguments,
    StartNewThreadRequestArguments,
};
use aula_api::services::messaging;
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// Read and send messages (threads).
#[derive(Debug, Subcommand)]
pub enum MessagesCommand {
    /// List message threads (inbox view).
    List {
        /// Maximum number of threads to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Page number for pagination.
        #[arg(long)]
        page: Option<u32>,
        /// Show only unread threads.
        #[arg(long)]
        unread: bool,
        /// Show only marked (starred) threads.
        #[arg(long)]
        marked: bool,
        /// Filter by folder ID.
        #[arg(long)]
        folder: Option<i64>,
    },
    /// Show messages in a thread.
    #[command(alias = "show")]
    Read {
        /// Thread ID.
        thread_id: i64,
        /// Page number for pagination.
        #[arg(long)]
        page: Option<i32>,
    },
    /// Send a new message (start a new thread).
    Send {
        /// Recipient profile IDs (comma-separated).
        #[arg(short, long, value_delimiter = ',')]
        to: Vec<i64>,
        /// Message subject.
        #[arg(short, long)]
        subject: Option<String>,
        /// Message body text.
        #[arg(short, long)]
        body: String,
    },
    /// Reply to an existing thread.
    Reply {
        /// Thread ID to reply to.
        thread_id: i64,
        /// Reply body text.
        #[arg(short, long)]
        body: String,
    },
    /// Mark a thread as read.
    MarkRead {
        /// Thread ID.
        thread_id: i64,
    },
    /// Delete a thread.
    Delete {
        /// Thread ID.
        thread_id: i64,
    },
    /// List message folders.
    Folders,
    /// Move a thread to a folder.
    Move {
        /// Thread ID to move.
        thread_id: i64,
        /// Target folder ID.
        #[arg(long)]
        folder: i64,
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

pub async fn handle(cmd: &MessagesCommand, json: bool, env_override: Option<&str>) {
    match cmd {
        MessagesCommand::List {
            limit,
            page,
            unread,
            marked,
            folder,
        } => handle_list(*limit, *page, *unread, *marked, *folder, json, env_override).await,
        MessagesCommand::Read { thread_id, page } => {
            handle_read(*thread_id, *page, json, env_override).await
        }
        MessagesCommand::Send { to, subject, body } => {
            handle_send(to, subject.as_deref(), body, json, env_override).await
        }
        MessagesCommand::Reply { thread_id, body } => {
            handle_reply(*thread_id, body, json, env_override).await
        }
        MessagesCommand::MarkRead { thread_id } => {
            handle_mark_read(*thread_id, json, env_override).await
        }
        MessagesCommand::Delete { thread_id } => {
            handle_delete(*thread_id, json, env_override).await
        }
        MessagesCommand::Folders => handle_folders(json, env_override).await,
        MessagesCommand::Move { thread_id, folder } => {
            handle_move(*thread_id, *folder, json, env_override).await
        }
    }
}

// ---------------------------------------------------------------------------
// List threads
// ---------------------------------------------------------------------------

async fn handle_list(
    limit: u32,
    page: Option<u32>,
    unread: bool,
    marked: bool,
    folder: Option<i64>,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    let filter_type = if unread {
        Some(FilterAndSortType::FilterUnread)
    } else if marked {
        Some(FilterAndSortType::FilterMarked)
    } else {
        None
    };

    let args = GetThreadListArguments {
        folder_id: folder,
        filter_type,
        sort_type: Some(FilterAndSortType::SortDate),
        sort_order: None,
        page: page.map(|p| p as i32),
        thread_ids: None,
        mail_box_owner_type: None,
        mail_box_owners: None,
        active_children: None,
    };

    match messaging::get_thread_list(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_thread_list(&result, limit);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_thread_list(
    list: &aula_api::models::messaging::MessageThreadSubscriptionList,
    limit: u32,
) {
    let threads = match list.threads.as_ref() {
        Some(t) => t,
        None => {
            println!("No threads found.");
            return;
        }
    };

    if threads.is_empty() {
        println!("No threads found.");
        return;
    }

    // Header
    println!(
        "{:<6} {:<1} {:<20} {:<40} {:<20}",
        "ID", "", "FROM", "SUBJECT", "DATE"
    );
    println!("{}", "-".repeat(90));

    for (i, thread) in threads.iter().enumerate() {
        if i >= limit as usize {
            break;
        }

        let id = thread.id.map(|id| id.to_string()).unwrap_or_default();
        let unread_marker = if thread.read { " " } else { "*" };

        let from = thread
            .latest_message
            .as_ref()
            .and_then(|lm| lm.sender.as_ref())
            .and_then(|s| s.display_name.as_deref())
            .or_else(|| thread.creator.as_ref().and_then(|c| c.full_name.as_deref()))
            .unwrap_or("(unknown)");

        let subject = thread.subject.as_deref().unwrap_or("(no subject)");

        let date = thread
            .latest_message
            .as_ref()
            .and_then(|lm| lm.send_date_time.as_deref())
            .unwrap_or("");

        // Truncate fields for display
        let from_display = truncate(from, 20);
        let subject_display = truncate(subject, 40);
        let date_display = truncate_date(date);

        println!(
            "{:<6} {:<1} {:<20} {:<40} {:<20}",
            id, unread_marker, from_display, subject_display, date_display
        );
    }

    if let Some(page) = list.page {
        if list.more_messages_exist {
            eprintln!(
                "\n(page {page}, more threads available -- use --page {})",
                page + 1
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Read thread
// ---------------------------------------------------------------------------

async fn handle_read(thread_id: i64, page: Option<i32>, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = GetMessagesForThreadArguments {
        thread_id: Some(thread_id),
        page,
        common_inbox_id: None,
    };

    match messaging::get_thread_by_id(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                print_thread_messages(&result);
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

fn print_thread_messages(thread: &aula_api::models::messaging::MessagesInThreadDto) {
    // Header
    let subject = thread.subject.as_deref().unwrap_or("(no subject)");
    println!("Thread: {subject}");
    if let Some(folder) = thread.folder_name.as_deref() {
        print!("  Folder: {folder}");
    }
    if thread.muted {
        print!("  [MUTED]");
    }
    if thread.marked {
        print!("  [MARKED]");
    }
    if thread.sensitive {
        print!("  [SENSITIVE]");
    }
    println!();

    if let Some(count) = thread.total_message_count {
        println!("  Messages: {count}");
    }
    println!("{}", "=".repeat(72));

    // First message
    if let Some(ref msg) = thread.first_message {
        print_message(msg);
    }

    // Subsequent messages
    if let Some(ref messages) = thread.messages {
        for msg in messages {
            print_message(msg);
        }
    }

    if thread.more_messages_exist {
        let next_page = thread.page.unwrap_or(0) + 1;
        eprintln!("\n(more messages available -- use --page {next_page})");
    }
}

fn print_message(msg: &aula_api::models::messaging::MessageDto) {
    let sender = msg
        .sender
        .as_ref()
        .and_then(|s| s.full_name.as_deref())
        .unwrap_or("(unknown)");

    let date = msg.send_date_time.as_deref().unwrap_or("");
    let msg_type = msg.message_type.as_deref().unwrap_or("");

    println!();
    println!("--- {sender}  ({date})  [{msg_type}]");

    if let Some(ref text) = msg.text {
        if let Some(ref html) = text.html {
            // Strip HTML tags for terminal display.
            let plain = strip_html_tags(html);
            println!("{plain}");
        }
    }

    if let Some(ref attachments) = msg.attachments {
        if !attachments.is_empty() {
            println!("  [{} attachment(s)]", attachments.len());
        }
    }
}

// ---------------------------------------------------------------------------
// Send new thread
// ---------------------------------------------------------------------------

async fn handle_send(
    to: &[i64],
    subject: Option<&str>,
    body: &str,
    json: bool,
    env_override: Option<&str>,
) {
    let mut session = build_session(env_override);

    if to.is_empty() {
        eprintln!("error: at least one recipient is required (--to)");
        std::process::exit(1);
    }

    let recipients: Vec<RecipientApiModel> = to
        .iter()
        .map(|&id| RecipientApiModel {
            id: Some(id),
            otp_inbox_id: None,
            mail_box_owner_type: Some("InstitutionProfile".to_string()),
            profile_id: Some(id),
            is_deactivated: false,
            is_deleted: false,
            portal_role: None,
        })
        .collect();

    let args = StartNewThreadRequestArguments {
        message: Some(MessageContentRequest {
            attachment_ids: Some(vec![]),
            text: Some(body.to_string()),
        }),
        subject: subject.map(|s| s.to_string()),
        recipients: Some(recipients),
        bcc_recipients: Some(vec![]),
        sensitive: false,
        creator: None,
    };

    match messaging::start_new_thread(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Message sent.");
                // Try to extract thread ID from response
                if let Some(id) = result.get("id").or_else(|| result.get("threadId")) {
                    println!("  Thread: {id}");
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
// Reply to thread
// ---------------------------------------------------------------------------

async fn handle_reply(thread_id: i64, body: &str, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = ReplyMessageArgument {
        thread_id: Some(thread_id),
        message: Some(MessageContentRequest {
            attachment_ids: Some(vec![]),
            text: Some(body.to_string()),
        }),
        common_inbox_id: None,
        bundle_id: None,
    };

    match messaging::reply_to_thread(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Reply sent to thread {thread_id}.");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Mark read
// ---------------------------------------------------------------------------

async fn handle_mark_read(thread_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    // To mark as read, we set the last read message. We first need to fetch
    // the thread to find the latest message ID.
    let fetch_args = GetMessagesForThreadArguments {
        thread_id: Some(thread_id),
        page: None,
        common_inbox_id: None,
    };

    let thread = match messaging::get_thread_by_id(&mut session, &fetch_args).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: failed to fetch thread: {e}");
            std::process::exit(1);
        }
    };

    // Find the last message ID: check messages list, then first_message.
    let last_msg_id = thread
        .messages
        .as_ref()
        .and_then(|msgs| msgs.last())
        .and_then(|m| m.id.as_deref())
        .or_else(|| thread.first_message.as_ref().and_then(|m| m.id.as_deref()));

    let msg_id = match last_msg_id {
        Some(id) => id.to_string(),
        None => {
            eprintln!("error: thread has no messages to mark as read");
            std::process::exit(1);
        }
    };

    let args = SetLastMessageRequestArguments {
        message_id: Some(msg_id),
        thread_id: Some(thread_id),
        common_inbox_id: None,
    };

    match messaging::set_last_read_message(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Thread {thread_id} marked as read.");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Delete thread
// ---------------------------------------------------------------------------

async fn handle_delete(thread_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = DeleteThreadArguments {
        subscription_ids: None,
        thread_ids: Some(vec![thread_id]),
        common_inbox_id: None,
    };

    match messaging::delete_threads(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Thread {thread_id} deleted.");
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

// ---------------------------------------------------------------------------
// Folders
// ---------------------------------------------------------------------------

async fn handle_folders(json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = GetFoldersArguments {
        include_deleted_folders: false,
        common_inbox_id: None,
    };

    match messaging::get_folders(&mut session, &args).await {
        Ok(folders) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&folders).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else if folders.is_empty() {
                println!("No folders found.");
            } else {
                println!("{:<8} {:<20} {:<10}", "ID", "NAME", "TYPE");
                println!("{}", "-".repeat(40));
                for f in &folders {
                    let id = f.id.map(|id| id.to_string()).unwrap_or_default();
                    let name = f.name.as_deref().unwrap_or("(unnamed)");
                    let ftype = f
                        .folder_type
                        .as_ref()
                        .map(|t| format!("{t:?}"))
                        .unwrap_or_default();
                    println!("{:<8} {:<20} {:<10}", id, name, ftype);
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
// Move thread to folder
// ---------------------------------------------------------------------------

async fn handle_move(thread_id: i64, folder_id: i64, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = MoveThreadsToFolderRequestArguments {
        thread_ids: Some(vec![thread_id]),
        subscription_ids: None,
        folder_id: Some(folder_id),
        common_inbox_id: None,
    };

    match messaging::move_threads_to_folder(&mut session, &args).await {
        Ok(result) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).unwrap_or_else(|e| format!(
                        "{{\"error\": \"serialization failed: {e}\"}}"
                    ))
                );
            } else {
                println!("Thread {thread_id} moved to folder {folder_id}.");
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

/// Truncate a string to at most `max` characters, appending "..." if needed.
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max.saturating_sub(3)])
    }
}

/// Extract just the date portion (first 16 chars: "YYYY-MM-DDTHH:MM") from a datetime.
fn truncate_date(s: &str) -> String {
    if s.len() >= 16 {
        s[..16].replace('T', " ")
    } else {
        s.to_string()
    }
}

/// Very basic HTML tag stripping for terminal display.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                // Add newline for block-level tags that just closed
            }
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    // Decode common HTML entities
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}
