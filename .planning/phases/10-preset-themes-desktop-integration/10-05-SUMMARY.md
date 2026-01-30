---
phase: 10-preset-themes-desktop-integration
plan: 05
status: complete
subsystem: appearance-manager-ui
tags: [rust, gtk4, relm4, discovery, third-party-apps]

dependency-graph:
  requires: ["10-04"]
  provides: ["discovery-section-ui", "app-theming-visibility"]
  affects: ["10-06", "10-07", "10-08"]

tech-stack:
  added: []
  patterns: ["relm4-component", "expander-ui", "badge-styling"]

file-tracking:
  created:
    - vulcan-appearance-manager/src/components/discovery_section.rs
  modified:
    - vulcan-appearance-manager/src/components/mod.rs
    - vulcan-appearance-manager/src/components/theme_view.rs

decisions:
  - discovery-expander-placement
  - badge-based-status-ui
  - controller-list-pattern

metrics:
  duration: "3 minutes"
  completed: "2026-01-30"
---

# Phase 10 Plan 05: Discovery Section UI Summary

**One-liner:** Relm4 discovery section component with app status badges and marketplace links, integrated into Themes tab expander

## What Was Built

Created a GTK4/Relm4 component that displays third-party app theming status with visual badges and quick access to theme resources.

**DiscoverySection component:**
- Displays list of discovered apps from app_discovery service
- Section header with "Third-Party App Theming" title
- Refresh button to re-run app detection
- Descriptive subtitle explaining purpose
- List of AppRow components

**AppRow component:**
- App icon (32px, freedesktop icon spec)
- App name label
- Installed/Not Installed badge (conditional styling)
- Themed/Not Themed badge (only visible if installed)
- Link button opens marketplace URLs via open::that()

**Integration:**
- Embedded in ThemeView left panel (theme browser side)
- Used gtk::Expander for collapsible UI
- Positioned at bottom of theme browser area
- Does not dominate the primary theme selection UI

## Technical Implementation

**Component architecture:**
```
ThemeView (container)
└── gtk::Expander ("Third-Party Apps")
    └── DiscoverySection (controller)
        └── Vec<Controller<AppRow>>
            ├── AppRow (Neovim)
            ├── AppRow (Kitty)
            ├── AppRow (Alacritty)
            ├── AppRow (btop)
            ├── AppRow (VS Code)
            └── AppRow (Firefox)
```

**Badge styling classes:**
- `.badge` - Base badge style
- `.badge-success` - Green (installed)
- `.badge-muted` - Gray (not installed)
- `.badge-accent` - Blue (themed)
- `.badge-warning` - Orange (not themed)

**Message flow:**
1. AppRowMsg::OpenDocs → app_discovery::open_url()
2. DiscoverySectionMsg::Refresh → Re-discover apps, rebuild controllers

## Decisions Made

**1. Expander placement over separate tab**

Chose to embed discovery section in Themes tab using gtk::Expander rather than creating a separate "Discovery" tab.

- **Rationale:** Discovery is contextual to theming, not a primary workflow
- **Alternative rejected:** Separate tab would elevate discovery too much
- **Trade-off:** Less prominent but more contextually appropriate

**2. Badge-based status UI**

Used colored label badges for status rather than icons or switches.

- **Rationale:** Clear visual distinction, compact, no interaction needed
- **Implementation:** CSS classes control colors, #[watch] for dynamic updates
- **Pattern:** Installed/Not Installed always visible, Themed badge only when installed

**3. Controller list pattern over Factory**

Used `Vec<Controller<AppRow>>` instead of Relm4 Factory pattern.

- **Rationale:** Fixed list of 6 apps, rarely changes
- **Alternative:** Factory would be more efficient for dynamic lists
- **Trade-off:** Less performant refresh, but simpler implementation
- **Future:** Can migrate to Factory if app list becomes user-configurable

## Deviations from Plan

None - plan executed exactly as written.

## Testing & Verification

**Compilation:**
- `cargo check` passed with 0 errors
- `cargo build --release` succeeded in 1m 12s

**Code verification:**
- discovery_section.rs created (214 lines)
- AppRow and DiscoverySection components defined
- Integration with app_discovery service confirmed
- ThemeView integration successful

**Manual testing required:**
- Visual verification of discovery section in Themes tab
- Badge color/visibility correctness
- Link button opens browser to marketplace URLs
- Refresh button re-detects apps
- Expander expand/collapse behavior

## Integration Points

**Depends on:**
- app_discovery service (10-04) for app detection logic
- theme_view component for embedding

**Provides:**
- UI visibility into third-party app theming status
- Quick access to theme resources
- User awareness of theming possibilities

**Affects:**
- Future plans may add direct theme installation features
- Discovery section could become actionable (install themes)

## Files Modified

**Created:**
- `vulcan-appearance-manager/src/components/discovery_section.rs` (214 lines)
  - AppRow component (badge-based status display)
  - DiscoverySection container (manages app row list)

**Modified:**
- `vulcan-appearance-manager/src/components/mod.rs` (+3 lines)
  - Added `pub mod discovery_section;` export
- `vulcan-appearance-manager/src/components/theme_view.rs` (+16 lines)
  - Added DiscoverySection import
  - Added discovery field to ThemeViewModel
  - Added expander widget in view! macro
  - Created discovery controller in init()

## Next Phase Readiness

**Status:** Ready for 10-06 (wallpaper installation script)

**Blockers:** None

**Recommendations:**
- Consider adding CSS for badge styling (currently relies on GTK defaults)
- Test on system with various app install states
- Verify link button works with different default browsers

## Git History

**Commits:**
- `f0084de` - feat(10-05): create discovery section component
- `92d06df` - feat(10-05): integrate discovery section into Themes tab

**Lines changed:**
- +233 insertions
- 3 files modified

**Build artifacts:**
- vulcan-appearance-manager binary (release) updated
