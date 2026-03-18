//! Groups subcommands.

use clap::Subcommand;

/// View and manage groups.
#[derive(Debug, Subcommand)]
pub enum GroupsCommand {
    /// List groups for the current user.
    List {
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Show group details.
    Show {
        /// Group ID.
        group_id: u64,
    },
    /// List members of a group.
    Members {
        /// Group ID.
        group_id: u64,
    },
}

pub fn handle(cmd: &GroupsCommand) {
    match cmd {
        GroupsCommand::List { .. } => println!("groups list: not yet implemented"),
        GroupsCommand::Show { .. } => println!("groups show: not yet implemented"),
        GroupsCommand::Members { .. } => println!("groups members: not yet implemented"),
    }
}
