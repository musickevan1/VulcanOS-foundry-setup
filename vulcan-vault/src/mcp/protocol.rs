//! MCP protocol definitions
//!
//! JSON-RPC 2.0 types for MCP communication

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool definition for MCP tools/list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Tool call result
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ToolResult {
    pub content: Vec<ContentItem>,
    #[serde(rename = "isError", skip_serializing_if = "std::ops::Not::not")]
    pub is_error: bool,
}

/// Content item in tool result
#[allow(dead_code)]
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "text")]
    Text { text: String },
}

#[allow(dead_code)]
impl ToolResult {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: vec![ContentItem::Text {
                text: content.into(),
            }],
            is_error: false,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![ContentItem::Text {
                text: message.into(),
            }],
            is_error: true,
        }
    }
}
