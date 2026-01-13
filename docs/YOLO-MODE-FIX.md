# YOLO Mode Fix - January 5, 2026

## Problem

After the OpenCode MCP cleanup on January 4, 2026, the YOLO mode stopped working. The `opencode --yolo` command was no longer auto-accepting file edits and bash commands.

## Root Cause

Two issues were identified:

1. **Missing Config Files**: The permission mode config files (`opencode-yolo.json` and `opencode-safe.json`) were renamed to `.OLD` during the MCP cleanup to prevent them from being loaded.

2. **Function Not Defined**: The `opencode()` wrapper function in `~/.bashrc` was defined AFTER the interactive shell check:
   ```bash
   # Line 7
   [[ $- != *i* ]] && return  # Exit if not interactive

   # Line 248
   opencode() { ... }  # Never reached in non-interactive shells!
   ```

   This meant the function was only available in interactive shells, causing the `opencode` command to bypass the wrapper and call the pnpm-installed binary directly.

## Solution

### 1. Restored Config Files with Modern MCP

Created new config files with the simplified docker-gateway MCP configuration:

- `~/.config/opencode/opencode-yolo.json` - YOLO mode (edit: allow, bash: allow)
- `~/.config/opencode/opencode-safe.json` - Safe mode (edit: ask, bash: ask)

Both configs now use the centralized docker-gateway MCP instead of individual MCP servers.

### 2. Moved Function Definition

Moved the `opencode()` function definition to BEFORE the interactive shell check in `~/.bashrc`:

```bash
# NEW ORDER (Lines 6-42):
# =============================================================================
# OPENCODE PERMISSION MODES (defined early to work in non-interactive shells)
# =============================================================================
opencode() { ... }

# If not running interactively, don't do anything
[[ $- != *i* ]] && return
```

This ensures the function is always available, even in non-interactive shells and scripts.

## How to Use

### After Fix

Open a new terminal or reload your shell:

```bash
source ~/.bashrc
```

### YOLO Mode (Auto-accept all permissions)

```bash
opencode --yolo
```

Output: `🚀 OpenCode in YOLO mode`

### Safe Mode (Ask for all permissions)

```bash
opencode --safe
```

Output: `🛡️  OpenCode in SAFE mode`

### Default Mode

```bash
opencode
```

Uses default config (edit: ask, bash: ask)

## Verification

Check the permissions in each config:

```bash
jq '.permission' ~/.config/opencode/opencode.json
jq '.permission' ~/.config/opencode/opencode-safe.json
jq '.permission' ~/.config/opencode/opencode-yolo.json
```

Expected output:
- `opencode.json`: edit=ask, bash=ask
- `opencode-safe.json`: edit=ask, bash=ask
- `opencode-yolo.json`: edit=allow, bash=allow

## Files Modified

1. **~/.config/opencode/opencode-yolo.json** - Created
2. **~/.config/opencode/opencode-safe.json** - Created
3. **~/.bashrc** - Moved `opencode()` function definition to line 6 (before interactive check)

## Technical Details

### Why the Function Wasn't Working

When bash sources `.bashrc`, it executes line by line:

1. Line 7: `[[ $- != *i* ]] && return` - Checks if shell is interactive
2. If NOT interactive: `return` exits immediately, never reaching line 248
3. If interactive: Continues to line 248 and defines the function

When running `opencode --yolo`, bash looks for:
1. Function named `opencode` (only if it was defined)
2. Binary in `$PATH`

Since the function wasn't defined, bash found the pnpm binary at `~/.local/share/pnpm/opencode`, which doesn't support the `--yolo` flag.

### Why Moving the Function Fixes It

By moving the function definition to line 6 (before the interactive check):
1. Function is always defined when `.bashrc` is sourced
2. Bash finds the function first when looking for `opencode`
3. The function parses `--yolo`/`--safe` flags and sets `OPENCODE_CONFIG`
4. The function calls `command opencode` to run the actual binary

## Related Documentation

- [PERMISSION_MODES.md](./PERMISSION_MODES.md) - Detailed guide on permission modes
- [OPENCODE-MCP-CLEANUP.md](./OPENCODE-MCP-CLEANUP.md) - Original cleanup that caused the issue
- [OPENCODE.md](./OPENCODE.md) - OpenCode user guide
