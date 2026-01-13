---
name: iso-sync-reminder
enabled: true
event: file
pattern: dotfiles/[^/]+/\.config/
action: warn
---

## Reminder: Sync to ISO if Needed

You've modified a dotfile in the stow directory.

### What This Means

- **Live changes**: Already applied via stow symlinks to `~/.config/`
- **ISO inclusion**: NOT automatically included - requires manual sync

### To Include in ISO

Copy the changes to the archiso skeleton:

```bash
# Example for hypr config
cp -r dotfiles/hypr/.config/hypr/* archiso/airootfs/etc/skel/.config/hypr/

# Example for waybar config
cp -r dotfiles/waybar/.config/waybar/* archiso/airootfs/etc/skel/.config/waybar/
```

### Quick Sync Script

Consider using: `./scripts/sync-dotfiles.sh` (if available) to automate this.
