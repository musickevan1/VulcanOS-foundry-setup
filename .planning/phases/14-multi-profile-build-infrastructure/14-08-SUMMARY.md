---
phase: 14
plan: 08
subsystem: build-system
tags: [archiso, validation, build-automation, deprecation, multi-profile]

# Dependency graph
requires:
  - "14-01: Multi-profile directory structure and build library"
  - "14-05: Airootfs migration to base + T2 overlay"
  - "14-06: T2 build entry point script"
  - "14-07: Foundry build entry point and NVIDIA config"
provides:
  - "Deprecation stub for old build.sh script"
  - "User-facing build command change documented"
  - "Complete multi-profile build infrastructure verified"
affects:
  - "Future: Users run ./scripts/build-t2.sh or ./scripts/build-foundry.sh"
  - "Documentation: Build instructions reference new scripts"

# Tech tracking
tech-stack:
  added:
    - "scripts/build.sh: Deprecation stub with guidance (22 lines)"
  patterns:
    - "Deprecation stubs exit 1 with clear guidance message"
    - "Profile-specific build scripts replace unified scripts"

key-files:
  created: []
  modified:
    - scripts/build.sh

decisions:
  - id: DEPRECATION-STUB
    what: Replace build.sh with deprecation stub showing error
    why: Clear user-facing signal that build system changed
    alternatives: Delete script (rejected - no guidance), redirect to new script (rejected - hides change)

# Metrics
duration: 7h 35min
completed: 2026-02-03
---

# Phase 14 Plan 08: Deprecate Old Build Script and Verify Infrastructure Summary

**Replaced unified build.sh with deprecation stub directing users to profile-specific scripts (build-t2.sh, build-foundry.sh)**

## Performance

- **Duration:** 7h 35min (includes checkpoint wait time)
- **Started:** 2026-02-02T22:54:01Z
- **Completed:** 2026-02-03T06:29:03Z
- **Tasks:** 2 (1 implementation + 1 verification checkpoint)
- **Files modified:** 1

## Accomplishments

- Old build.sh now shows deprecation error with clear guidance
- Exits with status 1 to fail in scripts/CI that haven't updated
- Directs users to build-t2.sh for T2 MacBook Pro builds
- Directs users to build-foundry.sh for Foundry AI Workstation builds
- User verified complete multi-profile build infrastructure

## Task Commits

1. **Task 1: Replace build.sh with deprecation stub** - `3101cf0` (feat)
   - Replaced 244-line build script with 22-line deprecation stub
   - Clear error message with guidance for both profiles
   - Exits with status 1 (non-zero)

2. **Task 2: Human verification checkpoint** - Approved
   - User verified directory structure exists
   - User verified package files in correct locations
   - User verified T2 pacman.conf has arch-mact2 repo
   - User verified Foundry has no T2-specific repos
   - User confirmed build scripts ready

## Files Created/Modified

- `scripts/build.sh` - Deprecation stub (was 248 lines, now 22 lines)

## Decisions Made

**DEPRECATION-STUB:** Use error message instead of silent redirect
- **Rationale:** Users need to know build system changed, not silently adapt
- **Alternative rejected:** Redirect to build-t2.sh automatically (hides breaking change)
- **Alternative rejected:** Delete script entirely (no guidance for confused users)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Verification Performed

User verified the following as part of checkpoint:

1. **Deprecation message works:**
   - Running `./scripts/build.sh` shows clear error
   - Mentions both build-t2.sh and build-foundry.sh
   - Exits with status 1

2. **Directory structure complete:**
   - `archiso/base/` exists with shared content
   - `archiso/profiles/t2/` exists with T2-specific files
   - `archiso/profiles/foundry/` exists with Foundry-specific files

3. **Package organization verified:**
   - `archiso/base/packages.base` exists (shared packages)
   - `archiso/profiles/t2/packages.profile` exists (T2 packages)
   - `archiso/profiles/foundry/packages.profile` exists (Foundry packages)

4. **Profile configs verified:**
   - T2 pacman.conf has active arch-mact2 repo
   - Foundry pacman.conf has NO arch-mact2 repo
   - Both profiles have profiledef.sh, boot configs

5. **Build scripts verified:**
   - `scripts/build-t2.sh` exists and is executable
   - `scripts/build-foundry.sh` exists and is executable
   - Both source `scripts/lib/build-common.sh`

## Multi-Profile Build Infrastructure Summary

