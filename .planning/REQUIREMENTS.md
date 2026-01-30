# Requirements: VulcanOS v2.1 Maintenance

**Defined:** 2026-01-30
**Core Value:** Cohesive, recoverable, keyboard-driven

## v2.1 Requirements

Maintenance milestone — wiring existing infrastructure, not building new features.

### Security

- [ ] **SEC-01**: Theme loading uses parse_and_validate() for all paths (builtin, custom, import)
- [ ] **SEC-02**: Theme import rejects files with dangerous patterns (command injection, eval, pipes)
- [ ] **SEC-03**: Theme import rejects files with path traversal attempts

### UX Polish

- [ ] **UX-01**: BindingMode auto-transitions to CustomOverride when user manually changes wallpaper after theme apply
- [ ] **UX-02**: All 10 preset themes have matching wallpapers
- [ ] **UX-03**: Missing wallpaper LICENSE files updated with attribution

### Architecture

- [ ] **ARCH-01**: AppState state machine integrated into App coordinator
- [ ] **ARCH-02**: Cancel Preview restores previous wallpaper (currently broken)
- [ ] **ARCH-03**: Preview/Apply/Cancel buttons respect state transitions (disabled during invalid states)

## Out of Scope

Explicitly excluded from v2.1:

| Feature | Reason |
|---------|--------|
| AppState error dialogs/spinners | Defer to v2.2 — requires UI design |
| Wallpaper preview state | Defer to v2.2 — separate feature |
| Multi-step undo/redo | Defer to v2.2 — beyond maintenance scope |
| vulcan-theme CLI validation | Separate tool, defer to v3.0 |

## Traceability

Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SEC-01 | TBD | Pending |
| SEC-02 | TBD | Pending |
| SEC-03 | TBD | Pending |
| UX-01 | TBD | Pending |
| UX-02 | TBD | Pending |
| UX-03 | TBD | Pending |
| ARCH-01 | TBD | Pending |
| ARCH-02 | TBD | Pending |
| ARCH-03 | TBD | Pending |

**Coverage:**
- v2.1 requirements: 9 total
- Mapped to phases: 0
- Unmapped: 9

---
*Requirements defined: 2026-01-30*
*Last updated: 2026-01-30 after initial definition*
