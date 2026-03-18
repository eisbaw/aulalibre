//! Domain model types extracted from the Aula .NET assemblies.
//!
//! These structs mirror the C# model classes found in `AulaNative.dll`,
//! organized by domain area. They use `serde(rename_all = "camelCase")`
//! to match the .NET JSON serialization conventions.

pub mod calendar;
pub mod documents;
pub mod files;
pub mod gallery;
pub mod groups;
pub mod institutions;
pub mod messaging;
pub mod posts;
pub mod presence;
pub mod profiles;
pub mod users;
