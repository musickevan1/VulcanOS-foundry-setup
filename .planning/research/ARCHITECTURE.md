# Architecture Patterns: System Backup Integration

**Domain:** Linux System Backup Integration for Desktop Environments
**Researched:** 2026-01-23
**Confidence:** HIGH

## Recommended Architecture

VulcanOS backup integration should follow a modular, event-driven architecture that hooks into the package manager, desktop environment, and user workflows without disrupting the existing system.

### Component Boundaries

| Component | Responsibility | Communicates With | Location |
|-----------|---------------|-------------------|----------|
| Pacman Hooks | Trigger backups on system changes | Backup Engine, Notification System | `/etc/pacman.d/hooks/*.hook` |
| Backup Engine | Execute backup operations | Storage Backend, Hook System | `/usr/local/bin/vulcan-backup` |
| Menu Integration | User-facing backup controls | Backup Engine, Wofi | `dotfiles/scripts/.local/bin/vulcan-menu` |
| Waybar Module | Backup status display | Backup Engine (via state files) | Custom waybar module config |
| Notification Layer | User feedback | Backup Engine, swaync | Via `notify-send` |
| Configuration | Backup profiles, schedules | All components | `~/.config/vulcan-backup/` |

### Data Flow

```
User Action (pacman -Syu)
    ↓
Pacman Transaction Starts
    ↓
PreTransaction Hook → Backup Engine → Storage Backend
    ↓                        ↓
Pacman Upgrade         Notify User (pre-backup complete)
    ↓
PostTransaction Hook → Backup Engine → Storage Backend
    ↓                        ↓
Transaction Complete   Notify User (post-backup complete)
                            ↓
                      Waybar Module (update status)
```

**Manual Workflow:**
```
User opens vulcan-menu
    ↓
Selects "System" → "Backup"
    ↓
Wofi submenu shows: [Create Backup | Restore | Manage | Schedule]
    ↓
User selects action → Backup Engine executes → Notification feedback
```

## Patterns to Follow

### Pattern 1: Pacman Hook Integration

**What:** Use alpm-hooks to trigger backups at transaction boundaries.

**When:** Critical system changes (kernel updates, major packages).

**File Structure:**
```
/etc/pacman.d/hooks/
├── 00-vulcan-pre-backup.hook    # PreTransaction (runs first, alphabetical)
└── 99-vulcan-post-backup.hook   # PostTransaction (runs last)
```

**Example Hook:**
```ini
[Trigger]
Operation = Upgrade
Type = Package
Target = linux*
Target = systemd
Target = grub

[Action]
Description = Creating pre-upgrade system backup...
When = PreTransaction
Exec = /usr/local/bin/vulcan-backup auto pre
Depends = rsync
NeedsTargets
```

**Key Points:**
- Use numeric prefixes (00-, 99-) to control execution order
- `NeedsTargets` passes matched package names to script via stdin
- `Depends` ensures backup tool is installed before hook runs
- `AbortOnFail` (PreTransaction only) can halt upgrades if backup fails

