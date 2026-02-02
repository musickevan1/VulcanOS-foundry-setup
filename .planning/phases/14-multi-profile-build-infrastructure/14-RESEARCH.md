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

## Assemble Script Details

### Script Organization

```
scripts/
├── build-t2.sh           # Entry point for T2 builds
├── build-foundry.sh      # Entry point for Foundry builds
├── build.sh              # Deprecation stub (errors with guidance)
└── lib/
    └── build-common.sh   # Shared functions
```

### Critical rsync Flags

```bash
# Step 1: Copy base (with --delete to ensure clean slate)
rsync -a --delete "$base_airootfs/" "$work_dir/airootfs/"

# Step 2: Overlay profile (NO --delete, profile wins on conflict)
rsync -a "$profile_airootfs/" "$work_dir/airootfs/"
```

| Flag | Purpose |
|------|---------|
| `-a` | Archive: preserves permissions, symlinks, timestamps |
| `--delete` | FIRST copy only: ensures work dir matches base exactly |
| NO `--delete` on overlay | Profile adds/overwrites but doesn't delete base files |
| NO `--ignore-existing` | Profile SHOULD override base files when both exist |

### Error Handling Pattern

```bash
set -e
trap cleanup EXIT

cleanup() {
    local exit_code=$?
    if [[ $exit_code -ne 0 ]]; then
        error "Build failed. Cleaning up..."
    fi
    [[ -d "$WORK_DIR" ]] && rm -rf "$WORK_DIR"
    [[ -d "$ASSEMBLED_DIR" ]] && rm -rf "$ASSEMBLED_DIR"
}
```

### Edge Cases

| Edge Case | Solution |
|-----------|----------|
| Profile dir doesn't exist | Validate before assemble, exit with clear error |
| Empty profile airootfs | rsync handles gracefully — just copies base |
| Symlinks in airootfs | rsync `-a` preserves symlinks (correct for stow structure) |
| Interrupted build | trap EXIT cleans up work directories |
| Concurrent builds | Different work dirs (`-t2` vs `-foundry`) prevent collision |

### Complete assemble_profile Function

```bash
assemble_profile() {
    local profile="$1"
    local assembled_dir="$2"

    local base_dir="$ARCHISO_DIR/base"
    local profile_dir="$ARCHISO_DIR/profiles/$profile"

    info "Assembling profile: $profile"

    # Clean assembled directory
    rm -rf "$assembled_dir"
    mkdir -p "$assembled_dir"

    # Step 1: Copy base airootfs
    info "  Copying base airootfs..."
    rsync -a --delete "$base_dir/airootfs/" "$assembled_dir/airootfs/"

    # Step 2: Overlay profile airootfs (profile wins)
    if [[ -d "$profile_dir/airootfs" ]]; then
        info "  Overlaying profile airootfs..."
        rsync -a "$profile_dir/airootfs/" "$assembled_dir/airootfs/"
    fi

    # Step 3: Merge package lists
    info "  Merging package lists..."
    cat "$base_dir/packages.base" "$profile_dir/packages.profile" | \
        grep -v '^[[:space:]]*#' | \
        grep -v '^[[:space:]]*$' | \
        sort -u > "$assembled_dir/packages.x86_64"

    # Step 4: Copy profile-specific configs
    info "  Copying profile configs..."
    cp "$profile_dir/pacman.conf" "$assembled_dir/"
    cp "$profile_dir/profiledef.sh" "$assembled_dir/"

    # Step 5: Copy boot configs
    cp -r "$profile_dir/grub" "$assembled_dir/"
    cp -r "$profile_dir/syslinux" "$assembled_dir/"

    # Step 6: Copy efiboot (profile-specific if exists, else shared)
    if [[ -d "$profile_dir/efiboot" ]]; then
        cp -r "$profile_dir/efiboot" "$assembled_dir/"
    else
        cp -r "$ARCHISO_DIR/efiboot" "$assembled_dir/"
    fi

    success "Profile assembled: $assembled_dir"
}
```

### Complete validate_profile Function

```bash
validate_profile() {
    local profile="$1"
    local errors=0

    info "Validating profile: $profile"

    local required_files=(
        "base/packages.base"
        "base/airootfs/etc/skel"
        "profiles/$profile/packages.profile"
        "profiles/$profile/pacman.conf"
        "profiles/$profile/profiledef.sh"
        "profiles/$profile/grub/grub.cfg"
        "profiles/$profile/syslinux/syslinux.cfg"
    )

    for file in "${required_files[@]}"; do
        if [[ ! -e "$ARCHISO_DIR/$file" ]]; then
            error "Missing: $file"
            ((errors++))
        fi
    done

    if [[ ! -d "$ARCHISO_DIR/base/airootfs" ]]; then
        error "Missing: base/airootfs directory"
        ((errors++))
    fi

    if [[ ! -d "$ARCHISO_DIR/profiles/$profile/airootfs" ]]; then
        warn "No profile-specific airootfs (will use base only)"
    fi

    if [[ $errors -gt 0 ]]; then
        error "Validation failed with $errors error(s)"
        exit 1
    fi

    success "Validation passed"
}
```

### build-t2.sh Entry Point

```bash
#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARCHISO_DIR="$PROJECT_DIR/archiso"
PROFILE="t2"
WORK_DIR="/tmp/vulcanos-work-$PROFILE"
ASSEMBLED_DIR="/tmp/vulcanos-assembled-$PROFILE"
OUT_DIR="$PROJECT_DIR/out"

source "$SCRIPT_DIR/lib/build-common.sh"
trap cleanup EXIT

main() {
    echo "=========================================="
    echo "  VulcanOS T2 - Build"
    echo "=========================================="

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

main "$@"
```

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

