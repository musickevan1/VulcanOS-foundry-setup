#!/bin/bash
# =============================================================================
# VulcanOS Bash Configuration
# =============================================================================

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# =============================================================================
# BLE.SH INITIALIZATION (must be sourced early for syntax highlighting)
# Install with: yay -S blesh
# =============================================================================
if [[ -f /usr/share/blesh/ble.sh ]]; then
    source /usr/share/blesh/ble.sh --noattach
fi

# =============================================================================
# HISTORY
# =============================================================================
HISTSIZE=10000
HISTFILESIZE=20000
HISTCONTROL=ignoreboth:erasedups
shopt -s histappend

# =============================================================================
# SHELL OPTIONS
# =============================================================================
shopt -s checkwinsize
shopt -s cdspell
shopt -s dirspell
shopt -s autocd
shopt -s globstar 2>/dev/null

# =============================================================================
# PATH ADDITIONS
# =============================================================================
export PATH="$HOME/.local/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="/usr/local/go/bin:$PATH"
export PATH="$HOME/go/bin:$PATH"

# =============================================================================
# DEFAULT APPLICATIONS
# =============================================================================
export EDITOR="nvim"
export VISUAL="nvim"
export TERMINAL="kitty"
export BROWSER="chromium"

# =============================================================================
# VULCANOS TERMINAL GREETING
# =============================================================================
vulcan_greeting() {
    # Only show once per session (not in subshells)
    if [[ -z "$VULCAN_GREETED" ]]; then
        export VULCAN_GREETED=1

        # Source theme colors (fallback to Vulcan Forge defaults)
        local COLORS_FILE="${HOME}/.config/vulcan/greeting-colors.sh"
        if [[ -f "${COLORS_FILE}" ]]; then
            source "${COLORS_FILE}"
            local O="${VULCAN_GREETING_COLOR1}"
            local G="${VULCAN_GREETING_COLOR2}"
            local R="${VULCAN_GREETING_RESET}"
        else
            # Fallback: Vulcan Forge theme colors
            local O='\033[38;2;249;115;22m'  # Forge Orange
            local G='\033[38;2;251;191;36m'  # Ember Gold
            local R='\033[0m'                 # Reset
        fi

        echo ""
        echo -e "${O} ██╗   ██╗██╗   ██╗██╗      ██████╗ █████╗ ███╗   ██╗${G} ██████╗ ███████╗${R}"
        echo -e "${O} ██║   ██║██║   ██║██║     ██╔════╝██╔══██╗████╗  ██║${G}██╔═══██╗██╔════╝${R}"
        echo -e "${O} ██║   ██║██║   ██║██║     ██║     ███████║██╔██╗ ██║${G}██║   ██║███████╗${R}"
        echo -e "${O} ╚██╗ ██╔╝██║   ██║██║     ██║     ██╔══██║██║╚██╗██║${G}██║   ██║╚════██║${R}"
        echo -e "${O}  ╚████╔╝ ╚██████╔╝███████╗╚██████╗██║  ██║██║ ╚████║${G}╚██████╔╝███████║${R}"
        echo -e "${O}   ╚═══╝   ╚═════╝ ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝${G} ╚═════╝ ╚══════╝${R}"
        echo ""
    fi
}

# Show greeting (comment out to disable)
vulcan_greeting

# =============================================================================
# MODERN CLI ALIASES
# =============================================================================
if command -v eza &> /dev/null; then
    alias ls='eza --icons --group-directories-first'
    alias ll='eza -l --icons --group-directories-first'
    alias la='eza -la --icons --group-directories-first'
    alias lt='eza --tree --icons --group-directories-first'
    alias l='eza -lah --icons --group-directories-first'
else
    alias ls='ls --color=auto'
    alias ll='ls -lh'
    alias la='ls -lah'
fi

if command -v bat &> /dev/null; then
    alias cat='bat --paging=never'
    alias less='bat --paging=always'
    export MANPAGER="sh -c 'col -bx | bat -l man -p'"
fi

if command -v rg &> /dev/null; then
    alias grep='rg'
fi

# =============================================================================
# UTILITY ALIASES
# =============================================================================
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias mkdir='mkdir -pv'
alias cp='cp -iv'
alias mv='mv -iv'
alias rm='rm -Iv'
alias df='df -h'
alias du='du -h'
alias free='free -h'
alias ip='ip -color=auto'
alias diff='diff --color=auto'

# Git aliases
alias g='git'
alias gs='git status'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git pull'
alias gd='git diff'
alias glog='git log --oneline --graph --decorate'

# Editor aliases
alias vim='nvim'
alias vi='nvim'

# =============================================================================
# FZF INTEGRATION
# =============================================================================
if command -v fzf &> /dev/null; then
    # Source fzf keybindings and completion (suppress ble.sh compatibility warnings)
    [[ -f /usr/share/fzf/key-bindings.bash ]] && source /usr/share/fzf/key-bindings.bash 2>/dev/null
    [[ -f /usr/share/fzf/completion.bash ]] && source /usr/share/fzf/completion.bash 2>/dev/null

    # FZF default options - VulcanOS Forge colors
    export FZF_DEFAULT_OPTS="
        --height 40%
        --layout=reverse
        --border
        --inline-info
        --color=bg+:#292524,bg:#1c1917,spinner:#f97316,hl:#ef4444
        --color=fg:#fafaf9,header:#ef4444,info:#fbbf24,pointer:#f97316
        --color=marker:#22c55e,fg+:#fafaf9,prompt:#f97316,hl+:#ef4444
    "

    # Use fd for fzf if available
    if command -v fd &> /dev/null; then
        export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
        export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
        export FZF_ALT_C_COMMAND='fd --type d --hidden --follow --exclude .git'
    fi
