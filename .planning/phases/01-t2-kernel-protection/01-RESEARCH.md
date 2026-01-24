# Phase 1: T2 Kernel Protection - Research

**Researched:** 2026-01-23
**Domain:** Arch Linux package management, GRUB bootloader, T2 MacBook Pro kernel safety
**Confidence:** HIGH

## Summary

T2 kernel protection requires preventing catastrophic boot failures through three mechanisms: pacman hooks that verify system state and block dangerous operations, GRUB fallback configuration preserving previous kernels, and verification scripts checking boot chain integrity. The standard Arch Linux approach uses alpm hooks for pre-transaction validation, IgnorePkg for kernel pinning, and grub-mkconfig with custom entries for fallback boot options.

Arch's rolling-release model normally replaces kernels completely during updates, unlike multi-version distributions. Protection requires active intervention: pacman hooks with `AbortOnFail` block transactions before damage occurs, manual kernel preservation in /boot provides fallback options, and initramfs module verification ensures T2 hardware support is present.

**Primary recommendation:** Use PreTransaction pacman hooks with `AbortOnFail` to block dangerous operations before they happen, combined with custom GRUB entries that survive grub-mkconfig regeneration.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| alpm-hooks | pacman 6.1+ | Pre/post-transaction validation | Built into pacman, official mechanism for transaction control |
| notify-send | libnotify | Desktop notifications | Standard D-Bus notification protocol, works with all notification daemons |
| lsinitcpio | mkinitcpio | Initramfs inspection | Official Arch tool for validating initramfs contents |
| findmnt / mountpoint | util-linux | Mount point detection | Standard Linux utilities for filesystem state |
| grub-mkconfig | GRUB 2.x | Bootloader configuration | Official GRUB configuration generator |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| swaync-client | swaync | Notification control | Already installed in VulcanOS for panel management |
| pacman -Q | pacman | Package version query | Detecting installed kernel packages |
| /var/cache/pacman/pkg/ | pacman | Package cache | Downgrade source for kernel rollback |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| PreTransaction hook | PostTransaction + snapshot | Hook runs after damage occurs; snapshot-restore is slower and more complex |
| IgnorePkg | Hook-based blocking | IgnorePkg is passive (user can override); hook is active enforcement |
| /boot/grub/custom.cfg | /etc/grub.d/40_custom | custom.cfg survives grub-mkconfig without regeneration; 40_custom requires rebuild |
| findmnt | grep /proc/mounts | findmnt more robust for nested mounts and specific mount point verification |

**Installation:**
No additional packages needed - all tools are part of base Arch Linux system (pacman, GRUB, mkinitcpio, util-linux, libnotify).

## Architecture Patterns

### Recommended Project Structure
```
/etc/pacman.d/hooks/
├── 10-vulcan-kernel-protect.hook       # Block mainline kernel, check /boot mount
├── 20-vulcan-kernel-warn.hook          # Warn about kernel changes
├── 90-vulcan-kernel-verify.hook        # Verify initramfs after kernel operations

/usr/local/bin/
├── vulcan-kernel-protect               # PreTransaction validation script
├── vulcan-kernel-verify                # PostTransaction verification script
└── vulcan-kernel-fallback              # GRUB fallback entry generator

/etc/grub.d/
├── 09_vulcan_fallback                  # Custom fallback entries (runs before 10_linux)
└── (40_custom - alternative approach)

/boot/grub/
└── custom.cfg                          # Manual fallback entries (survives grub-mkconfig)
```

### Pattern 1: PreTransaction Abort Hook
**What:** Hook that validates system state before allowing package operations to proceed
**When to use:** Preventing operations that would cause immediate system failure (kernel operations without /boot mounted)
**Example:**
```ini
# Source: Official alpm-hooks documentation
# https://man.archlinux.org/man/alpm-hooks.5.en

[Trigger]
Type = Package
Operation = Install
Operation = Upgrade
Operation = Remove
Target = linux
Target = linux-lts
Target = linux-zen
Target = linux-t2

[Action]
Description = Checking kernel operation safety...
When = PreTransaction
Exec = /usr/local/bin/vulcan-kernel-protect
AbortOnFail
```

**Key insight:** `AbortOnFail` only works with `PreTransaction` hooks. The hook script exits non-zero to abort the transaction. PostTransaction hooks don't run if the transaction fails.

