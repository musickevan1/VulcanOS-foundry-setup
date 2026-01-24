# Project Research Summary

**Project:** VulcanOS Backup & Sync System
**Domain:** Linux System Backup Integration for T2 MacBook Pro
**Researched:** 2026-01-23
**Confidence:** HIGH

## Executive Summary

VulcanOS requires a backup system that protects against the unique risks of T2 MacBook Pro kernel updates while maintaining the distribution's philosophy of simplicity and local control. Research shows the optimal approach combines **Timeshift** (system snapshots via rsync), **pacman hooks** (pre-update automation), and **desktop integration** (wofi menu + waybar status) without requiring cloud services or complex setup.

The recommended architecture is hook-driven and modular: pacman hooks trigger backups before risky operations (kernel updates), a backup engine manages rsync operations, and desktop components provide user visibility through VulcanOS's existing menu system. This leverages Timeshift's recent move to official Arch repos (v25.12.4) and proven ext4 snapshot patterns without requiring filesystem migration.

Critical risks center on T2 hardware: installing the wrong kernel makes systems completely unbootable with no keyboard/trackpad recovery path. Secondary risks include external drive disconnects during backup, unmounted /boot during updates, and untested restoration procedures. Mitigation requires pre-flight verification hooks, robust drive handling, and automated validation - all integrated into VulcanOS's existing workflows.

## Key Findings

### Recommended Stack

For ext4-based system protection without btrfs, the stack centers on **Timeshift rsync mode** (official Arch package as of Dec 2024) combined with **timeshift-autosnap** pacman hooks for pre-update snapshots. GNU Stow remains unchanged for dotfiles (already deployed), and pacman's native package list export (`-Qqen`/`-Qqem`) handles reproducible installs.

**Core technologies:**
- **Timeshift 25.12.4-1** (official repo): Full system snapshots via rsync + hardlinks — mature, GUI+CLI, T2-compatible
- **timeshift-autosnap 0.10.0-1** (AUR): Pacman hook integration — automatic pre-update snapshots with retention policies
- **GNU Stow 2.4.1-1**: Dotfile symlinks — already in production, no changes needed
- **pacman built-in**: Package list export — native `-Qqen`/`-Qqem` flags, git versioning
- **rsync 3.4.1-2**: Snapshot engine — Timeshift backend, efficient hardlink-based incremental backups

**Optional for user data:**
- **Restic 0.18.1-1**: Encrypted backups to external/cloud — if /home needs offsite protection
- **Borg 1.4.3-2**: Deduplicating backups — if local external drive preferred over cloud

**Key decision:** Timeshift in RSYNC mode (not btrfs mode) because VulcanOS uses ext4. Each snapshot is a full browsable backup sharing unchanged files via hardlinks. First snapshot takes 10-30min, incrementals take 30sec-2min.

### Expected Features

Research identified three feature categories: table stakes (expected by all backup tools), differentiators (competitive advantages), and anti-features (explicitly avoid).

**Must have (table stakes):**
- Snapshot creation/restoration — core functionality, single command or button
- Automated scheduling — hourly/daily/weekly intervals via systemd timers
- Exclusion filters — skip /tmp, ~/.cache, build artifacts (node_modules, target/)
- Retention policies — automatic cleanup (keep last N daily/weekly/monthly)
- External drive support — backup to local SSD/HDD for offline protection
- Boot snapshot — pre-update protection before risky kernel changes

**Should have (competitive/VulcanOS-specific):**
- Pre-pacman hook — automatic snapshot before kernel updates (critical for T2)
- Quick action GUI — wofi submenu integration (Create/Restore/Status)
- Status reporting — "Last backup: 2h ago, 5 snapshots" visible in menu/waybar
- Notification alerts — desktop notifications via swaync on success/failure
- Package list export — sync pacman -Qqe output to git for reproducibility

