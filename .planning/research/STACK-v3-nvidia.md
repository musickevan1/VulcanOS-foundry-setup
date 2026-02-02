# Stack Research: VulcanOS v3.0 AI Workstation

**Project:** VulcanOS v3.0 - Vulcan Foundry Profile
**Researched:** 2026-02-02
**Hardware Target:** AMD Ryzen 9 9950X, NVIDIA RTX 5070 Ti 16GB, 64GB DDR5
**Overall Confidence:** MEDIUM-HIGH

## Executive Summary

The RTX 5070 Ti (Blackwell architecture, sm_120 compute capability) requires specific driver and CUDA considerations that differ from previous NVIDIA generations. The critical finding is that **Blackwell GPUs require the open-source kernel modules** (`nvidia-open-dkms`) - the proprietary drivers are officially unsupported. Additionally, stable PyTorch releases do not yet support sm_120, requiring nightly builds with CUDA 12.8+. Arch Linux's recent transition to open kernel modules by default (NVIDIA 590 driver series) aligns well with Blackwell requirements.

**Key decisions:**
1. Use `nvidia-open-dkms` (590.48.01+) - mandatory for Blackwell
2. Use CUDA 13.1.1 from official repos with cuDNN 9.14
3. Install PyTorch from nightly builds (not stable) for sm_120 support
4. Use `ollama-cuda` for local LLM inference (official repo, not AUR)
5. Gaming stack fully supported via multilib with Proton-GE from AUR

---

## NVIDIA/CUDA Stack

### Driver Layer

| Package | Version | Source | Notes |
|---------|---------|--------|-------|
| `nvidia-open-dkms` | 590.48.01-2 | extra | **REQUIRED for Blackwell** - proprietary drivers unsupported |
| `nvidia-utils` | 590.48.01 | extra | Core utilities, pulled as dependency |
| `lib32-nvidia-utils` | 590.48.01-1 | multilib | Required for 32-bit games (Steam/Proton) |

**Why `nvidia-open-dkms` over `nvidia-dkms`:**
- NVIDIA's official position: "For cutting-edge platforms such as NVIDIA Blackwell, you must use the open-source GPU kernel modules"
- Arch Linux has made open modules the default as of NVIDIA 590 driver series (December 2025)
- Users report proprietary drivers fail to recognize Blackwell GPUs or cause display issues

**Kernel Configuration Required:**
```bash
# /etc/mkinitcpio.conf - Remove 'kms' from HOOKS
HOOKS=(base udev autodetect modconf keyboard keymap consolefont block filesystems fsck)

# /etc/default/grub - Add modeset parameter
GRUB_CMDLINE_LINUX_DEFAULT="loglevel=3 nvidia-drm.modeset=1"
```

**Post-install:**
```bash
sudo mkinitcpio -P
sudo grub-mkconfig -o /boot/grub/grub.cfg
```

### CUDA Toolkit

| Package | Version | Size | Source |
|---------|---------|------|--------|
| `cuda` | 13.1.1-1 | 4.7 GB installed | extra |

**Dependencies (auto-installed):**
- `gcc` - C/C++ compiler
- `opencl-nvidia` - OpenCL support
- `python` - Python runtime

**Optional but recommended:**
- `gdb` - for cuda-gdb debugging
- `nsight-compute` - CUDA profiling
- `nsight-systems` - system-wide profiling

**Important:** CUDA installs to `/opt/cuda` on Arch Linux, not `/usr/local/cuda-XX.X` as expected by some tools (notably JAX). Create symlink if needed:
```bash
sudo ln -s /opt/cuda /usr/local/cuda
```

### cuDNN

| Package | Version | Source |
|---------|---------|--------|
| `cudnn` | 9.14.0.64-1 | extra |

**Installation:** Simply install via pacman - it integrates with the system CUDA installation.

**Note:** Only one CUDA toolkit version of cuDNN 9 can be installed at a time. If you need multiple versions, use pip wheels or conda instead of system packages.

### Container Runtime

| Package | Version | Source |
|---------|---------|--------|
| `nvidia-container-toolkit` | 1.18.1-2 | extra |