### Pattern 2: PostTransaction Verification Hook
**What:** Hook that verifies system integrity after successful package changes
**When to use:** Validating that kernel operations completed successfully (initramfs generated, modules present)
**Example:**
```ini
# Source: Official alpm-hooks documentation
# https://man.archlinux.org/man/alpm-hooks.5.en

[Trigger]
Type = Path
Operation = Install
Operation = Upgrade
Target = usr/lib/modules/*/vmlinuz

[Action]
Description = Verifying boot chain integrity...
When = PostTransaction
Exec = /usr/local/bin/vulcan-kernel-verify
NeedsTargets
```

**Key insight:** PostTransaction hooks only run if the transaction succeeds. Use Path triggers to detect kernel file changes. NeedsTargets provides matched paths via stdin.

### Pattern 3: GRUB Custom Fallback Entries
**What:** Manually-maintained GRUB entries for previous kernel versions that survive grub-mkconfig
**When to use:** Preserving boot options for known-good kernel versions
**Example:**
```bash
# Source: Arch GRUB Wiki - Tips and Tricks
# https://wiki.archlinux.org/title/GRUB/Tips_and_tricks

# /boot/grub/custom.cfg (sourced by 41_custom, no regeneration needed)
menuentry 'Arch Linux (Fallback: linux-t2 6.6.68)' --class arch --class gnu-linux --class gnu --class os {
    load_video
    set gfxpayload=keep
    insmod gzio
    insmod part_gpt
    insmod ext2
    search --no-floppy --fs-uuid --set=root <UUID>
    echo 'Loading fallback kernel linux-t2 6.6.68...'
    linux /boot/vmlinuz-linux-t2.backup root=UUID=<UUID> rw intel_iommu=on iommu=pt pcie_ports=compat
    initrd /boot/initramfs-linux-t2.backup.img
}
```

**Key insight:** Files in /boot/grub/custom.cfg are sourced automatically by /etc/grub.d/41_custom and don't require grub-mkconfig regeneration. Entries appear at the bottom of the GRUB menu. To change ordering, use /etc/grub.d/09_* to run before 10_linux (requires grub-mkconfig).

### Pattern 4: Mount Point Detection
**What:** Bash script validation that /boot is mounted before proceeding
**When to use:** PreTransaction hooks checking filesystem state
**Example:**
```bash
# Source: Baeldung Linux - Check if Directory Is Mounted
# https://www.baeldung.com/linux/bash-is-directory-mounted

#!/bin/bash
# Check if /boot is mounted before kernel operations

if ! mountpoint -q /boot; then
    echo "ERROR: /boot is not mounted. Cannot safely update kernel."
    echo "Run: mount /boot"
    exit 1
fi

exit 0
```

**Key insight:** `mountpoint -q` is the cleanest approach - returns exit code 0 if path is a mount point, 1 otherwise. Alternative: `findmnt --mountpoint /boot >/dev/null` is more robust for nested mounts.

### Pattern 5: Desktop Notification Integration
**What:** Send desktop notifications from scripts via D-Bus
**When to use:** Warning users about critical operations in GUI environment
**Example:**
```bash
# Source: Arch manual pages - notify-send
# https://man.archlinux.org/man/notify-send.1.en

#!/bin/bash
# Send critical warning notification

notify-send \
    --urgency=critical \
    --icon=dialog-warning \
    --expire-time=0 \
    "Kernel Update Warning" \
    "T2 kernel linux-t2 will be updated. System will require reboot.\n\nCurrent: 6.6.68\nNew: 6.6.70"
```

**Key insight:** `notify-send` works with any notification daemon including swaync. Use `--urgency=critical` for important warnings (persists until dismissed, red icon by default). `--expire-time=0` prevents auto-dismiss. swaync-client doesn't send notifications - it only controls the daemon state.

