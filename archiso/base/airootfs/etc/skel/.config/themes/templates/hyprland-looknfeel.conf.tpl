# Look and Feel Configuration
# Visual settings: gaps, borders, animations, decorations
# Theme: ${THEME_NAME}

# General appearance
general {
    gaps_in = 5
    gaps_out = 10
    border_size = 2

    col.active_border = rgba(${GRADIENT_START//#/}ff) rgba(${GRADIENT_END//#/}ff) 45deg
    col.inactive_border = rgba(${BORDER_INACTIVE//#/}aa)

    resize_on_border = true
    extend_border_grab_area = 15
    hover_icon_on_border = true
}

# Window decorations
decoration {
    rounding = 8

    active_opacity = 1.0
    inactive_opacity = 0.95
    fullscreen_opacity = 1.0

    dim_inactive = true
    dim_strength = 0.1
    dim_special = 0.5

    shadow {
        enabled = true
        range = 12
        render_power = 3
        ignore_window = true
        color = rgba(${BG_SURFACE//#/}ee)
        color_inactive = rgba(${BG_SURFACE//#/}99)
        offset = 2 2
        scale = 1.0
    }

    blur {
        enabled = true
        size = 8
        passes = 2
        ignore_opacity = true
        new_optimizations = true
        xray = false
        noise = 0.0117
        contrast = 0.8916
        brightness = 0.8172
        vibrancy = 0.1696
        vibrancy_darkness = 0.0
        special = false
        popups = true
        popups_ignorealpha = 0.2
    }
}

# Animations
animations {
    enabled = true
    first_launch_animation = true

    # Fast animation curves
    bezier = easeOutExpo, 0.16, 1, 0.3, 1
    bezier = easeInOutQuart, 0.76, 0, 0.24, 1
    bezier = linear, 0, 0, 1, 1
    bezier = snappy, 0.2, 1, 0.3, 1
    bezier = overshot, 0.05, 0.9, 0.1, 1.05

    # Window animations (fast)
    animation = windows, 1, 3, snappy, slide
    animation = windowsIn, 1, 3, snappy, slide
    animation = windowsOut, 1, 3, snappy, slide
    animation = windowsMove, 1, 3, snappy

    # Fade animations (fast)
    animation = fade, 1, 3, easeOutExpo
    animation = fadeIn, 1, 3, easeOutExpo
    animation = fadeOut, 1, 3, easeOutExpo
    animation = fadeDim, 1, 3, easeOutExpo
    animation = fadeShadow, 1, 3, easeOutExpo

    # Border animations
    animation = border, 1, 5, easeOutExpo
    animation = borderangle, 1, 30, linear, loop

    # Workspace animations (DISABLED for performance)
    animation = workspaces, 0
    animation = specialWorkspace, 1, 3, snappy, slidevert

    # Layer animations
    animation = layers, 1, 3, snappy, fade
    animation = layersIn, 1, 3, snappy, fade
    animation = layersOut, 1, 3, snappy, fade
}

# Group settings
group {
    insert_after_current = true
    focus_removed_window = true

    col.border_active = rgba(${ACCENT//#/}ff)
    col.border_inactive = rgba(${BORDER_INACTIVE//#/}aa)
    col.border_locked_active = rgba(${RED//#/}ff)
    col.border_locked_inactive = rgba(${BORDER_INACTIVE//#/}aa)

    groupbar {
        enabled = true
        font_family = JetBrainsMono Nerd Font
        font_size = 10
        gradients = true
        height = 20
        priority = 3
        render_titles = true
        scrolling = true
        text_color = rgba(${FG_PRIMARY//#/}ff)
        col.active = rgba(${ACCENT//#/}ff)
        col.inactive = rgba(${BORDER_INACTIVE//#/}aa)
        col.locked_active = rgba(${RED//#/}ff)
        col.locked_inactive = rgba(${BORDER_INACTIVE//#/}aa)
    }
}
