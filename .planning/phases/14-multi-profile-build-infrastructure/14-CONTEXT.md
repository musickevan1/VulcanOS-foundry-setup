# Phase 14: Multi-Profile Build Infrastructure — Context

## Overview

This document captures implementation decisions for restructuring the build system to support multiple archiso profiles (T2 and Foundry) from a shared base.

---

## Directory Structure

### Decision: Profiles inside archiso/

Profiles live at `archiso/profiles/` — nested inside the archiso directory.

```
archiso/
├── base/                    # Shared content (Claude decides structure)
├── profiles/
│   ├── t2/                  # T2 MacBook profile
│   └── foundry/             # Generic workstation profile
├── efiboot/                 # Shared boot config (if applicable)
├── grub/                    # Shared GRUB config (if applicable)
└── ...
```

### Decision: Current archiso/ becomes T2 profile

The existing archiso/ structure is extracted into the T2 profile. Shared parts move to base/. This is a migration, not a from-scratch rebuild.

### Decision: Airootfs overlay strategy — Claude decides

Claude will choose the cleanest approach for managing base vs. profile-specific airootfs content. Options include:
- Merge directories (base + profile, profile wins on conflict)
- Profile-only complete airootfs
- Other approaches that minimize duplication

**Constraint:** Whatever approach is chosen must be documented in the implementation.

---

## CLI Interface

### Decision: Separate build scripts per profile

```bash
./scripts/build-t2.sh       # Build T2 ISO
./scripts/build-foundry.sh  # Build Foundry ISO
```

No unified `build.sh --profile=x` script. Simple, explicit invocation.

### Decision: Old build.sh shows error with guidance

After restructuring, running `./scripts/build.sh` should:
1. Exit with non-zero status
2. Print message: "Use build-t2.sh or build-foundry.sh"

### Decision: Fail-fast validation

Build scripts validate that required profile-specific resources exist before invoking mkarchiso:
- Check profile directory exists
- Check package lists exist
- Check required configs present
- Fail early with clear error messages

### Decision: prepare.sh and test-iso.sh — Claude decides

Claude will determine the most practical approach for these scripts:
- **prepare.sh**: May stay unified, split per profile, or take profile argument based on what preparation actually needs to happen per profile
- **test-iso.sh**: May auto-detect most recent ISO, take profile argument, or split — based on typical testing workflow

### Decision: ISO output location — Claude decides

Claude will choose between:
- Subdirectories: `out/t2/`, `out/foundry/`
- Flat with profile in filename: `out/vulcanos-t2-YYYY.MM.DD.iso`

Based on typical archiso workflows and what works best with existing tooling.

### Decision: build-all.sh — Claude decides

Claude will determine if a unified build-all script is needed based on practical use cases (CI/CD, release builds, etc.).

---

## Package Organization

### Decision: Package file structure — Claude decides

Claude will choose the cleanest approach:
- Single merged file per profile (build merges base + profile at build time)
- Separate explicit files (base/packages.x86_64 + profiles/t2/packages.x86_64)

**Constraint:** Whatever approach is chosen must be easy to maintain and understand.

### Decision: Package exclusions — Claude decides

Claude will determine if profiles need the ability to exclude base packages, based on practical needs during implementation.

### Decision: T2-specific packages — Claude decides

Claude will organize T2-specific packages (linux-t2, apple-bcm-firmware, apple-t2-audio-config, t2fanrd, tiny-dfr) for clarity and maintainability. These are clearly T2-only and should be obviously separate from shared packages.

### Decision: Foundry includes GPU/ML stack

The Foundry profile includes GPU and ML packages in the ISO:
- NVIDIA drivers (nvidia, nvidia-utils)
- AMD drivers (mesa, vulkan-radeon, etc.)
- CUDA toolkit
- ROCm (if practical for ISO size)
- ML frameworks or dependencies

**Rationale:** Foundry is an AI workstation profile — users expect GPU acceleration out of the box.

---

## Profile Inheritance

### Decision: Separate pacman.conf per profile

Each profile has its own complete `pacman.conf`:
- **T2**: Includes arch-mact2 repository
- **Foundry**: Standard repos only (no T2-specific repos)

No snippet-merging or includes — each profile's pacman.conf is self-contained.

### Decision: One profiledef.sh per profile

Each profile has its own `profiledef.sh` with:
- Profile-specific ISO filename (e.g., `vulcanos-t2-...`, `vulcanos-foundry-...`)
- Profile-specific metadata

### Decision: Profile-branded boot menus

GRUB boot entries are branded per profile:
- T2: "VulcanOS T2" or similar
- Foundry: "VulcanOS Foundry" or similar

Users should immediately know which ISO they booted.

### Decision: Shared dotfiles with profile overrides

Desktop configuration (hypr, waybar, kitty, etc.) follows:
1. **Shared base**: Same dotfiles for both profiles (consistent VulcanOS experience)
2. **Profile overrides**: Profiles can override specific files when needed

Example use case: T2 might have different monitor.conf defaults, Foundry might have GPU-specific hyprland settings.

---

## Deferred Ideas

*(Items mentioned during discussion that belong in other phases)*

- Install process and scripts → Phase 15
- First-boot configuration → Phase 15

---

## Implementation Notes

This phase restructures the build system. After completion:
- `./scripts/build-t2.sh` produces a bootable T2 MacBook ISO
- `./scripts/build-foundry.sh` produces a bootable generic workstation ISO
- Both ISOs share the VulcanOS desktop experience
- Profile-specific packages and configs are cleanly separated

The installer (Phase 15) handles installation FROM these ISOs to disk.
