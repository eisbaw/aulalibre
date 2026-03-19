//! Messaging subcommands: list threads, read, send, reply, delete, folders, move.

use clap::Subcommand;

use aula_api::enums::common::FilterAndSortType;
use aula_api::models::messaging::{
    DeleteThreadArguments, GetFoldersArguments, GetMessagesForThreadArguments,
    GetThreadListArguments, MessageContentRequest, MoveThreadsToFolderRequestArguments,
    RecipientApiModel, ReplyMessageArgument, SetLastMessageRequestArguments,
    StartNewThreadRequestArguments,
};
use aula_api::services::messaging;

use crate::output::{
    self, bold, dim, format_datetime, print_json, print_pagination_hint, strip_html_tags, truncate,
    unread_marker, Column, Table,
};
use crate::session_util::build_session;

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
                print_json(&result);
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

    let table = Table::new(vec![
        Column::new("ID", 6),
        Column::new("", 1),
        Column::new("FROM", 20),
        Column::new("SUBJECT", 40),
        Column::new("DATE", 20),
    ]);
    table.print_header();

    for (i, thread) in threads.iter().enumerate() {
        if i >= limit as usize {
            break;
        }

        let id = thread.id.map(|id| id.to_string()).unwrap_or_default();
        let marker = unread_marker(thread.read);

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

        // Build the row -- unread marker may have ANSI codes
        let row_str = format!(
            "{:<6} {} {:<20} {:<40} {:<20}",
            id,
            marker,
            truncate(from, 20),
            truncate(subject, 40),
            format_datetime(date)
        );
        println!("{row_str}");
    }

    if let Some(page) = list.page {
        print_pagination_hint(Some(page), list.more_messages_exist, "--page");
    }
}

// ---------------------------------------------------------------------------
// Read thread
// ---------------------------------------------------------------------------

async fn handle_read(thread_id: i64, page: Option<i32>, json: bool, env_override: Option<&str>) {
    let mut session = build_session(env_override);

    let args = GetMessagesForThreadArguments {
        thread_id: Some(thread_id),
        page: Some(page.unwrap_or(0)),
        common_inbox_id: None,
    };

    match messaging::get_thread_by_id(&mut session, &args).await {
        Ok(result) => {
            if json {
                print_json(&result);
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
    let subject = thread.subject.as_deref().unwrap_or("(no subject)");
    println!("{}", bold(&format!("Thread: {subject}")));
    if let Some(folder) = thread.folder_name.as_deref() {
        print!("  Folder: {folder}");
    }
    if thread.muted {
        print!("  {}", dim("[MUTED]"));
    }
    if thread.marked {
        print!("  {}", output::yellow("[MARKED]"));
    }
    if thread.sensitive {
        print!("  {}", output::red("[SENSITIVE]"));
    }
    println!();

    if let Some(count) = thread.total_message_count {
        println!("  Messages: {count}");
    }
    println!("{}", dim(&"=".repeat(72)));

    if let Some(ref msg) = thread.first_message {
        print_message(msg);
    }

    if let Some(ref messages) = thread.messages {
        for msg in messages {
            print_message(msg);
        }
    }

    if thread.more_messages_exist {
        let next_page = thread.page.unwrap_or(0) + 1;
        print_pagination_hint(Some(next_page - 1), true, "--page");
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
    println!("--- {}  ({})  [{}]", bold(sender), dim(date), dim(msg_type));

    if let Some(ref text) = msg.text {
        if let Some(ref html) = text.html {
            let plain = strip_html_tags(html);
            println!("{plain}");
        }
    }

    if let Some(ref attachments) = msg.attachments {
        if !attachments.is_empty() {
            println!(
                "  {}",
                dim(&format!("[{} attachment(s)]", attachments.len()))
            );
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
                print_json(&result);
            } else {
                println!("Message sent.");
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
                print_json(&result);
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
                print_json(&result);
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
                print_json(&result);
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
                print_json(&folders);
            } else if folders.is_empty() {
                println!("No folders found.");
            } else {
                let table = Table::new(vec![
                    Column::new("ID", 8),
                    Column::new("NAME", 20),
                    Column::new("TYPE", 10),
                ]);
                table.print_header();
                for f in &folders {
                    let id = f.id.map(|id| id.to_string()).unwrap_or_default();
                    let name = f.name.as_deref().unwrap_or("(unnamed)");
                    let ftype = f
                        .folder_type
                        .as_ref()
                        .map(|t| format!("{t:?}"))
                        .unwrap_or_default();
                    table.print_row(&[&id, name, &ftype]);
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
                print_json(&result);
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
