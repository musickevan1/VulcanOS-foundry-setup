# VulcanOS Kitty Terminal

Kitty is a GPU-accelerated terminal emulator configured with VulcanOS Forge theming and developer-focused keybindings.

---

## Quick Reference

```
CLIPBOARD
Ctrl+Shift+C         Copy
Ctrl+Shift+V         Paste
Ctrl+Shift+S         Paste from selection

TABS
Ctrl+Shift+T         New tab
Ctrl+Shift+Q         Close tab
Ctrl+Shift+Right     Next tab
Ctrl+Shift+Left      Previous tab
Ctrl+1-9             Jump to tab #

WINDOWS (SPLITS)
Ctrl+Shift+Enter     New window (split)
Ctrl+Shift+W         Close window
Ctrl+Shift+]         Next window
Ctrl+Shift+[         Previous window

FONT SIZE
Ctrl+Shift+=         Increase font
Ctrl+Shift+-         Decrease font
Ctrl+Shift+Backspace Reset font

SCROLLING
Ctrl+Shift+Up/Down   Scroll line
Ctrl+Shift+PgUp/PgDn Scroll page
Ctrl+Shift+Home/End  Scroll to top/bottom
Ctrl+Shift+H         Show scrollback in pager
```

---

## Keybindings

### Clipboard

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+C` | Copy to clipboard |
| `Ctrl+Shift+V` | Paste from clipboard |
| `Ctrl+Shift+S` | Paste from selection |
| `Shift+Insert` | Paste from selection |

### Tab Management

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+T` | New tab |
| `Ctrl+Shift+Q` | Close tab |
| `Ctrl+Shift+Right` | Next tab |
| `Ctrl+Shift+Left` | Previous tab |
| `Ctrl+Shift+.` | Move tab forward |
| `Ctrl+Shift+,` | Move tab backward |
| `Ctrl+Shift+Alt+T` | Rename tab |
| `Ctrl+1` through `Ctrl+9` | Jump to tab 1-9 |

