# Technology Stack: Linux System Backup & Sync on ext4

**Project:** VulcanOS Backup/Sync Milestone
**Filesystem:** ext4 (no btrfs)
**Researched:** 2025-01-23
**Overall confidence:** HIGH

## Executive Summary

For ext4-based Arch Linux system backup and recovery, the recommended stack combines **Timeshift** (system snapshots), **GNU Stow** (dotfiles), **pacman** native features (package lists), and **timeshift-autosnap** (automatic pre-update protection). This provides comprehensive coverage without requiring btrfs or complex configuration.

**Key insight:** Timeshift moved from AUR to official repos in 2025 (v25.12.4), making it a first-class citizen for ext4 snapshots on Arch. Combined with pacman hooks, it provides macOS Time Machine-like protection with minimal overhead.

## Recommended Stack

### Core System Snapshots
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **Timeshift** | 25.12.4-1 | System restore/rollback | Official repo, mature rsync+hardlinks for ext4, GUI+CLI, excludes /home by default |
| **timeshift-autosnap** | 0.10.0-1 (AUR) | Pre-update snapshots | Pacman hook integration, automatic cleanup, prevents multiple snapshots per update |
| **cronie** | 1.7.2-2 | Scheduled snapshots | Required by Timeshift, standard Arch cron implementation |
| **rsync** | 3.4.1-2 | Snapshot engine | Timeshift backend for ext4, efficient hardlink-based incremental backups |

**Rationale:** Timeshift in RSYNC mode creates full browsable snapshots using hardlinks for space efficiency. Each snapshot is a complete system backup but shares unchanged files between snapshots. Perfect for ext4 where filesystem-level snapshots (like btrfs) aren't available. The 25.12.4 release (Dec 2024) is actively maintained by Linux Mint team.

**Confidence:** HIGH - Official Arch package, extensive Arch Wiki documentation, proven on ext4 for years.

### Dotfile Management
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **GNU Stow** | 2.4.1-1 | Dotfile symlinks | Already in use by VulcanOS, simple, no magic, works with any git workflow |
| **Git** | 2.52.0-2 | Version control | Standard VCS, manual sync to remote for backup |
| **git-delta** | 0.18.2-6 | Diff viewer | Better commit review (already in VulcanOS) |

**Rationale:** VulcanOS already uses Stow for dotfile management. Don't replace what works. Stow's symlink approach is transparent, reversible, and requires no special tooling. For backup, simply `git push` to remote repository (GitHub/GitLab/self-hosted). No need for chezmoi's complexity (templates, secrets) unless multi-machine with different configs is needed—not the case for VulcanOS.

**Why NOT chezmoi:** VulcanOS is single-machine focused (T2 MacBook Pro). Chezmoi's strengths (templates for machine-specific configs, password manager integration) are overkill. Stow + Git is simpler and already deployed.

**Confidence:** HIGH - Already in production use on VulcanOS, well-understood workflow.

### Package List Management
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **pacman** | Built-in | Package list export | Native `-Qqe` flag exports explicit packages, `-Qqen`/`-Qqem` separates official/AUR |
| **Git** | 2.52.0-2 | Package list versioning | Track package list changes over time |

**Rationale:** Pacman has native package list export. No additional tools needed. Simply:
- `pacman -Qqen > pkglist-official.txt` (official repo packages)
- `pacman -Qqem > pkglist-aur.txt` (AUR packages)
- Commit to git, push to remote

Restore with:
- `pacman -S --needed - < pkglist-official.txt`
- `yay -S --needed - < pkglist-aur.txt`

**Why NOT specialized tools:** Tools like `pkglist` generator scripts add complexity without significant benefit. Pacman's built-in flags are sufficient and documented in Arch Wiki.

**Confidence:** HIGH - Standard Arch practice, documented in official wiki.

### Pacman Hook Automation
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **pacman hooks** | Built-in | Pre-transaction automation | Native libalpm feature, no external dependencies |
| **timeshift-autosnap** | 0.10.0-1 (AUR) | Timeshift pacman integration | Automatic snapshots before updates, configurable retention, grub-btrfs compatible |

