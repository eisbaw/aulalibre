//! File-based token storage for persisting login data across sessions.
//!
//! Mirrors the `SecureStorageManager` / `SecureStorageHelper` pattern from the
//! APK (see `auth_flow.md` Section 5), but uses file-based JSON storage in
//! `$XDG_DATA_HOME/aula/` (or platform-appropriate fallback) instead of
//! Android Keystore-backed encrypted preferences.
//!
//! # Security notes
//!
//! - Token files are created with restrictive permissions (0600 on Unix).
//! - Token values are never logged or printed.
//! - The `secrets/` directory or XDG data directory should be excluded from
//!   version control (gitignored).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::auth::LoginData;
use crate::error::AulaError;

/// File name for the persisted token data.
const TOKEN_FILE_NAME: &str = "tokens.json";

/// Subdirectory under XDG_DATA_HOME (or fallback).
const APP_DIR_NAME: &str = "aula";

// ---------------------------------------------------------------------------
// TokenStore
// ---------------------------------------------------------------------------

/// File-based token persistence.
///
/// Stores [`LoginData`] as JSON on disk. The storage directory is determined
/// by (in priority order):
///
/// 1. An explicit path passed to [`TokenStore::new`].
/// 2. `$XDG_DATA_HOME/aula/` (typically `~/.local/share/aula/`).
///
/// This is the CLI equivalent of the APK's `SecureStorageHelper.Save<T>()` /
/// `SecureStorageHelper.FindValueForKey<T>()`.
/// Where tokens are kept.
#[derive(Debug, Clone)]
enum Backend {
    /// `tokens.json` inside a directory (CLI, local stdio server).
    File { dir: PathBuf },
    /// Process memory. Hosted deployments load tokens from their own storage
    /// (e.g. Supabase) and seed a session with them, then write back whatever
    /// the session ends up with.
    Memory(Arc<Mutex<Option<LoginData>>>),
}

#[derive(Debug, Clone)]
pub struct TokenStore {
    backend: Backend,
}

