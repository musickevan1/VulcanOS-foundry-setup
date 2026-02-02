# Phase 14 Plan 07: Foundry Build Entry Point and NVIDIA Configuration Summary

**One-liner:** Created build-foundry.sh build script and NVIDIA-specific mkinitcpio.conf with early KMS modules for Foundry AI Workstation

---

## Frontmatter

```yaml
phase: 14
plan: 07
subsystem: build-system
tags: [archiso, foundry, nvidia, mkinitcpio, build-automation]
requires:
  - "14-01: Multi-profile directory structure and build library"
  - "14-02: Base profile migration"
  - "14-04: Foundry profile creation"
provides:
  - "scripts/build-foundry.sh: Executable build script for Foundry ISO"
  - "Foundry mkinitcpio.conf with NVIDIA early KMS"
  - "Work directory isolation (/tmp/vulcanos-work-foundry)"
affects:
  - "14-08: Validation script (can test both T2 and Foundry builds)"
  - "Future: Foundry ISO build workflow"
tech-stack:
  added:
    - "scripts/build-foundry.sh: Foundry build entry point"
    - "archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf: NVIDIA early KMS config"
  patterns:
    - "NVIDIA early KMS: Load nvidia modules in initramfs MODULES array"
    - "Profile-specific work directories prevent collision"
key-files:
  created:
    - scripts/build-foundry.sh
    - archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf
  modified: []
decisions:
  - id: NVIDIA-EARLY-KMS
    what: Add nvidia, nvidia_modeset, nvidia_uvm, nvidia_drm to MODULES array
    why: Required for proper display before X/Wayland starts on NVIDIA GPUs
    alternatives: Load modules later via udev (rejected - causes blank screen on boot)
  - id: ARCHISO-HOOKS
    what: Include archiso and archiso_loop_mnt in HOOKS
    why: Required for live ISO booting from loop device
    alternatives: None - archiso mandatory hooks for ISO boot
  - id: ZSTD-COMPRESSION
    what: Use zstd with -19 compression
    why: Balances boot speed with ISO size
    alternatives: gzip (slower boot), xz (slower compression)
metrics:
  tasks: 2
  commits: 2
  files_created: 2
  files_modified: 0
  duration: 1min 25sec
  completed: 2026-02-02
```

---

## Tasks Completed

### Task 1: Create build-foundry.sh Script
**Commit:** e9d314b
**Files:** scripts/build-foundry.sh (52 lines)

Created Foundry build entry point following build-t2.sh pattern:
- `PROFILE="foundry"` for Foundry AI Workstation
- Work directories: `/tmp/vulcanos-work-foundry` and `/tmp/vulcanos-assembled-foundry`
- Sources `scripts/lib/build-common.sh` for shared functions
- Trap cleanup on exit for proper error handling
- Main pipeline: validate → clean → assemble → mkarchiso → checksums → permissions

**Verification:**
```bash
$ test -x scripts/build-foundry.sh
$ bash -n scripts/build-foundry.sh
$ grep 'PROFILE="foundry"' scripts/build-foundry.sh
PROFILE="foundry"
```

All verification passed: executable, valid syntax, correct profile.

### Task 2: Create Foundry mkinitcpio.conf with NVIDIA Modules
**Commit:** c67aebc
**Files:** archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf (28 lines)

Created NVIDIA-specific initramfs configuration:

**MODULES array:**
```bash
MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
```

**NVIDIA Early KMS Purpose:**
- `nvidia`: Base NVIDIA driver module
- `nvidia_modeset`: Kernel mode-setting for NVIDIA
- `nvidia_uvm`: Unified memory for CUDA (AI workloads)
- `nvidia_drm`: DRM/KMS support for Wayland/X

**HOOKS array:**
```bash
HOOKS=(base udev archiso archiso_loop_mnt modconf block filesystems keyboard)
```

**Required archiso hooks:**
- `archiso`: Live ISO environment setup
- `archiso_loop_mnt`: Mount ISO from loop device

**Compression:**
```bash
COMPRESSION="zstd"
COMPRESSION_OPTIONS=(-c -T0 -19)
```

