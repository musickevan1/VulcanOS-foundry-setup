# Phase 14 Plan 06: T2 Build Entry Point Script Summary

**One-liner:** Created build-t2.sh entry point script that sources shared library, validates T2 profile, assembles profile, and builds vulcanos-t2-YYYY.MM.DD-x86_64.iso

---

## Frontmatter

```yaml
phase: 14
plan: 06
subsystem: build-system
tags: [archiso, t2-profile, bash, build-automation, entry-point]
requires:
  - "14-01: Multi-profile directory structure and shared build library"
  - "14-02: Profile migration (provides base and profile directories)"
  - "14-03: T2 profile configuration files"
provides:
  - "scripts/build-t2.sh: Entry point for T2 MacBook Pro ISO builds"
  - "Executable build script for T2 profile"
affects:
  - "14-07: Foundry build script (will follow same pattern)"
  - "14-08: Build script integration testing"
tech-stack:
  added:
    - "scripts/build-t2.sh: T2 build entry point (52 lines)"
  patterns:
    - "Profile-specific entry points source shared library"
    - "Trap cleanup EXIT for error handling"
    - "Separate work directories per profile prevent contamination"
key-files:
  created:
    - scripts/build-t2.sh
  modified: []
decisions:
  - id: T2-WORK-DIR
    what: Use /tmp/vulcanos-work-t2 for T2 builds
    why: Separate work directories prevent profile contamination
    alternatives: Shared work dir (rejected - causes build contamination)
  - id: CLEANUP-TRAP
    what: Use trap cleanup EXIT for automatic cleanup
    why: Ensures cleanup happens even on error
    alternatives: Manual cleanup calls (rejected - easy to miss on errors)
  - id: EXPORT-VARS
    what: Export WORK_DIR and ASSEMBLED_DIR for cleanup trap
    why: Cleanup function needs access to these paths
    alternatives: Pass as arguments (rejected - trap can't receive args easily)
metrics:
  tasks: 2
  commits: 1
  files_created: 1
  files_modified: 0
  duration: 1min
  completed: 2026-02-02
```

---

## Tasks Completed

### Task 1: Create build-t2.sh script
**Commit:** 447a2b8
**Files:** scripts/build-t2.sh (52 lines)

Created the T2 build entry point script following the pattern from RESEARCH.md:

**Key features:**
1. Sets `PROFILE="t2"` for T2-specific builds
2. Sources `lib/build-common.sh` for shared functions
3. Uses separate work directory `/tmp/vulcanos-work-t2`
4. Uses separate assembled directory `/tmp/vulcanos-assembled-t2`
5. Exports WORK_DIR and ASSEMBLED_DIR for cleanup trap
6. Trap cleanup EXIT ensures cleanup on success or failure
7. Full pipeline: validate → clean → assemble → build → checksums
8. Sources shared library for validate_profile, assemble_profile, run_mkarchiso

**Script structure:**
```bash
#!/bin/bash
set -e

# Variables
PROFILE="t2"
WORK_DIR="/tmp/vulcanos-work-$PROFILE"
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"

# Source shared library
source "$SCRIPT_DIR/lib/build-common.sh"

# Export for cleanup trap
export WORK_DIR ASSEMBLED_DIR

# Trap cleanup
trap cleanup EXIT

# Main pipeline
main() {
    check_root
    check_dependencies
    validate_profile "$PROFILE"
    clean_build "$WORK_DIR" "$ASSEMBLED_DIR"
    assemble_profile "$PROFILE" "$ASSEMBLED_DIR"
    run_mkarchiso "$ASSEMBLED_DIR" "$WORK_DIR" "$OUT_DIR"
    generate_checksums "$OUT_DIR"
    fix_permissions "$OUT_DIR"
    show_info "$OUT_DIR" "$PROFILE"
}
```

**Verification:**
```bash
$ bash -n scripts/build-t2.sh
# Syntax check passed

$ grep 'PROFILE="t2"' scripts/build-t2.sh
PROFILE="t2"

$ grep "source.*lib/build-common.sh" scripts/build-t2.sh
source "$SCRIPT_DIR/lib/build-common.sh"

$ test -x scripts/build-t2.sh && echo "Executable"
Executable
```

### Task 2: Verify script can be parsed and sourced
**Status:** Complete (no separate commit - validation only)

Verified the script works correctly:

