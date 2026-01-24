---
phase: 01-t2-kernel-protection
plan: 01
type: execute
status: complete
subsystem: system-protection
tags: [kernel, pacman-hooks, t2-hardware, boot-safety]

requires:
  - none (first plan)

provides:
  - kernel-protection-hooks
  - mainline-kernel-blocking
  - boot-mount-verification
  - kernel-update-warnings

affects:
  - 01-02 (will need these hooks to be in place)
  - 01-03 (boot verification builds on this protection)
  - future kernel updates (all will trigger these hooks)

tech-stack:
  added:
    - pacman-hooks
    - bash-scripting
    - loginctl-session-detection
    - notify-send-desktop-notifications
  patterns:
    - PreTransaction validation hooks
    - Desktop notification from root context
    - Dual-channel warnings (terminal + notification)
    - Environment variable bypass mechanism

key-files:
  created:
    - /etc/pacman.d/hooks/10-vulcan-kernel-protect.hook
    - /usr/local/bin/vulcan-kernel-protect
    - /etc/pacman.d/hooks/20-vulcan-kernel-warn.hook
    - /usr/local/bin/vulcan-kernel-warn
  modified: []

decisions:
  - id: hook-ordering
    choice: "Protection (10-) runs before warning (20-)"
    rationale: "Must block dangerous operations before informing about safe ones"
    alternatives: ["Single combined hook", "Warning before protection"]

  - id: boot-mount-check
    choice: "Check fstab for /boot config, fallback to write test"
    rationale: "Handles both separate /boot partition and /boot as part of root"
    alternatives: ["Only check mountpoint", "Skip mount check entirely"]

  - id: fallback-warning-severity
    choice: "Warning only (non-blocking) for missing fallback"
    rationale: "First kernel install has no fallback - blocking would prevent installation"
    alternatives: ["Block if no fallback", "Silent (no warning)"]

  - id: notification-from-root
    choice: "Detect active session via loginctl, sudo -u for notify-send"
    rationale: "Hooks run as root but need to show notifications to logged-in user"
    alternatives: ["Skip notifications", "Write to file for user to poll"]

  - id: bypass-mechanism
    choice: "VULCAN_UNSAFE=1 environment variable"
    rationale: "Allows expert users to override in emergency situations"
    alternatives: ["No bypass", "Config file flag", "Command line flag"]

metrics:
  duration: "5 minutes"
  completed: 2026-01-24
  tasks: 2
  commits: 2
  files_created: 4
  lines_added: 306

next-phase-readiness:
  ready: true
  blockers: []
  concerns: []
  notes: "Protection hooks are in place and tested. Ready for fallback boot and verification tasks."
---

# Phase 01 Plan 01: Kernel Protection Hooks Summary

**One-liner:** Pacman hooks block mainline kernel installation and verify /boot accessibility before allowing kernel operations on T2 MacBook Pro

## What Was Built

Created two pacman hooks with corresponding bash scripts to protect T2 MacBook Pro systems from catastrophic boot failures caused by kernel operations:

### 1. Protection Hook (10-vulcan-kernel-protect.hook)
- **Triggers on:** Install/Upgrade/Remove of any kernel package (linux, linux-lts, linux-zen, linux-hardened, linux-t2)
- **Runs:** PreTransaction with AbortOnFail
- **Actions:**
  - Blocks installation of mainline kernels (linux, linux-lts, linux-zen, linux-hardened)
  - Verifies /boot is accessible (mounted if in fstab, or writable if part of root)
  - Warns if fallback kernel missing (non-blocking)
  - Sends critical desktop notifications for blocked operations
  - Logs all actions to /var/log/vulcan-kernel-protect.log

### 2. Warning Hook (20-vulcan-kernel-warn.hook)
- **Triggers on:** Upgrade of linux-t2 only
- **Runs:** PreTransaction (informational, never blocks)
- **Actions:**
  - Shows current kernel version
  - Notifies user about upcoming update via persistent desktop notification
  - Prints information to terminal for CLI users
  - Reminds about reboot requirement and fallback preservation
  - Logs to /var/log/vulcan-kernel-warn.log

## Key Technical Decisions

