#!/bin/bash
# Test vulcan-todo MCP server locally

echo "Testing vulcan-todo MCP server..."
echo

# Test 1: Initialize
echo "1. Testing initialize..."
INIT_RESPONSE=$(echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | vulcan-todo --mcp 2>/dev/null)
if echo "$INIT_RESPONSE" | grep -q "vulcan-todo"; then
    echo "   ✓ Initialize successful"
else
    echo "   ✗ Initialize failed"
    echo "$INIT_RESPONSE"
    exit 1
fi

# Test 2: List tools
echo "2. Testing tools/list..."
TOOLS_RESPONSE=$(echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | vulcan-todo --mcp 2>/dev/null)
if echo "$TOOLS_RESPONSE" | grep -q "list_tasks"; then
    echo "   ✓ Tools list successful"
else
    echo "   ✗ Tools list failed"
    exit 1
fi

# Test 3: Check outputSchema
echo "3. Verifying outputSchema is null..."
if echo "$TOOLS_RESPONSE" | grep -q '"outputSchema":null'; then
    echo "   ✓ outputSchema is null (correct)"
else
    echo "   ✗ outputSchema is not null"
    exit 1
fi

# Test 4: Call a tool
echo "4. Testing tool call (get_stats)..."
STATS_RESPONSE=$(echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_stats","arguments":{}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$STATS_RESPONSE" | grep -q "isError.*false"; then
    echo "   ✓ Tool call successful"
else
    echo "   ✗ Tool call failed"
    echo "$STATS_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 5: Create task with project
echo "5. Testing create_task with project..."
CREATE_RESPONSE=$(echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"create_task","arguments":{"title":"Test project task","priority":"high","project":"vulcan-os","tags":["test","mcp"]}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$CREATE_RESPONSE" | grep -q "project.*vulcan-os"; then
    echo "   ✓ Create task with project successful"
else
    echo "   ✗ Create task with project failed"
    echo "$CREATE_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 6: List projects
echo "6. Testing list_projects..."
PROJECTS_RESPONSE=$(echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"list_projects","arguments":{}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$PROJECTS_RESPONSE" | grep -q "vulcan-os"; then
    echo "   ✓ List projects successful"
else
    echo "   ✗ List projects failed"
    echo "$PROJECTS_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 7: Get project tasks
echo "7. Testing get_project..."
PROJECT_RESPONSE=$(echo '{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"get_project","arguments":{"name":"vulcan-os"}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$PROJECT_RESPONSE" | grep -q "project.*vulcan-os"; then
    echo "   ✓ Get project successful"
else
    echo "   ✗ Get project failed"
    echo "$PROJECT_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 8: Test get_next_task (agent tool)
echo "8. Testing get_next_task..."
NEXT_TASK_RESPONSE=$(echo '{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"get_next_task","arguments":{"project":"vulcan-os"}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$NEXT_TASK_RESPONSE" | grep -q "task"; then
    echo "   ✓ Get next task successful"
else
    echo "   ✗ Get next task failed"
    echo "$NEXT_TASK_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 9: Test suggest_project (agent tool)
echo "9. Testing suggest_project..."
SUGGEST_RESPONSE=$(echo '{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"suggest_project","arguments":{"title":"Implement new feature for vulcan-os"}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$SUGGEST_RESPONSE" | grep -q "suggestions"; then
    echo "   ✓ Suggest project successful"
else
    echo "   ✗ Suggest project failed"
    echo "$SUGGEST_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

# Test 10: Test migrate_projects
echo "10. Testing migrate_projects..."
MIGRATE_RESPONSE=$(echo '{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"migrate_projects","arguments":{}}}' | vulcan-todo --mcp 2>/dev/null)
if echo "$MIGRATE_RESPONSE" | grep -q "success"; then
    echo "   ✓ Migrate projects successful"
else
    echo "   ✗ Migrate projects failed"
    echo "$MIGRATE_RESPONSE" | jq '.' 2>/dev/null
    exit 1
fi

echo
echo "========================================="
echo "✅ All MCP server tests passed!"
echo "========================================="
echo
echo "Next steps:"
echo "1. Restart OpenCode (required for MCP changes)"
echo "2. Try: vulcan-todo_get_stats"
echo "3. Try: vulcan-todo_list_projects"
echo "4. Try: vulcan-todo_get_next_task (agent workflow)"
echo "5. Try: vulcan-todo_suggest_project (for task creation)"
