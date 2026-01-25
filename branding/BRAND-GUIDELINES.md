# VulcanOS Brand Guidelines

Official branding specification for VulcanOS - a developer-focused Arch Linux distribution forged for T2 MacBooks.

## Brand Philosophy

**"Forged for Developers"** - VulcanOS draws from the Roman god Vulcan, the divine smith and craftsman of the gods. The brand reflects:
- **Forge**: Powerful tools crafted with precision
- **Fire**: Warm orange/gold palette evoking molten metal and volcanic energy
- **Dark Workshop**: Deep charcoal backgrounds like a smith's forge at night
- **Precision**: Clean, minimal, code-focused aesthetic

---

## Color Palette

### Primary Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Ember** | `#f97316` | `249, 115, 22` | Primary accent, active states, CTAs |
| **Gold** | `#fbbf24` | `251, 191, 36` | Secondary accent, highlights, links |
| **Molten** | `#ea580c` | `234, 88, 12` | Hover states, emphasis |

### Background Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Obsidian** | `#1c1917` | `28, 25, 23` | Primary background |
| **Charcoal** | `#292524` | `41, 37, 36` | Elevated surfaces, cards |
| **Ash** | `#44403c` | `68, 64, 60` | Borders, dividers |

### Text Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| **White Smoke** | `#fafaf9` | `250, 250, 249` | Primary text |
| **Stone** | `#a8a29e` | `168, 162, 158` | Secondary text, muted |
| **Warm Gray** | `#78716c` | `120, 113, 108` | Disabled, placeholder |

### Semantic Colors

| Name | Hex | Purpose |
|------|-----|---------|
| **Success** | `#22c55e` | Confirmations, online status |
| **Warning** | `#fbbf24` | Cautions (uses Gold) |
| **Error** | `#ef4444` | Errors, critical alerts |
| **Info** | `#3b82f6` | Informational, links |

### Full Terminal Palette (ANSI Colors)

```
Black:    #1c1917 / #44403c (bright)
Red:      #ef4444 / #f87171 (bright)
Green:    #22c55e / #4ade80 (bright)
Yellow:   #fbbf24 / #fcd34d (bright)
Blue:     #3b82f6 / #60a5fa (bright)
Magenta:  #a855f7 / #c084fc (bright)
Cyan:     #06b6d4 / #22d3ee (bright)
White:    #a8a29e / #fafaf9 (bright)
```

---

## Typography

### Primary Font: JetBrains Mono

Used for: Terminal, code blocks, monospace UI elements

```css
font-family: "JetBrainsMono Nerd Font", "JetBrains Mono", "Fira Code", monospace;
```

| Weight | Usage |
|--------|-------|
| Regular (400) | Body code, terminal output |
| Medium (500) | UI labels, tabs |
| Bold (700) | Headings, emphasis |

### Secondary Font: Inter

Used for: UI text, documentation, wordmark

```css
font-family: "Inter", system-ui, -apple-system, sans-serif;
```

| Weight | Usage |
|--------|-------|
| Regular (400) | Body text |
| Medium (500) | UI elements |
| Bold (700) | Headings, wordmark |
| Black (900) | Display, hero text |

### Font Sizes

| Name | Size | Line Height | Usage |
|------|------|-------------|-------|
| xs | 11px | 1.4 | Labels, badges |
| sm | 12px | 1.4 | Secondary text |
| base | 13px | 1.5 | Body text, terminal |
| md | 14px | 1.5 | UI elements |
| lg | 16px | 1.5 | Section headers |
| xl | 20px | 1.3 | Page headers |
| 2xl | 28px | 1.2 | Hero text |
| 3xl | 42px | 1.1 | Display, logo |

---

## Logo

### Variants

1. **Full Logo** (`vulcan-logo-full.svg`)
   - Flipped Arch icon + "VulcanOS" wordmark
   - "Vulcan" in White Smoke, "OS" in Ember
   - Use for: Headers, splash screens, documentation

