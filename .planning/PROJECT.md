# VulcanOS Foundation

## What This Is

A personal system management layer for a 2019 T2 MacBook Pro running Arch Linux with Hyprland. VulcanOS provides comprehensive backup/sync, T2 kernel protection, and organized tooling for a single-user development workstation. Future versions may target other systems, but this milestone is specifically for one machine.

## Core Value

**Never lose work or boot capability.** Pre-update snapshots and kernel pinning ensure the system is always recoverable, while automated sync keeps configs and data backed up.

## Requirements

### Validated

Existing capabilities from current codebase:

- ✓ Arch Linux ISO build system (archiso profile) — existing
- ✓ GNU Stow dotfile management structure — existing
- ✓ Hyprland compositor with modular config — existing
- ✓ Waybar status bar with custom modules — existing
- ✓ vulcan-todo MCP server (tasks, sprints, projects) — existing
- ✓ vulcan-vault MCP server (notes, semantic search) — existing
- ✓ Desktop integration scripts (vulcan-menu, themes, etc.) — existing
- ✓ T2 MacBook Pro hardware support (linux-t2 kernel) — existing
- ✓ Speech-to-text via hyprwhspr — existing

### Active

New requirements for this milestone:

**Sync/Backup System:**
- [ ] Dotfiles sync to Git repository (version controlled)
- [ ] Package list export and sync to Git (reproducible installs)
- [ ] Personal data backup to local external drive
- [ ] Full system snapshots via Timeshift rsync mode
- [ ] Sensitive file exclusion patterns (.env, SSH keys, GPG)
- [ ] Sync status display (last sync time, snapshot count)

**T2 Kernel Protection:**
- [ ] Pre-update snapshots before pacman operations
- [ ] linux-t2 kernel version pinning
- [ ] Easy rollback mechanism after failed updates
- [ ] Pacman hook integration for automatic protection

**Menu Integration:**
- [ ] Sync/backup submenu in vulcan-menu
- [ ] Quick actions: Sync Now, Create Snapshot, Restore
- [ ] Status display in submenu (last sync, snapshot count)
- [ ] Waybar icon linking to vulcan-menu (top left)

**Codebase Organization:**
- [ ] Consolidate scattered scripts into organized structure
- [ ] Clear directory layout for vulcan-* tools
- [ ] Establish patterns for new tool development
- [ ] Documentation for tool locations and purposes

### Out of Scope

- Multi-machine sync — this is for one specific T2 MacBook Pro
- Cloud backup destinations — local external drive only for now
- Btrfs migration — staying on ext4, using rsync-based snapshots
- Universal installer — no support for non-T2 hardware this milestone
- Encrypted backup vaults — excluding sensitive files instead

## Context

**Hardware:** 2019 MacBook Pro 16" with T2 chip. Requires linux-t2 kernel from arch-mact2 repo. Kernel updates are risky because wrong kernel = unbootable system.

**Filesystem:** Root on ext4 (no btrfs snapshots available). Must use rsync-based backup tools like Timeshift.

**Existing tools:**
- `vulcan-menu` — main system menu (wofi-based)
- `vulcan-todo` / `vulcan-vault` — Rust MCP servers for AI integration
- Various `vulcan-*` scripts scattered in dotfiles and archiso

**Current pain points:**
- Scripts spread across `dotfiles/scripts/`, `archiso/airootfs/usr/local/bin/`, and other locations
- No automated backup — manual git commits for dotfiles
- Kernel updates have broken the system before (lost mount)
- No clear structure for adding new system tools

## Constraints

- **Hardware:** T2 MacBook Pro 2019 only — no other machines targeted
- **Filesystem:** ext4 root — must use rsync-based snapshots, not btrfs
- **Kernel:** linux-t2 from arch-mact2 — cannot use mainline kernel
- **Storage:** External SSD for backups — no cloud dependencies
- **Security:** Exclude sensitive files — no encryption layer this milestone

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Timeshift rsync for snapshots | ext4 filesystem, battle-tested tool, CLI available | — Pending |
| Git for dotfiles/packages | Version control, easy diff, reproducible | — Pending |
| Local drive for data/snapshots | Fast restore, no cloud dependency, simple | — Pending |
| Exclude vs encrypt sensitive | Simpler implementation, manual key management | — Pending |
| Pacman hooks for auto-snapshot | Automatic protection before risky updates | — Pending |

---
*Last updated: 2026-01-23 after initialization*
