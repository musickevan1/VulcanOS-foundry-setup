# Architecture Research: VulcanOS v3.0 Multi-Profile Structure

**Domain:** archiso multi-profile build system
**Researched:** 2026-02-02
**Confidence:** MEDIUM (archiso has no native multi-profile mechanism; architecture is custom design)

## Executive Summary

VulcanOS currently uses a single archiso profile targeting T2 MacBook Pro hardware. The v3.0 goal is to support two profiles: **foundry** (generic hardware) and **t2** (T2 MacBook Pro specific). After researching archiso internals and how distributions like ArcoLinux handle multiple variants, the key finding is that **archiso has no built-in multi-profile mechanism** — each `mkarchiso` invocation builds exactly one profile.

The recommended architecture uses a **base profile with profile-specific overlays** approach, where:
1. A shared `base/` directory contains common packages and configs
2. Profile directories (`foundry/`, `t2/`) contain only profile-specific additions
3. A custom build script assembles the complete profile at build time

This design minimizes duplication while maintaining clear separation between profiles.

## Current Architecture

### Single Profile Structure

```
archiso/
├── airootfs/                    # Root filesystem overlay
│   ├── etc/
│   │   ├── hostname
│   │   ├── locale.conf
│   │   ├── mkinitcpio.conf      # VM version (no T2 modules)
│   │   ├── mkinitcpio.conf.full # T2 version (with apple-bce)
│   │   ├── modprobe.d/
│   │   │   ├── apple-bce.conf   # T2-specific
│   │   │   └── apple-gmux.conf  # T2-specific
│   │   ├── pacman.d/
│   │   ├── sddm.conf.d/
│   │   ├── skel/                # User skeleton (configs, themes)
│   │   └── systemd/
│   └── usr/
│       ├── local/bin/           # Custom scripts
│       └── share/               # SDDM theme, backgrounds
├── efiboot/                     # systemd-boot configs
├── grub/                        # GRUB configs (T2-specific params)
├── syslinux/                    # BIOS boot
├── packages.x86_64              # All packages (T2-specific included)
├── packages.x86_64.full         # Backup of full T2 list
├── pacman.conf                  # VM version (no arch-mact2)
├── pacman.conf.full             # T2 version (with arch-mact2)
└── profiledef.sh                # Build metadata
```

### Build Process

The current `scripts/build.sh`:
1. Checks dependencies (mkarchiso, git, mksquashfs, xorriso)
2. Cleans previous build artifacts
3. Runs `prepare_skel()` to copy dotfiles to airootfs/etc/skel
4. Calls `mkarchiso -v -w $WORK_DIR -o $OUT_DIR $ARCHISO_DIR`
5. Generates checksums
6. Cleans up work directory

**Key limitation:** The `.full` suffix convention for T2 configs requires manual swapping before builds. This is error-prone and doesn't scale to multiple profiles.

### T2-Specific Components

Components that are T2-only (not needed for foundry):

| Component | Location | Purpose |
|-----------|----------|---------|
| linux-t2 | packages.x86_64 | T2-patched kernel |
| apple-bcm-firmware | packages.x86_64 | WiFi firmware |
| apple-t2-audio-config | packages.x86_64 | Audio configuration |
| t2fanrd | packages.x86_64 | Fan control |
| tiny-dfr | packages.x86_64 | Touch Bar support |
| apple-bce.conf | modprobe.d/ | BCE driver config |
| apple-gmux.conf | modprobe.d/ | GPU switching |
| arch-mact2 repo | pacman.conf | T2 package repository |
| T2 kernel params | grub.cfg | intel_iommu, pcie_ports |
| apple-bce MODULES | mkinitcpio.conf | Keyboard/trackpad early |

## Multi-Profile Architecture

### How mkarchiso Works

From examining `/usr/bin/mkarchiso` and official documentation:

1. **Profile path is the only input** — `mkarchiso` takes a single profile directory path
2. **Package list is a flat file** — `packages.x86_64` parsed with sed to strip comments
3. **No include/import mechanism** — Cannot reference other package files
4. **Template substitution** — Only for `%ARCHISO_LABEL%`, `%INSTALL_DIR%`, `%ARCH%`
5. **airootfs is copied verbatim** — No overlay/merge mechanism