### Window Management (Splits)

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+Enter` | New window (split) |
| `Ctrl+Shift+N` | New OS window |
| `Ctrl+Shift+W` | Close window |
| `Ctrl+Shift+]` | Next window |
| `Ctrl+Shift+[` | Previous window |
| `Ctrl+Shift+F` | Move window forward |
| `Ctrl+Shift+B` | Move window backward |
| `Ctrl+Shift+`` ` | Move window to top |
| `Ctrl+Shift+R` | Resize window mode |
| `Ctrl+Shift+L` | Cycle layouts |

### Scrolling

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+Up` | Scroll up one line |
| `Ctrl+Shift+Down` | Scroll down one line |
| `Ctrl+Shift+Page Up` | Scroll up one page |
| `Ctrl+Shift+Page Down` | Scroll down one page |
| `Ctrl+Shift+Home` | Scroll to top |
| `Ctrl+Shift+End` | Scroll to bottom |
| `Ctrl+Shift+H` | Open scrollback in pager |

### Font Size

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+=` | Increase font size |
| `Ctrl+Shift+-` | Decrease font size |
| `Ctrl+Shift+Backspace` | Reset font size |

### Background Opacity

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+A` then `M` | Increase opacity |
| `Ctrl+Shift+A` then `L` | Decrease opacity |
| `Ctrl+Shift+A` then `1` | Set opacity to 100% |
| `Ctrl+Shift+A` then `D` | Reset to default |

### Utilities

| Keybinding | Action |
|------------|--------|
| `Ctrl+Shift+F5` | Reload config |
| `Ctrl+Shift+F6` | Debug config |
| `Ctrl+Shift+U` | Unicode character input |
| `Ctrl+Shift+E` | Open URL with hints |
| `Ctrl+Shift+G` | Show last command output |

---

## Theme: VulcanOS Forge

The terminal uses the Forge color scheme matching VulcanOS branding:

| Color | Hex | Usage |
|-------|-----|-------|
| Background | `#1c1917` | Stone/charcoal base |
| Foreground | `#fafaf9` | Off-white text |
| Cursor | `#f97316` | Orange accent |
| Selection | `#f97316` | Orange highlight |
| URL | `#fbbf24` | Gold links |
| Active Tab | `#f97316` | Orange tab bar |

**Color palette:**
- Red: `#ef4444` / `#f87171`
- Green: `#22c55e` / `#4ade80`
- Yellow: `#fbbf24` / `#fcd34d` (Ember Gold)
- Blue: `#3b82f6` / `#60a5fa`
- Magenta: `#a855f7` / `#c084fc`
- Cyan: `#06b6d4` / `#22d3ee`

---

## Features

### GPU Acceleration
Kitty uses OpenGL for rendering, providing smooth scrolling and low latency input even with complex terminal content.

### Tabs & Splits
Work with multiple terminals in a single window:
- **Tabs**: Independent terminal sessions in a tab bar
- **Windows**: Split the current tab into multiple panes

### Shell Integration
When enabled (default), provides:
- Clickable file paths and URLs
- Visual prompts marking command boundaries
- `Ctrl+Shift+G` to view last command output

### Image Support
Kitty supports inline images via the `icat` kitten:
```bash
kitty +kitten icat image.png
```

### URL Hints
Press `Ctrl+Shift+E` to highlight all URLs on screen with hint characters for quick keyboard-based opening.

### Unicode Input
Press `Ctrl+Shift+U` to search and insert Unicode characters by name.

---

## Configuration

Config file: `~/.config/kitty/kitty.conf`

### Common Customizations

**Change font size:**
```conf
font_size 12.0
```

**Adjust opacity:**
```conf
background_opacity 0.90
```

**Change tab bar position:**
```conf
tab_bar_edge bottom
```

**Disable cursor blinking:**
```conf
cursor_blink_interval 0
```

### Reload Config
After editing, press `Ctrl+Shift+F5` or run:
```bash
kill -SIGUSR1 $(pgrep kitty)
```

---

## Layouts

Cycle through layouts with `Ctrl+Shift+L`:

| Layout | Description |
|--------|-------------|
| `tall` | One large pane on left, stack on right |
| `fat` | One large pane on top, stack on bottom |
| `stack` | Only active window visible (like tabs) |
| `grid` | Equal size grid |
| `splits` | Arbitrary splits (default) |
| `horizontal` | All windows side by side |
| `vertical` | All windows stacked vertically |

---

## Tips & Tricks

### SSH with Kitty
Use `kitten ssh` for better integration:
```bash
kitten ssh user@host
```
This copies terminfo to the remote host automatically.

### Broadcast to All Windows
Type in all visible windows simultaneously:
1. Press `Ctrl+Shift+F7` to enable broadcasting
2. Type commands (sent to all windows)
3. Press `Ctrl+Shift+F7` again to disable

### Copy Last Command Output
After running a command, press `Ctrl+Shift+G` to view its output in a pager (useful for copying long output).

### Hyperlinks
Kitty detects and underlines URLs. `Ctrl+Click` to open, or use `Ctrl+Shift+E` for keyboard-only navigation.

---

## Comparison with Alacritty

VulcanOS includes both Kitty and Alacritty. Key differences:

| Feature | Kitty | Alacritty |
|---------|-------|-----------|
| Tabs/Splits | Built-in | None (use tmux) |
| Images | Supported | Not supported |
| GPU | OpenGL | OpenGL |
| Config | kitty.conf | TOML |
| Ligatures | Supported | Supported |
| Shell integration | Built-in | None |

**Default terminal**: Alacritty (via `Super+Return`)
**Alternative**: `kitty` command or modify `~/.config/hypr/bindings.conf`

---

## Resources

- Kitty Documentation: https://sw.kovidgoyal.net/kitty/
- Configuration Reference: https://sw.kovidgoyal.net/kitty/conf/
- Kittens (Extensions): https://sw.kovidgoyal.net/kitty/kittens_intro/
