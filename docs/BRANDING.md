# VulcanOS Brand Guidelines

A comprehensive guide to the VulcanOS visual identity system.

---

## Brand Story

**VulcanOS** draws its name from Vulcan, the Roman god of fire and forge - the divine craftsman who created weapons and tools for the gods. This identity embodies:

- **Craftsmanship**: Precision tools for developers who build
- **Power**: Raw capability harnessed through skill
- **Creation**: Transforming raw materials into refined products

The visual identity uses warm forge colors - molten orange, ember gold, and deep charcoal - evoking the glow of a forge against darkness.

---

## Color Palette

### Primary Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Forge Black** | `#1c1917` | `28, 25, 23` | Primary backgrounds |
| **Forge Orange** | `#f97316` | `249, 115, 22` | Primary accent, CTAs, highlights |
| **Ember Gold** | `#fbbf24` | `251, 191, 36` | Secondary accent, hover states |

### Background Scale

| Name | Hex | Usage |
|------|-----|-------|
| Forge Black | `#1c1917` | Primary background |
| Charcoal | `#292524` | Secondary/elevated surfaces |
| Stone Gray | `#44403c` | Tertiary surfaces, borders |
| Ash | `#57534e` | Subtle borders, dividers |

### Text Colors

| Name | Hex | Usage |
|------|-----|-------|
| Primary | `#fafaf9` | Main text, headings |
| Secondary | `#a8a29e` | Descriptions, labels |
| Muted | `#78716c` | Placeholders, disabled |

### Accent Scale

| Name | Hex | Usage |
|------|-----|-------|
| Forge Orange | `#f97316` | Primary accent |
| Ember Gold | `#fbbf24` | Secondary accent |
| Flame Red | `#ef4444` | Tertiary/hot accent |
| Molten | `#fb923c` | Blend/transition |

### Semantic Colors

| State | Hex | Notes |
|-------|-----|-------|
| Success | `#22c55e` | Green - completion, valid |
| Error | `#ef4444` | Red - matches Flame Red |
| Warning | `#f97316` | Orange - matches Forge Orange |
| Info | `#3b82f6` | Blue - contrasting accent |

### Gradients

```css
/* Primary gradient - molten glow effect */
.forge-glow {
  background: linear-gradient(45deg, #f97316, #fbbf24);
}

/* Subtle background gradient */
.forge-ambient {
  background: linear-gradient(180deg, #1c1917 0%, #292524 100%);
}

/* Border gradient for active elements */
.forge-border {
  border-image: linear-gradient(45deg, #f97316, #fbbf24) 1;
}
```

---

## Logo

### Concept

The VulcanOS logo is an abstract geometric "V" mark inspired by:
- The letter V (Vulcan)
- An anvil silhouette (forge symbolism)
- Upward motion (building, progress)

### Logo Variants

| Variant | File | Use Case |
|---------|------|----------|
| Icon | `vulcan-logo-icon.svg` | Favicons, app icons, small contexts |
| Full | `vulcan-logo-full.svg` | Headers, splash screens, documentation |
| Mono | `vulcan-logo-mono.svg` | Single-color contexts, terminal, print |

### Specifications

**Icon Proportions**:
- Aspect ratio: 1:1 (square)
- Artboard: 64x64 px (scalable)
- Safe zone: 8px padding on all sides

**Full Logo Proportions**:
- Aspect ratio: 3.5:1
- Icon height equals text cap height
- Spacing: 12px between icon and wordmark

**Minimum Sizes**:
- Icon alone: 16px
- Full logo: 80px wide

**Clear Space**:
- Minimum clear space around logo: 25% of icon height
- Example: 64px icon requires 16px clear space

### Color Usage

**On Dark Backgrounds** (preferred):
- Icon: Forge Orange (`#f97316`) or gradient
- Wordmark: Primary text (`#fafaf9`)

**On Light Backgrounds**:
- Icon: Forge Orange (`#f97316`)
- Wordmark: Forge Black (`#1c1917`)

**Monochrome**:
- Light on dark: `#fafaf9`
- Dark on light: `#1c1917`

### Logo Don'ts

- Do not rotate or skew the logo
- Do not stretch or compress
- Do not change logo colors outside brand palette
- Do not place on busy/low-contrast backgrounds
- Do not add effects (shadows, outlines, etc.)
- Do not recreate with different proportions

---

## Typography

### Font Stack

| Use | Font | Weight | Fallback |
|-----|------|--------|----------|
| Code/Terminal | JetBrainsMono Nerd Font | Regular (400), Bold (700) | monospace |
| UI/Body | Inter | Regular (400), Medium (500), Bold (700) | system-ui, sans-serif |
| Headings | Inter | Bold (700), Black (900) | system-ui, sans-serif |

### Type Scale

