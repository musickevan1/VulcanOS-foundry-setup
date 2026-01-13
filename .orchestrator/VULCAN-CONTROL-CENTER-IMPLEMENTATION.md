# VulcanOS Control Center Enhancement - Implementation Orchestrator

@orchestrator

## Task Description

Implement the comprehensive **VulcanOS Control Center Enhancement Plan** to transform VulcanOS from a functional system into a polished, user-friendly distribution with a complete control center, rich theme library, version-controlled updates, and curated app installation system.

## Project Context

**Location**: `/home/evan/VulcanOS/`
**Target Version**: 0.2.0 "Forge"
**Current State**: Functional but incomplete - has waybar with orchestra module, basic vulcan-menu with bugs, limited themes, no version control

## Implementation Phases

### Phase 1: Foundation (CRITICAL - 2-3 hours)
**Priority**: URGENT - Fix existing bugs and improve waybar layout

1. **Waybar Layout Enhancement**
   - Add `vulcan-menu` module to left side with 󰍜 icon
   - Remove `custom/orchestra` module from right side
   - Add CSS styling for new module (Gruvbox theme)
   - Update config.jsonc and style.css files

2. **Vulcan Menu Bug Fixes**
   - Fix Display Settings bug (line 123: replace `info` with wofi submenu)
   - Implement Default Applications menu with xdg-mime
   - Implement Restore Defaults with source selection (/etc/skel vs repo)

### Phase 2: Theme System Expansion (HIGH - 4-6 hours)
**Priority**: HIGH - Significantly improve user experience

1. **Create 7 New Themes**
   - Solarized Dark, Ayu Dark, Material Deep Ocean, Everforest, Kanagawa, Monokai Pro, Palenight
   - Each theme file follows standard format with colors, GTK themes, neovim colorscheme
   - Update vulcan-theme THEMES array with new entries

2. **Wallpaper Library Expansion**
   - Organize existing 7 Vulcan wallpapers properly
   - Download/curate 50+ high-quality wallpapers from Unsplash/theme repos
   - Create category structure: landscapes, abstract, space, minimal, anime, themes
   - Enhance wallpaper menu with browsing by category and theme

### Phase 3: Config Access Menu (MEDIUM - 2 hours)
**Priority**: MEDIUM - Centralize configuration access

1. **New Main Menu Category**
   - Add "Configs" category to main vulcan-menu
   - Create comprehensive config access submenu
   - Include: Hyprland (with sub-submenu), Waybar, Terminal, Themes, Wallpapers, Autostart, Input, Displays

### Phase 4: Version-Controlled Update System (HIGH - 3-4 hours)
**Priority**: HIGH - Essential for maintenance

1. **GitHub Sync System**
   - Add "Sync from GitHub" to update menu
   - Implement pull, version display, changelog, update checker
   - Create functions for repo operations with proper error handling
   - Add to existing update menu structure

### Phase 5: Modular App Install Menu (MEDIUM - 4-5 hours)
**Priority**: MEDIUM - Replace generic search with curated categories

1. **Curated Install Categories**
   - Essential Tools (git, gh, nvim, docker, tmux, ranger)
   - Editors & IDEs (VS Code, Cursor, Zed, Sublime Text)
   - Web Browsers (Firefox, Chromium, Brave, Zen)
   - Development Tools (Node.js, Python, Rust, Go, databases)
   - Design & Creative (GIMP, Inkscape, Blender, Krita)
   - Communication (Discord, Slack, Telegram, Signal)
   - Productivity (Obsidian, LibreOffice)
   - Media & Entertainment (VLC, Spotify, OBS)

2. **Status Indicators**
   - Show [✓] for installed, [ ] for not installed
   - Batch installation capabilities
   - Package search integration

## Key Files to Modify

### Waybar Files (Phase 1)
- `/home/evan/VulcanOS/dotfiles/waybar/.config/waybar/config.jsonc`
- `/home/evan/VulcanOS/dotfiles/waybar/.config/waybar/style.css`
- `/home/evan/VulcanOS/archiso/airootfs/etc/skel/.config/waybar/config.jsonc`
- `/home/evan/VulcanOS/archiso/airootfs/etc/skel/.config/waybar/style.css`

