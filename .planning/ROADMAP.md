# Roadmap: VulcanOS Foundation

## Overview

VulcanOS Foundation builds a comprehensive backup and sync system for a T2 MacBook Pro running Arch Linux, prioritizing system recoverability above all else. The journey begins with kernel protection (preventing catastrophic boot failures unique to T2 hardware), then implements manual backup operations, adds desktop integration for visibility and quick actions, and finally automates the proven workflows with validation testing.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: T2 Kernel Protection** - Prevent catastrophic boot failures from kernel updates
- [ ] **Phase 2: Core Backup Engine** - Manual snapshot and restore capabilities via Timeshift
- [ ] **Phase 3: Desktop Integration** - Menu system and status display for backup operations
- [ ] **Phase 4: Automation & Validation** - Scheduled backups and pre-update protection

## Phase Details

### Phase 1: T2 Kernel Protection
**Goal**: System is protected from catastrophic kernel update failures that make T2 MacBook Pro unbootable
**Depends on**: Nothing (first phase)
**Requirements**: T2-01, T2-02, T2-03, T2-04
**Success Criteria** (what must be TRUE):
  1. User cannot accidentally install mainline linux kernel (pacman refuses upgrade)
  2. User sees warning if kernel package would change during system update
  3. Pacman operation aborts if /boot is not mounted before kernel update
  4. User can verify initramfs was generated successfully after kernel operations
  5. GRUB menu shows previous kernel version as fallback boot option
**Plans**: 3 plans in 2 waves

Plans:
- [ ] 01-01-PLAN.md — Protection hooks and scripts (blocks mainline kernel, /boot check, warnings)
- [ ] 01-02-PLAN.md — Verification and fallback (boot chain verify, GRUB fallback entries)
- [ ] 01-03-PLAN.md — Archiso sync and end-to-end testing

### Phase 2: Core Backup Engine
**Goal**: User can create, list, restore, and manage system snapshots manually using proven backup tools
**Depends on**: Phase 1
**Requirements**: SNAP-01, SNAP-02, SNAP-03, SNAP-04, SNAP-05, SYNC-01, SYNC-02, SYNC-03, SYNC-04, DATA-01, DATA-02
**Success Criteria** (what must be TRUE):
  1. User can create full system snapshot from command line in under 5 minutes (incremental)
  2. User can list all existing snapshots with creation date, size, and description
  3. User can restore system from selected snapshot to previous working state
  4. User can delete old snapshots to reclaim disk space
  5. Dotfiles are synced to Git repository with each backup operation
  6. Package lists (explicit and AUR) are exported and versioned in Git
  7. Sensitive files (.env, SSH keys, GPG) are automatically excluded from backups
  8. Personal data folders are backed up incrementally to external drive
**Plans**: TBD

Plans:
- TBD (will be created during `/gsd:plan-phase 2`)

### Phase 3: Desktop Integration
**Goal**: Backup operations are accessible through native VulcanOS desktop interface with status visibility
**Depends on**: Phase 2
**Requirements**: UI-01, UI-02, UI-03, UI-04, ORG-01, ORG-02, ORG-03
**Success Criteria** (what must be TRUE):
  1. User can access backup submenu from vulcan-menu (Super+Space → Backup)
  2. Waybar icon in top-left displays current backup status (snapshot count, last sync time)
  3. Desktop notifications appear on backup start, completion, and errors
  4. User can trigger "Create Snapshot", "Restore", "List Backups" from menu without terminal
  5. All backup scripts follow vulcan-* naming convention and live in organized directories
  6. Documentation describes where each tool lives and what it does
**Plans**: TBD

Plans:
- TBD (will be created during `/gsd:plan-phase 3`)

### Phase 4: Automation & Validation
**Goal**: Backup system runs automatically before risky operations with scheduled snapshots and tested restoration
**Depends on**: Phase 3
**Requirements**: (All Phase 1-3 requirements automated or validated)
**Success Criteria** (what must be TRUE):
  1. System snapshot is created automatically before every pacman operation that modifies kernel packages
  2. Daily snapshots are created automatically via systemd timer
  3. Old snapshots are automatically removed based on retention policy (keep last 7 daily, 4 weekly)
  4. Package lists are exported to Git automatically after system updates
  5. Backup restoration procedure has been tested and documented with validation steps
  6. Backup scripts are integrated into archiso skeleton for fresh installs
**Plans**: TBD

Plans:
- TBD (will be created during `/gsd:plan-phase 4`)

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. T2 Kernel Protection | 0/3 | Planned | - |
| 2. Core Backup Engine | 0/? | Not started | - |
| 3. Desktop Integration | 0/? | Not started | - |
| 4. Automation & Validation | 0/? | Not started | - |

---
*Roadmap created: 2026-01-23*
*Last updated: 2026-01-23 after Phase 1 planning*