```
Display:    32px / 2rem    - Hero headings
H1:         24px / 1.5rem  - Page titles
H2:         20px / 1.25rem - Section headings
H3:         16px / 1rem    - Subsections
Body:       14px / 0.875rem - Main content
Small:      12px / 0.75rem  - Captions, labels
Micro:      10px / 0.625rem - Status indicators
```

### Line Heights

- Headings: 1.2
- Body text: 1.5
- Code blocks: 1.6

---

## UI Components

### Buttons

**Primary Button**:
```css
.btn-primary {
  background: #f97316;
  color: #fafaf9;
  border-radius: 6px;
  padding: 8px 16px;
  font-weight: 500;
}
.btn-primary:hover {
  background: #fbbf24;
}
```

**Secondary Button**:
```css
.btn-secondary {
  background: transparent;
  color: #f97316;
  border: 1px solid #f97316;
  border-radius: 6px;
}
```

### Cards & Surfaces

```css
.card {
  background: #292524;
  border: 1px solid #44403c;
  border-radius: 8px;
  padding: 16px;
}
```

### Focus States

```css
:focus {
  outline: 2px solid #f97316;
  outline-offset: 2px;
}
```

### Window Borders (Hyprland)

- Active: Gradient from `#f97316` to `#fbbf24` at 45 degrees
- Inactive: `#44403c` at 60% opacity
- Border width: 2px
- Corner radius: 8px

---

## Application Contexts

### GRUB Bootloader

- Background: Forge Black (`#1c1917`)
- Selection highlight: Forge Orange (`#f97316`)
- Text: Primary (`#fafaf9`)
- Logo: Centered, ~200px wide

### Lock Screen (Hyprlock)

- Background: Wallpaper with blur (8px, 3 passes)
- Input field border: Forge Orange
- Text: Primary white
- Success feedback: `#22c55e`
- Failure feedback: `#ef4444`

### Terminal (Alacritty/Kitty)

- Background: Forge Black with 0.95 opacity
- Foreground: Primary text
- Cursor: Forge Orange, blinking block
- Selection: `#44403c`

### Status Bar (Waybar)

- Background: Charcoal (`#292524`)
- Module backgrounds: Stone Gray (`#44403c`)
- Active workspace: Forge Orange background
- Text: Primary, icons colored by function

### Launcher (Wofi)

- Background: Forge Black
- Input border: Forge Orange
- Selected item: Orange highlight
- Border radius: 10px

### Notifications (SwayNC)

- Background: Charcoal
- Border: Stone Gray
- Urgent: Flame Red accent
- Close button: Forge Orange

---

## Iconography

### Style Guidelines

- Line weight: 1.5-2px stroke
- Corner radius: Slightly rounded (2px)
- Color: Single-color, match context
- Size: 16px, 24px, or 32px grid

### System Icons

Use icon fonts (Nerd Fonts) for UI consistency:
- Workspace indicators
- Battery/volume/network status
- Application launchers

---

## File Assets

### Logo Files

```
branding/logos/
├── vulcan-logo-icon.svg      # Square icon mark
├── vulcan-logo-full.svg      # Icon + wordmark
└── vulcan-logo-mono.svg      # Single-color variant
```

### Future Assets

```
branding/wallpapers/
├── vulcan-forge-dark.png     # Primary wallpaper
├── vulcan-forge-minimal.png  # Minimal variant
└── vulcan-lockscreen.png     # Lock screen specific

branding/grub/
├── background.png            # GRUB background
├── theme.txt                 # Theme definition
└── icons/                    # Boot option icons
```

---

## Theme Integration

VulcanOS uses a template-based theming system. The Vulcan Forge theme integrates via:

```bash
# Activate Vulcan Forge theme
vulcan-theme set vulcan-forge

# Theme file location
~/.config/themes/colors/vulcan-forge.sh
```

The theme file exports color variables that get substituted into templates for each application (Hyprland, Waybar, terminals, etc.).

---

## Quick Reference

### Core Palette

```
Background:  #1c1917  ████  Forge Black
Surface:     #292524  ████  Charcoal
Border:      #44403c  ████  Stone Gray
Primary:     #f97316  ████  Forge Orange
Secondary:   #fbbf24  ████  Ember Gold
Text:        #fafaf9  ████  Warm White
```

### CSS Variables Template

```css
:root {
  /* Backgrounds */
  --bg-primary: #1c1917;
  --bg-secondary: #292524;
  --bg-surface: #44403c;

  /* Accents */
  --accent-primary: #f97316;
  --accent-secondary: #fbbf24;
  --accent-tertiary: #ef4444;

  /* Text */
  --fg-primary: #fafaf9;
  --fg-secondary: #a8a29e;
  --fg-muted: #78716c;

  /* Semantic */
  --success: #22c55e;
  --error: #ef4444;
  --warning: #f97316;
  --info: #3b82f6;
}
```

---

## Version

Brand Guidelines v1.0.0
Last updated: December 2024
