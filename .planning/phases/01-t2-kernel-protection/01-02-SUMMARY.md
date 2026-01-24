---
phase: 01-t2-kernel-protection
plan: 02
subsystem: infra
tags: [pacman-hooks, kernel-protection, grub, boot-chain, t2-macbook]

# Dependency graph
requires:
  - phase: 01-t2-kernel-protection
    provides: Domain understanding and phase plan

provides:
  - PostTransaction verification hook that checks boot chain integrity after kernel operations
  - Manual verification command for checking kernel, initramfs, T2 modules, and GRUB config
  - GRUB fallback generator that backs up current kernel before updates
  - Two-level fallback system with automatic rotation
  - Desktop notifications for verification results

affects: [01-03-abort-hooks, 01-04-warning-hooks]

# Tech tracking
tech-stack:
  added: [lsinitcpio, grub-script-check, notify-send]
  patterns: [pacman-hooks, kernel-verification, grub-custom-config]

key-files:
  created:
    - archiso/airootfs/etc/pacman.d/hooks/90-vulcan-kernel-verify.hook
    - archiso/airootfs/usr/local/bin/vulcan-kernel-verify
    - archiso/airootfs/usr/local/bin/vulcan-kernel-fallback
    - archiso/airootfs/boot/grub/custom.cfg
  modified: []

key-decisions:
  - "PostTransaction hook runs AFTER kernel operations (verification only, damage already done)"
  - "Two fallback kernels with automatic rotation (backup → backup2 → remove oldest)"
  - "GRUB custom.cfg sourced automatically by /etc/grub.d/41_custom (no grub-mkconfig needed)"
  - "Desktop notifications persist until user dismisses (critical results need attention)"
  - "Exit codes: 0 for success/warnings, 1 for critical errors"

patterns-established:
  - "Verification scripts use colored terminal output with ok/warn/error functions"
  - "All scripts log to /var/log/vulcan-*.log with timestamps"
  - "Scripts detect system configuration automatically (root UUID, kernel params)"
  - "Human-readable file sizes in output (numfmt --to=iec-i)"

# Metrics
duration: 3m 52s
completed: 2026-01-24
---

# Phase 01 Plan 02: T2 Kernel Verification and Fallback Summary

**PostTransaction verification hook checks boot chain integrity and fallback generator maintains two previous kernel versions with GRUB entries**

## Performance

- **Duration:** 3m 52s
- **Started:** 2026-01-24T04:23:31Z
- **Completed:** 2026-01-24T04:27:23Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Created PostTransaction pacman hook that automatically verifies boot chain after kernel operations
- Implemented comprehensive verification script checking kernel, initramfs, T2 modules, GRUB config, and fallback status
- Built fallback generator that backs up current kernel and creates GRUB menu entries
- Established two-level fallback system with automatic rotation (backup → backup2 → remove oldest)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create PostTransaction Verification Hook and Script** - `5f64734` (feat)
2. **Task 2: Create GRUB Fallback Generator Script** - `2b4f443` (feat)

## Files Created/Modified

- `archiso/airootfs/etc/pacman.d/hooks/90-vulcan-kernel-verify.hook` - PostTransaction hook that triggers verification after kernel operations
- `archiso/airootfs/usr/local/bin/vulcan-kernel-verify` - Boot chain verification script (242 lines) checking kernel, initramfs, T2 modules (apple_bce, applespi, intel_lpss_pci, spi_pxa2xx_platform), GRUB config, and fallback status
- `archiso/airootfs/usr/local/bin/vulcan-kernel-fallback` - Fallback generator (213 lines) that backs up kernel, maintains two versions, and generates GRUB custom.cfg
- `archiso/airootfs/boot/grub/custom.cfg` - Placeholder for GRUB fallback entries (populated by vulcan-kernel-fallback)

## Decisions Made

**PostTransaction timing:** Hook runs AFTER kernel operations complete, not before. This means verification is informational only - if initramfs is broken, the damage is already done. User must manually fix before rebooting. This differs from the abort hooks (plans 03-04) which run PreTransaction and can block operations.

**Two-level fallback rotation:** Maintains exactly two previous kernel versions. When creating new backup: current backup → backup2, new kernel → backup. When limit reached (2), oldest (backup2) is automatically removed. This provides reasonable rollback history without consuming excessive /boot space.

**GRUB custom.cfg sourcing:** Generated custom.cfg is automatically sourced by GRUB's /etc/grub.d/41_custom script. No grub-mkconfig needed after running vulcan-kernel-fallback - entries appear immediately on next boot.

**Notification persistence:** Desktop notifications use default urgency (normal) for success/warnings, critical urgency for errors. Notifications persist until user dismisses - critical boot chain failures need user attention, not auto-dismiss.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - all scripts created successfully with expected functionality.

## User Setup Required

None - scripts will be included in ISO and available after installation. Users can run manually:
- `sudo vulcan-kernel-verify` - Check current boot chain status
- `sudo vulcan-kernel-fallback` - Create fallback backup and GRUB entries

## Next Phase Readiness

**Ready for abort hooks (plan 03):**
- Verification infrastructure in place to reference from abort conditions
- Fallback system ready to check before allowing kernel updates

**Ready for warning hooks (plan 04):**
- Verification command available to suggest after warnings
- Fallback generator ready to recommend before risky operations

**Blockers/Concerns:**
- None

**Integration points:**
- Plan 03 abort hooks should check if fallback exists using same logic as vulcan-kernel-verify
- Plan 04 warning hooks should suggest running vulcan-kernel-fallback before updates
- Both should reference /var/log/vulcan-*.log for debugging

---
*Phase: 01-t2-kernel-protection*
*Completed: 2026-01-24*
