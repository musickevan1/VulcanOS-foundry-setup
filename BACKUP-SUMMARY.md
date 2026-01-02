# VulcanOS Backup System - Quick Summary

## 🛡️ Three-Layer Backup Protection

### Layer 1: Automatic (Orchestrator) ✅
```bash
# Happens automatically before each phase
# Location: ~/VulcanOS-backups/YYYYMMDD-HHMMSS-before-phase-N/
# Instant rollback: $(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
```

### Layer 2: Manual Full Backup ⚡
```bash
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "backup-name"
# Creates: Full backup + compression + restore script
# Location: ~/VulcanOS-backups/YYYYMMDD-HHMMSS-backup-name/
```

### Layer 3: Git Version Control 📦
```bash
cd ~/VulcanOS
git add -A
git commit -m "Checkpoint: [description]"
git push origin main
```

---

## 🚀 Quick Commands

| Task | Command |
|------|---------|
| **Create backup** | `/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "name"` |
| **Verify backups** | `/home/evan/VulcanOS/scripts/verify-backups.sh` |
| **List backups** | `ls -lt ~/VulcanOS-backups/` |
| **Instant rollback** | `$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh` |
| **Restore specific** | `~/VulcanOS-backups/[backup-dir]/restore.sh` |

---

## 📋 Pre-Orchestrator Checklist

**Before running orchestrator:**

```bash
# 1. Create manual backup
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "pre-orchestrator"

# 2. Verify backup
/home/evan/VulcanOS/scripts/verify-backups.sh

# 3. Git checkpoint
cd ~/VulcanOS
git add -A
git commit -m "Checkpoint: Before orchestrator"
git push origin main

# 4. Check disk space
df -h ~

# ✅ Ready to run orchestrator!
```

---

## 🔄 Rollback Procedures

### Scenario 1: Last Phase Failed
```bash
$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
hyprctl reload
pkill waybar && waybar &
```

### Scenario 2: Need Earlier Backup
```bash
# List backups
ls -lt ~/VulcanOS-backups/

# Restore
~/VulcanOS-backups/20260101-143022-before-phase-1/restore.sh
```

### Scenario 3: Full Undo
```bash
# Find first backup from session
FIRST=$(ls -lt ~/VulcanOS-backups/ | grep "before-phase-1" | head -1 | awk '{print $9}')

# Restore
~/VulcanOS-backups/${FIRST}/restore.sh

# Unstow
cd ~/VulcanOS/dotfiles
stow -D -v -t ~ *

# Reload
hyprctl reload
```

---

## ⏰ Backup Schedule (Recommended)

| Frequency | When | Command |
|-----------|------|---------|
| **Before changes** | Every time | `backup-vulcan-config.sh "pre-edit"` |
| **Daily** | After work | Git commit + push |
| **Weekly** | Sunday 2 AM | `backup-vulcan-config.sh "weekly"` |
| **Before updates** | System updates | `backup-vulcan-config.sh "pre-update"` |

---

## 📊 Backup Verification

```bash
# Run verification
/home/evan/VulcanOS/scripts/verify-backups.sh

# Expected output:
#   ✓ All backups verified successfully!
#   ✓ Recent backup exists (X days old)
#   ℹ Available disk space: XXG
```

---

## 🆘 Emergency Recovery

### If system won't boot to desktop:

**TTY Recovery (Ctrl+Alt+F2):**
```bash
# Login via TTY
ls -lt ~/VulcanOS-backups/
~/VulcanOS-backups/[latest-working]/restore.sh
systemctl --user restart greetd
```

**USB Recovery:**
```bash
# Boot VulcanOS USB
# Mount system
sudo mount /dev/nvme0n1p2 /mnt
arch-chroot /mnt
cd /home/evan/VulcanOS-backups
./[backup]/restore.sh
reboot
```

---

## 📚 Full Documentation

- **Complete Guide:** `/home/evan/VulcanOS/docs/BACKUP-RESTORE.md`
- **Orchestrator Plan:** `/home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md`
- **Pre-Flight Check:** `/home/evan/VulcanOS/.orchestrator/pre-flight-checklist.md`

---

## ✅ Best Practices

1. **Always backup before changes** - It takes 30 seconds
2. **Verify backups work** - Test restore monthly
3. **Keep 10 recent backups** - Clean up old ones
4. **Git commit frequently** - Version control is free
5. **Test in safe environment** - Make backup, test, restore if needed

---

**Remember:** The best backup is the one you have when you need it! 🛡️

**GitHub Repo (Off-site backup):** https://github.com/musickevan1/VulcanOS.git
