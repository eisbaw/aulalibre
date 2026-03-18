//! API service modules.
//!
//! Each module groups related API endpoints into ergonomic async methods
//! that operate on a [`Session`](crate::Session) with automatic token refresh.
//!
//! Service modules correspond to the service classes discovered in the
//! decompiled `AulaNative.Services.Web` namespace
//! (see `api_endpoints.md` Section 3).

pub mod additional_master_data;
pub mod calendar;
pub mod comments;
pub mod configuration;
pub mod consent;
pub mod documents;
pub mod files;
pub mod gallery;
pub mod groups;
pub mod health;
pub mod messaging;
pub mod notifications;
pub mod onboarding;
pub mod personal_reference;
pub mod posts;
pub mod presence;
pub mod profiles;
pub mod push_notifications;
pub mod search;
pub mod widget;
