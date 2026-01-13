//! MCP (Model Context Protocol) server for vulcan-todo
//!
//! This module provides MCP server functionality for OpenCode integration,
//! allowing AI agents to interact with the task manager.

pub mod protocol;
pub mod server;
pub mod tools;

pub use server::run_mcp_server;
pub use tools::{get_tools, Tool, ToolContext, ToolResult};
