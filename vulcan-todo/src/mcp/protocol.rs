//! MCP Protocol types for JSON-RPC communication
//!
//! Implements the Model Context Protocol for task management operations.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// MCP Protocol Version
pub const PROTOCOL_VERSION: &str = "2024-11-05";

/// JSON-RPC Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// JSON-RPC Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse {
    Success(JsonRpcSuccess),
    Error(JsonRpcError),
}

/// Successful JSON-RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcSuccess {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(default)]
    pub result: Value,
}

/// Error JSON-RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub jsonrpc: String,
    pub id: Value,
    pub error: JsonRpcErrorContent,
}

/// Error content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorContent {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub data: Option<Value>,
}

/// MCP Request params for tool calls
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolCallParams {
    #[serde(default)]
    pub _meta: Option<HashMap<String, Value>>,
}

/// MCP Result for tool results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    #[serde(serialize_with = "serialize_value")]
    pub content: Vec<ContentItem>,
    #[serde(default)]
    #[serde(rename = "isError")]
    pub isError: bool,
}

fn serialize_value<S>(v: &Vec<ContentItem>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serde_json::Value::Array(
        v.iter()
            .map(|c| serde_json::to_value(c).map_err(|e| serde::ser::Error::custom(e.to_string())))
            .collect::<Result<Vec<_>, _>>()?,
    )
    .serialize(serializer)
}

/// Content item in a tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentItem {
    /// Text content
    Text { text: String },
    /// Image content
    Image { data: String, mime_type: String },
    /// Resource content
    Resource { resource: ResourceContent },
}

/// Resource content for MCP resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    pub uri: String,
    pub mime_type: String,
    pub text: String,
}

/// Initialize request parameters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InitializeParams {
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub capabilities: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub client_info: Option<ClientInfo>,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
}

/// Initialize result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "capabilities")]
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "version")]
    pub version: Option<String>,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerCapabilities {
    #[serde(default)]
    #[serde(rename = "tools")]
    pub tools: Option<HashMap<String, ToolDefinition>>,
    #[serde(default)]
    #[serde(rename = "resources")]
    pub resources: Option<HashMap<String, Value>>,
    #[serde(default)]
    #[serde(rename = "prompts")]
    pub prompts: Option<HashMap<String, Value>>,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ToolDefinition {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(default, rename = "inputSchema")]
    pub inputSchema: Value,
    #[serde(
        default,
        rename = "outputSchema",
        skip_serializing_if = "Option::is_none"
    )]
    pub outputSchema: Option<Value>,
}

/// Tool call notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallNotification {
    pub method: String,
    pub params: ToolCallNotificationParams,
}

/// Tool call notification params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallNotificationParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Option<Value>,
}

/// List tools result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<ToolDefinition>,
}

impl JsonRpcRequest {
    /// Create a new request
    pub fn new(method: String, params: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Value::Null,
            method,
            params,
        }
    }

    /// Check if this is an initialize request
    pub fn is_initialize(&self) -> bool {
        self.method == "initialize"
    }

    /// Check if this is a notification
    pub fn is_notification(&self) -> bool {
        self.id == Value::Null
    }

    /// Parse tool name from tool/call request
    pub fn tool_name(&self) -> Option<String> {
        if self.method == "tools/call" {
            self.params
                .get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    }

    /// Parse tool arguments from tool/call request
    pub fn tool_arguments(&self) -> Option<Value> {
        if self.method == "tools/call" {
            self.params.get("arguments").cloned()
        } else {
            None
        }
    }
}

impl JsonRpcResponse {
    /// Create a success response
    pub fn success(id: Value, result: Value) -> Self {
        Self::Success(JsonRpcSuccess {
            jsonrpc: "2.0".to_string(),
            id,
            result,
        })
    }

    /// Create an error response
    pub fn error(id: Value, code: i32, message: String) -> Self {
        Self::Error(JsonRpcError {
            jsonrpc: "2.0".to_string(),
            id,
            error: JsonRpcErrorContent {
                code,
                message,
                data: None,
            },
        })
    }

    /// Create an error response with data
    pub fn error_with_data(id: Value, code: i32, message: String, data: Value) -> Self {
        Self::Error(JsonRpcError {
            jsonrpc: "2.0".to_string(),
            id,
            error: JsonRpcErrorContent {
                code,
                message,
                data: Some(data),
            },
        })
    }
}

impl ToolCallResult {
    /// Create a text result
    pub fn text(content: String) -> Self {
        Self {
            content: vec![ContentItem::Text { text: content }],
            isError: false,
        }
    }

    /// Create a JSON result
    pub fn json<T: Serialize>(val: &T) -> Self {
        let json = serde_json::to_string_pretty(val).unwrap_or_default();
        Self {
            content: vec![ContentItem::Text { text: json }],
            isError: false,
        }
    }

    /// Create an error result
    pub fn error(message: String) -> Self {
        Self {
            content: vec![ContentItem::Text { text: message }],
            isError: true,
        }
    }
}

impl ContentItem {
    /// Create a text content item
    pub fn text(text: String) -> Self {
        Self::Text { text }
    }

    /// Create an image content item
    pub fn image(data: String, mime_type: String) -> Self {
        Self::Image { data, mime_type }
    }

    /// Create a resource content item
    pub fn resource(resource: ResourceContent) -> Self {
        Self::Resource { resource }
    }
}

/// Error codes
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    pub const TOOL_NOT_FOUND: i32 = -32000;
    pub const TOOL_ERROR: i32 = -32001;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_parsing() {
        let json = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "list_tasks",
                "arguments": {"status": "pending"}
            }
        }"#;

        let request: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "tools/call");
        assert_eq!(request.tool_name().unwrap(), "list_tasks");
    }

    #[test]
    fn test_response_serialization() {
        let response =
            JsonRpcResponse::success(Value::Number(1.into()), serde_json::json!({"tasks": []}));

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("2.0"));
        assert!(json.contains("result"));
    }

    #[test]
    fn test_content_item() {
        let content = ContentItem::text("test content".to_string());

        if let ContentItem::Text { text } = content {
            assert!(text.contains("test"));
        }
    }
}
