# VulcanOS Installation Guide

Complete guide for installing VulcanOS on Apple T2 MacBook Pro (2019).

## Table of Contents

1. [Pre-Installation (macOS)](#pre-installation-macos)
2. [Creating Installation Media](#creating-installation-media)
3. [Booting the Live Environment](#booting-the-live-environment)
4. [Installation Process](#installation-process)
5. [Post-Installation](#post-installation)
6. [Troubleshooting](#troubleshooting)

---

## Pre-Installation (macOS)

### 1. Disable Secure Boot

**Important:** This must be done from macOS Recovery.

1. Shut down your Mac completely
2. Press and hold **Command (⌘) + R** immediately after pressing the power button
3. Release when you see the Apple logo or spinning globe
4. Wait for macOS Recovery to load
5. From the menu bar, select **Utilities > Startup Security Utility**
6. Authenticate with your admin password
7. Set **Secure Boot** to **"No Security"**
8. Set **External Boot** to **"Allow booting from external media"**
9. Close the utility and restart

### 2. Create Partition for Linux

Option A: Using Disk Utility (Recommended)
1. Boot into macOS normally
2. Open **Disk Utility** (Applications > Utilities)
3. Select your main drive (usually "Macintosh HD")
4. Click **Partition**
5. Click **+** to add a partition
6. Set size (minimum 50GB recommended, 100GB+ for comfortable usage)
7. Format as **Mac OS Extended** (we'll reformat during Linux install)
8. Name it "Linux" for easy identification
9. Click **Apply**

Option B: Keep existing partition layout
- You can partition during Linux installation, but pre-partitioning from macOS is safer

### 3. Backup Important Data

Before proceeding:
- Back up all important macOS data
- Note your macOS partition sizes
- Export WiFi passwords (you'll need them for Linux)

### 4. Note Important Information

Write down:
- Your timezone
- Keyboard layout
- Network credentials (WiFi SSID and password)
- Desired username and password

---

## Creating Installation Media

### Requirements

- USB drive (8GB minimum, 16GB recommended)
- VulcanOS ISO file
- Another computer or dual-boot capability

### Writing the ISO

**On Linux:**
```bash
sudo dd if=newos-*.iso of=/dev/sdX bs=4M status=progress oflag=sync
```
Replace `/dev/sdX` with your USB device (check with `lsblk`).

**On macOS:**
```bash
# Find your USB device
diskutil list

# Unmount it
diskutil unmountDisk /dev/diskN

# Write the ISO
sudo dd if=newos-*.iso of=/dev/rdiskN bs=4m
```
Replace `diskN` with your USB disk number.

**On Windows:**
Use [Rufus](https://rufus.ie/) or [Etcher](https://etcher.io/) to write the ISO.

---

## Booting the Live Environment

### 1. Boot from USB

1. Shut down your Mac
2. Insert the USB drive
3. Press and hold **Option (⌥)** key
4. Press the power button
5. Keep holding Option until you see the boot selection screen
6. Select the USB drive (usually labeled "EFI Boot")
7. Press Enter

### 2. Select Boot Option

From the bootloader menu (GRUB or SYSLINUX):
- **VulcanOS Live (T2 MacBook Pro)** - Standard boot
- **VulcanOS Live (Copy to RAM)** - Loads entire ISO to RAM (faster, requires more RAM)

### 3. Verify Hardware

Once booted into the live environment:

**Check WiFi:**
```bash
iwctl station wlan0 scan
iwctl station wlan0 get-networks
```

**Check Audio:**
```bash
aplay -l
# Should show "AppleT2xN"
```

**Check Keyboard/Trackpad:**
- They should work out of the box

---

## Installation Process

### Option 1: Guided CLI Installation

1. Open terminal (Super + Return)
2. Run the installer:
```bash
sudo newos-install
```
3. Follow the prompts:
   - Select disk
   - Choose partitioning scheme
   - Set timezone
   - Create user account
   - Set passwords

### Option 2: Manual Installation (Advanced)

#### A. Connect to Internet

```bash
# List networks
iwctl station wlan0 scan
iwctl station wlan0 get-networks

# Connect
iwctl station wlan0 connect "YOUR_SSID"
# Enter password when prompted

# Verify
ping -c 3 archlinux.org
```

#### B. Partition the Disk

```bash
# List disks
lsblk

# Partition with fdisk (example for /dev/nvme0n1)
fdisk /dev/nvme0n1

# Keep existing EFI partition (usually /dev/nvme0n1p1)
# Delete the Linux partition you created in macOS
# Create new partitions:
#   - Root partition (ext4, rest of Linux space)
#   - Swap partition (optional, size of RAM or 8GB)
```

**Example partition layout:**
```
/dev/nvme0n1p1  300M   EFI System (keep existing)
/dev/nvme0n1p2  xxxG   macOS (keep existing)
/dev/nvme0n1p3  8G     Linux swap
/dev/nvme0n1p4  rest   Linux filesystem
```

#### C. Format Partitions

```bash
# DO NOT format the EFI partition!

# Format root partition
mkfs.ext4 /dev/nvme0n1p4

# Setup swap
mkswap /dev/nvme0n1p3
swapon /dev/nvme0n1p3
```

#### D. Mount Partitions

```bash
# Mount root
mount /dev/nvme0n1p4 /mnt

# Mount EFI
mount --mkdir /dev/nvme0n1p1 /mnt/boot
```

#### E. Install Base System

```bash
# Install packages
pacstrap -K /mnt base linux-t2 linux-t2-headers linux-firmware base-devel

# Generate fstab
genfstab -U /mnt >> /mnt/etc/fstab
```

#### F. Configure System

```bash
# Chroot into new system
arch-chroot /mnt

# Set timezone
ln -sf /usr/share/zoneinfo/America/New_York /etc/localtime
hwclock --systohc

# Set locale
echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf

# Set hostname
echo "newos" > /etc/hostname

# Set root password
passwd

# Create user
useradd -m -G wheel,audio,video,docker -s /bin/bash username
passwd username

# Enable sudo for wheel group
EDITOR=nano visudo
# Uncomment: %wheel ALL=(ALL:ALL) ALL

# Add arch-mact2 repository to /etc/pacman.conf
nano /etc/pacman.conf
# Add before [core]:
# [arch-mact2]
# Server = https://mirror.funami.tech/arch-mact2/os/x86_64
# SigLevel = Never

# Update and install packages
pacman -Syu
pacman -S apple-bcm-firmware apple-t2-audio-config t2fanrd tiny-dfr
pacman -S networkmanager grub efibootmgr
pacman -S hyprland waybar wofi kitty starship # ... and other desired packages

# Configure mkinitcpio
nano /etc/mkinitcpio.conf
# Add to MODULES: apple-bce
mkinitcpio -P

# Install GRUB
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB --removable

# Configure GRUB
nano /etc/default/grub
# Add to GRUB_CMDLINE_LINUX:
# intel_iommu=on iommu=pt pcie_ports=compat

# Generate GRUB config
grub-mkconfig -o /boot/grub/grub.cfg

# Enable services
systemctl enable iwd
systemctl enable greetd
systemctl enable t2fanrd
systemctl enable bluetooth
systemctl enable docker

# Exit chroot
exit

# Unmount
umount -R /mnt

# Reboot
reboot
```

---

## Post-Installation

### 1. First Boot

1. Remove USB drive
2. Hold **Option (⌥)** at startup
3. Select **GRUB** or your Linux partition
4. Log in with your username and password

### 2. Connect to WiFi

```bash
iwctl
# station wlan0 scan
# station wlan0 get-networks
# station wlan0 connect "Your-Network-Name"
```

### 3. Update System

```bash
sudo pacman -Syu
```

### 4. Install Additional Software

```bash
# Development tools
sudo pacman -S git neovim code rustup go python nodejs npm

# Desktop apps
sudo pacman -S firefox gimp inkscape blender
```

### 5. Configure Audio

Audio should work automatically with PipeWire. If not:

```bash
# Verify audio card
aplay -l

# Should show AppleT2xN
# If not, check kernel parameters
cat /proc/cmdline
# Should include: intel_iommu=on iommu=pt pcie_ports=compat

# Reinstall audio config
sudo pacman -S apple-t2-audio-config
```

### 6. Configure Touch Bar (Optional)

```bash
# Configure tiny-dfr
sudo cp /usr/share/tiny-dfr/config.toml /etc/tiny-dfr/config.toml
sudo nano /etc/tiny-dfr/config.toml

# Customize and restart
sudo systemctl restart tiny-dfr
```

### 7. Set Up Development Environment

```bash
# Initialize Rust
rustup default stable
rustup component add rust-src rust-analyzer

# Configure Git
git config --global user.name "Your Name"
git config --global user.email "your@email.com"

# Starship prompt is pre-configured in VulcanOS
# If needed, initialize manually:
starship init bash >> ~/.bashrc
source ~/.bashrc
```

### 8. Install Syntax Highlighting (Optional)

VulcanOS supports **ble.sh** for real-time command syntax highlighting. This is an AUR package:

```bash
# Install yay (AUR helper) if not already installed
sudo pacman -S --needed git base-devel
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si

# Install ble.sh for syntax highlighting
yay -S blesh

# ble.sh is automatically configured in ~/.bashrc
# Restart your terminal to see syntax highlighting
```

**What ble.sh provides:**
- Real-time syntax highlighting as you type
- Auto-suggestions based on history
- Fish-like command completion
- Error highlighting before you press Enter

**Note:** ble.sh is optional. The terminal works perfectly without it.

### 9. Terminal Keybindings

VulcanOS uses **Kitty** as the default terminal with these keybindings:

| Keybinding | Action |
|------------|--------|
| Ctrl+Shift+T | New tab |
| Ctrl+Shift+Q | Close tab |
| Ctrl+1-9 | Switch to tab 1-9 |
| Ctrl+Shift+Enter | New window (split) |
| Ctrl+Shift+H | Show scrollback in pager |
| Ctrl+Shift+= / - | Increase/decrease font size |

---

## Troubleshooting

### WiFi Not Working

**Symptom:** No wireless networks found

**Solution:**
1. Verify firmware is installed:
```bash
ls /lib/firmware/brcm/ | grep -i 4377
```

2. If missing, extract from macOS:
```bash
# Mount macOS partition
sudo mount -t hfsplus /dev/nvme0n1p2 /mnt/macos
# Run firmware extraction script
```

3. Ensure iwd is running:
```bash
sudo systemctl enable --now iwd
iwctl station wlan0 scan
iwctl station wlan0 get-networks
```

### No Audio Output

**Symptom:** No sound or no audio device

**Solution:**
1. Check kernel parameters:
```bash
cat /proc/cmdline
# Must include: intel_iommu=on iommu=pt pcie_ports=compat
```

2. Check apple-bce module:
```bash
lsmod | grep apple_bce
# If not loaded:
sudo modprobe apple-bce
```

3. Verify audio device:
```bash
aplay -l
# Should show AppleT2xN
```

### Trackpad Not Working

**Symptom:** Trackpad doesn't respond

**Solution:**
1. Check if apple-bce is loaded:
```bash
lsmod | grep apple_bce
```

2. Verify USB devices:
```bash
lsusb
# Should show Apple devices
```

3. Check dmesg for errors:
```bash
dmesg | grep -i apple
```

### Cannot Boot After Installation

**Symptom:** Mac doesn't see Linux bootloader

**Solution:**
1. Boot into live USB
2. Reinstall GRUB with --removable flag:
```bash
sudo mount /dev/nvme0n1p4 /mnt
sudo mount /dev/nvme0n1p1 /mnt/boot
sudo arch-chroot /mnt
grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB --removable
exit
```

3. Try systemd-boot as alternative:
```bash
bootctl --path=/boot install
```

### Suspend Issues

**Symptom:** System crashes after suspend/resume

**Solution:**
Create suspend fix service:
```bash
sudo nano /etc/systemd/system/suspend-fix-t2.service
```

```ini
[Unit]
Description=Fix T2 suspend issues
Before=sleep.target

[Service]
Type=oneshot
ExecStart=/usr/bin/rmmod -f apple-bce
ExecStop=/usr/bin/modprobe apple-bce
RemainAfterExit=yes

[Install]
WantedBy=sleep.target
```

```bash
sudo systemctl enable suspend-fix-t2.service
```

**Note:** Touch Bar may not work after resume with this workaround.

---

## Additional Resources

- **t2linux Wiki:** https://wiki.t2linux.org/
- **Arch Wiki:** https://wiki.archlinux.org/
- **Hyprland Wiki:** https://wiki.hyprland.org/
- **VulcanOS Repository:** (your GitHub URL)

---

## Getting Help

If you encounter issues:

1. Check the troubleshooting section above
2. Search the t2linux wiki and Discord
3. Open an issue on the VulcanOS GitHub repository
4. Provide:
   - Hardware model (System Information in macOS)
   - Error messages (from `dmesg`, `journalctl`)
   - Steps to reproduce
