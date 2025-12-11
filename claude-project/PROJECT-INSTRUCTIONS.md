# VulcanOS - Claude.ai Project Instructions

## Project Description

VulcanOS is a custom Arch Linux distribution designed for the **2019 T2 MacBook Pro**. It uses:
- **Hyprland** as the Wayland compositor (tiling window manager)
- **linux-t2 kernel** from arch-mact2 repository (T2 hardware support)
- Pre-configured development environment with modern CLI tools

**GitHub Repository:** https://github.com/musickevan1/VulcanOS

---

## Current Status: Fresh Arch Install in Progress

The user (Evan) is performing a fresh Arch Linux installation on their T2 MacBook Pro using the t2linux archiso. Help them through the installation process.

---

## T2 MacBook Pro Critical Information

### Hardware Specifics
- **WiFi**: Requires `apple-bcm-firmware` package + `iwd` backend
- **Audio**: Requires `apple-t2-audio-config` package + PipeWire
- **Keyboard/Trackpad**: Works via `apple-bce` kernel module (included in linux-t2)
- **Touch Bar**: Requires `tiny-dfr` service
- **Fan Control**: Requires `t2fanrd` service
- **Touch ID**: NOT supported (T2 Secure Enclave limitation)

### Required Kernel Parameters
```
intel_iommu=on iommu=pt pcie_ports=compat
```
Add these to `GRUB_CMDLINE_LINUX_DEFAULT` in `/etc/default/grub`

### T2 Repository (add to /etc/pacman.conf)
```ini
[arch-mact2]
Server = https://mirror.funami.tech/arch-mact2/os/x86_64
SigLevel = Never
```

---

## Installation Quick Reference

### Boot from USB
1. Shut down Mac completely
2. Hold **Option (⌥)** key and press power
3. Select **"EFI Boot"** (orange/yellow icon)

### Connect to WiFi (Live Environment)
```bash
iwctl
station wlan0 connect "NetworkName"
exit
ping -c 3 archlinux.org
```

### Partition (DO NOT DELETE EFI PARTITION)
```bash
lsblk                          # Identify partitions
cfdisk /dev/nvme0n1            # Partition tool
# Create Linux partition in free space
# DO NOT touch partition 1 (EFI) or existing macOS partitions
```

### Format & Mount
```bash
mkfs.ext4 /dev/nvme0n1pX       # Format root (replace X)
mount /dev/nvme0n1pX /mnt      # Mount root
mkdir -p /mnt/boot/efi
mount /dev/nvme0n1p1 /mnt/boot/efi  # Mount EFI (DO NOT FORMAT)
```

### Install Base System
```bash
pacstrap -K /mnt base linux-t2 linux-t2-headers linux-firmware \
    networkmanager iwd sudo nano vim git base-devel nodejs npm
genfstab -U /mnt >> /mnt/etc/fstab
arch-chroot /mnt
```

### Configure System (inside chroot)
```bash
# Timezone (adjust as needed)
ln -sf /usr/share/zoneinfo/America/Los_Angeles /etc/localtime
hwclock --systohc

# Locale
echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Hostname
echo "vulcan" > /etc/hostname

# Root password
passwd

# Create user
useradd -m -G wheel -s /bin/bash evan
passwd evan

# Enable sudo
EDITOR=nano visudo
# Uncomment: %wheel ALL=(ALL:ALL) ALL

# Enable NetworkManager
systemctl enable NetworkManager
```

### Install GRUB Bootloader
```bash
pacman -S grub efibootmgr

grub-install --target=x86_64-efi --efi-directory=/boot/efi \
    --bootloader-id=GRUB --removable

# Edit GRUB config
nano /etc/default/grub
# Add to GRUB_CMDLINE_LINUX_DEFAULT: intel_iommu=on iommu=pt pcie_ports=compat

grub-mkconfig -o /boot/grub/grub.cfg
```

### Install T2 Support
```bash
# Add arch-mact2 repo to /etc/pacman.conf (see above)
nano /etc/pacman.conf

pacman -Sy apple-bcm-firmware apple-t2-audio-config t2fanrd tiny-dfr
systemctl enable t2fanrd
systemctl enable tiny-dfr
```

### Exit & Reboot
```bash
exit
umount -R /mnt
reboot
# Hold Option key → Select GRUB
```

### After First Boot
```bash
# Connect WiFi
nmcli device wifi connect "NetworkName" password "Password"

# Clone VulcanOS repo
git clone https://github.com/musickevan1/VulcanOS.git
cd VulcanOS

# Install Claude Code
sudo npm install -g @anthropic-ai/claude-code
claude
```

---

## Post-Install Configuration (Phase 2)

After base system is working, the next steps are:
1. Install Hyprland and desktop packages
2. Deploy dotfiles from VulcanOS repo using `stow`
3. Configure greetd display manager
4. Set up development environment

---

## Key Files in Repository

| File | Purpose |
|------|---------|
| `INSTALL-PROGRESS.md` | Detailed install tracker with checklist |
| `CLAUDE.md` | Full project documentation |
| `dotfiles/` | Pre-configured Hyprland, Waybar, Alacritty, etc. |
| `archiso/packages.x86_64` | Full package list for VulcanOS |
| `scripts/` | Build scripts for creating custom ISO |

---

## Common Issues & Solutions

### WiFi not working after install
```bash
# Ensure iwd is backend for NetworkManager
sudo nano /etc/NetworkManager/conf.d/iwd.conf
# Add:
# [device]
# wifi.backend=iwd

sudo systemctl enable --now iwd
sudo systemctl restart NetworkManager
```

### No audio
```bash
# Verify T2 audio config is installed
pacman -S apple-t2-audio-config pipewire pipewire-pulse wireplumber
systemctl --user enable --now pipewire pipewire-pulse wireplumber
```

### GRUB not showing at boot
- Hold Option key during boot
- Select GRUB/Linux entry
- If not visible, may need to re-run grub-install with `--removable` flag

### Touch Bar not working
```bash
sudo systemctl enable --now tiny-dfr
```

---

## User Info

- **Username**: evan
- **Hostname**: vulcan
- **Shell**: bash
- **Timezone**: America/Los_Angeles (adjust if different)
