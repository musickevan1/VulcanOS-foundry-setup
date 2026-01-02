# VulcanOS Keybindings

Keyboard-first workflow inspired by [Omarchy](https://learn.omacom.io/2/the-omarchy-manual/53/hotkeys).

**Main Modifier**: `Super` (Command key on Mac)

**Quick Help**: Press `Super + K` to show all hotkeys in a searchable menu.

---

## Navigation

| Keybinding | Action |
|------------|--------|
| `Super + Space` | Application launcher |
| `Super + Alt + Space` | Vulcan menu |
| `Super + Escape` | Power menu |
| `Super + K` | Show hotkeys guide |
| `Super + W` | Close window |
| `Ctrl + Alt + Delete` | Close all windows |
| `Super + Shift + Q` | Exit Hyprland |

---

## Window Management

| Keybinding | Action |
|------------|--------|
| `Super + T` | Toggle tiling/floating |
| `Super + O` | Toggle sticky (pin window) |
| `Super + F` | Fullscreen |
| `Super + Alt + F` | Fake fullscreen |
| `Super + J` | Toggle split direction (H/V) |
| `Super + P` | Pseudo-tile |
| `Super + Backspace` | Toggle transparency |

---

## Focus & Movement

### Focus (Arrow Keys)
| Keybinding | Action |
|------------|--------|
| `Super + Arrow` | Move focus in direction |

### Swap Windows
| Keybinding | Action |
|------------|--------|
| `Super + Shift + Arrow` | Swap window in direction |

---

## Resizing

| Keybinding | Action |
|------------|--------|
| `Super + =` | Grow width |
| `Super + -` | Shrink width |
| `Super + Shift + =` | Grow height |
| `Super + Shift + -` | Shrink height |

---

## Workspaces

| Keybinding | Action |
|------------|--------|
| `Super + 1-4` | Jump to workspace 1-4 |
| `Super + 5-0` | Extended workspaces 5-10 |
| `Super + Tab` | Next workspace |
| `Super + Shift + Tab` | Previous workspace |
| `Super + Ctrl + Tab` | Last workspace |
| `Super + [ / ]` | Previous/Next workspace |

### Move Window to Workspace
| Keybinding | Action |
|------------|--------|
| `Super + Shift + 1-0` | Move window to workspace |

### Scratchpad
| Keybinding | Action |
|------------|--------|
| `Super + Z` | Toggle scratchpad |
| `Super + Ctrl + Z` | Move window to scratchpad |
| `Super + Alt + Z` | Pop window from scratchpad |

---

## Window Grouping

| Keybinding | Action |
|------------|--------|
| `Super + G` | Toggle grouping |
| `Super + Alt + G` | Remove from group |
| `Super + Alt + Tab` | Cycle windows in group |
| `Super + Alt + 1-4` | Jump to window in group |
| `Super + Alt + Arrow` | Move window into group |

---

## Launching Apps

| Keybinding | Application |
|------------|-------------|
| `Super + Return` | Terminal (Kitty) |
| `Super + Shift + A` | Claude AI |
| `Super + Shift + B` | Browser (Chromium) |
| `Super + Shift + F` | File manager (Thunar) |
| `Super + Shift + T` | System monitor (btop) |
| `Super + Shift + N` | Neovim |
| `Super + Shift + G` | Lazygit |
| `Super + Shift + D` | Lazydocker |
| `Super + Shift + W` | Network settings (nmtui) |

---

## Clipboard

| Keybinding | Action |
|------------|--------|
| `Super + C` | Copy (universal - works in terminals and GUI apps) |
| `Super + V` | Paste (universal - works in terminals and GUI apps) |
| `Super + Ctrl + V` | Clipboard history |

---

## Screenshots & Capture

### Quick Screenshots (Super + S)
| Keybinding | Action |
|------------|--------|
| `Super + Shift + S` | Screenshot region |
| `Super + Ctrl + S` | Screenshot full screen |
| `Super + Alt + S` | Screenshot active window |

### Print Key Alternatives
| Keybinding | Action |
|------------|--------|
| `Print` | Screenshot region (edit with Swappy) |
| `Shift + Print` | Screenshot full to clipboard |
| `Alt + Print` | Screen recording |
| `Super + Print` | Color picker |

Screenshots saved to: `~/Pictures/Screenshots/`
Recordings saved to: `~/Videos/`

---

## Notifications

| Keybinding | Action |
|------------|--------|
| `Super + ,` | Toggle notification panel |
| `Super + Shift + ,` | Clear all notifications |
| `Super + Ctrl + ,` | Toggle Do Not Disturb |

---

## Style & Themes

| Keybinding | Action |
|------------|--------|
| `Super + Ctrl + Shift + Space` | Theme selector |
| `Super + Ctrl + Space` | Rotate wallpaper |
| `Super + Backspace` | Toggle transparency |

---

## Monitor Configuration

| Keybinding | Action |
|------------|--------|
| `Super + M` | Open HyprMon TUI (primary tool) |
| `Super + Alt + M` | Open nwg-displays GUI (visual tweaking) |
| `Super + F1` | Apply "3-monitor" monitor profile |
| `Super + F2` | Apply "laptop-only" monitor profile |
| `Super + F3` | Open profile selection menu |
| `Super + Shift + W` | Rotate wallpaper profiles |
| `Super + Alt + W` | Reload wallpapers (no profile change) |

---

## System

| Keybinding | Action |
|------------|--------|
| `Super + Ctrl + L` | Lock screen |
| `Super + Ctrl + I` | Toggle auto-lock (hypridle) |
| `Super + Ctrl + Escape` | System menu |
| `Super + Ctrl + U` | Update menu |

---

## Media Keys

### Volume
| Key | Action |
|-----|--------|
| `Volume Up/Down` | Adjust volume 5% |
| `Mute` | Toggle mute |
| `Super + Mute` | Switch audio output |

### Brightness
| Key | Action |
|-----|--------|
| `Brightness Up/Down` | Adjust 10% |
| `Kbd Brightness Up/Down` | Keyboard backlight |

### Playback
| Key | Action |
|-----|--------|
| `Play/Pause` | Toggle playback |
| `Next/Prev` | Track navigation |

---

## Mouse

| Action | Binding |
|--------|---------|
| Move window | `Super + Left Click + Drag` |
| Resize window | `Super + Right Click + Drag` |
| Scroll workspaces | `Super + Scroll` |

---

## Touchpad Gestures

| Gesture | Action |
|---------|--------|
| 3-finger swipe | Switch workspace |

---

## Quick Reference Card

```
ESSENTIALS
Super + Space        Launch app
Super + Return       Terminal
Super + W            Close window
Super + F            Fullscreen
Super + T            Float/Tile
Super + K            Show hotkeys
Super + Escape       Power menu

WORKSPACES
Super + 1-4          Switch workspace
Super + Tab          Next workspace
Super + Z            Scratchpad

WINDOWS
Super + Arrow        Focus
Super + Shift + Arrow   Move
Super + =/−          Resize
Super + J            Toggle split

CLIPBOARD
Super + C            Copy (universal)
Super + V            Paste (universal)

SCREENSHOTS
Super + Shift + S    Region
Super + Ctrl + S     Full screen
Super + Alt + S      Window

NOTIFICATIONS
Super + ,            Toggle panel
Super + Ctrl + ,     Do Not Disturb

APPS
Super + Shift + A    Claude AI
Super + Shift + B    Browser
Super + Shift + F    Files
Super + Shift + N    Neovim
Super + Shift + T    System monitor
Super + Shift + G    Git
```

---

## Configuration

Edit keybindings: `~/.config/hypr/bindings.conf`

Hyprland wiki: https://wiki.hyprland.org/Configuring/Binds/
