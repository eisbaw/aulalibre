//! aula-cli -- command-line interface for the Aula school platform.
//!
//! Built on top of the `aula-api` library crate. Provides subcommands for each
//! major domain: auth, messages, calendar, presence, posts, gallery, documents,
//! notifications, search, groups, profile, and config.

mod commands;
mod config;

use clap::{Parser, Subcommand};

use commands::{
    auth, calendar, config as config_cmd, documents, gallery, groups, messages, notifications,
    posts, presence, profile, search,
};

/// CLI tool for interacting with the Aula school platform.
///
/// Aula is Denmark's school communication platform by Netcompany A/S.
/// This tool provides command-line access to messages, calendar, presence,
/// posts, gallery, documents, notifications, search, and more.
#[derive(Parser)]
#[command(name = "aula", version, about, long_about = None)]
struct Cli {
    /// Output results as JSON instead of human-readable text.
    #[arg(long, global = true)]
    json: bool,

    /// Aula environment to connect to.
    ///
    /// Valid values: production, preprod, hotfix, test1, test3, dev1, dev3, dev11.
    /// Overrides the default_environment setting in config.toml.
    #[arg(long, global = true)]
    env: Option<String>,

    /// Enable verbose output (debug logging).
    #[arg(long, short, global = true)]
    verbose: bool,

    /// Institution profile selector.
    ///
    /// When a user has children at multiple institutions, this selects which
    /// institution profile to use. Overrides default_profile in config.toml.
    #[arg(long, global = true)]
    profile: Option<String>,

    /// Print version information and exit.
    #[arg(long)]
    info: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

/// Top-level subcommands, one per API domain.
#[derive(Debug, Subcommand)]
enum Command {
    /// Authenticate with the Aula platform (login, logout, status).
    #[command(subcommand)]
    Auth(auth::AuthCommand),

    /// Read and send messages (threads).
    #[command(subcommand)]
    Messages(messages::MessagesCommand),

    /// View and manage calendar events.
    #[command(subcommand)]
    Calendar(calendar::CalendarCommand),

    /// View and manage child presence (attendance).
    #[command(subcommand)]
    Presence(presence::PresenceCommand),

    /// View and manage posts in the institution feed.
    #[command(subcommand)]
    Posts(posts::PostsCommand),

    /// Browse and download gallery media.
    #[command(subcommand)]
    Gallery(gallery::GalleryCommand),

    /// Browse and download shared documents.
    #[command(subcommand)]
    Documents(documents::DocumentsCommand),

    /// View and manage notifications.
    #[command(subcommand)]
    Notifications(notifications::NotificationsCommand),

    /// Search across Aula content.
    Search(search::SearchCommand),

    /// View and manage groups.
    #[command(subcommand)]
    Groups(groups::GroupsCommand),

    /// View and manage user profiles.
    #[command(subcommand)]
    Profile(profile::ProfileCommand),

    /// View and manage CLI configuration.
    #[command(subcommand)]
    Config(config_cmd::ConfigCommand),
}

#[tokio::main]
async fn main() {
    let cfg = config::Config::load();
    let cli = Cli::parse();

    if cli.verbose || cfg.verbose.unwrap_or(false) {
        eprintln!("[verbose] config loaded from {:?}", config::Config::path());
        if let Some(ref env) = cli.env {
            eprintln!("[verbose] env override: {env}");
        }
        if cli.json {
            eprintln!("[verbose] JSON output enabled");
        }
    }

    if cli.info {
        println!("aula-cli v{}", env!("CARGO_PKG_VERSION"));
        println!("aula-api v{}", aula_api::version());
        return;
    }

    // Resolve environment: CLI flag > config file > default (production).
    let env_str = cli.env.as_deref().or(cfg.default_environment.as_deref());

    match cli.command {
        Some(Command::Auth(ref cmd)) => auth::handle(cmd, env_str).await,
        Some(Command::Messages(ref cmd)) => messages::handle(cmd, cli.json, env_str).await,
        Some(Command::Calendar(ref cmd)) => calendar::handle(cmd, cli.json, env_str).await,
        Some(Command::Presence(ref cmd)) => presence::handle(cmd),
        Some(Command::Posts(ref cmd)) => posts::handle(cmd),
        Some(Command::Gallery(ref cmd)) => gallery::handle(cmd),
        Some(Command::Documents(ref cmd)) => documents::handle(cmd),
        Some(Command::Notifications(ref cmd)) => notifications::handle(cmd),
        Some(Command::Search(ref cmd)) => search::handle(cmd),
        Some(Command::Groups(ref cmd)) => groups::handle(cmd),
        Some(Command::Profile(ref cmd)) => profile::handle(cmd),
        Some(Command::Config(ref cmd)) => config_cmd::handle(cmd),
        None => {
            println!(
                "aula-cli v{} -- no command given, try --help",
                env!("CARGO_PKG_VERSION")
            );
        }
    }
}