fi

# =============================================================================
# ZOXIDE (Smart cd)
# =============================================================================
if command -v zoxide &> /dev/null; then
    eval "$(zoxide init bash)"
fi

# =============================================================================
# STARSHIP PROMPT
# =============================================================================
if command -v starship &> /dev/null; then
    eval "$(starship init bash)"
else
    # Fallback prompt (traditional style)
    PS1='[\u@\h \W]\$ '
fi

# =============================================================================
# BLE.SH MINIMAL THEME - Monochrome + Orange + Gold
# VulcanOS colors: 208=orange, 220=gold, 255=white, 249=gray, 240=dim
# =============================================================================
if [[ ${BLE_VERSION-} ]]; then
    # Theme function - called after ble.sh loads syntax module
    function vulcanos-theme {
        # Commands - Orange for recognized
        ble-color-setface command_alias        fg=208
        ble-color-setface command_builtin      fg=208
        ble-color-setface command_builtin_dot  fg=208
        ble-color-setface command_directory    fg=249
        ble-color-setface command_file         fg=255
        ble-color-setface command_function     fg=208
        ble-color-setface command_jobs         fg=208
        ble-color-setface command_keyword      fg=208

        # Syntax - Orange commands, Gold strings/vars
        ble-color-setface syntax_brace         fg=220
        ble-color-setface syntax_command       fg=208
        ble-color-setface syntax_comment       fg=240
        ble-color-setface syntax_default       fg=255
        ble-color-setface syntax_delimiter     fg=249
        ble-color-setface syntax_document      fg=220
        ble-color-setface syntax_document_begin fg=208
        ble-color-setface syntax_error         fg=208,bold
        ble-color-setface syntax_expr          fg=220
        ble-color-setface syntax_function_name fg=208
        ble-color-setface syntax_glob          fg=220
        ble-color-setface syntax_history_expansion fg=208
        ble-color-setface syntax_param_expansion fg=220
        ble-color-setface syntax_quotation     fg=220
        ble-color-setface syntax_quoted        fg=220
        ble-color-setface syntax_tilde         fg=220
        ble-color-setface syntax_varname       fg=220

        # Filenames - Subtle grays
        ble-color-setface filename_block       fg=249
        ble-color-setface filename_character   fg=249
        ble-color-setface filename_directory   fg=249
        ble-color-setface filename_directory_sticky fg=249
        ble-color-setface filename_executable  fg=255
        ble-color-setface filename_link        fg=249,underline
        ble-color-setface filename_ls_colors   fg=249
        ble-color-setface filename_orphan      fg=240
        ble-color-setface filename_other       fg=249
        ble-color-setface filename_pipe        fg=249
        ble-color-setface filename_setgid      fg=249
        ble-color-setface filename_setuid      fg=249
        ble-color-setface filename_socket      fg=249
        ble-color-setface filename_warning     fg=208

        # UI
        ble-color-setface auto_complete        fg=240
        ble-color-setface menu_filter_input    fg=208
        ble-color-setface menu_filter_fixed    fg=249
        ble-color-setface region_target        fg=black,bg=208
    }

    # Autocomplete settings
    bleopt complete_auto_delay=50

    # Attach ble.sh first
    ble-attach

    # Apply theme after attach (when all modules are loaded)
    vulcanos-theme 2>/dev/null
fi

# =============================================================================
# OPENCODE PERMISSION MODES
# =============================================================================
opencode() {
    local MODE="default"
    local ARGS=()

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --yolo)
                MODE="yolo"
                shift
                ;;
            --safe)
                MODE="safe"
                shift
                ;;
            *)
                ARGS+=("$1")
                shift
                ;;
        esac
    done

    # Set config based on mode
    case "$MODE" in
        yolo)
            export OPENCODE_CONFIG="$HOME/.config/opencode/opencode-yolo.json"
            echo "🚀 OpenCode in YOLO mode"
            ;;
        safe)
            export OPENCODE_CONFIG="$HOME/.config/opencode/opencode-safe.json"
            echo "🛡️  OpenCode in SAFE mode"
            ;;
        default)
            # Use default config (no export)
            ;;
    esac

    # Run opencode with remaining arguments
    command opencode "${ARGS[@]}"
}

# =============================================================================
# YAZI FILE MANAGER (with directory change on exit)
# =============================================================================
# 'y' launches yazi and cd's to directory when you exit with 'q'
y() {
    local tmp="$(mktemp -t "yazi-cwd.XXXXXX")"
    yazi "$@" --cwd-file="$tmp"
    if cwd="$(cat -- "$tmp")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then
        cd -- "$cwd"
    fi
    rm -f -- "$tmp"
}

# Alias for full path launches (without directory change)
alias yazi='yazi'

# Quick drag-and-drop from current directory
alias yd='ripdrag .'

# =============================================================================
# LOCAL CUSTOMIZATIONS
# =============================================================================
[[ -f ~/.bashrc.local ]] && source ~/.bashrc.local

# pnpm
export PNPM_HOME="/home/evan/.local/share/pnpm"
case ":$PATH:" in
  *":$PNPM_HOME:"*) ;;
  *) export PATH="$PNPM_HOME:$PATH" ;;
esac
# pnpm end

# =============================================================================
# OPENCODE & MCP ENVIRONMENT VARIABLES
# Secrets are stored in ~/.bashrc.local (not tracked by git)
# =============================================================================
