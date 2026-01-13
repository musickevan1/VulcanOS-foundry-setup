# Docker MCP Gateway Fix Summary

## Problem
The Docker MCP gateway configuration had several issues:
- OpenCode config was set to use HTTP/SSE mode on port 3001
- Gateway script was trying to use stdio mode by default
- The symlink to docker-mcp-gateway was broken (pointing to wrong path)
- Legacy docker-mcp-start* scripts cluttering ~/.local/bin
- Inconsistent scripts directory structure (root-level vs .local/bin/)
- MCP configuration name mismatch between dotfiles and archiso

## Solution

### 1. Fixed the broken symlink
**Before**: `~/.local/bin/docker-mcp-gateway` → `/home/evan/VulcanOS/dotfiles/scripts/docker-mcp-gateway` (broken)
**After**: `~/.local/bin/docker-mcp-gateway` → `/home/evan/VulcanOS/dotfiles/scripts/.local/bin/docker-mcp-gateway` (working)

### 2. Standardized on stdio mode
Changed both OpenCode config and gateway script to use stdio mode (simpler, no authentication needed):

**OpenCode Config** (`~/.config/opencode/opencode.json`):
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  }
}
```

**Gateway Script** (`docker-mcp-gateway`):
- Runs `docker mcp gateway run` (stdio mode, no --transport or --port flags)
- OpenCode communicates directly via stdin/stdout
- No authentication token needed

### 3. Cleaned up legacy scripts (Jan 4, 2026)
Removed 4 obsolete docker-mcp-start variants:
- `docker-mcp-simple-start` (removed)
- `docker-mcp-start` (removed)
- `docker-mcp-start-final` (removed)
- `docker-mcp-start-new` (removed)

Only `docker-mcp-gateway` remains as the canonical wrapper.

### 4. Consolidated scripts directory structure (Jan 4, 2026)
Moved all root-level scripts to `.local/bin/` for proper GNU Stow structure:

**Moved scripts:**
- hyprmon-desc
- restore-monitor-config
- vulcan-dictation
- vulcan-s2t (and 4 related scripts)
- vulcan-screensaver
- vulcan-wallpaper

**Result:** All 23 scripts now in `dotfiles/scripts/.local/bin/`, enabling clean `stow scripts` workflow.

### 5. Fixed MCP configuration naming
Standardized MCP configuration key to `"docker-gateway"` across:
- `/home/evan/VulcanOS/dotfiles/opencode/.config/opencode/opencode.json`
- `/home/evan/VulcanOS/archiso/airootfs/etc/skel/.config/opencode/opencode.json`

### 6. Updated files
- `/home/evan/VulcanOS/dotfiles/scripts/.local/bin/docker-mcp-gateway` (live version)
- `/home/evan/VulcanOS/archiso/airootfs/usr/local/bin/docker-mcp-gateway` (ISO version)
- `/home/evan/.config/opencode/opencode.json` (live OpenCode config - user customized)
- `/home/evan/VulcanOS/dotfiles/opencode/.config/opencode/opencode.json` (reference version)
- `/home/evan/VulcanOS/archiso/airootfs/etc/skel/.config/opencode/opencode.json` (ISO version)

## Testing
All tests pass:
✓ Docker is ready
✓ MCP Toolkit available (v0.34.0)
✓ docker-mcp-gateway command found

## Usage
The gateway will automatically start when OpenCode needs it. You can also:
```bash
# Ensure Docker is running
docker-mcp-gateway ensure-docker

# Check gateway status
docker-mcp-gateway status

# View wrapper logs
docker-mcp-gateway logs
```

## Benefits of stdio mode over HTTP/SSE:
1. No authentication complexity
2. Simpler configuration
3. Direct communication (no network layer)
4. Automatic lifecycle management by OpenCode
5. Better security (no exposed network port)

## Current Status (Jan 4, 2026)

### ✅ Verified Working
- ✓ Docker is running
- ✓ MCP Toolkit v0.34.0 available
- ✓ `docker-mcp-gateway` command found and functional
- ✓ Gateway starting successfully (logs confirm multiple successful starts)
- ✓ All 23 scripts properly organized in `dotfiles/scripts/.local/bin/`
- ✓ All scripts properly symlinked to `~/.local/bin/`
- ✓ No legacy/duplicate MCP scripts remaining
- ✓ OpenCode configs synchronized (dotfiles ↔ archiso)

### 📂 Directory Structure
```
dotfiles/scripts/
└── .local/
    └── bin/
        ├── docker-mcp-gateway         # MCP Gateway wrapper
        ├── opencode-picker            # Project picker
        ├── hyprmon-desc               # Monitor utilities
        ├── restore-monitor-config
        ├── vulcan-s2t                 # Speech-to-text
        ├── vulcan-s2t-deps
        ├── vulcan-s2t-server
        ├── vulcan-s2t-settings
        ├── vulcan-s2t-waybar
        ├── vulcan-dictation
        ├── vulcan-screensaver         # UI utilities
        ├── vulcan-screenshot
        ├── vulcan-wallpaper
        ├── vulcan-wallpapers
        ├── vulcan-copy-paste          # System utilities
        ├── vulcan-hotkeys
        ├── vulcan-idle
        ├── vulcan-menu
        ├── vulcan-power
        ├── vulcan-theme
        ├── vulcan-logo.sh
        ├── vulcan-logo-trace.sh
        └── pipes.sh
```

All scripts are symlinked via GNU Stow to `~/.local/bin/`.
