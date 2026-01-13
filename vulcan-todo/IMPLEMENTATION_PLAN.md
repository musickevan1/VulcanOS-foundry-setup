# Vulcan-Todo: Complete Implementation Plan
## MCP Server + CLI + TUI Recovery and Enhancement

**Status:** Planning  
**Priority:** High  
**Estimated Effort:** 4-6 hours  
**Target:** Fully functional MCP server, CLI, and lightweight TUI

---

## Executive Summary

The vulcan-todo project has multiple critical issues preventing it from functioning:
1. **Compilation failures** - Rust code has syntax errors preventing build
2. **Invalid JSON configs** - OpenCode configuration has malformed JSON
3. **Missing MCP registration** - User config doesn't register the MCP server
4. **No working binary** - Cannot test or use the tool

This plan provides a systematic approach to fix all issues, verify functionality, and ensure robust operation.

---

## Phase 1: Code Audit and Repair

### 1.1 Complete Codebase Audit

**Objective:** Identify all compilation errors and logical issues

**Tasks:**
- [ ] Read and analyze ALL source files in `src/`:
  - `src/main.rs` - Entry point and CLI handler
  - `src/cli.rs` - CLI argument definitions
  - `src/models/mod.rs` and `src/models/task.rs` - Task data model
  - `src/store/mod.rs` and `src/store/json_store.rs` - Data persistence
  - `src/mcp/mod.rs`, `src/mcp/server.rs`, `src/mcp/protocol.rs`, `src/mcp/tools.rs` - MCP implementation
  - `src/ui/mod.rs`, `src/ui/app.rs`, `src/ui/tui.rs` - TUI implementation

**Expected Issues:**
- Duplicate match arms in main.rs (already identified)
- Missing method implementations (reopen/uncomplete)
- Potential async/await issues in MCP server
- TUI feature flag dependencies

**Deliverables:**
- List of all compilation errors
- List of logical errors (unreachable code, dead code)
- List of missing implementations

### 1.2 Fix main.rs Critical Errors

**File:** `vulcan-todo/src/main.rs`

**Issues Identified:**
1. Line 66: Stray closing brace and incomplete code fragment
2. Lines 209-253: Four duplicate `cli::Commands::Done` match arms
3. Unmatched braces causing parse failures

**Actions:**

**Step 1:** Remove invalid code block (lines 65-67)
```rust
// DELETE THESE LINES:
        cli::Commands::Undone { id } => {
}
```

**Step 2:** Remove duplicate Done handlers (keep only lines 209-223, delete 225-253)

**Step 3:** Verify the Done handler implementation:
```rust
cli::Commands::Done { id } => {
    let task = store.get(&id)?;
    match task {
        Some(mut t) => {
            t.complete();
            store.update(&t)?;
            println!("Task completed: {}", t.title);
            Ok(())
        }
        None => {
            eprintln!("Task not found: {}", id);
            Ok(())
        }
    }
}
```

**Step 4:** Add Undone handler if cli.rs defines it (check first):
```rust
cli::Commands::Undone { id } => {
    let task = store.get(&id)?;
    match task {
        Some(mut t) => {
            t.reopen();
            store.update(&t)?;
            println!("Task reopened: {}", t.title);
            Ok(())
        }
        None => {
            eprintln!("Task not found: {}", id);
            Ok(())
        }
    }
}
```

**Verification:**
```bash
cd /home/evan/VulcanOS/vulcan-todo
cargo check
```

Expected: No compilation errors

### 1.3 Verify CLI Definition

**File:** `vulcan-todo/src/cli.rs`

**Tasks:**
- [ ] Read cli.rs to understand all defined commands
- [ ] Verify each command in cli.rs has a handler in main.rs
- [ ] Check if `Undone` command exists
- [ ] Verify argument types match usage

**Common Issues to Check:**
- Missing commands in match statement
- Type mismatches (Option<String> vs String)
- Missing required arguments

### 1.4 Verify Task Model

**File:** `vulcan-todo/src/models/task.rs`

**Tasks:**
- [ ] Verify `Task::complete()` method exists and is correct
- [ ] Check if `Task::reopen()` or `uncomplete()` method exists
- [ ] If missing, implement reopen method:
```rust
pub fn reopen(&mut self) {
    self.status = Status::Pending;
    self.completed_at = None;
}
```

