# Phase 14 Plan 01: Multi-Profile Directory Structure and Build Library Summary

**One-liner:** Created multi-profile directory structure (base, t2, foundry) and shared build library with validation, assembly, and build orchestration functions

---

## Frontmatter

```yaml
phase: 14
plan: 01
subsystem: build-system
tags: [archiso, multi-profile, bash, build-automation]
requires:
  - "13-03: v2.1 Foundation complete (clean build system baseline)"
provides:
  - "Multi-profile directory structure (archiso/base/, archiso/profiles/t2/, archiso/profiles/foundry/)"
  - "Shared build library (scripts/lib/build-common.sh)"
  - "Reusable functions for profile validation, assembly, and build orchestration"
affects:
  - "14-02: Profile migration (will use this structure)"
  - "14-03: Profile-specific build scripts (will source build-common.sh)"
tech-stack:
  added:
    - "scripts/lib/build-common.sh: Shared build functions for multi-profile builds"
  patterns:
    - "Profile assembly pattern: rsync base + profile overlay"
    - "Package list merging: cat + grep + sort -u for deduplication"
    - "Fail-fast validation: Check required files before expensive builds"
key-files:
  created:
    - archiso/base/.gitkeep
    - archiso/profiles/t2/.gitkeep
    - archiso/profiles/foundry/.gitkeep
    - scripts/lib/build-common.sh
  modified: []
decisions:
  - id: DIR-STRUCTURE
    what: Use archiso/base/ for shared content, archiso/profiles/{t2,foundry}/ for profile-specific
    why: Aligns with CONTEXT.md decision and archiso best practices
    alternatives: Flat structure or profiles outside archiso/ (rejected for clarity)
  - id: BASE-DIR-VARIABLE
    what: BASE_DIR points to archiso/base/, overridable via environment
    why: Allows calling scripts to customize paths while providing sensible defaults
    alternatives: Hardcoded paths (rejected for flexibility)
  - id: RSYNC-OVERLAY
    what: Use rsync -a for base copy, then rsync -a for profile overlay (profile wins)
    why: Handles permissions, symlinks, and precedence correctly
    alternatives: cp -r (rejected - doesn't handle overlays well)
metrics:
  tasks: 2
  commits: 2
  files_created: 4
  files_modified: 0
  duration: 2min 5sec
  completed: 2026-02-02
```

---

## Tasks Completed

### Task 1: Create Multi-Profile Directory Structure
**Commit:** 4f5359c
**Files:** archiso/base/.gitkeep, archiso/profiles/t2/.gitkeep, archiso/profiles/foundry/.gitkeep

Created the directory structure for multi-profile builds:
- `archiso/base/` - For shared content across all profiles
- `archiso/profiles/t2/` - T2 MacBook Pro profile
- `archiso/profiles/foundry/` - Generic workstation profile

Added `.gitkeep` files to preserve empty directories in git.

**Verification:**
```bash
$ ls -la archiso/base/ archiso/profiles/t2/ archiso/profiles/foundry/
# All showed .gitkeep files present
```

### Task 2: Create Shared Build Library
**Commit:** ee82d8c
**Files:** scripts/lib/build-common.sh (316 lines)

Created shared build library with comprehensive functions:

**Logging Functions:**
- `info()`, `success()`, `warn()`, `error()` - Color-coded output with consistent formatting

**Dependency Checking:**
- `check_root()` - Verify running as root
- `check_dependencies()` - Verify mkarchiso, rsync, git, mksquashfs, xorriso installed

**Validation:**
- `validate_profile()` - Check required files exist before build:
  - Required files: packages.base, packages.profile, pacman.conf, profiledef.sh, grub/grub.cfg, syslinux/syslinux.cfg
  - Required directories: base/airootfs (warning if missing), profile airootfs (warning if missing)

**Assembly:**
- `assemble_profile()` - Merge base + profile into work directory:
  - Step 1: Copy base airootfs with rsync -a
  - Step 2: Overlay profile airootfs (profile wins on conflict)
  - Step 3: Merge package lists (cat base + profile, remove comments, sort -u)
  - Step 4: Copy profile-specific configs (pacman.conf, profiledef.sh)
  - Step 5: Copy boot configs (grub/, syslinux/)
  - Step 6: Copy efiboot (profile-specific if exists, else shared)

