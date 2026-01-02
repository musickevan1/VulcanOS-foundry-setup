# VulcanOS Backup & Restore Guide

**Critical:** Always have backups before making system changes.

---

## Backup Types

### 1. Automatic Phase Backups (Orchestrator)

**When:** Before each orchestrator phase automatically  
**Location:** `~/VulcanOS-backups/YYYYMMDD-HHMMSS-before-phase-N/`  
**Coverage:** Configs, scripts, dotfiles, metadata  
**Restore Time:** Instant (< 1 minute)

**How it works:**
- Orchestrator uses `auto-backup-wrapper.sh` for each phase
- Creates timestamped backup automatically
- Generates `quick-restore.sh` script
- Stores last backup path in `/tmp/vulcan-last-backup.txt`

**Instant Rollback:**
```bash
$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
```

### 2. Manual Full Backup

**When:** Before major changes, system experiments, or weekly  
**Location:** `~/VulcanOS-backups/YYYYMMDD-HHMMSS-backup-name/`  
**Coverage:** Full system configs + compression option  
**Restore Time:** 2-5 minutes

**Create backup:**
```bash
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "weekly-backup"
```

**Features:**
- Interactive compression option
- Detailed metadata (package versions, system info)
- File inventory
- Complete restore script
- Old backup cleanup

**Restore:**
```bash
~/VulcanOS-backups/20260101-143022-weekly-backup/restore.sh
```

### 3. Git Version Control

**When:** After tested changes, before pushing to GitHub  
**Location:** Git history  
**Coverage:** All VulcanOS repo files  
**Restore Time:** Variable (depends on changes)

**Commit checkpoint:**
```bash
cd /home/evan/VulcanOS
git add -A
git commit -m "Checkpoint: [describe state]"
git push origin main
```

**Restore from git:**
```bash
cd /home/evan/VulcanOS
git log --oneline  # Find commit hash
git checkout <commit-hash>
```

### 4. System Snapshots (Future - Timeshift/Snapper)

**Status:** Not yet implemented  
**When:** Daily/weekly automatic  
**Coverage:** Entire system filesystem  
**Restore Time:** 10-30 minutes

---

## Quick Reference: Backup Commands

| Situation | Command |
|-----------|---------|
| Before orchestrator session | `/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "pre-orchestrator"` |
| Weekly maintenance backup | `/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "weekly"` |
| Before manual config edit | `/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "pre-edit-waybar"` |
| Before theme experiment | `/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "before-custom-theme"` |
| Emergency checkpoint | `cd ~/VulcanOS && git add -A && git commit -m "Emergency checkpoint"` |

---

## Quick Reference: Restore Commands

| Situation | Command |
|-----------|---------|
| Rollback last phase | `$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh` |
| List all backups | `ls -lt ~/VulcanOS-backups/` |
| Restore specific backup | `~/VulcanOS-backups/[backup-dir]/restore.sh` |
| Restore from git | `cd ~/VulcanOS && git checkout <commit-hash>` |

---

## Backup Strategy: Triple Layer Protection

### Layer 1: Automatic (Orchestrator)
- ✅ Happens automatically before each phase
- ✅ No user action required
- ✅ Instant rollback capability
- ⚠️ Only covers configs/scripts (not full system)

### Layer 2: Manual Full Backup
- ⚠️ Requires user to run command
- ✅ Comprehensive coverage
- ✅ Compressed option (saves space)
- ✅ Detailed metadata

### Layer 3: Git Commits
- ⚠️ Requires user to commit
- ✅ Version history
- ✅ Synced to GitHub (off-site backup)
- ✅ Can compare changes over time

**Recommended workflow for critical changes:**
```bash
# Layer 1 (automatic via orchestrator)
# Layer 2
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "before-critical-change"

# Layer 3
cd ~/VulcanOS
git add -A
git commit -m "Checkpoint before critical change"
git push origin main

# Now make your changes...

# After testing and confirming changes work
git add -A
git commit -m "Applied critical change - tested and working"
git push origin main
```

---

## Detailed Restore Procedures

### Scenario 1: Phase Failed During Orchestrator

**Symptoms:**
- Orchestrator reports phase failure
- Keybindings broken
- Services not starting
- Waybar missing

**Solution:**
```bash
# 1. Get last backup path
BACKUP=$(cat /tmp/vulcan-last-backup.txt)
echo "Restoring from: $BACKUP"

# 2. Run quick restore
${BACKUP}/quick-restore.sh

# 3. Reload services
hyprctl reload
pkill waybar && sleep 0.5 && waybar &
swaync-client -R

# 4. Verify
which vulcan-menu
ls -la ~/.config/waybar

# 5. Test keybindings
# Super+Escape, Super+Alt+Space, etc.
```

### Scenario 2: Config Edit Went Wrong

**Symptoms:**
- Edited config file, broke functionality
- Service won't start
- Syntax errors

**Solution:**
```bash
# Option A: Restore just that config
BACKUP=$(ls -t ~/VulcanOS-backups/ | head -1)
cp ~/VulcanOS-backups/${BACKUP}/config/waybar/* ~/.config/waybar/
pkill waybar && waybar &

# Option B: Full restore
~/VulcanOS-backups/${BACKUP}/restore.sh
```

### Scenario 3: Want to Undo Entire Orchestrator Session

**Symptoms:**
- Completed orchestrator, but want to go back
- Everything works but you prefer old setup

**Solution:**
```bash
# Find the FIRST backup from the session
ls -lt ~/VulcanOS-backups/ | grep "before-phase-1"

# Restore it
~/VulcanOS-backups/20260101-120000-before-phase-1/restore.sh

# Unstow everything
cd ~/VulcanOS/dotfiles
stow -D -v -t ~ hypr waybar wofi swaync kitty alacritty nvim opencode starship scripts git bash

# Reload
hyprctl reload
pkill waybar && waybar &
```

