//! Messaging subcommands.

use clap::Subcommand;

/// Read and send messages (threads).
#[derive(Debug, Subcommand)]
pub enum MessagesCommand {
    /// List message threads.
    List {
        /// Maximum number of threads to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
        /// Page number for pagination.
        #[arg(long)]
        page: Option<u32>,
    },
    /// Show a single thread by ID.
    Show {
        /// Thread ID.
        thread_id: u64,
    },
    /// Send a new message.
    Send {
        /// Recipient profile IDs (comma-separated).
        #[arg(short, long, value_delimiter = ',')]
        to: Vec<u64>,
        /// Message subject.
        #[arg(short, long)]
        subject: Option<String>,
        /// Message body text.
        #[arg(short, long)]
        body: String,
    },
    /// Mark a thread as read.
    MarkRead {
        /// Thread ID.
        thread_id: u64,
    },
    /// Delete a thread.
    Delete {
        /// Thread ID.
        thread_id: u64,
    },
}

pub fn handle(cmd: &MessagesCommand) {
    match cmd {
        MessagesCommand::List { .. } => println!("messages list: not yet implemented"),
        MessagesCommand::Show { .. } => println!("messages show: not yet implemented"),
        MessagesCommand::Send { .. } => println!("messages send: not yet implemented"),
        MessagesCommand::MarkRead { .. } => println!("messages mark-read: not yet implemented"),
        MessagesCommand::Delete { .. } => println!("messages delete: not yet implemented"),
    }
}
