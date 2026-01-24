# Project Research Summary

**Project:** VulcanOS Unified Appearance Manager
**Domain:** Desktop appearance management (theme + wallpaper coordination)
**Researched:** 2026-01-24
**Confidence:** HIGH

## Executive Summary

The VulcanOS Unified Appearance Manager merges two existing GTK4/Relm4 applications (vulcan-theme-manager and vulcan-wallpaper-manager) into a single cohesive interface. This is a **subsequent milestone** project that builds on proven foundations rather than a greenfield effort. Research confirms the existing GTK4/Relm4 stack is solid and should be preserved.

**Recommended approach:** Tab-based UI merging pattern that preserves existing components while adding three new integration layers: (1) theme-wallpaper binding service, (2) shared CSS infrastructure for third-party apps, and (3) app discovery system. The critical architectural decision is to **delegate theme application to the existing vulcan-theme CLI** rather than reimplementing templating logic in Rust. This avoids duplicating 490 lines of working bash code and maintains consistency between CLI and GUI behavior.

**Key risks:** The most dangerous pitfall is state synchronization drift between three sources of truth (GUI state, CLI state, live system state). Prevention requires establishing the live system as the single source of truth with explicit state transitions for preview/apply operations. The second critical risk is theme-wallpaper binding coherence — when themes suggest wallpapers but users have multi-monitor profiles, conflicts arise. This requires an explicit binding mode system (theme-controlled vs profile-controlled vs user-override).

## Key Findings

### Recommended Stack

The existing GTK4/Relm4 foundation is validated and should be kept unchanged. Research identifies minimal stack additions for unified functionality.

**Core technologies (existing):**
- **GTK4 0.10.3** — GUI toolkit with excellent Wayland support, already used
- **Relm4 0.10.1** — Elm-inspired reactive framework for component communication, already used
- **libadwaita 0.8.1** — Modern GNOME styling for integrated appearance, already used
- **tokio 1.x** — Async runtime for wallpaper backend (swww calls), already used
- **serde/serde_json 1.0** — Data serialization for profiles, already used

**New additions (minimal):**
- **toml 0.8** — Unified config format (TOML beats JSON/YAML for readability and Rust conventions)
- **walkdir 2.x** — Directory traversal for third-party app discovery
- **notify 6.x** — File system watching (OPTIONAL, defer to post-MVP if users request live theme reload)

**Explicit rejections:**
- **cssparser** — Not needed for MVP, envsubst template approach works well
- **D-Bus crates** — Over-engineering for single-user desktop system
- **Custom config parsers** — TOML + serde handles everything

### Expected Features

Research confirms most table-stakes features already exist in the separate apps. Unified app adds coordination features as differentiators.

**Must have (table stakes):**
- Theme browser with visual preview — ✅ Already implemented
- Apply theme system-wide (GTK, Qt, terminal, compositor) — ✅ Already implemented via vulcan-theme CLI
- Wallpaper picker with thumbnails — ✅ Already implemented
- Per-monitor wallpaper assignment — ✅ Already implemented via swww
- Save/load profiles — ✅ Already implemented for wallpapers
- Instant apply with live preview — ✅ Already implemented

**Should have (competitive differentiators):**
- **Theme-suggested wallpapers** — Each theme includes recommended wallpaper (LOW complexity, existing field)
- **Wallpaper-suggested themes** — Extract colors from wallpaper, rank themes by similarity (MEDIUM complexity, pywal/matugen pattern)
- **Unified theme+wallpaper profiles** — Save coordinated appearance as single profile (LOW complexity, combine existing storage)
- **Third-party app discovery** — Detect VS Code, browsers, terminals and show theme status (MEDIUM complexity, filesystem scanning)
- **Panoramic wallpaper splitting** — ✅ Already implemented

**Defer (v2+):**
- Automatic theme generation from wallpaper (anti-feature — pywal produces ugly themes)
- Online theme marketplace (anti-feature — maintenance burden, quality issues)
- Per-application theme overrides (anti-feature — creates visual inconsistency)
- Animated wallpapers (performance impact, battery drain)
- Cloud sync (privacy concerns, network dependency)

### Architecture Approach

Tab-based merging pattern using libadwaita TabView to combine two apps without complete rewrite. The architecture preserves clean separation of concerns while adding three new service layers for integration.

