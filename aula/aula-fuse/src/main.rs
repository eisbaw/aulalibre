//! Aula FUSE filesystem binary.
//!
//! Mounts Aula school platform data as a read-only FUSE filesystem.
//!
//! # Usage
//!
//! ```text
//! aula-fuse /mnt/aula           # mount to /mnt/aula (production)
//! aula-fuse /mnt/aula --verbose # mount with debug logging
//! ```

mod cache;
mod fs;
mod inode_table;
mod sanitize;
mod timestamp;

use std::sync::{Arc, Mutex};

use clap::Parser;
use fuser::MountOption;

use aula_api::{AulaClient, AulaClientConfig, Environment, Session, SessionConfig, TokenStore};

/// Aula FUSE filesystem -- mount Aula data as a directory tree.
#[derive(Parser, Debug)]
#[command(name = "aula-fuse", version, about)]
struct Args {
    /// Mount point directory (must exist).
    mountpoint: String,

    /// Aula environment (production, preprod, hotfix).
    #[arg(long, default_value = "production")]
    env: String,

    /// Enable verbose/debug logging.
    #[arg(long, short)]
    verbose: bool,
}

fn parse_environment(s: &str) -> Environment {
    match s.to_lowercase().as_str() {
        "production" | "prod" => Environment::Production,
        "preprod" | "pre" => Environment::Preprod,
        "hotfix" => Environment::Hotfix,
        "test1" => Environment::Test1,
        "test3" => Environment::Test3,
        "dev1" => Environment::Dev1,
        "dev3" => Environment::Dev3,
        "dev11" => Environment::Dev11,
        _ => {
            eprintln!("Unknown environment '{}', defaulting to production", s);
            Environment::Production
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging.
    let log_level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::Builder::new().filter_level(log_level).init();

    // Parse environment.
    let environment = parse_environment(&args.env);

    // Create the tokio runtime.
    let rt = tokio::runtime::Runtime::new()?;

    // Create session.
    let client = AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 23,
    })?;
    let store =
        TokenStore::default_location().expect("Cannot determine default token store location");
    let mut session = Session::new(client, store, SessionConfig::default())?;

    // Verify we have valid tokens.
    if !session.has_valid_tokens() {
        eprintln!("Error: No valid Aula session tokens found.");
        eprintln!("Please log in first using aula-cli: cargo run -p aula-cli -- auth login");
        std::process::exit(1);
    }

    // Initialize context (required for API calls).
    rt.block_on(session.ensure_context_initialized())?;
    log::info!("Session initialized, mounting filesystem...");

    let session = Arc::new(Mutex::new(session));
    let aula_fs = fs::AulaFs::new(session, rt.handle().clone());

    // Verify mount point exists.
    if !std::path::Path::new(&args.mountpoint).is_dir() {
        eprintln!(
            "Error: Mount point '{}' does not exist or is not a directory.",
            args.mountpoint
        );
        std::process::exit(1);
    }

    let mount_options = vec![
        MountOption::RO,
        MountOption::FSName("aula".to_string()),
        MountOption::AutoUnmount,
        MountOption::AllowOther,
    ];

    log::info!("Mounting Aula filesystem at {}", args.mountpoint);
    log::info!("Press Ctrl+C to unmount and exit.");

    // Mount the filesystem (blocks until unmounted).
    fuser::mount2(aula_fs, &args.mountpoint, &mount_options)?;

    log::info!("Filesystem unmounted.");
    Ok(())
}
