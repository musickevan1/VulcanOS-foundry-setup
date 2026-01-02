# 🔥 VulcanOS - START HERE

```
██╗   ██╗██╗   ██╗██╗      ██████╗ █████╗ ███╗   ██╗ ██████╗ ███████╗
██║   ██║██║   ██║██║     ██╔════╝██╔══██╗████╗  ██║██╔═══██╗██╔════╝
██║   ██║██║   ██║██║     ██║     ███████║██╔██╗ ██║██║   ██║███████╗
╚██╗ ██╔╝██║   ██║██║     ██║     ██╔══██║██║╚██╗██║██║   ██║╚════██║
 ╚████╔╝ ╚██████╔╝███████╗╚██████╗██║  ██║██║ ╚████║╚██████╔╝███████║
  ╚═══╝   ╚═════╝ ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
```

**Development-focused Arch Linux distribution for T2 MacBook Pro**

---

## 🚨 Current Status: Configuration Repair Needed

**Issue:** Critical scripts missing, keybindings broken  
**Solution:** Orchestrator session ready to execute  
**Estimated Time:** 2-3 hours  
**Risk:** Low (triple-layer backups in place)

### What's Broken Right Now?

- ❌ `Super+Escape` - Power menu
- ❌ `Super+Alt+Space` - System menu
- ❌ `Super+K` - Hotkeys help
- ❌ Screenshot keybindings
- ❌ Theme switching keybindings

---

## 🎯 Quick Actions

### Option 1: Emergency Quick Fix (2 minutes)

**Just want it working NOW?**

```bash
# Copy scripts to PATH (temporary fix)
cd /home/evan/VulcanOS/archiso/airootfs/usr/local/bin
cp vulcan-* ~/.local/bin/
chmod +x ~/.local/bin/vulcan-*

# Test
which vulcan-menu
# Press Super+Escape to test
```

⚠️ **Warning:** This won't fix the root cause. Use Option 2 for permanent solution.

---

### Option 2: Proper Fix with Orchestrator (Recommended)

**Permanent solution with safety guarantees:**

#### Step 1: Create Pre-Flight Backup

```bash
/home/evan/VulcanOS/scripts/backup-vulcan-config.sh "final-pre-orchestrator"
/home/evan/VulcanOS/scripts/verify-backups.sh
```

#### Step 2: Read the Plan

```bash
cat /home/evan/VulcanOS/ORCHESTRATOR-READY.md
cat /home/evan/VulcanOS/QUICKSTART-ORCHESTRATOR.md
```

#### Step 3: Launch Orchestrator

```bash
opencode
```

Then paste the prompt from `ORCHESTRATOR-READY.md`

---

## 📚 Documentation Map

| Document | Purpose | Read Time |
|----------|---------|-----------|
| **START-HERE.md** | You are here! Overview | 2 min |
| **ORCHESTRATOR-READY.md** | Launch instructions | 5 min |
| **QUICKSTART-ORCHESTRATOR.md** | Quick guide | 3 min |
| **BACKUP-SUMMARY.md** | Backup quick reference | 2 min |
| `.orchestrator/vulcanos-config-alignment.md` | Full plan (5 phases) | 15 min |
| `.orchestrator/pre-flight-checklist.md` | Pre-launch checklist | 5 min |
| `docs/BACKUP-RESTORE.md` | Complete backup guide | 20 min |
| **CLAUDE.md** | Full project reference | 30 min |

---

## 🛡️ Backup System (Already Set Up!)

You have **triple-layer protection**:

### Layer 1: Automatic (Orchestrator)
- Happens before each phase automatically
- Instant rollback: `$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh`

### Layer 2: Manual Full Backup
- Create: `scripts/backup-vulcan-config.sh "name"`
- Verify: `scripts/verify-backups.sh`
- Restore: `~/VulcanOS-backups/[backup-dir]/restore.sh`

### Layer 3: Git (Off-site)
- Current commit: `026274a`
- GitHub: https://github.com/musickevan1/VulcanOS.git
- Rollback: `git reset --hard <commit-hash>`

