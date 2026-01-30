# Missing Wallpapers Inventory

**Project:** VulcanOS v2.1 Maintenance
**Research Type:** Tech Debt Inventory
**Date:** 2026-01-30
**Status:** 7 of 10 themes missing wallpapers

## Executive Summary

VulcanOS includes 10 preset themes (8 dark, 2 light), but only 3 currently have wallpapers. The remaining 7 themes need wallpapers downloaded from their documented community sources. All sources are verified as GPL-compatible (MIT, GPL-3.0) or public domain.

**Quick Stats:**
- Total themes: 10
- Themes with wallpapers: 3 (30%)
- Themes missing wallpapers: 7 (70%)
- All sources documented: Yes
- License compliance: All sources GPL-compatible

## Themes With Wallpapers (3/10)

These themes already have wallpapers downloaded:

| Theme | Wallpaper File | Source | License | Resolution |
|-------|----------------|--------|---------|------------|
| Catppuccin Latte | catppuccin-latte.png | zhichaoh/catppuccin-wallpapers | MIT | 4K (3840x2160) |
| Catppuccin Mocha | catppuccin-mocha.png | zhichaoh/catppuccin-wallpapers | MIT | 4K (3840x2160) |
| Dracula | dracula.png | dracula/wallpaper | MIT | 8K (8001x4501) |

## Themes Missing Wallpapers (7/10)

### Dark Themes (5)

#### 1. Gruvbox Dark
- **Theme ID:** `gruvbox-dark`
- **Status:** Needs manual download
- **Color Palette:**
  - Dark background: #282828, #1d2021
  - Medium: #504945, #665c54, #7c6f64
  - Accents: #cc241d, #d79921, #98971a, #458588, #b16286, #689d6a
- **Expected Filename:** `gruvbox-dark.png`

#### 2. Nord
- **Theme ID:** `nord`
- **Status:** Needs manual download
- **Color Palette:**
  - Polar Night: #2E3440, #3B4252, #434C5E, #4C566A
  - Snow Storm: #D8DEE9, #E5E9F0, #ECEFF4
  - Frost: #8FBCBB, #88C0D0, #81A1C1, #5E81AC
  - Aurora: #BF616A, #D08770, #EBCB8B, #A3BE8C, #B48EAD
- **Expected Filename:** `nord.png`

#### 3. One Dark
- **Theme ID:** `onedark`
- **Status:** Needs manual download
- **Color Palette:**
  - Background: #282c34, #21252b
  - Foreground: #abb2bf
  - Accents: #e06c75, #98c379, #e5c07b, #61afef, #c678dd, #56b6c2
- **Expected Filename:** `onedark.png`

#### 4. Rosé Pine
- **Theme ID:** `rosepine`
- **Status:** Needs manual download
- **Color Palette:**
  - Base background: #191724
  - Surface: #1f1d2e
  - Overlay: #26233a
  - Muted: #6e6a86
  - Accents: #ebbcba, #eb6f92, #f6c177, #9ccfd8, #c4a7e7
- **Expected Filename:** `rosepine.png`

#### 5. Tokyo Night
- **Theme ID:** `tokyonight`
- **Status:** Needs manual download
- **Color Palette:**
  - Dark background: #1a1b26, #16161e, #24283b
  - Accents: #7aa2f7, #bb9af7, #7dcfff, #73daca, #9ece6a
- **Expected Filename:** `tokyonight.png`

### Light Themes (1)

#### 6. Gruvbox Light
- **Theme ID:** `gruvbox-light`
- **Status:** Needs manual download
- **Color Palette:**
  - Light background: #fbf1c7, #f9f5d7, #f2e5bc
  - Medium: #d5c4a1, #bdae93, #a89984
  - Accents: #cc241d, #d79921, #98971a, #458588, #b16286, #689d6a
- **Expected Filename:** `gruvbox-light.png`

### Custom Themes (1)

#### 7. Vulcan Forge
- **Theme ID:** `vulcan-forge`
- **Status:** AI-generated wallpaper needed
- **Color Palette:**
  - Background: #1c1917 (Forge Black), #292524 (Charcoal)
  - Accents: #f97316 (Forge Orange), #fbbf24 (Ember Gold)
