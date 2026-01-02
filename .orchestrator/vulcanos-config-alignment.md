# VulcanOS Configuration Alignment & Enhancement Plan

**Project:** VulcanOS Configuration Repair and ISO Alignment  
**Date:** 2026-01-01  
**Status:** Ready for Orchestrator Execution  
**Estimated Time:** 2-3 hours

---

## Executive Summary

This plan addresses critical broken features in VulcanOS caused by missing scripts and misaligned configurations between the live system and the VulcanOS repository/ISO. The orchestrator will systematically repair, align, and enhance the system across 5 phases.

**Key Goals:**
1. ✅ Restore all broken keybindings (vulcan-menu, vulcan-power, vulcan-hotkeys, etc.)
2. ✅ Establish single source of truth (VulcanOS repo)
3. ✅ Implement proper GNU Stow structure for live syncing
4. ✅ Sync archiso scripts → dotfiles → live system
5. ✅ Add quality-of-life enhancements

---

## Current State Analysis

### 🔴 Critical Issues (Blocking Usage)

| Issue | Impact | Affected Features |
|-------|--------|-------------------|
| Missing `vulcan-menu` | Cannot access system menu | `Super+Alt+Space`, all submenus |
| Missing `vulcan-power` | Cannot use power menu | `Super+Escape` |
| Missing `vulcan-hotkeys` | Cannot view keybindings | `Super+K` |
| Missing `vulcan-wallpaper` | Cannot rotate wallpapers | `Super+Ctrl+Space` |
| Missing `vulcan-screenshot` | Broken screenshot bindings | `Super+Ctrl+S`, `Super+Alt+S` |
| Missing `vulcan-theme` in PATH | Cannot switch themes | `Super+Ctrl+Shift+Space` |

### 🟡 Configuration Drift

- `~/.config/waybar/` is a directory (not symlink) → changes in repo don't apply
- `~/.config/wofi/` is a directory (not symlink) → changes in repo don't apply
- Scripts exist in `archiso/airootfs/usr/local/bin/` but not in `dotfiles/scripts/`
- Inconsistent stow structure for some apps (kitty, starship)

### ✅ Working Components

- Hyprland core configuration
- Core keybindings (window management, workspaces)
- `vulcan-wallpapers`, `vulcan-idle`, `vulcan-copy-paste` (already in PATH)

---

## Architecture Decision: Single Source of Truth

```
┌─────────────────────────────────────────────────────────┐
│              VulcanOS Repository (SSOT)                 │
│  /home/evan/VulcanOS/                                   │
└─────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┴───────────────┐
            ▼                               ▼
┌───────────────────────┐       ┌───────────────────────┐
│   dotfiles/           │       │   archiso/airootfs/   │
│   (User configs)      │       │   (ISO skeleton)      │
│                       │       │                       │
│   GNU Stow →          │       │   /etc/skel/ ←────────┤
│   ~/.config/          │       │   /usr/local/bin/     │
│   ~/.local/bin/       │       │                       │
└───────────────────────┘       └───────────────────────┘
            │                               │
            └───────────────┬───────────────┘
                            ▼
                ┌───────────────────────┐
                │   Live System         │
                │   ~/.config/          │
                │   ~/.local/bin/       │
                └───────────────────────┘
```

**Flow:**
1. Scripts live in `dotfiles/scripts/`
2. Stow symlinks `dotfiles/scripts/` → `~/.local/bin/`
3. Build process copies `dotfiles/` → `archiso/airootfs/etc/skel/`
4. ISO users get fresh copy from `/etc/skel/`

---

## Backup & Rollback System

**CRITICAL:** Before executing ANY phase, automatic backups are created.

### Automatic Backup Wrapper

All phases execute through the backup wrapper:
```bash
/home/evan/VulcanOS/scripts/auto-backup-wrapper.sh "phase-name" command
```

**What it does:**
1. Creates timestamped backup in `~/VulcanOS-backups/YYYYMMDD-HHMMSS-before-phase-name/`
2. Backs up: configs, scripts, dotfiles, metadata
3. Generates instant restore script: `quick-restore.sh`
4. Executes the phase command
5. On failure: Displays rollback instructions