---

## 🎓 What Orchestrator Will Do

```
┌─────────────────────────────────────────────────────────┐
│  Phase 1: Emergency Script Rescue          [30 min]    │
│  ✅ Copy missing scripts from archiso                   │
│  ✅ Deploy to ~/.local/bin                              │
│  ✅ All keybindings start working                       │
├─────────────────────────────────────────────────────────┤
│  Phase 2: Stow Structure                   [30 min]    │
│  ✅ Reorganize dotfiles for GNU Stow                    │
│  ✅ Clean up duplicates                                 │
│  ✅ Prepare for live sync                               │
├─────────────────────────────────────────────────────────┤
│  Phase 3: Live System Re-alignment         [45 min]    │
│  ✅✅✅ TRIPLE BACKUP FIRST                              │
│  ✅ Convert configs to stow symlinks                    │
│  ✅ Edit dotfiles → changes apply instantly             │
├─────────────────────────────────────────────────────────┤
│  Phase 4: ISO Synchronization              [30 min]    │
│  ✅ Sync dotfiles → archiso                             │
│  ✅ ISO builds always current                           │
├─────────────────────────────────────────────────────────┤
│  Phase 5: Documentation                    [45 min]    │
│  ✅ Verification tools                                  │
│  ✅ Complete documentation                              │
└─────────────────────────────────────────────────────────┘

Result: Single source of truth architecture
        dotfiles/ = live system = ISO builds
```

---

## ✅ After Orchestrator: New Workflow

```bash
# 1. Edit config
vim ~/VulcanOS/dotfiles/waybar/.config/waybar/config.jsonc

# 2. Changes apply instantly (symlink magic)
# Waybar updates automatically!

# 3. Commit to git
cd ~/VulcanOS
git add dotfiles/
git commit -m "Update waybar config"
git push

# 4. Sync to ISO (when ready)
scripts/sync-dotfiles-to-iso.sh
scripts/build.sh

# Done! ✅
```

---

## 🚀 Ready to Fix VulcanOS?

### Recommended Path

1. ✅ Read this file (you're doing it!)
2. ✅ Read `ORCHESTRATOR-READY.md` (5 min)
3. ✅ Create pre-flight backup (2 min)
4. ✅ Launch orchestrator (2-3 hours)
5. ✅ Test and verify (10 min)
6. 🎉 Enjoy your properly configured VulcanOS!

### Time Commitment

- **Quick fix:** 2 minutes (temporary)
- **Proper fix:** 3 hours (permanent + learning)

---

## 🆘 Need Help?

### Rollback Anything

```bash
# Last phase failed? Instant rollback:
$(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh

# List all backups:
ls -lt ~/VulcanOS-backups/

# Restore specific backup:
~/VulcanOS-backups/[backup-dir]/restore.sh
```

### Can't Boot to Desktop?

```bash
# TTY (Ctrl+Alt+F2)
~/VulcanOS-backups/[latest]/restore.sh
systemctl --user restart greetd
```

---

## 📊 Project Stats

- **Total Scripts:** 15+ vulcan-* utilities
- **Configs Managed:** 12 applications
- **Backup Layers:** 3 (auto + manual + git)
- **Documentation:** 10+ comprehensive guides
- **Safety Rating:** 🛡️🛡️🛡️ Triple-protected

---

## 🔗 Quick Links

- **GitHub:** https://github.com/musickevan1/VulcanOS.git
- **Current Commit:** 026274a
- **Branch:** main

---

## 🎯 Bottom Line

**Your VulcanOS needs configuration alignment. The orchestrator is ready to fix it safely.**

**Choose your path:**
- Quick & dirty → Emergency fix above (2 min)
- Proper & permanent → Read `ORCHESTRATOR-READY.md` then launch (3 hours)

**Recommendation:** Use the orchestrator. It's safe, documented, and teaches you the system.

---

**Let's fix VulcanOS! 🔥**

Next file to read: `ORCHESTRATOR-READY.md`
