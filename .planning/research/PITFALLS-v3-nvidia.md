# Pitfalls Research: VulcanOS v3.0 NVIDIA AI Workstation

**Domain:** NVIDIA AI Workstation on Arch Linux with Hyprland
**Target Hardware:** RTX 5070 Ti (Blackwell GB203)
**Researched:** 2026-02-02
**Overall Confidence:** HIGH (verified against official documentation and recent community reports)

## Executive Summary

Building an NVIDIA-based AI workstation on Arch Linux with a bleeding-edge Blackwell GPU presents significant challenges across four interconnected domains: driver compatibility, CUDA/ML framework support, Wayland compositor integration, and Arch's rolling release model. The RTX 5070 Ti specifically requires the open kernel modules (proprietary drivers are unsupported on Blackwell), CUDA 12.8+, and has known PCIe link speed bugs. The roadmap must sequence these dependencies carefully: driver stability first, then CUDA stack, then AI frameworks, then gaming/Proton.

---

## Critical Pitfalls

Mistakes that cause system-breaking issues or require complete rewrites.

### Pitfall 1: Using Proprietary Kernel Modules on Blackwell

**What goes wrong:** System fails to boot or GPU is not detected. The proprietary NVIDIA kernel modules do not support Blackwell (RTX 50-series) architecture at all.

**Why it happens:** NVIDIA mandates open kernel modules for Grace Hopper and Blackwell platforms. The proprietary modules are unsupported on these architectures.

**Consequences:**
- Black screen on boot
- No GPU acceleration available
- System falls back to software rendering or fails entirely

**Prevention:**
- Install `nvidia-open` or `nvidia-open-dkms`, NOT `nvidia` or `nvidia-dkms`
- Verify with: `cat /proc/driver/nvidia/gpus/*/information | grep "GPU UUID"`
- Add `nvidia-open` to mkinitcpio MODULES array

**Detection (warning signs):**
- Boot hangs after GRUB
- `lspci` shows GPU but `nvidia-smi` fails
- Journal shows "NVRM: No NVIDIA GPU found"

**Phase to address:** Phase 1 (Driver Foundation) - This must be correct from the start.

