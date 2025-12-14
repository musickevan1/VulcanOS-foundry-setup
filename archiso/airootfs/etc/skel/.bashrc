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
export BROWSER="firefox"

# =============================================================================
# VULCANOS TERMINAL GREETING
# =============================================================================
vulcan_greeting() {
    # Only show once per session (not in subshells)
    if [[ -z "$VULCAN_GREETED" ]]; then
        export VULCAN_GREETED=1

        # VulcanOS ASCII Logo (V-shape from inverted Arch logo)
        echo -e "\033[38;2;249;115;22m"  # Forge Orange
        echo "      \\\\    //"
        echo "       \\\\  //"
        echo "        \\\\//"
        echo -e "\033[0m"
        echo -e "\033[38;2;251;191;36mVulcanOS\033[0m - Forge your development environment"
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
    # Source fzf keybindings and completion
    [[ -f /usr/share/fzf/key-bindings.bash ]] && source /usr/share/fzf/key-bindings.bash
    [[ -f /usr/share/fzf/completion.bash ]] && source /usr/share/fzf/completion.bash

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
# BLE.SH CONFIGURATION (syntax highlighting colors)
# =============================================================================
if [[ ${BLE_VERSION-} ]]; then
    # VulcanOS Forge color scheme for syntax highlighting
    ble-face -s command_builtin       fg=#f97316           # Forge Orange - builtins
    ble-face -s command_alias         fg=#fbbf24           # Ember Gold - aliases
    ble-face -s command_function      fg=#22c55e           # Green - functions
    ble-face -s command_file          fg=#3b82f6           # Blue - executables
    ble-face -s command_keyword       fg=#a855f7           # Purple - keywords
    ble-face -s command_directory     fg=#06b6d4,underline # Cyan - directories
    ble-face -s filename_directory    fg=#06b6d4,bold      # Cyan bold
    ble-face -s filename_executable   fg=#22c55e,bold      # Green bold
    ble-face -s filename_link         fg=#a855f7           # Purple - symlinks
    ble-face -s filename_orphan       fg=#ef4444,bold      # Red - broken links
    ble-face -s syntax_default        fg=#fafaf9           # Primary text
    ble-face -s syntax_command        fg=#22c55e           # Green - commands
    ble-face -s syntax_quoted         fg=#fbbf24           # Ember Gold - strings
    ble-face -s syntax_quotation      fg=#fbbf24           # Ember Gold
    ble-face -s syntax_escape         fg=#f97316           # Forge Orange - escapes
    ble-face -s syntax_expr           fg=#06b6d4           # Cyan - expressions
    ble-face -s syntax_error          fg=#ef4444,bold      # Red - errors
    ble-face -s syntax_varname        fg=#3b82f6           # Blue - variables
    ble-face -s syntax_delimiter      fg=#a8a29e           # Secondary text
    ble-face -s syntax_param_expansion fg=#f97316          # Forge Orange
    ble-face -s syntax_history_expansion fg=#a855f7        # Purple
    ble-face -s syntax_glob           fg=#fbbf24           # Ember Gold - globs
    ble-face -s syntax_brace          fg=#06b6d4           # Cyan
    ble-face -s syntax_tilde          fg=#a855f7           # Purple
    ble-face -s syntax_document       fg=#fbbf24           # Ember Gold - heredocs
    ble-face -s syntax_document_begin fg=#fbbf24,bold
    ble-face -s varname_array         fg=#f97316           # Forge Orange
    ble-face -s varname_empty         fg=#ef4444           # Red
    ble-face -s varname_export        fg=#22c55e,bold      # Green bold - exports
    ble-face -s varname_global        fg=#fbbf24           # Ember Gold
    ble-face -s varname_local         fg=#3b82f6           # Blue
    ble-face -s varname_readonly      fg=#a855f7           # Purple
    ble-face -s varname_unset         fg=#78716c           # Muted

    # Auto-suggestion color (dimmed)
    ble-face -s auto_complete         fg=#57534e           # Ash gray

    # Menu/completion colors
    ble-face -s menu_filter_input     fg=#1c1917,bg=#f97316
    ble-face -s menu_filter_fixed     fg=#fafaf9,bold
    ble-face -s region_target         fg=#1c1917,bg=#fbbf24

    # Settings
    bleopt complete_auto_delay=300
    bleopt history_share=1
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
# BLE.SH ATTACH (must be at end of bashrc)
# =============================================================================
if [[ ${BLE_VERSION-} ]]; then
    ble-attach
fi

# =============================================================================
# LOCAL CUSTOMIZATIONS
# =============================================================================
[[ -f ~/.bashrc.local ]] && source ~/.bashrc.local