### Rollback at Any Time

**Instant Rollback (Last Backup):**
```bash
LAST_BACKUP=$(cat /tmp/vulcan-last-backup.txt)
${LAST_BACKUP}/quick-restore.sh
```

**List All Backups:**
```bash
ls -lt ~/VulcanOS-backups/
```

**Restore Specific Backup:**
```bash
~/VulcanOS-backups/20260101-143022-before-phase-3/quick-restore.sh
```

### Manual Full Backup (Anytime)

```bash
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "pre-orchestrator"
```

This creates:
- Full backup with compression option
- Detailed metadata
- Complete restore script
- File inventory

---

## Phase 1: Emergency Script Rescue (HIGH PRIORITY)

**Objective:** Restore all broken keybindings by deploying missing scripts

**Backup:** ✅ Automatic before execution

### Tasks

#### 1.1 Copy Missing Scripts from archiso → dotfiles

**Source:** `/home/evan/VulcanOS/archiso/airootfs/usr/local/bin/`  
**Destination:** `/home/evan/VulcanOS/dotfiles/scripts/`

Scripts to copy:
- ✅ `vulcan-menu` (831 lines - full system menu)
- ✅ `vulcan-power` (157 lines - power/session menu)
- ✅ `vulcan-hotkeys` (120 lines - hotkey display)
- ✅ `vulcan-wallpaper` (different from `vulcan-wallpapers`)
- ✅ `vulcan-screensaver`
- ✅ `vulcan-logo.sh`
- ✅ `vulcan-logo-trace.sh`
- ✅ `pipes.sh` (screensaver effect)

**Already exist in dotfiles/scripts (no action):**
- `vulcan-copy-paste`
- `vulcan-idle`
- `vulcan-screenshot`
- `vulcan-theme`
- `vulcan-wallpapers`
- `opencode-picker`
- `docker-mcp-gateway`

#### 1.2 Make All Scripts Executable

```bash
chmod +x /home/evan/VulcanOS/dotfiles/scripts/*
```

#### 1.3 Symlink to ~/.local/bin (Immediate Fix)

**Execute with backup wrapper:**
```bash
/home/evan/VulcanOS/scripts/auto-backup-wrapper.sh "phase1-symlinks" bash -c '
cd /home/evan/VulcanOS/dotfiles
for script in scripts/*; do
    ln -sf "$(pwd)/$script" ~/.local/bin/$(basename "$script")
done
'
```

**Verification:**
```bash
which vulcan-menu
which vulcan-power
which vulcan-hotkeys
# Test keybindings
# Super+Escape, Super+Alt+Space, Super+K
```

**Rollback if needed:**
```bash
# Automatic backup location stored in /tmp/vulcan-last-backup.txt
$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
```

---

## Phase 2: Stow Structure Standardization (MEDIUM PRIORITY)

**Objective:** Establish proper GNU Stow directory structure for all configs

### Current Issues

```
dotfiles/
├── hypr/.config/hypr/          ✅ CORRECT
├── waybar/.config/waybar/       ✅ CORRECT
├── nvim/.config/nvim/           ✅ CORRECT
├── kitty/kitty.conf             ❌ WRONG (should be .config/kitty/)
├── starship/starship.toml       ❌ DUPLICATE (has both)
└── scripts/                     ❌ NEEDS .local/bin/ structure
```

### Tasks

#### 2.1 Fix Kitty Structure

```bash
# Move kitty.conf to proper location
mkdir -p dotfiles/kitty/.config/kitty/
mv dotfiles/kitty/kitty.conf dotfiles/kitty/.config/kitty/kitty.conf
# Remove old duplicate if exists at wrong level
```

#### 2.2 Fix Starship Structure

```bash
# Remove duplicate (keep only .config/ version)
rm dotfiles/starship/starship.toml
# Keep: dotfiles/starship/.config/starship.toml
```

#### 2.3 Create Proper Scripts Structure