### Pattern 6: Initramfs Module Verification
**What:** Validate that initramfs contains required T2 kernel modules
**When to use:** PostTransaction verification after kernel operations
**Example:**
```bash
# Source: Arch mkinitcpio Wiki
# https://wiki.archlinux.org/title/Mkinitcpio

#!/bin/bash
# Verify T2 modules are present in initramfs

INITRAMFS="/boot/initramfs-linux-t2.img"
REQUIRED_MODULES=("apple_bce" "applespi" "intel_lpss_pci" "spi_pxa2xx_platform")

if [[ ! -f "$INITRAMFS" ]]; then
    echo "ERROR: Initramfs not found: $INITRAMFS"
    exit 1
fi

# List modules in initramfs
MODULES=$(lsinitcpio "$INITRAMFS" | grep -E '\.ko(\.gz|\.xz|\.zst)?$')

for module in "${REQUIRED_MODULES[@]}"; do
    if ! echo "$MODULES" | grep -q "$module"; then
        echo "ERROR: Required T2 module missing: $module"
        exit 1
    fi
done

echo "SUCCESS: All T2 modules present in initramfs"
exit 0
```

**Key insight:** `lsinitcpio` lists initramfs contents. Modules are at paths like `usr/lib/modules/<version>/kernel/`. Use `lsinitcpio -a` for "human-friendly listing of important parts". Modules may be compressed (.ko.gz, .ko.xz, .ko.zst).

### Anti-Patterns to Avoid
- **PostTransaction-only protection:** Damage already done before hook runs; use PreTransaction with AbortOnFail
- **Manual grub.cfg editing:** Changes overwritten by grub-mkconfig; use custom.cfg or /etc/grub.d/ scripts
- **IgnorePkg alone:** Passive protection (user can override with -Syu --ignore=); combine with active hook enforcement
- **Checking running kernel vs installed:** `uname -r` shows running kernel; use `pacman -Q linux-t2` for installed version
- **Comma-separated IgnorePkg:** Use spaces in pacman.conf (commas only for --ignore command-line flag)
- **Assuming /boot is always mounted:** Some setups mount /boot only during updates; always verify

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Notification daemon protocol | Custom D-Bus notification code | notify-send | Standard tool, works with all notification daemons, handles icons/urgency |
| Mount detection | Parsing /etc/fstab or /proc/mounts manually | mountpoint -q or findmnt | Handles edge cases (bind mounts, nested mounts, network mounts) |
| Initramfs inspection | Manual cpio extraction and parsing | lsinitcpio | Official tool, handles compression formats, provides structured output |
| Package version queries | Parsing /var/lib/pacman/local/ | pacman -Q | Handles database format changes, provides version comparison |
| GRUB config generation | String concatenation to grub.cfg | /etc/grub.d/ scripts + grub-mkconfig | Handles UUID detection, kernel parameter standardization, theme integration |
| Kernel module presence | grep on /lib/modules or lsmod | lsinitcpio for initramfs, modinfo for installed | Modules may be built-in, compressed, or renamed; tools handle variants |

**Key insight:** Arch provides official tools for all common system inspection tasks. Using them ensures compatibility with future Arch changes and handles edge cases you won't discover until production failure.

## Common Pitfalls

### Pitfall 1: Partial Upgrades from IgnorePkg
**What goes wrong:** Using IgnorePkg to prevent kernel updates creates unsupported partial upgrade state where system libraries (glibc, systemd) may be newer than kernel expects
**Why it happens:** Arch is rolling-release and assumes full system upgrades; kernel and userspace evolve together
**How to avoid:** Use IgnorePkg only temporarily or with linux-lts as fallback. Always validate that ignored kernel still works with updated system libraries
**Warning signs:** Boot failures after non-kernel updates, systemd errors, module loading failures

### Pitfall 2: /boot Not Mounted During Kernel Operations
**What goes wrong:** Kernel update writes new vmlinuz and initramfs to wrong location (root partition instead of /boot EFI partition), system unbootable
**Why it happens:** Some setups use separate /boot partition that isn't auto-mounted; pacman proceeds without checking
**How to avoid:** PreTransaction hook with mountpoint check and AbortOnFail
**Warning signs:** No error during update but boot fails, vmlinuz file present in both /boot on root and actual EFI partition

### Pitfall 3: Trusting Initramfs Generation Success
**What goes wrong:** mkinitcpio completes without error but missing T2 modules renders system unbootable (keyboard/trackpad non-functional for LUKS passphrase entry)
**Why it happens:** mkinitcpio warns about missing firmware but doesn't fail; autodetect hook may filter needed modules for different hardware
**How to avoid:** PostTransaction verification hook checking specific T2 modules (apple_bce, applespi) are in initramfs
**Warning signs:** Warnings during mkinitcpio about missing firmware, system boots but T2 hardware non-functional

