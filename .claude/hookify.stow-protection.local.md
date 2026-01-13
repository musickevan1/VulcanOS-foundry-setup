---
name: protect-stow-directories
enabled: true
event: bash
pattern: rm\s+(-rf?|--recursive)\s+.*dotfiles/[^/]+/\.config
action: block
---

## BLOCKED: Stow Source Directory Deletion

You are attempting to delete a GNU Stow source directory. This would:

1. **Break your live desktop** - these directories are symlinked to `~/.config/`
2. **Lose all configuration** - the actual config files live here

### VulcanOS Dotfiles Structure

```
dotfiles/
├── hypr/.config/hypr/     ← ACTUAL FILES (stow source)
├── waybar/.config/waybar/ ← ACTUAL FILES (stow source)
└── ...

~/.config/
├── hypr → dotfiles/hypr/.config/hypr   ← SYMLINK
├── waybar → dotfiles/waybar/.config/waybar ← SYMLINK
```

### Safe Alternatives

- **Remove a single file**: `rm dotfiles/hypr/.config/hypr/specific-file.conf`
- **Unstow a package**: `cd dotfiles && stow -D hypr`
- **Modify configs**: Edit files directly (changes apply immediately)

### See CLAUDE.md for full dotfiles documentation.