## Package Split Analysis

Based on current `packages.x86_64` (198 lines), here's the verified split:

### packages.base (~85 packages)
All shared packages — desktop, tools, development, fonts, audio, theming:

```
# BASE SYSTEM (NO KERNEL)
base
linux-firmware

# BOOTLOADER
grub
efibootmgr
syslinux

# SYSTEM ESSENTIALS
networkmanager
sudo
which
mkinitcpio
mkinitcpio-archiso

# DESKTOP ENVIRONMENT
hyprland
xdg-desktop-portal-hyprland
xdg-desktop-portal-gtk
sddm
qt6-5compat
qt6-declarative
qt6-svg
polkit-gnome
waybar
wofi
swaync
hyprlock
hypridle
hyprpaper
swww
hyprmon-bin
nwg-displays
nwg-dock-hyprland
kitty

# FILE MANAGEMENT
nautilus
gvfs
gvfs-mtp
yazi
ffmpegthumbnailer
p7zip
poppler
imagemagick
ripdrag

# WAYLAND UTILITIES
wl-clipboard
grim
slurp
swappy
wf-recorder
hyprpicker
brightnessctl

# AUDIO
pipewire
wireplumber
pipewire-pulse
pipewire-alsa
pamixer

# BASIC TOOLS
git
neovim
openssh
curl
wget

# CLI UTILITIES
socat
vim
ripgrep
fd
fzf
bat
glow
mdcat
jq
wtype
eza
btop
starship
zoxide
tree
less
unzip
zip
cliphist

# DEVELOPMENT
docker
docker-compose
lazygit
github-cli
stow
base-devel

# LANGUAGE SERVERS
bash-language-server
typescript-language-server
pyright
yaml-language-server
lua-language-server
rust-analyzer
gopls

# SCREENSAVERS
cmatrix

# THEMING
nwg-look
papirus-icon-theme
kvantum
qt5ct
qt6ct
gnome-themes-extra
libnotify

# PRODUCTIVITY
libreoffice-fresh
obsidian

# MEDIA
spotify-launcher

# FONTS
ttf-jetbrains-mono-nerd
noto-fonts
noto-fonts-emoji

# SPEECH-TO-TEXT
hyprwhspr
python-requests
ffmpeg
ydotool
```

### profiles/t2/packages.profile (6 packages)

```
# T2 KERNEL
linux-t2
linux-t2-headers

# T2 HARDWARE
apple-bcm-firmware
apple-t2-audio-config
t2fanrd
tiny-dfr
```

### profiles/foundry/packages.profile (~10 packages)

```
# GENERIC KERNEL
linux
linux-headers

# NVIDIA GPU DRIVERS (RTX 5070 Ti)
nvidia-open-dkms
nvidia-utils
nvidia-settings
lib32-nvidia-utils

# CUDA/ML
cuda
cudnn
nvidia-container-toolkit
```

### Key Decisions
- **No kernel in base** — Avoids conflicts between linux and linux-t2
- **wl-clipboard once** — Was duplicated in original, now in base only
- **NVIDIA in Foundry only** — T2 MacBook has no dedicated GPU
- **Speech-to-text in base** — Both profiles can use local Whisper
- **Development tools in base** — Same dev experience on both machines

## Migration Strategy

### Current State

```
archiso/
├── packages.x86_64      # 197 lines, T2-specific (linux-t2, apple-*)
├── pacman.conf          # arch-mact2 COMMENTED OUT (VM testing mode)
├── profiledef.sh        # Single profile
├── grub/, syslinux/     # Single boot configs
└── airootfs/            # 5.8MB overlay
    ├── etc/modprobe.d/  # T2-SPECIFIC: apple-bce.conf, apple-gmux.conf
    └── etc/skel/        # SHARED: hypr, kitty, themes, etc.
```

### Migration Approach: Copy-First, Remove-Last

1. **Create new structure** (add directories, don't touch existing)
2. **Copy content** (base gets shared, profiles get specific)
3. **Create new scripts** (build-t2.sh, build-foundry.sh, lib/)
4. **Test T2 build** (compare to known-working ISO)
5. **Test Foundry build** (new profile, expect issues)
6. **Remove old structure** (only after both verified)

### What Goes Where

**base/airootfs/etc/skel/** — All dotfiles (shared desktop experience)
**base/airootfs/usr/** — Scripts, themes, icons, backgrounds
**profiles/t2/airootfs/etc/modprobe.d/** — apple-bce.conf, apple-gmux.conf
**profiles/t2/pacman.conf** — WITH arch-mact2 repo UNCOMMENTED
**profiles/foundry/airootfs/etc/** — mkinitcpio.conf with nvidia modules

### Critical Discovery

Current `pacman.conf` has arch-mact2 **commented out** — it's in VM testing mode. The T2 profile's pacman.conf must have arch-mact2 **uncommented** and placed FIRST for kernel priority.

### Verification Checklist

**After T2 build:**
- [ ] ISO size similar to previous
- [ ] ISO boots in QEMU and on T2 hardware
- [ ] WiFi works (apple-bcm-firmware)
- [ ] Kernel is `linux-t2` not `linux`

**After Foundry build:**
- [ ] ISO boots in QEMU
- [ ] Kernel is `linux` not `linux-t2`
- [ ] No T2 packages present
- [ ] NVIDIA drivers present

### Rollback Plan

```bash
# Before migration: commit everything
git add -A && git commit -m "Pre-multiprofile backup"

# If migration fails:
git checkout archiso/
```

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