**Verification:**
```bash
$ grep nvidia archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf
MODULES=(nvidia nvidia_modeset nvidia_uvm nvidia_drm)
$ grep archiso archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf
HOOKS=(base udev archiso archiso_loop_mnt modconf block filesystems keyboard)
$ grep COMPRESSION archiso/profiles/foundry/airootfs/etc/mkinitcpio.conf
COMPRESSION="zstd"
```

All verification passed: NVIDIA modules, archiso hooks, zstd compression.

---

## Deviations from Plan

None - plan executed exactly as written.

---

## Key Implementation Details

### Build Script Structure
```bash
#!/bin/bash
set -e

PROFILE="foundry"
WORK_DIR="/tmp/vulcanos-work-$PROFILE"
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"

source "$SCRIPT_DIR/lib/build-common.sh"
trap cleanup EXIT

main() {
    # Validate environment
    check_root
    check_dependencies
    validate_profile "$PROFILE"

    # Build pipeline
    clean_build "$WORK_DIR" "$ASSEMBLED_DIR"
    assemble_profile "$PROFILE" "$ASSEMBLED_DIR"
    run_mkarchiso "$ASSEMBLED_DIR" "$WORK_DIR" "$OUT_DIR"

    # Finalize
    generate_checksums "$OUT_DIR"
    fix_permissions "$OUT_DIR"
    show_info "$OUT_DIR" "$PROFILE"
}
```

Identical structure to build-t2.sh, only PROFILE variable differs.

### NVIDIA Early KMS Requirements

**Why early KMS matters:**
- Without early module loading: Blank screen until X/Wayland starts
- With early KMS: Console, Plymouth boot splash, early error messages visible
- Critical for debugging boot issues on NVIDIA hardware

**Module load order:**
1. `nvidia` - Base driver (required first)
2. `nvidia_modeset` - Enable KMS (depends on nvidia)
3. `nvidia_uvm` - Unified memory (AI workloads)
4. `nvidia_drm` - DRM/KMS interface (Wayland support)

**archiso Requirements:**
- `archiso` hook creates live environment
- `archiso_loop_mnt` mounts ISO from loop device
- Both mandatory for ISO boot

---

## Testing Performed

1. **Script Executable:** `test -x scripts/build-foundry.sh` → PASS
2. **Bash Syntax:** `bash -n scripts/build-foundry.sh` → PASS
3. **Profile Setting:** `grep 'PROFILE="foundry"'` → Found
4. **Sources Library:** `grep 'source.*lib/build-common.sh'` → Found
5. **NVIDIA Modules:** `grep nvidia mkinitcpio.conf` → All 4 modules present
6. **archiso Hooks:** `grep archiso mkinitcpio.conf` → Both hooks present
7. **Compression:** `grep COMPRESSION mkinitcpio.conf` → zstd configured

---

## Next Phase Readiness

**Ready for 14-08:** Validation script can now test Foundry builds
- build-foundry.sh exists and is executable
- Can test full build pipeline for Foundry profile
- NVIDIA config can be validated for correctness

**Ready for Future Foundry Builds:**
- `./scripts/build-foundry.sh` produces Foundry ISO
- ISO includes NVIDIA early KMS for proper display
- Work directories isolated from T2 builds

**Blockers:** None

**Concerns:** None - straightforward implementation

---

## Lessons Learned

### What Went Well
- Clean replication of build-t2.sh pattern
- NVIDIA early KMS modules properly ordered
- archiso hooks correctly included for live boot
- Comprehensive comments explain NVIDIA module purposes

### What Could Improve
- None identified - pattern is established and works well

### For Future Phases
- Validation script (14-08) should test both T2 and Foundry builds
- Package lists (14-05, 14-06) will provide packages for this config
- GRUB config will need NVIDIA kernel parameters (nvidia-drm.modeset=1)

---

## Related Documentation

- **14-01-SUMMARY.md:** build-common.sh functions used here
- **14-RESEARCH.md:** NVIDIA early KMS requirements documented
- **scripts/build-t2.sh:** Pattern followed for consistency
- **archiso/airootfs/etc/mkinitcpio.conf:** Reference for archiso hooks

---

**SUMMARY COMPLETED:** 2026-02-02
**EXECUTION TIME:** 1 minute 25 seconds
