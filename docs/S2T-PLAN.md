# VulcanOS Speech-to-Text Implementation Plan

> **Version**: 1.0  
> **Created**: 2026-01-01  
> **Status**: Ready for Implementation  
> **Branch**: `feature/speech-to-text`

---

## Executive Summary

Build a fully integrated speech-to-text system for VulcanOS using **whisper-overlay** as the core engine, wrapped in a VulcanOS-native experience with wofi-based settings, Hyprland hotkey integration, and Waybar status display.

### Core Design Principles

1. **Follow VulcanOS patterns** - Use existing bash script conventions, wofi for menus, Waybar integration
2. **Simplicity over complexity** - Avoid systemd where direct process management suffices
3. **Bash-native configs** - Use bash-sourceable configuration files (no jq/yq dependencies)
4. **Manual activation** - User starts S2T via CLI (`vulcan-s2t start`) per user preference

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      VulcanOS S2T System                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────────┐    ┌─────────────────┐                   │
│   │  User (CLI)     │    │  User (Wofi)    │                   │
│   │  vulcan-s2t     │    │  Settings Menu  │                   │
│   └────────┬────────┘    └────────┬────────┘                   │
│            │                      │                             │
│            └──────┬───────────────┘                             │
│                   │                                             │
│            ┌──────▼──────┐    ┌─────────────────┐               │
│            │ Main Script │    │   Waybar Module │               │
│            │ (wrapper)   │    │   (status)      │               │
│            └──────┬──────┘    └─────────────────┘               │
│                   │                                             │
│     ┌─────────────┼─────────────┐                               │
│     │             │             │                               │
│     ▼             ▼             ▼                               │
│ ┌────────┐  ┌──────────┐  ┌──────────┐                         │
│ │ Server │  │  Client  │  │ Status   │                         │
│ │ (Python│  │ (whisper │  │ Checker  │                         │
│ │ STT)   │  │ overlay) │  │          │                         │
│ └────────┘  └──────────┘  └──────────┘                         │
│     │             │             │                               │
│     ▼             ▼             ▼                               │
│ ┌─────────────────────────────────────────────┐                 │
│ │              Audio Input (PipeWire)          │                 │
│ └─────────────────────────────────────────────┘                 │
│                                                                 │
│ ┌─────────────────────────────────────────────┐                 │
│ │              Text Output (wtype)             │                 │
│ └─────────────────────────────────────────────┘                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Dependencies

### System Dependencies (Official Repos)

| Package | Purpose | Status |
|---------|---------|--------|
| `wtype` | Wayland text input | ✅ Already installed (0.4-2) |
| `python` | Server runtime | ✅ Core system |
| `pipewire` + `wireplumber` | Audio I/O | ✅ Already installed |
| `notify-send` (libnotify) | Notifications | ✅ Available |

### AUR Dependencies

| Package | Purpose | Install Method |
|---------|---------|----------------|
| `python-faster-whisper` | Transcription backend | `yay -S python-faster-whisper` |
| `cargo` + `rust` | Build whisper-overlay client | `pacman -S rust` |

### Build-Time Dependencies

| Tool | Purpose | Status |
|------|---------|--------|
| `cargo` | Build whisper-overlay | Install from official repos |
| `git` | Clone repositories | ✅ Core system |

---

## File Structure

```
VulcanOS/
├── dotfiles/
│   ├── scripts/                          # Executable scripts (stow → ~/.local/bin)
│   │   ├── vulcan-s2t                   # Main CLI entry point
│   │   ├── vulcan-s2t-settings          # Wofi-based settings menu
│   │   └── vulcan-s2t-waybar            # Waybar status script
│   │
│   └── vulcan-s2t/                      # Config and data files
│       ├── .config/vulcan-s2t/
│       │   ├── server.conf.default      # Server configuration template
│       │   └── settings.conf.default    # User settings template
│       │
│       └── .local/share/vulcan-s2t/     # Runtime data directory
│           ├── realtime-stt-server.py   # Python server (cloned from repo)
│           └── server.log               # Server log file
│
├── docs/
│   ├── S2T.md                            # User guide
│   └── S2T-INSTALL.md                    # Installation instructions
│
├── archiso/
│   ├── packages.x86_64                   # +python-faster-whisper, rust, cargo
│   └── airootfs/etc/skel/
│       ├── .config/vulcan-s2t/           # Copy default configs
│       │   ├── server.conf.default
│       │   └── settings.conf.default
│       └── .local/bin/vulcan-s2t         # Symlink to settings script
│
└── CLAUDE.md                             # Update with S2T reference
```