### Scenario 4: Catastrophic Failure - System Won't Boot to Desktop

**Solution A: TTY Recovery**
```bash
# Switch to TTY (Ctrl+Alt+F2)
# Login

# Find latest working backup
ls -lt ~/VulcanOS-backups/

# Restore
~/VulcanOS-backups/[latest-working]/restore.sh

# Restart Hyprland
systemctl --user restart greetd
# Or logout and login again
```

**Solution B: Boot from VulcanOS USB**
```bash
# Boot from VulcanOS installation media
# Mount your system partition
sudo mount /dev/nvme0n1p2 /mnt  # Adjust partition as needed
sudo mount /dev/nvme0n1p1 /mnt/boot  # EFI partition

# Chroot into system
arch-chroot /mnt

# Restore from backup
cd /home/evan/VulcanOS-backups
./[backup-dir]/restore.sh

# Exit and reboot
exit
reboot
```

---

## Backup Maintenance

### View Backup Disk Usage

```bash
du -sh ~/VulcanOS-backups/*
df -h ~  # Check available space
```

### Manual Cleanup

```bash
# List old backups
ls -lt ~/VulcanOS-backups/ | tail -20

# Remove specific backup
rm -rf ~/VulcanOS-backups/20250101-old-backup

# Keep only last 10 backups (automatic in backup script)
# Run backup script and select cleanup option
```

### Compress Old Backups

```bash
# Compress a specific backup
cd ~/VulcanOS-backups
tar -czf 20260101-backup.tar.gz 20260101-143022-weekly-backup/
rm -rf 20260101-143022-weekly-backup/

# Extract when needed
tar -xzf 20260101-backup.tar.gz
```

---

## Testing Your Backups

**IMPORTANT:** Backups are useless if they don't work. Test regularly!

### Test Backup Creation

```bash
# Create test backup
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "test-backup"

# Verify contents
LATEST=$(ls -t ~/VulcanOS-backups/ | head -1)
tree ~/VulcanOS-backups/${LATEST}/

# Check each component
ls ~/VulcanOS-backups/${LATEST}/config/
ls ~/VulcanOS-backups/${LATEST}/local-bin/
ls ~/VulcanOS-backups/${LATEST}/dotfiles/
cat ~/VulcanOS-backups/${LATEST}/metadata/backup-info.txt
```

### Test Quick Restore (Safe Test)

```bash
# 1. Note current state
ls -la ~/.config/waybar

# 2. Make a safe test change
echo "# test comment" >> ~/.config/waybar/config.jsonc

# 3. Create backup
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "test-restore"

# 4. Make another change
echo "# another test" >> ~/.config/waybar/config.jsonc

# 5. Restore
LATEST=$(ls -t ~/VulcanOS-backups/ | head -1)
~/VulcanOS-backups/${LATEST}/restore.sh

# 6. Verify restore worked
grep "another test" ~/.config/waybar/config.jsonc
# Should NOT be found (was removed by restore)

grep "test comment" ~/.config/waybar/config.jsonc
# SHOULD be found (was in the backup)

# 7. Clean up
vim ~/.config/waybar/config.jsonc  # Remove test comments
```

---

## Backup Verification Checklist

Before considering a backup valid, verify:

- [ ] Backup directory exists: `ls ~/VulcanOS-backups/[latest]/`
- [ ] Config directories present: `ls ~/VulcanOS-backups/[latest]/config/`
- [ ] Scripts backed up: `ls ~/VulcanOS-backups/[latest]/local-bin/vulcan-*`
- [ ] Metadata file exists: `cat ~/VulcanOS-backups/[latest]/metadata/backup-info.txt`
- [ ] Restore script executable: `test -x ~/VulcanOS-backups/[latest]/restore.sh`
- [ ] Test restore in safe environment

---

## Emergency Contact Sheet

**If all backups fail and you need to rebuild:**

1. **Boot VulcanOS Live USB**
2. **Clone fresh repo:**
   ```bash
   git clone https://github.com/musickevan1/VulcanOS.git
   cd VulcanOS/dotfiles
   stow -v -t ~ *
   ```

3. **Or reinstall from ISO** (nuclear option)

**GitHub repo:** https://github.com/musickevan1/VulcanOS.git

---

## Best Practices

### Daily
- ✅ Git commit after successful changes
- ✅ Test new configs in safe environment first

### Before Major Changes
- ✅ Create manual full backup
- ✅ Git commit checkpoint
- ✅ Verify backup created successfully

### Weekly
- ✅ Create weekly backup: `backup-vulcan-config.sh "weekly"`
- ✅ Test one backup restore in safe way
- ✅ Clean up old backups (keep 10 most recent)
- ✅ Push git commits to GitHub

### Before Orchestrator Sessions
- ✅ Manual full backup
- ✅ Git checkpoint
- ✅ Verify disk space available
- ✅ Close critical applications

### After Orchestrator Sessions
- ✅ Verify changes work
- ✅ Git commit successful changes
- ✅ Keep orchestrator backups for 7 days
- ✅ Document any issues encountered

---

## Automation (Future Enhancement)

**Automated Backup Cron Job:**
```bash
# Add to crontab -e
0 2 * * 0 /home/evan/VulcanOS/scripts/backup-vulcan-config.sh "weekly-auto" >/dev/null 2>&1
```

**Automated Git Commits:**
```bash
# Add to crontab -e
0 23 * * * cd /home/evan/VulcanOS && git add -A && git commit -m "Auto-commit $(date)" && git push
```

---

**Remember:** The best backup is the one you have when you need it. Backup early, backup often! 🛡️
