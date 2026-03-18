//! Notification subcommands.

use clap::Subcommand;

/// View and manage notifications.
#[derive(Debug, Subcommand)]
pub enum NotificationsCommand {
    /// List recent notifications.
    List {
        /// Maximum number of notifications to show.
        #[arg(short = 'n', long, default_value = "20")]
        limit: u32,
    },
    /// Mark a notification as read.
    MarkRead {
        /// Notification ID.
        notification_id: u64,
    },
    /// Mark all notifications as read.
    MarkAllRead,
}

pub fn handle(cmd: &NotificationsCommand) {
    match cmd {
        NotificationsCommand::List { .. } => println!("notifications list: not yet implemented"),
        NotificationsCommand::MarkRead { .. } => {
            println!("notifications mark-read: not yet implemented")
        }
        NotificationsCommand::MarkAllRead => {
            println!("notifications mark-all-read: not yet implemented")
        }
    }
}
