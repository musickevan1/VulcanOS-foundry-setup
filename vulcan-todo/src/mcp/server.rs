//! MCP Server implementation
//!
//! Runs the MCP server in stdio mode for OpenCode integration.

use crate::mcp::protocol::{
    ContentItem, InitializeParams, InitializeResult, JsonRpcRequest, JsonRpcResponse,
    ListToolsResult, ServerCapabilities, ServerInfo, ToolCallResult, ToolDefinition,
    PROTOCOL_VERSION,
};
use crate::mcp::tools::{get_tools, ToolContext};
use crate::store::Store;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tracing::{debug, error, info};

// D-Bus imports (for future session detection)
// use zbus::{Connection, ConnectionBuilder};

/// MCP Server for vulcan-todo
pub struct McpServer {
    ctx: ToolContext,
}

/// Detect OpenCode session ID from environment variable or D-Bus
fn detect_session_id() -> Option<String> {
    // Try environment variable (from OpenCode)
    if let Ok(session_id) = std::env::var("OPENCODE_SESSION_ID") {
        info!("Detected session from OPENCODE_SESSION_ID: {}", session_id);
        return Some(session_id);
    }

    // FUTURE: D-Bus detection (system reminder added)
    // For now, return None which means "global"
    None
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(store: Arc<dyn Store>) -> Self {
        Self {
            ctx: ToolContext::new(store),
        }
    }

    /// Run the server in stdio mode
    pub async fn run_stdio(&mut self) -> Result<()> {
        use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt};

        let stdin_reader = BufReader::new(stdin());
        let mut stdout = stdout();
        let mut lines = stdin_reader.lines();

        info!("MCP Server started, waiting for requests...");

        while let Ok(Some(line)) = lines.next_line().await {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            // Parse request
            let parse_result: Result<JsonRpcRequest, _> = serde_json::from_str(&line);

            let req = match parse_result {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse JSON: {} - Request was: {}", e, line);
                    // Send immediate parse error response (JSON-RPC -32700)
                    let error_response =
                        JsonRpcResponse::error(Value::Null, -32700, format!("Parse error: {}", e));

                    let response_json = serde_json::to_string(&error_response)?;
                    let response_json = response_json + "\n";

                    if let Err(write_err) = stdout.write_all(response_json.as_bytes()).await {
                        error!("Failed to write error response: {}", write_err);
                        break;
                    }
                    if let Err(flush_err) = stdout.flush().await {
                        error!("Failed to flush error response: {}", flush_err);
                        break;
                    }
                    continue;
                }
            };

            // Detect OpenCode session ID
            let session_id = detect_session_id();
            debug!("Detected session: {:?}", session_id);

            // Check if this is a notification (no response needed)
            let is_notification = req.is_notification();

            // Handle request
            let response = self.handle_request(req).await;

            // Don't send response for notifications (per JSON-RPC spec)
            if !is_notification {
                // Send response
                let mut response_json = serde_json::to_string(&response)?;
                response_json.push('\n');

                if let Err(e) = stdout.write_all(response_json.as_bytes()).await {
                    error!("Failed to write response: {}", e);
                    break;
                }

                // Flush
                if let Err(e) = stdout.flush().await {
                    error!("Failed to flush output: {}", e);
                    break;
                }
            }
        }

        info!("MCP Server shutting down");
        Ok(())
    }

    /// Handle an MCP request
    async fn handle_request(&mut self, req: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling request: method={}", req.method);

        // Handle notifications (no response needed)
        if req.is_notification() {
            if req.method == "notifications/initialized" {
                info!("Client initialized");
            }
            return JsonRpcResponse::success(req.id, json!({}));
        }

        // Route to appropriate handler
        match req.method.as_str() {
            "initialize" => self.handle_initialize(req).await,
            "tools/list" => {
                debug!("Handling tools/list request");
                self.handle_tools_list(req).await
            }
            "notifications/listChanged" => {
                debug!("Handling notifications/listChanged");
                // Tools list changed notification - ignore
                JsonRpcResponse::success(req.id, json!({}))
            }
            _ if req.method.starts_with("tools/") => self.handle_tool_call(req).await,
            _ => {
                JsonRpcResponse::error(req.id, -32601, format!("Method not found: {}", req.method))
            }
        }
    }

    /// Handle initialize request
    async fn handle_initialize(&mut self, req: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling initialize request");

        // Detect OpenCode session ID
        let session_id = detect_session_id();
        debug!("Detected session: {:?}", session_id);

        // Store session_id in context for tool functions
        self.ctx.session_id = session_id;

        let tools = get_tools();
        let capabilities = ServerCapabilities {
            tools: Some(
                tools
                    .into_iter()
                    .map(|t| {
                        (
                            t.name.clone(),
                            ToolDefinition {
                                name: t.name,
                                description: t.description,
                                inputSchema: t.input_schema,
                                // Omit outputSchema - tools return text content, not structured data
                                outputSchema: None,
                            },
                        )
                    })
                    .collect(),
            ),
            resources: Some(HashMap::new()),
            prompts: Some(HashMap::new()),
        };

        let result = InitializeResult {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities,
            server_info: ServerInfo {
                name: "vulcan-todo".to_string(),
                version: Some("0.1.0".to_string()),
            },
        };

        JsonRpcResponse::success(req.id, serde_json::to_value(result).unwrap())
    }

    /// Handle tools/list request
    async fn handle_tools_list(&mut self, req: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling tools/list request with params: {:?}", req.params);
        let tools: Vec<ToolDefinition> = get_tools()
            .into_iter()
            .map(|t| ToolDefinition {
                name: t.name,
                description: t.description,
                inputSchema: t.input_schema,
                // Omit outputSchema - tools return text content, not structured data
                outputSchema: None,
            })
            .collect();
        let result = ListToolsResult { tools };

        JsonRpcResponse::success(req.id, serde_json::to_value(result).unwrap())
    }

    /// Handle tools/call request
    async fn handle_tool_call(&mut self, req: JsonRpcRequest) -> JsonRpcResponse {
        let tool_name = match req.tool_name() {
            Some(name) => name,
            None => {
                return JsonRpcResponse::error(
                    req.id,
                    -32600,
                    "Invalid request: missing tool name".to_string(),
                );
            }
        };

        // Arguments may be null for some tools
        let arguments = req
            .tool_arguments()
            .unwrap_or(Value::Object(serde_json::Map::new()));
        debug!("Executing tool: {} with args: {:?}", tool_name, arguments);

        // Find and execute tool
        let tools = get_tools();
        let tool = tools.iter().find(|t| t.name == tool_name);

        match tool {
            Some(tool) => {
                info!("Executing tool: {}", tool_name);
                let result = (tool.function)(&self.ctx, arguments);

                let content = vec![ContentItem::text(if result.success {
                    serde_json::to_string_pretty(&result.to_json())
                        .unwrap_or_else(|_| result.message.clone())
                } else {
                    result.message.clone()
                })];

                let tool_result = ToolCallResult {
                    content,
                    isError: !result.success,
                };

                JsonRpcResponse::success(req.id, serde_json::to_value(tool_result).unwrap())
            }
            None => {
                JsonRpcResponse::error(req.id, -32000, format!("Tool not found: {}", tool_name))
            }
        }
    }
}

