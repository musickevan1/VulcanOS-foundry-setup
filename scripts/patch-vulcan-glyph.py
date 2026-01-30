#!/usr/bin/env fontforge
# VulcanOS Font Patcher
# Creates an upside-down Arch Linux glyph for VulcanOS branding
#
# Usage: fontforge -script patch-vulcan-glyph.py
#
# This script patches all JetBrainsMono Nerd Font weights (Regular, Bold,
# Italic, BoldItalic) with the VulcanOS glyph (inverted Arch logo) at U+E900.

import fontforge
import os
import sys

# Configuration
ARCH_GLYPH = 0xF08C7  # 󰣇 Arch Linux logo in Nerd Fonts (nf-md-arch)
VULCAN_GLYPH = 0xE900  # Private Use Area - empty slot for VulcanOS
FONT_NAME_SUFFIX = "-Vulcan"

# Font weights to patch
FONT_WEIGHTS = [
    ("Regular", "Regular"),
    ("Bold", "Bold"),
    ("Italic", "Italic"),
    ("BoldItalic", "Bold Italic"),
]

def find_nerd_fonts():
    """Find all JetBrainsMono Nerd Font weights on the system"""
    search_paths = [
        "/usr/share/fonts/TTF",
        "/usr/share/fonts/truetype/jetbrains-mono",
        "/usr/share/fonts/OTF",
        "/usr/share/fonts/nerd-fonts",
        "/usr/share/fonts"
    ]

    fonts = {}
    for path in search_paths:
        if not os.path.exists(path):
            continue
        for weight, _ in FONT_WEIGHTS:
            patterns = [
                f"JetBrainsMonoNerdFont-{weight}.ttf",
                f"JetBrains Mono Nerd Font {weight}.ttf",
            ]
            for pattern in patterns:
                font_path = os.path.join(path, pattern)
                if os.path.exists(font_path) and weight not in fonts:
                    fonts[weight] = font_path
                    break

    return fonts

def patch_font(input_path, output_dir, weight, style_name):
    """Patch a single font file with VulcanOS glyph"""
    print(f"  Patching {weight}...")
    font = fontforge.open(input_path)

    # Check if Arch glyph exists
    if ARCH_GLYPH not in font:
        print(f"    Warning: Arch glyph not found in {weight}, skipping")
        font.close()
        return None

    # Select and copy the Arch glyph
    font.selection.select(ARCH_GLYPH)
    font.copy()

    # Create the VulcanOS glyph slot
    font.selection.select(VULCAN_GLYPH)
    font.paste()

    # Get the glyph and flip it vertically
    glyph = font[VULCAN_GLYPH]

    # Get bounding box for flip calculation
    bbox = glyph.boundingBox()
    if bbox:
        center_y = (bbox[1] + bbox[3]) / 2
        glyph.transform((1, 0, 0, -1, 0, 2 * center_y))

    # Update font metadata
    font.fontname = f"JetBrainsMonoNFVulcan-{weight}"
    font.familyname = "JetBrainsMono NF Vulcan"
    font.fullname = f"JetBrainsMono NF Vulcan {style_name}"

    # Update SFNT names table
    font.sfnt_names = tuple(
        (lang, strid, "JetBrainsMono NF Vulcan" if strid == 1 else
                      style_name if strid == 2 else
                      f"JetBrainsMono NF Vulcan {style_name}" if strid in (3, 4) else val)
        for lang, strid, val in font.sfnt_names
    )

    # Generate output filename
    input_basename = os.path.basename(input_path)
    name, ext = os.path.splitext(input_basename)
    output_filename = f"{name}{FONT_NAME_SUFFIX}{ext}"
    output_path = os.path.join(output_dir, output_filename)

    font.generate(output_path)
    font.close()

    print(f"    Saved: {output_filename}")
    return output_path

def main():
    print("VulcanOS Font Patcher")
    print("=" * 50)
    print("Patches JetBrainsMono Nerd Font with VulcanOS glyph")
    print(f"Glyph: Inverted Arch logo at U+{VULCAN_GLYPH:04X}")
    print("=" * 50)
    print()

    # Find all font weights
    fonts = find_nerd_fonts()
    if not fonts:
        print("Error: Could not find JetBrainsMono Nerd Font")
        print("Please install it: sudo pacman -S ttf-jetbrains-mono-nerd")
        sys.exit(1)

    print(f"Found {len(fonts)} font weight(s):")
    for weight, path in fonts.items():
        print(f"  {weight}: {os.path.basename(path)}")
    print()

    # Output directory
    output_dir = os.path.expanduser("~/.local/share/fonts/vulcan")
    os.makedirs(output_dir, exist_ok=True)

    # Patch all fonts
    print("Patching fonts...")
    patched = []
    for weight, style_name in FONT_WEIGHTS:
        if weight in fonts:
            result = patch_font(fonts[weight], output_dir, weight, style_name)
            if result:
                patched.append(result)

    print()
    print("=" * 50)
    print(f"Patched {len(patched)} font(s) successfully!")
    print()
    print("To activate:")
    print("  1. Update font cache: fc-cache -fv")
    print("  2. Restart your terminal")
    print()
    print(f"VulcanOS glyph: U+E900 (use \\uE900 or copy the character)")
    print(f"Fonts installed to: {output_dir}")

if __name__ == "__main__":
    main()