**Build Helpers:**
- `clean_build()` - Remove work/assembled directories
- `run_mkarchiso()` - Invoke mkarchiso with proper args and SOURCE_DATE_EPOCH
- `generate_checksums()` - Create SHA256SUMS, MD5SUMS
- `fix_permissions()` - chown to SUDO_USER
- `cleanup()` - Trap handler for failed builds
- `show_info()` - Display build results

**Verification:**
```bash
$ bash -c 'source scripts/lib/build-common.sh && type assemble_profile && type validate_profile'
assemble_profile is a function
validate_profile is a function
```

All 14 expected functions defined and sourceable without errors.

---

## Deviations from Plan

None - plan executed exactly as written.

---

## Key Implementation Details

### Directory Variables
```bash
ARCHISO_DIR="${ARCHISO_DIR:-$PROJECT_DIR/archiso}"
BASE_DIR="${BASE_DIR:-$ARCHISO_DIR/base}"
PROFILE_DIR="${PROFILE_DIR:-$ARCHISO_DIR/profiles}"
```

All variables are overridable via environment, with sensible defaults.

### rsync Overlay Pattern
```bash
# Base first (creates clean slate)
rsync -a "$base_dir/airootfs/" "$assembled_dir/airootfs/"

# Profile overlay (NO --delete, profile wins)
rsync -a "$profile_dir/airootfs/" "$assembled_dir/airootfs/"
```

This ensures profile-specific files override base files when both exist.

### Package List Merging
```bash
cat "$base_dir/packages.base" "$profile_dir/packages.profile" | \
    grep -v '^[[:space:]]*#' | \
    grep -v '^[[:space:]]*$' | \
    sort -u > "$assembled_dir/packages.x86_64"
```

Removes comments, blank lines, and deduplicates packages.

### Validation Safety
```bash
# Hard errors for critical files
for file in "${required_files[@]}"; do
    if [[ ! -f "$ARCHISO_DIR/$file" ]]; then
        error "Missing required file: $file"
        ((errors++))
    fi
done

# Warnings for optional directories
if [[ ! -d "$ARCHISO_DIR/base/airootfs" ]]; then
    warn "Missing base/airootfs directory (will build without base overlay)"
fi
```

Distinguishes between critical failures and acceptable warnings.

---

## Testing Performed

1. **Directory Structure:** Verified .gitkeep files in all three directories
2. **Library Sourcing:** Sourced build-common.sh without errors
3. **Function Definitions:** Verified all 14 expected functions are defined
4. **Variable Patterns:** Confirmed BASE_DIR points to archiso/base
5. **Line Count:** 316 lines (well above 100 line minimum)

---

## Next Phase Readiness

**Ready for 14-02:** Profile migration can now proceed
- Directory structure exists
- Shared functions available for sourcing
- Validation logic ready for use

**Ready for 14-03:** Build scripts can be created
- `assemble_profile()` ready to use
- `run_mkarchiso()` ready to invoke
- Error handling patterns established

**Blockers:** None

**Concerns:** None

---

## Lessons Learned

### What Went Well
- Clean separation of concerns (logging, validation, assembly, build)
- Comprehensive error handling with fail-fast validation
- Flexible variable defaults that can be overridden
- Well-documented function purposes

### What Could Improve
- None identified - straightforward implementation

### For Future Phases
- Profile-specific build scripts (14-03) should follow this pattern
- Migration script (14-02) should use these functions for validation
- Test coverage for edge cases (missing files, empty directories)

---

## Related Documentation

- **CONTEXT.md:** Directory structure decision (profiles inside archiso/)
- **RESEARCH.md:** Assembly patterns, rsync flags, validation best practices
- **scripts/build.sh:** Original build script used as reference for color codes and logging

---

**SUMMARY COMPLETED:** 2026-02-02
**EXECUTION TIME:** 2 minutes 5 seconds
