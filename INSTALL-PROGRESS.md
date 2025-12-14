# VulcanOS Fresh Install Progress Tracker

**Started:** 2025-12-11
**Target:** 2019 T2 MacBook Pro
**Method:** Fresh install via t2linux Arch ISO

## Current Step: Step 1 - Pre-Installation (macOS Recovery)

---

## Checklist Overview

### Phase 1: Pre-Installation (macOS Side)
- [ ] Backup verified (Google Drive + Flash drive)
- [ ] Disable Secure Boot (macOS Recovery)
- [ ] Enable external boot (macOS Recovery)
- [ ] Create Linux partition (Disk Utility or diskutil)
- [ ] Download t2linux Arch ISO
- [ ] Flash ISO to USB drive

### Phase 2: Boot & Install Base System
- [ ] Boot from USB (hold Option key)
- [ ] Connect to WiFi (iwctl)
- [ ] Partition disks (keep EFI intact!)
- [ ] Format partitions
- [ ] Mount partitions
- [ ] Pacstrap base system with linux-t2
- [ ] Generate fstab
- [ ] Chroot into new system

### Phase 3: System Configuration
- [ ] Set timezone
- [ ] Set locale
- [ ] Set hostname
- [ ] Create user account
- [ ] Install and configure GRUB
- [ ] Enable iwd
- [ ] Install T2 packages (firmware, audio, fan control, Touch Bar)

### Phase 4: Desktop Environment (VulcanOS Style)
- [ ] Install Hyprland and dependencies
- [ ] Install greetd display manager
- [ ] Configure dotfiles
- [ ] Install development packages
- [ ] Configure themes and appearance

---

## Detailed Steps

### Step 1: Pre-Installation (macOS Recovery)

**1.1 Disable Secure Boot**
1. Shut down Mac completely
2. Press and hold `Cmd + R` while pressing power button
3. Keep holding until you see Apple logo or spinning globe
4. Once in Recovery, go to **Utilities → Startup Security Utility**
5. Set Secure Boot to **"No Security"**
6. Check **"Allow booting from external media"**

**1.2 Create Linux Partition**
Option A - From Recovery Disk Utility:
- Select internal drive
- Click "Partition"
- Add new partition (recommend 100GB+ for Linux)
- Format as "Free Space" or "MS-DOS (FAT)"

Option B - From macOS Terminal:
```bash
# List disks to find your internal drive
diskutil list

# Resize APFS container (example: shrink by 100GB)
# Replace disk0s2 with your APFS container identifier
sudo diskutil apfs resizeContainer disk0s2 0b -100GB
```

**1.3 Download t2linux Arch ISO**
- URL: https://github.com/t2linux/archiso-t2/releases
- Get the latest `.iso` file

**1.4 Flash ISO to USB**
From macOS:
```bash
# Find USB drive
diskutil list

# Unmount USB (replace diskX)
diskutil unmountDisk /dev/diskX

# Flash ISO (replace diskX and path to ISO)
sudo dd if=/path/to/archlinux-t2-*.iso of=/dev/rdiskX bs=4m status=progress
```

---

### Step 2: Boot & Connect

**2.1 Boot from USB**
1. Shut down Mac
2. Press and hold **Option (⌥)** key
3. Press power button while holding Option
4. Select "EFI Boot" (the orange/yellow drive icon)

**2.2 Connect to WiFi**
```bash
# Start iwctl
iwctl

# Inside iwctl:
device list
station wlan0 scan
station wlan0 get-networks
station wlan0 connect "YourNetworkName"
# Enter password when prompted
exit

# Verify connection
ping -c 3 archlinux.org
```

---

### Step 3: Partition & Format

**CRITICAL: Do NOT delete or format the EFI partition! macOS needs it.**

```bash
# List current partitions
lsblk

# Identify:
# - EFI partition (usually /dev/nvme0n1p1, ~200MB, type EFI)
# - macOS partition (APFS, large)
# - Your new Linux space (unallocated or the partition you created)

# Use fdisk or cfdisk to create Linux partitions
cfdisk /dev/nvme0n1

# Create:
# 1. Root partition (/) - remaining space, type "Linux filesystem"
# 2. (Optional) Swap partition - 8-16GB, type "Linux swap"

# Format partitions
mkfs.ext4 /dev/nvme0n1pX      # Replace X with root partition number
mkswap /dev/nvme0n1pY         # Replace Y with swap partition number (if created)
```

---

### Step 4: Mount & Install

```bash
# Mount root partition
mount /dev/nvme0n1pX /mnt

# Mount EFI partition (DO NOT FORMAT)
mkdir -p /mnt/boot/efi
mount /dev/nvme0n1p1 /mnt/boot/efi

# Enable swap (if created)
swapon /dev/nvme0n1pY

# Install base system with T2 kernel
pacstrap -K /mnt base linux-t2 linux-t2-headers linux-firmware \
    networkmanager iwd sudo nano vim git base-devel

# Generate fstab
genfstab -U /mnt >> /mnt/etc/fstab

# Verify fstab looks correct
cat /mnt/etc/fstab
```

---

### Step 5: Chroot & Configure

```bash
# Enter new system
arch-chroot /mnt

# Set timezone
ln -sf /usr/share/zoneinfo/America/Los_Angeles /etc/localtime
hwclock --systohc

# Set locale
echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set hostname
echo "vulcan" > /etc/hostname

# Set root password
passwd

# Create user
useradd -m -G wheel -s /bin/bash evan
passwd evan

# Enable sudo for wheel group
EDITOR=nano visudo
# Uncomment: %wheel ALL=(ALL:ALL) ALL

# Enable iwd
systemctl enable iwd
```

---

### Step 6: Install GRUB

