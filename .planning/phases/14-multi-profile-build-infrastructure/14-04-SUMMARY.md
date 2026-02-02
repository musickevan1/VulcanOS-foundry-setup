---
phase: 14
plan: 04
subsystem: build-system
tags: [archiso, foundry, configuration, boot, nvidia]
requires: [14-01, 14-02, 14-03]
provides:
  - Foundry pacman.conf (standard repos, no arch-mact2)
  - Foundry profiledef.sh (vulcanos-foundry ISO naming)
  - Foundry GRUB config with NVIDIA kernel params
  - Foundry SYSLINUX config with NVIDIA kernel params
  - Nouveau fallback boot option
affects: [14-05, 14-06]
tech-stack:
  added: []
  patterns: [profile-specific-configs, kernel-parameter-tuning]
key-files:
  created:
    - archiso/profiles/foundry/pacman.conf
    - archiso/profiles/foundry/profiledef.sh
    - archiso/profiles/foundry/grub/grub.cfg
    - archiso/profiles/foundry/syslinux/syslinux.cfg
  modified: []
decisions:
  - id: foundry-standard-repos
    choice: Standard Arch repos only (core, extra, multilib)
    rationale: Foundry is generic AI workstation, not T2-specific
  - id: foundry-iso-naming
    choice: vulcanos-foundry-YYYY.MM.DD-x86_64.iso
    rationale: Clear differentiation from T2 ISO
  - id: nvidia-kernel-params
    choice: nvidia-drm.modeset=1 for NVIDIA GPUs
    rationale: Required for proper NVIDIA driver initialization
  - id: nouveau-fallback
    choice: Provide Nouveau boot option
    rationale: Compatibility fallback if NVIDIA drivers fail
metrics:
  tasks: 3
  commits: 3
  files-created: 11
  duration: 164s
  completed: 2026-02-02
---

# Phase 14 Plan 04: Create Foundry Profile Configuration Files Summary

**One-liner:** Foundry profile configs with NVIDIA kernel params, standard repos, and Nouveau fallback

## What Was Built

Created Foundry-specific configuration files for AI workstation profile:

1. **pacman.conf** - Standard Arch repositories (NO arch-mact2)
   - [core], [extra], [multilib] sections
   - multilib included for Steam and 32-bit NVIDIA libraries
   - Comment explains this is for generic/AI workstation

2. **profiledef.sh** - Foundry ISO metadata
   - iso_name="vulcanos-foundry" → produces vulcanos-foundry-YYYY.MM.DD-x86_64.iso
   - iso_label contains FDRY identifier (abbreviated for filesystem limits)
   - iso_application mentions "AI Workstation"

3. **Boot configurations** - GRUB and SYSLINUX
   - Branded "VulcanOS Foundry" (not "T2 MacBook Pro")
   - NVIDIA kernel parameter: `nvidia-drm.modeset=1`
   - NO T2 parameters (intel_iommu, iommu, pcie_ports)
   - Nouveau fallback option (blacklists NVIDIA modules for compatibility)
   - Copied theme files and splash.png from base

## Key Implementation Details

### Package Manager Configuration

The Foundry pacman.conf uses standard Arch repositories ONLY:
- NO arch-mact2 repository (T2-specific packages excluded)
- multilib enabled for 32-bit support (Steam, NVIDIA 32-bit libs)
- Standard repo order: core → extra → multilib

### ISO Naming Strategy

- **iso_name:** `vulcanos-foundry`
- **iso_label:** `VULCAN_FDRY_YYYYMM` (FDRY abbreviated due to filesystem label length limits)
- **iso_application:** "VulcanOS Foundry - AI Workstation Live/Install Medium"
- **Resulting ISO:** `vulcanos-foundry-2026.02.02-x86_64.iso`

### Kernel Parameters

**NVIDIA boot option:**
```
nvidia-drm.modeset=1
```

**Nouveau fallback option:**
```
modprobe.blacklist=nvidia,nvidia_drm,nvidia_modeset,nvidia_uvm nomodeset
```

**T2 params REMOVED** (these were in T2 config but are NOT in Foundry):
```
intel_iommu=on iommu=pt pcie_ports=compat
```

### Boot Menu Branding

