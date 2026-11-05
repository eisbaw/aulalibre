//! Vercel Functions adapter for `aula-mcp`.
//!
//! The server is a library plus transports: `aula-mcp` speaks JSON-RPC and
//! knows nothing about HTTP, and this crate maps a Vercel request onto it.
//! All the interesting logic lives in [`handler`] so it can be unit-tested
//! without deploying.

pub mod handler;
