# VulcanOS

## What This Is

A development-focused, opinionated Arch Linux distribution with Hyprland compositor. VulcanOS provides system protection, organized tooling, and a cohesive visual experience for single-user development workstations. v3.0 introduces multi-profile support targeting two hardware platforms: **Vulcan Foundry** (AMD AI workstation) and **Vulcan T2** (T2 MacBook Pro).

## Core Value

**Cohesive, recoverable, keyboard-driven.** The system should feel unified (one visual identity), be always recoverable (snapshots + kernel protection), and stay out of the way (minimal, fast, keyboard-first).

## Current Milestone: v3.0 Multi-Profile + AI Workstation

**Goal:** Restructure VulcanOS into a multi-profile architecture supporting both the new Vulcan Foundry AI workstation and the existing T2 MacBook Pro, with intentionally curated package sets and full NVIDIA/CUDA AI stack.

**Target features:**
- Multi-profile archiso structure (foundry/t2 profiles)
- Full NVIDIA CUDA + AI/ML stack (PyTorch, JAX, Ollama, llama.cpp, ComfyUI)
- Intentionally rebuilt package list (fresh curation, not legacy trimming)
- Hyprland plugins (hyprexpo, hyprspace, hyprtrails)
- yazi terminal file manager as primary
- Gaming support for Foundry (Steam, Proton, gamemode)
- Drop Alacritty (Kitty only)

**Vulcan Foundry Hardware:**
- AMD Ryzen 9 9950X (16-core, 32-thread)
- NVIDIA RTX 5070 Ti 16GB (Blackwell architecture)
- 64GB DDR5-6000 RAM
- 2TB Samsung 990 PRO NVMe
- ASUS TUF X870E-PLUS WiFi 7

## Current State

**Shipped:** v2.1 Maintenance (2026-02-01)

The Vulcan Appearance Manager is now production-ready with security hardening, complete wallpaper assets, and a proper preview/apply/cancel workflow with state machine integration.

**Current codebase:**
- 8,159 lines of Rust (vulcan-appearance-manager)
- 10 polished preset themes (8 dark, 2 light) with official color palettes
- 6 desktop components with automatic theme propagation
- Complete wallpaper library with CC0 licensing
- 53 unit tests (security, state machine, etc.)

## Requirements

### Validated

Capabilities shipped and working:

**v1.0 Foundation:**
- Arch Linux ISO build system (archiso profile) — v1.0
- GNU Stow dotfile management structure — v1.0
- Hyprland compositor with modular config — v1.0
- Waybar status bar with custom modules — v1.0
- vulcan-todo MCP server (tasks, sprints, projects) — v1.0
- vulcan-vault MCP server (notes, semantic search) — v1.0
- Desktop integration scripts (vulcan-menu, themes, etc.) — v1.0
- T2 MacBook Pro hardware support (linux-t2 kernel) — v1.0
- Speech-to-text via hyprwhspr — v1.0
- T2 kernel protection (pacman hooks, boot verification) — v1.0
- vulcan-wallpaper-manager (per-monitor wallpaper GUI) — v1.0
- vulcan-theme CLI (applies themes to desktop tools) — v1.0
- 8 built-in color themes — v1.0

**v2.0 Appearance Manager:**
- Unified Vulcan Appearance Manager (replaces theme-manager + wallpaper-manager) — v2.0
- Theme browser with color preview cards — v2.0
- Per-monitor wallpaper assignment with profile save/load — v2.0
- Theme-wallpaper binding (themes suggest wallpapers) — v2.0
- Unified profiles (theme + wallpaper + binding mode) — v2.0
- Theme propagation to 6 components (waybar, wofi, swaync, hyprlock, kitty, alacritty) — v2.0
- App self-theming (GUI uses active theme colors) — v2.0
- 10 polished preset themes (8 dark, 2 light) — v2.0
- Third-party app discovery with marketplace links — v2.0
- Desktop integration (vulcan-menu, archiso sync) — v2.0