### Pitfall 4: GRUB Fallback Entries Disappearing
**What goes wrong:** Manually added fallback kernel entries in grub.cfg vanish after system update
**Why it happens:** pacman hook (grub) auto-runs grub-mkconfig on kernel updates, regenerating grub.cfg and overwriting manual changes
**How to avoid:** Use /boot/grub/custom.cfg (sourced automatically, never overwritten) or /etc/grub.d/40_custom (regenerated with grub-mkconfig)
**Warning signs:** Fallback entries present after manual addition but missing after next kernel update

### Pitfall 5: Mainline Kernel Installation Sneaking In
**What goes wrong:** User runs `pacman -S linux` or metapackage pulls it in, replacing linux-t2 with mainline kernel, T2 hardware non-functional
**Why it happens:** No active enforcement preventing mainline kernel installation; IgnorePkg only prevents upgrades, not explicit installation
**How to avoid:** PreTransaction hook blocking linux/linux-lts/linux-zen packages with AbortOnFail
**Warning signs:** None until boot - system installs successfully but next boot has no WiFi, keyboard, trackpad

### Pitfall 6: Notification Environment Variables Missing
**What goes wrong:** notify-send fails silently when called from pacman hook because DBUS_SESSION_BUS_ADDRESS and DISPLAY aren't set for root
**Why it happens:** Pacman runs as root without user's graphical environment variables
**How to avoid:** Detect active user session and export environment variables, or write to /tmp and have user service poll for notifications
**Warning signs:** Terminal output works but desktop notifications never appear

### Pitfall 7: Old Kernel Files Overwritten
**What goes wrong:** Backup strategy copies vmlinuz-linux-t2 to vmlinuz-linux-t2.backup, but next update overwrites backup with "new" kernel before testing
**Why it happens:** PreTransaction hook runs before new kernel installed, can't distinguish "current stable" from "new untested"
**How to avoid:** Keep versioned backups (vmlinuz-linux-t2-6.6.68) or copy in PostTransaction of previous update
**Warning signs:** Fallback kernel is same version as current kernel, no actual fallback available

## Code Examples

Verified patterns from official sources:

### Example 1: Complete PreTransaction Safety Hook
```ini
# File: /etc/pacman.d/hooks/10-vulcan-kernel-protect.hook
# Source: alpm-hooks(5) - https://man.archlinux.org/man/alpm-hooks.5.en

[Trigger]
Type = Package
Operation = Install
Operation = Upgrade
Operation = Remove
Target = linux
Target = linux-lts
Target = linux-zen
Target = linux-hardened
Target = linux-t2

[Action]
Description = Verifying kernel operation safety (T2 MacBook Pro)...
When = PreTransaction
Exec = /usr/local/bin/vulcan-kernel-protect
AbortOnFail
```

### Example 2: Kernel Protection Script with All Validations
```bash
#!/bin/bash
# File: /usr/local/bin/vulcan-kernel-protect
# Source: Multiple - see inline citations

set -euo pipefail

LOGFILE="/var/log/vulcan-kernel-protect.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

log() {
    echo "[$TIMESTAMP] $*" | tee -a "$LOGFILE"
}

# Check 1: Verify /boot is mounted
# Source: https://www.baeldung.com/linux/bash-is-directory-mounted
if ! mountpoint -q /boot; then
    log "ERROR: /boot is not mounted"
    notify-send --urgency=critical --icon=dialog-error \
        "Kernel Update Blocked" \
        "/boot partition is not mounted. Cannot safely update kernel."
    exit 1
fi

# Check 2: Block mainline kernel packages (T2 requires linux-t2)
# Source: Reading from stdin (NeedsTargets provides package list)
while read -r package; do
    case "$package" in
        linux|linux-lts|linux-zen|linux-hardened)
            log "ERROR: Attempted to install mainline kernel: $package"
            notify-send --urgency=critical --icon=dialog-error \
                "T2 Kernel Protection" \
                "Cannot install $package on T2 MacBook Pro.\n\nOnly linux-t2 kernel is supported for T2 hardware."
            exit 1
            ;;
        linux-t2)
            # Allowed - log and continue
            log "INFO: linux-t2 kernel operation detected (allowed)"
            ;;
    esac
done

# Check 3: Verify at least one fallback kernel exists
# Source: Standard filesystem checks
if [[ ! -f /boot/vmlinuz-linux-t2.backup ]] && [[ $(ls /boot/vmlinuz-* 2>/dev/null | wc -l) -lt 2 ]]; then
    log "WARNING: No fallback kernel detected"
    notify-send --urgency=normal --icon=dialog-warning \
        "Kernel Update Warning" \
        "No fallback kernel found. Proceeding, but recommend creating backup."
fi

log "SUCCESS: All safety checks passed"
exit 0
```

