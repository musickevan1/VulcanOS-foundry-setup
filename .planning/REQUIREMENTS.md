# Requirements: VulcanOS v3.0

**Defined:** 2026-02-02
**Core Value:** Cohesive, recoverable, keyboard-driven

## v3.0 Requirements

Requirements for VulcanOS v3.0 Multi-Profile + AI Workstation. Each maps to roadmap phases.

### Multi-Profile Build System

- [ ] **PROF-01**: Build system supports multiple archiso profiles (foundry, t2)
- [ ] **PROF-02**: Shared base packages extracted to `profiles/base/packages.base`
- [ ] **PROF-03**: Profile-specific packages in `profiles/{profile}/packages.profile`
- [ ] **PROF-04**: Config overlay system merges base + profile airootfs
- [ ] **PROF-05**: `build.sh --profile=<name>` builds specified profile
- [ ] **PROF-06**: assemble-profile.sh merges components before mkarchiso
- [ ] **PROF-07**: Output ISOs named `vulcanos-{profile}-{date}-x86_64.iso`

### NVIDIA Driver Foundation

- [ ] **DRV-01**: Foundry profile installs `nvidia-open-dkms` (required for Blackwell)
- [ ] **DRV-02**: DRM kernel mode setting enabled (`nvidia_drm.modeset=1`)
- [ ] **DRV-03**: NVIDIA modules in mkinitcpio MODULES array for early loading
- [ ] **DRV-04**: Pacman hook rebuilds initramfs on driver updates
- [ ] **DRV-05**: NVIDIA power services enabled (suspend, hibernate, resume)
- [ ] **DRV-06**: 32-bit NVIDIA libs installed (`lib32-nvidia-utils`) for gaming

### CUDA/AI Stack

- [ ] **CUDA-01**: CUDA toolkit installed from official repos
- [ ] **CUDA-02**: cuDNN installed for deep learning
- [ ] **CUDA-03**: nvidia-container-toolkit configured for Docker GPU access
- [ ] **CUDA-04**: CUDA environment variables set in Hyprland config
- [ ] **CUDA-05**: PyTorch installation documented (nightly for sm_120)
- [ ] **CUDA-06**: JAX installation documented with CUDA path workaround

### Local LLM Infrastructure

- [ ] **LLM-01**: Ollama (CUDA version) installed from official repos
- [ ] **LLM-02**: Ollama service enabled for user
- [ ] **LLM-03**: llama.cpp-cuda available (AUR or custom build documented)
- [ ] **LLM-04**: Open WebUI deployment documented (Docker or native)

### Image Generation

- [ ] **IMG-01**: ComfyUI installation documented (Python venv workflow)
- [ ] **IMG-02**: PyTorch nightly with CUDA 12.8 documented for sm_120 support
- [ ] **IMG-03**: Model storage location standardized (`~/.local/share/comfyui/`)

### Gaming Stack

- [ ] **GAME-01**: Multilib repository enabled in pacman.conf
- [ ] **GAME-02**: Steam installed from multilib
- [ ] **GAME-03**: Proton-GE available (AUR package)
- [ ] **GAME-04**: MangoHud and lib32-mangohud installed
- [ ] **GAME-05**: gamemode and lib32-gamemode installed
- [ ] **GAME-06**: gamescope installed for XWayland game wrapper
- [ ] **GAME-07**: Launch options documented (gamemoderun, MangoHud, gamescope)

### Desktop Improvements

- [ ] **DESK-01**: Kitty replaces Alacritty as default terminal
- [ ] **DESK-02**: yazi installed as primary file manager
- [ ] **DESK-03**: Thunar retained as GUI fallback file manager
- [ ] **DESK-04**: yazi shell integration in bashrc/zshrc
- [ ] **DESK-05**: Hyprland keybind updated for Kitty
- [ ] **DESK-06**: Alacritty removed from package list

### Hyprland Plugins

