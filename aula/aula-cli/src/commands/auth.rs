//! Authentication subcommands.

use clap::Subcommand;

/// Authenticate with the Aula platform.
#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    /// Log in with username and password (UniLogin or MitID).
    Login {
        /// Username for authentication.
        #[arg(short, long)]
        username: Option<String>,
        /// Password for authentication.
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Log out and clear the current session.
    Logout,
    /// Show current authentication status.
    Status,
}

pub fn handle(cmd: &AuthCommand) {
    match cmd {
        AuthCommand::Login { .. } => println!("auth login: not yet implemented"),
        AuthCommand::Logout => println!("auth logout: not yet implemented"),
        AuthCommand::Status => println!("auth status: not yet implemented"),
    }
}