2. **Icon Only** (`vulcan-logo-icon.svg`)
   - Flipped Arch icon with ember gradient
   - Use for: Favicons, app icons, small spaces

3. **Monochrome** (`vulcan-logo-mono.svg`)
   - Single-color version
   - Use for: Watermarks, embossing, single-color contexts

### Logo Gradient

```css
/* Forge Glow - bottom to top */
background: linear-gradient(to top, #fbbf24 0%, #f97316 50%, #ea580c 100%);
```

### Clear Space

Maintain minimum padding equal to the height of the "V" in VulcanOS around the logo.

### Minimum Sizes

- Full logo: 80px width minimum
- Icon only: 24px minimum

---

## UI Components

### Buttons

```css
/* Primary Button */
.btn-primary {
  background-color: #f97316;
  color: #1c1917;
  font-weight: 600;
}
.btn-primary:hover {
  background-color: #ea580c;
}

/* Secondary Button */
.btn-secondary {
  background-color: #292524;
  color: #fafaf9;
  border: 1px solid #44403c;
}
.btn-secondary:hover {
  background-color: #44403c;
}
```

### Cards & Surfaces

```css
.card {
  background-color: #292524;
  border: 1px solid #44403c;
  border-radius: 8px;
}
```

### Focus States

```css
:focus {
  outline: 2px solid #f97316;
  outline-offset: 2px;
}
```

---

## CSS Variables

Use these variables for consistent theming across all VulcanOS components:

```css
:root {
  /* Primary */
  --vulcan-ember: #f97316;
  --vulcan-gold: #fbbf24;
  --vulcan-molten: #ea580c;

  /* Backgrounds */
  --vulcan-obsidian: #1c1917;
  --vulcan-charcoal: #292524;
  --vulcan-ash: #44403c;

  /* Text */
  --vulcan-white: #fafaf9;
  --vulcan-stone: #a8a29e;
  --vulcan-gray: #78716c;

  /* Semantic */
  --vulcan-success: #22c55e;
  --vulcan-warning: #fbbf24;
  --vulcan-error: #ef4444;
  --vulcan-info: #3b82f6;

  /* Typography */
  --font-mono: "JetBrainsMono Nerd Font", "JetBrains Mono", monospace;
  --font-sans: "Inter", system-ui, -apple-system, sans-serif;

  /* Spacing */
  --radius-sm: 4px;
  --radius-md: 8px;
  --radius-lg: 12px;
}
```

---

## Application Examples

### Terminal (Kitty)

- Background: Obsidian (`#1c1917`)
- Text: White Smoke (`#fafaf9`)
- Cursor: Ember (`#f97316`)
- Selection: Ember bg with Obsidian text
- Tab bar: Ember active, Charcoal inactive

### Status Bar (Waybar)

- Background: Obsidian (`#1c1917`)
- Module backgrounds: Charcoal (`#292524`)
- Active workspace: Ember (`#f97316`)
- Text: White Smoke / Stone

### Lock Screen (Hyprlock)

- Background: Gradient overlay on wallpaper
- Input field: Charcoal with Ember focus ring
- Text: White Smoke

---

## Don'ts

- Don't use the orange accent on light backgrounds
- Don't stretch or distort the logo
- Don't use colors outside the defined palette
- Don't use pure black (`#000000`) - use Obsidian instead
- Don't use pure white (`#ffffff`) - use White Smoke instead
- Don't combine the Forge palette with unrelated color schemes (like Gruvbox blue)

---

## Asset Downloads

All brand assets are located in `/branding/`:

```
branding/
├── logos/
│   ├── vulcan-logo-full.svg
│   ├── vulcan-logo-icon.svg
│   ├── vulcan-logo-mono.svg
│   └── vulcan-logo-gradient.png
├── wallpapers/
│   ├── vulcan-gradient.svg/png
│   ├── vulcan-minimal.svg
│   └── vulcan-login-bg.svg/png
├── icons/
│   └── vulcan-lock.svg
└── BRAND-GUIDELINES.md (this file)
```

---

*VulcanOS - Forged for Developers*
