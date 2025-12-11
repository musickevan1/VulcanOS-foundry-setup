#!/bin/bash

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# History settings
HISTSIZE=10000
HISTFILESIZE=20000
HISTCONTROL=ignoreboth:erasedups
shopt -s histappend

# Shell options
shopt -s checkwinsize
shopt -s cdspell
shopt -s dirspell
shopt -s autocd

# PATH additions
export PATH="$HOME/.local/bin:$PATH"
export PATH="$HOME/.cargo/bin:$PATH"
export PATH="/usr/local/go/bin:$PATH"
export PATH="$HOME/go/bin:$PATH"

# Default applications
export EDITOR="nvim"
export VISUAL="nvim"
export TERMINAL="alacritty"
export BROWSER="firefox"

# Modern CLI tool aliases
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

# Useful aliases
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

# Neovim
alias vim='nvim'
alias vi='nvim'

# fzf integration
if command -v fzf &> /dev/null; then
    # Source fzf keybindings and completion
    [[ -f /usr/share/fzf/key-bindings.bash ]] && source /usr/share/fzf/key-bindings.bash
    [[ -f /usr/share/fzf/completion.bash ]] && source /usr/share/fzf/completion.bash

    # fzf default options
    export FZF_DEFAULT_OPTS="
        --height 40%
        --layout=reverse
        --border
        --inline-info
        --color=bg+:#33467c,bg:#1a1b26,spinner:#7dcfff,hl:#f7768e
        --color=fg:#c0caf5,header:#f7768e,info:#e0af68,pointer:#7dcfff
        --color=marker:#9ece6a,fg+:#c0caf5,prompt:#7aa2f7,hl+:#f7768e
    "

    # Use fd for fzf if available
    if command -v fd &> /dev/null; then
        export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
        export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
        export FZF_ALT_C_COMMAND='fd --type d --hidden --follow --exclude .git'
    fi
fi

# zoxide initialization (smarter cd)
if command -v zoxide &> /dev/null; then
    eval "$(zoxide init bash)"
fi

# Starship prompt initialization
if command -v starship &> /dev/null; then
    eval "$(starship init bash)"
else
    # Fallback prompt
    PS1='[\u@\h \W]\$ '
fi

# Load local customizations if they exist
[[ -f ~/.bashrc.local ]] && source ~/.bashrc.local