```bash
# Install GRUB packages
pacman -S grub efibootmgr os-prober

# Install GRUB to EFI
grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=GRUB --removable

# Edit GRUB config for T2
nano /etc/default/grub
# Add to GRUB_CMDLINE_LINUX_DEFAULT:
# intel_iommu=on iommu=pt pcie_ports=compat

# Generate GRUB config
grub-mkconfig -o /boot/grub/grub.cfg
```

---

### Step 7: T2 Hardware Support

```bash
# Add arch-mact2 repository to pacman.conf
nano /etc/pacman.conf
# Add at the end:
# [arch-mact2]
# Server = https://mirror.funami.tech/arch-mact2/os/x86_64
# SigLevel = Never

# Refresh package database
pacman -Sy

# Install T2 packages
pacman -S apple-bcm-firmware apple-t2-audio-config t2fanrd tiny-dfr

# Enable T2 services
systemctl enable t2fanrd
systemctl enable tiny-dfr
```

---

### Step 8: Exit & Reboot

```bash
# Exit chroot
exit

# Unmount
umount -R /mnt

# Reboot
reboot
```

**Hold Option key during reboot to select GRUB/Linux**

---

## Decisions Made

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Install method | Fresh install | Clean slate |
| ISO source | t2linux Arch ISO | T2 hardware support |
| Kernel | linux-t2 | Required for T2 hardware |

## Notes

- Keep macOS EFI partition intact for dual-boot capability
- Use `--removable` flag with grub-install for T2 compatibility
- WiFi requires apple-bcm-firmware package

---

## Quick Reference Card (Print This!)

```
┌─────────────────────────────────────────────────────────┐
│  VULCANOS INSTALL QUICK REFERENCE                       │
├─────────────────────────────────────────────────────────┤
│  BOOT: Hold Option (⌥) key → Select EFI Boot            │
├─────────────────────────────────────────────────────────┤
│  WIFI:                                                  │
│    iwctl                                                │
│    station wlan0 connect "NETWORK_NAME"                 │
│    exit                                                 │
├─────────────────────────────────────────────────────────┤
│  PARTITIONS (use cfdisk /dev/nvme0n1):                  │
│    - DO NOT touch partition 1 (EFI) or 2 (macOS)        │
│    - Create new Linux partition in free space           │
├─────────────────────────────────────────────────────────┤
│  INSTALL:                                               │
│    mkfs.ext4 /dev/nvme0n1pX                             │
│    mount /dev/nvme0n1pX /mnt                            │
│    mkdir -p /mnt/boot/efi                               │
│    mount /dev/nvme0n1p1 /mnt/boot/efi                   │
│    pacstrap -K /mnt base linux-t2 linux-t2-headers \    │
│      linux-firmware networkmanager iwd sudo nano git    │
│      base-devel nodejs npm                              │
│    genfstab -U /mnt >> /mnt/etc/fstab                   │
│    arch-chroot /mnt                                     │
├─────────────────────────────────────────────────────────┤
│  IN CHROOT:                                             │
│    # Timezone (adjust for your location)                │
│    ln -sf /usr/share/zoneinfo/America/Los_Angeles \     │
│      /etc/localtime                                     │
│    hwclock --systohc                                    │
│                                                         │
│    # Locale                                             │
│    echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen          │
│    locale-gen                                           │
│    echo "LANG=en_US.UTF-8" > /etc/locale.conf           │
│                                                         │
│    # Hostname                                           │
│    echo "vulcan" > /etc/hostname                        │
│                                                         │
│    # Root password                                      │
│    passwd                                               │
│                                                         │
│    # Create user                                        │
│    useradd -m -G wheel -s /bin/bash evan                │
│    passwd evan                                          │
│                                                         │
│    # Sudo                                               │
│    EDITOR=nano visudo                                   │
│    # Uncomment: %wheel ALL=(ALL:ALL) ALL                │
│                                                         │
│    # iwd (network)                                      │
│    systemctl enable iwd                                 │
├─────────────────────────────────────────────────────────┤
│  GRUB:                                                  │
│    pacman -S grub efibootmgr                            │
│    grub-install --target=x86_64-efi \                   │
│      --efi-directory=/boot/efi \                        │
│      --bootloader-id=GRUB --removable                   │
│                                                         │
│    nano /etc/default/grub                               │
│    # Add to GRUB_CMDLINE_LINUX_DEFAULT:                 │
│    # intel_iommu=on iommu=pt pcie_ports=compat          │
│                                                         │
│    grub-mkconfig -o /boot/grub/grub.cfg                 │
├─────────────────────────────────────────────────────────┤
│  T2 REPO (add to /etc/pacman.conf):                     │
│    [arch-mact2]                                         │
│    Server = https://mirror.funami.tech/arch-mact2/os/x86_64 │
│    SigLevel = Never                                     │
│                                                         │
│    pacman -Sy apple-bcm-firmware apple-t2-audio-config  │
│      t2fanrd tiny-dfr                                   │
│    systemctl enable t2fanrd tiny-dfr                    │
├─────────────────────────────────────────────────────────┤
│  EXIT & REBOOT:                                         │
│    exit                                                 │
│    umount -R /mnt                                       │
│    reboot                                               │
│    # Hold Option key → Select GRUB                      │
├─────────────────────────────────────────────────────────┤
│  AFTER FIRST BOOT - GET CLAUDE CODE:                    │
│    nmcli device wifi connect "NETWORK" password "PASS"  │
│    sudo npm install -g @anthropic-ai/claude-code        │
│    claude                                               │
└─────────────────────────────────────────────────────────┘
```

## Next Session Resume Point

**If conversation context is lost, share this file and say:**
> "Continuing VulcanOS install, currently on Step X"
