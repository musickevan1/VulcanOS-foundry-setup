# Phase 14: Multi-Profile Build Infrastructure - Research

**Researched:** 2026-02-02
**Domain:** archiso build system, bash scripting, directory overlay patterns
**Confidence:** HIGH

## Summary

This phase restructures VulcanOS's archiso-based build system to support multiple profiles (T2 and Foundry) from a shared base. Research confirms that archiso does NOT have built-in profile inheritance or composition - each profile is a standalone directory with its own `profiledef.sh`, `packages.x86_64`, `pacman.conf`, and `airootfs/` overlay.

The standard approach for multi-profile builds is to create separate profile directories with independent configurations, then use shell scripts to:
1. Merge shared base content with profile-specific content before mkarchiso runs
2. Concatenate package lists (base + profile)
3. Apply profile-specific overlays using rsync with precedence rules

**Primary recommendation:** Create an `assemble-profile.sh` script that generates a complete, ready-to-build profile directory by merging base + profile content into a temporary working directory, then invoke mkarchiso on that assembled profile.

## Standard Stack

The established tools for this domain:

### Core
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| archiso | latest | ISO generation framework | Official Arch tool, mkarchiso command |
| bash | 5.x | Build script orchestration | Standard, portable, no external deps |
| rsync | 3.x | Directory merging with precedence | Handles overlay semantics correctly |
| cat | coreutils | Package list concatenation | Simple, reliable file merging |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| mkarchiso | Core ISO builder | Final ISO generation step |
| pacstrap | Package installation | Used internally by mkarchiso |
| mksquashfs | Filesystem image | Used internally by mkarchiso |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| rsync for overlay | cp -a | rsync handles existing files better with `--ignore-existing` or `--update` |
| bash arrays | external config | Bash arrays are simpler, no parsing needed |
| separate scripts | unified with flags | Separate scripts are clearer, per CONTEXT.md decision |

## Architecture Patterns

### Recommended Project Structure

```
archiso/
├── base/                           # Shared content
│   ├── airootfs/                   # Common filesystem overlay
│   │   ├── etc/
│   │   │   ├── skel/               # User dotfiles (shared)
│   │   │   ├── pacman.d/mirrorlist
│   │   │   └── ...
│   │   └── usr/local/bin/          # Common scripts
│   └── packages.base               # Packages for ALL profiles
├── profiles/
│   ├── t2/                         # T2 MacBook profile
│   │   ├── airootfs/               # T2-specific overlay (merged after base)
│   │   │   └── etc/
│   │   │       └── modprobe.d/     # T2 kernel modules
│   │   ├── packages.profile        # T2-only packages
│   │   ├── pacman.conf             # T2 repos (includes arch-mact2)
│   │   ├── profiledef.sh           # T2 ISO naming/metadata
│   │   ├── grub/grub.cfg           # T2-branded boot menu
│   │   └── syslinux/syslinux.cfg   # T2-branded boot menu
│   └── foundry/                    # Foundry workstation profile
│       ├── airootfs/               # Foundry-specific overlay
│       ├── packages.profile        # GPU/ML packages
│       ├── pacman.conf             # Standard repos (no arch-mact2)
│       ├── profiledef.sh           # Foundry ISO naming/metadata
│       ├── grub/grub.cfg           # Foundry-branded boot menu
│       └── syslinux/syslinux.cfg   # Foundry-branded boot menu
├── efiboot/                        # Shared EFI config (if same for both)
└── work/                           # Build-time assembled profile (gitignored)
```

### Pattern 1: Assemble-Then-Build

**What:** Create assembled profile directory before mkarchiso runs
**When to use:** Always - this is the core pattern for multi-profile builds
**Example:**
```bash
# Source: Derived from archiso best practices
assemble_profile() {
    local profile="$1"
    local work_dir="$2"

    # Clean workspace
    rm -rf "$work_dir"
    mkdir -p "$work_dir"

    # Copy base airootfs first
    rsync -a "$BASE_DIR/airootfs/" "$work_dir/airootfs/"

    # Overlay profile-specific airootfs (profile wins on conflict)
    rsync -a "$PROFILE_DIR/$profile/airootfs/" "$work_dir/airootfs/"

    # Merge package lists
    cat "$BASE_DIR/packages.base" \
        "$PROFILE_DIR/$profile/packages.profile" \
        > "$work_dir/packages.x86_64"

    # Copy profile-specific configs (not merged)
    cp "$PROFILE_DIR/$profile/pacman.conf" "$work_dir/"
    cp "$PROFILE_DIR/$profile/profiledef.sh" "$work_dir/"

    # Copy boot configs
    cp -r "$PROFILE_DIR/$profile/grub" "$work_dir/"
    cp -r "$PROFILE_DIR/$profile/syslinux" "$work_dir/"

    # Copy shared efiboot (or profile-specific if exists)
    if [[ -d "$PROFILE_DIR/$profile/efiboot" ]]; then
        cp -r "$PROFILE_DIR/$profile/efiboot" "$work_dir/"
    else
        cp -r "$ARCHISO_DIR/efiboot" "$work_dir/"
    fi
}
```