**Verification:**
- Ensure status transitions are valid
- Check that completed_at is properly set/cleared

### 1.5 Verify Store Trait Implementation

**File:** `vulcan-todo/src/store/json_store.rs`

**Tasks:**
- [ ] Verify all Store trait methods are implemented
- [ ] Check file locking implementation (fs4 usage)
- [ ] Test concurrent access safety
- [ ] Verify error handling

**Common Issues:**
- Missing error handling for file I/O
- Race conditions in read-modify-write cycles
- Improper lock release

---

## Phase 2: MCP Server Implementation

### 2.1 Review MCP Protocol Implementation

**File:** `vulcan-todo/src/mcp/protocol.rs`

**Tasks:**
- [ ] Verify protocol version matches spec (2024-11-05)
- [ ] Check JSON-RPC 2.0 compliance
- [ ] Verify all message types are correctly defined
- [ ] Test serialization/deserialization

**Critical Points:**
- ContentItem enum must use correct serde tags
- ToolDefinition must match MCP spec
- Error codes must be standard JSON-RPC codes

### 2.2 Review MCP Server Logic

**File:** `vulcan-todo/src/mcp/server.rs`

**Tasks:**
- [ ] Verify stdio reading/writing logic
- [ ] Check request routing (initialize, tools/list, tools/call)
- [ ] Verify error handling for malformed requests
- [ ] Check async/await usage

**Known Issues to Address:**
- Line 40: Using non-existent `stdin.read_line()` method
  - Should use `BufReader::lines()` or similar
- Ensure proper flushing of stdout
- Handle EOF gracefully

**Correct Implementation Pattern:**
```rust
pub async fn run_stdio(&mut self) -> Result<()> {
    use tokio::io::{stdin, stdout, AsyncBufReadExt, AsyncWriteExt, BufReader};
    
    let stdin = stdin();
    let mut stdout = stdout();
    let mut lines = BufReader::new(stdin).lines();
    
    info!("MCP Server started, waiting for requests...");
    
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        
        if line.is_empty() {
            continue;
        }
        
        // ... rest of processing
    }
    
    Ok(())
}
```

### 2.3 Review MCP Tools Implementation

**File:** `vulcan-todo/src/mcp/tools.rs`

**Tasks:**
- [ ] Read complete tools.rs (currently only read first 100 lines)
- [ ] Verify all 9 tools are implemented:
  1. list_tasks
  2. get_task
  3. create_task
  4. update_task
  5. complete_task
  6. uncomplete_task
  7. delete_task
  8. search_tasks
  9. get_stats

**For Each Tool, Verify:**
- [ ] JSON schema is valid and matches tool behavior
- [ ] Error handling is comprehensive
- [ ] Return values match expected format
- [ ] Store operations are correctly called

**Tool Implementation Checklist:**

```markdown
| Tool | Schema | Handler | Error Handling | Return Format |
|------|--------|---------|----------------|---------------|
| list_tasks | [ ] | [ ] | [ ] | [ ] |
| get_task | [ ] | [ ] | [ ] | [ ] |
| create_task | [ ] | [ ] | [ ] | [ ] |
| update_task | [ ] | [ ] | [ ] | [ ] |
| complete_task | [ ] | [ ] | [ ] | [ ] |
| uncomplete_task | [ ] | [ ] | [ ] | [ ] |
| delete_task | [ ] | [ ] | [ ] | [ ] |
| search_tasks | [ ] | [ ] | [ ] | [ ] |
| get_stats | [ ] | [ ] | [ ] | [ ] |
```

### 2.4 Test MCP Server Standalone

**Tasks:**
- [ ] Create test script: `vulcan-todo/test-mcp.sh`
- [ ] Test initialize handshake
- [ ] Test tools/list request
- [ ] Test each tool with valid inputs
- [ ] Test error handling with invalid inputs

