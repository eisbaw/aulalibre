//! Config management subcommands.

use clap::Subcommand;

/// View and manage CLI configuration.
#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration (merged from file and defaults).
    Show,
    /// Show the configuration file path.
    Path,
    /// Set a configuration value.
    Set {
        /// Key to set (e.g. default_environment, default_format, default_profile).
        key: String,
        /// Value to set.
        value: String,
    },
    /// Initialize a default configuration file.
    Init,
}

pub fn handle(cmd: &ConfigCommand) {
    match cmd {
        ConfigCommand::Show => println!("config show: not yet implemented"),
        ConfigCommand::Path => match crate::config::Config::path() {
            Some(p) => println!("{}", p.display()),
            None => println!("Could not determine config directory"),
        },
        ConfigCommand::Set { .. } => println!("config set: not yet implemented"),
        ConfigCommand::Init => println!("config init: not yet implemented"),
    }
}