**v2.1 Maintenance:**
- All theme loading paths use parse_and_validate() security function — v2.1
- Theme import rejects dangerous patterns and path traversal — v2.1
- BindingMode auto-transitions to CustomOverride on manual wallpaper change — v2.1
- All 10 preset themes have bundled wallpapers with attribution — v2.1
- AppState state machine integrated for preview/apply/cancel lifecycle — v2.1
- Cancel Preview restores both theme and wallpapers — v2.1
- Implicit apply on window close during preview — v2.1

### Active

**v3.0 Multi-Profile + AI Workstation:**

- [ ] Multi-profile archiso structure (shared base, foundry/t2 profiles)
- [ ] Vulcan Foundry profile with NVIDIA/CUDA/AI stack
- [ ] Vulcan T2 profile (continuation of current T2 support)
- [ ] Fresh package curation (intentional inclusion, not legacy)
- [ ] Full AI/ML toolchain (PyTorch, JAX, Ollama, llama.cpp, ComfyUI)
- [ ] Hyprland plugins (hyprexpo, hyprspace, hyprtrails)
- [ ] yazi as primary file manager
- [ ] Gaming stack for Foundry (Steam, Proton, gamemode)
- [ ] Terminal consolidation (Kitty only, drop Alacritty)
- [ ] Triple+ monitor configuration support

### Out of Scope

- Automatic third-party theme installation — discovery links only
- AI wallpaper generation on-demand — pre-made wallpapers bundled
- Cloud sync for themes/wallpapers — local dotfiles + git sync
- Core backup system (Phases 2-4) — deferred from v1.0
- ARM/non-x86_64 support — x86_64 only for now

## Context

**Hardware targets:**
- **Vulcan Foundry:** AMD Ryzen 9 9950X, RTX 5070 Ti 16GB, 64GB DDR5, 2TB NVMe, triple+ monitors
- **Vulcan T2:** 2019 MacBook Pro 16" with T2 chip, requires linux-t2 kernel from arch-mact2 repo

**Shipped apps:**
- `vulcan-appearance-manager/` — GTK4/Relm4, 8,159 lines, unified theme + wallpaper management
- `vulcan-theme` CLI — Bash script for theme application via envsubst templates

**Technical debt:** None remaining from v2.0/v2.1. Codebase is clean for new features.

**v3.0 architecture change:** Moving from single archiso profile to multi-profile structure with shared base packages and profile-specific overlays.

## Constraints

- **Framework:** GTK4/Libadwaita + Relm4 (established pattern)
- **Theme format:** `.sh` script format (vulcan-theme compatibility)
- **Wallpaper backend:** swww
- **Profile storage:** TOML files in ~/.config/vulcan-appearance-manager/

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Merge both apps into one | Themes and wallpapers are conceptually linked | Good — unified UX |
| Keep GTK4/Relm4 framework | Both existing apps use it; proven patterns | Good — consistent codebase |
| Theme suggests wallpaper (overridable) | Balance between cohesion and user choice | Good — BindingMode works well |
| Pre-made wallpapers, not on-demand AI | Simpler, faster UX | Good — CC0 wallpapers bundled |
| Discovery-only for third-party apps | Avoid complexity of auto-installing themes | Good — marketplace links sufficient |
| ViewStack over TabView | Fixed application views, not user-managed tabs | Good — matches libadwaita HIG |
| BindingMode enum (3 states) | Explicit tracking of theme-wallpaper relationship | Good — UI clarity |
| STYLE_PROVIDER_PRIORITY_USER | Runtime CSS overrides brand defaults | Good — self-theming works |
| parse_and_validate() for all theme loading | Central security validation | Good — 6 dangerous pattern checks |
| Implicit apply on close | Prevents loss of preview work | Good — matches user expectation |
| Apply failure -> Previewing | User can retry or cancel | Good — resilient workflow |

---
*Last updated: 2026-02-02 after v3.0 milestone start*