**Configuration:**
```bash
# Configure Docker runtime
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker

# Test GPU access
docker run --rm --gpus all nvidia/cuda:13.0-base nvidia-smi
```

**Usage:**
```bash
docker run --runtime=nvidia --gpus all <image>
# Or with compose: deploy.resources.reservations.devices with driver: nvidia
```

---

## AI/ML Packages

### PyTorch

**CRITICAL:** Stable PyTorch does NOT support RTX 5070 Ti (sm_120). You must use nightly builds.

| Method | Package | Notes |
|--------|---------|-------|
| **Recommended** | pip nightly | `pip install --pre torch torchvision --index-url https://download.pytorch.org/whl/nightly/cu128` |
| Alternative | `python-pytorch-cuda` | Arch repo - may not support sm_120 yet |

**The sm_120 Problem:**
- RTX 5070 Ti uses Blackwell architecture with compute capability sm_120
- Current stable PyTorch (2.9.x) only supports up to sm_90
- CUDA 12.8+ required for sm_120 compilation
- PyTorch nightly builds include sm_120 support

**Recommended Installation:**
```bash
# Create virtual environment
python -m venv ~/ai-env
source ~/ai-env/bin/activate

# Install PyTorch nightly with CUDA 12.8
pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu128
```

**Verify installation:**
```python
import torch
print(torch.cuda.is_available())  # Should be True
print(torch.cuda.get_device_name(0))  # Should show RTX 5070 Ti
```

### JAX

| Method | Package | Notes |
|--------|---------|-------|
| **Recommended** | pip wheels | Pre-built CUDA 12/13 wheels |
| Alternative | AUR `python-jaxlib-cuda` | Build issues with clang-20, bazel dependency |

**Driver Requirements:**
- CUDA 12: Driver >= 525
- CUDA 13: Driver >= 580 (590 meets this)

**Installation:**
```bash
# For CUDA 13 (recommended with Blackwell)
pip install -U jax jaxlib "jax-cuda12-plugin[with-cuda]" jax-cuda12-pjrt

# Verify Arch CUDA path (may need symlink)
export XLA_FLAGS="--xla_gpu_cuda_data_dir=/opt/cuda"
```

**Known Issue:** JAX expects CUDA at `/usr/local/cuda-XX.X`, but Arch uses `/opt/cuda`. Either:
- Set `XLA_FLAGS` environment variable
- Create symlink: `sudo ln -s /opt/cuda /usr/local/cuda`

### Ollama

| Package | Version | Source | Notes |
|---------|---------|--------|-------|
| `ollama-cuda` | 0.15.3-1 | extra | **Use this for NVIDIA GPUs** |
| `ollama` | 0.14.1 | extra | CPU-only version |

**Do NOT use:**
- AUR `ollama-bin` - outdated, unnecessary with official packages
- Snap version - defeats purpose of Arch

**Installation:**
```bash
sudo pacman -S ollama-cuda
sudo systemctl enable --now ollama.service
```

**Verify:**
```bash
ollama run llama3.2:3b  # Quick test
```

### llama.cpp

| Package | Source | Notes |
|---------|--------|-------|
| `llama.cpp-cuda` | AUR | CUDA-accelerated build |
| Custom build | - | For specific optimizations |

**AUR Installation:**
```bash
yay -S llama.cpp-cuda
```

**Custom Build (for specific CUDA arch):**
```bash
git clone https://github.com/ggml-org/llama.cpp
cd llama.cpp
cmake -B build -DGGML_CUDA=ON -DCMAKE_CUDA_ARCHITECTURES=120
cmake --build build --config Release
```

**Note:** Use `CMAKE_CUDA_ARCHITECTURES=120` for RTX 5070 Ti specifically. The AUR package builds for multiple architectures by default.

### ComfyUI

**Not an Arch package** - install in virtual environment.

| Dependency | Package |
|------------|---------|
| Git | `git` |
| Python | `python` (3.13 recommended) |
| PyTorch | Via pip (nightly for sm_120) |

**Installation:**
```bash
# Create dedicated environment
python -m venv ~/comfyui-env
source ~/comfyui-env/bin/activate

# Install PyTorch with CUDA 12.8 (for sm_120)
pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu128

# Clone and install ComfyUI
git clone https://github.com/comfyanonymous/ComfyUI.git ~/ComfyUI
cd ~/ComfyUI
pip install -r requirements.txt
```

