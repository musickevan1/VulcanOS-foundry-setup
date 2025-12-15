# VulcanOS Lock Screen Design Brief

## Objective
Design a sleek, minimal lock screen for VulcanOS using hyprlock that fits the Vulcan Forge theme aesthetic.

## Current State
- hyprlock.conf exists but the current design doesn't look good
- VulcanOS branding assets available in `branding/` directory
- Theme colors defined in Vulcan Forge palette

## Design Requirements

### Aesthetic Direction
- **Sleek and minimal** - avoid clutter, embrace negative space
- **VulcanOS branded** - subtle use of logo/identity, not overwhelming
- **Dark theme** - consistent with Vulcan Forge (`#1c1917` base)
- **Orange accents** - `#f97316` as primary accent sparingly

### Key Elements to Design
1. **Background** - What approach? Options:
   - Solid dark color with subtle gradient/glow
   - Blurred current wallpaper
   - Custom minimal graphic
   - Screenshot blur of current screen

2. **Time display** - Prominence, font size, position
   - Consider: large centered vs corner placement
   - 12hr vs 24hr format
   - Include date? Day of week?

3. **User identity** - How to show who's logging in
   - Username only vs avatar + username
   - Position relative to input field

4. **Password input** - Styling of the input field
   - Size, border style, placeholder text
   - Feedback colors (typing, success, failure)

5. **Logo usage** - If any
   - Full logo vs icon only vs none
   - Size and placement
   - Opacity/subtlety

### Vulcan Forge Color Palette
```
Background:     #1c1917 (stone-950)
Surface:        #292524 (stone-800)
Border:         #44403c (stone-700)
Text Primary:   #fafaf9 (stone-50)
Text Secondary: #a8a29e (stone-400)
Text Muted:     #78716c (stone-500)
Accent:         #f97316 (orange-500)
Accent Alt:     #fbbf24 (amber-400)
Success:        #22c55e (green-500)
Error:          #ef4444 (red-500)
```

### Available Assets
- `branding/logos/vulcan-logo-mono.svg` - monochrome wordmark
- `branding/logos/vulcan-logo-icon.svg` - icon only
- `branding/wallpapers/vulcan-login-bg.svg` - subtle geometric background
- `branding/wallpapers/vulcan-minimal.svg` - minimal variant

### Hyprlock Capabilities
- `background {}` - image or color, blur, noise, contrast adjustments
- `image {}` - display images (logos, avatars)
- `input-field {}` - password entry with extensive styling
- `label {}` - text with fonts, colors, dynamic content ($TIME, $USER, cmd[])
- `shape {}` - rectangles, circles for decorative elements

## Questions to Decide
1. Should the background use the current wallpaper (blurred) or a dedicated lock background?
2. How prominent should the VulcanOS branding be? Subtle watermark vs featured element?
3. Vertical layout (logo → time → input → username) or more creative arrangement?
4. Any animated elements? (hyprlock supports basic animations)
5. Should it match SDDM login theme for consistency?

## Reference Inspiration
- macOS lock screen (clean, time-focused, minimal)
- Modern Linux lock screens (swaylock, gtklock examples)
- Omarchy aesthetic if applicable

## Implementation Notes
- Test on actual hardware (T2 MacBook display)
- Consider different monitor sizes/orientations
- Ensure readability and usability
- Keep config in `dotfiles/hypr/.config/hypr/hyprlock.conf`
- Sync to `archiso/airootfs/etc/skel/.config/hypr/` for ISO

## Session Goal
Create a polished, production-ready lock screen configuration that:
- Looks professional and cohesive with VulcanOS
- Is minimal without being boring
- Functions well (clear feedback, readable text)
- Makes users feel good about the OS choice