**Test Script Template:**
```bash
#!/bin/bash
# test-mcp.sh - Test vulcan-todo MCP server

set -e

BINARY="./target/release/vulcan-todo"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found. Run 'cargo build --release' first"
    exit 1
fi

echo "Testing MCP Server..."

# Test 1: Initialize
echo "Test 1: Initialize"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | $BINARY --mcp | tee /tmp/mcp-test-1.json

# Test 2: List tools
echo -e "\nTest 2: List Tools"
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | $BINARY --mcp | tee /tmp/mcp-test-2.json

# Test 3: List tasks
echo -e "\nTest 3: List Tasks"
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list_tasks","arguments":{}}}' | $BINARY --mcp | tee /tmp/mcp-test-3.json

# Test 4: Create task
echo -e "\nTest 4: Create Task"
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"create_task","arguments":{"title":"Test task","priority":"high"}}}' | $BINARY --mcp | tee /tmp/mcp-test-4.json

# Test 5: Get stats
echo -e "\nTest 5: Get Stats"
echo '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"get_stats","arguments":{}}}' | $BINARY --mcp | tee /tmp/mcp-test-5.json

echo -e "\nAll tests completed. Check /tmp/mcp-test-*.json for results"
```

**Verification:**
- All responses should be valid JSON
- All responses should have `"jsonrpc":"2.0"`
- Success responses should have `"result"` key
- Error responses should have `"error"` key with proper code

---

## Phase 3: Configuration Fixes

### 3.1 Fix Dotfiles OpenCode Config

**File:** `dotfiles/opencode/.config/opencode/opencode.json`

**Issue:** Lines 41-55 have malformed JSON

**Current (BROKEN):**
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  },
  "vulcan-todo": {
    "type": "local",
    "command": ["vulcan-todo", "--mcp"]
  }
},
  "vulcan-todo": {
    "type": "local",
    "command": ["vulcan-todo", "--mcp"]
  }
},
```

**Fixed:**
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  },
  "vulcan-todo": {
    "type": "local",
    "command": ["vulcan-todo", "--mcp"]
  }
},
```

**Actions:**
- [ ] Remove duplicate "vulcan-todo" entry (lines 51-54)
- [ ] Remove extra closing brace (line 55)
- [ ] Validate JSON with `jq`

**Validation:**
```bash
jq empty dotfiles/opencode/.config/opencode/opencode.json
```

Expected: No errors

### 3.2 Update User OpenCode Config

**File:** `~/.config/opencode/opencode.json`

**Issue:** Missing vulcan-todo MCP server registration

**Action:** Add vulcan-todo to mcp section

**Before:**
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  }
},
```

**After:**
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  },
  "vulcan-todo": {
    "type": "local",
    "command": ["vulcan-todo", "--mcp"]
  }
},
```

**Validation:**
```bash
jq empty ~/.config/opencode/opencode.json
jq '.mcp."vulcan-todo"' ~/.config/opencode/opencode.json
```

Expected: Shows vulcan-todo configuration

### 3.3 Sync Dotfiles to ISO (Optional)

**Context:** VulcanOS includes configs in archiso for fresh installs

**File:** `archiso/airootfs/etc/skel/.config/opencode/opencode.json`

**Action:** Ensure ISO skeleton has corrected config

**Verification:**
```bash
diff dotfiles/opencode/.config/opencode/opencode.json \
     archiso/airootfs/etc/skel/.config/opencode/opencode.json
```

---

## Phase 4: Build and Installation

### 4.1 Clean Build

**Tasks:**
- [ ] Clean previous build artifacts
- [ ] Build with all features
- [ ] Verify binary creation
- [ ] Test binary execution

**Commands:**
```bash
cd /home/evan/VulcanOS/vulcan-todo

# Clean
cargo clean

# Build with all features
cargo build --release --features tui

# Verify binary exists
ls -lh target/release/vulcan-todo

# Test binary
./target/release/vulcan-todo --help
```

**Expected Output:**
- Binary at `target/release/vulcan-todo` (~5-10 MB)
- Help text shows all subcommands
- No errors or panics

### 4.2 Install Binary

**Options:**

**Option A: Local install (recommended for development)**
```bash
cargo install --path . --features tui
```

**Option B: System install**
```bash
sudo cp target/release/vulcan-todo /usr/local/bin/
sudo chmod +x /usr/local/bin/vulcan-todo
```

**Verification:**
```bash
which vulcan-todo
vulcan-todo --version
```

### 4.3 Test CLI Functionality

**Test all CLI commands:**

```bash
# List tasks
vulcan-todo list

# Add a task
vulcan-todo add "Test task" --priority high --tags test,demo

# List again to see new task
vulcan-todo list

# Show task details (replace <id> with actual ID)
vulcan-todo show <id>

# Complete task
vulcan-todo done <id>

# Search tasks
vulcan-todo search "test"

# Show statistics
vulcan-todo stats

# Delete task
vulcan-todo delete <id>
```

