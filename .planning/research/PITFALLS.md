# Domain Pitfalls: T2 MacBook Kernel Updates & System Backups

**Domain:** Arch Linux system protection on T2 MacBook Pro (2019)
**Researched:** 2026-01-23
**Overall confidence:** HIGH (verified with official t2linux docs + Arch community sources)

---

## Critical Pitfalls

Mistakes that cause rewrites, data loss, or unbootable systems.

### Pitfall 1: Wrong Kernel Installed During Update

**What goes wrong:**
Pacman updates to mainline `linux` kernel from official Arch repos instead of `linux-t2` from arch-mact2. System becomes completely unbootable with no keyboard/trackpad/WiFi access in recovery.

**Why it happens:**
- User runs `pacman -Syu` without `IgnorePkg` configured
- Mainline kernel gets higher priority than T2-patched kernel
- Pacman doesn't know the system requires T2-specific patches
- The `linux` package appears as a "newer" update than `linux-t2`

**Consequences:**
- Boot succeeds but no hardware works (keyboard, trackpad, WiFi, audio, Touch Bar)
- Cannot access terminal to fix the issue
- Requires external USB keyboard/mouse + live USB to chroot and reinstall `linux-t2`
- On encrypted systems with LUKS, cannot even type password without USB keyboard

**Prevention:**
```bash
# In /etc/pacman.conf, add to [options] section:
IgnorePkg = linux linux-headers linux-lts linux-lts-headers

# Only allow T2 kernels to update
# Explicitly update T2 kernel when needed:
sudo pacman -S linux-t2 linux-t2-headers
```

**Detection warning signs:**
- Pacman output shows `linux` package being upgraded (not `linux-t2`)
- Kernel version changes from `*-t2-*` to regular version number
- After reboot: GRUB shows kernel entries without `-t2` suffix

**Recovery steps:**
1. Boot from VulcanOS live USB (or any T2-compatible live ISO)
2. Mount your encrypted root partition:
   ```bash
   cryptsetup open /dev/nvme0n1p3 cryptroot
   mount /dev/mapper/cryptroot /mnt
   mount /dev/nvme0n1p1 /mnt/boot  # Critical: mount boot!
   ```
3. Chroot into broken system:
   ```bash
   arch-chroot /mnt
   ```
4. Reinstall T2 kernel:
   ```bash
   pacman -S linux-t2 linux-t2-headers
   # Verify mkinitcpio ran: check /boot for new vmlinuz-linux-t2
   ```
5. Update GRUB:
   ```bash
   grub-mkconfig -o /boot/grub/grub.cfg
   ```
6. Exit chroot, unmount, reboot

**Phase assignment:** Phase 1 (Pre-flight checks) — prevent before first update

---

### Pitfall 2: /boot Partition Not Mounted During Kernel Update

**What goes wrong:**
Kernel update succeeds in pacman's database, but kernel files aren't copied to EFI partition. System boots to old kernel (if lucky) or fails to boot entirely.

**Why it happens:**
- `/boot` unmounted manually for backup operations
- Systemd automount not configured
- User forgets to check before running `pacman -Syu`
- Backup script unmounts `/boot` and doesn't remount before update
- Power loss during backup while `/boot` unmounted

**Consequences:**
- New kernel exists in `/usr/lib/modules/` but not in `/boot`
- GRUB can't find kernel to boot
- System drops to emergency shell or GRUB rescue mode
- Mismatch between installed modules and booted kernel causes driver failures

**Prevention:**
```bash
# Add pre-update check to ~/.bashrc or system-wide hook:
alias pacman-update='mountpoint -q /boot && sudo pacman -Syu || echo "ERROR: /boot not mounted!"'

# Or create pacman hook at /etc/pacman.d/hooks/00-check-boot.hook:
[Trigger]
Operation = Upgrade
Type = Package
Target = linux-t2

[Action]
Description = Verifying /boot is mounted before kernel update...
When = PreTransaction
Exec = /usr/bin/mountpoint -q /boot
AbortOnFail
```

**Detection warning signs:**
- Running `lsblk` shows `/boot` without mount point
- `mount | grep boot` returns nothing
- `df -h` doesn't show EFI partition
- After kernel update: `ls /boot` shows old kernel timestamp

