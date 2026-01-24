# Coding Conventions

**Analysis Date:** 2026-01-23

## Naming Patterns

### Files

**Rust:**
- Modules: `snake_case.rs` (e.g., `task.rs`, `json_store.rs`, `mcp_tools.rs`)
- Test modules: Tests are colocated in the same file using `#[cfg(test)]` blocks with `mod tests {}`
- Module hierarchies: Flat structure with `mod.rs` files that re-export (e.g., `src/models/mod.rs` exports `Note`, `Task`, `Memory`)

**Shell Scripts:**
- Command scripts: `snake_case` with no `.sh` extension (e.g., `vulcan-menu`, `opencode-picker`, `workspace-switch`)
- Executable scripts in `.local/bin/` follow kebab-case naming (e.g., `vulcan-dock`, `vulcan-hotkeys`, `vulcan-power`)
- Helper scripts: May use `.sh` extension (e.g., `pipes.sh`, `vulcan-logo.sh`)
- All scripts use shebang: `#!/bin/bash`

**Config Files:**
- Hyprland: `.conf` extension (e.g., `bindings.conf`, `autostart.conf`)
- Waybar: JSON with optional `.jsonc` extension
- TOML configs for Rust projects (Cargo.toml for package manifests)

### Functions

**Rust:**
- Getter methods: Use `get_*` pattern (e.g., `get_all()`, `get_store()`, `get_by_project()`)
- Setter methods: Use `set_*` or direct field assignment
- Checkers: Use `is_*` or `has_*` prefix (e.g., `is_pending()`, `is_done()`, `has_context_notes()`)
- Action methods: Verb-based (e.g., `complete()`, `uncomplete()`, `toggle()`, `cycle_status()`)
- Resource creation: `new()` for main constructor, `new_with_*()` for variants (e.g., `new_with_scope()`)
- Trait methods: Include documentation comments describing the contract

**Shell Scripts:**
- Function names: `snake_case` (e.g., `notify()`, `run_in_terminal()`, `wofi_menu()`)
- Global configuration: Uppercase constants (e.g., `WOFI_WIDTH`, `CONFIG_DIR`, `TERMINAL`)
- Helper sections marked with comments (e.g., `# Helpers`, `# Wofi menu helper`)

### Variables

**Rust:**
- Local variables: `snake_case` (e.g., `store_path`, `task_id`, `chunk_content`)
- Constants: `UPPER_SNAKE_CASE` (e.g., `CURRENT_VERSION`, `EMBEDDING_DIM`)
- Struct fields: `snake_case` (e.g., `created_at`, `sprint_order`, `auto_fetch_context`)
- Type aliases: `PascalCase` (e.g., `StoreType`, `ToolFn`, `StoreResult<T>`)

**Shell:**
- Environment variables: `UPPER_SNAKE_CASE` (e.g., `HOME`, `XDG_DATA_HOME`, `PROJECT_ROOTS`)
- Local variables: `lower_snake_case` (e.g., `options`, `choice`, `action`)
- Script directories: Capture with `SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"`

### Types

**Rust:**
- Enums: `PascalCase` variants (e.g., `Status::Pending`, `Priority::Urgent`, `NoteType::Project`)
- Enum serialization: `#[serde(rename_all = "lowercase")]` for JSON compatibility (e.g., "pending" in JSON maps to `Status::Pending`)
- Struct names: `PascalCase` (e.g., `Task`, `Note`, `Chunk`, `Sprint`, `ToolResult`)
- Type parameters: Single uppercase letter or descriptive (e.g., `T`, `F: Fn()`)

## Code Style

### Formatting

**Rust:**
- Use Rust default formatting (2-space indentation via rustfmt defaults)
- Line length: No hard limit enforced, but aim for readability (80-100 chars is conventional)
- Blank lines: Separate logical sections within functions and between methods
- Imports: Group into three sections:
  1. Standard library (e.g., `use std::*`)
  2. External crates (e.g., `use serde::*`)
  3. Local modules (e.g., `use crate::models::*`)

**Shell:**
- Use bash formatting conventions: 2 or 4 space indentation (observed: 4 spaces)
- Shebang always: `#!/bin/bash`
- Error handling: `set -e` for early exit on error, `set -euo pipefail` for strict mode
- Function definitions: `function_name() {` on same line with opening brace
- Command grouping: Use `$()` for subshells instead of backticks

