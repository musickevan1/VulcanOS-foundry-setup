# =============================================================================
# VulcanOS Custom Kitty Tab Bar
# Powerline-style tabs with zoom indicator on the far right
# =============================================================================
# pyright: reportMissingImports=false
# type: ignore

import os
from kitty.fast_data_types import Screen, get_options
from kitty.tab_bar import DrawData, ExtraData, TabBarData, as_rgb, draw_title
from kitty.utils import color_as_int

opts = get_options()

# Powerline separator character
SEP = "\ue0b0"  # 

# Zoom state file location (per-user)
ZOOM_STATE_FILE = os.path.expanduser("~/.cache/kitty-zoom-state")


def _get_zoom_indicator() -> str:
    """Read zoom indicator from state file."""
    try:
        if os.path.exists(ZOOM_STATE_FILE):
            with open(ZOOM_STATE_FILE, "r") as f:
                zoom = f.read().strip()
                if zoom:
                    return f"[{zoom}]"
    except Exception:
        pass
    return ""


def draw_tab(
    draw_data: DrawData,
    screen: Screen,
    tab: TabBarData,
    before: int,
    max_title_length: int,
    index: int,
    is_last: bool,
    extra_data: ExtraData,
) -> int:
    """
    Custom tab drawing function - powerline style with zoom indicator.
    """
    # Get colors
    if tab.is_active:
        bg = as_rgb(color_as_int(draw_data.active_bg))
        fg = as_rgb(color_as_int(draw_data.active_fg))
    else:
        bg = as_rgb(color_as_int(draw_data.inactive_bg))
        fg = as_rgb(color_as_int(draw_data.inactive_fg))

    tab_bar_bg = as_rgb(color_as_int(draw_data.tab_bar_background))

    # Set colors and draw tab content
    screen.cursor.bg = bg
    screen.cursor.fg = fg

    # Tab content: " {index}: {title} "
    title = tab.title
    if len(title) > max_title_length - 8:
        title = title[: max_title_length - 11] + "..."

    screen.draw(f" {index + 1}: {title} ")

    # Draw powerline separator
    screen.cursor.fg = bg
    screen.cursor.bg = tab_bar_bg
    screen.draw(SEP)

    end = screen.cursor.x

    # On last tab, draw zoom indicator on far right
    if is_last:
        zoom_text = _get_zoom_indicator()
        remaining = screen.columns - screen.cursor.x

        if remaining > 0:
            # Fill gap with background
            if zoom_text and remaining > len(zoom_text) + 2:
                gap = remaining - len(zoom_text) - 2
                screen.cursor.bg = tab_bar_bg
                screen.cursor.fg = tab_bar_bg
                screen.draw(" " * gap)

                # Draw zoom indicator
                screen.cursor.fg = as_rgb(color_as_int(draw_data.inactive_fg))
                screen.draw(f" {zoom_text} ")
            else:
                # Just fill remaining space
                screen.cursor.bg = tab_bar_bg
                screen.cursor.fg = tab_bar_bg
                screen.draw(" " * remaining)

    return end
