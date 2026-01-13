//! MCP server implementation

use anyhow::Result;
use std::io::{BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{db_path, SqliteStore};

/// JSON-RPC request
#[derive(Debug, Deserialize)]
struct Request {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// JSON-RPC response
#[derive(Debug, Serialize)]
struct Response {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<RpcError>,
}

#[derive(Debug, Serialize)]
struct RpcError {
    code: i32,
    message: String,
}

/// Run the MCP server (reads from stdin, writes to stdout)
pub async fn run_server() -> Result<()> {
    let store = SqliteStore::new(db_path())?;

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    tracing::info!("vulcan-vault MCP server started");

    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => handle_request(&store, request).await,
            Err(e) => Response {
                jsonrpc: "2.0".to_string(),
                id: None,
                result: None,
                error: Some(RpcError {
                    code: -32700,
                    message: format!("Parse error: {}", e),
                }),
            },
        };

        let response_json = serde_json::to_string(&response)?;
        writeln!(stdout, "{}", response_json)?;
        stdout.flush()?;
    }

    Ok(())
}

async fn handle_request(store: &SqliteStore, request: Request) -> Response {
    let result = match request.method.as_str() {
        "initialize" => handle_initialize(),
        "tools/list" => handle_tools_list(),
        "tools/call" => handle_tool_call(store, request.params).await,
        _ => Err(anyhow::anyhow!("Method not found: {}", request.method)),
    };

    match result {
        Ok(value) => Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(value),
            error: None,
        },
        Err(e) => Response {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(RpcError {
                code: -32603,
                message: e.to_string(),
            }),
        },
    }
}

fn handle_initialize() -> Result<Value> {
    Ok(serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": "vulcan-vault",
            "version": env!("CARGO_PKG_VERSION")
        }
    }))
}

fn handle_tools_list() -> Result<Value> {
    Ok(serde_json::json!({
        "tools": super::tools::get_tool_definitions()
    }))
}

async fn handle_tool_call(store: &SqliteStore, params: Option<Value>) -> Result<Value> {
    let params = params.ok_or_else(|| anyhow::anyhow!("Missing params"))?;
    let name = params.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing tool name"))?;
    let arguments = params.get("arguments")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));

    super::tools::call_tool(store, name, arguments).await
}