### Pattern 2: Fail-Fast Validation

**What:** Check all required files exist before expensive operations
**When to use:** At start of every build script
**Example:**
```bash
# Source: Shell scripting best practices
validate_profile() {
    local profile="$1"
    local errors=0

    local required_files=(
        "profiles/$profile/packages.profile"
        "profiles/$profile/pacman.conf"
        "profiles/$profile/profiledef.sh"
        "profiles/$profile/grub/grub.cfg"
        "profiles/$profile/syslinux/syslinux.cfg"
        "base/packages.base"
    )

    for file in "${required_files[@]}"; do
        if [[ ! -f "$ARCHISO_DIR/$file" ]]; then
            error "Missing required file: $file"
            ((errors++))
        fi
    done

    local required_dirs=(
        "profiles/$profile/airootfs"
        "base/airootfs"
    )

    for dir in "${required_dirs[@]}"; do
        if [[ ! -d "$ARCHISO_DIR/$dir" ]]; then
            error "Missing required directory: $dir"
            ((errors++))
        fi
    done

    if [[ $errors -gt 0 ]]; then
        error "Validation failed with $errors error(s)"
        exit 1
    fi
}
```

### Pattern 3: Package List Organization

**What:** Separate base packages from profile-specific packages with comments
**When to use:** For maintainability
**Example:**
```
# packages.base - Shared across all profiles
# ===========================================

# BASE SYSTEM (generic kernel - profiles override)
base
linux
linux-headers
linux-firmware

# SYSTEM ESSENTIALS
networkmanager
sudo
which
mkinitcpio
mkinitcpio-archiso

# BOOTLOADER
grub
efibootmgr
syslinux

# DESKTOP ENVIRONMENT
hyprland
xdg-desktop-portal-hyprland
...
```

```
# packages.profile (t2) - T2 MacBook specific
# ===========================================

# T2 KERNEL (overrides generic linux)
linux-t2
linux-t2-headers

# T2 HARDWARE
apple-bcm-firmware
apple-t2-audio-config
t2fanrd
tiny-dfr
```

### Anti-Patterns to Avoid

- **Modifying files in-place:** Never edit base/ or profiles/ during build - always work in a temp directory
- **Hardcoded paths:** Use variables like `$ARCHISO_DIR`, `$PROFILE_DIR` consistently
- **Missing validation:** Never run mkarchiso without checking prerequisites first
- **Sharing work directories:** Each profile build needs its own work directory to avoid contamination

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Directory overlay/merge | Custom recursive copy | `rsync -a` | Handles permissions, symlinks, existing files correctly |
| Package list deduplication | Custom awk/sort | Let pacman handle it | Pacman ignores duplicates, no build-time processing needed |
| ISO generation | Custom squashfs+ISO | `mkarchiso` | Handles boot loaders, EFI, initramfs correctly |
| File permissions in airootfs | chmod scripts | `file_permissions` in profiledef.sh | Official mechanism, applied consistently |
| Template variable replacement | Custom sed | archiso's built-in `%ARCHISO_LABEL%` etc. | Already implemented in mkarchiso |

**Key insight:** archiso is battle-tested. The build scripts should orchestrate and assemble - not replicate mkarchiso functionality.

## Common Pitfalls

### Pitfall 1: Package Conflicts Between Base and Profile Kernels
**What goes wrong:** Both `linux` (in base) and `linux-t2` (in T2 profile) get installed, causing conflicts
**Why it happens:** Package lists are concatenated, both kernels requested
**How to avoid:**
- Put generic `linux` kernel ONLY in Foundry profile, not in base
- Or use pacman `Conflicts` mechanism (but this requires AUR package modification)
- Best: Base has no kernel, each profile specifies its own kernel
**Warning signs:** Build fails with "conflicting files" error for vmlinuz

