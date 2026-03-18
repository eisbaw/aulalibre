//! Search subcommands.

use clap::Args;

/// Search across Aula content.
#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Search query string.
    pub query: String,
    /// Maximum number of results.
    #[arg(short = 'n', long, default_value = "20")]
    pub limit: u32,
    /// Filter by institution profile ID.
    #[arg(long)]
    pub institution: Option<u64>,
}

pub fn handle(cmd: &SearchCommand) {
    println!(
        "search '{}' (limit {}): not yet implemented",
        cmd.query, cmd.limit
    );
}