All boot menu entries branded "VulcanOS Foundry" instead of "VulcanOS T2 MacBook Pro":
- GRUB: "VulcanOS Foundry (NVIDIA)"
- GRUB: "VulcanOS Foundry (Nouveau - open source fallback)"
- SYSLINUX: "VulcanOS Foundry Boot Menu"

## Deviations from Plan

None - plan executed exactly as written.

## Decisions Made

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Repository configuration | Standard Arch repos only | Foundry is generic workstation, not T2-specific |
| ISO naming | vulcanos-foundry | Clear differentiation from T2 ISO |
| Label abbreviation | FDRY | Filesystem label length limits |
| NVIDIA kernel param | nvidia-drm.modeset=1 | Required for proper NVIDIA driver initialization |
| Fallback option | Nouveau with NVIDIA blacklist | Compatibility if proprietary driver fails |
| Theme reuse | Copy from base archiso/ | Consistent VulcanOS branding across profiles |

## Files Created/Modified

### Created (11 files)
- `archiso/profiles/foundry/pacman.conf` - Package manager config (30 lines)
- `archiso/profiles/foundry/profiledef.sh` - ISO metadata (35 lines)
- `archiso/profiles/foundry/grub/grub.cfg` - GRUB boot menu (66 lines)
- `archiso/profiles/foundry/syslinux/syslinux.cfg` - SYSLINUX boot menu (47 lines)
- `archiso/profiles/foundry/grub/themes/vulcanos/` - Theme files (6 files)
- `archiso/profiles/foundry/syslinux/splash.png` - Boot splash

### Modified
None

## Testing Evidence

All verification criteria passed:

```bash
✓ NO [arch-mact2] repo section in pacman.conf
✓ core and multilib repos present
✓ iso_name is vulcanos-foundry in profiledef.sh
✓ AI Workstation mentioned in profiledef.sh
✓ FDRY identifier in iso_label
✓ GRUB branded VulcanOS Foundry
✓ GRUB has nvidia-drm.modeset=1
✓ GRUB has NO T2 params
✓ GRUB has Nouveau fallback
✓ SYSLINUX branded VulcanOS Foundry
✓ GRUB themes directory exists
✓ SYSLINUX splash.png exists
```

## Next Phase Readiness

**Ready for Phase 14 Plan 05** (Create T2 airootfs overlay)

### What's Available
- Complete Foundry profile configuration
- NVIDIA-optimized boot parameters
- Nouveau fallback for compatibility
- Standard Arch repository configuration

### Blockers/Concerns
None

### Dependencies Satisfied
- Phase 14 Plan 01: Build library created
- Phase 14 Plan 02: Package lists defined
- Phase 14 Plan 03: T2 profile configs created

## Commits

| Hash | Message | Files |
|------|---------|-------|
| 5e7fdc8 | feat(14-04): create Foundry pacman.conf (standard repos only) | archiso/profiles/foundry/pacman.conf |
| abb5953 | feat(14-04): create Foundry profiledef.sh for ISO naming | archiso/profiles/foundry/profiledef.sh |
| 376b3b3 | feat(14-04): create Foundry boot configs with NVIDIA params | archiso/profiles/foundry/grub/, archiso/profiles/foundry/syslinux/ |

## Lessons Learned

1. **Filesystem label limits** - ISO labels have length restrictions, requiring abbreviations (FDRY instead of FOUNDRY)

2. **NVIDIA fallback essential** - Nouveau fallback option provides compatibility path if proprietary drivers fail

3. **Kernel parameter divergence** - T2 and Foundry profiles have completely different kernel parameter requirements (T2: IOMMU, Foundry: NVIDIA)

4. **Theme reuse** - Copying theme files instead of symlinking ensures profile independence

## Related Documentation

- 14-CONTEXT.md: Multi-profile build infrastructure decisions
- 14-RESEARCH.md: archiso patterns and profile structure
- archiso/profiles/t2/: T2 profile for comparison
- archiso/grub/: Base GRUB configs (source for themes)

## Future Considerations

1. **NVIDIA driver version** - May need profile-specific driver versioning based on GPU generation
2. **AMD GPU support** - Future Foundry builds might need AMD-specific boot options
3. **Multi-GPU scenarios** - Consider boot options for systems with both integrated and discrete GPUs
4. **CUDA version pinning** - May need mechanism to specify CUDA version in ISO metadata