**Recovery steps:**
1. Boot from live USB
2. Mount partitions **in correct order**:
   ```bash
   cryptsetup open /dev/nvme0n1p3 cryptroot
   mount /dev/mapper/cryptroot /mnt
   mount /dev/nvme0n1p1 /mnt/boot  # THIS IS THE CRITICAL STEP
   ```
3. Chroot and reinstall kernel:
   ```bash
   arch-chroot /mnt
   pacman -S linux-t2  # Triggers mkinitcpio hook again
   # Verify: ls -lh /boot should show new vmlinuz-linux-t2
   grub-mkconfig -o /boot/grub/grub.cfg
   ```

**Phase assignment:** Phase 1 (Pre-flight checks) — automated verification

---

### Pitfall 3: Backup to External Drive While System Running Without Snapshots

**What goes wrong:**
Using `rsync` to backup a running system creates inconsistent filesystem state. Database files, log files, and application state become corrupted in backup. Restore fails or produces broken system.

**Why it happens:**
- Naive `rsync -av / /mnt/backup` while applications running
- No filesystem snapshots (ext4 doesn't support atomic snapshots natively)
- Files change mid-backup (databases write, logs append, package manager runs)
- Incremental backups capture partial state transitions

**Consequences:**
- Backup "succeeds" but is unusable for restore
- Package database (`/var/lib/pacman`) corrupted in backup
- Systemd journal files inconsistent
- User dotfiles in inconsistent state if modified during backup
- **Worse:** You don't discover this until disaster recovery fails

**Prevention:**

**Option A: Stop critical services (simple but intrusive)**
```bash
# Before backup:
systemctl stop docker postgresql redis  # Stop databases
# Run backup
# Restart services
```

**Option B: Use LVM snapshots (if using LVM - VulcanOS doesn't by default)**
```bash
lvcreate -L 5G -s -n root-snapshot /dev/vg0/root
mount /dev/vg0/root-snapshot /mnt/snapshot
rsync -aAXv /mnt/snapshot/ /mnt/backup/
umount /mnt/snapshot
lvremove /dev/vg0/root-snapshot
```

**Option C: Migrate to Btrfs (best long-term solution)**
- Allows instant read-only snapshots
- Zero-copy snapshots consume minimal space
- Timeshift can automate snapshot-based backups
- **Tradeoff:** Requires filesystem conversion (destructive operation)

**Option D: Exclude volatile paths (pragmatic approach for ext4)**
```bash
rsync -aAXv \
  --exclude='/dev/*' \
  --exclude='/proc/*' \
  --exclude='/sys/*' \
  --exclude='/tmp/*' \
  --exclude='/run/*' \
  --exclude='/mnt/*' \
  --exclude='/media/*' \
  --exclude='/lost+found' \
  --exclude='/var/cache/pacman/pkg/*' \
  --exclude='/var/lib/docker/overlay2/*' \
  --exclude='/home/*/.cache/*' \
  / /mnt/backup/
```

**Detection warning signs:**
- Backup runs slowly (> 30 minutes for typical system)
- Multiple passes show different file counts
- Restored system has pacman database errors
- Systemd-journald fails on restored system

**Recovery:**
If backup is already corrupted:
1. **DO NOT trust the backup for full restore**
2. Use it only for recovering `/home` user data
3. Rebuild system from fresh ISO
4. Restore user files selectively after reinstall

**Phase assignment:** Phase 2 (Backup strategy) — implement snapshot-aware backups

---

### Pitfall 4: External Backup Drive Disconnects During rsync

**What goes wrong:**
USB controller issues, power management, or cable problems cause external SSD to disconnect mid-backup. Rsync fails silently or reports "Input/output error". User thinks backup succeeded.

**Why it happens:**
- **USB controller buffer overflow:** `xhci_hcd swiotlb buffer is full` kernel error
- **Aggressive power management:** USB autosuspend kicks in during long rsync
- **Loose USB-C cable:** Especially on T2 MacBooks with worn ports
- **Insufficient power delivery:** USB-powered SSDs brown out during heavy writes
- **Filesystem mounted async:** Buffer cache not flushed before disconnect
- **T2 MacBook-specific:** Thunderbolt/USB-C controller quirks under Linux

**Consequences:**
- Partial backup looks complete (directory structure exists)
- Files truncated at moment of disconnect
- Ext4 filesystem on external drive becomes corrupted
- **Worse:** Cron job reports success but backup is incomplete
- Only discovered during emergency restore attempt

**Prevention:**

**1. Disable USB autosuspend for backup drive:**
```bash
# Find USB device ID:
lsusb
# Example output: Bus 001 Device 003: ID 152d:0583 JMicron Technology

# Disable autosuspend permanently:
echo 'ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="152d", ATTR{idProduct}=="0583", ATTR{power/autosuspend}="-1"' | sudo tee /etc/udev/rules.d/50-usb-backup-no-autosuspend.rules

# Reload udev:
sudo udevadm control --reload-rules
```

**2. Use sync mount option:**
```bash
# In /etc/fstab for backup partition:
UUID=xxx /mnt/backup ext4 defaults,sync 0 2

# Or mount manually with sync:
mount -o sync /dev/sda1 /mnt/backup
```

**3. Verify drive stays connected:**
```bash
#!/bin/bash
# backup-with-verification.sh

BACKUP_DEV="/dev/disk/by-uuid/YOUR-BACKUP-UUID"
BACKUP_MNT="/mnt/backup"

# Check drive is present
if [ ! -e "$BACKUP_DEV" ]; then
  echo "ERROR: Backup drive not found!"
  exit 1
fi

# Mount if not already mounted
mountpoint -q "$BACKUP_MNT" || mount "$BACKUP_MNT"

# Run rsync with verification
rsync -aAXv --checksum / "$BACKUP_MNT/" 2>&1 | tee /var/log/backup-last.log

# Check exit status
if [ ${PIPESTATUS[0]} -ne 0 ]; then
  echo "ERROR: rsync failed! Check /var/log/backup-last.log"
  notify-send "Backup FAILED" "Check logs immediately"
  exit 1
fi

# Verify drive still connected after rsync
if [ ! -e "$BACKUP_DEV" ]; then
  echo "CRITICAL: Backup drive disconnected during rsync!"
  notify-send "Backup CORRUPTED" "Drive disconnected during backup!"
  exit 1
fi

# Force filesystem sync
sync

echo "Backup completed successfully"
notify-send "Backup Complete" "System backup finished"
```

**4. Monitor kernel logs during backup:**
```bash
# In separate terminal while backup runs:
journalctl -f | grep -E 'usb|xhci|I/O error'

# Warning signs:
# - "xhci_hcd ... swiotlb buffer is full"
# - "usb ... device descriptor read/64, error -110"
# - "I/O error, dev sda, sector XXX"
```

**Detection warning signs:**
- Backup completes in suspiciously short time
- File count in backup doesn't match source
- `dmesg` shows USB disconnect events during backup window
- External drive LED stopped blinking mid-backup
- Rsync exit code non-zero but cron reports success

**Recovery:**
If drive disconnected during backup:
1. **DO NOT use this backup for restore**
2. Run filesystem check on external drive:
   ```bash
   umount /mnt/backup
   fsck.ext4 -f /dev/sda1
   ```
3. Delete corrupted backup
4. Fix underlying issue (USB autosuspend, cable, power)
5. Re-run full backup with monitoring

**Phase assignment:** Phase 2 (Backup strategy) — robust external drive handling

---

### Pitfall 5: No LTS/Fallback Kernel Configured in GRUB

**What goes wrong:**
New `linux-t2` kernel has regression (WiFi breaks, suspend fails, display issues). No fallback kernel in GRUB menu. User stuck with broken system.

**Why it happens:**
- Default Arch install only keeps one kernel
- GRUB configuration doesn't expose old kernel entries
- User removes old kernel with `pacman -Sc` after update
- T2 patches sometimes introduce hardware regressions
- Upstream kernel changes break T2-specific drivers

**Consequences:**
- Cannot boot to working kernel
- Forced to use live USB for every kernel regression
- WiFi/Bluetooth broken = cannot download older kernel
- No quick rollback path for testing new kernels
- Extended downtime for critical work

**Prevention:**

**1. Install LTS kernel as fallback (if T2 LTS exists):**
```bash
# Check if arch-mact2 provides LTS:
pacman -Ss linux-t2-lts

# If available:
pacman -S linux-t2-lts linux-t2-lts-headers
grub-mkconfig -o /boot/grub/grub.cfg
```

**2. Keep previous kernel version:**
```bash
# Configure pacman to keep old kernel packages:
# In /etc/pacman.conf:
CleanMethod = KeepCurrent

# Or manually before removing old kernel:
# 1. After update, DON'T run: pacman -Sc
# 2. Old vmlinuz stays in /boot
# 3. GRUB will show both kernel versions
```

**3. Configure GRUB to show menu:**
```bash
# Edit /etc/default/grub:
GRUB_TIMEOUT=5           # Show menu for 5 seconds
GRUB_TIMEOUT_STYLE=menu  # Always show menu
# Remove quiet from: GRUB_CMDLINE_LINUX_DEFAULT
# (so you can see boot errors)

# Regenerate config:
grub-mkconfig -o /boot/grub/grub.cfg
```

**4. Manual kernel preservation:**
```bash
# Before kernel update, backup current working kernel:
cp /boot/vmlinuz-linux-t2 /boot/vmlinuz-linux-t2.backup
cp /boot/initramfs-linux-t2.img /boot/initramfs-linux-t2-backup.img

# Create custom GRUB entry in /boot/grub/custom.cfg:
menuentry "Arch Linux (T2 Backup Kernel)" {
    linux /vmlinuz-linux-t2.backup root=/dev/mapper/cryptroot intel_iommu=on iommu=pt pcie_ports=compat
    initrd /initramfs-linux-t2-backup.img
}
```

**Detection warning signs:**
- GRUB menu only shows one kernel entry
- No "Advanced options" submenu
- After kernel update: old kernel files removed from `/boot`
- `pacman -Q | grep linux` shows only one kernel package

**Recovery:**
If new kernel is broken and no fallback:
1. Boot from live USB
2. Chroot into system
3. Downgrade to previous kernel:
   ```bash
   # Check pacman cache for old kernel:
   ls /var/cache/pacman/pkg/ | grep linux-t2

   # Install old version:
   pacman -U /var/cache/pacman/pkg/linux-t2-6.X.X-1-x86_64.pkg.tar.zst

   # Prevent auto-update temporarily:
   echo "IgnorePkg = linux-t2 linux-t2-headers" >> /etc/pacman.conf
   ```

**Phase assignment:** Phase 1 (Pre-flight checks) — ensure fallback exists

---

### Pitfall 6: mkinitcpio Hook Failure Goes Unnoticed

**What goes wrong:**
Kernel package updates successfully, but `mkinitcpio` hook fails to generate initramfs. Boot entry exists but kernel panics on boot because initramfs is missing or outdated.

**Why it happens:**
- `/boot` partition full (ESP only 512MB, logs/old kernels accumulate)
- `mkinitcpio.conf` misconfigured (missing hooks, wrong module order)
- DKMS module fails to build for new kernel
- Pacman hook runs with old kernel modules still loaded
- Power loss during mkinitcpio execution
- Insufficient permissions (rare, but possible with ACL issues)

**Consequences:**
- System boots to kernel panic: "failed to mount root"
- GRUB shows kernel entry but boot fails
- Emergency shell has limited tools (no network, no keyboard on T2)
- Must chroot from live USB to fix

**Prevention:**

**1. Monitor mkinitcpio output during updates:**
```bash
# Don't run pacman in background or with piped output
# Watch for these SUCCESS indicators:
# ==> Building image from preset: /etc/mkinitcpio.d/linux-t2.preset: 'default'
# ==> Image generation successful

# FAILURE indicators:
# ==> ERROR: module not found: 'xxx'
# ==> WARNING: No modules were added to the image
```

**2. Create post-transaction verification hook:**
```bash
# /etc/pacman.d/hooks/99-verify-initramfs.hook
[Trigger]
Operation = Install
Operation = Upgrade
Type = Package
Target = linux-t2

[Action]
Description = Verifying initramfs was generated...
When = PostTransaction
Exec = /usr/local/bin/verify-initramfs.sh
```

```bash
#!/bin/bash
# /usr/local/bin/verify-initramfs.sh

KERNEL_VERSION=$(pacman -Q linux-t2 | cut -d' ' -f2 | cut -d'-' -f1)
INITRAMFS="/boot/initramfs-linux-t2.img"

# Check file exists and is recent (modified in last 5 minutes)
if [ ! -f "$INITRAMFS" ]; then
  echo "CRITICAL: $INITRAMFS not found!"
  exit 1
fi

if [ $(find "$INITRAMFS" -mmin -5 | wc -l) -eq 0 ]; then
  echo "WARNING: $INITRAMFS not updated recently"
  exit 1
fi

# Check file size (should be > 10MB typically)
SIZE=$(stat -c%s "$INITRAMFS")
if [ $SIZE -lt 10485760 ]; then
  echo "WARNING: $INITRAMFS suspiciously small ($SIZE bytes)"
  exit 1
fi

echo "✓ Initramfs verified: $SIZE bytes, recently updated"
exit 0
```

**3. Keep /boot partition clean:**
```bash
# Automate old kernel cleanup:
# /etc/pacman.d/hooks/98-clean-boot.hook
[Trigger]
Operation = Remove
Type = Package
Target = linux-t2

[Action]
Description = Cleaning old kernels from /boot...
When = PostTransaction
Exec = /bin/sh -c 'rm -f /boot/vmlinuz-linux-t2.old /boot/initramfs-linux-t2-fallback.img.old'
```

**4. Increase ESP partition size (if consistently full):**
```bash
# Check current space:
df -h /boot

# If < 100MB free, consider:
# - Removing old bootloaders (if dual-boot removed)
# - Moving /boot to separate larger partition
# - This requires repartitioning (advanced/risky)
```

**Detection warning signs:**
- Kernel update completes in < 30 seconds (mkinitcpio usually takes 1-2 minutes)
- No "Building image" messages in pacman output
- `ls -lh /boot` shows old timestamp on initramfs
- `/boot` partition at > 90% capacity

**Recovery:**
1. Boot from live USB
2. Chroot and regenerate initramfs:
   ```bash
   arch-chroot /mnt
   mkinitcpio -P  # Rebuild all presets
   # Check for errors in output!
   ls -lh /boot  # Verify new files created
   ```

**Phase assignment:** Phase 1 (Pre-flight checks) — automated verification

---

## Moderate Pitfalls

Mistakes that cause delays, data loss, or technical debt but are recoverable.

### Pitfall 7: Pacman Database Lock During Automated Backups

**What goes wrong:**
Automated backup script runs `rsync` while `pacman` is mid-transaction. Pacman lock file (`/var/lib/pacman/db.lck`) copied to backup. Later manual `pacman` run fails with "unable to lock database".

**Why it happens:**
- Cron job runs backup at fixed time
- User/system runs `pacman -Syu` at same time
- Lock file exists temporarily during package transactions
- Rsync copies lock file to backup

**Prevention:**
```bash
# Exclude lock file from backup:
rsync -aAXv --exclude='/var/lib/pacman/db.lck' / /mnt/backup/

# Or check if pacman is running before backup:
if pgrep -x pacman > /dev/null; then
  echo "Pacman running, skipping backup"
  exit 1
fi
```

**Recovery:**
```bash
# If restored system has stale lock:
rm /var/lib/pacman/db.lck
```

**Phase assignment:** Phase 2 (Backup strategy) — exclude volatile files

---

### Pitfall 8: Backup Script Doesn't Preserve Extended Attributes

**What goes wrong:**
Using `rsync -av` instead of `rsync -aAXv` loses file capabilities, SELinux contexts (if used), and ACLs. Restored system has permission issues.

**Why it happens:**
- User copies common rsync examples without understanding flags
- `-a` (archive) doesn't include `-A` (ACLs) and `-X` (xattrs)

**Prevention:**
```bash
# Always use:
rsync -aAXv / /mnt/backup/
# -a = archive (permissions, timestamps, etc.)
# -A = preserve ACLs
# -X = preserve extended attributes
# -v = verbose
```

**Recovery:**
After restore, fix capabilities if needed:
```bash
# Example: If ping doesn't work (needs CAP_NET_RAW):
sudo setcap cap_net_raw+ep /usr/bin/ping
```

**Phase assignment:** Phase 2 (Backup strategy) — correct rsync flags

---

### Pitfall 9: GRUB Updates Without T2 Kernel Parameters

**What goes wrong:**
Running `grub-mkconfig` without T2 kernel parameters in `/etc/default/grub` results in bootable but broken system (no keyboard/trackpad).

**Why it happens:**
- User edits GRUB config manually
- Forgets to add `intel_iommu=on iommu=pt pcie_ports=compat`
- GRUB update from pacman regenerates config

**Prevention:**
```bash
# Verify /etc/default/grub contains:
GRUB_CMDLINE_LINUX_DEFAULT="quiet intel_iommu=on iommu=pt pcie_ports=compat"

# After any GRUB change:
grub-mkconfig -o /boot/grub/grub.cfg
# Verify output shows kernel parameters
```

**Detection:**
- Boot succeeds but no keyboard/trackpad/WiFi
- `cat /proc/cmdline` missing T2 parameters

**Recovery:**
1. Boot with external USB keyboard
2. Edit `/etc/default/grub`
3. Run `grub-mkconfig -o /boot/grub/grub.cfg`
4. Reboot

**Phase assignment:** Phase 1 (Pre-flight checks) — verify GRUB config

---

### Pitfall 10: Forgotten to Test Backup Restoration

**What goes wrong:**
Backups run successfully for months. Disaster strikes. Restoration fails because backup process had subtle bug never detected.

**Why it happens:**
- "Set and forget" backup mentality
- Never validated backup integrity
- Assumptions about rsync correctness
- No dry-run restoration tests

**Prevention:**
```bash
# Monthly restoration test in VM:
1. Boot VirtualBox/QEMU with empty virtual disk
2. Boot from VulcanOS ISO
3. Mount virtual disk
4. Restore from backup: rsync -aAXv /mnt/backup/ /mnt/newroot/
5. Chroot and verify: pacman -Qk (check all packages)
6. Try booting restored system in VM
```

**Phase assignment:** Phase 3 (Validation system) — automated restore testing

---

## Minor Pitfalls

Annoyances that are easily fixable.

### Pitfall 11: Backup Notifications Ignored

**What goes wrong:**
Cron job runs backup, sends mail to local mailbox, user never checks `/var/spool/mail/`.

**Prevention:**
```bash
# Send desktop notifications instead:
notify-send "Backup complete" "$(date)"

# Or integrate with swaync (VulcanOS notification daemon)
```

**Phase assignment:** Phase 3 (Validation system) — user-visible notifications

---

### Pitfall 12: Huge /var/cache/pacman/pkg Directory Backed Up

**What goes wrong:**
Backing up 10GB+ of cached packages wastes backup space and time.

**Prevention:**
```bash
# Exclude from backup:
--exclude='/var/cache/pacman/pkg/*'

# Or periodically clean cache:
paccache -rk2  # Keep last 2 versions
```

**Phase assignment:** Phase 2 (Backup strategy) — exclude unnecessary data

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation Strategy |
|-------------|---------------|---------------------|
| Pre-flight checks | Wrong kernel installed | `IgnorePkg` in pacman.conf + verification hook |
| Pre-flight checks | `/boot` unmounted during update | Mountpoint check hook before kernel updates |
| Pre-flight checks | No fallback kernel | Install/preserve old kernel + GRUB menu timeout |
| Pre-flight checks | GRUB missing T2 parameters | Automated `/etc/default/grub` verification |
| Pre-flight checks | mkinitcpio failure | Post-transaction initramfs verification hook |
| Backup strategy | Live system rsync | Document limitations, recommend snapshot migration |
| Backup strategy | External drive disconnect | USB autosuspend disable + sync mount + monitoring |
| Backup strategy | Exclude volatile files | Proper rsync flags + exclusion list |
| Validation system | Never test restore | Automated monthly VM restore test |
| Validation system | Silent failures | Desktop notifications + log aggregation |

---

## T2 MacBook-Specific Considerations

### Hardware-Related Pitfalls

1. **Touch Bar becomes non-functional after suspend**
   - S3 suspend broken since macOS Sonoma
   - Requires systemd workaround (not kernel-related)
   - Recovery: Reboot system

2. **WiFi firmware missing after restore**
   - `apple-bcm-firmware` package must be in backup
   - Firmware files in `/lib/firmware/brcm/` critical
   - Prevention: Verify firmware files in backup

3. **T2 audio config lost**
   - `apple-t2-audio-config` package provides ALSA config
   - Recovery: Reinstall package after restore

4. **Keyboard backlight not working after kernel update**
   - `apple-magic-backlight` module may need recompilation
   - DKMS should auto-rebuild but sometimes fails
   - Check: `modprobe apple-magic-backlight`

### Firmware Update Pitfalls

**Pitfall:** macOS firmware update changes EFI variables
- T2 firmware updates from macOS may reset boot order
- Linux entry might disappear from boot menu
- Prevention: Keep macOS partition, document EFI boot entry restoration:
  ```bash
  efibootmgr -c -d /dev/nvme0n1 -p 1 -L "Arch Linux" -l '\EFI\arch\grubx64.efi'
  ```

---

## Sources

Research sources, categorized by confidence level:

### HIGH Confidence (Official Documentation)
- [t2linux wiki - Post-installation guide](https://wiki.t2linux.org/guides/postinstall/)
- [t2linux wiki - Arch kernel updating](https://wiki.t2linux.org/distributions/arch/faq/)
- [Arch Wiki - mkinitcpio](https://wiki.archlinux.org/title/Mkinitcpio)
- [Arch Wiki - Timeshift](https://wiki.archlinux.org/title/Timeshift)
- [Arch Wiki - pacman](https://wiki.archlinux.org/title/Pacman)

### MEDIUM Confidence (Community Documentation + Forum Consensus)
- [Repairing a Botched Arch Linux Kernel Upgrade](https://arusahni.net/blog/2024/04/arch-crash-recovery-chroot.html)
- [Arch Forums - T2 Macbook Air and kernel updates](https://bbs.archlinux.org/viewtopic.php?id=299034)
- [Arch Forums - Making a pacman hook to backup /boot](https://bbs.archlinux.org/viewtopic.php?id=289248)
- [Arch Forums - rsync system backup discussions](https://bbs.archlinux.org/viewtopic.php?id=277249)
- [Arch Forums - GRUB fallback kernel](https://bbs.archlinux.org/viewtopic.php?id=99762)
- [Arch Forums - mkinitcpio module errors](https://bbs.archlinux.org/viewtopic.php?id=298525)
- [Arch Forums - External USB drive LUKS rsync issues](https://bbs.archlinux.org/viewtopic.php?id=247760)

### MEDIUM-LOW Confidence (Blog Posts, Recent User Reports)
- [The Terrors of Linux on a T2 Mac](https://awpsec.medium.com/the-terrors-of-linux-on-a-t2-mac-9b66699a8693)
- [CachyOS - T2 MacBook installation guide](https://wiki.cachyos.org/installation/installation_t2macbook/)

---

## Research Methodology Notes

**Verification approach:**
- Cross-referenced t2linux official docs (HIGH confidence)
- Validated against multiple Arch Forums threads (MEDIUM confidence)
- Confirmed T2-specific issues in community wikis
- Tested kernel parameter requirements against official recommendations

**Gaps identified:**
- No official documentation on T2 kernel rollback procedures (relied on general Arch practices)
- Limited information on T2-specific GRUB quirks (extrapolated from general T2 requirements)
- No comprehensive guide on backup strategies for T2 Macs specifically (applied general Linux + T2 constraints)

**Confidence assessment:**
- **T2 kernel pitfalls:** HIGH (well-documented in t2linux wiki)
- **Arch backup pitfalls:** HIGH (extensive Arch Wiki + forum consensus)
- **T2 + backup interaction:** MEDIUM (inferred from separate sources)
- **Hardware-specific edge cases:** MEDIUM-LOW (community reports, not all verified)
