# Feature Landscape: Linux Backup and Sync Tools

**Domain:** System backup and snapshot management
**Researched:** 2026-01-23
**Context:** VulcanOS ext4-based system with Timeshift rsync mode, pacman integration, wofi menu

## Executive Summary

Linux backup/sync tools divide into three primary categories: snapshot-based (btrfs/LVM), incremental backup archives (borg/restic), and synchronization tools (rsync/rclone). For VulcanOS's ext4 filesystem, rsync-based snapshot tools like Timeshift are the standard choice.

**Table stakes** features include snapshot creation/restoration, automated scheduling, and exclusion filters. **Differentiators** include pre-update hooks, GUI quick actions, and status reporting. **Anti-features** to avoid include excessive quota tracking, cloud-only dependencies, and overly complex configuration wizards.

---

## Table Stakes

Features users expect from backup/snapshot tools. Missing these = frustrated users.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Snapshot creation** | Core functionality | Low | Single command or button to create snapshot |
| **Snapshot restoration** | Core functionality | Low | Ability to rollback system to previous state |
| **Automated scheduling** | Manual backups fail | Medium | Hourly/daily/weekly/monthly intervals |
| **Exclusion filters** | Avoid backing up cache/temp | Low | Skip /tmp, ~/.cache, node_modules, etc. |
| **Snapshot listing** | Need to see what exists | Low | Show snapshots with timestamp and description |
| **Snapshot deletion** | Disk space management | Low | Remove old snapshots manually or automatically |
| **Boot snapshot** | Pre-update protection | Medium | Create snapshot before risky operations |
| **Incremental backups** | Disk space efficiency | Medium | Only backup changed files (rsync mode) |
| **External drive support** | Offline backup storage | Low | Backup to local external drive |
| **Retention policies** | Automatic cleanup | Medium | Keep last N daily/weekly/monthly snapshots |