**Sources:**
- [Arch Linux Pacman Wiki - Hooks](https://wiki.archlinux.org/title/Pacman)
- [alpm-hooks(5) manual](https://man.archlinux.org/man/alpm-hooks.5)

---

### Pattern 2: Wofi Submenu Integration

**What:** Nested menu system using wofi's dmenu mode for backup controls.

**When:** User wants manual backup operations through GUI.

**Integration Point:** `dotfiles/scripts/.local/bin/vulcan-menu` (existing file)

**Example Pattern:**
```bash
show_backup_menu() {
    local options="󰆓  Create Backup
󰁯  Restore Backup
󰋖  List Backups
󰃰  Schedule Backup
󰛖  Settings
$ICON_BACK  Back"

    local choice
    choice=$(wofi_menu "Backup Manager" "$options")

    local action
    action=$(echo "$choice" | sed 's/^[^ ]* *//' | tr '[:upper:]' '[:lower:]')

    case "$action" in
        "create backup")  create_backup_interactive ;;
        "restore backup") restore_backup_interactive ;;
        "list backups")   list_backups_interactive ;;
        "schedule backup") schedule_backup_interactive ;;
        "settings")       edit_backup_config ;;
        "back")           show_main_menu ;;
        *)                exit 0 ;;
    esac
}
```

**Location in vulcan-menu:**
Add to main menu options (around line 64-72):
```bash
$ICON_BACKUP  Backup    # New entry
```

Add case handler (around line 80-91):
```bash
"backup")  show_backup_menu ;;
```

**Key Points:**
- Follow existing VulcanOS menu patterns (see vulcan-power, vulcan-menu)
- Use Nerd Font icons for consistency
- Wofi dmenu mode: `wofi --dmenu --prompt "..." --cache-file /dev/null`
- Return to parent menu with "Back" option
- Extract action by removing icon prefix: `sed 's/^[^ ]* *//'`

**Sources:**
- [Hyprland Wiki - App Launchers](https://wiki.hypr.land/Useful-Utilities/App-Launchers/)
- [wofi manual page](https://sr.ht/~scoopta/wofi/)

---

### Pattern 3: Waybar Module Integration

**What:** Custom Waybar module displaying backup status and last backup time.

**When:** User wants persistent backup status visibility.

**Configuration:** `dotfiles/waybar/.config/waybar/config.jsonc`

**Example Module:**
```json
"custom/backup": {
    "format": "{icon} {}",
    "format-icons": {
        "idle": "󰆓",
        "running": "󰦖",
        "error": "󰅙",
        "success": "󰄬"
    },
    "return-type": "json",
    "exec": "/home/evan/.local/bin/vulcan-backup-status",
    "interval": 60,
    "on-click": "vulcan-menu backup",
    "tooltip": true,
    "exec-on-event": true
}
```

**Status Script Pattern:**
```bash
#!/bin/bash
# vulcan-backup-status - Waybar custom module output

STATE_FILE="$HOME/.config/vulcan-backup/state"
LAST_BACKUP_FILE="$HOME/.config/vulcan-backup/last_backup"

if [[ -f "$STATE_FILE" ]]; then
    state=$(cat "$STATE_FILE")
else
    state="idle"
fi

if [[ -f "$LAST_BACKUP_FILE" ]]; then
    last_backup=$(cat "$LAST_BACKUP_FILE")
    tooltip="Last backup: $last_backup"
else
    tooltip="No backups yet"
fi

# Output JSON for Waybar
cat <<EOF
{
    "text": "",
    "tooltip": "$tooltip",
    "class": "$state"
}
EOF
```

**CSS Styling:** `dotfiles/waybar/.config/waybar/style.css`
```css
#custom-backup.idle { color: #a6adc8; }
#custom-backup.running { color: #f9e2af; }
#custom-backup.success { color: #a6e3a1; }
#custom-backup.error { color: #f38ba8; }
```

**Key Points:**
- Use `return-type: json` for structured output
- `interval`: seconds between polls (60 = every minute)
- `exec-on-event: true` refreshes after click events
- State files in `~/.config/vulcan-backup/` for persistence
- Follow existing VulcanOS Waybar patterns (see hyprwhspr module, lines 68-75)

**Sources:**
- [Waybar Custom Module Documentation](https://man.archlinux.org/man/extra/waybar/waybar-custom.5.en)
- [Waybar Custom Scripts Guide](https://waybar.org/can-i-add-custom-scripts-to-waybar/)

---

### Pattern 4: Script Organization

**What:** Logical separation of user scripts vs system scripts.

**When:** All VulcanOS tools and utilities.

**Directory Structure:**
```
# User scripts (symlinked via GNU Stow)
dotfiles/scripts/.local/bin/
├── vulcan-backup           # Main backup CLI
├── vulcan-backup-status    # Waybar status generator
└── vulcan-menu             # Modified to include backup submenu

# System scripts (installed to ISO, used by hooks)
archiso/airootfs/usr/local/bin/
├── vulcan-backup           # Copy of user script for system-wide access
└── (other system utilities)

# Pacman hooks
archiso/airootfs/etc/pacman.d/hooks/
├── 00-vulcan-pre-backup.hook
└── 99-vulcan-post-backup.hook

# Configuration
~/.config/vulcan-backup/
├── config.toml             # User settings
├── profiles/               # Backup profiles
├── state                   # Current operation state
└── last_backup             # Timestamp of last successful backup
```

**Installation Flow:**
1. **Development:** Edit in `dotfiles/scripts/.local/bin/`
2. **Testing:** Stow to `~/.local/bin/` for live testing
3. **ISO Integration:** Copy to `archiso/airootfs/usr/local/bin/` and hooks

**Key Points:**
- User scripts in `~/.local/bin` (via Stow, personal use)
- System scripts in `/usr/local/bin` (system-wide, untouched by pacman)
- NEVER put custom scripts in `/usr/bin` (pacman-managed)
- Configuration in `~/.config/` (XDG Base Directory spec)
- State files in `~/.config/vulcan-backup/` (not `~/.local/share` - easier access)

**Sources:**
- [Arch Linux Forums - Script Organization](https://bbs.archlinux.org/viewtopic.php?id=165042)
- [FHS - Filesystem Hierarchy Standard](https://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard)

---

### Pattern 5: Notification Integration

**What:** User feedback via swaync during backup operations.

**When:** All backup events (start, progress, completion, errors).

**Example Pattern:**
```bash
notify_backup() {
    local title="$1"
    local message="$2"
    local urgency="${3:-normal}"
    local icon="${4:-drive-harddisk}"

    if command -v notify-send &> /dev/null; then
        notify-send "$title" "$message" \
            -i "$icon" \
            -u "$urgency" \
            -t 5000 \
            -a "VulcanOS Backup"
    fi
}

# Usage examples
notify_backup "Backup Started" "Creating system backup..." "normal" "document-save"
notify_backup "Backup Complete" "Backup saved to /mnt/backups" "normal" "emblem-default"
notify_backup "Backup Failed" "Insufficient disk space" "critical" "dialog-error"
```

**Integration with Hooks:**
```bash
# In hook script
/usr/local/bin/vulcan-backup auto pre 2>&1 | while read -r line; do
    notify_backup "Pre-Upgrade Backup" "$line"
done
```

**Key Points:**
- Use `-a "VulcanOS Backup"` for notification grouping in swaync
- Standard urgency levels: `low`, `normal`, `critical`
- Timeout: 5000ms (5 seconds) for non-critical
- Icon names: use freedesktop icon naming spec
- Follow existing VulcanOS patterns (see vulcan-power, vulcan-menu)

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Blocking Pacman Transactions Indefinitely

**What:** Long-running backup operations that freeze package manager.

**Why bad:** User can't cancel, system appears hung, poor UX.

**Instead:**
- Use PostTransaction hooks for full backups (non-blocking)
- PreTransaction hooks should be fast (metadata snapshots only)
- Set reasonable timeouts in backup scripts
- Provide cancellation mechanisms

---

### Anti-Pattern 2: Hard-Coding Paths

**What:** Absolute paths to backup destinations, user directories.

**Why bad:** Breaks on different mount points, multi-user systems.

**Instead:**
- Use configuration files: `~/.config/vulcan-backup/config.toml`
- Environment variables: `${HOME}`, `${USER}`
- Detect mount points dynamically: `findmnt`, `lsblk`

---

### Anti-Pattern 3: Silent Failures

**What:** Backup fails but user isn't notified.

**Why bad:** False sense of security, data loss risk.

**Instead:**
- Always notify on error (critical urgency)
- Log to systemd journal: `systemd-cat`
- Exit with non-zero status codes
- Use `AbortOnFail` in PreTransaction hooks for critical backups

---

### Anti-Pattern 4: Multiple Menu Systems

**What:** Creating separate GUI tools instead of integrating with vulcan-menu.

**Why bad:** Fragmented UX, inconsistent styling, maintenance burden.

**Instead:**
- Extend existing `vulcan-menu` with submenu
- Follow established VulcanOS patterns
- Use same wofi configuration and styling
- Consistent icon usage (Nerd Fonts)

---

### Anti-Pattern 5: Root-Owned User Configurations

**What:** Backup configs in `/etc` or owned by root when user-specific.

**Why bad:** Permission issues, can't customize per-user.

**Instead:**
- System defaults in `/etc/vulcan-backup/` (if needed)
- User configs in `~/.config/vulcan-backup/` (XDG spec)
- Copy system defaults to user config on first run
- Use appropriate permissions (644 for configs, 755 for scripts)

---

## Build Order & Dependencies

### Phase 1: Core Infrastructure
1. Backup engine script (`vulcan-backup`)
2. Configuration schema (`config.toml`)
3. State management (state files)

**Dependencies:**
- rsync (table stakes)
- OR borg/restic (if chosen)
- notify-send (swaync)

### Phase 2: Desktop Integration
1. Waybar module (`vulcan-backup-status`)
2. Waybar config additions
3. CSS styling

**Dependencies:**
- Phase 1 complete
- Waybar installed
- State files working

### Phase 3: Menu Integration
1. Modify `vulcan-menu` (add backup submenu)
2. Interactive functions
3. Wofi integration

**Dependencies:**
- Phase 1 complete
- vulcan-menu patterns established
- wofi installed

### Phase 4: Automation
1. Pacman hooks (pre/post transaction)
2. Optional: systemd timers for scheduled backups
3. ISO integration

**Dependencies:**
- Phase 1-3 tested
- Backup engine stable
- Hook syntax validated

---

## Integration Points: VulcanOS Specifics

### 1. Existing Hook System
**Status:** NONE - No pacman hooks currently exist in VulcanOS.

**Opportunity:** Clean slate, establish patterns from scratch.

**Location:** Create `archiso/airootfs/etc/pacman.d/hooks/` directory.

---

### 2. Waybar Configuration
**Current File:** `dotfiles/waybar/.config/waybar/config.jsonc`

**Modules-right current:** `["tray", "custom/notification", "bluetooth", "network", "pulseaudio", "cpu", "memory", "battery"]`

**Proposed:** Insert `"custom/backup"` after `"memory"`, before `"battery"`.

**Existing Custom Modules:**
- `custom/separator` (line 62-66)
- `custom/hyprwhspr` (line 68-75) - **Good reference pattern**
- `custom/notification` (line 118-138)

**Pattern to follow:** `custom/hyprwhspr` uses:
- JSON return type
- 1-second interval
- Exec-on-event: true
- Tooltip enabled
- External script for status

---

### 3. Menu System
**Current File:** `dotfiles/scripts/.local/bin/vulcan-menu`

**Main Menu Options (lines 64-72):**
```
System, Style, Quick Settings, Install, Remove, Setup, Update, T2 MacBook, Power
```

**Proposed Insertion:** Add "Backup" between "System" and "Style".

**Existing Patterns:**
- Icon prefix with Nerd Fonts
- Submenu functions: `show_system_menu()`, `show_style_menu()`, etc.
- Wofi menu helper: `wofi_menu()` function (lines 43-57)
- Notification helper: `notify()` function (lines 28-32)
- Terminal execution: `run_in_terminal()`, `run_in_terminal_hold()` (lines 34-40)

**Pattern to replicate:** Follow `show_system_menu()` (lines 464-504) as template.

---

### 4. Script Organization
**Current Structure:**
- User scripts: `dotfiles/scripts/.local/bin/` (29 scripts)
- System scripts: `archiso/airootfs/usr/local/bin/` (19 scripts)
- Naming convention: `vulcan-*` prefix

**Existing Patterns:**
- `vulcan-menu` - Multi-level menu system (1,642 lines)
- `vulcan-power` - Simple action menu (157 lines)
- `vulcan-wallpaper` - Resource management (~200 lines)
- `vulcan-theme` - Configuration switcher

**Backup Script Should:**
- Use `vulcan-backup` naming
- Include `--help` flag
- Support both CLI and interactive modes
- Output JSON for Waybar integration
- Follow VulcanOS script header format

---

## Scalability Considerations

| Concern | Initial (Single User) | Multi-Monitor Desktop | Network Backups |
|---------|----------------------|----------------------|-----------------|
| **Storage** | Local disk (rsync) | NAS mount (rsync) | Remote (borg/restic) |
| **Automation** | Pacman hooks only | + systemd timers | + remote scheduling |
| **Restoration** | Manual via menu | Profile-based | Remote pull |
| **Monitoring** | Waybar module | + Desktop notifications | + Email/webhook |
| **Encryption** | Optional (LUKS disk) | Optional | Required (borg/restic) |

---

## Architecture Recommendations Summary

1. **Hook-Driven Architecture:** Use pacman hooks as primary automation trigger
2. **Modular Design:** Separate backup engine, UI, and automation
3. **State-Based UI:** Waybar reads state files, doesn't execute backups
4. **Event Notifications:** All operations notify via swaync
5. **Configuration Hierarchy:** System defaults + user overrides
6. **Script Location:** User scripts in `~/.local/bin`, system in `/usr/local/bin`
7. **Integration Pattern:** Extend existing vulcan-menu, don't create new tools

**Critical Success Factors:**
- Fast PreTransaction hooks (< 5 seconds)
- Clear user feedback at all stages
- Non-blocking PostTransaction operations
- Consistent with VulcanOS UX patterns
- Logged errors for debugging

---

## Sources

**Official Documentation (HIGH confidence):**
- [Arch Linux Pacman Wiki](https://wiki.archlinux.org/title/Pacman)
- [alpm-hooks(5) manual page](https://man.archlinux.org/man/alpm-hooks.5)
- [Waybar Custom Module Documentation](https://man.archlinux.org/man/extra/waybar/waybar-custom.5.en)
- [Hyprland Wiki - App Launchers](https://wiki.hypr.land/Useful-Utilities/App-Launchers/)

**Community Resources (MEDIUM confidence):**
- [Arch Linux Forums - Pacman Hook Examples](https://bbs.archlinux.org/viewtopic.php?id=289248)
- [GitHub - desbma/pacman-hooks](https://github.com/desbma/pacman-hooks)
- [Waybar Custom Scripts Guide](https://waybar.org/can-i-add-custom-scripts-to-waybar/)
- [Arch Linux Forums - Script Organization](https://bbs.archlinux.org/viewtopic.php?id=165042)

**Backup Tool Comparisons (MEDIUM confidence):**
- [Backup Speed Benchmark: rsync vs borg vs restic](https://grigio.org/backup-speed-benchmark/)
- [Restic vs Borg Comparison](https://faisalrafique.com/restic-vs-borg/)
- [GitHub - restic/others (Exhaustive Backup List)](https://github.com/restic/others)
