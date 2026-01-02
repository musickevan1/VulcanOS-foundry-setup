# Orchestrator Launch Prompt

**Copy and paste this entire prompt into OpenCode to start the orchestrator session.**

---

```
@orchestrator Execute VulcanOS Configuration Alignment Plan

I need you to fix critical broken features in my VulcanOS system by executing a comprehensive 5-phase configuration alignment plan.

PLAN LOCATION: /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md

CONTEXT:
- Multiple vulcan-* scripts are missing from PATH (vulcan-menu, vulcan-power, vulcan-hotkeys, etc.)
- Keybindings are broken: Super+Escape, Super+Alt+Space, Super+K, and others
- Configuration files are not properly synced between dotfiles repo and live system
- Need to establish single source of truth architecture using GNU Stow

CRITICAL REQUIREMENTS - BACKUP PROTOCOL:

1. BEFORE ANY PHASE:
   - Use auto-backup-wrapper.sh to create automatic backup
   - Verify backup created successfully
   - Store backup location in /tmp/vulcan-last-backup.txt
   
2. BEFORE PHASE 3 (Live System Changes):
   - STOP and confirm with me first
   - Create triple-layer backup:
     a) Manual full backup via backup-vulcan-config.sh
     b) Git checkpoint and push
     c) Automatic wrapper backup
   - Wait for my confirmation before proceeding

3. AFTER EACH PHASE:
   - Report completion status
   - Show backup location
   - Verify affected functionality works
   - Wait for my "proceed" before next phase

4. ON ANY FAILURE:
   - STOP immediately
   - Report the error clearly
   - Display rollback command: $(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
   - DO NOT proceed to next phase

PHASE EXECUTION PLAN:

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 1: Emergency Script Rescue (30 minutes)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Objective: Restore broken keybindings by deploying missing scripts

Tasks:
1. Copy missing scripts from archiso/airootfs/usr/local/bin/ to dotfiles/scripts/:
   - vulcan-menu
   - vulcan-power
   - vulcan-hotkeys
   - vulcan-wallpaper
   - vulcan-screensaver
   
2. Make all scripts executable: chmod +x dotfiles/scripts/*

3. Execute with backup wrapper:
   /home/evan/VulcanOS/scripts/auto-backup-wrapper.sh "phase1-deploy-scripts" bash -c '
     cd /home/evan/VulcanOS/dotfiles
     for script in scripts/*; do
       ln -sf "$(pwd)/$script" ~/.local/bin/$(basename "$script")
     done
   '

4. Verify:
   - which vulcan-menu (should show path)
   - which vulcan-power (should show path)
   - which vulcan-hotkeys (should show path)
   - Test keybindings: Super+Escape, Super+Alt+Space, Super+K

Expected Result: All keybindings working immediately

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 2: Stow Structure Standardization (30 minutes)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Objective: Fix directory structure for proper GNU Stow compatibility

Tasks:
1. Fix kitty structure:
   mkdir -p dotfiles/kitty/.config/kitty/
   mv dotfiles/kitty/kitty.conf dotfiles/kitty/.config/kitty/kitty.conf

2. Fix starship (remove duplicate):
   rm dotfiles/starship/starship.toml
   (Keep only dotfiles/starship/.config/starship.toml)

3. Create proper scripts structure:
   mkdir -p dotfiles/scripts/.local/bin/
   mv dotfiles/scripts/* dotfiles/scripts/.local/bin/ 2>/dev/null || true

4. Test stow dry-run:
   cd dotfiles && stow -n -v -t ~ hypr waybar wofi kitty scripts

Expected Result: Stow dry-run succeeds with no conflicts

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 3: Live System Re-alignment (45 minutes) ⚠️ CRITICAL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ STOP BEFORE THIS PHASE - WAIT FOR MY CONFIRMATION ⚠️

This phase makes destructive changes to live system configs.

PRE-PHASE 3 CHECKLIST (I will do this manually):
□ Create manual full backup
□ Git commit and push
□ Confirm ready to proceed

Tasks (after my confirmation):
1. Backup current configs (automatic via wrapper)

2. Execute with backup wrapper:
   /home/evan/VulcanOS/scripts/auto-backup-wrapper.sh "phase3-stow-conversion" bash -c '
     # Remove existing configs
     rm -rf ~/.config/waybar ~/.config/wofi ~/.config/swaync
     rm -rf ~/.config/kitty ~/.config/alacritty
     
     # Stow all packages
     cd /home/evan/VulcanOS/dotfiles
     stow -v -t ~ hypr waybar wofi swaync kitty alacritty nvim opencode starship scripts git bash
   '

3. Verify symlinks:
   ls -la ~/.config/waybar (should show → /home/evan/VulcanOS/dotfiles/...)
   ls -la ~/.config/wofi (should show →)
   ls -la ~/.local/bin/vulcan-menu (should show →)

4. Reload services:
   hyprctl reload
   pkill waybar && sleep 0.5 && hyprctl dispatch exec waybar
   swaync-client -R && swaync-client -rs

5. Test functionality:
   - Waybar visible and working
   - Wofi launcher works (Super+Space)
   - All keybindings still work
   - swaync notifications work

Expected Result: All configs are symlinks, live system auto-syncs with dotfiles

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 4: ISO Synchronization (30 minutes)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Objective: Sync dotfiles to archiso for future ISO builds

Tasks:
1. Sync scripts to archiso:
   rsync -av --delete \
     /home/evan/VulcanOS/dotfiles/scripts/.local/bin/ \
     /home/evan/VulcanOS/archiso/airootfs/usr/local/bin/
   chmod +x /home/evan/VulcanOS/archiso/airootfs/usr/local/bin/*

2. Sync configs to /etc/skel:
   For each: hypr waybar wofi swaync kitty alacritty nvim opencode
   rsync -av --delete \
     dotfiles/<app>/.config/ \
     archiso/airootfs/etc/skel/.config/

3. Sync special files:
   - starship config
   - themes directory
   - .gitconfig
   - .bashrc

4. Create sync automation script at scripts/sync-dotfiles-to-iso.sh

5. Verify:
   diff -r dotfiles/waybar/.config/waybar/ archiso/airootfs/etc/skel/.config/waybar/
   ls archiso/airootfs/usr/local/bin/vulcan-*

Expected Result: archiso matches dotfiles, automation script created

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
PHASE 5: Documentation & Verification (45 minutes)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Objective: Create verification tools and update documentation

Tasks:
1. Verify all scripts exist and work:
   /home/evan/VulcanOS/scripts/verify-vulcan.sh

2. Verify backups integrity:
   /home/evan/VulcanOS/scripts/verify-backups.sh

3. Update CLAUDE.md with new configuration flow

4. Test configuration changes:
   - Edit dotfiles/waybar/.config/waybar/config.jsonc
   - Verify change appears in live system immediately
   - Revert test change

5. Create session report at .orchestrator/sessions/completion-report.md

Expected Result: All verification passes, documentation complete

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
FINAL VERIFICATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

After all phases complete, verify:
1. All vulcan-* scripts in PATH and executable
2. All keybindings work (test each broken one)
3. Configs are symlinks (ls -la ~/.config/waybar shows →)
4. Services running (waybar, swaync, hypridle, hyprpaper)
5. Configuration sync works (edit dotfiles → live update)
6. archiso synced (diff shows no differences)

COMMUNICATION STYLE:

- Report progress clearly after each step
- Show actual command output (don't summarize)
- Highlight any warnings or errors immediately
- Ask for confirmation before Phase 3
- Provide rollback commands if anything fails

ROLLBACK PROCEDURES:

If any phase fails:
1. Show this command: $(cat /tmp/vulcan-last-backup.txt)/quick-restore.sh
2. Explain what went wrong
3. Ask if I want to rollback or investigate
4. DO NOT proceed to next phase

ESTIMATED TIMELINE:

Phase 1: 30 minutes
Phase 2: 30 minutes
Phase 3: 45 minutes (includes pre-phase backup + confirmation wait)
Phase 4: 30 minutes
Phase 5: 45 minutes
Total: ~3 hours

REFERENCE DOCUMENTS:

- Full plan: /home/evan/VulcanOS/.orchestrator/vulcanos-config-alignment.md
- Pre-flight checklist: /home/evan/VulcanOS/.orchestrator/pre-flight-checklist.md
- Backup guide: /home/evan/VulcanOS/docs/BACKUP-RESTORE.md
- Project context: /home/evan/VulcanOS/CLAUDE.md

SUCCESS CRITERIA:

□ All vulcan-* scripts working
□ All keybindings functional
□ Live configs are stow symlinks
□ Edit dotfiles → instant live update
□ archiso synced with dotfiles
□ All verification scripts pass
□ At least 5 backups created during process
□ Git committed and pushed

Let's begin with Phase 1. Create the automatic backup first, then proceed with emergency script deployment.
```

---

**To launch the orchestrator:**

1. Copy everything above (from `@orchestrator` to the end)
2. Open OpenCode: `opencode`
3. Paste the entire prompt
4. Press Enter

The orchestrator will start Phase 1 immediately and ask for confirmation before Phase 3.
