# VulcanOS

## What This Is

A development-focused, opinionated Arch Linux distribution for T2 MacBook Pro hardware with Hyprland compositor. VulcanOS provides system protection, organized tooling, and a cohesive visual experience for a single-user development workstation.

## Core Value

**Cohesive, recoverable, keyboard-driven.** The system should feel unified (one visual identity), be always recoverable (snapshots + kernel protection), and stay out of the way (minimal, fast, keyboard-first).

## Current State

**Shipped:** v2.0 Vulcan Appearance Manager (2026-01-30)

VulcanOS now has a unified Vulcan Appearance Manager that combines theme and wallpaper management into a single GTK4/Libadwaita application. Themes suggest matching wallpapers, profiles save coordinated appearance configurations, and theme changes propagate automatically to all 6 desktop components (waybar, wofi, swaync, hyprlock, kitty, alacritty).

**Current codebase:**
- 7,759 lines of Rust (vulcan-appearance-manager)
- 10 polished preset themes (8 dark, 2 light) with official color palettes
- 6 desktop components with automatic theme propagation
- Third-party app discovery for 6 themeable apps

## Next Milestone Goals

Planning next milestone. Options:
- **v2.1 Maintenance** — Tech debt cleanup (AppState integration, theme validation, wallpaper downloads)
- **v3.0 Backup System** — Core backup engine from deferred Phases 2-4
- **v3.0 New Feature** — TBD based on user needs

## Requirements

### Validated

Capabilities shipped and working:

**v1.0 Foundation:**
- ✓ Arch Linux ISO build system (archiso profile) — v1.0
- ✓ GNU Stow dotfile management structure — v1.0
- ✓ Hyprland compositor with modular config — v1.0
- ✓ Waybar status bar with custom modules — v1.0
- ✓ vulcan-todo MCP server (tasks, sprints, projects) — v1.0
- ✓ vulcan-vault MCP server (notes, semantic search) — v1.0
- ✓ Desktop integration scripts (vulcan-menu, themes, etc.) — v1.0
- ✓ T2 MacBook Pro hardware support (linux-t2 kernel) — v1.0
- ✓ Speech-to-text via hyprwhspr — v1.0
- ✓ T2 kernel protection (pacman hooks, boot verification) — v1.0
- ✓ vulcan-wallpaper-manager (per-monitor wallpaper GUI) — v1.0
- ✓ vulcan-theme CLI (applies themes to desktop tools) — v1.0
- ✓ 8 built-in color themes — v1.0

**v2.0 Appearance Manager:**
- ✓ Unified Vulcan Appearance Manager (replaces theme-manager + wallpaper-manager) — v2.0
- ✓ Theme browser with color preview cards — v2.0
- ✓ Per-monitor wallpaper assignment with profile save/load — v2.0
- ✓ Theme-wallpaper binding (themes suggest wallpapers) — v2.0
- ✓ Unified profiles (theme + wallpaper + binding mode) — v2.0
- ✓ Theme propagation to 6 components (waybar, wofi, swaync, hyprlock, kitty, alacritty) — v2.0
- ✓ App self-theming (GUI uses active theme colors) — v2.0
- ✓ 10 polished preset themes (8 dark, 2 light) — v2.0
- ✓ Third-party app discovery with marketplace links — v2.0
- ✓ Desktop integration (vulcan-menu, archiso sync) — v2.0

### Active

No active requirements — next milestone not started.

### Out of Scope

- Automatic third-party theme installation — discovery links only
- AI wallpaper generation on-demand — pre-made wallpapers bundled
- Cloud sync for themes/wallpapers — local dotfiles + git sync
- Non-T2 hardware support — personal workstation only
- Core backup system (Phases 2-4) — deferred from v1.0

## Context

**Hardware:** 2019 MacBook Pro 16" with T2 chip. Requires linux-t2 kernel from arch-mact2 repo.

**Shipped apps:**
- `vulcan-appearance-manager/` — GTK4/Relm4, 7,759 lines, unified theme + wallpaper management
- `vulcan-theme` CLI — Bash script for theme application via envsubst templates

**Tech debt from v2.0:**
- AppState state machine created but not integrated into UI components
- parse_and_validate() security function exists but bypassed by theme_storage.rs
- BindingMode transition from ThemeBound to CustomOverride on manual wallpaper change not implemented
- 7 of 10 themes missing wallpapers (LICENSE files document sources)

## Constraints

- **Framework:** GTK4/Libadwaita + Relm4 (established pattern)
- **Theme format:** `.sh` script format (vulcan-theme compatibility)
- **Wallpaper backend:** swww
- **Profile storage:** TOML files in ~/.config/vulcan-appearance-manager/

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Merge both apps into one | Themes and wallpapers are conceptually linked | ✓ Good — unified UX |
| Keep GTK4/Relm4 framework | Both existing apps use it; proven patterns | ✓ Good — consistent codebase |
| Theme suggests wallpaper (overridable) | Balance between cohesion and user choice | ✓ Good — BindingMode works well |
| Pre-made wallpapers, not on-demand AI | Simpler, faster UX | ✓ Good — 3 downloaded, sources documented |
| Discovery-only for third-party apps | Avoid complexity of auto-installing themes | ✓ Good — marketplace links sufficient |
| ViewStack over TabView | Fixed application views, not user-managed tabs | ✓ Good — matches libadwaita HIG |
| BindingMode enum (3 states) | Explicit tracking of theme-wallpaper relationship | ✓ Good — UI clarity |
| STYLE_PROVIDER_PRIORITY_USER | Runtime CSS overrides brand defaults | ✓ Good — self-theming works |

---
*Last updated: 2026-01-30 after v2.0 milestone*