### Example 3: PostTransaction Verification Script
```bash
#!/bin/bash
# File: /usr/local/bin/vulcan-kernel-verify
# Source: https://wiki.archlinux.org/title/Mkinitcpio

set -euo pipefail

INITRAMFS="/boot/initramfs-linux-t2.img"
VMLINUZ="/boot/vmlinuz-linux-t2"
REQUIRED_T2_MODULES=("apple_bce" "applespi" "intel_lpss_pci" "spi_pxa2xx_platform" "bce")

echo "=== VulcanOS Kernel Verification ==="

# Check 1: Kernel image exists
if [[ ! -f "$VMLINUZ" ]]; then
    echo "ERROR: Kernel image missing: $VMLINUZ"
    exit 1
fi
KERNEL_SIZE=$(stat -c%s "$VMLINUZ")
echo "✓ Kernel image present: $VMLINUZ ($((KERNEL_SIZE / 1024 / 1024)) MB)"

# Check 2: Initramfs exists
if [[ ! -f "$INITRAMFS" ]]; then
    echo "ERROR: Initramfs missing: $INITRAMFS"
    exit 1
fi
INITRAMFS_SIZE=$(stat -c%s "$INITRAMFS")
echo "✓ Initramfs present: $INITRAMFS ($((INITRAMFS_SIZE / 1024 / 1024)) MB)"

# Check 3: Verify T2 modules in initramfs
# Source: lsinitcpio from mkinitcpio package
echo "Checking T2 modules in initramfs..."
MODULES=$(lsinitcpio "$INITRAMFS" 2>/dev/null | grep -E '\.ko(\.gz|\.xz|\.zst)?$' || true)

MISSING_MODULES=()
for module in "${REQUIRED_T2_MODULES[@]}"; do
    if ! echo "$MODULES" | grep -q "$module"; then
        MISSING_MODULES+=("$module")
    fi
done

if [[ ${#MISSING_MODULES[@]} -gt 0 ]]; then
    echo "ERROR: Missing required T2 modules: ${MISSING_MODULES[*]}"
    notify-send --urgency=critical --icon=dialog-error \
        "Kernel Verification Failed" \
        "Initramfs missing T2 modules: ${MISSING_MODULES[*]}\n\nSystem may not boot properly."
    exit 1
fi

echo "✓ All required T2 modules present"

# Check 4: Verify GRUB config valid
if [[ -f /boot/grub/grub.cfg ]]; then
    if grub-script-check /boot/grub/grub.cfg; then
        echo "✓ GRUB configuration valid"
    else
        echo "WARNING: GRUB configuration has syntax errors"
    fi
fi

# Check 5: Get kernel version
KERNEL_VERSION=$(pacman -Q linux-t2 | awk '{print $2}')
echo "✓ Kernel package version: $KERNEL_VERSION"

echo ""
echo "=== Verification Complete ==="
echo "Boot chain is ready. Reboot required to load new kernel."

notify-send --urgency=normal --icon=dialog-information \
    "Kernel Update Complete" \
    "linux-t2 $KERNEL_VERSION verified successfully.\n\nReboot to load new kernel."

exit 0
```

