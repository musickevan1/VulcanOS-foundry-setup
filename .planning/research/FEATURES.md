# Features Research: VulcanOS v3.0 AI Workstation

**Domain:** Linux AI Workstation + Multi-Profile Distribution + Gaming-Capable Workstation
**Researched:** 2026-02-02
**Target Hardware:** Vulcan Foundry (AMD 9950X, RTX 5070 Ti 16GB, 64GB DDR5, triple+ monitors)
**Confidence:** MEDIUM-HIGH (verified with official NVIDIA docs, Arch Wiki, community patterns)

## Executive Summary

VulcanOS v3.0 targets a significant expansion from the current T2 MacBook Pro distribution to support a dedicated AMD/NVIDIA workstation for AI development, gaming, and professional software development. The key additions are:

1. **Multi-Profile archiso Build System** - Generate different ISOs from shared base (T2 vs Foundry)
2. **Full NVIDIA GPU Stack** - CUDA 12.8+, driver 570+, container toolkit for AI/ML workloads
3. **Local LLM Infrastructure** - Ollama + Open WebUI for private AI chat and inference
4. **Image Generation** - ComfyUI/AUTOMATIC1111 for Stable Diffusion workflows
5. **Gaming Capability** - Steam, Proton, MangoHud, GameMode for Windows game compatibility
6. **Multi-Monitor Excellence** - Triple+ monitor Hyprland configuration with per-workspace bindings

The existing VulcanOS strengths (keyboard-driven workflow, unified theming, developer tools) become differentiators when combined with AI workstation capabilities.

---

## AI Workstation Table Stakes

Features users expect from any Linux AI/ML workstation. Missing = product feels incomplete.

### GPU Compute Foundation

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **NVIDIA Proprietary Drivers** | Required for CUDA, deep learning frameworks | Medium | No | RTX 5070 Ti requires driver 570+ with open kernel modules |
| **CUDA Toolkit** | PyTorch, TensorFlow, all ML frameworks need it | Medium | No | CUDA 12.8+ required for RTX 50 series (sm_120 compute capability) |
| **cuDNN** | Optimized deep learning primitives | Low | No | Bundled with PyTorch, separate install for TensorFlow |
| **NVIDIA Container Toolkit** | GPU passthrough for Docker containers | Low | No | Required for containerized ML workflows |
| **nvidia-smi Access** | GPU monitoring is expected | Low | No | Comes with drivers |