**Defer (v2+):**
- Single file restore — can browse snapshots manually via file manager for now
- Diff browsing — compare snapshots file-by-file (high complexity, low ROI initially)
- GRUB boot entries — boot into old snapshot from GRUB (very complex for rsync mode)
- Git auto-commit — automatic dotfile commits after backup (can do manually initially)
- Cloud sync — explicitly out of scope per VulcanOS constraints

**Anti-features (explicitly avoid):**
- Cloud-only backup — support local external drives first (VulcanOS philosophy)
- Quota tracking — performance overhead, simple disk checks sufficient for ext4
- Setup wizards — sane defaults, optional config (Timeshift works zero-config)
- Encryption required — exclude sensitive files instead (lower complexity)
- GUI-only interface — always provide CLI equivalents for scripting

### Architecture Approach

The backup system should follow a **hook-driven, event-based architecture** that integrates with VulcanOS's existing desktop environment (Hyprland + wofi + waybar) without disrupting workflows. Components communicate via state files, notifications flow through swaync, and automation triggers on package manager events.

**Major components:**
1. **Pacman Hooks** (`/etc/pacman.d/hooks/*.hook`) — Trigger backups on kernel/critical package updates (PreTransaction), fast metadata snapshots only
2. **Backup Engine** (`/usr/local/bin/vulcan-backup`) — Execute rsync operations, manage state files, call Timeshift CLI or direct rsync
3. **Menu Integration** (`dotfiles/scripts/.local/bin/vulcan-menu`) — Add backup submenu to existing VulcanOS menu with Create/Restore/List actions
4. **Waybar Module** (`custom/backup`) — Display status (last backup time, snapshot count) with JSON output from status script
5. **Notification Layer** (via `notify-send`) — User feedback at all stages (start, progress, completion, errors) through swaync
6. **Configuration** (`~/.config/vulcan-backup/config.toml`) — User settings for schedules, retention, external drive paths

**Data flow:** User action (pacman -Syu) → Pacman PreTransaction hook → Backup engine → Storage backend → Notify user → Waybar updates status. Manual workflow: vulcan-menu → Backup submenu (wofi) → Action → Backup engine → Notification.

