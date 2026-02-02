# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-02)

**Core value:** Cohesive, recoverable, keyboard-driven
**Current focus:** v3.0 Multi-Profile + AI Workstation — Phase 14

## Current Position

Phase: 14 of 22 (Multi-Profile Build Infrastructure)
Plan: 3 of ~8 complete
Status: In progress
Last activity: 2026-02-02 — Completed 14-03-PLAN.md (T2 profile configuration files)

Progress: [████████████████████░░░░] 53/~68 plans (v1.0-v2.1 complete, v3.0 in progress)

## Performance Metrics

**Velocity:**
- Total plans completed: 53 (v1.0 + v2.0 + v2.1 + v3.0)
- v1.0 Foundation: 11 plans
- v2.0 Appearance Manager: 28 plans
- v2.1 Maintenance: 9 plans
- v3.0 Multi-Profile: 3 plans (14-01, 14-02, 14-03 complete)

**By Milestone:**

| Milestone | Phases | Plans | Status |
|-----------|--------|-------|--------|
| v1.0 Foundation | 2 | 11 | Shipped 2026-01-24 |
| v2.0 Appearance Manager | 5 | 28 | Shipped 2026-01-30 |
| v2.1 Maintenance | 3 | 9 | Shipped 2026-02-01 |
| v3.0 Multi-Profile | 9 | TBD | In progress |

**Recent Trend:**
- v2.0 completed in 6 days (5 phases, 28 plans)
- v2.1 completed in 2 days (3 phases, 9 plans)
- v3.0 is larger scope (9 phases, 69 requirements)

## Accumulated Context

### Decisions

Recent decisions from Phase 14:

| Phase | Decision | Rationale | Outcome |
|-------|----------|-----------|---------|
| 14-01 | Multi-profile directory structure | archiso/base/ for shared, archiso/profiles/{t2,foundry}/ for specific | Clean separation per CONTEXT.md |
| 14-01 | rsync overlay for airootfs | Profile files override base on conflict | Handles permissions and precedence correctly |
| 14-01 | Package list merging | cat + grep + sort -u for deduplication | Simple, reliable merge without duplicates |
| 14-03 | arch-mact2 repo FIRST in T2 pacman.conf | Repository order determines package priority | linux-t2 kernel takes precedence over linux |
| 14-03 | Copy boot configs as-is | Current configs already T2-specific | No modification needed, preserves tested config |

All decisions are also logged in PROJECT.md Key Decisions table.

### Pending Todos

None yet.

### Blockers/Concerns

**Technical:**
- RTX 5070 Ti (Blackwell) requires nvidia-open-dkms, not nvidia-dkms
- PyTorch stable does NOT support sm_120 — must use nightly builds
- PCIe Gen1 fallback bug may affect Foundry — needs BIOS verification

**Hardware:**
- Vulcan Foundry hardware not yet assembled/available
- Phase 15+ cannot be fully validated until hardware arrives

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
|---|-------------|------|--------|-----------|
| 001 | Fix workspace keybinds for dynamic monitors | 2026-02-01 | 4b6709e | [001-fix-workspace-keybinds-dynamic-monitors](./quick/001-fix-workspace-keybinds-dynamic-monitors/) |
| 002 | Create VulcanOS VS Code theme | 2026-02-01 | 906f33f | [002-create-vulcanos-vscode-theme](./quick/002-create-vulcanos-vscode-theme/) |

## Session Continuity

Last session: 2026-02-02
Stopped at: Completed 14-03-PLAN.md (T2 profile configuration files)
Resume file: None

Next: Continue with remaining Phase 14 plans (14-04+)