### Linting

**Rust:**
- No explicit linting configuration found (no `.clippy.toml` or `rustfmt.toml`)
- Relies on Rust standard conventions and cargo defaults
- Clippy may be used implicitly during builds

**Shell:**
- No shellcheck configuration detected
- Scripts follow defensive programming patterns (e.g., `set -e`, error checks)

## Import Organization

### Rust

**Pattern observed in `vulcan-todo/src/main.rs`:**

```rust
// 1. Standard library
use std::path::PathBuf;
use std::sync::Arc;

// 2. External crates
use anyhow::Result;
use clap::Parser;
use tokio;

// 3. Local modules
mod cli;
mod mcp;
mod models;
mod store;
mod ui;

// 4. Re-exports (for public API)
pub use models::{Sprint, Task};
```

**Pattern in modules:**
- Use `pub use` to re-export types in module `mod.rs` files
- Example: `src/models/mod.rs` exports `Note`, `NoteType`, `Chunk` via public use statements
- Enables cleaner imports: `use crate::Note` instead of `use crate::models::note::Note`

### Shell

**Pattern in shell scripts:**
- Configuration section at top with variables
- Functions grouped by category with comment separators (e.g., `# Helpers`, `# Main Menu`)
- Source external scripts using relative paths

## Error Handling

### Rust

**Pattern: Using `anyhow::Result<T>`**

```rust
// Main functions return Result<()>
async fn main() -> Result<()> {
    // Operations that can fail
    let store = get_store(path)?;
    // ...
    Ok(())
}

// Helper functions return Result<T>
fn get_store(path: Option<PathBuf>) -> Result<Arc<dyn store::Store>> {
    let store = if let Some(p) = path {
        StoreType::with_path(p)?
    } else {
        StoreType::new()?
    };
    Ok(Arc::new(store))
}
```

**Error propagation:**
- Use `?` operator for error propagation (prefer over `match`)
- anyhow provides flexible error context with `.context()` method
- CLI output: Use `eprintln!()` for error messages, `println!()` for success

**Error wrapping:**
- Store trait methods return `Result<T>` (anyhow or custom)
- MCP tool results use `ToolResult { success, message, data }` struct (not exceptions)

**Panic handling:**
- Minimal use of `unwrap()` / `expect()` - prefer explicit error handling
- `unwrap_or()` used for defaults (e.g., `t.sprint_order.unwrap_or(i32::MAX)`)

### Shell

**Pattern: Early exit strategy**

```bash
#!/bin/bash
set -e  # Exit on any error

# Error handling
error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        error "This script must be run as root (use sudo)"
    fi
}
```

**Error messaging:**
- Error function: `error()` writes to stderr and exits with code 1
- Warning function: `warn()` for non-fatal issues
- Success function: `success()` for operation confirmations
- Color codes: `${RED}`, `${GREEN}`, `${YELLOW}`, `${BLUE}`, `${NC}`

**Logging:**
- Functions: `info()`, `success()`, `warn()`, `error()`
- All output prefixed with `[INFO]`, `[SUCCESS]`, `[WARNING]`, `[ERROR]`
- Timestamps not used in observed scripts

## Logging

### Rust

**Framework:** `tracing` crate with `tracing-subscriber` initialization

```rust
use tracing_subscriber;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    // Now logs are available via tracing macros
}
```

**Pattern:** Initialize once in main, use throughout codebase (not observed in shown code, but configured in Cargo.toml)

### Shell

**Pattern:** Simple print-based logging

- Use `info()`, `warn()`, `success()`, `error()` helper functions
- All visible in console output with color codes
- No file-based logging in observed scripts
- Environment variable control: `VERBOSE` or similar (not observed)

## Comments

### When to Comment

**Rust:**
- Module-level: Document with `//!` comments (shown in `lib.rs` with ASCII diagrams)
- Function-level: Use `///` for public API documentation
- Implementation comments: Use `//` sparingly - prefer self-documenting code
- TODO items: Use `// TODO: Description` (seen in sprint command parsing)
- Section dividers: Use `// ==================== Section Name ====================` for logical grouping

**Shell:**
- Header comments: Describe script purpose and usage
- Section markers: `# =============================================================================`
- Inline comments: Use `# Comment` for complex logic
- Config sections marked clearly (e.g., `# Configuration`, `# Colors`)