**Source:** [NVIDIA CUDA Installation Guide](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/) | [RTX 5070 Ti Linux Guide](https://medium.com/@mbonsign/complete-guide-for-nvidia-rtx-5070-ti-on-linux-with-pytorch-358454521f04)

### Model Training & ML Development

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **PyTorch with CUDA** | Most popular ML framework | Low | No | Install via pip/conda with CUDA support |
| **TensorFlow with GPU** | Second most popular, used in production | Medium | No | Requires specific CUDA/cuDNN versions |
| **Jupyter with GPU** | Interactive ML development standard | Low | No | JupyterLab in packages, needs GPU verification |
| **Conda/Miniconda** | Environment management for ML | Low | No | Multiple CUDA versions per project |
| **NVTOP/nvitop** | GPU process monitoring | Low | No | Essential for training visibility |

**Source:** [PyTorch Get Started](https://pytorch.org/get-started/locally/) | [GPU-Jupyter](https://github.com/iot-salzburg/gpu-jupyter)

### Local LLM Inference

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **Ollama** | De facto standard for local LLM serving | Low | No | Simple `ollama run llama3.2` interface |
| **Model Library** | Users expect pre-configured models | Low | No | Ollama handles download/caching |
| **OpenAI-Compatible API** | Integration with tools expecting OpenAI | Low | No | Ollama provides this natively |
| **GPU Acceleration** | LLM inference must use GPU | Low | No | Ollama auto-detects NVIDIA CUDA |

**Source:** [Ollama GitHub](https://github.com/ollama/ollama) | [Local LLM Hosting Guide](https://www.glukhov.org/post/2025/11/hosting-llms-ollama-localai-jan-lmstudio-vllm-comparison/)

### Image Generation

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **Stable Diffusion Backend** | Core image gen capability | Medium | No | ComfyUI or AUTOMATIC1111 |
| **Web UI** | Non-CLI interface expected | Low | No | Browser-based workflow |
| **Model Management** | Download/organize SD models | Medium | No | Civitai downloads, safetensors |
| **VRAM Optimization** | 16GB VRAM good but not unlimited | Low | No | --medvram flags, model offloading |

**Source:** [AUTOMATIC1111 Wiki](https://github.com/AUTOMATIC1111/stable-diffusion-webui/wiki/Install-and-Run-on-NVidia-GPUs) | [ComfyUI vs AUTOMATIC1111](https://www.propelrc.com/comfyui-vs-automatic1111-vs-fooocus/)

---

## Multi-Profile Distribution Table Stakes

Features expected from a distribution supporting multiple hardware targets.

### Profile Selection & Build System

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **Profile Directory Structure** | Organized separation of configs | Medium | Partial | Currently T2-only in archiso/ |
| **Shared Base Packages** | Don't duplicate common packages | Low | Yes | packages.x86_64 exists |
| **Profile-Specific Packages** | T2 vs Foundry hardware packages | Medium | No | Need packages.t2.x86_64, packages.foundry.x86_64 |
| **Profile-Specific Configs** | Different kernel params, services | Medium | No | grub.cfg, systemd units vary by profile |
| **Build Script Profiles** | `./build.sh --profile foundry` | Low | No | Currently single-profile build |

**Source:** [archiso ArchWiki](https://wiki.archlinux.org/title/Archiso) | [archiso GitLab](https://github.com/archlinux/archiso)

### Shared vs Profile-Specific Components

| Component | Shared | T2-Specific | Foundry-Specific |
|-----------|--------|-------------|------------------|
| Desktop (Hyprland, Waybar) | Yes | - | - |
| Theme System | Yes | - | - |
| Development Tools | Yes | - | - |
| Kernel | - | linux-t2 | linux (mainline) |
| GPU Drivers | - | (none/Intel) | nvidia-open-dkms |
| Hardware Firmware | - | apple-bcm-firmware, tiny-dfr | - |
| Fan Control | - | t2fanrd | - |
| CUDA Stack | - | - | cuda, cudnn, nvidia-container-toolkit |
| Gaming Tools | - | - | steam, mangohud, gamemode |

---

## Gaming Table Stakes

Features expected for Linux gaming capability.

| Feature | Why Expected | Complexity | Existing? | Notes |
|---------|--------------|------------|-----------|-------|
| **Steam** | Primary game distribution | Low | No | steam package from multilib |
| **Proton/Wine** | Windows game compatibility | Low | No | Bundled with Steam, also proton-ge-custom |
| **Vulkan** | Graphics API for modern games | Low | No | vulkan-icd-loader, nvidia-utils |
| **32-bit Libraries** | Many games need lib32 | Medium | No | Enable multilib repo |
| **MangoHud** | FPS/performance overlay | Low | No | mangohud package |
| **GameMode** | Performance optimization daemon | Low | No | gamemode package |
| **Controller Support** | USB/Bluetooth gamepads | Low | No | Usually works out of box |

**Source:** [Proton Requirements](https://github.com/ValveSoftware/Proton/wiki/Requirements) | [MangoHud](https://github.com/flightlessmango/MangoHud)

---

## Differentiators

Features that set VulcanOS apart from Pop!_OS, Ubuntu AI, Fedora Workstation, etc.

### Keyboard-Driven AI Workflow

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Hyprland + AI Tools** | Tiling WM optimized for multiple terminals, model outputs, code editors simultaneously | Low | Leverage existing Hyprland config |
| **Speech-to-Text Integration** | hyprwhspr already configured for voice input | Low | Already built, extends to AI coding |
| **Keybind for LLM Chat** | `Super+A` opens local AI chat (Open WebUI) | Low | New binding, analogous to `Super+T` for terminal |
| **Quick Model Switch** | wofi/dmenu launcher to switch Ollama models | Medium | Script + binding |
| **GPU Monitor in Waybar** | Real-time VRAM usage in status bar | Low | Waybar module for nvidia-smi |

### Unified Theming Extends to AI Tools

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Open WebUI Theming** | Match VulcanOS dark theme in AI chat | Medium | CSS injection or custom deployment |
| **Jupyter Dark Theme** | Coordinated appearance with desktop | Low | JupyterLab theme extension |
| **ComfyUI Theming** | Consistent dark mode | Low | Built-in theme support |
| **nvitop Styling** | Terminal colors match theme | Low | Uses terminal theme automatically |

### Developer-First AI Integration

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **MCP Servers for AI** | vulcan-todo, vulcan-vault already MCP-enabled | Low | Existing, extend to AI tools |
| **Local LLM for Code** | Claude Code / OpenCode can use local Ollama backend | Medium | Configuration, not code |
| **Containerized ML** | Pre-configured docker-compose for common ML stacks | Medium | Ship compose files in /usr/share/vulcanos/ |
| **GPU Jupyter One-Liner** | `vulcan-jupyter` script to launch GPU-enabled Jupyter | Low | Wrapper script |

### Multi-Monitor Excellence

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Triple Monitor Defaults** | Pre-configured Hyprland layout for 3 monitors | Low | monitors.conf templates |
| **Per-Workspace Monitor Binding** | Workspaces 1-3 on monitor 1, 4-6 on monitor 2, etc. | Low | Already possible in Hyprland |
| **Monitor-Aware Wallpapers** | Different wallpaper per monitor (already built) | Low | vulcan-wallpaper-manager exists |
| **GPU Output Routing** | Ensure all monitors on dGPU (no hybrid switching needed) | Low | NVIDIA-only system |

---

## Anti-Features

Features to explicitly NOT build. These are tempting but wrong for VulcanOS.

### Over-Engineering

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Custom Model Training GUI** | Complex, better tools exist (MLflow, W&B) | Document CLI workflows, provide scripts |
| **Built-in Model Zoo** | Storage bloat, models change rapidly | Point to Hugging Face/Ollama library, let users pull |
| **Automatic GPU Switching** | Complexity nightmare, single-GPU system | Foundry profile is NVIDIA-only, no hybrid graphics |
| **vLLM Production Server** | Overkill for personal workstation | Ollama is sufficient for single-user inference |

### Maintenance Burden

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Custom Kernel Patches for AI** | Upstream NVIDIA drivers work, don't fork | Use mainline kernel + NVIDIA DKMS |
| **Pre-installed Large Models** | 7GB+ per model, bloats ISO | First-run wizard to download preferred models |
| **Ollama as System Service** | Not needed for single user | User service or on-demand start |
| **Cloud API Integration** | Privacy-focused local-first philosophy | Document how to add if user wants |

### Scope Creep

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Windows Dual-Boot Helper** | Complex, VulcanOS is primary OS | Document if users ask, don't automate |
| **Remote GPU Access** | Server feature, not workstation | User can configure SSH/VNC themselves |
| **Kubernetes for ML** | Enterprise feature, single-node overkill | Docker Compose is sufficient |
| **AMD ROCm Support** | Foundry has NVIDIA, T2 has no dGPU | Keep profiles hardware-specific |

---

## Feature Dependencies

```
Existing VulcanOS Features (v2.x):
├── Hyprland compositor with modular config
├── Waybar status bar
├── Unified theming (vulcan-theme, vulcan-appearance-manager)
├── Wallpaper management (vulcan-wallpaper-manager)
├── Speech-to-text (hyprwhspr)
├── MCP servers (vulcan-todo, vulcan-vault)
├── Development tools (Docker, git, neovim, VSCode)
└── T2 hardware support (kernel, firmware, drivers)

v3.0 New Dependencies:

Multi-Profile Build System
├── Requires: Refactored archiso structure
├── Shared: dotfiles/, common packages
└── Profile-specific: kernel, hardware packages, configs

NVIDIA GPU Stack (Foundry profile only)
├── Requires: linux (mainline kernel)
├── Depends: nvidia-open-dkms (570+)
├── Depends: cuda (12.8+)
├── Depends: nvidia-container-toolkit
└── Enables: All AI/ML features below

Local LLM Infrastructure
├── Requires: NVIDIA GPU Stack (for acceleration)
├── Ollama (server)
├── Open WebUI (interface)
└── Keybindings + Waybar module

Image Generation
├── Requires: NVIDIA GPU Stack
├── ComfyUI OR AUTOMATIC1111
├── Python venv isolation
└── Model storage (~/.local/share/stable-diffusion/)

Gaming
├── Requires: NVIDIA GPU Stack
├── Requires: multilib repository
├── Steam + Proton
├── MangoHud + GameMode
└── Vulkan drivers

ML Development
├── Requires: NVIDIA GPU Stack
├── Conda/Miniconda
├── PyTorch + TensorFlow (user installs)
├── JupyterLab with GPU
└── nvtop/nvitop monitoring
```

---

## MVP Recommendation

### Phase 1: Multi-Profile Foundation (Required First)
1. **Refactor archiso structure** for profile support
2. **Create Foundry profile** with mainline kernel
3. **NVIDIA driver installation** in Foundry profile (570+ open modules)
4. **Basic CUDA** (toolkit only, not full ML stack)

### Phase 2: Local LLM Core
5. **Ollama installation** in Foundry packages
6. **Open WebUI** (Docker deployment or package)
7. **Keybindings** for AI chat (`Super+A` → browser to localhost:8080)
8. **Waybar GPU module** showing VRAM usage

### Phase 3: Gaming & Media
9. **Enable multilib** repository
10. **Steam + Proton** packages
11. **MangoHud + GameMode** packages
12. **Vulkan** driver verification

### Phase 4: ML Development (Post-MVP)
- ComfyUI/AUTOMATIC1111 (optional, user can install)
- JupyterLab GPU integration
- Containerized ML stacks
- Pre-configured docker-compose templates

### Explicitly Deferred
- vLLM production server
- Cloud API integrations
- Custom model training GUI
- AMD ROCm support
- Kubernetes/orchestration

---

## Complexity Assessment

| Feature Category | Table Stakes | Differentiators | Total Effort |
|------------------|--------------|-----------------|--------------|
| Multi-Profile Build | Medium | N/A | 2-3 days |
| NVIDIA Stack | Medium | Low | 1-2 days |
| Local LLM | Low | Medium | 1-2 days |
| Gaming | Low | N/A | 1 day |
| Image Gen | Medium | Low | 2 days (optional) |
| ML Development | Medium | Medium | 2-3 days |

**Total Estimated Effort:** 8-12 days for full v3.0 AI Workstation

---

## Existing Feature Leverage

Features already built that become more valuable in v3.0:

| Existing Feature | v3.0 Enhancement |
|------------------|------------------|
| vulcan-appearance-manager | Theme AI tools (Open WebUI, Jupyter) |
| vulcan-wallpaper-manager | Multi-monitor defaults for triple setup |
| hyprwhspr | Voice input to AI chat/coding |
| Hyprland modular config | Add GPU-specific bindings, workspace layouts |
| Docker pre-installed | Run containerized ML/AI tools |
| MCP servers | Extend to AI workflow automation |
| Waybar | Add GPU monitoring module |

---

## Sources

### High Confidence (Official Documentation)
- [NVIDIA CUDA Installation Guide for Linux](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/) - CUDA 12.x installation
- [NVIDIA Container Toolkit Install Guide](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) - Docker GPU support
- [archiso ArchWiki](https://wiki.archlinux.org/title/Archiso) - Multi-profile build system
- [Hyprland Monitors Wiki](https://wiki.hypr.land/Configuring/Monitors/) - Multi-monitor configuration
- [PyTorch Get Started](https://pytorch.org/get-started/locally/) - GPU installation

### Medium Confidence (Verified Community Sources)
- [RTX 5070 Ti Linux Guide](https://medium.com/@mbonsign/complete-guide-for-nvidia-rtx-5070-ti-on-linux-with-pytorch-358454521f04) - Blackwell GPU setup
- [Local LLM Hosting Guide 2026](https://www.glukhov.org/post/2025/11/hosting-llms-ollama-localai-jan-lmstudio-vllm-comparison/) - Ollama vs alternatives
- [Open WebUI Documentation](https://docs.openwebui.com/) - Self-hosted AI chat interface
- [GPU-Jupyter GitHub](https://github.com/iot-salzburg/gpu-jupyter) - Containerized Jupyter with GPU
- [MangoHud GitHub](https://github.com/flightlessmango/MangoHud) - Gaming overlay

### Low Confidence (WebSearch, Needs Validation)
- ComfyUI performance on RTX 5070 Ti (limited real-world reports yet)
- Specific Ollama model performance on 16GB VRAM (varies by model)
- Multi-monitor gaming with MangoHud (community reports vary)

### Internal Sources (VulcanOS Codebase - High Confidence)
- `/home/evan/VulcanOS/archiso/packages.x86_64` - Current package list
- `/home/evan/VulcanOS/dotfiles/hypr/.config/hypr/` - Hyprland configuration
- `/home/evan/VulcanOS/.planning/codebase/ARCHITECTURE.md` - System architecture
- `/home/evan/VulcanOS/.planning/research/FEATURES.md` (previous) - Theming features
