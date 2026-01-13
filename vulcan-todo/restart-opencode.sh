#!/bin/bash
# Restart OpenCode to pick up MCP configuration changes

echo "🔄 Restarting OpenCode for MCP updates..."
echo ""

# Kill all OpenCode processes
echo "1. Killing OpenCode processes..."
pkill -f "opencode" || true
sleep 2

# Verify processes are killed
REMAINING=$(ps aux | grep -i opencode | grep -v grep | wc -l)
if [ $REMAINING -gt 0 ]; then
    echo "   ⚠️  Still have $REMAINING OpenCode processes, forcing kill..."
    pkill -9 -f "opencode" || true
    sleep 1
fi

echo "   ✅ OpenCode processes stopped"
echo ""

# Verify MCP configuration
echo "2. Verifying MCP configuration..."
CONFIG_PATH="$HOME/.config/opencode/opencode.json"
if [ -f "$CONFIG_PATH" ]; then
    if grep -q 'vulcan-todo.*target/release.*vulcan-todo.*--mcp' "$CONFIG_PATH"; then
        echo "   ✅ MCP configuration looks correct"
    else
        echo "   ⚠️  MCP configuration may need update"
        echo "   Path: $CONFIG_PATH"
    fi
else
    echo "   ❌ Config file not found: $CONFIG_PATH"
fi
echo ""

# Verify binary exists
echo "3. Verifying vulcan-todo binary..."
BINARY_PATH="/home/evan/VulcanOS/vulcan-todo/target/release/vulcan-todo"
if [ -x "$BINARY_PATH" ]; then
    echo "   ✅ Binary exists and is executable"
    echo "   Path: $BINARY_PATH"
else
    echo "   ❌ Binary not found or not executable"
    echo "   Path: $BINARY_PATH"
fi
echo ""

# Test MCP server
echo "4. Testing MCP server..."
MCP_TEST=$(echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | "$BINARY_PATH" --mcp 2>/dev/null)
if echo "$MCP_TEST" | grep -q "vulcan-todo"; then
    TOOL_COUNT=$(echo "$MCP_TEST" | grep -o '"name":"[^"]*"' | wc -l)
    echo "   ✅ MCP server is working"
    echo "   Tools available: $TOOL_COUNT"
else
    echo "   ❌ MCP server test failed"
fi
echo ""

echo "========================================="
echo "✅ OpenCode ready to restart"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. Start OpenCode manually or run: opencode"
echo "2. Wait for OpenCode to fully load"
echo "3. Check MCP tools panel for:"
echo "   - vulcan-todo_list_tasks"
echo "   - vulcan-todo_list_projects"
echo "   - vulcan-todo_get_next_task"
echo "   - vulcan-todo_suggest_project"
echo "   (and 9 more tools...)"
echo ""
echo "💡 Tip: The MCP tools should appear after OpenCode loads"