**NEW STRUCTURE:**
```
dotfiles/scripts/
└── .local/
    └── bin/
        ├── vulcan-menu
        ├── vulcan-power
        ├── vulcan-hotkeys
        └── ... (all scripts)
```

**Implementation:**
```bash
cd /home/evan/VulcanOS/dotfiles
mkdir -p scripts/.local/bin/
mv scripts/*.sh scripts/.local/bin/ 2>/dev/null || true
mv scripts/vulcan-* scripts/.local/bin/ 2>/dev/null || true
mv scripts/opencode-* scripts/.local/bin/ 2>/dev/null || true
mv scripts/docker-* scripts/.local/bin/ 2>/dev/null || true
```

#### 2.4 Create Stow Ignore File

Create `dotfiles/.stow-local-ignore`:
```
\.git
\.gitignore
README.*
LICENSE
\.DS_Store
```

---

## Phase 3: Live System Re-alignment (MEDIUM PRIORITY)

**Objective:** Convert live configs to stow symlinks for automatic syncing

**Backup:** ✅✅✅ **CRITICAL** - Triple backup before this phase!

### Pre-Phase 3 Backup Strategy

**Three backup layers:**
1. **Automatic wrapper backup** (happens first)
2. **Manual full backup** (you create this)
3. **Git commit** (current working state)

**Execute before Phase 3:**
```bash
# Layer 1: Manual full backup with compression
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "critical-before-phase3"

# Layer 2: Git commit current state
cd /home/evan/VulcanOS
git add -A
git commit -m "Checkpoint: Before Phase 3 live system re-alignment"
git push origin main

# Layer 3: Wrapper backup (automatic when phase executes)
```

### Tasks

#### 3.1 Backup Current Live Configs

**NOTE:** This is now handled by the automatic backup wrapper. This section kept for reference only.

~~Manual backup~~ (Automatic via wrapper - no action needed)

#### 3.2 Remove Non-Symlinked Configs

```bash
# Only remove if backup succeeded
rm -rf ~/.config/waybar
rm -rf ~/.config/wofi
rm -rf ~/.config/swaync
rm -rf ~/.config/kitty
rm -rf ~/.config/alacritty
rm -rf ~/.local/bin/vulcan-*
rm -rf ~/.local/bin/opencode-picker
rm -rf ~/.local/bin/docker-mcp-gateway
```

#### 3.3 Stow All Dotfiles

**Execute with backup wrapper:**
```bash
/home/evan/VulcanOS/scripts/auto-backup-wrapper.sh "phase3-stow" bash -c '
cd /home/evan/VulcanOS/dotfiles

# Remove existing configs first (backed up by wrapper)
rm -rf ~/.config/waybar ~/.config/wofi ~/.config/swaync
rm -rf ~/.config/kitty ~/.config/alacritty

# Stow all packages
stow -v -t ~ hypr
stow -v -t ~ waybar
stow -v -t ~ wofi
stow -v -t ~ swaync
stow -v -t ~ kitty
stow -v -t ~ alacritty
stow -v -t ~ nvim
stow -v -t ~ opencode
stow -v -t ~ starship
stow -v -t ~ scripts
stow -v -t ~ git
stow -v -t ~ bash

echo "✅ Stow complete"
'
```

**If anything goes wrong:**
```bash
# Instant rollback
$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh

# Then reload services
hyprctl reload
pkill waybar && waybar &
```

#### 3.4 Verify Symlinks

```bash
ls -la ~/.config/hypr
ls -la ~/.config/waybar
ls -la ~/.config/wofi
ls -la ~/.local/bin/vulcan-menu
```

**Expected Output:**
```
~/.config/hypr -> /home/evan/VulcanOS/dotfiles/hypr/.config/hypr
~/.config/waybar -> /home/evan/VulcanOS/dotfiles/waybar/.config/waybar
~/.local/bin/vulcan-menu -> /home/evan/VulcanOS/dotfiles/scripts/.local/bin/vulcan-menu
```

#### 3.5 Reload Services

```bash
# Reload Hyprland config
hyprctl reload

# Restart Waybar
pkill waybar
sleep 0.5
hyprctl dispatch exec waybar

# Reload swaync
swaync-client -R
swaync-client -rs
```