**Known Issue:** There's a sentencepiece wheel bug affecting some Linux configurations with Python 3.13 and CUDA 12.9-13. If encountered, try Python 3.12 or use conda.

---

## Gaming Stack

### Steam

| Package | Source | Notes |
|---------|--------|-------|
| `steam` | multilib | Official Steam client |

**Prerequisites:**
1. Enable multilib in `/etc/pacman.conf`
2. Install 32-bit NVIDIA libs: `lib32-nvidia-utils`

**Installation:**
```bash
sudo pacman -S steam
```

**Enable Proton:**
Steam > Settings > Compatibility > Enable Steam Play for all titles

### Proton

| Package | Source | Notes |
|---------|--------|-------|
| Proton Experimental | Steam | Auto-installed, default |
| `proton-ge-custom-bin` | AUR | Community fork with extra patches |

**Proton-GE Installation:**
```bash
yay -S proton-ge-custom-bin
```

Then select "GE-Proton" in Steam game properties > Compatibility.

**Note:** The AUR package was flagged out-of-date (2026-01-02) but remains functional. Dependencies listed are for use outside Steam; within Steam, no extra dependencies needed.

### Gamemode

| Package | Source | Notes |
|---------|--------|-------|
| `gamemode` | extra | CPU governor, I/O priority, GPU optimizations |
| `lib32-gamemode` | multilib | 32-bit game support |

**Usage in Steam:**
```
gamemoderun %command%
```

**With MangoHud:**
```
gamemoderun mangohud %command%
```

### Performance Monitoring

| Package | Version | Source | Notes |
|---------|---------|--------|-------|
| `mangohud` | 0.8.2-2 | extra | Vulkan/OpenGL overlay |
| `lib32-mangohud` | 0.8.2 | multilib | 32-bit game support |
| `gamescope` | 3.16.19-3 | extra | Micro-compositor for gaming |

**MangoHud with Gamescope:**
Traditional MangoHud injection is NOT supported with gamescope. Use:
```
gamescope --mangoapp -- %command%
```

---

## Hyprland Plugins

### Official Plugins (hyprwm/hyprland-plugins)

| Plugin | Purpose | Notes |
|--------|---------|-------|
| `hyprexpo` | Workspace overview (GNOME Activities-style) | Grid layout of all workspaces |
| `hyprtrails` | Smooth trails behind moving windows | Visual effect |

**Installation via hyprpm:**
```bash
hyprpm update
hyprpm add https://github.com/hyprwm/hyprland-plugins
hyprpm enable hyprexpo
hyprpm enable hyprtrails
```

**Autoload in config:**
```
exec-once = hyprpm reload
```

### Hyprspace (Third-party)

| Source | Purpose |
|--------|---------|
| github.com/KZDKM/Hyprspace | Workspace overview with drag-and-drop |

**Installation:**
```bash
hyprpm add https://github.com/KZDKM/Hyprspace
hyprpm enable Hyprspace
```

**Configuration options:**
- `plugin:overview:dragAlpha` - Window opacity when dragging
- `plugin:overview:centerAligned` - KDE/macOS style (true) or Windows style (false)

**Note:** Hyprspace developer recommends trying niri as an alternative - it's a scrolling WM with built-in overview.

---

## New Tools

### Yazi (File Manager)

| Package | Version | Source |
|---------|---------|--------|
| `yazi` | 26.1.4-1 | extra |

**Features:**
- Async I/O (non-blocking)
- GPU-accelerated image preview
- Uberzug++ and Chafa integration
- Built-in code highlighting
- Git integration
- Bulk rename, archive extraction

**Optional dependencies (recommended):**
```bash
sudo pacman -S yazi ffmpeg 7zip jq poppler fd ripgrep fzf zoxide imagemagick
```

**Shell integration (add to ~/.bashrc):**
```bash
function yy() {
    local tmp="$(mktemp -t "yazi-cwd.XXXXXX")"
    yazi "$@" --cwd-file="$tmp"
    if cwd="$(cat -- "$tmp")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then
        cd -- "$cwd"
    fi
    rm -f -- "$tmp"
}
```

