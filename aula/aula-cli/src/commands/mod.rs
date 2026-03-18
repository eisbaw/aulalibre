//! Subcommand definitions for aula-cli.
//!
//! Each module defines the clap subcommands for one API domain.
//! Handlers are stubs for now -- actual API integration comes in later tasks.

pub mod auth;
pub mod calendar;
pub mod config;
pub mod documents;
pub mod gallery;
pub mod groups;
pub mod messages;
pub mod notifications;
pub mod posts;
pub mod presence;
pub mod profile;
pub mod search;