### Pitfall 2: airootfs File Permission Issues
**What goes wrong:** Files have wrong permissions (e.g., 600 instead of 755 for scripts)
**Why it happens:** Git doesn't preserve ownership; build runs as root
**How to avoid:**
- Use `file_permissions` array in profiledef.sh for all non-default permissions
- Use `--no-preserve=ownership` when copying (mkarchiso does this)
- Set executable permissions explicitly: `["/usr/local/bin/"]="0:0:755"`
**Warning signs:** "Permission denied" when running scripts from live ISO

### Pitfall 3: Work Directory Contamination
**What goes wrong:** Building profile B includes leftover files from profile A
**Why it happens:** Reusing work directory between profile builds
**How to avoid:**
- Use separate work directories per profile: `/tmp/vulcanos-work-t2`, `/tmp/vulcanos-work-foundry`
- Always clean work directory before assembling
- Or use mkarchiso's `-r` flag to remove work dir after build
**Warning signs:** Wrong packages or configs appearing in ISO

### Pitfall 4: pacman.conf Repository Order
**What goes wrong:** Wrong package version installed (generic instead of T2 kernel)
**Why it happens:** Repository order matters - first match wins
**How to avoid:**
- T2's pacman.conf: arch-mact2 repo BEFORE core/extra
- Foundry's pacman.conf: Standard order (core, extra, multilib)
**Warning signs:** `linux` package installed instead of `linux-t2`

### Pitfall 5: Kernel Parameter Divergence
**What goes wrong:** T2 boots with missing drivers, Foundry boots with unnecessary T2 params
**Why it happens:** Boot configs (grub.cfg, syslinux.cfg) not profile-specific
**How to avoid:**
- Each profile has its own grub/syslinux directory
- T2: `intel_iommu=on iommu=pt pcie_ports=compat`
- Foundry: `nvidia-drm.modeset=1` (if NVIDIA in profile)
**Warning signs:** Hardware not working on live ISO

### Pitfall 6: Missing Firmware in initramfs
**What goes wrong:** T2 hardware (WiFi, keyboard) doesn't work on boot
**Why it happens:** T2 firmware not included in initramfs, or wrong mkinitcpio hooks
**How to avoid:**
- T2 profile airootfs must have correct mkinitcpio.conf
- Include `apple-bce` module in MODULES
- Verify firmware packages in package list
**Warning signs:** "Firmware not found" warnings during build, non-working hardware

## Code Examples

Verified patterns for implementation:

### build-t2.sh Structure
```bash
#!/bin/bash
# Source: VulcanOS build pattern
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARCHISO_DIR="$PROJECT_DIR/archiso"
PROFILE="t2"
WORK_DIR="/tmp/vulcanos-work-$PROFILE"
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"
OUT_DIR="$PROJECT_DIR/out"

# Source shared functions
source "$SCRIPT_DIR/lib/build-common.sh"

main() {
    check_root
    check_dependencies
    validate_profile "$PROFILE"

    clean_build "$WORK_DIR" "$ASSEMBLED_DIR"
    assemble_profile "$PROFILE" "$ASSEMBLED_DIR"

    run_mkarchiso "$ASSEMBLED_DIR" "$WORK_DIR" "$OUT_DIR"

    generate_checksums "$OUT_DIR"
    fix_permissions "$OUT_DIR"
    cleanup "$WORK_DIR" "$ASSEMBLED_DIR"
    show_info "$OUT_DIR" "$PROFILE"
}

main "$@"
```

### rsync Overlay with Precedence
```bash
# Source: rsync man page, verified pattern
# Base files copied first, then profile overlays (profile wins)
merge_airootfs() {
    local base_dir="$1"
    local profile_dir="$2"
    local target_dir="$3"

    # Copy base first
    rsync -a --delete "$base_dir/" "$target_dir/"

    # Overlay profile (--ignore-existing NOT used - profile wins)
    rsync -a "$profile_dir/" "$target_dir/"
}
```

### Package List Concatenation
```bash
# Source: Shell best practices
merge_packages() {
    local base_pkgs="$1"
    local profile_pkgs="$2"
    local output="$3"

    # Concatenate, remove comments and empty lines for clean output
    cat "$base_pkgs" "$profile_pkgs" | \
        grep -v '^[[:space:]]*#' | \
        grep -v '^[[:space:]]*$' | \
        sort -u > "$output"
}
```