### Kitty (Terminal)

| Package | Source | Notes |
|---------|--------|-------|
| `kitty` | extra | GPU-accelerated terminal |

**Why Kitty over Alacritty:**
- GPU-rendered with OpenGL (similar performance)
- Built-in image protocol (inline images in terminal)
- Tiling/tabs built-in (no tmux needed for basic splits)
- Kitten extensions (ssh, hints, unicode input)
- Better ligature support

**NVIDIA Considerations:**
- Works well with NVIDIA drivers
- Use `--single-instance` to share GPU sprite cache
- Debug GPU issues with `--debug-gl`

**Configuration location:** `~/.config/kitty/kitty.conf`

**Key settings for NVIDIA:**
```
# kitty.conf
linux_display_server wayland
wayland_titlebar_color background

# GPU rendering
repaint_delay 10
input_delay 3
sync_to_monitor yes
```

---

## Complete Package List

### Official Repositories (pacman)

```bash
# NVIDIA/CUDA (extra)
sudo pacman -S nvidia-open-dkms nvidia-utils cuda cudnn nvidia-container-toolkit

# Gaming (multilib)
sudo pacman -S steam lib32-nvidia-utils gamemode lib32-gamemode mangohud lib32-mangohud gamescope

# AI/ML
sudo pacman -S ollama-cuda

# Tools
sudo pacman -S yazi kitty ffmpeg 7zip jq poppler fd ripgrep fzf zoxide imagemagick
```

### AUR Packages

```bash
# Gaming
yay -S proton-ge-custom-bin

# AI/ML (optional - consider pip instead)
yay -S llama.cpp-cuda
```

### Pip Packages (in virtual environment)

```bash
# PyTorch with CUDA 12.8 (nightly for sm_120)
pip install --pre torch torchvision torchaudio --index-url https://download.pytorch.org/whl/nightly/cu128

# JAX
pip install -U jax jaxlib "jax-cuda12-plugin[with-cuda]" jax-cuda12-pjrt

# ComfyUI dependencies
pip install -r requirements.txt  # From ComfyUI repo
```

---

## Package Conflicts / Warnings

### CRITICAL: Blackwell Driver Compatibility

| Issue | Impact | Solution |
|-------|--------|----------|
| `nvidia-dkms` vs `nvidia-open-dkms` | Blackwell GPUs won't work with proprietary drivers | Use `nvidia-open-dkms` exclusively |
| PyTorch stable + sm_120 | GPU not recognized, CUDA errors | Use PyTorch nightly builds |
| CUDA path mismatch | JAX/TensorFlow can't find CUDA | Symlink `/opt/cuda` to `/usr/local/cuda` |

### Do NOT Install

| Package | Reason |
|---------|--------|
| `nvidia-dkms` | Conflicts with `nvidia-open-dkms`, doesn't support Blackwell |
| `nvidia-lts` | Use `nvidia-open-dkms` instead |
| `python-pytorch-cuda` (system) | May not support sm_120 yet; use pip nightly |
| `ollama` (base) | Use `ollama-cuda` for GPU support |
| `ollama-bin` (AUR) | Outdated, official package is better |

### Version Pinning Warnings

| Package | Issue | Mitigation |
|---------|-------|------------|
| PyTorch nightly | API instability | Pin working version in requirements.txt |
| Proton-GE | Flagged out-of-date | Monitor AUR updates |
| NVIDIA drivers | Frequent updates can break | Test in VM before production updates |

### Testing Repository Considerations

If using `[testing]` repos:
- Must also enable `[multilib-testing]`
- Driver version mismatches between `nvidia-utils` and `lib32-nvidia-utils` can occur
- Solution: Use all testing repos or none

---

## Integration with Existing VulcanOS Stack

### Hyprland Configuration Additions

Add to `hyprland.conf`:
```
# Load Hyprland plugins
exec-once = hyprpm reload

# Hyprexpo configuration
plugin {
    hyprexpo {
        columns = 3
        gap_size = 5
        bg_col = rgb(111111)
        workspace_method = first 1
    }
}

# Hyprtrails configuration
plugin {
    hyprtrails {
        color = rgba(ffaa00ff)
    }
}
```