**Major components:**
1. **AppModel (new)** — Tab orchestration with message routing, coordinates existing components via Relm4 channels
2. **theme_tab/** (existing) — Theme browser, editor, preview panel from theme-manager (moved, not rewritten)
3. **wallpaper_tab/** (existing) — Monitor layout, wallpaper picker, profile manager from wallpaper-manager (moved, not rewritten)
4. **binding_tab/** (new) — Theme-wallpaper binding editor, binding list
5. **apps_tab/** (new) — Third-party app discovery browser, config editor
6. **binding_manager service** (new) — Coordinates theme+wallpaper application atomically
7. **css_generator service** (new) — Generates app-specific CSS from theme model
8. **app_discovery service** (new) — Scans ~/.config/ for supported apps

**Critical pattern:** CLI tool delegation. The Rust app calls `vulcan-theme set <id>` via Command::new() rather than reimplementing envsubst templating. This preserves the battle-tested bash implementation and maintains consistency.

### Critical Pitfalls

Research identified 13 pitfalls across critical/moderate/minor severity. Top 5 that could derail the project:

1. **State synchronization drift (CRITICAL)** — Three sources of truth (GUI, CLI, live system) can diverge. **Prevention:** Make live system state the single source of truth. Query actual state on startup (`swww query`, `vulcan-theme current`). Use explicit state transitions (Idle/Previewing/Applying) with stateless preview implementation (apply new + store original, cancel = restore original). Add startup reconciliation to detect and warn about inconsistencies.

2. **Shell script parsing fragility (CRITICAL)** — Theme files are bash .sh scripts parsed with regex. Breaks on nested quotes, spaces in paths, multiline strings. **Prevention:** Validate before parse with `bash -n theme.sh`. Normalize on save with consistent safe format (single-quoted values, double-quoted paths). Whitelist simple `export VAR="value"` syntax only. Add parser tests with malformed themes.

3. **Wallpaper backend assumption mismatch (CRITICAL)** — Code assumes swww but documentation references hyprpaper, old configs exist. Four failure modes: old config loaded, backend detection fails, transition period with both configs, future backend change requires rewrite. **Prevention:** Runtime backend detection (check which daemon is running). Backend abstraction trait (WallpaperBackend with apply/query/preload methods). Explicit backend selection in UI with detected backend shown in status bar.

4. **Component lifecycle memory leaks (CRITICAL)** — Relm4 Controllers for child components stored in parent without cleanup. GTK4 cairo renderer has known ~70kb leak per window. Memory grows with each preview. **Prevention:** Explicit cleanup with `Controller.take()` and drop when dialog closes. Detach read-only child components. Update colors in existing widgets rather than recreating preview panel. Resource pooling for thumbnails with LRU cache.

5. **Theme-wallpaper binding coherence (CRITICAL)** — Theme files have optional THEME_WALLPAPER field. Wallpaper profiles have per-monitor paths. Conflicts arise when both exist, single vs multi-monitor mismatch, profile switching, missing wallpaper files, circular dependencies. **Prevention:** Explicit binding modes (ThemeControlled, ProfileControlled, ThemeDefaulted, UserOverride). Per-monitor theme wallpaper support in theme format. Binding state saved in profile. Clear UI indicator (lock icon when wallpaper "locked to theme"). Smart application order based on binding mode.

## Implications for Roadmap

Based on research, the project naturally divides into 4 phases following dependency order and risk mitigation priorities.

### Phase 1: Foundation Architecture (Week 1)
**Rationale:** Must establish single source of truth pattern and backend abstraction before building unified UI. All critical pitfalls (#1, #2, #3) must be addressed in foundation to avoid rewrites later.

**Delivers:**
- New vulcan-appearance-manager crate with merged models and services
- State management architecture with explicit transitions (Idle/Previewing/Applying)
- Wallpaper backend abstraction trait (supports swww + hyprpaper)
- Hardened theme parser with validation
- Shared CSS module extracted from duplicated code

**Addresses features:**
- Infrastructure for all future features (no user-facing features yet)

**Avoids pitfalls:**
- Pitfall #1: State synchronization drift (establishes live system as truth)
- Pitfall #2: Shell script parsing fragility (hardens parser)
- Pitfall #3: Wallpaper backend mismatch (abstracts backends)
- Pitfall #13: Hardcoded paths (audit and fix /home/evan references)

**Research needs:** NONE (standard patterns)

### Phase 2: Component Integration & Tab Merging (Week 1-2)
**Rationale:** With foundation solid, merge existing components into tabbed interface. Memory profiling must happen during this phase to catch leaks early. CSS conflicts must be tested with extreme themes.

**Delivers:**
- app.rs with libadwaita TabView skeleton
- theme_tab/ with moved theme-manager components
- wallpaper_tab/ with moved wallpaper-manager components
- Existing functionality working in tabs (feature parity with separate apps)
- Memory leak prevention patterns implemented

**Addresses features:**
- Theme browser (table stakes, existing)
- Wallpaper picker (table stakes, existing)
- Visual theme preview (table stakes, existing)
- Per-monitor wallpaper (table stakes, existing)

**Avoids pitfalls:**
- Pitfall #4: Component lifecycle memory leaks (profile memory, implement cleanup)
- Pitfall #6: GTK CSS cascading conflicts (namespace app CSS, test with extreme themes)
- Pitfall #7: Config file format fragmentation (format abstraction layer)
- Pitfall #12: No validation on theme import (validate before parse, sandbox)

**Research needs:** NONE (Relm4 tab patterns well-documented)

### Phase 3: Theme-Wallpaper Binding System (Week 2)
**Rationale:** Core differentiation feature. Requires Phase 1's state management foundation and Phase 2's merged UI. This phase addresses the most complex architectural challenge (binding coherence, pitfall #5).

**Delivers:**
- models/binding.rs with ThemeWallpaperBinding struct
- services/binding_manager.rs with binding mode logic
- binding_tab/ UI for creating/editing bindings
- Modified theme_applier.rs with binding hooks
- Unified profile format (theme_id + wallpaper_profile + binding_mode)

**Addresses features:**
- Theme-suggested wallpapers (differentiator, LOW complexity)
- Unified theme+wallpaper profiles (differentiator, LOW complexity)
- Apply theme system-wide atomically with wallpaper (table stakes enhancement)

**Avoids pitfalls:**
- Pitfall #5: Theme-wallpaper binding coherence (explicit binding modes)
- Pitfall #8: Live reload race conditions (atomic writes, synchronous reload)

**Research needs:** NONE (data model and state patterns established in Phase 1)

### Phase 4: Third-Party App Discovery & Polish (Week 3)
**Rationale:** Adds competitive differentiator (app discovery) after core functionality stable. Polish items (thumbnail performance, undo) can be prioritized based on testing feedback.

**Delivers:**
- services/app_discovery.rs with extensible discovery algorithm
- services/css_generator.rs (start with VS Code, add incrementally)
- apps_tab/ UI showing discovered apps and theme status
- Thumbnail cache with async loading
- Undo/history stack (separate for themes and wallpapers)

**Addresses features:**
- Third-party app discovery (differentiator, MEDIUM complexity)
- Wallpaper-suggested themes (differentiator, MEDIUM complexity — color extraction)

**Avoids pitfalls:**
- Pitfall #9: Thumbnail performance (implement cache, async generation)
- Pitfall #10: Undo complexity (separate stacks, diff-based history)
- Pitfall #11: File picker default directory (set to known locations)

**Research needs:** MEDIUM for wallpaper color extraction
- Need research on color extraction algorithms (palette generation)
- Need research on color similarity/matching algorithms for theme ranking
- Investigate existing libraries (image-rs palette extraction, CIEDE2000 color distance)
- Could use `/gsd:research-phase` if color matching proves complex

### Phase Ordering Rationale

1. **Foundation first** — Addresses all critical architectural risks (state drift, backend abstraction, parser hardening) before building UI. Prevents rewrites.

2. **Component merge second** — Once foundation solid, moving existing components is low-risk. Memory profiling during this phase catches leaks before adding complex features.

3. **Binding system third** — Core differentiation feature depends on Phase 1's state management and Phase 2's merged UI. This is the complex integration challenge that defines product value.

4. **Discovery and polish last** — Competitive features that can be prioritized based on user testing. Color extraction for wallpaper-suggested themes is only moderately complex research item.

**Dependency chain:**
```
Phase 1 (Foundation)
  ↓
Phase 2 (Component Integration) — depends on state management, backend abstraction
  ↓
Phase 3 (Binding System) — depends on merged UI, state management
  ↓
Phase 4 (Discovery & Polish) — depends on stable core functionality
```

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 4 (wallpaper-suggested themes):** Color extraction and matching algorithms moderately complex. If color similarity proves difficult, use `/gsd:research-phase` for palette generation and CIEDE2000 color distance research.

Phases with standard patterns (skip research-phase):
- **Phase 1 (Foundation):** State management, backend abstraction, parser hardening are standard Rust patterns
- **Phase 2 (Component Integration):** Relm4 tab merging well-documented, memory profiling is standard testing
- **Phase 3 (Binding System):** Data model and coordination patterns established in Phase 1 architecture

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All versions verified from docs.rs, existing stack proven in production |
| Features | MEDIUM | Table stakes confirmed via competitive analysis, differentiators based on existing codebase analysis and domain research |
| Architecture | HIGH | Existing codebase provides working components, tab-based merging is proven Relm4 pattern |
| Pitfalls | HIGH | State drift, parser fragility, backend mismatch directly observed in codebase; GTK4 memory leaks confirmed by upstream bug reports |

**Overall confidence:** HIGH

### Gaps to Address

Areas where research was inconclusive or needs validation during implementation:

- **Color extraction algorithm selection:** Phase 4 feature (wallpaper-suggested themes) requires color palette extraction from images and color similarity matching. Research identified pywal/matugen as prior art but didn't validate specific algorithms. Consider using `/gsd:research-phase` if implementation proves complex. Alternatively, defer to v2 if color matching is harder than expected.

- **Third-party app config format diversity:** App discovery research covered standard apps (VS Code, Firefox, Thunderbird) but actual config paths and CSS injection methods need per-app validation. Start with VS Code (JSON settings) as known-good case, expand incrementally during Phase 4.

- **Actual memory usage thresholds:** Pitfall #4 (memory leaks) prevention strategies are general best practices. Actual memory growth rates and acceptable thresholds need profiling during Phase 2 implementation. GTK4 cairo leak is ~70kb per window (upstream confirmed) but compounding effect with Relm4 controllers needs measurement.

- **Thumbnail generation performance:** Pitfall #9 mitigation suggests async generation and caching, but actual batch sizes and cache eviction policies need tuning based on real wallpaper directories (100 images vs 1000 images vs 10000 images). Defer optimization to Phase 4 based on user feedback.

- **Binding mode UX design:** Phase 3's binding coherence solution (ThemeControlled/ProfileControlled/ThemeDefaulted/UserOverride) is architecturally sound but UX presentation needs design iteration. Mock up UI during Phase 3 planning to validate users can understand the mental model.

## Sources

### Primary (HIGH confidence)
- [GTK4 Rust Bindings Documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/)
- [Relm4 Book](https://relm4.org/book/stable/)
- [Cargo Configuration Reference](https://doc.rust-lang.org/cargo/reference/config.html)
- [Relm4 GitHub Repository](https://github.com/Relm4/Relm4)
- [swww GitHub - Wayland Wallpaper Solution](https://github.com/LGFae/swww)
- [Hyprland Wiki - Wallpapers](https://wiki.hypr.land/Useful-Utilities/Wallpapers/)
- VulcanOS codebase (vulcan-theme-manager, vulcan-wallpaper-manager, dotfiles/themes)

### Secondary (MEDIUM confidence)
- [GTK4 memory leak in cairo renderer - Issue #6404](https://gitlab.gnome.org/GNOME/gtk/-/issues/6404)
- [Suspected memory leak in GTK4 NGL renderer - Issue #7045](https://gitlab.gnome.org/GNOME/gtk/-/issues/7045)
- [nwg-look GitHub Repository](https://github.com/nwg-piotr/nwg-look) — Wayland GTK theme manager patterns
- [KDE System Settings - Appearance Documentation](https://userbase.kde.org/System_Settings/Appearance)
- [JSON vs YAML vs TOML Comparison](https://dev.to/jsontoall_tools/json-vs-yaml-vs-toml-which-configuration-format-should-you-use-in-2026-1hlb)
- [ArchWiki: Desktop Entries](https://wiki.archlinux.org/title/Desktop_entries)

### Tertiary (LOW confidence)
- [Material You Color Generation - matugen](https://github.com/InioX/matugen) — Reference for color extraction anti-pattern
- [GTK CSS Theming - ArchWiki](https://wiki.archlinux.org/title/GTK) — CSS cascading issues
- Community discussions on wallpaper-theme coordination strategies
- Interior design color coordination principles (applied to digital themes)

---
*Research completed: 2026-01-24*
*Ready for roadmap: yes*