---

## Phase 4: ISO Synchronization (MEDIUM PRIORITY)

**Objective:** Ensure archiso/airootfs matches dotfiles for new ISO builds

### Tasks

#### 4.1 Sync Scripts to archiso

```bash
# Copy all scripts from dotfiles to archiso
rsync -av --delete \
    /home/evan/VulcanOS/dotfiles/scripts/.local/bin/ \
    /home/evan/VulcanOS/archiso/airootfs/usr/local/bin/

# Make executable
chmod +x /home/evan/VulcanOS/archiso/airootfs/usr/local/bin/*
```

#### 4.2 Sync Configs to /etc/skel

```bash
cd /home/evan/VulcanOS

# Sync each config directory
rsync -av --delete dotfiles/hypr/.config/hypr/ archiso/airootfs/etc/skel/.config/hypr/
rsync -av --delete dotfiles/waybar/.config/waybar/ archiso/airootfs/etc/skel/.config/waybar/
rsync -av --delete dotfiles/wofi/.config/wofi/ archiso/airootfs/etc/skel/.config/wofi/
rsync -av --delete dotfiles/swaync/.config/swaync/ archiso/airootfs/etc/skel/.config/swaync/
rsync -av --delete dotfiles/kitty/.config/kitty/ archiso/airootfs/etc/skel/.config/kitty/
rsync -av --delete dotfiles/alacritty/.config/alacritty/ archiso/airootfs/etc/skel/.config/alacritty/
rsync -av --delete dotfiles/nvim/.config/nvim/ archiso/airootfs/etc/skel/.config/nvim/
rsync -av --delete dotfiles/opencode/.config/opencode/ archiso/airootfs/etc/skel/.config/opencode/

# Sync starship
cp dotfiles/starship/.config/starship.toml archiso/airootfs/etc/skel/.config/starship.toml

# Sync themes
rsync -av --delete dotfiles/themes/ archiso/airootfs/etc/skel/.config/themes/

# Sync git config
cp dotfiles/git/.gitconfig archiso/airootfs/etc/skel/.gitconfig

# Sync bashrc
cp dotfiles/bash/.bashrc archiso/airootfs/etc/skel/.bashrc
```

#### 4.3 Create Sync Script for Future Updates

Create `/home/evan/VulcanOS/scripts/sync-dotfiles-to-iso.sh`:

```bash
#!/bin/bash
# Sync dotfiles to archiso for ISO building
set -e

DOTFILES_DIR="/home/evan/VulcanOS/dotfiles"
ARCHISO_SKEL="/home/evan/VulcanOS/archiso/airootfs/etc/skel"
ARCHISO_BIN="/home/evan/VulcanOS/archiso/airootfs/usr/local/bin"

echo "Syncing dotfiles to archiso..."

# Sync scripts
echo "  → Scripts to /usr/local/bin"
rsync -av --delete "${DOTFILES_DIR}/scripts/.local/bin/" "${ARCHISO_BIN}/"
chmod +x "${ARCHISO_BIN}"/*

# Sync configs to /etc/skel
for app in hypr waybar wofi swaync kitty alacritty nvim opencode; do
    echo "  → ${app} config"
    rsync -av --delete "${DOTFILES_DIR}/${app}/.config/" "${ARCHISO_SKEL}/.config/"
done

# Special cases
echo "  → starship config"
cp "${DOTFILES_DIR}/starship/.config/starship.toml" "${ARCHISO_SKEL}/.config/starship.toml"

echo "  → themes"
rsync -av --delete "${DOTFILES_DIR}/themes/" "${ARCHISO_SKEL}/.config/themes/"

echo "  → git config"
cp "${DOTFILES_DIR}/git/.gitconfig" "${ARCHISO_SKEL}/.gitconfig"

echo "  → bashrc"
cp "${DOTFILES_DIR}/bash/.bashrc" "${ARCHISO_SKEL}/.bashrc"

echo "✅ Sync complete!"
```

Make executable:
```bash
chmod +x /home/evan/VulcanOS/scripts/sync-dotfiles-to-iso.sh
```

