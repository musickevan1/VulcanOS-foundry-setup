# v3.0 Multi-Profile + AI Workstation Research Summary

**Project:** VulcanOS v3.0
**Domain:** NVIDIA AI Workstation, Multi-Profile Distribution, Gaming
**Researched:** 2026-02-02
**Confidence:** HIGH

## Executive Summary

VulcanOS v3.0 represents a significant architectural expansion from a single T2 MacBook Pro distribution to a multi-profile system supporting both the original T2 hardware and a new "Vulcan Foundry" AI workstation profile targeting AMD Ryzen 9 9950X with NVIDIA RTX 5070 Ti. The research reveals that archiso has **no native multi-profile support**, requiring a custom base+overlay assembly system where shared components live in `profiles/base/` and hardware-specific packages/configs are merged at build time. This is a clean, maintainable approach that minimizes duplication while keeping profile differences explicit.

The RTX 5070 Ti (Blackwell architecture, sm_120 compute capability) presents unique challenges: **proprietary NVIDIA drivers are officially unsupported** on Blackwell GPUs, mandating `nvidia-open-dkms`. Additionally, stable PyTorch and TensorFlow releases do not support sm_120, requiring nightly builds with CUDA 12.8+. A documented PCIe Gen1 fallback bug affects some motherboards. These constraints dictate strict phase ordering: driver foundation must be verified before any CUDA/AI work begins.

The recommended approach builds the multi-profile infrastructure first (all other features depend on it), then establishes the NVIDIA driver foundation with extensive validation, followed by CUDA/AI stack integration, desktop polish, and finally gaming support. Existing VulcanOS strengths (keyboard-driven workflow, unified theming, MCP servers) become differentiators when combined with AI workstation capabilities. The full scope is estimated at 8-12 days of implementation effort.

## Key Findings

### Multi-Profile Architecture

archiso processes one profile directory per `mkarchiso` invocation with no include/import mechanism for packages or configs. The recommended **base+overlay pattern** structures the codebase as:

```
profiles/
├── base/           # ~180 shared packages, common configs
├── foundry/        # NVIDIA packages, mainline kernel
├── t2/             # T2 packages, apple-bce configs
└── _build/         # Build-time assembly (gitignored)
```

**Key components:**
- `assemble-profile.sh` — New script that merges base + profile-specific components
- `packages.base` + `packages.profile` — Concatenated at build time
- `pacman.profile.conf` — Profile repos prepended for priority
- `grub.profile.cfg` — Profile-specific kernel parameters substituted

**Migration path:** Create new profiles/ structure, keep archiso/ working during transition, validate both profiles, then deprecate old structure.

### NVIDIA/CUDA Stack

The RTX 5070 Ti requires specific driver and framework considerations:

**Core technologies:**
- `nvidia-open-dkms` (590.48.01+): **MANDATORY** for Blackwell — proprietary drivers unsupported
- `cuda` (13.1.1): System toolkit from Arch extra repository
- `cudnn` (9.14): Deep learning primitives integration
- `nvidia-container-toolkit`: GPU passthrough for Docker containers
- PyTorch nightly (`cu128`): Stable releases do NOT support sm_120
- `ollama-cuda`: Local LLM inference (official repo, not AUR)

**Critical requirements:**
- Kernel parameter: `nvidia-drm.modeset=1`
- mkinitcpio MODULES: `nvidia nvidia_modeset nvidia_uvm nvidia_drm`
- Pacman hook for initramfs rebuild on driver updates
- CUDA path symlink: `/usr/local/cuda -> /opt/cuda` (for JAX compatibility)

**Gaming stack** (all in official repos):
- `steam`, `lib32-nvidia-utils` (requires multilib)
- `mangohud`, `lib32-mangohud`, `gamemode`, `lib32-gamemode`
- `gamescope` (micro-compositor for XWayland games)
- `proton-ge-custom-bin` (AUR)

### Feature Landscape

**Must have (table stakes):**
- NVIDIA proprietary drivers with CUDA support
- PyTorch/TensorFlow with GPU acceleration
- Local LLM inference (Ollama)
- Steam + Proton for gaming
- Multi-monitor Hyprland configuration