### Environment Variables

Add to `envs.conf`:
```
# CUDA
env = CUDA_PATH,/opt/cuda
env = PATH,$PATH:/opt/cuda/bin

# For JAX compatibility
env = XLA_FLAGS,--xla_gpu_cuda_data_dir=/opt/cuda
```

### Terminal Transition (Alacritty -> Kitty)

Update bindings:
```
# bindings.conf
bind = $mod, Return, exec, kitty
```

---

## Sources

### NVIDIA Drivers
- [Arch Wiki - NVIDIA](https://wiki.archlinux.org/title/NVIDIA)
- [Arch Linux News - NVIDIA 590 driver changes](https://archlinux.org/news/nvidia-590-driver-drops-pascal-support-main-packages-switch-to-open-kernel-modules/)
- [nvidia-open-dkms package](https://archlinux.org/packages/extra/x86_64/nvidia-open-dkms/)
- [Ultimate Guide to Installing RTX 5000 Blackwell Drivers on Linux](https://gist.github.com/jatinkrmalik/86afb07cbe6abf5baa2d29d3842aa328)
- [NVIDIA Developer Forums - RTX 5070 Ti installation issues](https://forums.developer.nvidia.com/t/install-of-rtx-5070-ti-problematic-on-linux/331240)

### CUDA/cuDNN
- [CUDA Installation Guide for Linux](https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html)
- [Arch Linux CUDA package](https://archlinux.org/packages/extra/x86_64/cuda/)
- [Arch Linux cuDNN package](https://archlinux.org/packages/extra/x86_64/cudnn/)

### PyTorch sm_120
- [PyTorch Forums - RTX 5070 Ti sm_120 compatibility](https://discuss.pytorch.org/t/nvidia-geforce-rtx-5070-ti-with-cuda-capability-sm-120/221509)
- [PyTorch GitHub Issue #153928](https://github.com/pytorch/pytorch/issues/153928)
- [Complete guide for Nvidia RTX 5070 TI on Linux with PyTorch](https://medium.com/@mbonsign/complete-guide-for-nvidia-rtx-5070-ti-on-linux-with-pytorch-358454521f04)

### AI/ML Stack
- [Ollama ArchWiki](https://wiki.archlinux.org/title/Ollama)
- [ollama-cuda package](https://archlinux.org/packages/extra/x86_64/ollama-cuda/)
- [JAX Installation Documentation](https://docs.jax.dev/en/latest/installation.html)
- [llama.cpp build documentation](https://github.com/ggml-org/llama.cpp/blob/master/docs/build.md)
- [ComfyUI Linux Installation](https://comfyui-wiki.com/en/install/install-comfyui/install-comfyui-on-linux)

### Gaming
- [Steam ArchWiki](https://wiki.archlinux.org/title/Steam)
- [Gaming ArchWiki](https://wiki.archlinux.org/title/Gaming)
- [MangoHud ArchWiki](https://wiki.archlinux.org/title/MangoHud)
- [Gamescope ArchWiki](https://wiki.archlinux.org/title/Gamescope)
- [proton-ge-custom-bin AUR](https://aur.archlinux.org/packages/proton-ge-custom-bin)

### Hyprland Plugins
- [Official hyprland-plugins repository](https://github.com/hyprwm/hyprland-plugins)
- [Hyprspace repository](https://github.com/KZDKM/Hyprspace)
- [Hyprland Wiki - Using Plugins](https://wiki.hypr.land/Plugins/Using-Plugins/)

### Tools
- [Yazi package](https://archlinux.org/packages/extra/x86_64/yazi/)
- [Yazi GitHub](https://github.com/sxyazi/yazi)
- [Kitty ArchWiki](https://wiki.archlinux.org/title/Kitty)
- [Kitty official site](https://sw.kovidgoyal.net/kitty/)

### Container Toolkit
- [nvidia-container-toolkit package](https://archlinux.org/packages/extra/x86_64/nvidia-container-toolkit/)
- [NVIDIA Container Toolkit Installation Guide](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
