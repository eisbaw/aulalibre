//! Shared session construction helpers for aula-cli commands.
//!
//! Eliminates the duplicated `resolve_environment`, `token_store`, and
//! `build_session` functions that were copy-pasted across every command module.

use aula_api::client::Environment;
use aula_api::session::Session;
use aula_api::token_store::TokenStore;

/// Map a CLI environment string to an [`Environment`] enum value.
pub fn resolve_environment(env: Option<&str>) -> Environment {
    Environment::from_name(env.unwrap_or("production"))
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
/// Checks for stored tokens and creates a fully-configured `Session` ready for
/// API calls. The MCP server uses [`aula_api::session::from_store`] directly and
/// reports failures as tool errors instead of exiting.
pub fn build_session(env_override: Option<&str>) -> Session {
    let environment = resolve_environment(env_override);

    if !token_store().exists() {
        eprintln!("Not logged in. Run 'aula auth login' first.");
        std::process::exit(1);
    }

    match aula_api::session::from_store(environment) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to create session: {e}");
            std::process::exit(1);
        }
    }
}
