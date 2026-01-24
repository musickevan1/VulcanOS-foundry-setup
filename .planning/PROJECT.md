# VulcanOS

## What This Is

A development-focused, opinionated Arch Linux distribution for T2 MacBook Pro hardware with Hyprland compositor. VulcanOS provides system protection, organized tooling, and a cohesive visual experience for a single-user development workstation.

## Core Value

**Cohesive, recoverable, keyboard-driven.** The system should feel unified (one visual identity), be always recoverable (snapshots + kernel protection), and stay out of the way (minimal, fast, keyboard-first).

## Current Milestone: v2.0 Vulcan Appearance Manager

**Goal:** Unified theme and wallpaper management with preset theme bundles and consistent theming across all VulcanOS tools.

**Target features:**
- Combined Theme + Wallpaper Manager GUI (replaces both existing apps)
- 8-10 preset themes with suggested wallpapers
- Shared theming infrastructure (propagates to waybar, wofi, swaync, hyprlock, terminals)
- Third-party app theming discovery (VS Code, nvim, etc.)
- Wallpaper library synced with dotfiles/backup system

## Requirements

### Validated

Existing capabilities from previous milestone:

- ✓ Arch Linux ISO build system (archiso profile) — existing
- ✓ GNU Stow dotfile management structure — existing
- ✓ Hyprland compositor with modular config — existing
- ✓ Waybar status bar with custom modules — existing
- ✓ vulcan-todo MCP server (tasks, sprints, projects) — existing
- ✓ vulcan-vault MCP server (notes, semantic search) — existing
- ✓ Desktop integration scripts (vulcan-menu, themes, etc.) — existing
- ✓ T2 MacBook Pro hardware support (linux-t2 kernel) — existing
- ✓ Speech-to-text via hyprwhspr — existing
- ✓ vulcan-theme-manager (standalone theme editor) — v1.0
- ✓ vulcan-wallpaper-manager (per-monitor wallpaper GUI) — v1.0
- ✓ vulcan-theme CLI (applies themes to desktop tools) — v1.0
- ✓ 8 built-in color themes — v1.0

### Active

New requirements for v2.0 milestone:

**Unified Appearance Manager:**
- [ ] Single GTK4/Relm4 app replacing theme-manager and wallpaper-manager
- [ ] Theme browser with color preview and wallpaper suggestion
- [ ] Per-monitor wallpaper assignment (migrated from wallpaper-manager)
- [ ] Profile save/load for wallpaper configurations
- [ ] Theme-wallpaper binding (each theme suggests a default wallpaper)
- [ ] User can override theme's suggested wallpaper

**Preset Themes with Wallpapers:**
- [ ] 8-10 polished preset themes with distinct personalities
- [ ] Each theme includes matching wallpaper(s) in wallpaper library
- [ ] Well-known themes use existing community wallpapers
- [ ] Custom VulcanOS themes have AI-generated wallpapers (pre-made)
- [ ] Wallpaper library stored in dotfiles, synced with backup

**Theming Infrastructure:**
- [ ] Audit existing tools for Vulcan brand color consistency
- [ ] Shared CSS/variables imported by all VulcanOS tools
- [ ] Theme changes propagate to: waybar, wofi, swaync, hyprlock
- [ ] Terminal theming (kitty, alacritty) synced with theme
- [ ] Vulcan Rust apps use consistent theming

**Third-party App Discovery:**
- [ ] List installed apps that support theming
- [ ] Link to theme marketplaces/resources for each app
- [ ] No automatic theme installation (discovery only)

### Out of Scope

- Automatic third-party theme installation — discovery links only, user installs manually
- AI wallpaper generation on-demand — pre-made wallpapers bundled with themes
- Cloud sync for themes/wallpapers — local dotfiles + git sync only
- Non-T2 hardware support — this milestone is personal workstation only

## Context

**Hardware:** 2019 MacBook Pro 16" with T2 chip. Requires linux-t2 kernel from arch-mact2 repo.

**Existing apps to merge:**
- `vulcan-theme-manager/` — GTK4/Relm4, ~2048 lines, handles 50+ color variables
- `vulcan-wallpaper-manager/` — GTK4/Relm4, ~2000+ lines, per-monitor wallpapers with swww

**Theming infrastructure:**
- `vulcan-theme` CLI script — applies themes via envsubst templates
- `branding/vulcan-palette.css` — CSS custom properties for brand colors
- Template files in `dotfiles/themes/templates/` for each app

**Current pain points:**
- Two separate apps for related functionality
- Theme changes don't automatically suggest matching wallpapers
- Inconsistent color usage across some tools
- No unified "appearance" concept

## Constraints

- **Framework:** GTK4/Libadwaita + Relm4 (match existing Vulcan Rust apps)
- **Code reuse:** Merge and refactor existing apps (not fresh start)
- **Theme format:** Keep existing `.sh` script format (vulcan-theme compatibility)
- **Wallpaper backend:** swww (already configured for VulcanOS)
- **Profile storage:** TOML files in ~/.config/ (consistent with wallpaper-manager)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Merge both apps into one | Themes and wallpapers are conceptually linked; separate apps create friction | — Pending |
| Keep GTK4/Relm4 framework | Both existing apps use it; proven patterns; native GNOME look | — Pending |
| Theme suggests wallpaper (overridable) | Balance between cohesion and user choice | — Pending |
| Pre-made wallpapers, not on-demand AI | Simpler, faster UX; generation happens during development | — Pending |
| Discovery-only for third-party apps | Avoid complexity of auto-installing VS Code/nvim themes | — Pending |
| Shared CSS for theming propagation | Single source of truth for colors; templates can import | — Pending |

---
*Last updated: 2026-01-24 after milestone v2.0 initialization*
