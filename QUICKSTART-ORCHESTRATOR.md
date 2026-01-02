# VulcanOS Configuration Fix - Quick Start

## TL;DR

Your VulcanOS has broken keybindings because critical scripts are missing from your PATH. This orchestrator session will fix everything in ~2 hours.

## What's Broken

- `Super+Escape` (power menu) ❌
- `Super+Alt+Space` (system menu) ❌
- `Super+K` (hotkeys help) ❌
- `Super+Ctrl+Shift+Space` (theme selector) ❌
- Screenshot keybindings ❌

## Quick Fix (If You Just Want It Working NOW)

```bash
# Emergency fix - just copy scripts to PATH
cd /home/evan/VulcanOS/archiso/airootfs/usr/local/bin
sudo cp vulcan-* ~/.local/bin/
chmod +x ~/.local/bin/vulcan-*

# Test
which vulcan-menu
# Press Super+Escape to test power menu
```

## Proper Fix (Recommended - Uses Orchestrator)

### Step 1: Create Pre-Flight Backup

**CRITICAL:** Create a manual backup before starting:

```bash
# Full backup with compression
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "pre-orchestrator"

# Verify backup created
/home/evan/VulcanOS/scripts/verify-backups.sh

# Git checkpoint
cd ~/VulcanOS
git add -A
git commit -m "Checkpoint: Before orchestrator configuration alignment"
git push origin main
```

### Step 2: Read the Full Plan

```bash
cat /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md
cat /home/evan/VulcanOS/.orchestrator/pre-flight-checklist.md
```

### Step 3: Launch Orchestrator

From your terminal:

```bash
opencode
```

Then in OpenCode, say:

```
@orchestrator Execute the VulcanOS Configuration Alignment Plan at /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md

Execute all 5 phases sequentially. Create backups before any destructive operations.

Report progress after each phase completion.
```

### Step 3: Verify After Completion

```bash
/home/evan/VulcanOS/scripts/verify-vulcan.sh
```

## What the Orchestrator Will Do

### Phase 1 (30 min) - Emergency Script Deployment
- Copy missing scripts from archiso → dotfiles
- Symlink scripts to ~/.local/bin
- **Result:** All keybindings work immediately

### Phase 2 (30 min) - Fix Directory Structure  
- Standardize dotfiles to proper GNU Stow format
- Clean up duplicates
- **Result:** Consistent, maintainable structure

### Phase 3 (45 min) - Live System Alignment
- Convert configs to stow symlinks
- Live system now auto-syncs with repo
- **Result:** Edit dotfiles, changes apply instantly

### Phase 4 (30 min) - ISO Synchronization
- Sync dotfiles → archiso for ISO building
- Create automation script for future syncs
- **Result:** ISO builds contain latest configs

### Phase 5 (45 min) - Documentation & Enhancements
- Create verification script
- Update documentation
- Add sync automation
- **Result:** Maintainable, documented system

## What You'll Gain

**Immediate:**
- ✅ All keybindings working
- ✅ Full system menu access
- ✅ Power menu restored
- ✅ Screenshot utilities working

**Long-term:**
- ✅ Single source of truth (repo = live system)
- ✅ Edit configs once, apply everywhere
- ✅ ISO builds always in sync
- ✅ Easy to maintain and update
- ✅ Documented and verified

## Architecture After Fix

```
VulcanOS Repo (GitHub)
        ↓
  dotfiles/ (Source of Truth)
        ↓
   [GNU Stow]
        ↓
  ~/.config/ (Symlinks - auto-sync)
  ~/.local/bin/ (Symlinks - auto-sync)
        ↓
    Live System ✅
        ↓
  [Sync Script]
        ↓
  archiso/airootfs/
        ↓
   ISO Build ✅
```

## Estimated Time

- **Quick Fix:** 2 minutes (but won't solve root cause)
- **Full Fix (Orchestrator):** 2-3 hours (solves everything permanently)

## Recommendation

Use the orchestrator. It will take a bit longer, but you'll end up with:
1. A properly structured system
2. Automatic config syncing
3. Maintainable ISO builds
4. Full documentation
5. Verification tools

Plus, you can watch the orchestrator work and learn the architecture.

## After Completion

Your workflow becomes:

```bash
# 1. Edit config
vim ~/VulcanOS/dotfiles/waybar/.config/waybar/config.jsonc

# 2. Changes apply immediately (symlink)
# Waybar updates automatically

# 3. Commit to git
cd ~/VulcanOS
git add dotfiles/
git commit -m "Update waybar config"

# 4. Sync to ISO (when ready for release)
./scripts/sync-dotfiles-to-iso.sh
./scripts/build.sh

# Done!
```

## Need Help?

Check the full plan: `/home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md`

Or ask Claude in OpenCode for clarification on any phase.

---

**Ready to fix VulcanOS? Launch the orchestrator! 🚀**