### Example 4: GRUB Fallback Entry Generator
```bash
#!/bin/bash
# File: /usr/local/bin/vulcan-kernel-fallback
# Source: Arch GRUB Wiki - https://wiki.archlinux.org/title/GRUB

set -euo pipefail

CUSTOM_CFG="/boot/grub/custom.cfg"
CURRENT_KERNEL="/boot/vmlinuz-linux-t2"
BACKUP_KERNEL="/boot/vmlinuz-linux-t2.backup"
BACKUP_INITRAMFS="/boot/initramfs-linux-t2.backup.img"

# Get root filesystem UUID
# Source: Standard blkid usage
ROOT_UUID=$(findmnt -n -o UUID /)

# Get kernel version from package
KERNEL_VERSION=$(pacman -Q linux-t2 | awk '{print $2}')

# Backup current kernel files
if [[ -f "$CURRENT_KERNEL" ]]; then
    cp "$CURRENT_KERNEL" "$BACKUP_KERNEL"
    cp "/boot/initramfs-linux-t2.img" "$BACKUP_INITRAMFS"
    echo "Backed up current kernel: $KERNEL_VERSION"
fi

# Generate custom.cfg with fallback entry
# Source: GRUB menuentry syntax - https://wiki.archlinux.org/title/GRUB/Tips_and_tricks
cat > "$CUSTOM_CFG" << EOF
# VulcanOS T2 Kernel Fallback Entry
# Generated: $(date)
# DO NOT EDIT - regenerated automatically by vulcan-kernel-fallback

menuentry 'Arch Linux (T2 Fallback: $KERNEL_VERSION)' --class arch --class gnu-linux --class gnu --class os {
    load_video
    set gfxpayload=keep
    insmod gzio
    insmod part_gpt
    insmod ext2
    search --no-floppy --fs-uuid --set=root $ROOT_UUID
    echo 'Loading fallback kernel linux-t2 $KERNEL_VERSION...'
    linux $BACKUP_KERNEL root=UUID=$ROOT_UUID rw intel_iommu=on iommu=pt pcie_ports=compat
    initrd $BACKUP_INITRAMFS
}
EOF

echo "Fallback entry created in $CUSTOM_CFG"
echo "Entry will appear in GRUB menu automatically (no grub-mkconfig needed)"
```

### Example 5: Kernel Warning Hook
```ini
# File: /etc/pacman.d/hooks/20-vulcan-kernel-warn.hook
# Purpose: Warn user about kernel changes (informational, doesn't abort)

[Trigger]
Type = Package
Operation = Upgrade
Target = linux-t2

[Action]
Description = Kernel update detected - generating warnings...
When = PreTransaction
Exec = /usr/local/bin/vulcan-kernel-warn
NeedsTargets
```