```bash
# How mkarchiso reads packages (from source):
mapfile -t pkg_list_from_file < <(sed '/^[[:blank:]]*#.*/d;s/#.*//;/^[[:blank:]]*$/d' "${packages}")
```

### Directory Structure Options

**Option A: Completely Separate Profiles**

```
profiles/
├── foundry/
│   ├── airootfs/
│   ├── packages.x86_64
│   ├── pacman.conf
│   └── profiledef.sh
└── t2/
    ├── airootfs/
    ├── packages.x86_64
    ├── pacman.conf
    └── profiledef.sh
```

*Pros:* Simple, clear boundaries, mkarchiso-native
*Cons:* High duplication (~95% identical), sync maintenance burden

**Option B: Base + Overlay (Recommended)**

```
profiles/
├── base/
│   ├── airootfs/           # Common filesystem overlay
│   ├── packages.base       # Core packages (~180 packages)
│   ├── pacman.base.conf    # Common repos (core, extra, multilib)
│   └── grub/               # Common GRUB structure
├── foundry/
│   ├── airootfs/           # Foundry-specific overlays (if any)
│   ├── packages.profile    # Foundry-specific packages (~5-10)
│   ├── pacman.profile.conf # No additional repos
│   ├── grub.profile.cfg    # Standard kernel params
│   └── profiledef.sh
├── t2/
│   ├── airootfs/           # T2-specific overlays
│   │   └── etc/modprobe.d/ # apple-bce.conf, apple-gmux.conf
│   ├── packages.profile    # T2 packages (~5)
│   ├── pacman.profile.conf # arch-mact2 repo
│   ├── grub.profile.cfg    # T2 kernel params
│   └── profiledef.sh
└── common/                  # Shared resources
    ├── grub-themes/
    ├── sddm-themes/
    └── wallpapers/
```

*Pros:* Minimal duplication, clear profile-specific changes
*Cons:* Requires custom assembly script

**Option C: Single Profile with Build-time Selection**

Keep current structure but parameterize:

```bash
./scripts/build.sh --profile=foundry
./scripts/build.sh --profile=t2
```

*Pros:* Minimal restructure, familiar build flow
*Cons:* Complex conditionals, harder to understand differences

### Recommendation: Option B (Base + Overlay)

This is the cleanest long-term architecture. The assembly cost is minimal (bash script concatenates files and merges directories), and the separation makes profile differences explicit.

## Recommended Structure

### Directory Layout

```
VulcanOS/
├── archiso/                     # DEPRECATED - migrate to profiles/
├── profiles/
│   ├── base/                    # Shared foundation
│   │   ├── airootfs/
│   │   │   ├── etc/
│   │   │   │   ├── hostname.tpl     # Template: VULCANOS_${PROFILE}
│   │   │   │   ├── locale.conf
│   │   │   │   ├── locale.gen
│   │   │   │   ├── pacman.d/
│   │   │   │   ├── sddm.conf.d/
│   │   │   │   ├── skel/            # User configs (from dotfiles/)
│   │   │   │   ├── sudoers.d/
│   │   │   │   ├── systemd/
│   │   │   │   └── vconsole.conf
│   │   │   └── usr/
│   │   │       ├── local/bin/       # vulcan-* scripts
│   │   │       └── share/           # SDDM theme, backgrounds
│   │   ├── efiboot/
│   │   ├── grub/
│   │   │   ├── grub-base.cfg        # Common entries
│   │   │   └── themes/
│   │   ├── syslinux/
│   │   ├── packages.base            # ~180 shared packages
│   │   └── pacman.base.conf         # core, extra, multilib
│   │
│   ├── foundry/                 # Generic hardware profile
│   │   ├── airootfs/
│   │   │   └── etc/
│   │   │       └── mkinitcpio.conf  # Standard modules
│   │   ├── packages.profile         # linux, linux-firmware
│   │   ├── pacman.profile.conf      # Empty (no extra repos)
│   │   ├── grub.profile.cfg         # Standard kernel params
│   │   └── profiledef.sh            # iso_name="vulcanos-foundry"
│   │
│   ├── t2/                      # T2 MacBook Pro profile
│   │   ├── airootfs/
│   │   │   └── etc/
│   │   │       ├── mkinitcpio.conf  # apple-bce in MODULES
│   │   │       └── modprobe.d/
│   │   │           ├── apple-bce.conf
│   │   │           └── apple-gmux.conf
│   │   ├── packages.profile         # linux-t2, apple-*, t2fanrd, tiny-dfr
│   │   ├── pacman.profile.conf      # arch-mact2 repo
│   │   ├── grub.profile.cfg         # T2 kernel params
│   │   └── profiledef.sh            # iso_name="vulcanos-t2"
│   │
│   └── _build/                  # Build-time assembly (gitignored)
│       ├── foundry/             # Assembled foundry profile
│       └── t2/                  # Assembled t2 profile
│
├── scripts/
│   ├── build.sh                 # Updated: takes --profile argument
│   ├── assemble-profile.sh      # NEW: merges base + profile
│   ├── prepare.sh
│   └── test-iso.sh
│
├── dotfiles/                    # Unchanged (GNU Stow structure)
└── ...
```