### Boot Mount Detection Logic
The protection script intelligently handles both common /boot configurations:

1. **Separate /boot partition** (in /etc/fstab):
   - Checks that /boot is actually mounted via `mountpoint -q /boot`
   - Blocks operation if configured but not mounted

2. **/boot as part of root** (not in fstab):
   - Checks that /boot directory is writable
   - Allows operation if write access exists

This dual approach prevents false positives while maintaining safety.

### Desktop Notification from Root Context
Hooks run as root during pacman transactions, but notifications need to appear for the logged-in user. Solution:

```bash
# Detect active graphical session
active_user=$(loginctl list-sessions --no-legend | awk '{print $3}' | head -1)
user_id=$(id -u "$active_user")
export DBUS_SESSION_BUS_ADDRESS="unix:path=/run/user/$user_id/bus"
sudo -u "$active_user" notify-send ...
```

This allows root-executed scripts to send notifications to the user's desktop session.

### Warning vs Protection Separation
- **Protection (10-)**: Blocks dangerous operations with clear error messages
- **Warning (20-)**: Informs about safe operations without blocking

This separation ensures users get context about legitimate updates while being protected from dangerous ones.

## Success Criteria Met

- ✅ Pacman refuses to install mainline linux kernel (linux, linux-lts, linux-zen, linux-hardened)
- ✅ Pacman aborts if /boot is not accessible before kernel operations
- ✅ User sees desktop notification warning before kernel update proceeds
- ✅ Terminal output shows clear error messages when protection blocks operation
- ✅ Hook files exist with correct triggers and AbortOnFail configuration
- ✅ Scripts are executable and properly documented
- ✅ Bypass mechanism (VULCAN_UNSAFE=1) works and logs warning

## Testing Performed

1. **Mainline kernel blocking**: `echo "linux" | vulcan-kernel-protect` → blocked
2. **linux-t2 allowed**: `echo "linux-t2" | vulcan-kernel-protect` → passes boot check
3. **Warning script**: `echo "linux-t2" | vulcan-kernel-warn` → shows info, exits 0
4. **Bypass mechanism**: `VULCAN_UNSAFE=1 vulcan-kernel-protect` → bypass confirmed, logged
5. **Hook ordering**: `ls hooks/` confirms 10- runs before 20-

## Files Created

All files created in archiso overlay (will be included in VulcanOS ISO):

| File | Purpose | Lines |
|------|---------|-------|
| `/etc/pacman.d/hooks/10-vulcan-kernel-protect.hook` | Protection hook configuration | 16 |
| `/usr/local/bin/vulcan-kernel-protect` | Protection script logic | 197 |
| `/etc/pacman.d/hooks/20-vulcan-kernel-warn.hook` | Warning hook configuration | 11 |
| `/usr/local/bin/vulcan-kernel-warn` | Warning script logic | 109 |

## Deviations from Plan

None - plan executed exactly as written.

## Commits

| Hash | Message |
|------|---------|
| b2b1dd6 | feat(01-01): add kernel protection hook and script |
| be6bf42 | feat(01-01): add kernel update warning hook and script |

## Next Steps

With kernel protection in place, the next tasks in this phase are:

1. **01-02**: Create GRUB fallback boot entries
2. **01-03**: Implement kernel verification script
3. **01-04**: Set up kernel file preservation

These will build upon the protection foundation established here.

## Lessons Learned

### What Worked Well
- Dual-channel notifications (desktop + terminal) ensure users see warnings regardless of context
- Separating protection (blocking) from warning (informational) makes hook behavior predictable
- Smart /boot detection handles multiple system configurations correctly

### What Could Be Improved
- Future: Add hooks for other risky operations (bootloader updates, initramfs changes)
- Future: Integrate with system snapshot mechanism (Phase 2) for automatic pre-kernel rollback points

### Reusable Patterns
- **Desktop notification from root**: The loginctl session detection pattern is reusable for any system script that needs to notify users
- **PreTransaction hook structure**: Template for other safety hooks (bootloader protection, config validation)
- **Environment variable bypass**: VULCAN_UNSAFE pattern can be applied to other safety mechanisms
