# VulcanOS Orchestrator - Pre-Flight Checklist

**Session:** Configuration Alignment and Repair  
**Date:** 2026-01-01

## Before Starting Orchestrator

### ✅ Prerequisites Check

- [ ] You are logged into your VulcanOS system
- [ ] You have sudo/root access
- [ ] You have at least 5GB free disk space
- [ ] You have a backup of critical data (orchestrator will create config backups)
- [ ] No critical work is in progress (system will be modified)
- [ ] Internet connection is stable (in case packages need updating)

### ✅ Current State Verification

Run these commands to verify current state:

```bash
# Check which scripts are missing
which vulcan-menu || echo "❌ MISSING"
which vulcan-power || echo "❌ MISSING"  
which vulcan-hotkeys || echo "❌ MISSING"

# Check current config structure
ls -la ~/.config/waybar | head -1
ls -la ~/.config/wofi | head -1
# If these show "drwx" (directory), they need converting to symlinks

# Check VulcanOS repo exists
ls -la ~/VulcanOS/dotfiles/
ls -la ~/VulcanOS/archiso/

# Check git status (should be clean or known changes)
cd ~/VulcanOS && git status
```

### ✅ Understanding What Will Happen

**Phase 1 - Scripts (SAFE):**
- Copies scripts from archiso → dotfiles
- Symlinks to ~/.local/bin
- NO destructive changes
- Keybindings start working

**Phase 2 - Structure (SAFE):**
- Reorganizes dotfiles directory
- NO changes to live system yet
- Prepares for stow

**Phase 3 - Live System (REQUIRES BACKUP):**
- Backs up current configs to ~/VulcanOS-live-backup/
- Removes ~/.config/{waybar,wofi,etc}
- Creates stow symlinks
- Services reload

**Phase 4 - ISO (SAFE):**
- Only touches archiso directory
- Prepares for future ISO builds
- NO live system changes

**Phase 5 - Documentation (SAFE):**
- Creates scripts and docs
- NO system changes

### ✅ Backup Confirmation

The orchestrator will create backups automatically, but verify backup directory exists:

```bash
mkdir -p ~/VulcanOS-live-backup/
ls -la ~/VulcanOS-live-backup/
```

### ✅ Test Current Broken Features

Before starting, confirm what's broken:

| Keybinding | Expected | Current Status |
|------------|----------|----------------|
| `Super+Escape` | Power menu | ❌ (test it) |
| `Super+Alt+Space` | System menu | ❌ (test it) |
| `Super+K` | Hotkeys | ❌ (test it) |
| `Super+Ctrl+Shift+Space` | Theme selector | ❌ (test it) |

This helps verify the fix worked afterward.

## Launching Orchestrator

### Method 1: OpenCode (Recommended)

```bash
opencode
```

Then paste this prompt:

```
@orchestrator Execute the VulcanOS Configuration Alignment Plan

Plan location: /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md

Execute phases 1-5 sequentially with the following requirements:

1. Create backups before ANY destructive operations
2. Verify each phase completes successfully before proceeding
3. If a phase fails, STOP and report the issue
4. Run verification after Phase 3 to ensure live system works
5. Report progress after each phase

Phase execution order:
- Phase 1: Emergency Script Rescue (30 min)
- Phase 2: Stow Structure Standardization (30 min)  
- Phase 3: Live System Re-alignment (45 min) ⚠️ BACKUP REQUIRED
- Phase 4: ISO Synchronization (30 min)
- Phase 5: Documentation & Enhancements (45 min)

Expected total time: 2-3 hours

Begin with Phase 1.
```

### Method 2: Manual Execution (If Orchestrator Unavailable)

Follow the plan step-by-step manually:

```bash
cd ~/VulcanOS
cat .orchestrator/vulcanos-config-alignment.md

# Execute each phase's commands in order
```

## During Execution

### What to Watch For

✅ **Good Signs:**
- Orchestrator reports "Phase X complete"
- Scripts appear in `which vulcan-menu`
- Symlinks created successfully
- Services reload without errors

⚠️ **Warning Signs:**
- Permission denied errors → May need sudo
- Stow conflicts → Might have unexpected files
- Service restart failures → Check logs

