//! Configuration file handling for aula-cli.
//!
//! Loads persistent settings from `$XDG_CONFIG_HOME/aula/config.toml`
//! (defaults to `~/.config/aula/config.toml` on Linux).

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Persistent CLI configuration loaded from disk.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Default environment to use when `--env` is not specified.
    pub default_environment: Option<String>,
    /// Default output format (`json` or `text`).
    pub default_format: Option<String>,
    /// Default institution profile name for `--profile`.
    pub default_profile: Option<String>,
    /// Enable verbose output by default.
    pub verbose: Option<bool>,
}

impl Config {
    /// Returns the path to the configuration file.
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("aula").join("config.toml"))
    }

    /// Load configuration from disk. Returns default config if the file does
    /// not exist or cannot be parsed.
    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };

        match fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}