### JSDoc/TSDoc

**Not applicable** - this is a Rust/Shell codebase, not TypeScript/JavaScript.

## Function Design

### Size

**Rust:**
- Functions range from 2 lines (getters) to 100+ lines (main handlers)
- Large functions: `handle_command()` (150 lines) and `main()` (40 lines)
- Pattern: Top-level commands use match statements for command routing, delegating to smaller handlers
- No observed function over ~200 lines

**Shell:**
- Functions: 5-50 lines typical
- Main entry point: `show_main_menu()` handles menu display and recursion
- Helper functions: 2-20 lines

### Parameters

**Rust:**
- Prefer owned types (`String`) for function parameters when mutation needed
- Use references (`&str`, `&[T]`) for read-only access
- Options: Use `Option<T>` for optional parameters (rather than null/nil)
- Context passing: Use `Arc<dyn Trait>` for shared, immutable resources (e.g., `Arc<dyn Store>`)

**Shell:**
- Positional parameters: `$1`, `$2`, etc.
- Optional parameters: Check with `${1:-default}` syntax
- Array parameters: Pass as `"${array[@]}"`
- Context: Global variables for config

### Return Values

**Rust:**
- Functions use `Result<T>` for operations that can fail
- Methods use `Option<T>` to indicate absence (e.g., `get()` returns `Option<Task>`)
- Prefer returning borrowed data when possible (`&Task` vs `Task`)
- Tuple returns for multiple values: `(pending_count, done_count)`

**Shell:**
- Return status: `0` for success, non-zero for failure
- Output: Via `echo` or `printf` (captured with `$()`)
- Multiple values: Via global variables or piped output

## Module Design

### Exports

**Rust:**
- `pub` keyword makes items public
- Module-level: `pub use` re-exports types in `mod.rs` for cleaner imports
- Example in `models/mod.rs`:
  ```rust
  pub use note::{Note, NoteType, NoteStatus};
  pub use task::{Task, Priority, Status};
  ```
- Allows: `use crate::Task` instead of `use crate::models::task::Task`

**Shell:**
- Functions are implicitly "exported" (sourced)
- No explicit module system; scripts are self-contained

### Barrel Files

**Rust:**
- `src/models/mod.rs` serves as barrel file: aggregates and re-exports types
- `src/store/mod.rs` defines trait, `sqlite_store.rs` implements it
- Pattern: Allows `use crate::Store` at any level

**Observed in:**
- `vulcan-todo/src/models/mod.rs` - re-exports `Task`, `Sprint`, `Priority`, `Status`
- `vulcan-vault/src/store/mod.rs` - re-exports `SqliteStore`, `StoreError`, trait definitions

### Trait Patterns

**Storage abstraction:**
- `Store` trait: Defines interface for task/note persistence
- Implementations: `JsonStore` (vulcan-todo), `SqliteStore` (vulcan-vault)
- Pattern: `Arc<dyn Store>` for trait objects passed to functions

**MCP Tools:**
- `Tool` struct: Name, description, schema, function pointer
- `ToolResult`: Encapsulates success/failure + data for JSON serialization

## Special Patterns

### Configuration Management

**Rust:**
- Environment variables: Read via `std::env::var()` (e.g., `VULCAN_TODO_PATH`)
- Fallback logic: `cli.path.or_else(|| std::env::var("VULCAN_TODO_PATH").ok().map(PathBuf::from))`
- No config file parsing observed in shown code

**Shell:**
- Top-level constants: `CONFIG_DIR="${HOME}/.config"`, `TERMINAL="${TERMINAL:-kitty}"`
- Defaults via bash syntax: `${VAR:-default_value}`
- No YAML/TOML parsing

### JSON Output

**Rust:**
- Use `serde_json::json!()` macro for inline JSON construction
- Example: Task output for JSON output mode uses `serde_json::json!()` in main.rs
- All models derive `Serialize, Deserialize` from serde

### Feature Flags

**Rust:**
- Conditional compilation: `#[cfg(feature = "tui")]`
- Used to include/exclude TUI code when building without `tui` feature
- Example: `cargo build --features tui` vs `cargo build`
- Observed in:
  - `vulcan-todo`: TUI feature gates ratatui, crossterm, notify dependencies
  - `vulcan-vault`: Similar TUI feature gating

---

*Convention analysis: 2026-01-23*