### Package List Strategy

**packages.base** (shared, ~180 packages):
```
# BASE SYSTEM
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

# DESKTOP (Hyprland + SDDM)
hyprland
xdg-desktop-portal-hyprland
xdg-desktop-portal-gtk
sddm
# ... rest of desktop packages

# DEVELOPMENT
# ... development packages

# FONTS
# ... font packages
```

**packages.profile (foundry)** (~5 packages):
```
# FOUNDRY PROFILE - Generic hardware
linux
linux-headers
```

**packages.profile (t2)** (~10 packages):
```
# T2 PROFILE - MacBook Pro T2 hardware
linux-t2
linux-t2-headers
apple-bcm-firmware
apple-t2-audio-config
t2fanrd
tiny-dfr
```

**Assembly script concatenates:**
```bash
cat profiles/base/packages.base profiles/${PROFILE}/packages.profile > profiles/_build/${PROFILE}/packages.x86_64
```

### Config Overlay Strategy

**pacman.conf assembly:**
```bash
# profiles/base/pacman.base.conf - shared repos
[options]
HoldPkg     = pacman glibc
Architecture = auto
CheckSpace
ParallelDownloads = 5
SigLevel    = Required DatabaseOptional

# profiles/t2/pacman.profile.conf - T2 repo (prepended for priority)
[arch-mact2]
Server = https://mirror.funami.tech/arch-mact2/os/x86_64
SigLevel = Never
```

**Assembly:**
```bash
# T2 profile: T2 repos first for kernel priority
cat profiles/t2/pacman.profile.conf profiles/base/pacman.base.conf > profiles/_build/t2/pacman.conf

# Foundry profile: just base repos
cp profiles/base/pacman.base.conf profiles/_build/foundry/pacman.conf
```

**grub.cfg assembly:**
```bash
# Inject profile-specific kernel params into template
sed "s/%KERNEL_PARAMS%/${PROFILE_KERNEL_PARAMS}/" \
    profiles/base/grub/grub-base.cfg > profiles/_build/${PROFILE}/grub/grub.cfg
```

**airootfs merge:**
```bash
# Copy base first, then profile-specific overlays
cp -r profiles/base/airootfs/* profiles/_build/${PROFILE}/airootfs/
cp -r profiles/${PROFILE}/airootfs/* profiles/_build/${PROFILE}/airootfs/
```

## Migration Path

### Phase 1: Create Profile Structure (No Breaking Changes)

1. Create `profiles/` directory structure
2. Extract shared components from `archiso/` to `profiles/base/`
3. Create `profiles/t2/` with T2-specific overrides
4. Create `profiles/foundry/` with generic kernel
5. **Keep `archiso/` working** during transition

Files to create:
- `profiles/base/packages.base` (current packages minus T2)
- `profiles/base/pacman.base.conf` (without arch-mact2)
- `profiles/base/airootfs/` (current airootfs minus modprobe.d T2 files)
- `profiles/t2/packages.profile` (T2 packages only)
- `profiles/t2/pacman.profile.conf` (arch-mact2 repo)
- `profiles/t2/airootfs/etc/modprobe.d/` (apple configs)
- `profiles/foundry/packages.profile` (linux, linux-headers)
- `scripts/assemble-profile.sh` (new assembly script)

### Phase 2: Build Script Updates

1. Update `scripts/build.sh` to accept `--profile` argument
2. Call `assemble-profile.sh` before mkarchiso
3. Point mkarchiso at `profiles/_build/${PROFILE}/`
4. Update output naming: `vulcanos-${PROFILE}-${VERSION}-x86_64.iso`