```bash
#!/bin/bash
# File: /usr/local/bin/vulcan-kernel-warn

# Get current installed version
CURRENT_VERSION=$(pacman -Q linux-t2 2>/dev/null | awk '{print $2}' || echo "none")

# NeedsTargets provides package names via stdin, but not new versions
# For version info, would need to parse pacman output differently
# This example shows notification structure

notify-send \
    --urgency=normal \
    --icon=system-software-update \
    --expire-time=0 \
    "T2 Kernel Update" \
    "linux-t2 kernel will be updated.\n\nCurrent: $CURRENT_VERSION\n\nSystem will require reboot after update.\n\nFallback kernel will be preserved in GRUB menu."

echo "Kernel update warning sent to desktop"
exit 0
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual grub.cfg editing | /boot/grub/custom.cfg or /etc/grub.d/ scripts | GRUB 2.0 (2012) | Manual entries survive regeneration |
| /etc/rc.d/ scripts | Pacman hooks (alpm-hooks) | Pacman 5.0 (2016) | Transaction-integrated validation instead of post-hoc |
| Manual initramfs inspection | lsinitcpio -a for analysis | mkinitcpio 0.7+ (2011) | Structured output instead of manual cpio extraction |
| Checking /etc/mtab | findmnt / mountpoint | util-linux 2.18+ (2010) | Proper mount namespace handling |
| grub-install after every change | grub-mkconfig only | GRUB 2.x | Faster, safer config updates |

**Deprecated/outdated:**
- **Direct grub.cfg editing:** Use custom.cfg or /etc/grub.d/ for persistence
- **Parsing /etc/mtab for mounts:** Deprecated in favor of /proc/mounts and findmnt
- **pacman-hooks package:** Merged into pacman core as alpm-hooks
- **GRUB_DISABLE_LINUX_UUID:** Removed; use search --fs-uuid in menuentry
- **mkinitcpio -k <path>:** Old syntax; modern is just kernel package trigger

## Open Questions

Things that couldn't be fully resolved:

1. **Desktop notification from root context**
   - What we know: notify-send requires DBUS_SESSION_BUS_ADDRESS and DISPLAY from user session; pacman hooks run as root without these
   - What's unclear: Best pattern for root process sending notifications to active user session (multiple users, multiple sessions, Wayland vs X11)
   - Recommendation: Detect active session via loginctl, export environment variables from user's systemd environment. Alternative: Write notification requests to /tmp and have user-space service poll/relay

2. **GRUB entry ordering with custom entries**
   - What we know: /etc/grub.d/ scripts run in alphabetical order (09_* before 10_linux before 40_custom); custom.cfg appears at end
   - What's unclear: Whether fallback entries should appear before or after main kernel entry (UX question: emphasize stability or latest)
   - Recommendation: Use 09_vulcan_fallback to place fallback entries at top (visible without scrolling) or custom.cfg for bottom (less cluttered main menu)

3. **Kernel module verification completeness**
   - What we know: Required T2 modules are apple_bce, applespi, intel_lpss_pci, spi_pxa2xx_platform based on t2linux wiki
   - What's unclear: Whether all T2 models require identical module set; whether built-in modules (not .ko files) are sufficient
   - Recommendation: Verify module list against actual VulcanOS T2 hardware; use `lsmod` on working system to identify critical modules; consider both .ko presence AND successful load

4. **Fallback kernel rotation strategy**
   - What we know: Arch normally keeps only current kernel; manual preservation needed for fallback
   - What's unclear: Optimal number of fallback kernels (1 previous vs 2 previous), when to rotate (every update vs only after successful boot)
   - Recommendation: Keep 2 fallback kernels; rotate only after confirming new kernel boots successfully (manual trigger or time-based "assumed good")

## Sources

### Primary (HIGH confidence)
- [alpm-hooks(5) - Arch manual pages](https://man.archlinux.org/man/alpm-hooks.5.en) - Hook file format, trigger types, execution order
- [GRUB/Tips and tricks - ArchWiki](https://wiki.archlinux.org/title/GRUB/Tips_and_tricks) - Custom entries, GRUB_DEFAULT, menu management
- [mkinitcpio - ArchWiki](https://wiki.archlinux.org/title/Mkinitcpio) - Initramfs generation, verification, module inclusion
- [t2linux wiki - Kernel](https://wiki.t2linux.org/guides/kernel/) - Required T2 modules, initramfs configuration
- [Pacman - ArchWiki](https://wiki.archlinux.org/title/Pacman) - IgnorePkg, package management, partial upgrade warnings

### Secondary (MEDIUM confidence)
- [pachooks GitHub examples](https://github.com/andrewgregory/pachooks) - Real-world hook implementations (check-boot example)
- [Baeldung - Check if Directory Is Mounted](https://www.baeldung.com/linux/bash-is-directory-mounted) - mountpoint vs findmnt comparison
- [notify-send(1) - Arch manual pages](https://man.archlinux.org/man/notify-send.1.en) - Urgency levels, icon handling

### Tertiary (LOW confidence - community wisdom, needs validation)
- [Arch Forums - Keep backup kernel](https://forum.endeavouros.com/t/arch-how-to-keep-a-backup-kernel-for-possible-update-accidents/4227) - Fallback strategies (linux-lts, pacman cache)
- [Arch Forums - GRUB default kernel selection](https://bbs.archlinux.org/viewtopic.php?id=267383) - GRUB_DEFAULT usage patterns
- [Arch Forums - IgnorePkg syntax](https://bbs.archlinux.org/viewtopic.php?id=42319) - Space vs comma separation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All tools are official Arch utilities documented in man pages and ArchWiki
- Architecture: HIGH - Patterns verified from official documentation and real-world hook examples
- Pitfalls: MEDIUM - Based on forum discussions and community experience; specific to T2 needs validation on hardware
- T2 modules: MEDIUM - Based on t2linux wiki; should verify against actual VulcanOS hardware (apple_bce vs bce naming)
- Notification from root: LOW - Multiple approaches exist; best pattern for VulcanOS workflow unclear

**Research date:** 2026-01-23
**Valid until:** 2026-02-23 (30 days - stable domain, Arch packaging patterns evolve slowly)