**Rationale:** Pacman hooks (`/etc/pacman.d/hooks/`) execute before/after package transactions. timeshift-autosnap provides pre-configured hook that:
- Creates Timeshift snapshot before package upgrades
- Deletes old auto-snapshots (configurable max count)
- Prevents multiple snapshots per update (checks last snapshot time)
- Updates GRUB if grub-btrfs installed (allows boot into snapshots)

**Alternative considered:** Writing custom hook - rejected because timeshift-autosnap is mature, configurable, and handles edge cases (AUR helper multiple pacman invocations, race conditions with grub-btrfsd).

**Confidence:** HIGH - Popular AUR package (timeshift-autosnap), well-maintained, based on standard pacman hook mechanism.

### Optional: User Data Backup
| Technology | Version | Purpose | When to Use |
|------------|---------|---------|-------------|
| **Restic** | 0.18.1-1 | Encrypted incremental backups | Large /home directory, need encryption, backup to remote/cloud |
| **Borg** | 1.4.3-2 | Deduplicating backups | Backup to local external drive or SSH server, want compression |

**Rationale:** Timeshift explicitly excludes /home by design (it's a system restore tool, not user data backup). For user data:
- **Restic:** If need encrypted backups to cloud (S3, Backblaze B2) or remote SSH. Fast, single binary, deduplication, supports many backends. Good for offsite backup strategy.
- **Borg:** If backup to local external drive or SSH server. Better compression than Restic, FUSE mount for browsing backups. Slightly higher learning curve but more efficient for local backups.

**Why NOT rsnapshot:** Older rsync wrapper. Restic/Borg offer better deduplication, encryption, and active development.

**When to skip:** If /home is small (<50GB) and already synced to cloud (Nextcloud, Syncthing, etc.), user data backup may not be needed. Focus on Timeshift for system and Git for dotfiles.

**Confidence:** MEDIUM - Both tools well-regarded, but user data backup strategy depends on /home size and existing cloud sync. Not essential for system bootability goal.

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| System Snapshots | Timeshift (rsync) | **Snapper** | Designed for btrfs/LVM snapshots. On ext4, requires LVM thin provisioning (complex setup). Timeshift's rsync mode simpler for ext4. |
| System Snapshots | Timeshift | **Systemback** | Unmaintained, fork exists but unstable. Timeshift actively maintained by Linux Mint. |
| System Snapshots | Timeshift | **Manual rsync scripts** | Reinventing wheel. Timeshift provides GUI, scheduling, GRUB integration, exclude patterns. |
| Dotfiles | GNU Stow | **chezmoi** | Over-engineered for single-machine use case. Stow already deployed in VulcanOS. |
| Dotfiles | Git + Stow | **yadm** | Adds wrapper layer. Direct Git + Stow is more transparent. |
| User Data | Restic/Borg | **Duplicity** | Older, slower, less active development than Restic/Borg. |
| User Data | Restic/Borg | **rsnapshot** | No encryption, no compression, no deduplication. Restic/Borg superior. |
| Package List | pacman -Qq | **Custom tools** | Unnecessary. Pacman has built-in export. |

## Installation & Configuration

### 1. System Snapshots (Timeshift)

```bash
# Install
sudo pacman -S timeshift

# Install AUR helper for autosnap (if not already installed)
# yay or paru already in VulcanOS

# Install autosnap hook
yay -S timeshift-autosnap

# Configure Timeshift (first time)
sudo timeshift-gtk  # GUI configuration
# OR
sudo timeshift --create --comments "Initial snapshot" --tags D  # CLI

# Configure autosnap
# Edit /etc/timeshift-autosnap.conf
skipAutosnap=false
deleteSnapshots=true
maxSnapshots=3
updateGrub=true
minHoursBetweenSnapshots=1

# Schedule regular snapshots
sudo systemctl enable cronie
sudo systemctl start cronie
# Configure schedule in Timeshift GUI (daily/weekly/monthly)
```

**Recommendation:**
- Keep 3 autosnap snapshots (before updates)
- Keep 5 daily snapshots
- Keep 2 weekly snapshots
- Keep 0 monthly (for personal laptop, not server)
- Snapshots stored in `/run/timeshift/backup/timeshift/snapshots/`

### 2. Dotfile Sync (Already Configured)

```bash
# VulcanOS already uses Stow - just add remote backup
cd ~/VulcanOS/dotfiles
git remote add backup git@github.com:user/vulcanos-dotfiles-backup.git
# OR self-hosted
git remote add backup user@server:~/backups/dotfiles.git

# Push dotfiles to remote
git push backup main

# Automate with cron or manual workflow
# Add to weekly routine: git push backup main
```

### 3. Package List Backup

```bash
# Create directory for package lists
mkdir -p ~/.local/share/packages

# Export package lists (manual or scripted)
pacman -Qqen > ~/.local/share/packages/pkglist-official.txt
pacman -Qqem > ~/.local/share/packages/pkglist-aur.txt

# Add to git repository
cd ~/.local/share/packages
git init
git add .
git commit -m "Initial package list"
git remote add origin git@github.com:user/vulcanos-packages.git
git push -u origin main

# Automate with pacman hook (optional)
# Create /etc/pacman.d/hooks/package-list-backup.hook
```

**Example hook** (`/etc/pacman.d/hooks/package-list-backup.hook`):
```ini
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = *

[Action]
Description = Backing up package list...
When = PostTransaction
Exec = /bin/bash -c 'pacman -Qqen > /home/evan/.local/share/packages/pkglist-official.txt && pacman -Qqem > /home/evan/.local/share/packages/pkglist-aur.txt'
```

### 4. Optional: User Data Backup (Restic to External Drive)

```bash
# Install
sudo pacman -S restic

# Initialize repository on external drive
# Assuming drive mounted at /mnt/backup
restic init --repo /mnt/backup/restic-repo
# Enter and confirm password (store in password manager!)

# Create backup
restic -r /mnt/backup/restic-repo backup /home/evan \
  --exclude=/home/evan/.cache \
  --exclude=/home/evan/VulcanOS/out \
  --exclude=/home/evan/VulcanOS/customrepo

# Automate with systemd timer (see Arch Wiki: Restic)
# Or manual weekly/monthly backup workflow
```

## Backup Strategy: 3-2-1 Rule Compliance

| Data Type | Primary | Secondary | Offsite |
|-----------|---------|-----------|---------|
| **System** | Timeshift snapshots (local) | Bootable USB (ISO) | N/A (recreatable from packages) |
| **Dotfiles** | Stow symlinks (local repo) | Git push to GitHub/GitLab | ✓ Remote git |
| **Package lists** | Local git repo | Git push to remote | ✓ Remote git |
| **User data** | Local filesystem | Restic/Borg to external drive | Restic to cloud (optional) |

**Note:** System snapshots (Timeshift) are local-only by design. For disaster recovery, VulcanOS ISO + package lists + dotfiles from Git = full system recreation. This is better than backing up system snapshots (which are large and OS-version specific).

## Restore Scenarios

### Scenario 1: Package Update Broke System
**Solution:** Boot from Timeshift snapshot
1. Reboot, select older snapshot from GRUB (if grub-btrfs installed)
2. OR: Boot from live USB, restore with `sudo timeshift --restore`
3. System rolls back to pre-update state

**Time to recovery:** 5-15 minutes

### Scenario 2: Accidentally Deleted Config Files
**Solution:** Restore from dotfiles git
```bash
cd ~/VulcanOS/dotfiles
git checkout HEAD -- hypr/.config/hypr/hyprland.conf
stow -R hypr  # Re-stow if needed
```

**Time to recovery:** <1 minute

### Scenario 3: Complete Disk Failure
**Solution:** Rebuild from VulcanOS ISO + cloud backups
1. Install VulcanOS from ISO
2. `git clone` dotfiles repository
3. `cd dotfiles && stow */`
4. `git clone` package list repository
5. `pacman -S --needed - < pkglist-official.txt`
6. `yay -S --needed - < pkglist-aur.txt`
7. Restore user data from Restic/Borg backup

**Time to recovery:** 1-3 hours (mostly package download time)

### Scenario 4: Corrupted /boot Partition
**Solution:** Timeshift can restore /boot
```bash
# Boot from live USB
sudo timeshift --restore --snapshot "2025-01-20_12-00-00"
# Select snapshot that includes /boot
```

**Time to recovery:** 10-20 minutes

## Performance Characteristics

### Timeshift (RSYNC mode on ext4)
- **First snapshot:** ~10-30 minutes (full copy of system, ~10-20GB depending on installed packages)
- **Incremental snapshot:** ~30 seconds - 2 minutes (only changed files copied, hardlinks for unchanged)
- **Disk space:** First snapshot = system size, each additional = ~500MB-2GB (depending on changes)
- **Restoration:** ~5-15 minutes (rsync files back)

### Package List Export
- **Export time:** <1 second
- **File size:** ~10-50KB (few hundred lines of package names)

### Dotfiles Git Push
- **Push time:** <5 seconds (configs are small text files)
- **Repo size:** ~1-5MB (configs + themes)

### Restic Backup (if used)
- **First backup:** ~10-60 minutes (depends on /home size, ~50-200GB typical)
- **Incremental:** ~1-5 minutes (only changed files)
- **Repo growth:** ~5-10% of original size per backup (due to deduplication)

## Caveats & Limitations

### Timeshift
- ❗ **Excludes /home by default** - This is intentional (system restore tool, not user backup)
- ❗ **Requires external drive or separate partition** - Don't snapshot to same partition (defeats purpose)
- ❗ **RSYNC mode slower than btrfs snapshots** - btrfs snapshots are instant, rsync takes minutes
- ⚠️ **Snapshots are not bootable images** - Can't create bootable USB from snapshot (use ISO for that)
- ℹ️ **Snapshot location matters** - Default `/run/timeshift/backup/` is tmpfs on some systems, change to persistent storage

**Mitigation:**
- Configure Timeshift to store snapshots on dedicated partition or external drive
- Edit `/etc/timeshift/timeshift.json` to set `backup_device_uuid`

### timeshift-autosnap
- ⚠️ **Snapshots before EVERY package operation** - Can create many snapshots if updating frequently
- ℹ️ **minHoursBetweenSnapshots config prevents spam** - Set to 1-6 hours to avoid multiple snapshots per day

**Mitigation:**
- Set `maxSnapshots=3` to limit auto-snapshot retention
- Set `minHoursBetweenSnapshots=6` if you update multiple times per day

### GNU Stow
- ⚠️ **Symlinks can break if source deleted** - Don't delete dotfiles repo while stowed
- ⚠️ **Conflicts with existing files** - Stow won't overwrite existing non-symlink files

**Mitigation:**
- Never delete `~/VulcanOS/dotfiles/` directory
- Use `stow --adopt` to pull existing configs into stow directory

### pacman Package Lists
- ❗ **Doesn't preserve configuration** - Only package names, not /etc configs or package settings
- ℹ️ **AUR packages may fail to install** - Packages removed from AUR or dependency issues

**Mitigation:**
- Backup `/etc` separately (include in Timeshift snapshots)
- Review AUR package list before mass install, remove obsolete packages

### Restic/Borg
- ⚠️ **Repository password required** - Lose password = lose backups (no recovery)
- ℹ️ **External drive must be mounted** - Backups fail if drive not available

**Mitigation:**
- Store Restic/Borg password in password manager with backup codes
- Use systemd mount unit for consistent external drive mounting
- Configure backup script to check if drive mounted before running

## Disk Space Planning

For VulcanOS system with typical development setup:

| Component | Size | Retention | Total Space |
|-----------|------|-----------|-------------|
| System (/) | ~20GB | Base | 20GB |
| Timeshift snapshots | ~2GB each | 3 auto + 5 daily + 2 weekly | ~20GB |
| /home | ~100GB | Live data | 100GB |
| Restic backups (optional) | ~110GB initial + ~10GB incremental | 10 snapshots | ~200GB |
| **Total recommended** | | | **340GB** |

**Recommendation for VulcanOS T2 MacBook Pro:**
- System partition: 40GB (20GB system + 20GB Timeshift)
- Home partition: Remaining space
- External drive: 500GB+ for Restic/Borg user data backups

**If space limited:**
- Reduce Timeshift daily snapshots to 2 (instead of 5)
- Skip monthly snapshots
- Use Restic with cloud backend (no local storage needed)

## Monitoring & Maintenance

### Weekly Tasks
```bash
# Check Timeshift snapshot status
sudo timeshift --list

# Verify dotfiles git status
cd ~/VulcanOS/dotfiles && git status

# Update package lists
pacman -Qqen > ~/.local/share/packages/pkglist-official.txt
pacman -Qqem > ~/.local/share/packages/pkglist-aur.txt
cd ~/.local/share/packages && git commit -am "Update $(date +%Y-%m-%d)" && git push
```

### Monthly Tasks
```bash
# Clean old Timeshift snapshots (if not auto-managed)
sudo timeshift --delete-all --older-than 30d

# Verify Restic repository integrity (if used)
restic -r /mnt/backup/restic-repo check

# Test restore (pick random config file)
cd ~/VulcanOS/dotfiles
git checkout HEAD~5 -- waybar/.config/waybar/config.jsonc
# Verify file restored correctly
git checkout HEAD -- waybar/.config/waybar/config.jsonc  # Restore to latest
```

### Automation Recommendations

1. **Timeshift snapshots:** Handled by cronie + Timeshift schedule (automatic)
2. **Package list backup:** Pacman hook (automatic) or weekly cron job
3. **Dotfiles git push:** Manual (configs don't change daily) or weekly cron
4. **User data backup:** Weekly/monthly cron job or systemd timer

**Example systemd timer for package list (optional):**
```ini
# /etc/systemd/system/package-list-backup.timer
[Unit]
Description=Weekly package list backup

[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
```

## Sources

### High Confidence (Official Documentation)
- [Arch Wiki: System backup](https://wiki.archlinux.org/title/System_backup)
- [Arch Wiki: Timeshift](https://wiki.archlinux.org/title/Timeshift)
- [Arch Wiki: Restic](https://wiki.archlinux.org/title/Restic)
- [Arch Wiki: pacman](https://wiki.archlinux.org/title/Pacman)
- [Arch Wiki: pacman/Tips and tricks - Package lists](https://wiki.archlinux.org/title/Pacman/Tips_and_tricks)
- [Arch Linux Packages: timeshift 25.12.4-1](https://archlinux.org/packages/extra/x86_64/timeshift/)
- [Arch Linux Packages: stow 2.4.1-1](https://archlinux.org/packages/extra/any/stow/)
- [Arch Linux Packages: restic 0.18.1-1](https://archlinux.org/packages/extra/x86_64/restic/)
- [Arch Linux Packages: borg 1.4.3-2](https://archlinux.org/packages/extra/x86_64/borg/)
- [GitHub: teejee2008/timeshift](https://github.com/teejee2008/timeshift)
- [chezmoi.io official site](https://www.chezmoi.io/)

### Medium Confidence (Community Resources)
- [AUR: timeshift-autosnap 0.10.0-1](https://aur.archlinux.org/packages/timeshift-autosnap)
- [GitHub: dagorret/timeshift-autosnap](https://github.com/dagorret/timeshift-autosnap)
- [MangoHost: Duplicacy vs Restic vs Borg comparison 2025](https://mangohost.net/blog/duplicacy-vs-restic-vs-borg-which-backup-tool-is-right-in-2025/)
- [Grigio: Backup speed benchmark: rsync vs borg vs restic vs kopia](https://grigio.org/backup-speed-benchmark/)
- [Solene: Backup software: borg vs restic](https://dataswamp.org/~solene/2021-05-21-borg-vs-restic.html)
- [Arch Linux Forums: Hooking the pacman update process](https://forum.manjaro.org/t/hooking-the-pacman-update-process/20422)
- [Arch Linux Forums: Making a pacman hook to backup /boot](https://bbs.archlinux.org/viewtopic.php?id=289248)

### Tools Mentioned (Not Recommended)
- Snapper - [Arch Wiki: Snapper](https://wiki.archlinux.org/title/Snapper) - Designed for btrfs/LVM, complex on ext4
- rsnapshot - [Arch Wiki: Synchronization and backup programs](https://wiki.archlinux.org/title/Synchronization_and_backup_programs) - Older, less feature-rich than Restic/Borg
- yadm - [yadm.io](https://yadm.io/) - Dotfile manager wrapper, unnecessary with Stow already in use
