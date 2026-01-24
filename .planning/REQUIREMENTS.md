# Requirements: VulcanOS Foundation

**Defined:** 2026-01-23
**Core Value:** Never lose work or boot capability

## v1 Requirements

Requirements for this milestone. Each maps to roadmap phases.

### T2 Kernel Protection

- [ ] **T2-01**: Pre-update snapshot created automatically before any pacman operation
- [ ] **T2-02**: linux-t2 kernel version pinned to prevent accidental upgrades
- [ ] **T2-03**: Warning displayed if kernel package would change during update
- [ ] **T2-04**: Pacman operation aborted if safety conditions not met (no snapshot, disk full)

### System Snapshots

- [ ] **SNAP-01**: User can create system snapshot manually via CLI and menu
- [ ] **SNAP-02**: User can list existing snapshots with creation date and size
- [ ] **SNAP-03**: User can restore system from selected snapshot
- [ ] **SNAP-04**: User can delete old snapshots to free space
- [ ] **SNAP-05**: Snapshot status displayed (count, last snapshot time, disk usage)

### Dotfile Sync

- [ ] **SYNC-01**: User can sync dotfiles to Git repository on demand
- [ ] **SYNC-02**: Installed package list exported to Git with each sync
- [ ] **SYNC-03**: Sensitive files excluded via patterns (.env, SSH keys, GPG)
- [ ] **SYNC-04**: Manual sync triggered via CLI or menu action

### Personal Data Backup

- [ ] **DATA-01**: User can backup specified folders to external drive
- [ ] **DATA-02**: Backups are incremental (only changed files via rsync)

### Desktop Integration

- [ ] **UI-01**: Sync/backup submenu appears in vulcan-menu
- [ ] **UI-02**: Waybar icon in top-left links to vulcan-menu
- [ ] **UI-03**: Status display shows last sync time, snapshot count in submenu
- [ ] **UI-04**: Desktop notification shown on backup/sync completion

### Codebase Organization

- [ ] **ORG-01**: Scripts consolidated into organized directory structure
- [ ] **ORG-02**: All tools follow vulcan-* naming convention
- [ ] **ORG-03**: Documentation describes tool locations and purposes

## v2 Requirements

Deferred to future milestone. Tracked but not in current roadmap.

### Automation

- **AUTO-01**: Scheduled automatic snapshots (daily/weekly)
- **AUTO-02**: Scheduled automatic dotfile sync
- **AUTO-03**: Pre-configured snapshot retention policy
- **AUTO-04**: Scheduled personal data backup

### Advanced Restore

- **REST-01**: GRUB boot menu entry for last-known-good snapshot
- **REST-02**: Restore dry-run preview before actual restore
- **REST-03**: Automatic rollback script after failed update

### Enhanced Backup

- **BACK-01**: External drive mount detection with auto-backup prompt
- **BACK-02**: Encryption for sensitive personal data
- **BACK-03**: Multiple backup destinations (local + cloud)
- **BACK-04**: Diff preview before dotfile sync

### Universal Support

- **UNIV-01**: Support for non-T2 machines
- **UNIV-02**: Btrfs native snapshot support
- **UNIV-03**: Multi-machine sync coordination

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Cloud backup destinations | Local external drive only for v1, cloud adds complexity |
| Btrfs migration | Staying on ext4, rsync-based approach works fine |
| Multi-machine sync | This is for one specific T2 MacBook Pro |
| Encrypted backup vault | Excluding sensitive files instead, simpler |
| Automatic scheduled backups | Manual trigger first, automation after validation |
| Non-T2 hardware support | Personal machine focus, universal later |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| T2-01 | Phase 1 | Pending |
| T2-02 | Phase 1 | Pending |
| T2-03 | Phase 1 | Pending |
| T2-04 | Phase 1 | Pending |
| SNAP-01 | Phase 2 | Pending |
| SNAP-02 | Phase 2 | Pending |
| SNAP-03 | Phase 2 | Pending |
| SNAP-04 | Phase 2 | Pending |
| SNAP-05 | Phase 2 | Pending |
| SYNC-01 | Phase 2 | Pending |
| SYNC-02 | Phase 2 | Pending |
| SYNC-03 | Phase 2 | Pending |
| SYNC-04 | Phase 2 | Pending |
| DATA-01 | Phase 2 | Pending |
| DATA-02 | Phase 2 | Pending |
| UI-01 | Phase 3 | Pending |
| UI-02 | Phase 3 | Pending |
| UI-03 | Phase 3 | Pending |
| UI-04 | Phase 3 | Pending |
| ORG-01 | Phase 3 | Pending |
| ORG-02 | Phase 3 | Pending |
| ORG-03 | Phase 3 | Pending |

**Coverage:**
- v1 requirements: 22 total
- Mapped to phases: 22
- Unmapped: 0

**Note:** Phase 4 (Automation & Validation) validates and automates Phase 1-3 requirements rather than introducing new requirements.

---
*Requirements defined: 2026-01-23*
*Last updated: 2026-01-23 after roadmap creation (traceability complete)*
