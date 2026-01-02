# VulcanOS Development Workspace

## Project Context
This workspace is configured for VulcanOS development - a custom Arch Linux distribution for T2 MacBook Pro with Hyprland compositor.

## Core Stack
- **OS Base**: Arch Linux with T2 MacBook support
- **Compositor**: Hyprland (Wayland)
- **Languages**: Bash, Rust, Python, TypeScript
- **Package Manager**: pacman + yay (AUR)
- **Development**: neovim, VS Code, Docker

## Agent Workflow Rules

### Before Writing Code
1. ALWAYS check existing patterns in the codebase first
2. Review the project structure in CLAUDE.md
3. Check for existing scripts in `dotfiles/scripts/` or `archiso/airootfs/usr/local/bin/`

### When Modifying Configs
1. **CRITICAL**: Read the dotfiles structure warning in CLAUDE.md
2. Edit files in `dotfiles/<app>/.config/<app>/` - changes apply immediately via stow symlinks
3. Never delete `.config/` subdirectories - they are stow sources
4. Sync changes to archiso when ready for ISO inclusion

### Code Quality Rules
- Use shellcheck for all bash scripts
- Follow existing naming conventions (vulcan-* for scripts)
- Add proper error handling with `set -euo pipefail`
- Include helpful comments and usage information
- Make scripts executable

### File Naming Conventions
- Scripts: `kebab-case` (e.g., `vulcan-screenshot`)
- Configs: Match application expectations
- Documentation: `UPPERCASE.md` for main docs

## Development Patterns

### Adding a New Script
1. Create in `dotfiles/scripts/` for stow management
2. Copy to `archiso/airootfs/usr/local/bin/` for ISO
3. Add keybinding in `dotfiles/hypr/.config/hypr/bindings.conf`
4. Update documentation

### Modifying Hyprland Config
1. Edit files in `dotfiles/hypr/.config/hypr/`
2. Changes apply immediately (symlinked)
3. Test the changes
4. Copy to `archiso/airootfs/etc/skel/.config/hypr/` for ISO

### Adding Packages
1. Edit `archiso/packages.x86_64`
2. For AUR packages, update `scripts/build-aur-repo.sh`
3. Rebuild ISO to test

## MCP Integration

### Docker MCP Gateway
OpenCode uses Docker MCP for tool access. Ensure Docker Desktop is running with MCP Toolkit enabled.

Available MCP tools:
- `context7` - Library documentation lookup
- `github` - GitHub API access
- `brave-search` - Web search
- `memory` - Persistent memory store
- `filesystem` - File operations
- `playwright` - Browser automation

### Using Context7
When unsure about library APIs:
```
@context7 get docs for [library-name]
```

## Common Tasks

### Testing Changes
```bash
# Test dotfile changes (already live via stow)
hyprctl reload

# Build and test ISO
./scripts/build.sh
./scripts/test-iso.sh
```

### Syncing Dotfiles to ISO
```bash
# Example: sync Hyprland config
cp -r dotfiles/hypr/.config/hypr/* archiso/airootfs/etc/skel/.config/hypr/
```
