//! Calendar subcommands.

use clap::Subcommand;

/// View and manage calendar events.
#[derive(Debug, Subcommand)]
pub enum CalendarCommand {
    /// List calendar events.
    List {
        /// Start date (YYYY-MM-DD). Defaults to today.
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD). Defaults to 7 days from start.
        #[arg(long)]
        to: Option<String>,
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Show details for a single event.
    Show {
        /// Event ID.
        event_id: u64,
    },
}

pub fn handle(cmd: &CalendarCommand) {
    match cmd {
        CalendarCommand::List { .. } => println!("calendar list: not yet implemented"),
        CalendarCommand::Show { .. } => println!("calendar show: not yet implemented"),
    }
}