**Expected:** All commands work without errors

### 4.4 Test TUI

**Command:**
```bash
vulcan-todo
# or
vulcan-todo tui
```

**Test Checklist:**
- [ ] TUI launches without errors
- [ ] Can navigate with j/k or arrow keys
- [ ] Can create new task (n key)
- [ ] Can toggle completion (x or space)
- [ ] Can delete task (d key)
- [ ] Can quit (q or Esc)
- [ ] UI renders correctly (no visual artifacts)

**Visual Verification:**
- Tasks display with correct status icons
- Priority indicators show (emoji or color)
- Help text displays on ? key
- Status bar shows correct info

---

## Phase 5: MCP Integration Testing

### 5.1 Test MCP Server Manually

**Commands:**
```bash
# Start MCP server
vulcan-todo --mcp

# In another terminal, send test requests
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | vulcan-todo --mcp
```

**Expected:**
- Server starts without errors
- Responds to initialize with server info
- Responds to tools/list with tool definitions

### 5.2 Test with OpenCode

**Prerequisites:**
- [ ] Config updated (Phase 3.2)
- [ ] Binary installed (Phase 4.2)

**Steps:**
1. Restart OpenCode (if running)
2. Check MCP server status
3. Test tool invocation

**Test in OpenCode:**
```
User: "List all my pending tasks using vulcan-todo"
```

Expected: OpenCode calls list_tasks tool and displays results

**Additional Tests:**
```
User: "Create a high priority task: Review PR #123"
User: "Mark task <id> as complete"
User: "Show me task statistics"
User: "Search for tasks containing 'review'"
```

### 5.3 Debug MCP Issues

**If MCP server doesn't appear:**

**Step 1: Check OpenCode logs**
```bash
# OpenCode logs location (varies)
tail -f ~/.local/state/opencode/logs/mcp.log
# or
journalctl --user -u opencode -f
```

**Step 2: Check binary path**
```bash
which vulcan-todo
# Should return /home/evan/.cargo/bin/vulcan-todo or /usr/local/bin/vulcan-todo
```

**Step 3: Test manual invocation**
```bash
vulcan-todo --mcp
# Should wait for input (stdio mode)
# Press Ctrl+D to exit
```

**Step 4: Check JSON config syntax**
```bash
jq empty ~/.config/opencode/opencode.json
```

**Step 5: Check permissions**
```bash
ls -l $(which vulcan-todo)
# Should be executable
```

### 5.4 Logging and Debugging

**Enable verbose logging:**

**Environment Variable:**
```bash
RUST_LOG=debug vulcan-todo --mcp
```

**In code (already configured):**
```rust
// In main.rs
tracing_subscriber::fmt::init();
```

**Log Locations:**
- MCP server logs to stderr
- OpenCode captures and logs MCP server output
- Check `~/.config/vulcan-todo/logs/` if configured

---

## Phase 6: Testing and Quality Assurance

### 6.1 Unit Tests

**Run existing tests:**
```bash
cd /home/evan/VulcanOS/vulcan-todo
cargo test
```

**Expected:** All tests pass

**If tests fail:**
- Review test output
- Fix failing tests
- Re-run until all pass

### 6.2 Integration Tests

**Create integration test script: `test-integration.sh`**

```bash
#!/bin/bash
# Integration test for vulcan-todo

set -e

echo "=== Vulcan-Todo Integration Tests ==="

# Setup
export TEST_TODO_PATH=/tmp/vulcan-todo-test-$$.json
trap "rm -f $TEST_TODO_PATH" EXIT

# Test 1: CLI - Add task
echo "Test 1: Add task via CLI"
TASK_OUTPUT=$(vulcan-todo --path $TEST_TODO_PATH add "Integration test task" --priority high)
echo "$TASK_OUTPUT"
TASK_ID=$(echo "$TASK_OUTPUT" | grep "Task created:" | awk '{print $3}')
echo "Created task: $TASK_ID"

# Test 2: CLI - List tasks
echo -e "\nTest 2: List tasks"
vulcan-todo --path $TEST_TODO_PATH list

# Test 3: CLI - Complete task
echo -e "\nTest 3: Complete task"
vulcan-todo --path $TEST_TODO_PATH done "$TASK_ID"

# Test 4: CLI - Show stats
echo -e "\nTest 4: Show stats"
vulcan-todo --path $TEST_TODO_PATH stats

# Test 5: MCP - List tasks
echo -e "\nTest 5: MCP list_tasks"
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_tasks","arguments":{"status":"done"}}}' | \
  VULCAN_TODO_PATH=$TEST_TODO_PATH vulcan-todo --mcp

echo -e "\n=== All Integration Tests Passed ==="
```