- **Expected Filename:** `vulcan-forge.png`
- **Special Requirements:**
  - VulcanOS-specific branding
  - Volcanic/forge-inspired imagery
  - Dark, professional aesthetics
  - Optimized for 16:10 MacBook Pro displays

## Download Checklist

| # | Theme | Primary Source | License | Alternative Sources |
|---|-------|----------------|---------|---------------------|
| 1 | Gruvbox Dark | [AngelJumbo/gruvbox-wallpapers](https://github.com/AngelJumbo/gruvbox-wallpapers) | MIT | lukejagg/gruvbox-wallpapers, pavel-corsaghin/Gruvbox-Wallpapers |
| 2 | Gruvbox Light | [AngelJumbo/gruvbox-wallpapers](https://github.com/AngelJumbo/gruvbox-wallpapers) | MIT | lukejagg/gruvbox-wallpapers, pavel-corsaghin/Gruvbox-Wallpapers |
| 3 | Nord | [linuxdotexe/nordic-wallpapers](https://github.com/linuxdotexe/nordic-wallpapers) | GPL-3.0 | dxnst/nord-backgrounds (MIT), the-little-pedestrian/nord-wallpapers |
| 4 | One Dark | [joshdick/onedark.vim](https://github.com/joshdick/onedark.vim) | MIT | navarasu/onedark.nvim, one-dark/vscode-one-dark-theme |
| 5 | Rosé Pine | [rose-pine/wallpapers](https://github.com/rose-pine/wallpapers) | MIT | Syndrizzle/rose-pine-wallpapers, rosepinetheme.com/gallery |
| 6 | Tokyo Night | [enkia/tokyo-night-vscode-theme](https://github.com/enkia/tokyo-night-vscode-theme) | MIT | folke/tokyonight.nvim, EdenEast/nightfox.nvim |
| 7 | Vulcan Forge | AI-generated or custom creation | VulcanOS | Community contributions matching theme palette |

## Download Instructions

### General Process

For each theme (1-6):

1. **Navigate to source repository** (use primary source from table above)
2. **Browse wallpapers** matching the theme's documented color palette
3. **Download high-quality wallpaper:**
   - Minimum resolution: 1920x1080
   - Preferred: 4K (3840x2160) for HiDPI displays
   - Format: PNG (preferred) or JPEG
   - File size: Under 5MB
4. **Place in theme directory:**
   ```bash
   # Example for Gruvbox Dark
   cp downloaded-wallpaper.png /home/evan/VulcanOS/dotfiles/wallpapers/gruvbox-dark/gruvbox-dark.png
   ```
5. **Update LICENSE file** with attribution:
   - Source URL
   - Original filename
   - Author
   - License type
   - Download date
   - Resolution

### Vulcan Forge (Special Case)

The Vulcan Forge theme requires custom wallpaper creation:

**Option A: AI Generation**
- Prompt: "Dark minimalist wallpaper with volcanic/forge theme, ember orange (#f97316) and gold (#fbbf24) accents, professional developer aesthetic, 4K resolution"
- Tools: Midjourney, DALL-E, Stable Diffusion
- Aspect ratio: 16:10 for MacBook Pro

**Option B: Custom Design**
- Tools: Blender, Inkscape, GIMP
- Theme: Abstract volcanic/forge imagery or minimalist geometric patterns
- Colors: Must match palette (Forge Black #1c1917, Ember Orange #f97316)

**Option C: Community Contribution**
- Accept contributions matching the Vulcan Forge aesthetic
- Verify GPL-compatible licensing
- Document source in LICENSE file

## File Locations

```
VulcanOS/
└── dotfiles/
    └── wallpapers/
        ├── catppuccin-latte/
        │   ├── catppuccin-latte.png  ✓ (downloaded)
        │   └── LICENSE
        ├── catppuccin-mocha/
        │   ├── catppuccin-mocha.png  ✓ (downloaded)
        │   └── LICENSE
        ├── dracula/
        │   ├── dracula.png  ✓ (downloaded)
        │   └── LICENSE
        ├── gruvbox-dark/
        │   ├── gruvbox-dark.png  ✗ (missing)
        │   └── LICENSE
        ├── gruvbox-light/
        │   ├── gruvbox-light.png  ✗ (missing)
        │   └── LICENSE
        ├── nord/
        │   ├── nord.png  ✗ (missing)
        │   └── LICENSE
        ├── onedark/
        │   ├── onedark.png  ✗ (missing)
        │   └── LICENSE
        ├── rosepine/
        │   ├── rosepine.png  ✗ (missing)
        │   └── LICENSE
        ├── tokyonight/
        │   ├── tokyonight.png  ✗ (missing)
        │   └── LICENSE
        └── vulcan-forge/
            ├── vulcan-forge.png  ✗ (missing - needs creation)
            └── LICENSE
```

## Licensing Requirements

All wallpapers must comply with:

- **CC0** (Creative Commons Zero - Public Domain), OR
- **GPL-compatible licenses:**
  - MIT License
  - GPL (v2, v3)
  - Apache License
  - BSD License

**Forbidden:**
- Proprietary wallpapers without redistribution rights
- Unsplash (license restricts redistribution in software products)
- Stock photos without proper licensing
- Copyrighted artwork without permission

## Quality Standards

To maintain consistency across the wallpaper library:

| Criterion | Minimum | Recommended |
|-----------|---------|-------------|
| Resolution | 1920x1080 | 3840x2160 (4K) |
| Aspect Ratio | 16:9 | 16:10 (MacBook Pro native) |
| Format | JPEG | PNG (better quality) |
| File Size | N/A | Under 5MB |
| Color Depth | 8-bit | 10-bit for gradients |
| DPI | 72 | 144+ (HiDPI) |

**Aesthetic Requirements:**
- Must match theme's documented color palette
- Avoid busy/distracting imagery (development-focused environment)
- Prefer abstract, minimalist, or nature-inspired designs
- Test readability of overlaid UI elements (terminal, windows)

## Notes

### Color Palette Matching

When selecting wallpapers, prioritize wallpapers that:
1. Use the exact theme colors from LICENSE files
2. Maintain appropriate contrast for UI readability
3. Don't clash with terminal/editor color schemes

Example: A Nord wallpaper should predominantly feature Polar Night backgrounds (#2E3440 - #4C566A) with Frost or Aurora accents.

### Theme Integration

Theme files already reference wallpapers via `THEME_WALLPAPER` export:

```bash
# From nord.sh
export THEME_WALLPAPER="nord.png"
```

The Vulcan Appearance Manager resolves this to the full path:
```
dotfiles/wallpapers/nord/nord.png
```

No code changes needed - just place wallpapers with correct filenames.

### Community Sources

All documented sources have active communities:
- **GitHub repositories:** Check for recent commits, stars, and issues
- **r/unixporn:** Search for theme name, verify licensing in comments
- **Official theme sites:** Dracula, Catppuccin, Nord have official galleries

Avoid:
- Dead/archived repositories
- Wallpapers without clear licensing
- Low-quality or watermarked images

### Sync to ISO

After downloading wallpapers to `dotfiles/wallpapers/`, sync to ISO skeleton:

```bash
# Example sync command (not automated yet)
rsync -av dotfiles/wallpapers/ archiso/airootfs/etc/skel/.local/share/wallpapers/
```

This ensures fresh VulcanOS installations include all wallpapers.

## Next Steps

1. **Download 6 wallpapers** from documented sources (Gruvbox Dark/Light, Nord, One Dark, Rosé Pine, Tokyo Night)
2. **Create/commission 1 wallpaper** for Vulcan Forge
3. **Update LICENSE files** with source attribution for all new wallpapers
4. **Test wallpaper quality** by applying each theme and verifying aesthetics
5. **Sync to archiso skeleton** for ISO inclusion
6. **Commit changes** with proper attribution in commit message

## References

- **Wallpapers README:** `/home/evan/VulcanOS/dotfiles/wallpapers/README.md`
- **Theme Files:** `/home/evan/.config/themes/colors/*.sh`
- **LICENSE Files:** `/home/evan/VulcanOS/dotfiles/wallpapers/*/LICENSE`
- **VulcanOS Branding:** `/home/evan/VulcanOS/branding/wallpapers/`
