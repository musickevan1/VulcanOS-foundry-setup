# Phase 1: T2 Kernel Protection - Context

**Gathered:** 2026-01-23
**Status:** Ready for planning

<domain>
## Phase Boundary

Prevent catastrophic boot failures from kernel updates on T2 MacBook Pro hardware. This phase implements protection mechanisms — pacman hooks, verification scripts, and GRUB fallback configuration — that guard against dangerous kernel operations. Backup and restore capabilities belong in Phase 2.

</domain>

<decisions>
## Implementation Decisions

### Warning behavior
- Terminal output + swaync desktop notification (both channels)
- Warnings include full context: what's happening, why it matters for T2, what to do
- Desktop notifications persist until user dismisses them (no auto-dismiss)

### Abort conditions
- Block pacman transaction completely if /boot is not mounted
- Block mainline `linux` kernel installation completely (only linux-t2 allowed)
- Block kernel updates if no valid fallback kernel exists
- Emergency bypass via `VULCAN_UNSAFE=1` environment variable

### Fallback boot setup
- Auto-generate GRUB fallback entries, but allow manual override in grub.cfg
- Keep two previous kernel versions as fallback options
- Fallback entries visible directly in GRUB main menu (not hidden in submenu)
- Auto-rotate old kernels when limit reached (remove oldest automatically)

### Verification feedback
- Runs automatically after kernel operations (pacman hook) AND available as manual command
- Full boot chain verification: initramfs exists, contains T2 modules, GRUB config valid, kernel present
- Detailed summary on success: kernel version, initramfs size, modules present
- On failure: report what failed and exit non-zero (user decides next steps)

### Claude's Discretion
- Warning urgency levels (red vs orange) based on specific risk
- Exact notification wording and formatting
- Specific T2 modules to verify in initramfs
- GRUB entry naming convention

</decisions>

<specifics>
## Specific Ideas

- Bypass mechanism should be deliberate but not too easy (env var requires explicit action)
- Verification should give enough information to diagnose issues
- Protection applies to pacman operations, not manual file manipulation

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 01-t2-kernel-protection*
*Context gathered: 2026-01-23*