🛑 **Stop Immediately If:**
- Data loss warnings appear
- Multiple consecutive errors
- System becomes unresponsive

### Monitoring Progress

Open a second terminal and monitor:

```bash
# Watch orchestrator session logs
tail -f ~/.opencode/logs/orchestrator-session-*.log

# Watch for errors in Hyprland
journalctl -f -u hyprland

# Check waybar status
systemctl --user status waybar
```

## After Completion

### ✅ Post-Flight Verification

1. **Run verification script:**
```bash
/home/evan/VulcanOS/scripts/verify-vulcan.sh
```

Expected: All green checkmarks ✅

2. **Test all previously broken keybindings:**
   - `Super+Escape` → Power menu ✅
   - `Super+Alt+Space` → System menu ✅
   - `Super+K` → Hotkeys display ✅
   - `Super+Ctrl+Shift+Space` → Theme selector ✅
   - `Super+Ctrl+S` → Screenshot ✅

3. **Verify symlinks:**
```bash
ls -la ~/.config/hypr
ls -la ~/.config/waybar
ls -la ~/.config/wofi
ls -la ~/.local/bin/vulcan-menu
```

All should show `->` pointing to `/home/evan/VulcanOS/dotfiles/`

4. **Test live config editing:**
```bash
# Edit a config
vim ~/VulcanOS/dotfiles/waybar/.config/waybar/config.jsonc
# Make a small change (like height)

# Restart waybar
pkill waybar && hyprctl dispatch exec waybar

# Verify change applied
```

5. **Check backup created:**
```bash
ls -la ~/VulcanOS-live-backup/$(date +%Y%m%d)/
```

Should contain: waybar, wofi, swaync, kitty, alacritty

### ✅ Test ISO Sync

```bash
# Run sync script
/home/evan/VulcanOS/scripts/sync-dotfiles-to-iso.sh

# Verify archiso updated
ls -la ~/VulcanOS/archiso/airootfs/usr/local/bin/vulcan-*
ls -la ~/VulcanOS/archiso/airootfs/etc/skel/.config/waybar/
```

### ✅ Commit Changes

```bash
cd ~/VulcanOS
git status
git add dotfiles/ archiso/ scripts/ .orchestrator/
git commit -m "Fix: Align VulcanOS configs - restore broken scripts and establish stow symlinks

- Phase 1: Deployed all missing vulcan-* scripts to dotfiles/scripts
- Phase 2: Standardized dotfiles structure for GNU Stow
- Phase 3: Converted live configs to stow symlinks for auto-sync
- Phase 4: Synced dotfiles to archiso for ISO builds
- Phase 5: Added documentation and verification tools

All keybindings now functional. Live system syncs with repo."

git push
```

## Rollback Plan (If Something Goes Wrong)

### Immediate Rollback

```bash
# Unstow all packages
cd ~/VulcanOS/dotfiles
stow -D -v -t ~ hypr waybar wofi swaync kitty alacritty nvim opencode starship scripts

# Restore from backup
cp -r ~/VulcanOS-live-backup/$(date +%Y%m%d)/* ~/.config/

# Restart services
hyprctl reload
pkill waybar && waybar &
```

### Partial Rollback (Per Package)

```bash
# Unstow just waybar
cd ~/VulcanOS/dotfiles
stow -D -v -t ~ waybar

# Restore waybar from backup
cp -r ~/VulcanOS-live-backup/$(date +%Y%m%d)/waybar ~/.config/

# Restart waybar
pkill waybar && waybar &
```

## Success Criteria

After completion, you should have:

- ✅ All vulcan-* scripts working
- ✅ All keybindings functional
- ✅ Live configs are symlinks to dotfiles/
- ✅ Editing dotfiles/ immediately updates live system
- ✅ archiso/ synced with dotfiles/
- ✅ Verification script passes
- ✅ Documentation complete
- ✅ Backup created and verified

## Questions Before Starting?

Review the full plan:
```bash
less /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md
```

Review the quick start:
```bash
less /home/evan/VulcanOS/QUICKSTART-ORCHESTRATOR.md
```

## Ready to Launch?

When ready, start the orchestrator with the prompt above.

Estimated time: **2-3 hours**

Good luck! 🚀

---

**Pre-Flight Checklist Complete** ✅