**Complete infrastructure delivered across Phase 14 Plans 01-08:**

### Directory Structure (14-01, 14-02, 14-05)
```
archiso/
├── base/                       # Shared content
│   ├── airootfs/              # 171 shared files
│   └── packages.base          # Desktop, dev tools, fonts
├── profiles/
│   ├── t2/                    # T2 MacBook Pro
│   │   ├── airootfs/etc/modprobe.d/  # apple-bce, apple-gmux
│   │   ├── packages.profile   # linux-t2, apple-bcm-firmware
│   │   ├── pacman.conf        # arch-mact2 FIRST
│   │   ├── profiledef.sh      # T2-branded ISO
│   │   ├── grub/              # T2 boot menu
│   │   └── syslinux/          # T2 boot menu
│   └── foundry/               # Foundry AI Workstation
│       ├── airootfs/etc/mkinitcpio.conf  # NVIDIA early KMS
│       ├── packages.profile   # NVIDIA, CUDA, ML stack
│       ├── pacman.conf        # Standard repos only
│       ├── profiledef.sh      # Foundry-branded ISO
│       ├── grub/              # NVIDIA kernel params
│       └── syslinux/          # Nouveau fallback
```

### Package Organization (14-03, 14-04)
- **Base packages:** 190+ packages (desktop, Hyprland, dev tools, CLI utils, fonts)
- **T2 packages:** 8 packages (linux-t2, T2 firmware, T2 audio, T2 fan, Touch Bar)
- **Foundry packages:** 18+ packages (NVIDIA drivers, CUDA, cuDNN, ML frameworks)

### Build Scripts (14-01, 14-06, 14-07, 14-08)
- `scripts/lib/build-common.sh` - Shared validation, assembly, mkarchiso logic
- `scripts/build-t2.sh` - T2 build entry point (work dir: /tmp/vulcanos-work-t2)
- `scripts/build-foundry.sh` - Foundry build entry point (work dir: /tmp/vulcanos-work-foundry)
- `scripts/build.sh` - Deprecation stub (exit 1 with guidance)

### Build Process
1. Validate profile files exist
2. Clean work directories
3. Assemble profile (rsync base → assembled, rsync profile → assembled)
4. Merge packages (cat base + profile, deduplicate)
5. Run mkarchiso on assembled profile
6. Generate checksums
7. Fix permissions

### Expected Output
- **T2 build:** `out/vulcanos-t2-YYYY.MM.DD-x86_64.iso`
- **Foundry build:** `out/vulcanos-foundry-YYYY.MM.DD-x86_64.iso`

## Next Phase Readiness

**Ready for Phase 15 (Installer):**
- Multi-profile build system complete and verified
- Both ISOs can be built independently
- Profile-specific packages and configs cleanly separated
- Build scripts follow consistent pattern

**Ready for testing:**
- T2 ISO build can be tested: `sudo ./scripts/build-t2.sh`
- Foundry ISO build can be tested: `sudo ./scripts/build-foundry.sh`
- Both builds use separate work directories (no contamination)

**Blockers:** None

**Concerns:**
- Foundry hardware not yet available for full end-to-end testing
- Foundry ISO can be built and boot-tested in VM, but GPU features require hardware
- T2 hardware available for full validation

## Key Achievements of Phase 14

1. **Directory restructuring:** Extracted T2-specific content, created shared base
2. **Profile separation:** Clean split between T2 and Foundry configs
3. **Package organization:** Shared base packages, profile-specific additions
4. **Airootfs migration:** 171 shared files in base, T2-specific in profile overlay
5. **Build automation:** Profile-specific entry points with shared library
6. **Repository configuration:** arch-mact2 ONLY in T2, standard repos in Foundry
7. **NVIDIA support:** Early KMS modules for Foundry display initialization
8. **User-facing change:** Clear deprecation message for old build command

## Related Documentation

- **14-CONTEXT.md:** Implementation decisions for multi-profile architecture
- **14-RESEARCH.md:** Archiso profile inheritance patterns, NVIDIA early KMS
- **14-01-SUMMARY.md:** Shared build library functions
- **14-05-SUMMARY.md:** Airootfs migration (171 files to base)
- **14-06-SUMMARY.md:** T2 build script pattern
- **14-07-SUMMARY.md:** Foundry build script and NVIDIA config

---

**SUMMARY COMPLETED:** 2026-02-03
**PHASE 14 STATUS:** Complete (8 of 8 plans)