impl TokenStore {
    /// Create a store at the given directory path.
    ///
    /// The directory is created (with parents) if it does not exist.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self {
            backend: Backend::File { dir: dir.into() },
        }
    }

    /// Create a store that keeps tokens in memory only.
    pub fn in_memory(tokens: Option<LoginData>) -> Self {
        Self {
            backend: Backend::Memory(Arc::new(Mutex::new(tokens))),
        }
    }

    /// Create a store using the platform-appropriate data directory.
    ///
    /// Uses `$XDG_DATA_HOME/aula/` on Linux, `~/Library/Application Support/aula/`
    /// on macOS, or `%APPDATA%/aula/` on Windows.
    ///
    /// Returns `None` if the home directory cannot be determined.
    pub fn default_location() -> Option<Self> {
        // Escape hatch for launchers whose data directory differs from the
        // shell's (MCP clients, sandboxed apps, systemd units).
        if let Some(dir) = std::env::var_os("AULA_DATA_DIR") {
            return Some(Self::new(PathBuf::from(dir)));
        }
        let base = dirs::data_dir()?;
        Some(Self::new(base.join(APP_DIR_NAME)))
    }

    /// The directory where tokens are stored, if this store is file-backed.
    pub fn dir(&self) -> Option<&Path> {
        match &self.backend {
            Backend::File { dir } => Some(dir.as_path()),
            Backend::Memory(_) => None,
        }
    }

    /// Full path to the token file, if this store is file-backed.
    pub fn token_path(&self) -> Option<PathBuf> {
        match &self.backend {
            Backend::File { dir } => Some(dir.join(TOKEN_FILE_NAME)),
            Backend::Memory(_) => None,
        }
    }

    /// Load persisted login data.
    ///
    /// Returns `Ok(None)` if no tokens have been stored yet.
    /// Returns `Err` on I/O or deserialization errors.
    pub fn load(&self) -> crate::Result<Option<LoginData>> {
        let path = match &self.backend {
            Backend::Memory(tokens) => {
                return Ok(tokens.lock().expect("token lock").clone());
            }
            Backend::File { dir } => dir.join(TOKEN_FILE_NAME),
        };

        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                let data: LoginData = serde_json::from_str(&contents)?;
                Ok(Some(data))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AulaError::Io(e)),
        }
    }

    /// Save login data.
    ///
    /// File-backed stores create the directory if needed and write the token
    /// file with mode 0600 (owner read/write only) on Unix.
    pub fn save(&self, data: &LoginData) -> crate::Result<()> {
        let dir = match &self.backend {
            Backend::Memory(tokens) => {
                *tokens.lock().expect("token lock") = Some(data.clone());
                return Ok(());
            }
            Backend::File { dir } => dir,
        };

        std::fs::create_dir_all(dir)?;

        let json = serde_json::to_string_pretty(data)?;
        let path = dir.join(TOKEN_FILE_NAME);

        // Write atomically: write to temp file, then rename.
        let tmp_path = dir.join(".tokens.json.tmp");
        std::fs::write(&tmp_path, json.as_bytes())?;

        // Set restrictive permissions on Unix before renaming.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(&tmp_path, perms)?;
        }

        std::fs::rename(&tmp_path, &path)?;
        Ok(())
    }

    /// Clear persisted tokens.
    ///
    /// Idempotent: succeeds even if nothing was stored.
    pub fn clear(&self) -> crate::Result<()> {
        let path = match &self.backend {
            Backend::Memory(tokens) => {
                *tokens.lock().expect("token lock") = None;
                return Ok(());
            }
            Backend::File { dir } => dir.join(TOKEN_FILE_NAME),
        };

        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AulaError::Io(e)),
        }
    }

    /// Check whether tokens are available.
    pub fn exists(&self) -> bool {
        match &self.backend {
            Backend::Memory(tokens) => tokens.lock().expect("token lock").is_some(),
            Backend::File { dir } => dir.join(TOKEN_FILE_NAME).is_file(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthLevel;

    fn test_login_data() -> LoginData {
        LoginData {
            access_token: "test_access_token".to_string(),
            refresh_token: Some("test_refresh_token".to_string()),
            expires_in: Some(3600),
            access_token_expiration: Some(1700000000),
            auth_level: AuthLevel::Level2,
            error: None,
            error_description: None,
        }
    }

    #[test]
    fn memory_store_roundtrips_without_touching_disk() {
        let store = TokenStore::in_memory(None);
        assert!(!store.exists());
        assert!(store.token_path().is_none(), "memory store has no path");
        assert!(store.dir().is_none());

        store.save(&test_login_data()).unwrap();
        assert!(store.exists());
        let loaded = store.load().unwrap().expect("seeded tokens");
        assert_eq!(loaded.access_token, "test_access_token");

        store.clear().unwrap();
        assert!(!store.exists());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!("aula_test_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        let data = test_login_data();
        store.save(&data).unwrap();

        assert!(store.exists());

        let loaded = store.load().unwrap().expect("should have data");
        assert_eq!(loaded.access_token, data.access_token);
        assert_eq!(loaded.refresh_token, data.refresh_token);
        assert_eq!(loaded.auth_level, data.auth_level);
        assert_eq!(loaded.access_token_expiration, data.access_token_expiration);

        // Cleanup.
        store.clear().unwrap();
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_nonexistent_returns_none() {
        let dir = std::env::temp_dir().join(format!("aula_test_noexist_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        let result = store.load().unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn clear_nonexistent_is_ok() {
        let dir = std::env::temp_dir().join(format!("aula_test_clear_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        // Should not fail even if nothing exists.
        store.clear().unwrap();
    }

    #[test]
    fn clear_removes_file() {
        let dir = std::env::temp_dir().join(format!("aula_test_clearrm_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        store.save(&test_login_data()).unwrap();
        assert!(store.exists());

        store.clear().unwrap();
        assert!(!store.exists());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn exists_false_when_no_file() {
        let dir = std::env::temp_dir().join(format!("aula_test_exists_{}", std::process::id()));
        let store = TokenStore::new(&dir);
        assert!(!store.exists());
    }

    #[test]
    fn save_creates_directory() {
        let dir =
            std::env::temp_dir().join(format!("aula_test_mkdir_{}/sub/dir", std::process::id()));
        let store = TokenStore::new(&dir);

        store.save(&test_login_data()).unwrap();
        assert!(dir.is_dir());
        assert!(store.exists());

        std::fs::remove_dir_all(
            std::env::temp_dir().join(format!("aula_test_mkdir_{}", std::process::id())),
        )
        .ok();
    }

    #[cfg(unix)]
    #[test]
    fn token_file_has_restrictive_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = std::env::temp_dir().join(format!("aula_test_perms_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        store.save(&test_login_data()).unwrap();

        let meta = std::fs::metadata(store.token_path().expect("file-backed store")).unwrap();
        let mode = meta.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "token file should be owner-only read/write");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_corrupt_json_returns_error() {
        let dir = std::env::temp_dir().join(format!("aula_test_corrupt_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(TOKEN_FILE_NAME), "not valid json").unwrap();

        let store = TokenStore::new(&dir);
        let result = store.load();
        assert!(result.is_err());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn default_location_returns_some() {
        // This should work on any machine with a home directory.
        let store = TokenStore::default_location();
        assert!(store.is_some(), "should find a data directory");
    }

    #[test]
    fn overwrite_existing_tokens() {
        let dir = std::env::temp_dir().join(format!("aula_test_overwrite_{}", std::process::id()));
        let store = TokenStore::new(&dir);

        let mut data = test_login_data();
        store.save(&data).unwrap();

        data.access_token = "new_token".to_string();
        data.auth_level = AuthLevel::Level3;
        store.save(&data).unwrap();

        let loaded = store.load().unwrap().unwrap();
        assert_eq!(loaded.access_token, "new_token");
        assert_eq!(loaded.auth_level, AuthLevel::Level3);

        std::fs::remove_dir_all(&dir).ok();
    }
}
