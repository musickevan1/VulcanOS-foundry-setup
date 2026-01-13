#!/bin/bash
# Force restart OpenCode and verify MCP configuration

set -euo pipefail

echo "========================================="
echo "🔄 OpenCode Force Restart Script"
echo "========================================="
echo ""

# Step 1: Kill all OpenCode processes
echo "1️⃣  Killing all OpenCode processes..."
pkill -9 -f "opencode" 2>/dev/null || true
pkill -9 -f "opencode-ai" 2>/dev/null || true

# Wait for processes to terminate
sleep 2

# Verify processes are killed
REMAINING=$(pgrep -f "opencode" 2>/dev/null | wc -l)
if [ $REMAINING -gt 0 ]; then
    echo "   ⚠️  Warning: $REMAINING OpenCode processes still running"
    pkill -9 -9 opencode 2>/dev/null || true
    sleep 1
else
    echo "   ✅ All OpenCode processes killed"
fi
echo ""

# Step 2: Clear OpenCode cache if it exists
echo "2️⃣  Clearing OpenCode cache..."
CACHE_DIR="$HOME/.cache/opencode"
if [ -d "$CACHE_DIR" ]; then
    rm -rf "$CACHE_DIR" 2>/dev/null || true
    echo "   ✅ Cache cleared: $CACHE_DIR"
else
    echo "   ℹ️  No cache directory found"
fi
echo ""

# Step 3: Verify configuration
echo "3️⃣  Verifying MCP configuration..."
CONFIG_PATH="$HOME/.config/opencode/opencode.json"
if [ -f "$CONFIG_PATH" ]; then
    if grep -q 'mcp-wrapper.sh' "$CONFIG_PATH"; then
        echo "   ✅ Using mcp-wrapper.sh for vulcan-todo"
    elif grep -q 'vulcan-todo.*--mcp' "$CONFIG_PATH"; then
        echo "   ✅ Using direct vulcan-todo --mcp"
    else
        echo "   ⚠️  MCP configuration not found"
    fi
else
    echo "   ❌ Config file not found: $CONFIG_PATH"
    exit 1
fi
echo ""

# Step 4: Verify binary
echo "4️⃣  Verifying vulcan-todo binary..."
BINARY="/home/evan/VulcanOS/vulcan-todo/target/release/vulcan-todo"
WRAPPER="/home/evan/VulcanOS/vulcan-todo/mcp-wrapper.sh"

if [ -x "$BINARY" ]; then
    echo "   ✅ Binary exists: $BINARY"
else
    echo "   ❌ Binary not found or not executable: $BINARY"
    exit 1
fi

if [ -x "$WRAPPER" ]; then
    echo "   ✅ Wrapper exists: $WRAPPER"
else
    echo "   ❌ Wrapper not found: $WRAPPER"
    exit 1
fi
echo ""

# Step 5: Test MCP server
echo "5️⃣  Testing MCP server..."
MCP_TEST=$(echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | timeout 3 "$BINARY" --mcp 2>/dev/null)
if echo "$MCP_TEST" | grep -q "vulcan-todo"; then
    TOOL_COUNT=$(echo "$MCP_TEST" | grep -o '"name":"[^"]*"' | wc -l)
    echo "   ✅ MCP server is working"
    echo "   📊 Tools available: $TOOL_COUNT"
else
    echo "   ❌ MCP server test failed"
    exit 1
fi
echo ""

# Step 6: Instructions
echo "========================================="
echo "✅ All checks passed!"
echo "========================================="
echo ""
echo "📝 Next steps:"
echo ""
echo "1. Start OpenCode with one of these methods:"
echo ""
echo "   Option A - Terminal:"
echo "      opencode"
echo ""
echo "   Option B - From application menu"
echo "      (Search for 'OpenCode' in your app launcher)"
echo ""
echo "2. Wait for OpenCode to fully load (about 5-10 seconds)"
echo ""
echo "3. Check MCP tools panel for:"
echo "   • vulcan-todo_list_tasks"
echo "   • vulcan-todo_list_projects"
echo "   • vulcan-todo_get_next_task"
echo "   • vulcan-todo_suggest_project"
echo "   • (and 11 more tools...)"
echo ""
echo "4. If tools still don't appear:"
echo "   • Try: File → Restart (or Ctrl+Shift+P → 'Reload Window')"
echo "   • Check logs: cat ~/.config/vulcan-todo/mcp-wrapper.log"
echo ""
echo "💡 Tip: The wrapper script logs to:"
echo "   ~/.config/vulcan-todo/mcp-wrapper.log"
echo ""
