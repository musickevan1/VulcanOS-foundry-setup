# Project: TouchBar Status Daemon for T2 MacBooks

## Goal
Create a Waybar-like status daemon for the MacBook Pro Touch Bar on Linux (T2 Macs running Hyprland).

## Context
- Target hardware: 2019 T2 MacBook Pro running Arch Linux
- Touch Bar: 2008x60 pixel display via `appletbdrm` DRM driver
- Current solution: `tiny-dfr` (Rust) - only supports static F-keys and media buttons
- Want: Dynamic content like Waybar (workspaces, battery, CPU, clock, etc.)

## Reference
- tiny-dfr source: https://github.com/AsahiLinux/tiny-dfr
- Uses: Cairo, librsvg, libinput, DRM
- T2 wiki: https://wiki.t2linux.org

## Requirements
- Modular design (configurable modules like Waybar)
- Real-time updates
- Touch input support for button actions
- Hyprland workspace integration (via hyprland-ipc)
- Low resource usage

## Decisions Needed
- Language: Rust (match tiny-dfr) vs Python (faster iteration) vs C
- Fork tiny-dfr or start fresh?
- Config format: TOML (like tiny-dfr) vs JSON (like Waybar)

## Session Prompt

Use this to start a new Claude Code session for this project:

---

I want to build a custom Touch Bar daemon for my T2 MacBook Pro running Arch Linux with Hyprland. The goal is to create a Waybar-like experience on the Touch Bar with dynamic modules (workspaces, battery, CPU, clock, etc.).

Technical context:
- Touch Bar: 2008x60 pixels via `appletbdrm` DRM driver
- Reference implementation: tiny-dfr (https://github.com/AsahiLinux/tiny-dfr) uses Rust, Cairo, librsvg, libinput
- Need Hyprland IPC integration for workspace display
- Should support touch input for button actions

Please start by:
1. Analyzing tiny-dfr's architecture to understand how it renders to the Touch Bar
2. Designing a modular system similar to Waybar
3. Creating a project plan with implementation phases

Project location: ~/VulcanOS/touchbar-daemon/ (or suggest better location)

---

## Notes
- This is a side project separate from the main VulcanOS ISO build
- Could potentially be contributed back to t2linux community
- May want to publish to AUR once stable