**Run:**
```bash
chmod +x test-integration.sh
./test-integration.sh
```

### 6.3 Performance Testing

**Test with large task lists:**

```bash
# Create 1000 tasks
for i in {1..1000}; do
  vulcan-todo add "Task $i" --priority low
done

# Test list performance
time vulcan-todo list

# Test search performance
time vulcan-todo search "Task 5"

# Test MCP performance
time echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_tasks","arguments":{}}}' | vulcan-todo --mcp
```

**Expected:**
- List: < 100ms for 1000 tasks
- Search: < 50ms for 1000 tasks
- MCP call: < 200ms for 1000 tasks

### 6.4 Concurrent Access Testing

**Test file locking:**

```bash
# Terminal 1
vulcan-todo --mcp

# Terminal 2
vulcan-todo add "Test concurrent access"

# Terminal 3
vulcan-todo list
```

**Expected:** No data corruption, proper locking

---

## Phase 7: Documentation

### 7.1 Update README.md

**Sections to verify/update:**
- [ ] Installation instructions
- [ ] Usage examples
- [ ] MCP integration guide
- [ ] Keybindings reference
- [ ] Troubleshooting section

**Add Troubleshooting Section:**
```markdown
## Troubleshooting

### MCP Server Not Showing in OpenCode

1. Verify binary is installed:
   ```bash
   which vulcan-todo
   ```

2. Test MCP server manually:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | vulcan-todo --mcp
   ```

3. Check OpenCode config:
   ```bash
   jq '.mcp."vulcan-todo"' ~/.config/opencode/opencode.json
   ```

4. Check logs:
   ```bash
   RUST_LOG=debug vulcan-todo --mcp
   ```

### Tasks Not Persisting

1. Check file location:
   ```bash
   ls -l ~/.config/vulcan-todo/tasks.json
   ```

2. Check permissions:
   ```bash
   chmod 644 ~/.config/vulcan-todo/tasks.json
   ```

3. Verify JSON format:
   ```bash
   jq empty ~/.config/vulcan-todo/tasks.json
   ```
```

### 7.2 Create MCP_INTEGRATION.md

**New file:** `vulcan-todo/docs/MCP_INTEGRATION.md`

**Contents:**
```markdown
# MCP Integration Guide

## Overview

Vulcan-todo implements the Model Context Protocol (MCP) to enable AI agents to manage tasks autonomously.

## Setup

### 1. Install vulcan-todo

```bash
cargo install --path . --features tui
```

### 2. Configure OpenCode

Add to `~/.config/opencode/opencode.json`:

```json
{
  "mcp": {
    "vulcan-todo": {
      "type": "local",
      "command": ["vulcan-todo", "--mcp"]
    }
  }
}
```

### 3. Restart OpenCode

OpenCode will automatically start the MCP server when needed.

## Available Tools

### list_tasks

List all tasks with optional filtering.

**Arguments:**
- `status` (optional): "pending", "done", or "all"
- `priority` (optional): "none", "low", "medium", "high", "urgent"
- `search` (optional): Search query
- `limit` (optional): Maximum number of results (default: 50)

**Example:**
```json
{
  "name": "list_tasks",
  "arguments": {
    "status": "pending",
    "priority": "high"
  }
}
```

### create_task

Create a new task.

**Arguments:**
- `title` (required): Task title
- `description` (optional): Task description
- `priority` (optional): "none", "low", "medium", "high", "urgent"
- `tags` (optional): Array of tags

**Example:**
```json
{
  "name": "create_task",
  "arguments": {
    "title": "Review PR #123",
    "priority": "high",
    "tags": ["code-review", "urgent"]
  }
}
```

### complete_task

Mark a task as complete.

**Arguments:**
- `id` (required): Task ID