### Vulcan Menu Script (Phases 1, 3, 4, 5)
- `/home/evan/VulcanOS/dotfiles/scripts/.local/bin/vulcan-menu`
- `/home/evan/VulcanOS/archiso/airootfs/usr/local/bin/vulcan-menu`

### Theme System (Phase 2)
- `/home/evan/VulcanOS/dotfiles/themes/colors/` (create 7 new theme files)
- `/home/evan/VulcanOS/dotfiles/themes/templates/` (verify templates exist)
- `/home/evan/VulcanOS/dotfiles/scripts/.local/bin/vulcan-theme` (update THEMES array)

### Wallpaper System (Phase 2)
- `/home/evan/VulcanOS/dotfiles/scripts/.local/bin/vulcan-menu` (enhance wallpaper menu)
- `/home/evan/Pictures/Wallpapers/` (organize existing, add new)

## Implementation Strategy

### Parallel Work
- Start Phase 1 immediately (waybar + menu fixes)
- Work on Phase 2 themes while Phase 1 tests
- Implement Phases 3-5 in sequence

### Testing Requirements
- Test waybar changes with `pkill -SIGUSR2 waybar`
- Test vulcan-menu with `vulcan-menu` command
- Test theme switching with `vulcan-theme set [theme]`
- Test GitHub sync with actual git operations
- Verify all changes persist after reboot

### Quality Standards
- Maintain existing functionality (no regressions)
- Follow existing code patterns and style
- Add proper error handling and user feedback
- Ensure all new features are keyboard-accessible
- Maintain Gruvbox theme consistency

## Success Criteria

**Phase 1 Complete:**
- [ ] Waybar shows vulcan-menu button at top left
- [ ] Orchestra module removed from waybar
- [ ] Display Settings submenu works via wofi
- [ ] Default Applications sets xdg-mime defaults
- [ ] Restore Defaults works with source selection

**Phase 2 Complete:**
- [ ] 15+ themes available in vulcan-theme menu
- [ ] 50+ wallpapers organized by category
- [ ] Wallpaper browsing works via wofi
- [ ] Theme-matched wallpapers function properly

**Phase 3 Complete:**
- [ ] New "Configs" menu category accessible
- [ ] All config files open in editor
- [ ] Hyprland submenu has all config files
- [ ] Reload function works

**Phase 4 Complete:**
- [ ] GitHub sync menu accessible
- [ ] Pull updates works
- [ ] Version display shows correctly
- [ ] Update checker compares local vs remote

**Phase 5 Complete:**
- [ ] All install categories accessible
- [ ] Status indicators show correctly
- [ ] Package installation completes successfully
- [ ] Batch installation works

**Overall Success:**
- [ ] No existing functionality broken
- [ ] All changes synced to archiso
- [ ] Documentation updated
- [ ] Build and test ISO successfully

## Pre-Implementation Notes

1. **Theme Selection**: Proceed with all 7 suggested themes unless user specifies otherwise
2. **Wallpaper Sources**: Use mix of existing theme repos and curated Unsplash images
3. **GitHub Sync**: Implement read-only pull operations (no auto-commit)
4. **Install Menu**: Use individual package selection with status indicators
5. **Version Control**: Manual version updates only (no auto-increment)
6. **Priority Order**: 1 → 4 → 2 → 5 → 3 (address critical bugs first, then maintenance features, then user experience)

## Expected Output

- Complete enhanced vulcan-menu script (~1200+ lines total)
- Updated waybar configuration with new visual launcher
- 7 new theme files with full color schemes
- Organized wallpaper library with 50+ images
- GitHub sync functionality for maintenance
- Curated app installation system
- All changes propagated to archiso for ISO builds
- Comprehensive testing and validation

**Total Implementation Time**: 24-32 hours across all phases