```bash
# Example updated build.sh usage
./scripts/build.sh --profile=foundry  # Builds foundry ISO
./scripts/build.sh --profile=t2       # Builds T2 ISO
./scripts/build.sh                    # Default: foundry (or error)
```

### Phase 3: Deprecate Old Structure

1. Verify both profiles build and boot correctly
2. Remove `archiso/` directory
3. Update CLAUDE.md and documentation
4. Update CI/CD if applicable

### Phase 4: Profile Polish

1. Profile-specific wallpapers/themes (if desired)
2. Profile-specific default configs
3. Profile-specific installer options

## Integration Points

### Existing Scripts

| Script | Impact | Changes Needed |
|--------|--------|----------------|
| `build.sh` | High | Add --profile arg, call assemble-profile.sh |
| `prepare.sh` | Low | None (builds AUR packages, profile-agnostic) |
| `test-iso.sh` | Medium | Accept profile arg for ISO path |
| `build-aur-repo.sh` | Low | None (AUR packages shared) |

### Dotfiles

The dotfiles structure remains unchanged. The `prepare_skel()` function in build.sh copies from `dotfiles/` to `profiles/_build/${PROFILE}/airootfs/etc/skel/`.

No changes needed to GNU Stow structure.

### vulcan-appearance-manager

The appearance manager works with user configs in `~/.config/`. It is profile-agnostic after installation.

No changes needed.

### Custom Repository

The `customrepo/` directory for AUR packages remains shared between profiles. Both profiles can use the same custom repository — the package list determines what gets installed.

## Build System Changes

### assemble-profile.sh (New Script)

```bash
#!/bin/bash
# Assembles a complete archiso profile from base + profile-specific components

set -e

PROFILE="${1:?Usage: assemble-profile.sh <profile>}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
PROFILES_DIR="${PROJECT_DIR}/profiles"
BUILD_DIR="${PROFILES_DIR}/_build/${PROFILE}"

# Validate profile exists
if [[ ! -d "${PROFILES_DIR}/${PROFILE}" ]]; then
    echo "Error: Profile '${PROFILE}' not found in ${PROFILES_DIR}"
    echo "Available profiles:"
    ls -1 "${PROFILES_DIR}" | grep -v "^base$" | grep -v "^_build$" | grep -v "^common$"
    exit 1
fi

echo "Assembling profile: ${PROFILE}"

# Clean previous assembly
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"

# 1. Copy base structure
echo "  Copying base profile..."
cp -r "${PROFILES_DIR}/base/"* "${BUILD_DIR}/"

# 2. Overlay profile-specific airootfs
echo "  Applying profile overlays..."
if [[ -d "${PROFILES_DIR}/${PROFILE}/airootfs" ]]; then
    cp -r "${PROFILES_DIR}/${PROFILE}/airootfs/"* "${BUILD_DIR}/airootfs/"
fi

# 3. Copy profile-specific profiledef.sh (replaces base)
if [[ -f "${PROFILES_DIR}/${PROFILE}/profiledef.sh" ]]; then
    cp "${PROFILES_DIR}/${PROFILE}/profiledef.sh" "${BUILD_DIR}/profiledef.sh"
fi

# 4. Assemble packages.x86_64
echo "  Assembling package list..."
cat "${PROFILES_DIR}/base/packages.base" \
    "${PROFILES_DIR}/${PROFILE}/packages.profile" \
    > "${BUILD_DIR}/packages.x86_64"

# 5. Assemble pacman.conf (profile repos prepended for priority)
echo "  Assembling pacman.conf..."
if [[ -f "${PROFILES_DIR}/${PROFILE}/pacman.profile.conf" ]]; then
    # Prepend profile-specific repos, then base repos
    cat "${PROFILES_DIR}/${PROFILE}/pacman.profile.conf" \
        "${PROFILES_DIR}/base/pacman.base.conf" \
        > "${BUILD_DIR}/pacman.conf"
else
    # Just use base repos
    cp "${PROFILES_DIR}/base/pacman.base.conf" "${BUILD_DIR}/pacman.conf"
fi

# 6. Handle GRUB configuration
echo "  Processing GRUB config..."
if [[ -f "${PROFILES_DIR}/${PROFILE}/grub.profile.cfg" ]]; then
    # Read profile-specific kernel params
    source "${PROFILES_DIR}/${PROFILE}/grub.profile.cfg"

    # Substitute into base grub config
    sed "s|%KERNEL_PARAMS%|${KERNEL_PARAMS:-}|g" \
        "${BUILD_DIR}/grub/grub.cfg" > "${BUILD_DIR}/grub/grub.cfg.tmp"
    mv "${BUILD_DIR}/grub/grub.cfg.tmp" "${BUILD_DIR}/grub/grub.cfg"
fi

# 7. Clean up template files (remove .tpl suffix if any)
find "${BUILD_DIR}" -name "*.tpl" -exec sh -c 'mv "$1" "${1%.tpl}"' _ {} \;

echo ""
echo "Profile '${PROFILE}' assembled at:"
echo "  ${BUILD_DIR}"
echo ""
echo "Package count: $(wc -l < "${BUILD_DIR}/packages.x86_64")"
```