- [ ] **PLUG-01**: hyprpm (plugin manager) available
- [ ] **PLUG-02**: hyprexpo plugin configured (workspace overview)
- [ ] **PLUG-03**: hyprspace plugin configured (workspace switcher)
- [ ] **PLUG-04**: hyprtrails plugin configured (window trails)
- [ ] **PLUG-05**: Plugin autoload configured (`exec-once = hyprpm reload`)

### Suspend/Resume Fixes

- [ ] **SUSP-01**: NVIDIA preserve video memory enabled in kernel params
- [ ] **SUSP-02**: nvidia-suspend.service enabled
- [ ] **SUSP-03**: nvidia-hibernate.service enabled
- [ ] **SUSP-04**: nvidia-resume.service enabled

### Waybar Enhancements

- [ ] **BAR-01**: GPU VRAM usage module added to Waybar
- [ ] **BAR-02**: GPU temperature module added to Waybar
- [ ] **BAR-03**: Modules configurable via Waybar config

### T2 Profile Maintenance

- [ ] **T2-01**: T2 profile continues building with linux-t2 kernel
- [ ] **T2-02**: T2-specific packages isolated to `profiles/t2/packages.profile`
- [ ] **T2-03**: arch-mact2 repo in T2-specific pacman.conf
- [ ] **T2-04**: T2 kernel params in T2-specific grub config
- [ ] **T2-05**: apple-bce, apple-gmux modprobe configs in T2 airootfs

### Foundry-T2 Sync & Remote Access

- [ ] **SYNC-01**: Syncthing installed on both profiles for project sync
- [ ] **SYNC-02**: Default Syncthing folder configured for `~/Projects`
- [ ] **SYNC-03**: Syncthing GUI accessible via localhost
- [ ] **SYNC-04**: Syncthing autostart via systemd user service
- [ ] **REM-01**: SSH server (sshd) enabled on Foundry profile
- [ ] **REM-02**: Wake-on-LAN configured on Foundry (ethtool, systemd service)
- [ ] **REM-03**: Foundry auto-login to desktop session (for remote desktop access)
- [ ] **REM-04**: VNC server (wayvnc) installed on Foundry for remote desktop
- [ ] **REM-05**: Ollama API exposed on LAN (`OLLAMA_HOST=0.0.0.0`)
- [ ] **REM-06**: T2 can use Foundry's Ollama as remote backend
- [ ] **REM-07**: vulcan-foundry-wake script on T2 to wake Foundry via WoL
- [ ] **REM-08**: Idle suspend configured on Foundry (suspend after X minutes inactive)

### Profile-Aware Dotfiles

- [ ] **DOT-01**: Hostname detection in Hyprland config (`$HOSTNAME` variable)
- [ ] **DOT-02**: Profile-specific monitor configs (foundry triple+, t2 laptop)
- [ ] **DOT-03**: Profile-specific NVIDIA env vars (Foundry only)
- [ ] **DOT-04**: Shared keybindings, theming, and app configs
- [ ] **DOT-05**: Git-based dotfile sync (VulcanOS repo, not Syncthing)

## Future Requirements

Deferred to v3.1 or later. Tracked but not in current roadmap.

### Open WebUI Native

- **WEBUI-01**: Open WebUI installed as native package (not Docker)
- **WEBUI-02**: Open WebUI themed to match VulcanOS

### Keybindings

- **KEY-01**: `Super+A` opens local AI chat (Open WebUI)
- **KEY-02**: Quick model switch via wofi launcher

### Additional AI Tools

