//! `aula-mcp` — Model Context Protocol server for the Aula school platform.
//!
//! The crate is a library plus two thin transports so the same dispatcher can
//! serve a local client over stdio and a hosted one over HTTP:
//!
//! - [`rpc`] — JSON-RPC 2.0 dispatch, transport-agnostic
//! - [`tools`] — tool registry, schemas and handlers
//! - [`compact`] — projections that shrink Aula's DTOs for an LLM
//!
//! Authentication is not handled here: tokens come from the store
//! `aula auth login` writes.

pub mod compact;
pub mod rpc;
pub mod tools;