**Example:**
```json
{
  "name": "complete_task",
  "arguments": {
    "id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

### get_stats

Get task statistics.

**Example:**
```json
{
  "name": "get_stats",
  "arguments": {}
}
```

## Testing

Test the MCP server manually:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | vulcan-todo --mcp
```

## Debugging

Enable debug logging:

```bash
RUST_LOG=debug vulcan-todo --mcp
```

## Protocol Version

Vulcan-todo implements MCP protocol version `2024-11-05`.
```

### 7.3 Create ARCHITECTURE.md

**New file:** `vulcan-todo/docs/ARCHITECTURE.md`

**Contents:**
```markdown
# Architecture

## Overview

Vulcan-todo is a multi-mode task manager with three interfaces:
1. Command-line interface (CLI)
2. Terminal user interface (TUI)
3. Model Context Protocol (MCP) server

## Module Structure

```
vulcan-todo/
├── src/
│   ├── main.rs           # Entry point, mode selection
│   ├── cli.rs            # CLI argument definitions
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   └── task.rs       # Task struct, Status, Priority
│   ├── store/            # Data persistence
│   │   ├── mod.rs        # Store trait
│   │   └── json_store.rs # JSON file-based storage
│   ├── mcp/              # MCP server implementation
│   │   ├── mod.rs
│   │   ├── server.rs     # MCP server logic
│   │   ├── protocol.rs   # JSON-RPC protocol types
│   │   └── tools.rs      # Tool definitions and handlers
│   └── ui/               # TUI implementation (optional)
│       ├── mod.rs
│       ├── app.rs        # Application state
│       └── tui.rs        # UI rendering
```

## Data Flow

### CLI Mode
```
User → CLI Args → main.rs → handle_command() → Store → JSON File
```

### TUI Mode
```
User → Keyboard → Event Loop → App State → Ratatui → Terminal
                                    ↓
                                  Store → JSON File
```

### MCP Mode
```
OpenCode → stdio → MCP Server → Tool Handler → Store → JSON File
                      ↓
                   JSON-RPC Response → stdio → OpenCode
```

## Storage

### File Location
- Default: `~/.config/vulcan-todo/tasks.json`
- Override: `VULCAN_TODO_PATH` environment variable or `--path` CLI flag

### File Format
```json
[
  {
    "id": "uuid",
    "title": "Task title",
    "description": "Optional description",
    "status": "Pending",
    "priority": "High",
    "tags": ["tag1", "tag2"],
    "created_at": "2024-01-05T12:00:00Z",
    "completed_at": null
  }
]
```

### Concurrency
- Uses `fs4` crate for file locking
- Exclusive lock for writes
- Shared lock for reads
- Prevents concurrent modification

## MCP Protocol

### Message Flow

1. **Initialize**
   ```
   Client → initialize request → Server
   Server → capabilities response → Client
   ```

2. **List Tools**
   ```
   Client → tools/list → Server
   Server → tool definitions → Client
   ```

3. **Call Tool**
   ```
   Client → tools/call → Server
   Server → execute tool → Store
   Store → return result → Server
   Server → tool result → Client
   ```

### Tool Execution

Each tool:
1. Receives JSON arguments
2. Validates input against schema
3. Executes operation via Store trait
4. Returns ToolResult with success/error status
5. Result serialized to JSON-RPC response

## Error Handling

- CLI: Print to stderr, exit with non-zero code
- TUI: Display error in status bar, continue running
- MCP: Return JSON-RPC error response with error code

## Testing Strategy

- Unit tests: Each module independently
- Integration tests: End-to-end CLI workflows
- MCP tests: Protocol compliance, tool execution
```

---

## Phase 8: Deployment and Distribution

### 8.1 Add to VulcanOS ISO

**Update package list:**

**File:** `archiso/packages.x86_64`

**Add:**
```
vulcan-todo
```

**Or build locally and include in customrepo:**

```bash
cd /home/evan/VulcanOS/vulcan-todo
cargo build --release --features tui

# Copy to custom repo
mkdir -p /home/evan/VulcanOS/customrepo/x86_64
cp target/release/vulcan-todo /home/evan/VulcanOS/customrepo/x86_64/

