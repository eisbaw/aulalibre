//! Shared session construction helpers for aula-cli commands.
//!
//! Eliminates the duplicated `resolve_environment`, `token_store`, and
//! `build_session` functions that were copy-pasted across every command module.

use aula_api::client::{AulaClient, AulaClientConfig, Environment};
use aula_api::session::{Session, SessionConfig};
use aula_api::token_store::TokenStore;

/// Map a CLI environment string to an [`Environment`] enum value.
pub fn resolve_environment(env: Option<&str>) -> Environment {
    match env {
        Some("preprod") => Environment::Preprod,
        Some("hotfix") => Environment::Hotfix,
        Some("test1") => Environment::Test1,
        Some("test3") => Environment::Test3,
        Some("dev1") => Environment::Dev1,
        Some("dev3") => Environment::Dev3,
        Some("dev11") => Environment::Dev11,
        _ => Environment::Production,
    }
}

/// Get the default token store location.
pub fn token_store() -> TokenStore {
    TokenStore::default_location().unwrap_or_else(|| {
        eprintln!("warning: could not determine data directory, using ./aula-data");
        TokenStore::new("./aula-data")
    })
}

/// Build an authenticated session, exiting on failure.
///
/// This checks for stored tokens and creates a fully-configured
/// `Session` ready for API calls.
pub fn build_session(env_override: Option<&str>) -> Session {
    let environment = resolve_environment(env_override);
    let store = token_store();

    if !store.exists() {
        eprintln!("Not logged in. Run 'aula auth login' first.");
        std::process::exit(1);
    }

    let client = match AulaClient::with_config(AulaClientConfig {
        environment,
        api_version: 19,
    }) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: failed to create client: {e}");
            std::process::exit(1);
        }
    };

    match Session::new(client, store, SessionConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to create session: {e}");
            std::process::exit(1);
        }
    }
}
