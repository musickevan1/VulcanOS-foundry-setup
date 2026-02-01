# Requirements: VulcanOS v2.1 Maintenance

**Defined:** 2026-01-30
**Core Value:** Cohesive, recoverable, keyboard-driven

## v2.1 Requirements

Maintenance milestone — wiring existing infrastructure, not building new features.

### Security

- [x] **SEC-01**: Theme loading uses parse_and_validate() for all paths (builtin, custom, import)
- [x] **SEC-02**: Theme import rejects files with dangerous patterns (command injection, eval, pipes)
- [x] **SEC-03**: Theme import rejects files with path traversal attempts

### UX Polish

- [x] **UX-01**: BindingMode auto-transitions to CustomOverride when user manually changes wallpaper after theme apply
- [x] **UX-02**: All 10 preset themes have matching wallpapers
- [x] **UX-03**: Missing wallpaper LICENSE files updated with attribution

### Architecture

- [x] **ARCH-01**: AppState state machine integrated into App coordinator
- [x] **ARCH-02**: Cancel Preview restores previous wallpaper (via RestoreWallpapers message)
- [x] **ARCH-03**: Preview/Apply/Cancel buttons respect state transitions (disabled during invalid states)

## Out of Scope

Explicitly excluded from v2.1:

| Feature | Reason |
|---------|--------|
| AppState error dialogs/spinners | Defer to v2.2 — requires UI design |
| Wallpaper preview state | Defer to v2.2 — separate feature |
| Multi-step undo/redo | Defer to v2.2 — beyond maintenance scope |
| vulcan-theme CLI validation | Separate tool, defer to v3.0 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SEC-01 | Phase 11 | Complete |
| SEC-02 | Phase 11 | Complete |
| SEC-03 | Phase 11 | Complete |
| UX-01 | Phase 12 | Complete |
| UX-02 | Phase 12 | Complete |
| UX-03 | Phase 12 | Complete |
| ARCH-01 | Phase 13 | Complete |
| ARCH-02 | Phase 13 | Complete |
| ARCH-03 | Phase 13 | Complete |

**Coverage:**
- v2.1 requirements: 9 total
- Mapped to phases: 9
- Complete: 9/9 (100%)

---
*Requirements defined: 2026-01-30*
*Last updated: 2026-02-01 (v2.1 milestone complete)*
