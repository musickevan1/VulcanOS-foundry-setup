# OpenCode Permission Modes

## Quick Start

Launch OpenCode with different permission modes using flags:

```bash
opencode                    # Default mode (asks for permissions)
opencode --safe             # Explicit safe mode (asks for permissions)
opencode --yolo             # YOLO mode (auto-accepts ALL permissions)
```

**If the modes don't work**, run `source ~/.bashrc` or open a new terminal.

## Modes

| Mode | All Permissions | Emoji |
|------|-----------------|--------|
| **Default** | Uses config defaults | - |
| **Safe** | `"ask"` (prompts for all) | 🛡️ |
| **YOLO** | `"allow"` (auto-accepts all) | 🚀 |

## How It Works

The `opencode` function in your `.bashrc` wraps the OpenCode command and:
1. Parses `--yolo` or `--safe` flags
2. Sets `OPENCODE_CONFIG_CONTENT` with inline permission override (highest precedence)
3. Passes through any other arguments to OpenCode

### Why OPENCODE_CONFIG_CONTENT?

OpenCode's [config precedence order](https://opencode.ai/docs/config#precedence-order):

```
1. Remote config        (lowest priority)
2. Global config        ~/.config/opencode/opencode.json
3. OPENCODE_CONFIG      custom config file path
4. Project config       opencode.json in project root
5. .opencode dirs       agents, commands, plugins
6. OPENCODE_CONFIG_CONTENT  ← HIGHEST PRIORITY (inline JSON)
```

By using `OPENCODE_CONFIG_CONTENT`, YOLO mode overrides ALL other configs, including project-specific settings. This ensures consistent behavior across all directories.

### Important: Function Must Be Defined

The `opencode()` wrapper function must be defined for modes to work. If you see errors like "unknown flag: --yolo", the function isn't loaded.

**To fix:**
```bash
source ~/.bashrc
# Or open a new terminal
```

## Examples

**Launch in YOLO mode:**
```bash
opencode --yolo
# Output: 🚀 OpenCode in YOLO mode
```

**Launch in safe mode:**
```bash
opencode --safe
# Output: 🛡️  OpenCode in SAFE mode
```

**Use with other opencode flags:**
```bash
opencode --yolo --continue          # Continue last session in YOLO mode
opencode --safe --session abc123    # Resume specific session in safe mode
opencode --yolo --agent build       # Run build agent in YOLO mode
```

**Launch multiple concurrent sessions:**
```bash
# Terminal 1: Safe mode
opencode --safe

# Terminal 2: YOLO mode (works concurrently!)
opencode --yolo
```

## Troubleshooting

### "unknown flag: --yolo" Error

This means the `opencode()` wrapper function isn't defined. Fix it:

```bash
source ~/.bashrc
```

Or open a new terminal.

### Verify Function is Defined

```bash
type opencode
```

Should show:
```
opencode is a function
```

NOT:
```
opencode is /home/evan/.local/share/pnpm/opencode
```

### YOLO Mode Still Asking for Permissions?

1. **Reload your shell:**
   ```bash
   source ~/.bashrc
   ```

2. **Verify the function is using OPENCODE_CONFIG_CONTENT:**
   ```bash
   type opencode | grep OPENCODE_CONFIG_CONTENT
   ```
   Should show the inline JSON permission override.

3. **Test with debug output:**
   ```bash
   opencode --yolo --print-logs 2>&1 | grep -i permission
   ```

## Warning

**YOLO mode auto-accepts ALL tool operations without prompting:**
- File edits and writes
- Bash command execution
- Web fetching
- And all other tools

Only use on trusted projects with version control (Git)!

## Technical Details

### The Wrapper Function

Located in `~/.bashrc` (lines 6-48), BEFORE the interactive shell check:

```bash
opencode() {
    local MODE="default"
    local ARGS=()

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --yolo) MODE="yolo"; shift ;;
            --safe) MODE="safe"; shift ;;
            *) ARGS+=("$1"); shift ;;
        esac
    done

    case "$MODE" in
        yolo)
            export OPENCODE_CONFIG_CONTENT='{"permission": {"*": "allow"}}'
            echo "🚀 OpenCode in YOLO mode"
            ;;
        safe)
            export OPENCODE_CONFIG_CONTENT='{"permission": {"*": "ask"}}'
            echo "🛡️  OpenCode in SAFE mode"
            ;;
        default)
            unset OPENCODE_CONFIG_CONTENT
            ;;
    esac

    command opencode "${ARGS[@]}"
}
```

### Key Design Decisions

1. **Uses `OPENCODE_CONFIG_CONTENT`** - Highest precedence in OpenCode's config hierarchy
2. **Wildcard permission (`"*": "allow"`)** - Affects all tools, not just edit/bash
3. **Defined before interactive check** - Works in scripts and non-interactive shells
4. **Unsets variable in default mode** - Clean slate when not using YOLO/safe flags

### Where the Function Lives

The `opencode()` function is defined in `~/.bashrc` at **line 6**, BEFORE the interactive shell check.

**Don't move this function** after line 51 (`[[ $- != *i* ]] && return`) or it won't work in scripts and non-interactive shells.

## History

- **Jan 8, 2026**: Switched from `OPENCODE_CONFIG` to `OPENCODE_CONFIG_CONTENT` for reliable permission override
- **Jan 5, 2026**: Fixed issue where function wasn't being defined in non-interactive shells
- **Jan 4, 2026**: Updated configs to use centralized docker-gateway MCP

## Related

- [OpenCode Config Docs](https://opencode.ai/docs/config)
- [OpenCode Permissions Docs](https://opencode.ai/docs/permissions)