**Confidence:** HIGH - [NVIDIA official documentation](https://developer.nvidia.com/blog/nvidia-transitions-fully-towards-open-source-gpu-kernel-modules/) explicitly states this requirement.

---

### Pitfall 2: PCIe Gen1 Fallback Bug (RTX 5070 Ti Specific)

**What goes wrong:** RTX 5070 Ti falls back to PCIe Gen1 (2.5 GT/s) instead of Gen4 (16 GT/s), causing system instability, black screens, random crashes, and severely degraded performance.

**Why it happens:** A known bug in NVIDIA open kernel modules (confirmed in driver 590.48.01) causes Blackwell GPUs to fail PCIe link negotiation on certain motherboards (B760 confirmed affected).

**Consequences:**
- ~75% bandwidth reduction affecting AI workloads
- System instability and random black screens
- Crashes during GPU-intensive operations

**Prevention:**
- Check current driver version against [NVIDIA open-gpu-kernel-modules issues](https://github.com/NVIDIA/open-gpu-kernel-modules/issues/1010)
- Enable "Above 4G Decoding" and "Resizable BAR" in BIOS
- Consider kernel parameter `pcie_aspm=off` if issues persist
- Test with: `nvidia-smi -q | grep "Link Gen"`

**Detection (warning signs):**
- `nvidia-smi` shows "Gen1" instead of "Gen4"
- Unexplained crashes during CUDA operations
- Poor AI inference/training performance vs expectations

**Phase to address:** Phase 1 (Driver Foundation) - Verify during initial driver testing.

**Confidence:** HIGH - [Verified GitHub issue](https://github.com/NVIDIA/open-gpu-kernel-modules/issues/1010) with multiple affected users.

---

### Pitfall 3: Driver/Library Version Mismatch After Updates

**What goes wrong:** After updating Arch, `nvidia-smi` fails with "Driver/library version mismatch" and GPU operations fail.

**Why it happens:** Arch's rolling release updates kernel and driver packages, but the running kernel still has old modules loaded. The initramfs may not have been rebuilt, or a reboot was skipped.

**Consequences:**
- Complete loss of GPU functionality until reboot
- AI workloads fail mid-operation
- Docker/Podman containers lose GPU access

**Prevention:**
1. Set up the pacman hook to auto-rebuild initramfs:
   ```bash
   # /etc/pacman.d/hooks/nvidia.hook
   [Trigger]
   Operation = Install
   Operation = Upgrade
   Operation = Remove
   Type = Package
   Target = nvidia-open
   Target = nvidia-open-dkms
   Target = linux
   Target = linux-lts

   [Action]
   Description = Rebuilding initramfs with NVIDIA modules...
   Depends = mkinitcpio
   When = PostTransaction
   NeedsTargets
   Exec = /bin/sh -c 'while read -r trg; do case $trg in linux*) exit 0; esac; done; /usr/bin/mkinitcpio -P'
   ```
2. Always reboot after kernel or nvidia package updates
3. Add NVIDIA modules to mkinitcpio.conf MODULES array

**Detection (warning signs):**
- `nvidia-smi` returns driver/library version mismatch
- `dmesg | grep nvidia` shows module version conflicts
- Docker containers can't access GPU

**Phase to address:** Phase 1 (Driver Foundation) - Include pacman hook in ISO.

**Confidence:** HIGH - [Arch Wiki NVIDIA documentation](https://wiki.archlinux.org/title/NVIDIA#pacman_hook).

---

### Pitfall 4: CUDA Compute Capability sm_120 Not Supported

**What goes wrong:** PyTorch, TensorFlow, and other ML frameworks fail with "no kernel image is available for execution on the device" or "CUDA capability sm_120 is not compatible."

**Why it happens:** RTX 50-series uses SM120 (Blackwell) compute capability. Current stable releases of PyTorch (2.5.x) only support sm_50 through sm_90. TensorFlow has similar limitations.

**Consequences:**
- GPU detected but all operations fall back to CPU
- Training/inference 10-100x slower
- Users think setup is "working" but performance is terrible

**Prevention:**
- Use PyTorch nightly builds with CUDA 12.8: `pip install --pre torch --extra-index-url https://download.pytorch.org/whl/nightly/cu128`
- Monitor [PyTorch Blackwell support issue](https://github.com/pytorch/pytorch/issues/151376)
- Use conda/mamba with nightly channels for faster updates
- Consider containers with pre-built CUDA 12.8 images

**Detection (warning signs):**
- `torch.cuda.is_available()` returns True but operations fail
- Error messages mentioning "sm_120" or "compute_120"
- GPU memory shows 0 usage during "GPU" operations

**Phase to address:** Phase 2 (CUDA/AI Stack) - Must use nightly/bleeding-edge packages.

**Confidence:** HIGH - [Verified PyTorch issue](https://github.com/pytorch/pytorch/issues/151376) and [TensorFlow issue](https://github.com/tensorflow/tensorflow/issues/99592).

---

### Pitfall 5: Suspend/Resume Crashes Hyprland

**What goes wrong:** After resuming from suspend, Hyprland crashes to login manager or displays black screen. External monitors may not wake.

**Why it happens:** NVIDIA drivers don't properly preserve video memory during suspend without explicit configuration. The open drivers have additional bugs around suspend handling.

**Consequences:**
- All open work lost on resume
- Users avoid suspend, impacting laptop usability
- Corrupted display/videos after wake

**Prevention:**
1. Add kernel parameter: `nvidia.NVreg_PreserveVideoMemoryAllocations=1`
2. Enable NVIDIA power services:
   ```bash
   systemctl enable nvidia-suspend.service
   systemctl enable nvidia-hibernate.service
   systemctl enable nvidia-resume.service
   ```
3. Consider using proprietary-based nvidia-dkms if suspend issues persist (note: this conflicts with Blackwell - see Pitfall 1)
4. Use [Hyprland suspend fix](https://github.com/0xFMD/hyprland-suspend-fix) script if issues persist

**Detection (warning signs):**
- Crash to login manager after opening laptop lid
- External monitors black after resume (internal works)
- Journal shows "Failed detecting connected display devices"

**Phase to address:** Phase 3 (Desktop Integration) - Configure after Hyprland is working.

**Confidence:** HIGH - [Hyprland Wiki NVIDIA section](https://wiki.hypr.land/Nvidia/) and multiple forum reports.

---

## Moderate Pitfalls

Mistakes that cause delays, debugging sessions, or technical debt.

### Pitfall 6: Missing DRM Kernel Mode Setting

**What goes wrong:** Hyprland fails to start, or starts with no GPU acceleration, flickering, or high CPU usage.

**Why it happens:** NVIDIA DRM must be enabled for Wayland compositors. While driver 560+ defaults to enabled, early boot can fail if modules aren't in initramfs.

**Prevention:**
1. Add to kernel parameters: `nvidia_drm.modeset=1` (or `nvidia-drm.modeset=1`)
2. Add to mkinitcpio.conf MODULES: `nvidia nvidia_modeset nvidia_uvm nvidia_drm`
3. Verify: `cat /sys/module/nvidia_drm/parameters/modeset` should return `Y`

**Detection:**
- Hyprland logs show "No DRM backend"
- High CPU usage in compositor
- Screen tearing or artifacts

**Phase to address:** Phase 1 (Driver Foundation)

**Confidence:** HIGH - [Hyprland NVIDIA Wiki](https://wiki.hypr.land/Nvidia/)

---

### Pitfall 7: XWayland Game Flickering

**What goes wrong:** Games running through XWayland flicker, show frames out of order, or are unplayable.

**Why it happens:** Lack of implicit sync in NVIDIA driver and/or incomplete explicit sync support. Pre-555 drivers are especially problematic.

**Prevention:**
- Ensure driver 555+ is installed
- Use gamescope as a nested compositor for games: `gamescope -f -- %command%`
- For Steam games, add launch option: `gamescope -f -- %command%`
- Ensure Hyprland is built with explicit sync support

**Detection:**
- Visual flickering only in XWayland apps (Wayland native works)
- Steam/Proton games unplayable while native apps work fine

**Phase to address:** Phase 4 (Gaming) - Configure gamescope wrapper.

**Confidence:** HIGH - [Hyprland Wiki](https://wiki.hypr.land/Nvidia/)

---

### Pitfall 8: Python Environment Conflicts (pip vs pacman)

**What goes wrong:** Installing ML packages with pip breaks system Python packages, causes "externally-managed-environment" errors, or creates version conflicts.

**Why it happens:** Arch adopted PEP 668, preventing system-wide pip installs. Mixing pacman and pip package managers corrupts dependencies.

**Prevention:**
1. NEVER use `pip install --break-system-packages`
2. Use virtual environments for all projects: `python -m venv .venv`
3. Recommend Miniconda/Mamba for ML workloads (manages CUDA versions too)
4. For ISO: include `python-pipx` for CLI tools, document venv workflow

**Detection:**
- "externally-managed-environment" errors
- Import errors for packages that "should" be installed
- Version conflicts between torch/tensorflow dependencies

**Phase to address:** Phase 2 (CUDA/AI Stack) - Document recommended workflow.

**Confidence:** HIGH - [Arch Wiki Python](https://wiki.archlinux.org/title/Python)

---

### Pitfall 9: Docker GPU Access Fails After Updates

**What goes wrong:** Docker containers can't access GPU with "could not select device driver" error.

**Why it happens:** nvidia-container-toolkit not properly configured, or systemd daemon not reloaded after updates.

**Prevention:**
1. Configure runtime: `sudo nvidia-ctk runtime configure --runtime=docker`
2. Restart Docker: `sudo systemctl restart docker`
3. After updates: `sudo systemctl daemon-reload && sudo systemctl restart docker`
4. Test: `docker run --rm --gpus all nvidia/cuda:12.8-base-ubi8 nvidia-smi`

**Detection:**
- "could not select device driver" errors
- Docker can't find nvidia runtime
- GPU works on host but not in containers

**Phase to address:** Phase 2 (CUDA/AI Stack) - Include in container setup.

**Confidence:** HIGH - [NVIDIA Container Toolkit docs](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)

---

### Pitfall 10: VRAM Leak on Wayland

**What goes wrong:** VRAM usage increases over time, eventually causing crashes or forcing logout.

**Why it happens:** Some Wayland compositors (including Hyprland) trigger VRAM leaks in NVIDIA driver if GLVidHeapReuseRatio isn't configured.

**Prevention:**
- Add environment variable: `__GL_MaxFramesAllowed=1`
- Monitor VRAM: `nvidia-smi -l 1`
- Consider restarting Hyprland daily if leaks persist
- File bug reports with specific driver version

**Detection:**
- `nvidia-smi` shows increasing VRAM usage without new applications
- System becomes sluggish over multi-day uptime
- OOM killer terminates GPU processes

**Phase to address:** Phase 3 (Desktop Integration) - Add to environment config.

**Confidence:** MEDIUM - Reported in [NVIDIA forums](https://forums.developer.nvidia.com/t/driver-leaking-memory-wayland-hyprland/309627) but may be driver version specific.

---

### Pitfall 11: Proton Games Stuck at Launch

**What goes wrong:** Steam games with Proton get stuck in "launching" state and never start.

**Why it happens:** Missing 32-bit libraries, particularly `lib32-gnutls`, or NVIDIA Reflex causing crashes.

**Prevention:**
1. Install 32-bit libs: `pacman -S lib32-gnutls lib32-mesa lib32-nvidia-utils`
2. For Reflex issues, add to game launch options: `__GL_NVIDIA_Reflex=0 %command%`
3. Use ext4 for game storage (not NTFS)
4. Enable multilib repository in pacman.conf

**Detection:**
- Steam shows "Running" but no window appears
- Must force-close Steam to try again
- Native Linux games work, Proton games don't

**Phase to address:** Phase 4 (Gaming) - Include 32-bit packages in gaming profile.

**Confidence:** HIGH - [Arch Linux Forums](https://bbs.archlinux.org/viewtopic.php?id=305918)

---

## Minor Pitfalls

Annoyances that are easily fixable once discovered.

### Pitfall 12: HDMI Color Space Issues

**What goes wrong:** Colors look darker/washed out on HDMI-connected displays at 60Hz.

**Why it happens:** Driver incorrectly assumes color space for HDMI output.

**Prevention:** Add kernel parameter: `nvidia-modeset.debug_force_color_space=2`

**Phase to address:** Phase 3 (Desktop Integration)

**Confidence:** HIGH - [Arch Wiki NVIDIA Troubleshooting](https://wiki.archlinux.org/title/NVIDIA/Troubleshooting)

---

### Pitfall 13: BusID Hex vs Decimal Confusion

**What goes wrong:** Xorg config fails to find GPU when specifying BusID.

**Why it happens:** `lspci` shows hex values but Xorg expects decimal.

**Prevention:** Convert hex to decimal: `lspci | grep NVIDIA` shows "01:00.0", use `BusID "PCI:1:0:0"` in config.

**Phase to address:** Phase 3 (Desktop Integration) - Only relevant for hybrid GPU setups.

**Confidence:** HIGH - [Arch Wiki](https://wiki.archlinux.org/title/NVIDIA/Troubleshooting)

---

### Pitfall 14: GSP Firmware Issues (Pre-575 Drivers)

**What goes wrong:** Vulkan failures, system crashes, or performance issues.

**Why it happens:** NVIDIA GSP (GPU System Processor) firmware introduced in 555 had bugs.

**Prevention:**
- Use driver 575+ where GSP issues are fixed
- If on older driver, disable with: `nvidia.NVreg_EnableGpuFirmware=0`

**Detection:** Random Vulkan crashes, especially on resume

**Phase to address:** Phase 1 (Driver Foundation) - Use current drivers.

**Confidence:** HIGH - [Arch Wiki](https://wiki.archlinux.org/title/NVIDIA/Troubleshooting)

---

## Arch Linux Rolling Release Pitfalls

### Pitfall 15: Breaking Driver Updates Without Warning

**What goes wrong:** System update includes major NVIDIA driver change (e.g., 590 dropping Pascal support), breaking graphics.

**Why it happens:** Arch's rolling release model pushes updates immediately. The nvidia-open transition in December 2025 caught many users off-guard.

**Prevention:**
1. Subscribe to [arch-announce mailing list](https://lists.archlinux.org/listinfo/arch-announce)
2. Check `pacman -Syu` output before confirming updates
3. Keep a bootable USB with working drivers
4. Consider timeshift/btrfs snapshots before updates

**Detection:**
- Arch news posts about NVIDIA transitions
- Package names changing (nvidia -> nvidia-open)

**Phase to address:** Ongoing maintenance - Document update procedures.

**Confidence:** HIGH - [Arch Linux news](https://archlinux.org/news/) and [community reports](https://itsfoss.com/news/arch-linux-old-nvidia-gpu-issue/)

---

### Pitfall 16: Kernel/Driver Race During Boot

**What goes wrong:** Display manager starts before NVIDIA modules are loaded, causing login issues.

**Why it happens:** systemd service ordering doesn't guarantee NVIDIA is ready.

**Prevention:**
1. Add nvidia modules to early load in mkinitcpio.conf
2. Run `mkinitcpio -P` after changes
3. Consider `nvidia-persistenced` service for faster GPU initialization

**Detection:**
- Login screen appears but is slow/glitchy
- Second login attempt works fine

**Phase to address:** Phase 1 (Driver Foundation)

**Confidence:** HIGH - [Arch Wiki](https://wiki.archlinux.org/title/NVIDIA#Early_loading)

---

## Multi-Profile archiso Pitfalls

### Pitfall 17: Work Directory Not Cleaned Between Builds

**What goes wrong:** archiso reuses cached data and silently skips rebuilding, resulting in stale packages.

**Why it happens:** mkarchiso caches aggressively for speed.

**Prevention:**
- Delete work directory before each build: `rm -rf /tmp/archiso-work`
- Create `clean-build.sh` script that removes work dir first

**Detection:**
- Package changes don't appear in new ISO
- ISO build completes suspiciously fast

**Phase to address:** ISO Build Infrastructure

**Confidence:** HIGH - [Arch Wiki archiso](https://wiki.archlinux.org/title/Archiso)

---

### Pitfall 18: Custom Repository Path Errors

**What goes wrong:** Custom repo packages not found during build: "error: target not found".

**Why it happens:** Incorrect Server path in pacman.conf (including db filename) or repo database not generated.

**Prevention:**
1. Server path should be directory only: `Server = file:///path/to/repo` (no .db filename)
2. Regenerate repo database: `repo-add /path/to/repo/customrepo.db.tar.gz /path/to/repo/*.pkg.tar.zst`
3. Place repo in non-volatile location (not /tmp on tmpfs)

**Detection:**
- "failed retrieving file 'repo.db'" errors
- "target not found" for custom packages

**Phase to address:** ISO Build Infrastructure

**Confidence:** HIGH - [Arch Forums](https://bbs.archlinux.org/viewtopic.php?id=169043)

---

### Pitfall 19: Profile Permission Issues

**What goes wrong:** Files in airootfs have wrong permissions in built ISO.

**Why it happens:** Archiso defaults all files to 644/755 owned by root. Custom ownership requires explicit configuration.

**Prevention:**
- Use `file_permissions` associative array in profiledef.sh:
  ```bash
  file_permissions=(
    ["/etc/shadow"]="0:0:400"
    ["/usr/local/bin/script.sh"]="0:0:755"
  )
  ```

**Detection:**
- Scripts not executable on live ISO
- Config files have wrong ownership

**Phase to address:** ISO Build Infrastructure

**Confidence:** HIGH - [archiso profile documentation](https://github.com/archlinux/archiso/blob/master/docs/README.profile.rst)

---

### Pitfall 20: Package Conflicts Between Profiles

**What goes wrong:** AI profile packages conflict with gaming profile packages, or base package conflicts with profile-specific packages.

**Why it happens:** Different profiles may want different versions or mutually exclusive packages.

**Prevention:**
1. Create base package list for shared packages
2. Use profile-specific `packages.x86_64` for additions
3. Test each profile build independently
4. Document package exclusions in profile README

**Detection:**
- Build fails with "conflicting packages" error
- Different profiles can't share work directory

**Phase to address:** ISO Build Infrastructure - Design profile architecture first.

**Confidence:** MEDIUM - Inferred from archiso structure and package management patterns.

---

## Anti-Cheat Gaming Pitfalls

### Pitfall 21: EAC/BattlEye Blocks Linux

**What goes wrong:** Games with Easy Anti-Cheat or BattlEye don't launch or ban Linux users.

**Why it happens:** Anti-cheat developers must explicitly enable Linux/Proton support. Many don't.

**Prevention:**
- Check [ProtonDB](https://www.protondb.com/) before expecting games to work
- Don't expect anti-cheat games to work by default
- Consider separate Windows partition/VM for unsupported games
- Keep glibc-eac from AUR for games that need DT_HASH patch

**Detection:**
- Game launches but immediately kicks to menu
- "Anti-cheat not initialized" errors
- Steam Deck verification status "Unsupported"

**Phase to address:** Phase 4 (Gaming) - Document supported/unsupported games.

**Confidence:** HIGH - [GamingOnLinux](https://www.gamingonlinux.com/2025/11/anti-cheat-will-still-be-one-of-the-biggest-problems-for-the-new-steam-machine/)

---

### Pitfall 22: DirectX 12 Performance Penalty

**What goes wrong:** DX12 games run significantly slower than on Windows, even accounting for normal Proton overhead.

**Why it happens:** NVIDIA's DX12 to Vulkan translation layer (via VKD3D-Proton) has overhead, and NVIDIA's Linux Vulkan driver is less optimized for this workload than AMD's.

**Prevention:**
- Prefer Vulkan/OpenGL native games or DX11 games where possible
- Use ProtonGE for bleeding-edge VKD3D-Proton
- Accept 10-15% performance penalty vs Windows for DX12 titles
- Enable shader pre-caching in Steam

**Detection:**
- DX12 games run 20-30% slower than benchmarks suggest
- Stuttering during shader compilation
- Frame time spikes

**Phase to address:** Phase 4 (Gaming) - Set expectations, configure shader cache.

**Confidence:** HIGH - [Tom's Hardware benchmarks](https://www.tomshardware.com/news/proton-overhead-slows-4090-4080-10-percent-on-linux)

---

## Phase-Specific Warnings Summary

| Phase | Primary Pitfalls | Risk Level |
|-------|------------------|------------|
| Phase 1: Driver Foundation | #1 Open modules required, #2 PCIe bug, #3 Version mismatch, #6 DRM, #14 GSP, #16 Boot race | CRITICAL |
| Phase 2: CUDA/AI Stack | #4 sm_120 support, #8 Python envs, #9 Docker GPU | HIGH |
| Phase 3: Desktop Integration | #5 Suspend/resume, #10 VRAM leak, #12 HDMI color | MEDIUM |
| Phase 4: Gaming | #7 XWayland flicker, #11 Proton stuck, #21 Anti-cheat, #22 DX12 penalty | MEDIUM |
| ISO Build | #17 Work dir cache, #18 Repo paths, #19 Permissions, #20 Conflicts | MEDIUM |
| Ongoing | #15 Breaking updates, Arch news monitoring | MEDIUM |

---

## Roadmap Implications

Based on this pitfall research, the following phase structure is recommended:

### Phase 1: Driver Foundation (CRITICAL)
- Install `nvidia-open-dkms` (NOT nvidia-dkms)
- Configure DRM kernel mode setting
- Set up pacman hooks for initramfs rebuild
- Verify PCIe link speed
- Enable nvidia power management services

### Phase 2: CUDA/AI Stack (HIGH PRIORITY)
- Install CUDA 12.8+ toolkit
- Configure PyTorch/TensorFlow nightly builds
- Set up conda/mamba environment
- Configure nvidia-container-toolkit for Docker

### Phase 3: Desktop Integration (MEDIUM)
- Configure Hyprland environment variables
- Set up suspend/resume fixes
- Configure color space for HDMI
- Add VRAM monitoring

### Phase 4: Gaming Profile (MEDIUM)
- Enable multilib repository
- Install 32-bit NVIDIA libraries
- Configure gamescope
- Document anti-cheat limitations

### ISO Build Infrastructure
- Clean build workflow
- Profile-specific package lists
- Custom repository setup

---

## Sources

### Official Documentation
- [NVIDIA Transitions to Open-Source GPU Kernel Modules](https://developer.nvidia.com/blog/nvidia-transitions-fully-towards-open-source-gpu-kernel-modules/)
- [Hyprland NVIDIA Wiki](https://wiki.hypr.land/Nvidia/)
- [Arch Wiki - NVIDIA](https://wiki.archlinux.org/title/NVIDIA)
- [Arch Wiki - NVIDIA Troubleshooting](https://wiki.archlinux.org/title/NVIDIA/Troubleshooting)
- [NVIDIA Container Toolkit Installation](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)
- [archiso Profile Documentation](https://github.com/archlinux/archiso/blob/master/docs/README.profile.rst)

### Issue Trackers and Forums
- [RTX 5070 Ti PCIe Gen1 Fallback Bug](https://github.com/NVIDIA/open-gpu-kernel-modules/issues/1010)
- [PyTorch RTX 5000 Series Support Request](https://github.com/pytorch/pytorch/issues/151376)
- [TensorFlow RTX 5000 Support Issue](https://github.com/tensorflow/tensorflow/issues/99592)
- [Hyprland Suspend Fix](https://github.com/0xFMD/hyprland-suspend-fix)
- [NVIDIA Driver Memory Leak on Wayland](https://forums.developer.nvidia.com/t/driver-leaking-memory-wayland-hyprland/309627)
- [Arch Linux Forums - Proton Launch Issues](https://bbs.archlinux.org/viewtopic.php?id=305918)
- [archiso Custom Repository Issues](https://bbs.archlinux.org/viewtopic.php?id=169043)

### Community Guides
- [Complete Guide for NVIDIA RTX 5070 Ti on Linux with PyTorch](https://medium.com/@mbonsign/complete-guide-for-nvidia-rtx-5070-ti-on-linux-with-pytorch-358454521f04)
- [Ultimate Guide to Installing RTX 5000 Blackwell Drivers on Linux](https://gist.github.com/jatinkrmalik/86afb07cbe6abf5baa2d29d3842aa328)
- [Python ML on Arch Linux](https://davelage.com/posts/python-machine-learning-on-arch-linux/)

### News and Analysis
- [Arch Linux Old NVIDIA GPU Issue](https://itsfoss.com/news/arch-linux-old-nvidia-gpu-issue/)
- [NVIDIA Drops Pascal Support](https://hackaday.com/2025/12/26/nvidia-drops-pascal-support-on-linux-causing-chaos-on-arch-linux/)
- [Anti-Cheat Problems for Steam Machine](https://www.gamingonlinux.com/2025/11/anti-cheat-will-still-be-one-of-the-biggest-problems-for-the-new-steam-machine/)
- [Proton Performance Overhead on NVIDIA](https://www.tomshardware.com/news/proton-overhead-slows-4090-4080-10-percent-on-linux)
- [AMD vs NVIDIA Linux Driver Comparison 2026](https://linuxano.com/amd-vs-nvidia-on-linux/)
