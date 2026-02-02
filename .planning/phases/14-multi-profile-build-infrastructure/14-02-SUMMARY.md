---
phase: 14-multi-profile-build-infrastructure
plan: 02
subsystem: infra
tags: [archiso, packages, multi-profile, build-system]

# Dependency graph
requires:
  - phase: 14-01
    provides: Research and architecture for multi-profile builds
provides:
  - Base packages list (106 packages shared across profiles)
  - T2 profile packages list (6 T2-specific packages)
  - Foundry profile packages list (10 GPU/ML packages)
  - Clear separation between shared and hardware-specific packages
affects: [14-03-airootfs-restructure, 14-04-profile-configs, 14-05-assembly-scripts]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "No kernel in base packages - profiles specify their own"
    - "Hardware-specific packages isolated to profile lists"
    - "Package lists concatenated at build time"

key-files:
  created:
    - archiso/base/packages.base
    - archiso/profiles/t2/packages.profile
    - archiso/profiles/foundry/packages.profile
  modified: []

key-decisions:
  - "No kernel in base - avoids conflicts between linux and linux-t2"
  - "Removed duplicate wl-clipboard (appeared twice in original)"
  - "NVIDIA open-source drivers (nvidia-open-dkms) for RTX 50 series support"
  - "CUDA/ML packages in Foundry profile for AI workstation capabilities"
  - "Speech-to-text (hyprwhspr) in base - both profiles benefit"

patterns-established:
  - "Package split: base (shared desktop/dev) + profile (kernel/hardware)"
  - "Clear section headers for maintainability"
  - "Comments documenting hardware requirements and dependencies"

# Metrics
duration: 1min 29sec
completed: 2026-02-02
---

# Phase 14 Plan 02: Package Split Summary

**106 shared packages in base, 6 T2-specific packages, 10 Foundry GPU/ML packages with clear kernel separation**

## Performance

- **Duration:** 1 min 29 sec
- **Started:** 2026-02-02T22:42:12Z
- **Completed:** 2026-02-02T22:43:41Z
- **Tasks:** 3
- **Files created:** 3

## Accomplishments
- Created shared base packages list (106 packages: desktop, dev tools, CLI utilities, fonts, theming)
- Created T2 profile packages list (linux-t2 kernel + 5 T2 hardware packages)
- Created Foundry profile packages list (generic kernel + NVIDIA/CUDA stack for RTX 5070 Ti)
- Removed duplicate wl-clipboard entry from original list
- Ensured NO kernel in base to prevent conflicts

## Task Commits

Each task was committed atomically:

1. **Task 1: Create base packages list** - `cd4a0f4` (feat)
2. **Task 2: Create T2 profile packages** - `713efda` (feat)
3. **Task 3: Create Foundry profile packages** - `7cf6945` (feat)

## Files Created/Modified

### Created
- `archiso/base/packages.base` - 106 shared packages (Hyprland desktop, dev tools, CLI utils)
- `archiso/profiles/t2/packages.profile` - 6 T2-specific packages (linux-t2, Apple firmware/drivers)
- `archiso/profiles/foundry/packages.profile` - 10 Foundry packages (linux, NVIDIA drivers, CUDA/ML)

## Package Distribution

**Base (106 packages):**
- Base system: base, linux-firmware
- Bootloader: grub, efibootmgr, syslinux
- Desktop: Hyprland, waybar, SDDM, wofi, swaync
- File management: Nautilus, yazi
- Audio: PipeWire stack
- Development: Docker, Git, LSPs (8 language servers)
- CLI: ripgrep, fd, fzf, bat, btop, starship
- Theming: nwg-look, kvantum, qt5ct/qt6ct
- Fonts: JetBrains Mono Nerd, Noto
- Productivity: LibreOffice, Obsidian
- Speech-to-text: hyprwhspr

**T2 Profile (6 packages):**
- Kernel: linux-t2, linux-t2-headers
- Hardware: apple-bcm-firmware (WiFi), apple-t2-audio-config (audio), t2fanrd (fans), tiny-dfr (Touch Bar)

**Foundry Profile (10 packages):**
- Kernel: linux, linux-headers (generic)
- GPU: nvidia-open-dkms, nvidia-utils, nvidia-settings, lib32-nvidia-utils
- ML: cuda, cudnn, nvidia-container-toolkit

## Decisions Made

1. **No kernel in base**: Avoids package conflicts between `linux` and `linux-t2`. Each profile specifies its own kernel.

2. **nvidia-open-dkms over nvidia-dkms**: RTX 50 series (Blackwell architecture, sm_120) requires open-source NVIDIA drivers. Standard proprietary drivers don't support Blackwell yet.

3. **CUDA 12.8+ requirement**: Documented in Foundry comments that CUDA 12.8+ is required for Blackwell support. PyTorch stable doesn't support sm_120 - nightly builds required (post-install).

4. **Removed duplicate wl-clipboard**: Original packages.x86_64 listed wl-clipboard twice (lines 80 and 196). Consolidated to single entry in base.

5. **Speech-to-text in base**: hyprwhspr provides local Whisper-based transcription. Both T2 and Foundry profiles benefit from this feature.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - package splitting was straightforward based on RESEARCH.md analysis.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

**Ready for:**
- Phase 14-03: airootfs restructure (split into base/ and profiles/)
- Phase 14-04: Profile-specific configs (pacman.conf, profiledef.sh, boot configs)
- Phase 14-05: Assembly scripts (build-t2.sh, build-foundry.sh, lib/build-common.sh)

**Package lists validated:**
- Base contains all shared packages (desktop, dev, CLI)
- T2 profile contains T2-specific kernel and hardware support
- Foundry profile contains generic kernel and NVIDIA/CUDA stack
- No conflicts between base and profile packages
- Combined base + T2 = original T2 functionality
- Combined base + Foundry = AI workstation capabilities

**Architecture decisions:**
- Kernel separation confirmed working (no kernel in base)
- Profile-specific packages cleanly isolated
- Ready for build-time concatenation

**Hardware considerations documented:**
- RTX 5070 Ti requirements explicitly noted (nvidia-open-dkms, CUDA 12.8+)
- PyTorch nightly requirement documented for post-install
- T2 hardware packages remain unchanged from working configuration

---
*Phase: 14-multi-profile-build-infrastructure*
*Completed: 2026-02-02*