**Sources:**
- [Arch Wiki: Synchronization and backup programs](https://wiki.archlinux.org/title/Synchronization_and_backup_programs)
- [Timeshift features](https://github.com/linuxmint/timeshift)
- [25 Best Backup Tools for Linux](https://www.tecmint.com/linux-system-backup-tools/)

---

## Differentiators

Features that set tools apart. Not expected by default, but highly valued when present.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Pre-pacman hook** | Automatic before updates | Medium | snap-pac style pacman hooks |
| **Quick action GUI** | One-click backup/restore | Low | Button in menu instead of terminal |
| **Status reporting** | Know backup is current | Low | "Last backup: 2 hours ago, 5 snapshots" |
| **Menu integration** | Access without terminal | Low | Wofi/rofi submenu for backup actions |
| **Diff browsing** | See what changed | High | Compare snapshots file-by-file |
| **Single file restore** | Granular recovery | Medium | Restore individual files from snapshot |
| **Kernel pinning** | Prevent boot failures | Medium | Hold specific kernel version from updates |
| **Pre/post comparisons** | Audit package changes | Medium | snap-pac creates paired snapshots |
| **GRUB boot menu** | Boot into old snapshot | High | grub-btrfs style boot entries (rsync harder) |
| **Notification alerts** | Inform of backup status | Low | Desktop notification on success/failure |
| **Dot rollback** | Recover config files | Low | Restore dotfiles from snapshot |
| **Package list export** | Reproducible installs | Low | Save pacman -Qqe to file with sync |
| **Git integration** | Version control configs | Low | Auto-commit dotfiles to git after backup |

**Sources:**
- [snap-pac documentation](https://github.com/wesbarnett/snap-pac)
- [Btrfs Assistant features](https://github.com/garuda-linux/btrfs-assistant)
- [Timeshift GUI integration](https://itsfoss.com/backup-restore-linux-timeshift/)

---

## Anti-Features

Features to explicitly NOT build. Common mistakes in backup tool design.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Cloud-only backup** | Dependency on third party | Support local external drives first |
| **Proprietary formats** | Lock-in, no portability | Use standard rsync/tar snapshots |
| **Quota tracking** | Performance degradation | Simple disk usage check instead |
| **Setup wizards** | Complexity, overwhelming | Sane defaults, optional config |
| **Backup everything** | Wasted space, slow restore | Exclude cache/temp/build dirs by default |
| **No manual override** | Loss of control | Allow emergency snapshots anytime |
| **Auto-delete without warning** | Data loss risk | Warn before deleting snapshots |
| **GUI-only interface** | No scriptability | Always provide CLI equivalent |
| **Encryption required** | Added complexity | Exclude sensitive files instead |
| **Online account required** | Privacy concern | Work fully offline |
| **Automatic cloud sync** | Bandwidth waste, privacy | Local only, explicit cloud opt-in |
| **Version history UI** | Feature bloat for simple tools | Show list, let file manager browse |

**Rationale:**

**Quota tracking:** Btrfs quota groups cause [severe performance issues](https://btrfs.readthedocs.io/en/latest/Qgroups.html) especially with frequent snapshots. Simple du or df checks are sufficient for ext4/rsync backups.

**Setup wizards:** [Tool bloat research](https://level.io/blog/tool-bloat-in-modern-it) shows complexity adds cognitive load. Timeshift works with zero config—just click Create.

**Encryption required:** For single-user desktop, excluding sensitive files (.env, .ssh/, .gnupg/) is simpler than managing encrypted vaults. No key management overhead.

**Sources:**
- [Btrfs quota performance problems](https://btrfs.readthedocs.io/en/latest/Qgroups.html)
- [Tool bloat in IT](https://level.io/blog/tool-bloat-in-modern-it)
- [Linux backup best practices](https://www.redhat.com/sysadmin/5-backup-tips)

---

## Feature Dependencies

```
Snapshot System (core)
├── Snapshot creation ──> Automated scheduling
├── Snapshot listing ──> Snapshot deletion
├── Snapshot restoration ──> GRUB boot entries (optional)
└── Exclusion filters ──> Retention policies

Menu Integration (differentiator)
├── Status reporting ──> Quick action GUI
└── Quick action GUI ──> Notification alerts

Package Manager Integration (differentiator)
├── Pre-pacman hook ──> Pre/post comparisons
├── Kernel pinning ──> Boot snapshot
└── Package list export ──> Git integration

Data Sync (table stakes)
├── External drive support ──> Automated scheduling
├── Git integration ──> Dot rollback
└── Incremental backups ──> Retention policies
```

**Critical path for VulcanOS:**
1. Snapshot creation/restoration (must work first)
2. Pre-pacman hook (T2 kernel protection)
3. Quick action GUI (menu integration)
4. Status reporting (user feedback)

**Nice-to-have later:**
- Single file restore (can browse manually for now)
- Diff browsing (complex, low ROI)
- GRUB boot entries (very complex for rsync mode)

---

## MVP Recommendation

For VulcanOS milestone, prioritize:

### Must Have (Table Stakes)
1. **Snapshot creation** — via Timeshift CLI or GUI
2. **Snapshot restoration** — recover from failed update
3. **Automated scheduling** — daily snapshots via systemd timer
4. **Exclusion filters** — skip cache, temp, build dirs
5. **Snapshot listing** — show available restore points
6. **External drive support** — backup to local SSD

### Must Have (Differentiators for VulcanOS)
7. **Pre-pacman hook** — auto-snapshot before updates (critical for T2 kernel safety)
8. **Quick action menu** — wofi submenu with Sync/Snapshot/Restore
9. **Status reporting** — "Last backup: X ago, N snapshots available"

### Defer to Post-MVP
- **Single file restore** — can mount snapshot and copy manually
- **Diff browsing** — nice but not critical, use git diff for dotfiles
- **GRUB boot entries** — very complex for rsync mode, manual restore sufficient
- **Notification alerts** — useful but not blocking
- **Package list export** — can run pacman -Qqe manually for now
- **Git auto-commit** — can commit dotfiles manually initially

### Explicitly Skip
- **Cloud sync** — out of scope (PROJECT.md constraint)
- **Btrfs features** — ext4 filesystem (PROJECT.md constraint)
- **Encrypted backups** — excluding sensitive files instead (PROJECT.md decision)
- **Multi-machine sync** — single T2 MacBook Pro only (PROJECT.md scope)

---

## Feature Complexity Analysis

| Category | Feature | Implementation Effort | Testing Effort | Maintenance Risk |
|----------|---------|----------------------|----------------|------------------|
| **Low** | Snapshot creation | Call timeshift CLI | Boot restore test | Low |
| **Low** | Snapshot listing | Parse timeshift list | Visual check | Low |
| **Low** | Quick action menu | Wofi entries → scripts | Click test | Low |
| **Low** | Status reporting | Check snapshot timestamps | Display test | Low |
| **Low** | Exclusion filters | Timeshift config file | Verify excluded | Low |
| **Medium** | Pre-pacman hook | Pacman hook file + script | Update test | Medium |
| **Medium** | Automated scheduling | Systemd timer | Wait for trigger | Low |
| **Medium** | Retention policies | Timeshift config | Check old deleted | Low |
| **Medium** | External drive support | Mount point config | Drive disconnect | Medium |
| **Medium** | Kernel pinning | Pacman IgnorePkg | Update test | Medium |
| **High** | Single file restore | Mount snapshot + file browser | Many file types | Medium |
| **High** | Diff browsing | File comparison UI | Many scenarios | High |
| **High** | GRUB boot entries | Bootloader integration | Boot failure risk | Very High |

**Recommendation:** Start with Low complexity features. Add Medium features one at a time with testing. Defer High complexity features until core system is stable.

---

## Snapshot Tool Comparison Matrix

Context: VulcanOS uses ext4, so btrfs-only tools are not applicable.

| Tool | Mode | Best For | VulcanOS Fit | Notes |
|------|------|----------|--------------|-------|
| **Timeshift** | rsync | ext4 full system | ✓ Excellent | De facto standard for ext4 snapshots |
| **Snapper** | btrfs/LVM | btrfs subvolumes | ✗ Not applicable | Requires btrfs or thin-provisioned LVM |
| **Borg** | Archive | Incremental backups | △ Possible | Good for data, not system snapshots |
| **Restic** | Archive | Cloud backups | △ Possible | Better for offsite, not system restore |
| **rsnapshot** | rsync | File-level backups | △ Alternative | Similar to Timeshift, less GUI |
| **Back In Time** | rsync | User data | △ Possible | More for /home than system |

**Verdict:** Timeshift rsync mode is the correct choice for VulcanOS.
- ✓ Works on ext4
- ✓ Full system snapshots
- ✓ CLI and GUI available
- ✓ Active development
- ✓ Well documented
- ✓ Arch package available

**Sources:**
- [Timeshift vs Snapper comparison](https://www.compsmag.com/vs/timeshift-vs-snapper/)
- [Borg vs Restic vs rsync benchmark](https://grigio.org/backup-speed-benchmark/)
- [Arch Wiki backup tools](https://wiki.archlinux.org/title/Synchronization_and_backup_programs)

---

## Menu Integration Patterns

Research on how Linux backup tools integrate with desktop menus and status bars.

### Common Patterns

**System Tray Icon:**
- Shows backup status (green = recent, yellow = old, red = failed)
- Click opens quick actions menu
- Right-click for settings/preferences
- Examples: Dropbox, MEGA, Insync

**File Manager Integration:**
- Context menu: "Restore from snapshot"
- Browse snapshots as folders
- Restore individual files with right-click
- Examples: Nautilus extensions, Dolphin plugins

**Application Menu:**
- Launcher entry for backup GUI
- Opens full application window
- Not quick for common actions
- Examples: Timeshift GUI, Back In Time

**Rofi/Wofi Submenu (Desktop WM):**
- Custom menu with backup actions
- Script-driven, keyboard-navigable
- Common in tiling WM setups
- Examples: Custom i3/sway/Hyprland menus

**Waybar Module (Status Bar):**
- Shows last backup time
- Click to open action menu
- Tooltip with snapshot count
- Examples: Custom Waybar modules

### VulcanOS Pattern (Recommended)

**Waybar icon (top-left)** → Click → **vulcan-menu (wofi)** → Navigate to **Backup submenu**

Backup submenu shows:
```
⬤ Last backup: 2 hours ago
📊 5 snapshots available
━━━━━━━━━━━━━━━━━━━━
🔄 Sync Now
📸 Create Snapshot
⏮ Restore Snapshot
⚙️ Configure
```

**Rationale:**
- Consistent with existing VulcanOS menu pattern
- No new system tray dependency
- Keyboard navigable (wofi)
- Scriptable (easy to maintain)
- Shows status inline (no extra query needed)

**Sources:**
- [Linux GUI backup tools](https://www.thelinuxvault.net/linux-backup-recovery/exploring-gui-tools-for-linux-backup-management/)
- [File manager integration patterns](https://www.linux.com/topic/cloud/5-linux-gui-cloud-backup-tools/)

---

## Status Reporting Patterns

How backup tools communicate state to users.

### Status Dimensions

| Dimension | Information | Display Method |
|-----------|-------------|----------------|
| **Last backup time** | "2 hours ago" | Relative timestamp |
| **Snapshot count** | "5 snapshots" | Integer count |
| **Disk usage** | "12.3 GB used" | Size calculation |
| **Next scheduled** | "In 22 hours" | Timer countdown |
| **Backup health** | "OK" / "Warning" / "Error" | Status indicator |
| **Drive status** | "Connected" / "Disconnected" | Mount check |

### Common Implementations

**Timeshift:**
- GUI shows list with creation time, description, size
- No persistent status indicator
- Must open GUI to check

**Back In Time:**
- System tray icon color indicates status
- Tooltip shows last backup time
- Click for detailed view

**Borg/Restic:**
- CLI output shows statistics
- No GUI status by default
- Custom scripts needed for display

**Custom Scripts:**
- Check snapshot directory timestamps
- Parse tool output for counts
- Display in menu/bar

### VulcanOS Implementation (Recommended)

**Status script:** `/usr/local/bin/vulcan-backup-status`
```bash
#!/bin/bash
# Query timeshift for last snapshot
# Count snapshots
# Check external drive mount
# Output: "Last: 2h ago | Snapshots: 5 | Drive: ✓"
```

**Display locations:**
- Wofi backup submenu (inline at top)
- Waybar tooltip (on hover)
- Optional: notification after backup completes

**Update frequency:**
- Check on menu open (dynamic)
- Cached for tooltip (refresh every 5min)
- Force update after backup action

---

## Pacman Hook Integration

Automatic snapshot creation before package updates.

### Hook Mechanism

Pacman supports hooks via files in `/etc/pacman.d/hooks/*.hook`:
- `[Trigger]` section defines when to run
- `[Action]` section defines what to run
- Runs at PreTransaction or PostTransaction

### Example: snap-pac for btrfs

```ini
[Trigger]
Operation = Upgrade
Operation = Install
Operation = Remove
Type = Package
Target = *

[Action]
Description = Creating pre-transaction snapshot...
When = PreTransaction
Exec = /usr/bin/snapper create --type=pre --cleanup-algorithm=number --print-number
```

### VulcanOS Adaptation (Timeshift + ext4)

```ini
[Trigger]
Operation = Upgrade
Type = Package
Target = linux-t2*

[Action]
Description = Creating T2 kernel snapshot...
When = PreTransaction
Exec = /usr/local/bin/vulcan-snapshot-pre-update
```

**Why only linux-t2:** Not every package update needs a snapshot. Focus on high-risk kernel updates that can break boot.

**Script responsibility:**
- Check if external drive mounted
- Create Timeshift snapshot with description "Pre-update: linux-t2"
- Exit 0 to allow update to proceed
- Exit 1 to abort update if snapshot fails

### Feature Flags

**Should hook be conditional?**
- YES: Only run if backup drive is mounted (avoid blocking updates)
- YES: Only run for kernel packages (avoid snapshot spam)
- NO: Don't require user confirmation (defeats automatic protection)

**Should hook block on failure?**
- MAYBE: If drive is mounted but snapshot fails → block update (safety)
- NO: If drive not mounted → warn but proceed (don't block updates)

**Sources:**
- [snap-pac implementation](https://github.com/wesbarnett/snap-pac)
- [Arch Wiki: Snapper](https://wiki.archlinux.org/title/Snapper)
- [Pacman hooks documentation](https://wiki.archlinux.org/title/Pacman#Hooks)

---

## Scheduling Patterns

Automated snapshot creation intervals.

### Common Schedules

| Interval | Retention | Use Case |
|----------|-----------|----------|
| **Hourly** | Keep 24 | Active development, frequent changes |
| **Daily** | Keep 7 | Standard desktop use |
| **Weekly** | Keep 4 | Long-term checkpoints |
| **Monthly** | Keep 3-12 | Historical archives |
| **Boot** | Keep 3 | Pre-startup snapshot |

### Timeshift Default

Timeshift's default schedule:
- Daily: 5 snapshots
- Weekly: 3 snapshots
- Monthly: 2 snapshots
- Boot: 3 snapshots

**Philosophy:** Timeshift runs hourly check, creates snapshot if due. Handles systems that aren't running 24/7.

### VulcanOS Recommendation

For development laptop (not always on):

- **Daily:** 7 snapshots (one week history)
- **Weekly:** 3 snapshots (rollback to last month)
- **Boot:** 3 snapshots (recent known-good states)
- **Manual:** Unlimited (user-created snapshots kept until manual delete)

**Rationale:**
- Daily covers short-term mistakes
- Weekly covers longer-term "when did this break?"
- Boot snapshots capture pre-update states
- Manual snapshots for pre-risky-operation protection

**Implementation:**
- Systemd timer: hourly check (Timeshift built-in)
- Cron alternative: @hourly if systemd timers unavailable
- Config: `/etc/timeshift/timeshift.json`

**Sources:**
- [Timeshift scheduling](https://github.com/linuxmint/timeshift)
- [Snapper timeline cleanup](https://wiki.archlinux.org/title/Snapper)

---

## Exclusion Patterns

What NOT to backup.

### Standard Exclusions

**Cache directories:**
- `~/.cache/`
- `/var/cache/`
- `/tmp/`
- `~/.local/share/Trash/`

**Build artifacts:**
- `node_modules/`
- `target/` (Rust)
- `dist/`, `build/`
- `__pycache__/`, `*.pyc`

**Virtual environments:**
- `venv/`, `.venv/`
- `.tox/`
- `env/`

**Large data (regeneratable):**
- `~/Downloads/` (optional)
- `~/.local/share/Steam/`
- Docker images (if using Docker)

**Already backed up elsewhere:**
- Git repositories (code is in remote)
- Synced cloud folders (Dropbox, etc.)

### Sensitive Files (Exclude for Security)

**Credentials:**
- `~/.ssh/` (private keys)
- `~/.gnupg/` (GPG keys)
- `.env` files
- `credentials.json`
- `*.pem`, `*.key`

**Tokens:**
- `~/.config/*/tokens`
- Browser cookies/sessions
- `~/.aws/credentials`

**Personal:**
- `~/Documents/Personal/` (if contains sensitive data)
- Password manager databases (separate backup)

### VulcanOS Exclusions

Per PROJECT.md: "Exclude sensitive files — no encryption layer this milestone"

**Default exclusions:**
- Standard cache/temp dirs
- Build artifacts
- SSH/GPG keys
- .env files

**User decision:**
- Whether to backup ~/Downloads
- Whether to backup large media libraries
- Whether to backup Docker data

**Implementation:**
- Timeshift config: exclude patterns
- Separate script for sensitive file list
- Documentation on what's excluded and why

**Sources:**
- [Linux backup best practices](https://www.redhat.com/sysadmin/5-backup-tips)
- [Common Linux backup mistakes](https://www.linuxinsider.com/story/essential-tips-for-reliable-linux-backups-177398.html)

---

## Confidence Assessment

| Area | Level | Reason |
|------|-------|--------|
| Table stakes features | **HIGH** | Well-established patterns across Timeshift, Snapper, Borg |
| Differentiators | **MEDIUM** | Based on tool comparisons and user requests, not all validated |
| Anti-features | **MEDIUM** | Based on performance research and best practices, some subjective |
| VulcanOS specifics | **HIGH** | PROJECT.md constraints clear (ext4, local drive, exclude sensitive) |
| Menu integration | **MEDIUM** | Common patterns identified, VulcanOS pattern is custom |
| Pacman hooks | **HIGH** | snap-pac is well-documented, adaptation is straightforward |

### Verification Sources

**HIGH confidence (official documentation):**
- Arch Wiki: Synchronization and backup programs
- Arch Wiki: Snapper
- snap-pac GitHub documentation
- Timeshift GitHub repository

**MEDIUM confidence (multiple sources agree):**
- Timeshift vs Snapper comparisons (multiple forums)
- Backup tool feature lists (multiple review sites)
- Linux backup best practices (Red Hat, Ubuntu wiki)

**LOW confidence (limited verification):**
- Specific wofi menu integration pattern (custom to VulcanOS)
- Waybar status module details (would need implementation testing)

---

## Open Questions for Implementation

**1. Timeshift CLI vs GUI:**
- Should scripts call `timeshift --create` (CLI) or integrate with existing GUI?
- **Recommendation:** CLI for automation, GUI for manual browsing

**2. External drive auto-mount:**
- Should backup drive be auto-mounted on connect, or manual?
- **Recommendation:** Auto-mount with udev rule, safer than always-mounted

**3. Snapshot naming:**
- Auto-generated timestamps vs descriptive names?
- **Recommendation:** Timeshift auto-naming, allow optional comment

**4. Failed update recovery:**
- Boot from live USB and restore, or GRUB entry?
- **Recommendation:** Live USB initially (GRUB rsync restore is complex)

**5. Status update frequency:**
- Real-time monitoring vs on-demand check?
- **Recommendation:** On-demand (check when menu opens, avoids polling)

**6. Package list sync:**
- Separate from snapshot, or include in snapshot?
- **Recommendation:** Separate (git commit package list independently)

---

## Summary for Roadmap

**Phase structure implications:**

1. **Foundation Phase** — Timeshift setup, basic snapshot/restore
2. **Automation Phase** — Pacman hooks, systemd timers, retention policies
3. **Integration Phase** — Wofi menu, status display, quick actions
4. **Polish Phase** — Notifications, documentation, error handling

**Critical path:**
- Timeshift working manually → Pacman hook → Menu integration → Status display

**Dependencies:**
- Menu integration requires working snapshots
- Status display requires snapshot metadata access
- Pacman hook requires Timeshift CLI tested

**Research flags:**
- GRUB rsync boot entries (if needed) — very complex, defer
- Diff browsing UI (if desired) — medium complexity, not critical

**Confidence level:** HIGH for core features, MEDIUM for custom integrations.

---

*Research complete. Ready for requirements definition and roadmap creation.*
