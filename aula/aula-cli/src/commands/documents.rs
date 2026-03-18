//! Document subcommands.

use clap::Subcommand;

/// Browse and download shared documents.
#[derive(Debug, Subcommand)]
pub enum DocumentsCommand {
    /// List documents in a folder.
    List {
        /// Parent folder ID (omit for root).
        #[arg(long)]
        folder: Option<u64>,
        /// Filter by institution profile ID.
        #[arg(long)]
        institution: Option<u64>,
    },
    /// Download a document by ID.
    Download {
        /// Document ID.
        document_id: u64,
        /// Output path.
        #[arg(short, long)]
        output: Option<String>,
    },
}

pub fn handle(cmd: &DocumentsCommand) {
    match cmd {
        DocumentsCommand::List { .. } => println!("documents list: not yet implemented"),
        DocumentsCommand::Download { .. } => println!("documents download: not yet implemented"),
    }
}