/// Run MCP server
pub async fn run_mcp_server(store: Arc<dyn Store>) -> Result<()> {
    let mut server = McpServer::new(store);
    server.run_stdio().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemoryStore;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_server_initialize() {
        let store = Arc::new(MemoryStore::new()) as Arc<dyn Store>;
        let mut server = McpServer::new(store);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Value::Number(1.into()),
            method: "initialize".to_string(),
            params: serde_json::json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            }),
        };

        let response = server.handle_initialize(request).await;

        match response {
            JsonRpcResponse::Success(s) => {
                let result = &s.result;
                assert_eq!(result.get("protocolVersion").unwrap(), "2024-11-05");
            }
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_server_tools_list() {
        let store = Arc::new(MemoryStore::new()) as Arc<dyn Store>;
        let mut server = McpServer::new(store);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Value::Number(1.into()),
            method: "tools/list".to_string(),
            params: Value::Null,
        };

        let response = server.handle_request(request).await;

        match response {
            JsonRpcResponse::Success(s) => {
                let tools = s.result.get("tools").unwrap().as_array().unwrap();
                assert!(!tools.is_empty());
                assert!(tools.iter().any(|t| t.get("name").unwrap() == "list_tasks"));
            }
            _ => panic!("Expected success response"),
        }
    }
}