**Syntax validation:**
```bash
$ bash -n scripts/build-t2.sh
# Passed
```

**Library sourcing:**
```bash
$ bash -c 'source scripts/lib/build-common.sh && echo "Library loaded"'
Library loaded
```

**Required variables present:**
- `PROFILE="t2"` ✓
- `WORK_DIR="/tmp/vulcanos-work-$PROFILE"` ✓
- `ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"` ✓

**Required function calls present:**
- `validate_profile "$PROFILE"` ✓
- `assemble_profile "$PROFILE" "$ASSEMBLED_DIR"` ✓
- `run_mkarchiso "$ASSEMBLED_DIR" "$WORK_DIR" "$OUT_DIR"` ✓
- `check_root` ✓
- `check_dependencies` ✓
- `clean_build` ✓
- `generate_checksums` ✓
- `fix_permissions` ✓
- `show_info` ✓

**Note:** Did NOT run actual build - requires all profile files to be in place and root privileges.

---

## Deviations from Plan

None - plan executed exactly as written.

---

## Key Implementation Details

### Profile-Specific Work Directories
```bash
WORK_DIR="/tmp/vulcanos-work-$PROFILE"       # /tmp/vulcanos-work-t2
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"  # /tmp/vulcanos-assembled-t2
```

This ensures T2 and Foundry builds don't contaminate each other if both are built on the same system.

### Cleanup Trap Pattern
```bash
export WORK_DIR ASSEMBLED_DIR
trap cleanup EXIT
```

The cleanup function (from build-common.sh) accesses `$WORK_DIR` and `$ASSEMBLED_DIR` environment variables. Exporting them ensures cleanup happens even on error.

### Build Pipeline Order
1. **Validate** - Check all required files exist before starting expensive operations
2. **Clean** - Remove previous work directories
3. **Assemble** - Merge base + profile into temporary directory
4. **Build** - Run mkarchiso on assembled profile
5. **Checksums** - Generate SHA256SUMS and MD5SUMS
6. **Permissions** - Fix ownership for SUDO_USER
7. **Info** - Display build results

This order follows fail-fast principle (validate early, fail early).

### Expected Build Output
```
out/vulcanos-t2-2026.02.02-x86_64.iso
out/SHA256SUMS
out/MD5SUMS
```

ISO naming follows archiso pattern with profile name embedded.

---

## Testing Performed

1. **Syntax validation:** `bash -n scripts/build-t2.sh` passed
2. **Executable check:** `test -x scripts/build-t2.sh` passed
3. **Library sourcing:** Verified build-common.sh can be sourced
4. **Variable presence:** Verified PROFILE, WORK_DIR, ASSEMBLED_DIR set correctly
5. **Function calls:** Verified all required function calls present

**Not tested (by design):**
- Actual ISO build (requires complete profile files and root privileges)
- Profile assembly (requires base and profile directories with content)

These will be tested in Phase 14 Plan 08 (integration testing).

---

## Next Phase Readiness

**Ready for 14-07:** Foundry build script can follow same pattern
- Entry point structure established
- Variable naming convention clear
- Trap pattern documented

**Ready for 14-08:** Integration testing can validate entire pipeline
- build-t2.sh ready to test with real profiles
- Expected behavior documented

**Blockers:** None

**Concerns:** None

---

## Lessons Learned

### What Went Well
- Clean entry point design - minimal logic, delegates to shared library
- Profile-specific work directories prevent contamination
- Trap cleanup ensures no leftover temporary directories
- Script is self-documenting with clear variable names

### What Could Improve
- None identified - straightforward implementation following RESEARCH.md patterns

### For Future Phases
- Foundry build script (14-07) should be nearly identical (change PROFILE="t2" to PROFILE="foundry")
- Integration testing (14-08) should test both profiles in sequence to verify no contamination
- Consider adding --dry-run flag for validation without actual build

---

## Related Documentation

- **14-RESEARCH.md:** Assemble-then-build pattern, entry point structure
- **14-01-SUMMARY.md:** Shared library functions (validate_profile, assemble_profile, run_mkarchiso)
- **14-CONTEXT.md:** Decision to use separate build scripts (not flags)
- **scripts/build.sh:** Original build script used as reference for structure

---

**SUMMARY COMPLETED:** 2026-02-02
**EXECUTION TIME:** 1 minute