### Profile-Specific profiledef.sh
```bash
#!/usr/bin/env bash
# shellcheck disable=SC2034
# Source: archiso/docs/README.profile.rst

iso_name="vulcanos-t2"
iso_label="VULCAN_T2_$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y%m)"
iso_publisher="VulcanOS Project"
iso_application="VulcanOS T2 MacBook Pro Live/Install Medium"
iso_version="$(date --date="@${SOURCE_DATE_EPOCH:-$(date +%s)}" +%Y.%m.%d)"
install_dir="arch"
buildmodes=('iso')
bootmodes=(
    'bios.syslinux.mbr'
    'bios.syslinux.eltorito'
    'uefi-ia32.grub.esp'
    'uefi-x64.grub.esp'
    'uefi-ia32.grub.eltorito'
    'uefi-x64.grub.eltorito'
)
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86' '-b' '1M' '-Xdict-size' '1M')
file_permissions=(
    ["/etc/shadow"]="0:0:400"
    ["/etc/gshadow"]="0:0:400"
    ["/root"]="0:0:750"
    ["/etc/sudoers.d/wheel"]="0:0:440"
    ["/usr/local/bin/"]="0:0:755"
)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Single profile per repo | Multi-profile with shared base | N/A (custom pattern) | Reduces duplication |
| ext4+squashfs rootfs | squashfs or erofs | archiso ~2023 | erofs is faster, smaller |
| Manual package concatenation | Build-time merge scripts | Standard practice | Cleaner separation |

**Deprecated/outdated:**
- `ext4+squashfs` airootfs type: Still works but erofs or plain squashfs preferred
- Hardcoded ISO labels: Use `%ARCHISO_LABEL%` template variables

## GPU/ML Stack for Foundry Profile

Based on CONTEXT.md requirements, Foundry includes GPU acceleration:

### NVIDIA Stack
```
# packages.profile (foundry) - GPU/ML section
nvidia
nvidia-utils
nvidia-settings
lib32-nvidia-utils
cuda
cudnn
```

### AMD/Intel Stack (for broader compatibility)
```
mesa
vulkan-radeon
lib32-mesa
lib32-vulkan-radeon
vulkan-intel
lib32-vulkan-intel
```

### ML Frameworks (optional, increases ISO size significantly)
```
python-pytorch-cuda
python-tensorflow-cuda
```

**Note:** CUDA alone is ~3GB. Full ML stack may push ISO to 10GB+. Consider making ML packages installer-time rather than ISO-time.

## Open Questions

Things that couldn't be fully resolved:

1. **Kernel in Base vs. Profile**
   - What we know: Both T2 and Foundry need kernels, but different ones
   - What's unclear: Should base have NO kernel, or should profiles override?
   - Recommendation: No kernel in base; each profile specifies its own (`linux-t2` vs `linux`)

2. **Shared EFI Boot Configuration**
   - What we know: efiboot/ contains systemd-boot config
   - What's unclear: Can it be truly shared, or does each profile need custom?
   - Recommendation: Start with shared, move to profile-specific if issues arise

3. **ML Package Size Impact**
   - What we know: CUDA is large (~3GB), full ML stack is huge
   - What's unclear: Acceptable ISO size for Foundry profile
   - Recommendation: Include NVIDIA drivers + CUDA in ISO; make pytorch/tensorflow post-install

## Sources

### Primary (HIGH confidence)
- [Arch Wiki - Archiso](https://wiki.archlinux.org/title/Archiso) - Profile structure, customization
- [archiso/docs/README.profile.rst](https://github.com/archlinux/archiso/blob/master/docs/README.profile.rst) - Official profile documentation
- [mkarchiso man page](https://man.archlinux.org/man/extra/archiso/mkarchiso.1.en) - Command options
- [archiso baseline profiledef.sh](https://github.com/archlinux/archiso/blob/master/configs/baseline/profiledef.sh) - Example profile
- [archiso releng profiledef.sh](https://github.com/archlinux/archiso/blob/master/configs/releng/profiledef.sh) - Example profile

### Secondary (MEDIUM confidence)
- [rsync man page](https://linux.die.net/man/1/rsync) - Directory merge options
- [ArcoLinux ISO building guide](https://www.arcolinuxiso.com/a-comprehensive-guide-to-iso-building/) - Multi-ISO patterns
- [EndeavourOS-ISO GitHub](https://github.com/endeavouros-team/EndeavourOS-ISO) - Real-world archiso customization

### Tertiary (LOW confidence)
- Community forum discussions on archiso gotchas

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - archiso is well-documented, rsync is standard
- Architecture: HIGH - Derived from official archiso patterns and EndeavourOS examples
- Pitfalls: HIGH - Common issues documented in Arch forums and wikis
- GPU/ML packages: MEDIUM - Package names verified but sizing estimates approximate

**Research date:** 2026-02-02
**Valid until:** 90 days (archiso is stable, changes infrequently)