# Create package (if using pacman)
# ... create PKGBUILD ...
```

### 8.2 Hyprland Keybinding

**File:** `dotfiles/hypr/.config/hypr/bindings.conf`

**Verify keybinding exists:**
```
bind = $mainMod, T, exec, kitty -e vulcan-todo
```

**If missing, add it**

### 8.3 Distribution

**Options:**

**Option A: Cargo install (for developers)**
```bash
cargo install --git https://github.com/VulcanOS/vulcan-todo --features tui
```

**Option B: Binary release**
```bash
# Build optimized binary
cargo build --release --features tui

# Strip symbols for smaller size
strip target/release/vulcan-todo

# Create tarball
tar -czf vulcan-todo-v0.1.0-x86_64.tar.gz -C target/release vulcan-todo
```

**Option C: AUR package**
- Create PKGBUILD
- Submit to AUR
- Users install with `yay -S vulcan-todo`

---

## Phase 9: Continuous Improvement

### 9.1 Add Pre-commit Hooks

**File:** `.git/hooks/pre-commit`

```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Check Rust code
echo "Checking Rust code..."
cargo check
cargo clippy -- -D warnings

# Format check
echo "Checking formatting..."
cargo fmt -- --check

# Validate JSON configs
echo "Validating JSON configs..."
for file in $(find . -name "*.json" -not -path "./target/*"); do
  echo "  Checking $file"
  jq empty "$file" || exit 1
done

echo "All checks passed!"
```

**Make executable:**
```bash
chmod +x .git/hooks/pre-commit
```

### 9.2 Add GitHub Actions CI

**File:** `.github/workflows/ci.yml`

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Test
        run: cargo test --all-features
      
      - name: Build
        run: cargo build --release --all-features
```

### 9.3 Add Issue Templates

**File:** `.github/ISSUE_TEMPLATE/bug_report.md`
**File:** `.github/ISSUE_TEMPLATE/feature_request.md`

### 9.4 Performance Monitoring

**Add benchmarks:**

**File:** `benches/store_bench.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vulcan_todo::store::{JsonStore, Store};
use vulcan_todo::Task;

fn bench_add_task(c: &mut Criterion) {
    c.bench_function("add task", |b| {
        let store = JsonStore::new().unwrap();
        let task = Task::new("Benchmark task".to_string());
        b.iter(|| {
            store.add(black_box(&task)).unwrap();
        });
    });
}

criterion_group!(benches, bench_add_task);
criterion_main!(benches);
```

---

## Success Criteria

### Must Have (Blocking)
- [ ] Code compiles without errors or warnings
- [ ] All CLI commands work correctly
- [ ] MCP server responds to all protocol methods
- [ ] All 9 MCP tools function correctly
- [ ] OpenCode can discover and use the MCP server
- [ ] JSON configs are valid and synced
- [ ] Tests pass
- [ ] Documentation is complete

### Should Have (Important)
- [ ] TUI works correctly with keyboard navigation
- [ ] File locking prevents data corruption
- [ ] Performance is acceptable (< 100ms for common operations)
- [ ] Error messages are helpful
- [ ] Logs are informative for debugging

### Nice to Have (Enhancements)
- [ ] Pre-commit hooks installed
- [ ] CI/CD pipeline configured
- [ ] Benchmarks added
- [ ] AUR package available

---

## Risk Mitigation

### Risk 1: Breaking Changes to Task Model

**Mitigation:**
- Create data migration script
- Version the data format
- Add backward compatibility check

### Risk 2: MCP Protocol Changes

**Mitigation:**
- Pin to specific protocol version
- Monitor MCP spec changes
- Add version negotiation

### Risk 3: Concurrent Access Issues

**Mitigation:**
- Comprehensive file locking tests
- Document concurrent access limitations
- Consider SQLite for future if needed

### Risk 4: Performance Degradation with Large Task Lists

**Mitigation:**
- Add pagination to list operations
- Implement indexing for searches
- Add benchmarks to catch regressions

---

## Timeline Estimate

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| 1. Code Audit | Audit + Fixes | 1-2 hours |
| 2. MCP Implementation | Review + Fix | 1 hour |
| 3. Configuration | Fix JSON configs | 15 minutes |
| 4. Build | Build + Install | 15 minutes |
| 5. MCP Integration | Testing | 30 minutes |
| 6. QA | Tests + Integration | 1 hour |
| 7. Documentation | Write docs | 1 hour |
| 8. Deployment | Package + Distribute | 30 minutes |
| 9. CI/CD | Setup automation | 1 hour |
| **Total** | | **6-8 hours** |

