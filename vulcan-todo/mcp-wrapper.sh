#!/bin/bash
# Wrapper script for vulcan-todo MCP server
# Provides better error handling and logging for OpenCode integration

LOG_FILE="$HOME/.config/vulcan-todo/mcp-wrapper.log"
mkdir -p "$(dirname "$LOG_FILE")"

# Enable more verbose logging
export RUST_LOG=debug

echo "[$(date)] MCP wrapper started" >> "$LOG_FILE"
echo "[$(date)] Args: $@" >> "$LOG_FILE"
echo "[$(date)] PID: $$" >> "$LOG_FILE"
echo "[$(date)] User: $(whoami)" >> "$LOG_FILE"

# Run the actual MCP server
# IMPORTANT: Only redirect stderr to log, stdout is used for MCP protocol!
exec /home/evan/VulcanOS/vulcan-todo/target/release/vulcan-todo --mcp "$@" 2>> "$LOG_FILE"
