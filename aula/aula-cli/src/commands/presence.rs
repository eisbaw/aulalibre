//! Presence/attendance subcommands.

use clap::Subcommand;

/// View and manage child presence (attendance).
#[derive(Debug, Subcommand)]
pub enum PresenceCommand {
    /// Show current presence status for children.
    Status {
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Report a child as absent.
    ReportAbsence {
        /// Child's institution profile ID.
        #[arg(long)]
        child: u64,
        /// Start date (YYYY-MM-DD).
        #[arg(long)]
        from: String,
        /// End date (YYYY-MM-DD).
        #[arg(long)]
        to: String,
        /// Reason for absence.
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// View presence history.
    History {
        /// Child's institution profile ID.
        #[arg(long)]
        child: u64,
        /// Start date (YYYY-MM-DD).
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD).
        #[arg(long)]
        to: Option<String>,
    },
}

pub fn handle(cmd: &PresenceCommand) {
    match cmd {
        PresenceCommand::Status { .. } => println!("presence status: not yet implemented"),
        PresenceCommand::ReportAbsence { .. } => {
            println!("presence report-absence: not yet implemented")
        }
        PresenceCommand::History { .. } => println!("presence history: not yet implemented"),
    }
}