### Updated build.sh Flow

```
1. Parse --profile argument (required or default to foundry)
2. Run assemble-profile.sh ${PROFILE}
3. Run prepare_skel() targeting profiles/_build/${PROFILE}/airootfs/etc/skel/
4. Run mkarchiso -v -w /tmp/vulcanos-${PROFILE}-work -o out/ profiles/_build/${PROFILE}/
5. Rename output: mv out/*.iso out/vulcanos-${PROFILE}-${VERSION}-x86_64.iso
6. Generate checksums
```

## Alternative Considerations

### Why Not Separate Repositories?

Some distributions maintain separate repos per variant (e.g., ArcoLinux had 64+ repos). This was rejected because:
- VulcanOS only has 2 profiles
- Separate repos increase maintenance burden
- Shared dotfiles/scripts benefit from single repo
- Build-time assembly is simpler than repo management

### Why Not Profile Inheritance in profiledef.sh?

archiso's profiledef.sh doesn't support inheritance or sourcing. While we could source a base profiledef.sh, the other profile components (packages, airootfs) still need assembly. A build-time assembly script is cleaner than partial inheritance.

### Why Base + Overlay vs Overlay-only?

An overlay-only approach would have:
- `profiles/foundry/` = complete profile
- `profiles/t2/` = only T2 additions

This requires complex merge logic to add/remove packages and configs. The base + overlay approach is simpler: base is always the same, overlays only add.

## Profile-Specific Configuration Summary

### Foundry Profile (Generic Hardware)

| Component | Value |
|-----------|-------|
| Kernel | `linux` (standard Arch kernel) |
| Kernel params | None extra |
| pacman repos | core, extra, multilib |
| mkinitcpio MODULES | `()` (empty, auto-detect) |
| modprobe.d | None extra |
| ISO name | `vulcanos-foundry-YYYY.MM.DD-x86_64.iso` |

### T2 Profile (MacBook Pro)

| Component | Value |
|-----------|-------|
| Kernel | `linux-t2` (patched for T2) |
| Kernel params | `intel_iommu=on iommu=pt pcie_ports=compat` |
| pacman repos | arch-mact2, core, extra, multilib |
| mkinitcpio MODULES | `(apple-bce)` |
| modprobe.d | `apple-bce.conf`, `apple-gmux.conf` |
| ISO name | `vulcanos-t2-YYYY.MM.DD-x86_64.iso` |

## Sources

- [Arch Wiki - archiso](https://wiki.archlinux.org/title/Archiso) - Official archiso documentation
- [archiso README.profile.rst](https://github.com/archlinux/archiso/blob/master/docs/README.profile.rst) - Profile structure specification
- [archlinux/archiso GitHub](https://github.com/archlinux/archiso) - Official archiso source
- `/usr/bin/mkarchiso` - Examined source for package handling logic
- `/usr/share/archiso/configs/releng/` - Reference official profile structure
- [ArcoLinux ISO documentation](https://www.arcolinuxiso.com/1-installing-and-learning-about-archiso/) - Example of multi-variant builds
- [mkarchiso Script DeepWiki](https://deepwiki.com/archlinux/archiso/2.1-mkarchiso-script) - mkarchiso documentation

**Confidence levels:**
- mkarchiso behavior: HIGH (examined source code directly)
- Recommended structure: MEDIUM (custom design, not battle-tested)
- Migration path: MEDIUM (logical but not validated)
- Integration points: HIGH (examined existing codebase)
