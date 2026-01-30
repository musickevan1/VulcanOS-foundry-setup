# Project Milestones: VulcanOS

## v2.0 Vulcan Appearance Manager (Shipped: 2026-01-30)

**Delivered:** Unified theme and wallpaper management with preset themes, binding system, and complete desktop theming infrastructure.

**Phases completed:** 6-10 (28 plans total)

**Key accomplishments:**

- Merged vulcan-theme-manager + vulcan-wallpaper-manager into unified GTK4/Libadwaita app
- Theme-wallpaper binding system with unified profiles (theme + wallpaper + binding mode)
- Complete theme propagation to 6 desktop components (waybar, wofi, swaync, hyprlock, kitty, alacritty)
- App self-theming via runtime CSS loading from active theme
- 10 polished preset themes (8 dark, 2 light) with verified official color palettes
- Third-party app discovery with marketplace links for 6 themeable apps
- Desktop integration via vulcan-menu Appearance submenu

**Stats:**

- 7,759 lines of Rust (vulcan-appearance-manager)
- 5 phases, 28 plans
- 33 requirements satisfied
- 6 days from start to ship

**Git range:** `feat(06-01)` → `docs(10-08)`

**What's next:** v2.1 maintenance — tech debt cleanup (AppState integration, theme validation, wallpaper downloads)

---

## v1.0 VulcanOS Foundation (Shipped: 2026-01-24)

**Delivered:** T2 MacBook Pro protection, wallpaper management, and desktop foundation for VulcanOS.

**Phases completed:** 1, 5 (11 plans total)

**Key accomplishments:**

- T2 kernel protection system (pacman hooks, boot verification, fallback kernel)
- VulcanOS Wallpaper Manager with multi-monitor support and profile system
- GTK4/Relm4 patterns established for future VulcanOS apps
- swww integration for efficient wallpaper transitions
- Archiso skeleton sync for fresh installs

**Stats:**

- Phase 1: 3 plans (T2 Kernel Protection)
- Phase 5: 8 plans (Wallpaper Manager)
- 2,000+ lines of Rust (vulcan-wallpaper-manager)

**Git range:** Initial development

**What's next:** v2.0 Vulcan Appearance Manager (unified theme + wallpaper app)

---

*Last updated: 2026-01-30*
