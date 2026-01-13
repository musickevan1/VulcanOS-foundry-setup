# OpenCode MCP Configuration Cleanup

## Date: January 4, 2026

## Problem

OpenCode was configured with multiple individual MCP servers that were timing out:
- `brave-search` - Individual MCP server (timed out)
- `filesystem` - Individual MCP server (timed out)
- `github` - Individual MCP server (timed out)
- `memory` - Individual MCP server (timed out)
- `playwright` - Individual MCP server (timed out)
- `docker-gateway` - Timed out despite being running

These were legacy configurations from before the Docker MCP gateway was properly set up.

## Solution

### Centralized MCP Architecture

The **Docker MCP gateway** provides access to ALL these services through a single connection:
- Context7 (documentation lookup)
- GitHub tools
- Brave Search (web search)
- Memory/knowledge graph
- Desktop Commander (filesystem operations)
- SQLite (database)
- Fetch (HTTP requests)
- Next.js DevTools
- GitMCP (git operations)

**Everything runs through docker-gateway** - no separate MCP connections needed!

### Configuration Changes

**Cleaned `~/.config/opencode/opencode.json` to only include:**
```json
"mcp": {
  "docker-gateway": {
    "type": "local",
    "command": ["docker-mcp-gateway", "run"]
  }
}
```

**Note**: Context7 is included within docker-gateway, so there's no need for a separate connection.

### Files Cleaned Up

**Renamed to prevent accidental loading:**
- `opencode-docker.json` → `opencode-docker.json.OLD`
- `opencode-yolo.json` → `opencode-yolo.json.OLD`
- `opencode-safe.json` → `opencode-safe.json.OLD`

**Created backup:**
- `opencode.json.backup-20260104-172438`

**Synced to dotfiles:**
- Updated `/home/evan/VulcanOS/dotfiles/opencode/.config/opencode/opencode.json`

### Additional Improvements

Added missing LSP servers to the config:
- `json` - JSON/JSONC language server
- `html` - HTML language server
- `css` - CSS/SCSS/Less language server
- `yaml` - YAML language server
- `bash` - Bash language server
- `go` - Go language server (gopls)
- `rust` - Rust language server (rust-analyzer)
- `python` - Python language server (pyright)
- `lua` - Lua language server

These were already in the archiso config but missing from the live config.

## How It Works Now

### Single Gateway Pattern

```
OpenCode
  └── docker-gateway (local) → Single MCP connection providing:
        ├── Context7 (documentation lookup)
        ├── GitHub (issues, PRs, code search)
        ├── Brave Search (web search)
        ├── Memory (knowledge graph)
        ├── Desktop Commander (filesystem)
        ├── SQLite (database)
        ├── Fetch (HTTP requests)
        ├── Next.js DevTools
        └── GitMCP (git operations)
```

### Environment Variables

Create `~/.config/opencode/.env` (if needed):
```bash
# Context7 (optional, for higher rate limits)
CONTEXT7_API_KEY=your_key_here

# GitHub access (used by docker-gateway)
GITHUB_TOKEN=ghp_xxxxxxxxxxxx

# Brave Search (used by docker-gateway)
BRAVE_API_KEY=BSAxxxxxxxxxxxx
```

## Testing

After OpenCode restart, the MCP panel should show:
- ✅ `docker-gateway` - Connected

That's it! Just **one MCP connection** that provides access to all services (Context7, GitHub, Brave Search, Memory, Desktop Commander, SQLite, Fetch, Next.js DevTools, GitMCP).

## Benefits

1. **Simpler configuration** - 1 MCP entry instead of 7
2. **Faster startup** - Only 1 connection to establish
3. **Better reliability** - Docker manages MCP services
4. **Consistent state** - All MCP tools share Docker's lifecycle
5. **No timeout issues** - Centralized connection management with static mode

## Additional Fix: Secrets Configuration (Jan 4, 2026 - Evening)

### Problem
Docker MCP gateway was timing out during initialization because it tried to read secrets from Docker Swarm (which isn't enabled) before falling back to environment variables.

### Solution
Created a dedicated secrets file with Docker MCP's expected format and updated the wrapper script to use it:

**Created `~/.docker/mcp/secrets.env`:**
```bash
brave.api_key=YOUR_BRAVE_API_KEY
github.personal_access_token=YOUR_GITHUB_TOKEN
context7.api_key=YOUR_CONTEXT7_KEY
```

**Updated wrapper script:**
```bash
exec docker mcp gateway run --secrets "$HOME/.docker/mcp/secrets.env"
```

This bypasses Docker Desktop's secrets API (which requires Swarm mode) and uses a simple .env file instead.

### Setup Instructions

If you need to update your API keys:

```bash
# Edit the secrets file
nano ~/.docker/mcp/secrets.env

# Format (use dotted notation):
brave.api_key=YOUR_KEY_HERE
github.personal_access_token=YOUR_TOKEN_HERE
context7.api_key=YOUR_KEY_HERE
```

## Troubleshooting

### If docker-gateway times out:

```bash
# Check Docker is running
docker info

# Check MCP toolkit version
docker mcp version

# View gateway logs
docker-mcp-gateway logs

# Verify secrets file exists
ls -la ~/.docker/mcp/secrets.env

# Test gateway manually
timeout 5 docker mcp gateway run --secrets ~/.docker/mcp/secrets.env

# Restart gateway manually
docker-mcp-gateway run
```

### Additional Fix: Static Mode (Jan 4, 2026 - Final)

**Problem**: Gateway took 22 seconds to initialize (OpenCode times out after 5 seconds).

**Solution**: Added `--static --long-lived` flags to pre-start servers, reducing initialization from 22s to 1.3s.

**Requirement**: Install `socat` package:
```bash
sudo pacman -S socat
```

After installing socat and restarting OpenCode, the MCP panel should show:
- ✅ `docker-gateway` - Connected (no timeout!)

## Related Documentation

- [DOCKER-MCP-GATEWAY-FIX.md](./DOCKER-MCP-GATEWAY-FIX.md) - Gateway setup details
- [OPENCODE.md](./OPENCODE.md) - OpenCode user guide
