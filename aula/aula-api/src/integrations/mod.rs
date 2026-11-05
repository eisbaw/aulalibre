//! Third-party school platforms reached through Aula widget tokens.
//!
//! Aula does not expose week plans, homework or week letters itself. Schools
//! embed those from external providers, and Aula issues a per-widget SSO token
//! (`aulaToken.getAulaToken`) that the provider accepts as a bearer token. So
//! these integrations need no separate credentials — they ride on the Aula
//! session.
//!
//! Each module keeps the network call separable from the Aula plumbing
//! ([`meebook::fetch`] vs [`meebook::week_plan`]) so it can be tested against a
//! mock server.

pub mod meebook;
