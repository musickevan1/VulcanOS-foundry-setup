# VulcanBar Touch Bar Redesign - Planning Session

## Context

VulcanBar is now functionally working with:
- Touch input detection
- Hyprland workspace switching (fixed privilege drop issue)
- Modular architecture with config.toml
- Battery, brightness, volume, clock, workspaces, fkeys modules

## Current Issues to Address

### 1. Workspace Number Mismatch
- Touch Bar workspace buttons don't match Waybar's workspace numbering
- Need to sync the displayed numbers/names with actual Hyprland workspace IDs

### 2. Touch Target Size
- Current buttons are too small for reliable touch input
- Touch Bar is 2170x60 pixels - need to maximize usable touch area
- Buttons need larger hit areas with adequate spacing

### 3. Layout Problems
- Time/date is not centered
- No clear visual hierarchy
- Module positioning needs rethinking

## Design Requirements

### Layout Goals
1. **Centered clock/date** - Primary focus, always visible
2. **Larger touch targets** - Minimum 100px wide buttons, preferably wider
3. **Clear visual separation** - Between functional groups
4. **Comfortable spacing** - Not too cramped, not wasted space

### Functional Requirements
1. **Workspaces** - Match Waybar numbering exactly
2. **Audio controls** - Volume up/down, mute toggle
3. **System info** - Battery, time/date
4. **Quick actions** - Brightness, maybe media controls

## Questions to Explore

### Layout Structure
- Should we use a 3-section layout (left | center | right)?
- How many workspace buttons to show? (Currently 10, maybe reduce to 5-6?)
- Should workspaces be on left or right side?

### Touch Bar Pages/Modes
- Should there be multiple "pages" you can swipe between?
- Ideas for pages:
  - **Main**: Workspaces + Clock + System
  - **Media**: Playback controls, volume, now playing
  - **System**: Brightness, WiFi toggle, Bluetooth, battery details
  - **Function Keys**: Traditional F1-F12 when holding Fn

### Button Ideas to Consider
- **Workspaces**: 1-5 or 1-10, with active indicator
- **Audio**: Vol-, Vol+, Mute (with icon change)
- **Media**: Prev, Play/Pause, Next
- **Brightness**: Brightness down/up
- **System**: Lock screen, screenshot, app launcher
- **Notifications**: Show/hide notification center
- **Do Not Disturb**: Toggle DND mode

### Visual Design
- Should buttons have icons, text, or both?
- Color coding for different button types?
- Active workspace highlight style?
- Should there be a subtle background gradient?

## Proposed Layout Options

### Option A: Balanced Three-Section
```
[WS1][WS2][WS3][WS4][WS5]  |  [◀][▶]  Thu Dec 12  12:15 AM  [🔊][🔆]  |  [🔋 85%]
      Workspaces             Media        Clock/Date        Quick      Battery
```

### Option B: Workspace-Centric
```
[1][2][3][4][5][6][7][8][9][10]  |  Thu Dec 12  12:15 AM  |  [🔉][🔊][🔆][🔅][🔋]
        Workspaces                      Clock/Date              System Controls
```

### Option C: Minimal with Pages
```
Main Page:   [WS1][WS2][WS3][WS4][WS5]  [Thu Dec 12  12:15 AM]  [Vol][Bri][Bat]
Media Page:  [⏮][⏯][⏭]  [Now Playing: Song Name]  [━━━━━━●━━━]  [🔊]
System Page: [WiFi][BT][DND][Lock][Screenshot][Settings]
```

### Option D: MacBook-Style Dynamic
```
[Esc]  [Workspaces...]  [        Clock/Date        ]  [Vol][Bri][Bat]
       Expandable area     Always centered/visible      Fixed controls
```

## Config Structure Proposal

```toml
[general]
height = 60
background = "#1a1a2e"
font_family = "JetBrains Mono"

[layout]
# Three sections: left, center, right
# Modules are assigned to sections
structure = "left|center|right"

[layout.left]
modules = ["workspaces"]
align = "start"

[layout.center]
modules = ["clock"]
align = "center"

[layout.right]
modules = ["volume", "brightness", "battery"]
align = "end"

[modules.workspaces]
count = 5  # Only show 5 workspaces
min_width = 80
show_numbers = true
active_color = "#7aa2f7"
inactive_color = "#414868"

[modules.clock]
format = "%a %b %d  %I:%M %p"
min_width = 300

[modules.volume]
icon_only = true
tap_action = "toggle_mute"
# Future: swipe for volume adjust?

[modules.brightness]
icon_only = true

[modules.battery]
show_percentage = true
min_width = 100
```

## Implementation Tasks (for next session)

1. **Sync workspace numbers with Waybar/Hyprland**
   - Query actual workspace IDs from Hyprland
   - Display correct numbers

2. **Implement centered layout**
   - Modify compositor to support left|center|right sections
   - Calculate positions to center the clock module

3. **Increase button sizes**
   - Update default min_width values
   - Adjust spacing calculations

4. **Add audio controls**
   - Volume module with mute toggle
   - Consider media playback controls

5. **Visual polish**
   - Better active/inactive states
   - Smoother transitions
   - Icon improvements

## Open Questions for Discussion

1. How should we handle more than N workspaces? Scroll? Overflow?
2. Should tapping clock do anything? (Maybe show calendar popup on main screen?)
3. Do we want haptic-style visual feedback on touch?
4. Should there be a gesture system? (Swipe left/right to change pages?)
5. How to handle Fn key for traditional function keys?

---

**Start next session with:** "Let's plan the VulcanBar redesign. Enter plan mode and let's work through the layout and features."