---

## Configuration Format

### Settings Configuration (Bash-Sourceable)

**File**: `~/.config/vulcan-s2t/settings.conf`

```bash
# VulcanOS Speech-to-Text Settings
# This file is sourced by vulcan-s2t scripts

# Whisper model: tiny, base, small, medium, large-v3
S2T_MODEL="small"

# Hotkey: evdev key name (KEY_GRAVE for ` key)
S2T_HOTKEY="KEY_GRAVE"

# Language: empty for auto, or en, es, fr, de, etc.
S2T_LANGUAGE=""

# Server: host and port
S2T_HOST="localhost"
S2T_PORT="7007"

# Audio: device name or "default"
S2T_AUDIO_DEVICE="default"

# Output mode: "type" (direct input) or "clipboard" (copy to clipboard)
S2T_OUTPUT_MODE="type"

# Debug logging: "true" or "false"
S2T_DEBUG="false"
```

### Server Configuration (Bash-Sourceable)

**File**: `~/.config/vulcan-s2t/server.conf`

```bash
# VulcanOS S2T Server Configuration

# Model settings
S2T_MODEL_SIZE="small"      # tiny, base, small, medium, large-v3
S2T_DEVICE="auto"           # cuda, cpu, or auto

# Language (empty = auto-detect)
S2T_LANGUAGE=""

# Audio settings
S2T_SAMPLE_RATE=16000
S2T_CHANNELS=1

# Server settings
S2T_LOG_LEVEL="info"        # debug, info, warning, error
```

---

## Implementation Phases

### Phase 1: Foundation (Day 1)
- [ ] Create directory structure
- [ ] Set up config templates
- [ ] Create main CLI script (vulcan-s2t)
- [ ] Test server download mechanism

### Phase 2: Server & Client (Day 2)
- [ ] Implement server wrapper script
- [ ] Build/install whisper-overlay client
- [ ] Implement client wrapper (vulcan-dictation)
- [ ] Test start/stop/status flow

### Phase 3: Settings UI (Day 3)
- [ ] Create wofi settings menu
- [ ] Implement config editing scripts
- [ ] Add audio device discovery
- [ ] Test settings persistence

### Phase 4: Hyprland Integration (Day 4)
- [ ] Add hotkey bindings (Super+`)
- [ ] Add window rules for overlay
- [ ] Test dictation hotkey

### Phase 5: Waybar Integration (Day 5)
- [ ] Add Waybar module definition
- [ ] Style S2T icon states
- [ ] Position module correctly
- [ ] Test status updates

### Phase 6: Documentation & ISO (Day 6)
- [ ] Write user guide (S2T.md)
- [ ] Write installation guide (S2T-INSTALL.md)
- [ ] Update packages.x86_64
- [ ] Add skel files

### Phase 7: Testing & Polish (Day 7)
- [ ] Full integration testing
- [ ] T2 hardware validation
- [ ] Performance tuning
- [ ] Bug fixes and polish

---

## Command Reference

### Main CLI: `vulcan-s2t`

```bash
vulcan-s2t start          # Start server and client
vulcan-s2t stop           # Stop all S2T processes
vulcan-s2t restart        # Restart S2T
vulcan-s2t status         # Show current status
vulcan-s2t settings       # Open wofi settings menu
vulcan-s2t toggle         # Toggle server on/off
vulcan-s2t models         # List available models
vulcan-s2t audio-devices  # List audio input devices
vulcan-s2t help           # Show help
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `SUPER + `` (hold)` | Start dictation, release to transcribe |
| `SUPER + SHIFT + `` | Open settings menu |
| `SUPER + CTRL + `` | Toggle S2T server |

---

## Waybar Module

**Position**: `modules-left` (after window title, before clock)

**Definition**:
```json
"custom/s2t": {
    "exec": "~/.local/bin/vulcan-s2t-waybar",
    "interval": 2,
    "return-type": "json",
    "format": "{icon}",
    "format-icons": {
        "idle": "<span foreground='#928374'>󰍜</span>",
        "ready": "<span foreground='#a3be8c'>󰍜</span>",
        "recording": "<span foreground='#fb4934'>󰍁</span>"
    },
    "tooltip": true,
    "on-click": "vulcan-s2t settings",
    "on-click-right": "vulcan-s2t toggle"
}
```

---

## Hyprland Integration

### Keybindings (`bindings.conf`)

```conf
# Speech-to-Text
bind = $mainMod, grave, exec, vulcan-dictation
bind = $mainMod SHIFT, grave, exec, vulcan-s2t settings
bind = $mainMod CTRL, grave, exec, vulcan-s2t toggle
```

### Window Rules (`windowrules.conf`)

```conf
# Whisper overlay window
windowrule = match:class whisper-overlay, noanim on
windowrule = match:class whisper-overlay, noblur on
windowrule = match:class whisper-overlay, noshadow on
```

---

## Model Performance Guidelines

| Model | Size | Speed | Accuracy | T2 Recommendation |
|-------|------|-------|----------|-------------------|
| tiny | ~75MB | ⚡ Fastest | Low | ❌ |
| base | ~150MB | ⚡ Fast | Medium | ✅ Fast response |
| small | ~480MB | 🔥 Fast | Good | ⭐ **Default** |
| medium | ~1.5GB | 🔧 Moderate | High | ⚠️ CPU only |
| large-v3 | ~3GB | 🐌 Slow | Highest | ❌ T2 too slow |

**T2 MacBook Pro Recommendation**: Use `small` by default. For faster response in meetings, use `base`.

---

## Error Handling

All scripts use:
```bash
set -euo pipefail