---

## Rollback Plan

If implementation fails:

1. **Code Issues:**
   - Revert to last working commit
   - Review changes incrementally

2. **MCP Issues:**
   - Remove from OpenCode config
   - Debug in isolation

3. **Data Corruption:**
   - Restore from backup
   - Fix locking issues before retry

---

## Post-Implementation

### Monitoring
- Watch for user-reported issues
- Monitor MCP server logs
- Track performance metrics

### Iteration
- Gather user feedback
- Prioritize feature requests
- Plan next version

### Maintenance
- Keep dependencies updated
- Monitor security advisories
- Update documentation as needed

---

## Appendix A: File Checklist

### Files to Modify
- [ ] `vulcan-todo/src/main.rs`
- [ ] `vulcan-todo/src/mcp/server.rs`
- [ ] `dotfiles/opencode/.config/opencode/opencode.json`
- [ ] `~/.config/opencode/opencode.json`

### Files to Create
- [ ] `vulcan-todo/test-mcp.sh`
- [ ] `vulcan-todo/test-integration.sh`
- [ ] `vulcan-todo/docs/MCP_INTEGRATION.md`
- [ ] `vulcan-todo/docs/ARCHITECTURE.md`
- [ ] `.github/workflows/ci.yml` (optional)

### Files to Review
- [ ] `vulcan-todo/src/cli.rs`
- [ ] `vulcan-todo/src/models/task.rs`
- [ ] `vulcan-todo/src/store/json_store.rs`
- [ ] `vulcan-todo/src/mcp/protocol.rs`
- [ ] `vulcan-todo/src/mcp/tools.rs`
- [ ] `vulcan-todo/README.md`

---

## Appendix B: Command Reference

### Development
```bash
# Check code
cargo check

# Run tests
cargo test

# Build debug
cargo build

# Build release
cargo build --release --features tui

# Install locally
cargo install --path . --features tui

# Format code
cargo fmt

# Lint code
cargo clippy

# Run benchmarks
cargo bench
```

### Testing
```bash
# Test CLI
vulcan-todo list
vulcan-todo add "Test" --priority high
vulcan-todo stats

# Test TUI
vulcan-todo

# Test MCP manually
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | vulcan-todo --mcp

# Test with debug logs
RUST_LOG=debug vulcan-todo --mcp

# Validate JSON
jq empty ~/.config/opencode/opencode.json
```

### Deployment
```bash
# Build ISO with vulcan-todo
cd /home/evan/VulcanOS
./scripts/build.sh

# Install on system
sudo pacman -U customrepo/x86_64/vulcan-todo-0.1.0-1-x86_64.pkg.tar.zst
```

---

## Appendix C: Debugging Guide

### Issue: Binary won't compile

**Check:**
```bash
cargo check 2>&1 | tee compile-errors.txt
```

**Common fixes:**
- Missing dependencies: Update Cargo.toml
- Syntax errors: Review error messages carefully
- Type mismatches: Check function signatures

### Issue: MCP server not responding

**Check:**
```bash
# Start server
RUST_LOG=debug vulcan-todo --mcp 2>&1 | tee mcp-debug.log

# In another terminal
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | nc localhost 12345
```

**Common fixes:**
- Check stdin/stdout handling
- Verify JSON parsing
- Check async/await usage

### Issue: OpenCode can't find MCP server

**Check:**
```bash
# Verify config
jq '.mcp."vulcan-todo"' ~/.config/opencode/opencode.json

# Verify binary
which vulcan-todo

# Test manually
vulcan-todo --mcp
```

**Common fixes:**
- Fix config path to binary
- Ensure binary is in PATH
- Check JSON syntax

### Issue: Tasks not persisting

**Check:**
```bash
# Check file exists
ls -l ~/.config/vulcan-todo/tasks.json

# Check contents
cat ~/.config/vulcan-todo/tasks.json

# Check permissions
ls -la ~/.config/vulcan-todo/
```

**Common fixes:**
- Create directory: `mkdir -p ~/.config/vulcan-todo`
- Fix permissions: `chmod 755 ~/.config/vulcan-todo`
- Check file locking implementation

---

**End of Implementation Plan**

This plan provides a comprehensive, step-by-step guide to restore full functionality to vulcan-todo including MCP server, CLI, and TUI modes. Follow phases sequentially for best results.