---

## Phase 5: Quality of Life Enhancements (LOW PRIORITY)

**Objective:** Add improvements, documentation, and future features

### 5.1 Documentation Updates

#### Create DOTFILES.md

Create `/home/evan/VulcanOS/dotfiles/README.md`:

```markdown
# VulcanOS Dotfiles

User configuration files for VulcanOS, managed with GNU Stow.

## Structure

All configs follow GNU Stow's mirrored directory structure:

```
dotfiles/
├── hypr/.config/hypr/          → ~/.config/hypr/
├── waybar/.config/waybar/       → ~/.config/waybar/
├── scripts/.local/bin/          → ~/.local/bin/
└── ... (etc)
```

## Installation

From the VulcanOS repository root:

```bash
cd dotfiles
stow -v -t ~ hypr waybar wofi kitty scripts
```

## Updating Live System

After editing configs in this directory, changes apply immediately via symlinks.

## Building ISO

Run the sync script to copy to archiso:

```bash
../scripts/sync-dotfiles-to-iso.sh
```

## Scripts

All Vulcan system scripts are in `scripts/.local/bin/`:

- `vulcan-menu` - Main system menu
- `vulcan-power` - Power/session menu
- `vulcan-hotkeys` - Keybinding help
- `vulcan-theme` - Theme switcher
- `vulcan-wallpapers` - Wallpaper manager
- `vulcan-screenshot` - Screenshot utility
- `vulcan-idle` - Idle/lock toggle
- `vulcan-copy-paste` - Universal clipboard

## Theme System

Theme files in `themes/`:
- `colors/` - Theme definitions
- `templates/` - Config templates

Apply themes with: `vulcan-theme set <theme-name>`
```

#### Update CLAUDE.md

Add to the "Configuration Structure" section in `/home/evan/VulcanOS/CLAUDE.md`:

```markdown
### Configuration Flow

1. **Development**: Edit files in `dotfiles/*/`
2. **Live System**: Changes apply immediately via stow symlinks
3. **ISO Building**: Run `scripts/sync-dotfiles-to-iso.sh` to sync to archiso
4. **Distribution**: ISO users get fresh copy from `/etc/skel/`

**Critical Rule**: NEVER edit files in `~/.config/` directly if they're symlinks. Always edit the source in `dotfiles/`.
```

### 5.2 Add Verification Script

Create `/home/evan/VulcanOS/scripts/verify-vulcan.sh`:

```bash
#!/bin/bash
# VulcanOS Configuration Verification Script

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔍 VulcanOS Configuration Verification"
echo "========================================"
echo ""

# Check scripts in PATH
echo "📜 Checking Vulcan Scripts..."
SCRIPTS=("vulcan-menu" "vulcan-power" "vulcan-hotkeys" "vulcan-theme" 
         "vulcan-wallpapers" "vulcan-screenshot" "vulcan-idle" "vulcan-copy-paste")

for script in "${SCRIPTS[@]}"; do
    if command -v "$script" &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} $script"
    else
        echo -e "  ${RED}✗${NC} $script (MISSING)"
    fi
done

echo ""

# Check symlinks
echo "🔗 Checking Stow Symlinks..."
CONFIGS=("hypr" "waybar" "wofi" "swaync" "kitty")

for config in "${CONFIGS[@]}"; do
    if [[ -L "$HOME/.config/$config" ]]; then
        target=$(readlink "$HOME/.config/$config")
        if [[ "$target" == *"VulcanOS/dotfiles"* ]]; then
            echo -e "  ${GREEN}✓${NC} ~/.config/$config → $target"
        else
            echo -e "  ${YELLOW}⚠${NC} ~/.config/$config → $target (unexpected target)"
        fi
    elif [[ -d "$HOME/.config/$config" ]]; then
        echo -e "  ${YELLOW}⚠${NC} ~/.config/$config (directory, not symlink)"
    else
        echo -e "  ${RED}✗${NC} ~/.config/$config (missing)"
    fi
done

echo ""

# Check themes
echo "🎨 Checking Theme System..."
if [[ -d "$HOME/.config/themes" ]]; then
    theme_count=$(ls -1 "$HOME/.config/themes/colors" 2>/dev/null | wc -l)
    echo -e "  ${GREEN}✓${NC} Themes directory exists ($theme_count themes)"
    
    if [[ -f "$HOME/.config/vulcan/current-theme" ]]; then
        current=$(cat "$HOME/.config/vulcan/current-theme")
        echo -e "  ${GREEN}✓${NC} Current theme: $current"
    else
        echo -e "  ${YELLOW}⚠${NC} No current theme set"
    fi
else
    echo -e "  ${RED}✗${NC} Themes directory missing"
fi

echo ""

# Check running services
echo "🔧 Checking Services..."
SERVICES=("Hyprland" "waybar" "swaync" "hypridle" "hyprpaper")

for service in "${SERVICES[@]}"; do
    if pgrep -x "$service" &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} $service is running"
    else
        echo -e "  ${YELLOW}⚠${NC} $service is not running"
    fi
done

echo ""
echo "✅ Verification complete!"
```

Make executable:
```bash
chmod +x /home/evan/VulcanOS/scripts/verify-vulcan.sh
```

### 5.3 Future Feature Ideas

#### Immediate Enhancements (Next Session)

1. **Monitor Configuration Enhancement**
   - Add `hyprmon` integration to vulcan-menu
   - Create persistent monitor profiles
   - Add hotkey for quick monitor switching

2. **Wallpaper System v2**
   - Per-monitor wallpaper profiles
   - Automatic profile switching based on connected monitors
   - Integration with `vulcan-wallpapers` script

3. **Theme System v2**
   - Add theme preview before applying
   - Create custom theme builder wizard
   - Export/import theme files

4. **Backup/Restore System**
   - Add "Restore Defaults" functionality to vulcan-menu
   - Automatic config backups before theme changes
   - Snapshot system for config versioning

#### Advanced Features (Future)

5. **VulcanOS Control Center GUI**
   - GTK/Qt GUI for all vulcan-menu features
   - Visual theme editor
   - Monitor configuration with live preview
   - Package manager integration

6. **Cloud Sync Integration**
   - Sync themes/wallpapers across machines
   - GitHub Gist integration for config sharing
   - Dotfile version control integration

7. **Plugin System**
   - Community theme repository
   - Script marketplace
   - Extension API for custom tools

8. **Smart Automation**
   - Auto-detect monitor setup and apply profile
   - Time-based theme switching (day/night)
   - Location-based wallpaper rotation
   - Workspace-specific themes

9. **Enhanced Notifications**
   - System health monitoring alerts
   - Update notifications with changelogs
   - Customizable notification rules

10. **Developer Tools Integration**
    - IDE theme synchronization
    - Terminal multiplexer configs
    - Git workflow shortcuts in menu

---

## Risk Assessment

| Phase | Risk Level | Mitigation |
|-------|-----------|------------|
| Phase 1 | 🟢 LOW | Scripts copied from working archiso, no system changes |
| Phase 2 | 🟡 MEDIUM | Stow structure changes, test with `stow -n` first |
| Phase 3 | 🟡 MEDIUM | Backup created before removing configs |
| Phase 4 | 🟢 LOW | Only affects future ISO builds |
| Phase 5 | 🟢 LOW | Documentation and optional features |

**Rollback Plan:**
- Backups saved to `~/VulcanOS-live-backup/$(date)`
- Can unstow: `stow -D -v -t ~ <package>`
- Can restore: `cp -r ~/VulcanOS-live-backup/YYYYMMDD/waybar ~/.config/`

---

## Success Criteria

### Phase 1 ✅
- [ ] All vulcan-* scripts exist in dotfiles/scripts/
- [ ] All scripts executable and in PATH
- [ ] `Super+Escape` opens power menu
- [ ] `Super+Alt+Space` opens system menu
- [ ] `Super+K` shows hotkeys
- [ ] `Super+Ctrl+Shift+Space` opens theme selector
- [ ] Screenshots work with all keybindings

### Phase 2 ✅
- [ ] All dotfiles follow proper stow structure
- [ ] No duplicate config files
- [ ] Stow dry-run succeeds: `stow -n -v -t ~ *`

### Phase 3 ✅
- [ ] All configs in ~/.config/ are symlinks to VulcanOS/dotfiles/
- [ ] Waybar displays correctly
- [ ] Wofi launcher works
- [ ] Theme changes apply immediately
- [ ] No broken services after reload

### Phase 4 ✅
- [ ] archiso scripts match dotfiles scripts
- [ ] archiso /etc/skel/ matches dotfiles configs
- [ ] sync script exists and works
- [ ] Test ISO build succeeds

### Phase 5 ✅
- [ ] Documentation complete and accurate
- [ ] Verification script runs without errors
- [ ] All features tested and working

---

## Timeline

**Estimated Total Time: 2-3 hours**

- Phase 1: 30 minutes (script copying and deployment)
- Phase 2: 30 minutes (stow restructuring)
- Phase 3: 45 minutes (live system re-alignment)
- Phase 4: 30 minutes (ISO synchronization)
- Phase 5: 45 minutes (documentation and enhancements)

---

## Orchestrator Execution Command

```bash
@orchestrator Execute VulcanOS Configuration Alignment Plan

Follow the plan document at /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md

Execute phases 1-4 sequentially. Phase 5 can be parallelized.

Requirements:
- Create backups before destructive operations
- Verify each phase before proceeding
- Run verification script after completion
- Report any deviations from expected behavior

Context files to reference:
- /home/evan/VulcanOS/CLAUDE.md
- /home/evan/VulcanOS/dotfiles/
- /home/evan/VulcanOS/archiso/
- ~/.config/ (live system)
```

---

## Post-Implementation

### Testing Checklist

After orchestrator completes all phases:

1. **Keybinding Tests:**
   - [ ] `Super+Escape` → Power menu appears
   - [ ] `Super+Alt+Space` → Vulcan menu appears
   - [ ] `Super+K` → Hotkeys display appears
   - [ ] `Super+Ctrl+Shift+Space` → Theme selector appears
   - [ ] `Super+Ctrl+Space` → Wallpaper rotates
   - [ ] `Super+Ctrl+S` → Full screen screenshot
   - [ ] `Super+Alt+S` → Window screenshot
   - [ ] `Super+Shift+S` → Region screenshot with Swappy

2. **Menu Navigation:**
   - [ ] Vulcan menu → System submenu works
   - [ ] Vulcan menu → Style submenu works
   - [ ] Vulcan menu → Install submenu works
   - [ ] Vulcan menu → Update submenu works
   - [ ] Vulcan menu → T2 MacBook submenu works
   - [ ] Power menu → All options work

3. **Configuration Sync:**
   - [ ] Edit `dotfiles/waybar/.config/waybar/config.jsonc` → Waybar updates
   - [ ] Edit `dotfiles/hypr/.config/hypr/bindings.conf` → Hyprland reloads
   - [ ] Change theme → All apps update

4. **ISO Build Test:**
   - [ ] Run `scripts/sync-dotfiles-to-iso.sh`
   - [ ] Build ISO: `scripts/build.sh`
   - [ ] Boot ISO in QEMU
   - [ ] Verify all scripts present in ISO

### Final Verification

Run the verification script:
```bash
/home/evan/VulcanOS/scripts/verify-vulcan.sh
```

Expected output: All green checkmarks ✅

---

## Notes for Future Maintenance

1. **Workflow for Config Changes:**
   ```
   Edit dotfiles/ → Changes apply live via symlinks → Test → Commit to git
   ```

2. **Workflow for ISO Releases:**
   ```
   Test dotfiles on live system → Run sync-dotfiles-to-iso.sh → Build ISO → Test in VM → Release
   ```

3. **Adding New Scripts:**
   ```
   Create in dotfiles/scripts/.local/bin/ → Make executable → Test → Sync to archiso
   ```

4. **Adding New Configs:**
   ```
   Create in dotfiles/<app>/.config/<app>/ → Stow → Test → Sync to archiso/etc/skel/
   ```

---

**END OF PLAN**