# Check required commands
for cmd in python wtype notify-send; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "Error: Required command '$cmd' not found"
        exit 1
    fi
done
```

---

## Testing Checklist

### Unit Tests
- [ ] Config parsing works correctly
- [ ] Status command shows accurate state
- [ ] Waybar JSON is valid
- [ ] Settings save and reload

### Integration Tests
- [ ] `vulcan-s2t start` launches server and client
- [ ] `vulcan-s2t stop` cleans up all processes
- [ ] Hotkey triggers dictation
- [ ] Text inserts into kitty, browser, nvim
- [ ] Settings menu opens and changes work

### Hardware Tests (T2 MacBook Pro)
- [ ] Audio input captures speech
- [ ] Transcription accuracy acceptable
- [ ] Latency under 3 seconds for hold-to-speak
- [ ] No system freezes or audio conflicts
- [ ] External microphone works if needed

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| whisper-overlay not in AUR | Medium | Medium | Document manual cargo build |
| T2 CPU-only slow | High | Low | Default to small model, document tips |
| Audio input issues | Medium | Medium | Add device discovery, recommend external mic |
| Hotkey conflicts | Low | Low | Grave key unused, verify in bindings |
| evdev permissions | Low | Medium | Document input group requirement |

---

## Post-Implementation Tasks

- [ ] Update `CLAUDE.md` with S2T reference
- [ ] Add S2T to `KEYBINDINGS.md`
- [ ] Create demo video for documentation
- [ ] Consider adding to installer script (`scripts/install.sh`)

---

## Appendix A: Quick Reference Card

```
┌─────────────────────────────────────────────┐
│      VulcanOS Speech-to-Text Quick Ref      │
├─────────────────────────────────────────────┤
│                                             │
│  Start:      vulcan-s2t start               │
│  Stop:       vulcan-s2t stop                │
│  Settings:   vulcan-s2t settings            │
│  Status:     vulcan-s2t status              │
│                                             │
│  Dictate:    Hold SUPER + `                 │
│              Release to insert text         │
│                                             │
│  Toggle:     SUPER + CTRL + `               │
│  Settings:   SUPER + SHIFT + `              │
│                                             │
└─────────────────────────────────────────────┘
```

---

## Appendix B: Troubleshooting Common Issues

### "Server failed to start"
```bash
# Check logs
cat ~/.local/share/vulcan-s2t/server.log

# Check port in use
lsof -i :7007

# Restart
vulcan-s2t restart
```

### "Microphone not working"
```bash
# List audio devices
vulcan-s2t audio-devices

# Check PipeWire
pactl info | grep Source

# Test recording
arecord -f cd -d 5 test.wav
```

### "Text not inserting"
```bash
# Verify wtype works
wtype "hello"

# Check Wayland focus
hyprctl activewindow

# Try clipboard mode
# Edit ~/.config/vulcan-s2t/settings.conf
# S2T_OUTPUT_MODE="clipboard"
```

### "Model download failing"
```bash
# Manual download
python -c "import whisper; whisper.load_model('small')"

# Check disk space
df -h ~/.cache/
```

---

*Document generated for VulcanOS Speech-to-Text implementation*
