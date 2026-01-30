# VulcanOS Wallpaper Library

This directory contains theme-specific wallpapers for VulcanOS preset themes.

## Directory Structure

Each theme has a dedicated subdirectory containing wallpapers that match its color palette:

```
wallpapers/
├── catppuccin-mocha/    # Catppuccin Mocha theme wallpapers
├── catppuccin-latte/    # Catppuccin Latte theme wallpapers
├── dracula/             # Dracula theme wallpapers
├── nord/                # Nord theme wallpapers
├── gruvbox-dark/        # Gruvbox Dark theme wallpapers
├── gruvbox-light/       # Gruvbox Light theme wallpapers
├── tokyonight/          # Tokyo Night theme wallpapers
├── rosepine/            # Rosé Pine theme wallpapers
├── onedark/             # One Dark theme wallpapers
└── vulcan-forge/        # VulcanOS Original theme wallpapers
```

## Theme-Wallpaper Binding

Themes can suggest wallpapers via the `THEME_WALLPAPER` export in their theme files:

```bash
# In dotfiles/themes/colors/catppuccin-mocha.sh
export THEME_WALLPAPER="wallpapers/catppuccin-mocha/catppuccin-mocha.png"
```

The path is relative to `dotfiles/` and must:
- Be relative (not absolute paths)
- Not contain `..` path traversal
- Point to an existing file

When users apply a theme, the Vulcan Appearance Manager offers to apply the suggested wallpaper, or they can choose a custom wallpaper (CustomOverride binding mode).

## Licensing Requirements

All wallpapers in this library must be:
- **CC0** (Creative Commons Zero - Public Domain)
- **GPL-compatible** (GPL, MIT, Apache, BSD)
- **Properly attributed** via LICENSE file in each theme directory

Each theme directory contains a `LICENSE` file documenting:
- Source URL and author
- License type
- Download/creation date
- Any modification notes

## Adding Custom Wallpapers

To add your own wallpapers:

1. **Place image in theme directory:**
   ```bash
   cp my-wallpaper.png dotfiles/wallpapers/catppuccin-mocha/
   ```

2. **Update theme file** (optional - to make it the default):
   ```bash
   # Edit dotfiles/themes/colors/catppuccin-mocha.sh
   export THEME_WALLPAPER="wallpapers/catppuccin-mocha/my-wallpaper.png"
   ```

3. **Document source** in LICENSE file if from external source

## Image Format Recommendations

- **Resolution:** Minimum 1920x1080, preferably 4K (3840x2160) for HiDPI displays
- **Format:** PNG or JPEG (PNG preferred for quality)
- **Aspect Ratio:** 16:9 or 16:10 for desktop displays
- **File Size:** Keep under 5MB for reasonable repo size

## Community Sources

See individual theme LICENSE files for verified sources of high-quality wallpapers.

Popular collections:
- [Catppuccin Wallpapers](https://github.com/zhichaoh/catppuccin-wallpapers)
- [Nordic Wallpapers](https://github.com/linuxdotexe/nordic-wallpapers)
- [Nord Backgrounds](https://github.com/dxnst/nord-backgrounds)
- [Dracula Wallpapers](https://draculatheme.com/wallpaper)
- r/unixporn community collections

Always verify licenses before using wallpapers from external sources.