**Should have (differentiators):**
- Keyboard-driven AI workflow integration (Super+A for AI chat)
- Unified theming extending to AI tools (Open WebUI, Jupyter)
- GPU monitoring in Waybar
- MCP servers for AI workflow automation
- hyprwhspr voice input for AI coding

**Defer (v2+):**
- vLLM production server
- Custom model training GUI
- Cloud API integrations
- Kubernetes/ML orchestration
- Pre-installed large models (ISO bloat)

### Critical Pitfalls

22 pitfalls documented across 6 categories. Top risks:

1. **Open kernel modules required** (CRITICAL) — Blackwell GPUs won't work with `nvidia-dkms`, only `nvidia-open-dkms`. Symptoms: black screen, no GPU detection. Prevention: Verify driver package before install.

2. **PCIe Gen1 fallback bug** (CRITICAL) — RTX 5070 Ti may negotiate Gen1 (2.5 GT/s) instead of Gen4 (16 GT/s), causing instability and 75% bandwidth loss. Prevention: Enable Above 4G Decoding + Resizable BAR in BIOS, verify with `nvidia-smi -q | grep "Link Gen"`.

3. **sm_120 not supported in stable frameworks** (CRITICAL) — PyTorch stable (2.9.x) only supports up to sm_90. GPU detected but operations fail or fall back to CPU. Prevention: Use PyTorch nightly with CUDA 12.8 index.

4. **Driver/library version mismatch** (HIGH) — After Arch updates, nvidia-smi fails until reboot. Prevention: Pacman hook to rebuild initramfs, always reboot after kernel/nvidia updates.

5. **Suspend/resume crashes Hyprland** (HIGH) — NVIDIA drivers don't preserve video memory without explicit configuration. Prevention: Kernel param `nvidia.NVreg_PreserveVideoMemoryAllocations=1`, enable nvidia-suspend/hibernate/resume services.

6. **Python environment conflicts** (MEDIUM) — PEP 668 prevents system-wide pip installs. Prevention: Always use venvs, recommend Miniconda for ML workloads.

## Implications for Roadmap

Based on research dependencies, the following phase structure is recommended:

### Phase 1: Multi-Profile Build Infrastructure
**Rationale:** All Foundry features depend on the profile system existing. Cannot install NVIDIA packages into T2 profile.
**Delivers:** profiles/ directory structure, assemble-profile.sh, updated build.sh with --profile flag
**Effort:** 2-3 days
**Avoids:** Package conflicts between profiles (Pitfall #20)
**Research needed:** LOW — archiso behavior well-documented

### Phase 2: NVIDIA Driver Foundation
**Rationale:** All AI/ML and gaming features require working GPU drivers. Must validate before CUDA stack.
**Delivers:** Bootable Foundry ISO with nvidia-open-dkms, DRM modeset, pacman hooks
**Effort:** 1-2 days
**Addresses:** Table stakes GPU compute foundation
**Avoids:** Pitfalls #1 (open modules), #2 (PCIe bug), #3 (version mismatch), #6 (DRM), #16 (boot race)
**Research needed:** LOW — NVIDIA docs and Arch Wiki comprehensive

### Phase 3: CUDA/AI Stack
**Rationale:** Once drivers verified, install CUDA toolkit and ML frameworks
**Delivers:** Working PyTorch/TensorFlow, Ollama, nvidia-container-toolkit
**Effort:** 1-2 days
**Uses:** CUDA 13.1.1, PyTorch nightly cu128, ollama-cuda
**Avoids:** Pitfalls #4 (sm_120), #8 (Python envs), #9 (Docker GPU)
**Research needed:** MEDIUM — PyTorch nightly API may change

### Phase 4: Desktop Integration
**Rationale:** Polish Hyprland experience for AI workstation use case
**Delivers:** Suspend/resume fixes, VRAM monitoring, Hyprland plugins, keybindings
**Effort:** 1-2 days
**Implements:** Keyboard-driven AI workflow differentiators
**Avoids:** Pitfalls #5 (suspend), #10 (VRAM leak), #12 (HDMI color)
**Research needed:** LOW — Hyprland NVIDIA wiki comprehensive

### Phase 5: Gaming Profile
**Rationale:** Gaming is additive after desktop works; requires 32-bit libs and gamescope
**Delivers:** Steam + Proton, MangoHud, GameMode, gamescope wrapper
**Effort:** 1 day
**Addresses:** Gaming table stakes
**Avoids:** Pitfalls #7 (XWayland flicker), #11 (Proton stuck), #21 (anti-cheat)
**Research needed:** LOW — well-documented on Arch Wiki

### Phase 6: AI Tools Integration (Optional)
**Rationale:** Advanced features once core is stable
**Delivers:** Open WebUI, ComfyUI setup scripts, Jupyter GPU integration, themed AI tools
**Effort:** 2-3 days
**Implements:** Differentiator features
**Research needed:** MEDIUM — ComfyUI sm_120 support not widely tested

### Phase Ordering Rationale

1. **Multi-profile first** — Cannot implement Foundry-specific features without profile system
2. **Drivers before CUDA** — CUDA installation validates against driver; bad driver = debugging nightmare
3. **CUDA before AI tools** — PyTorch/Ollama require working CUDA
4. **Desktop before gaming** — Gaming builds on working Wayland + NVIDIA; gamescope is a compositor
5. **AI tools last** — Optional polish; core workstation functional without them

### Research Flags

**Phases likely needing deeper research during planning:**
- **Phase 3 (CUDA/AI Stack):** PyTorch nightly builds change frequently; verify sm_120 support at implementation time
- **Phase 6 (AI Tools):** ComfyUI + RTX 5070 Ti has sparse real-world reports

**Phases with standard patterns (skip research-phase):**
- **Phase 1 (Multi-Profile):** archiso behavior confirmed via source code examination
- **Phase 2 (Drivers):** NVIDIA + Arch Wiki documentation is comprehensive
- **Phase 5 (Gaming):** Steam/Proton is mature and well-documented

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Official NVIDIA docs, Arch Wiki, verified package versions |
| Features | MEDIUM-HIGH | Table stakes validated, differentiators are design decisions |
| Architecture | MEDIUM | Custom design (base+overlay), not battle-tested but logical |
| Pitfalls | HIGH | Verified against official docs, GitHub issues, community reports |

**Overall confidence:** HIGH

### Gaps to Address

- **PyTorch sm_120 timeline:** Nightly today, but when will stable support land? Monitor pytorch/pytorch#151376
- **PCIe bug resolution:** Verify if driver updates fix Gen1 fallback; may need BIOS workarounds
- **ComfyUI on Blackwell:** Limited real-world reports; may encounter sentencepiece wheel bug
- **Multi-profile testing:** Recommended architecture not yet validated with actual builds
- **Ollama VRAM usage:** 16GB sufficient for most models, but larger models (70B) may need offloading

## Sources

### Primary (HIGH confidence)
- [NVIDIA Driver Documentation](https://developer.nvidia.com/blog/nvidia-transitions-fully-towards-open-source-gpu-kernel-modules/) — Open kernel module requirements
- [Arch Wiki - NVIDIA](https://wiki.archlinux.org/title/NVIDIA) — Driver installation, DRM, pacman hooks
- [Hyprland NVIDIA Wiki](https://wiki.hypr.land/Nvidia/) — Wayland compositor integration
- [archiso GitLab](https://github.com/archlinux/archiso) — Profile structure, mkarchiso behavior
- [CUDA Installation Guide](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/) — Toolkit installation

### Secondary (MEDIUM confidence)
- [RTX 5070 Ti Linux Guide](https://medium.com/@mbonsign/complete-guide-for-nvidia-rtx-5070-ti-on-linux-with-pytorch-358454521f04) — Community setup guide
- [PyTorch Forums sm_120](https://discuss.pytorch.org/t/nvidia-geforce-rtx-5070-ti-with-cuda-capability-sm-120/221509) — Nightly build requirements
- [PCIe Gen1 Bug Issue](https://github.com/NVIDIA/open-gpu-kernel-modules/issues/1010) — Known Blackwell issue
- [Open WebUI Documentation](https://docs.openwebui.com/) — Self-hosted AI chat interface

### Tertiary (LOW confidence)
- ComfyUI performance on RTX 5070 Ti — Limited reports, needs validation
- Multi-monitor gaming with MangoHud — Community reports vary
- Specific Ollama model VRAM usage — Varies by model and quantization

---
*Research completed: 2026-02-02*
*Ready for roadmap: YES*