**Critical patterns:** Fast PreTransaction hooks (<5 seconds), state-based UI (waybar reads files, doesn't execute), non-blocking PostTransaction operations, consistent with VulcanOS UX (wofi menus, nerd fonts, notification style).

### Critical Pitfalls

**1. Wrong Kernel Installed (T2-CRITICAL)** — Pacman updates to mainline `linux` instead of `linux-t2`. System boots but no keyboard/trackpad/WiFi = completely unrecoverable without USB keyboard. **Prevention:** Add `IgnorePkg = linux linux-headers linux-lts` to `/etc/pacman.conf`, create verification hook to check kernel package name before upgrade.

**2. /boot Unmounted During Kernel Update** — Kernel installs to pacman database but files don't reach EFI partition. Boot fails or uses old kernel with new modules (driver mismatch). **Prevention:** PreTransaction hook with `mountpoint -q /boot` + `AbortOnFail` to halt updates if /boot not mounted.

**3. External Drive Disconnects During Backup** — USB autosuspend or T2 controller issues cause drive to disconnect mid-rsync. Backup appears complete but is corrupted. User discovers during disaster recovery. **Prevention:** Disable USB autosuspend via udev rules for backup drive, mount with `sync` option, verify drive still connected after rsync completes.

**4. No Fallback Kernel in GRUB** — New linux-t2 kernel has regression (WiFi breaks, suspend fails). No old kernel to boot. Forced to use live USB for every kernel issue. **Prevention:** Keep previous kernel version (`CleanMethod = KeepCurrent` in pacman.conf), configure GRUB timeout to show menu, or manually preserve working kernel to /boot/vmlinuz-linux-t2.backup.

**5. mkinitcpio Hook Failure Goes Unnoticed** — Kernel package updates but initramfs generation fails silently (full /boot, module errors). System boots to kernel panic. **Prevention:** PostTransaction verification hook checks initramfs exists, is recent (<5min old), and is reasonable size (>10MB).

## Implications for Roadmap

Based on research, suggested 4-phase structure prioritizing T2 safety, then automation, then integration, then polish:

### Phase 1: Pre-flight Safety (T2 Kernel Protection)
**Rationale:** T2 kernel mistakes are catastrophic (unbootable, no recovery path). Must implement safeguards before adding any backup features. This is the foundation that makes all other phases safe.

**Delivers:**
- Pacman hook to verify /boot is mounted before kernel updates
- Pacman hook to verify correct kernel package (linux-t2, not linux)
- GRUB configuration with fallback kernel entries and visible menu
- PostTransaction hook to verify initramfs generation succeeded
- Documentation on T2 kernel parameters and IgnorePkg settings

**Addresses (from FEATURES.md):**
- Boot snapshot (table stakes) — ensures system bootable before risky operations
- Pre-pacman hook (differentiator) — T2-specific protection

**Avoids (from PITFALLS.md):**
- Pitfall 1: Wrong kernel installed (T2-CRITICAL)
- Pitfall 2: /boot unmounted during update
- Pitfall 5: No fallback kernel in GRUB
- Pitfall 6: mkinitcpio failure goes unnoticed
- Pitfall 9: GRUB updates without T2 parameters

**Research flags:** Standard patterns (pacman hooks well-documented), no additional research needed.

### Phase 2: Core Backup Engine (Timeshift Integration)
**Rationale:** With safety hooks in place, implement the backup mechanism itself. Start with manual operations before adding automation. Validate Timeshift works correctly on VulcanOS before exposing to users.

**Delivers:**
- Timeshift installed and configured (rsync mode, ext4)
- vulcan-backup script wrapper around Timeshift CLI
- Snapshot creation (manual CLI first, then interactive)
- Snapshot listing and restoration (manual process)
- Exclusion patterns (cache, temp, build dirs, sensitive files)
- External drive mount detection and handling

**Addresses (from FEATURES.md):**
- Snapshot creation/restoration (table stakes)
- Exclusion filters (table stakes)
- External drive support (table stakes)

**Uses (from STACK.md):**
- Timeshift 25.12.4-1 (official repo)
- rsync 3.4.1-2 (backend)
- Configuration in ~/.config/vulcan-backup/

**Avoids (from PITFALLS.md):**
- Pitfall 3: Backup while system running (document rsync limitations for ext4)
- Pitfall 4: External drive disconnects (USB autosuspend disable, sync mount)
- Pitfall 7: Pacman database lock (exclude /var/lib/pacman/db.lck)
- Pitfall 8: Missing extended attributes (use rsync -aAXv)
- Pitfall 12: Huge pacman cache backed up (exclude /var/cache/pacman/pkg)

**Research flags:** Need testing on actual T2 hardware (USB-C quirks, drive detection), otherwise standard.

### Phase 3: Desktop Integration (Menu + Status)
**Rationale:** Backup engine works, now make it accessible through VulcanOS's existing UI patterns. Users need visibility without opening terminal. Integration should feel native, not bolted-on.

**Delivers:**
- vulcan-menu backup submenu (wofi integration)
- Waybar custom module showing backup status
- Status script (last backup time, snapshot count, drive status)
- Desktop notifications via swaync (start, complete, error)
- Quick actions: Create Backup, Restore Backup, List Backups, Configure

**Addresses (from FEATURES.md):**
- Quick action GUI (differentiator)
- Status reporting (differentiator)
- Notification alerts (differentiator)
- Menu integration (differentiator)

**Implements (from ARCHITECTURE.md):**
- Wofi submenu integration (follow existing VulcanOS patterns)
- Waybar custom module (JSON return type, state files)
- Notification layer (notify-send → swaync)
- Script organization (dotfiles/scripts/.local/bin/)

**Avoids (from PITFALLS.md):**
- Pitfall 11: Backup notifications ignored (desktop notifications, not email)
- Anti-pattern: Multiple menu systems (extend vulcan-menu, don't create new tool)

**Research flags:** Standard patterns (Waybar custom modules documented, wofi patterns established in VulcanOS).

### Phase 4: Automation & Validation
**Rationale:** Manual backups work, UI is polished, now add automation for scheduled snapshots and pre-update protection. Include validation to prevent "set and forget" false security.

**Delivers:**
- timeshift-autosnap pacman hook (pre-update snapshots)
- Systemd timer for scheduled backups (daily/weekly)
- Retention policies (keep last N snapshots)
- Package list export automation (pacman -Qqen → git)
- Backup validation testing (verify restore process)
- ISO integration (copy scripts to archiso skeleton)

**Addresses (from FEATURES.md):**
- Automated scheduling (table stakes)
- Retention policies (table stakes)
- Pre-pacman hook (differentiator, now automated)
- Package list export (differentiator)

**Uses (from STACK.md):**
- timeshift-autosnap 0.10.0-1 (AUR)
- cronie 1.7.2-2 (systemd timer alternative)
- Git for package list versioning

**Avoids (from PITFALLS.md):**
- Pitfall 10: Forgotten to test backup restoration (automated monthly VM test)
- Anti-pattern: Blocking pacman indefinitely (fast PreTransaction, full backup PostTransaction)
- Anti-pattern: Silent failures (notify on error with critical urgency)

**Research flags:** Need validation testing procedures, otherwise standard automation patterns.

### Phase Ordering Rationale

- **Phase 1 first** because T2 kernel mistakes are unrecoverable without live USB + external keyboard. Safeguards must exist before touching anything kernel-related.
- **Phase 2 before 3** because backup engine must work correctly before exposing to users via GUI. Manual testing validates the mechanism.
- **Phase 3 before 4** because users need to understand and test backup process manually before automation runs silently. UI provides visibility for debugging automation issues.
- **Phase 4 last** because automation multiplies the impact of bugs. Only automate after manual process is proven reliable.

**Dependency chain:** Pre-flight hooks → Backup engine → Desktop UI → Automation. Each phase validates the previous, each phase is independently useful.

### Research Flags

**Phases needing deeper research during planning:**
- None — all phases use well-documented patterns. Pacman hooks, Timeshift rsync mode, Waybar custom modules, and systemd timers are standard Arch practices with extensive documentation.

**Phases with standard patterns (skip research-phase):**
- **Phase 1:** Pacman hooks documented in Arch Wiki + alpm-hooks(5) manual, GRUB configuration standard
- **Phase 2:** Timeshift official documentation, rsync widely understood, ext4 standard
- **Phase 3:** Waybar custom modules documented, wofi patterns established in VulcanOS codebase
- **Phase 4:** timeshift-autosnap well-documented AUR package, systemd timers standard

**Areas needing validation (not research):**
- T2 hardware USB-C quirks (test on actual hardware)
- External drive disconnect scenarios (practical testing)
- Restore procedures (VM testing, monthly validation)
- VulcanOS menu integration (code review against existing patterns)

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | **HIGH** | Timeshift official Arch package (Dec 2024), extensive Wiki docs, rsync proven for ext4 |
| Features | **HIGH** | Table stakes validated across Timeshift/Snapper/Borg comparisons, differentiators from snap-pac/btrfs-assistant patterns |
| Architecture | **HIGH** | Pacman hooks well-documented (alpm-hooks manual), Waybar/wofi patterns exist in VulcanOS codebase |
| Pitfalls | **HIGH** | T2 kernel issues documented in t2linux wiki, backup pitfalls from Arch forums + official guides |

**Overall confidence:** **HIGH**

All recommendations backed by official documentation (Arch Wiki, t2linux wiki) or established community consensus (Arch forums, proven tools). No experimental approaches or unverified patterns.

### Gaps to Address

**T2 USB-C controller quirks** — Community reports mention USB disconnects under load, but no definitive mitigation documented beyond USB autosuspend disable. **Resolution:** Test on actual hardware during Phase 2, document findings, may need kernel parameter tweaks or udev rules refinement.

**Rsync consistency on live ext4 system** — Research confirms rsync-while-running creates inconsistent snapshots (database files, logs), but ext4 has no native snapshot support. **Resolution:** Document limitations clearly, recommend stopping critical services (Docker, databases) before backup, or note this as known limitation of ext4 vs btrfs. For VulcanOS (desktop workstation, not server), acceptable for Phase 2, consider btrfs migration in future milestone.

**GRUB rsync boot entries complexity** — Research shows booting into rsync-based snapshots is significantly harder than btrfs snapshots (requires chroot restore, not direct boot). **Resolution:** Defer to post-MVP, document manual restore procedure, focus on preventing need to restore rather than optimizing restoration UX.

**Waybar refresh behavior** — Need to validate exec-on-event and interval interaction for status updates. **Resolution:** Follow hyprwhspr module pattern (lines 68-75 in config.jsonc), test during Phase 3.

**Package list restoration edge cases** — AUR packages may fail to reinstall if removed from AUR or dependency changes. **Resolution:** Document as known limitation, recommend review of AUR list before mass reinstall, keep pacman cache for local reinstall fallback.

## Sources

### Primary (HIGH confidence)
- [Arch Wiki: System backup](https://wiki.archlinux.org/title/System_backup) — backup strategies, tool comparisons
- [Arch Wiki: Timeshift](https://wiki.archlinux.org/title/Timeshift) — configuration, rsync mode, ext4 usage
- [Arch Wiki: pacman](https://wiki.archlinux.org/title/Pacman) — hooks system, package lists
- [alpm-hooks(5) manual](https://man.archlinux.org/man/alpm-hooks.5) — hook syntax, PreTransaction/PostTransaction
- [t2linux wiki: Post-installation](https://wiki.t2linux.org/guides/postinstall/) — T2 kernel requirements
- [t2linux wiki: Arch FAQ](https://wiki.t2linux.org/distributions/arch/faq/) — kernel updating, IgnorePkg
- [Waybar Custom Module Docs](https://man.archlinux.org/man/extra/waybar/waybar-custom.5.en) — JSON format, exec-on-event
- [Arch Linux Packages: timeshift 25.12.4-1](https://archlinux.org/packages/extra/x86_64/timeshift/) — version verification
- [GitHub: teejee2008/timeshift](https://github.com/teejee2008/timeshift) — official repository, features
- [GitHub: dagorret/timeshift-autosnap](https://github.com/dagorret/timeshift-autosnap) — pacman hook integration

### Secondary (MEDIUM confidence)
- [AUR: timeshift-autosnap 0.10.0-1](https://aur.archlinux.org/packages/timeshift-autosnap) — package details, configuration
- [Arch Forums: T2 Macbook kernel updates](https://bbs.archlinux.org/viewtopic.php?id=299034) — community experiences
- [Arch Forums: Pacman hook to backup /boot](https://bbs.archlinux.org/viewtopic.php?id=289248) — hook examples
- [Arch Forums: rsync system backup](https://bbs.archlinux.org/viewtopic.php?id=277249) — best practices, pitfalls
- [Backup Speed Benchmark: rsync vs borg vs restic](https://grigio.org/backup-speed-benchmark/) — performance comparisons
- [GitHub: wesbarnett/snap-pac](https://github.com/wesbarnett/snap-pac) — pattern reference for pacman hooks
- [Hyprland Wiki: App Launchers](https://wiki.hypr.land/Useful-Utilities/App-Launchers/) — wofi integration patterns
- [Red Hat: 5 Linux backup tips](https://www.redhat.com/sysadmin/5-backup-tips) — best practices

### Tertiary (LOW confidence)
- [The Terrors of Linux on a T2 Mac](https://awpsec.medium.com/the-terrors-of-linux-on-a-t2-mac-9b66699a8693) — anecdotal T2 issues
- [Timeshift vs Snapper comparison](https://www.compsmag.com/vs/timeshift-vs-snapper/) — feature comparison (not official)
- [25 Best Backup Tools for Linux](https://www.tecmint.com/linux-system-backup-tools/) — general overview

---
*Research completed: 2026-01-23*
*Ready for roadmap: yes*
