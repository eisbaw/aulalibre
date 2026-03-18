//! Profile subcommands.

use clap::Subcommand;

/// View and manage user profiles.
#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    /// Show current user's profile.
    Me,
    /// Show a profile by institution profile ID.
    Show {
        /// Institution profile ID.
        profile_id: u64,
    },
    /// List children associated with the current user.
    Children,
    /// List institution profiles for the current user.
    Institutions,
}

pub fn handle(cmd: &ProfileCommand) {
    match cmd {
        ProfileCommand::Me => println!("profile me: not yet implemented"),
        ProfileCommand::Show { .. } => println!("profile show: not yet implemented"),
        ProfileCommand::Children => println!("profile children: not yet implemented"),
        ProfileCommand::Institutions => println!("profile institutions: not yet implemented"),
    }
}
