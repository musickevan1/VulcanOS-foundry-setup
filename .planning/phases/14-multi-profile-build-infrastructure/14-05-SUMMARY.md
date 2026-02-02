---
phase: 14-multi-profile-build-infrastructure
plan: 05
subsystem: build-infrastructure
tags: [archiso, airootfs, multi-profile, t2-hardware, overlay-pattern]

# Dependency graph
requires:
  - phase: 14-01
    provides: "Directory structure for base and profiles"
  - phase: 14-02
    provides: "Package list organization"
  - phase: 14-03
    provides: "T2 profile boot configuration"
provides:
  - "Base airootfs with all shared content (dotfiles, scripts, themes)"
  - "T2 profile overlay with hardware-specific modprobe configs"
  - "Clean separation: shared vs. T2-specific filesystem overlays"
affects: [14-06, 14-07, 14-08, foundry-profile-development]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Overlay pattern: profile files override base at build time"
    - "Copy-first, remove-last migration strategy"

key-files:
  created:
    - "archiso/base/airootfs/" # All shared content (171 files)
    - "archiso/profiles/t2/airootfs/etc/modprobe.d/" # T2-specific configs
  modified: []

key-decisions:
  - "Use cp -a instead of rsync (rsync not available on system)"
  - "Preserve original archiso/airootfs/ for rollback safety"
  - "T2-specific files ONLY in profile overlay (apple-*.conf modprobe configs)"

patterns-established:
  - "Shared base pattern: All desktop configs, scripts, themes in base/airootfs/"
  - "Profile overlay pattern: Hardware-specific configs in profiles/{profile}/airootfs/"
  - "File precedence: Profile overlay files win over base on conflict"

# Metrics
duration: 2min
completed: 2026-02-02
---

# Phase 14 Plan 05: Migrate airootfs to Base + T2 Overlay Structure

**Base airootfs with 171 shared files (dotfiles, scripts, themes) + T2 overlay with 2 hardware-specific modprobe configs**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-02T22:49:09Z
- **Completed:** 2026-02-02T22:50:41Z
- **Tasks:** 3
- **Files modified:** 173 total (171 base + 2 T2 overlay)

## Accomplishments

- Migrated entire airootfs structure to base/airootfs/ (171 shared files)
- Extracted T2-specific modprobe configs to profiles/t2/airootfs/ overlay (2 files)
- Verified migration integrity: base + T2 overlay = original airootfs functionality
- Preserved original archiso/airootfs/ for rollback safety

## Task Commits

Each task was committed atomically:

1. **Task 1: Copy shared content to base/airootfs/** - `ad8c0c4` (feat)
   - Copied all airootfs content to base (173 files)
   - Includes dotfiles, scripts, themes, system configs

2. **Task 2: Create T2-specific airootfs overlay** - `d73c1e8` (feat)
   - Moved apple-bce.conf and apple-gmux.conf to T2 profile
   - Created profiles/t2/airootfs/etc/modprobe.d/ structure

3. **Task 3: Verify migration integrity** - `13f7c4c` (docs)
   - Verified file counts: 171 base + 2 T2 = 173 original
   - Verified critical files in correct locations

## Files Created/Modified

**Created:**
- `archiso/base/airootfs/` (171 files)
  - `etc/skel/.config/hypr/` - Hyprland desktop configs
  - `etc/skel/.config/kitty/` - Terminal config
  - `etc/skel/.config/themes/` - Theme system
  - `etc/skel/.config/nvim/` - Neovim config
  - `etc/skel/.config/opencode/` - OpenCode AI config
  - `etc/skel/.local/bin/` - User scripts
  - `usr/local/bin/` - System scripts (vulcan-power, vulcan-screenshot, etc.)
  - `usr/share/sddm/themes/vulcanos/` - SDDM login theme
  - `etc/sddm.conf.d/`, `etc/locale.conf`, `etc/mkinitcpio.conf`, etc.

- `archiso/profiles/t2/airootfs/etc/modprobe.d/` (2 files)
  - `apple-bce.conf` - T2 keyboard/trackpad driver config
  - `apple-gmux.conf` - T2 GPU switching config

**Modified:**
- None (migration was copy operation)

## Decisions Made

1. **Use `cp -a` instead of `rsync`**
   - Rationale: rsync not available on system, cp -a provides same functionality
   - Impact: None - cp -a handles permissions, symlinks, and directory copying correctly

2. **Preserve original archiso/airootfs/**
   - Rationale: Safety for rollback if migration issues found during build testing (Plan 08)
   - Impact: Allows safe removal after successful T2 build verification

3. **Minimal T2 overlay (only modprobe configs)**
   - Rationale: Research showed only apple-*.conf files are hardware-specific
   - Impact: Clean separation - all desktop experience shared, only kernel module configs differ

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

**Tool availability:**
- rsync not available on system
- Resolved by using `cp -a` which provides equivalent functionality for this use case
- No impact on migration integrity

## Next Phase Readiness

**Ready for:**
- Plan 14-06: T2 build script can now assemble base + T2 overlay
- Plan 14-07: Foundry build script will use same base with Foundry overlay
- Plan 14-08: Build verification will test both profiles

**Migration status:**
- ✓ Base has all shared content (desktop configs, scripts, themes)
- ✓ T2 overlay has hardware-specific modprobe configs
- ✓ Combined content matches original airootfs (173 files)
- ✓ Original airootfs/ preserved for rollback
- Pending: Build scripts to assemble profiles (Plans 14-06, 14-07)
- Pending: Verification builds before removing original (Plan 14-08)

**Blockers:**
- None - migration structure ready for build scripts

---
*Phase: 14-multi-profile-build-infrastructure*
*Completed: 2026-02-02*