- **AI-01**: JupyterLab with GPU verification script
- **AI-02**: nvitop/nvtop GPU process monitoring
- **AI-03**: Pre-configured docker-compose templates for ML stacks

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| vLLM production server | Overkill for personal workstation; Ollama sufficient |
| Cloud API integrations | Privacy-focused local-first philosophy |
| Custom model training GUI | Better tools exist (MLflow, W&B) |
| AMD ROCm support | Foundry has NVIDIA, T2 has no dGPU |
| Kubernetes/orchestration | Enterprise feature, Docker Compose sufficient |
| Pre-installed large models | Storage bloat; user downloads via Ollama |
| Automatic GPU switching | Single-GPU system (Foundry), complexity not needed |
| Windows dual-boot helper | VulcanOS is primary OS; document if asked |
| Different branding per profile | Unified VulcanOS look across profiles |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PROF-01 | Phase 14 | Pending |
| PROF-02 | Phase 14 | Pending |
| PROF-03 | Phase 14 | Pending |
| PROF-04 | Phase 14 | Pending |
| PROF-05 | Phase 14 | Pending |
| PROF-06 | Phase 14 | Pending |
| PROF-07 | Phase 14 | Pending |
| DRV-01 | Phase 15 | Pending |
| DRV-02 | Phase 15 | Pending |
| DRV-03 | Phase 15 | Pending |
| DRV-04 | Phase 15 | Pending |
| DRV-05 | Phase 15 | Pending |
| DRV-06 | Phase 15 | Pending |
| CUDA-01 | Phase 16 | Pending |
| CUDA-02 | Phase 16 | Pending |
| CUDA-03 | Phase 16 | Pending |
| CUDA-04 | Phase 16 | Pending |
| CUDA-05 | Phase 16 | Pending |
| CUDA-06 | Phase 16 | Pending |
| LLM-01 | Phase 16 | Pending |
| LLM-02 | Phase 16 | Pending |
| LLM-03 | Phase 16 | Pending |
| LLM-04 | Phase 16 | Pending |
| IMG-01 | Phase 16 | Pending |
| IMG-02 | Phase 16 | Pending |
| IMG-03 | Phase 16 | Pending |
| GAME-01 | Phase 17 | Pending |
| GAME-02 | Phase 17 | Pending |
| GAME-03 | Phase 17 | Pending |
| GAME-04 | Phase 17 | Pending |
| GAME-05 | Phase 17 | Pending |
| GAME-06 | Phase 17 | Pending |
| GAME-07 | Phase 17 | Pending |
| DESK-01 | Phase 18 | Pending |
| DESK-02 | Phase 18 | Pending |
| DESK-03 | Phase 18 | Pending |
| DESK-04 | Phase 18 | Pending |
| DESK-05 | Phase 18 | Pending |
| DESK-06 | Phase 18 | Pending |
| PLUG-01 | Phase 19 | Pending |
| PLUG-02 | Phase 19 | Pending |
| PLUG-03 | Phase 19 | Pending |
| PLUG-04 | Phase 19 | Pending |
| PLUG-05 | Phase 19 | Pending |
| SUSP-01 | Phase 15 | Pending |
| SUSP-02 | Phase 15 | Pending |
| SUSP-03 | Phase 15 | Pending |
| SUSP-04 | Phase 15 | Pending |
| BAR-01 | Phase 19 | Pending |
| BAR-02 | Phase 19 | Pending |
| BAR-03 | Phase 19 | Pending |
| T2-01 | Phase 20 | Pending |
| T2-02 | Phase 20 | Pending |
| T2-03 | Phase 20 | Pending |
| T2-04 | Phase 20 | Pending |
| T2-05 | Phase 20 | Pending |
| SYNC-01 | Phase 21 | Pending |
| SYNC-02 | Phase 21 | Pending |
| SYNC-03 | Phase 21 | Pending |
| SYNC-04 | Phase 21 | Pending |
| REM-01 | Phase 21 | Pending |
| REM-02 | Phase 21 | Pending |
| REM-03 | Phase 21 | Pending |
| REM-04 | Phase 21 | Pending |
| REM-05 | Phase 21 | Pending |
| REM-06 | Phase 21 | Pending |
| REM-07 | Phase 21 | Pending |
| REM-08 | Phase 21 | Pending |
| DOT-01 | Phase 22 | Pending |
| DOT-02 | Phase 22 | Pending |
| DOT-03 | Phase 22 | Pending |
| DOT-04 | Phase 22 | Pending |
| DOT-05 | Phase 22 | Pending |

**Coverage:**
- v3.0 requirements: 69 total
- Mapped to phases: 69
- Unmapped: 0

---
*Requirements defined: 2026-02-02*
*Last updated: 2026-02-02 — Traceability filled from roadmap*
