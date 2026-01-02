# OpenCode Setup for VulcanOS

OpenCode is an AI-powered coding assistant that runs in the terminal. VulcanOS includes pre-configured settings for OpenCode with Docker MCP integration.

## Installation

OpenCode is not available in official Arch repositories. Install it after first boot:

```bash
# Install OpenCode
curl -fsSL https://opencode.ai/install | bash

# The binary will be installed to ~/.local/bin/opencode
# Ensure ~/.local/bin is in your PATH (already configured in VulcanOS)
```

## Quick Start

### Launching OpenCode

**Hotkey**: `Super + Shift + O`

This opens the project picker, allowing you to select a directory before launching OpenCode. You can also launch directly:

```bash
# Open in current directory
opencode

# Open in specific directory
cd ~/projects/myproject && opencode
```

### Authentication

OpenCode requires authentication with your AI provider:

```bash
# Login (opens browser for OAuth)
opencode auth login

# Check auth status
opencode auth status
```

## Configuration

VulcanOS provides a pre-configured OpenCode setup at `~/.config/opencode/`:

```
~/.config/opencode/
├── opencode.json          # Main configuration
├── AGENTS.md              # Agent instructions
├── agent/
│   ├── build.md           # Build agent (full tool access)
│   └── plan.md            # Plan agent (read-only)
└── knowledge/
    ├── stack.md           # VulcanOS tech stack
    └── conventions.md     # Project conventions
```

### Configuration File

The main config (`opencode.json`) includes:

- **Model**: Claude Sonnet (configurable)
- **Tools**: Edit, write, read, bash, glob, grep, webfetch, todo
- **LSP Servers**: TypeScript, Python, Rust, Go, Lua, Bash, and more
- **MCP**: Docker MCP Gateway for extended tools

### Changing the Model

Edit `~/.config/opencode/opencode.json`:

```json
{
  "model": "anthropic/claude-sonnet-4-20250514"
}
```

Available models depend on your subscription.

## Docker MCP Gateway

OpenCode uses Docker MCP (Model Context Protocol) for extended capabilities like GitHub access, web search, browser automation, and more.

### Prerequisites

1. **Docker Desktop** with MCP Toolkit enabled:
   ```bash
   # Install Docker Desktop (Arch Linux)
   yay -S docker-desktop
   
   # Start Docker Desktop
   systemctl --user start docker-desktop
   ```

2. **Enable MCP Toolkit**:
   - Open Docker Desktop
   - Go to Settings → Beta features
   - Enable "MCP Toolkit"

### Using Docker MCP

The `docker-mcp-gateway` wrapper script handles starting Docker and the MCP gateway:

```bash
# Check status
docker-mcp-gateway status

# View logs
docker-mcp-gateway logs

# Manually start (usually automatic)
docker-mcp-gateway run
```

### Available MCP Tools

When Docker MCP is enabled, OpenCode has access to:

| Tool | Description |
|------|-------------|
| `context7` | Library documentation lookup |
| `github` | GitHub API (issues, PRs, repos) |
| `brave-search` | Web search |
| `memory` | Persistent knowledge store |
| `filesystem` | Extended file operations |
| `playwright` | Browser automation |

### Environment Variables

Create `~/.config/opencode/.env` for API keys:

```bash
# GitHub access (for GitHub MCP)
GITHUB_TOKEN=ghp_xxxxxxxxxxxx

# Brave Search (for web search)
BRAVE_API_KEY=BSAxxxxxxxxxxxx

# Context7 (optional, for higher rate limits)
CONTEXT7_API_KEY=xxxxxxxxxxxx
```

## Agents

VulcanOS configures two agents:

### Build Agent

Full tool access for implementing changes:

```
/agent build
```

- Can edit files, run commands, make changes
- Used for implementation tasks

### Plan Agent

Read-only access for analysis:

```
/agent plan
```

- Can read files and search codebase
- Cannot make changes
- Used for planning and analysis

## Keybindings

| Keybinding | Action |
|------------|--------|
| `Super + Shift + O` | Open project picker → launch OpenCode |

## Project Picker

The project picker (`opencode-picker`) shows:

1. **Quick Options**: Browse, current directory, home
2. **Recent Directories**: From zoxide history (if installed)
3. **Git Projects**: Automatically found in common locations

### Customizing Project Roots

Edit `~/.local/bin/opencode-picker`:

```bash
PROJECT_ROOTS=(
    "$HOME"
    "$HOME/projects"
    "$HOME/code"
    "$HOME/work"
    "$HOME/dev"
)
```

## LSP Integration

OpenCode includes pre-configured LSP servers for intelligent code assistance:

| Language | Server |
|----------|--------|
| TypeScript/JavaScript | typescript-language-server |
| Python | pyright |
| Rust | rust-analyzer |
| Go | gopls |
| Lua | lua-language-server |
| Bash | bash-language-server |
| YAML | yaml-language-server |
| JSON | vscode-json-language-server |
| HTML | vscode-html-language-server |
| CSS | vscode-css-language-server |

LSP provides:
- Autocompletion suggestions
- Go to definition
- Find references
- Hover documentation
- Error diagnostics

## Troubleshooting

### OpenCode Not Found

Ensure `~/.local/bin` is in your PATH:

```bash
echo $PATH | grep -q ".local/bin" || echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Docker MCP Not Working

1. Check Docker is running:
   ```bash
   docker info
   ```

2. Check MCP Toolkit is enabled:
   ```bash
   docker mcp version
   ```

3. View gateway logs:
   ```bash
   docker-mcp-gateway logs
   ```

### Authentication Issues

```bash
# Re-authenticate
opencode auth logout
opencode auth login

# Check status
opencode auth status
```

### LSP Not Working

Ensure the language server is installed:

```bash
# Example: TypeScript
npm install -g typescript-language-server typescript

# Example: Python
pip install pyright
```

## Tips

### Using Context7 for Documentation

When unsure about a library API:

```
use context7 to look up [library-name] documentation
```

Or in chat:
```
@context7 get docs for react-query
```

### Using Agents Effectively

1. Start with `plan` agent for complex tasks
2. Switch to `build` agent for implementation
3. Use todo lists for multi-step tasks

### Keyboard Shortcuts in OpenCode

| Key | Action |
|-----|--------|
| `Ctrl+C` | Cancel current operation |
| `Ctrl+D` | Exit OpenCode |
| `Tab` | Autocomplete |
| `Up/Down` | Navigate history |

## Updates

OpenCode auto-updates by default. To change:

```json
{
  "autoupdate": "notify"  // or "off"
}
```

Manual update:
```bash
curl -fsSL https://opencode.ai/install | bash
```
