# VulcanOS Keybindings

Keyboard-first workflow inspired by [Omarchy](https://learn.omacom.io/2/the-omarchy-manual/53/hotkeys).

**Main Modifier**: `Super` (Command key on Mac)

---

## Navigation

| Keybinding | Action |
|------------|--------|
| `Super + Space` | Application launcher |
| `Super + Escape` | Lock screen |
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
| `Super + P` | Pseudo-tile |
| `Super + Backspace` | Toggle transparency |

---

## Focus & Movement

### Focus (Arrow Keys)
| Keybinding | Action |
|------------|--------|
| `Super + Arrow` | Move focus in direction |

### Focus (Vim-style)
| Keybinding | Action |
|------------|--------|
| `Super + H/J/K/L` | Focus left/down/up/right |

### Swap Windows
| Keybinding | Action |
|------------|--------|
| `Super + Shift + Arrow` | Swap window in direction |
| `Super + Shift + H/J/K/L` | Swap (vim-style) |

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
| `Super + 1/2/3/4` | Jump to workspace |
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
| `Super + S` | Toggle scratchpad |
| `Super + Alt + S` | Move window to scratchpad |

---

## Window Grouping

| Keybinding | Action |
|------------|--------|
| `Super + G` | Toggle grouping |
| `Super + Alt + G` | Remove from group |
| `Super + Alt + Tab` | Cycle windows in group |
| `Super + Alt + 1/2/3/4` | Jump to window in group |
| `Super + Alt + Arrow` | Move window into group |

---

## Launching Apps

| Keybinding | Application |
|------------|-------------|
| `Super + Return` | Terminal |
| `Super + Shift + F` | File manager (Thunar) |
| `Super + Shift + T` | Activity monitor (btop) |
| `Super + Shift + N` | Neovim |
| `Super + Shift + G` | Lazygit |
| `Super + Shift + W` | Network settings (nmtui) |

---

## Clipboard

| Keybinding | Action |
|------------|--------|
| `Super + Ctrl + V` | Clipboard history |

---

## Screenshots & Capture

| Keybinding | Action |
|------------|--------|
| `Print` | Screenshot region (edit) |
| `Shift + Print` | Screenshot full to clipboard |
| `Alt + Print` | Screen recording |
| `Super + Print` | Color picker |
| `Super + Shift + Print` | Screenshot region to file |

Screenshots saved to: `~/Pictures/Screenshots/`
Recordings saved to: `~/Videos/`

---

## Notifications

| Keybinding | Action |
|------------|--------|
| `Super + ,` | Dismiss notification |
| `Super + Shift + ,` | Dismiss all |
| `Super + Ctrl + ,` | Toggle notification center |

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
Super + Escape       Lock

WORKSPACES
Super + 1-4          Switch workspace
Super + Tab          Next workspace
Super + S            Scratchpad

WINDOWS
Super + Arrow        Focus
Super + Shift + Arrow   Move
Super + =/- 		 Resize

APPS
Super + Shift + F    Files
Super + Shift + N    Neovim
Super + Shift + T    System monitor
Super + Shift + G    Git
```

---

## Configuration

Edit keybindings: `~/.config/hypr/bindings.conf`

Hyprland wiki: https://wiki.hyprland.org/Configuring/Binds/
