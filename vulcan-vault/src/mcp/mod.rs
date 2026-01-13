//! MCP (Model Context Protocol) server implementation
//!
//! Exposes vulcan-vault functionality as MCP tools for AI agents.
//! Uses JSON-RPC 2.0 protocol over stdio.


mod server;
mod protocol;
mod tools;

pub use server::run_server;
