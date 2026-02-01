---
type: quick
plan: 001
wave: 1
files_modified:
  - dotfiles/scripts/.local/bin/workspace-switch
  - dotfiles/scripts/.local/bin/vulcan-workspace-init
autonomous: true

must_haves:
  truths:
    - "Super+1-5 switches to workspace 1-5 on the focused monitor"
    - "Workspace order reads from EDID-derived cache, not hardcoded DP numbers"
    - "Works across all profiles: Desktop (5), Campus (2), Laptop (1)"
  artifacts:
    - path: "dotfiles/scripts/.local/bin/workspace-switch"
      provides: "Relative workspace switching using cache"
    - path: "dotfiles/scripts/.local/bin/vulcan-workspace-init"
      provides: "Workspace init using EDID cache for DP resolution"
  key_links:
    - from: "workspace-switch"
      to: "~/.cache/vulcan-monitors/sceptre-mapping"
      via: "source file for DP names"
    - from: "vulcan-workspace-init"
      to: "~/.cache/vulcan-monitors/sceptre-mapping"
      via: "source file for DP names"
---

<objective>
Fix workspace keybindings to use EDID-based monitor detection instead of hardcoded DP numbers.

Purpose: Super+1-5 currently fails because workspace-switch and vulcan-workspace-init have hardcoded DP numbers (DP-15, DP-11, DP-13, DP-4) that don't match current system (DP-15, DP-11, DP-13, DP-6). The EDID fingerprint system already works and produces correct mappings in ~/.cache/vulcan-monitors/sceptre-mapping.

Output: Both scripts dynamically read from EDID cache instead of using hardcoded values.
</objective>

<execution_context>
@/home/evan/.claude/get-shit-done/workflows/execute-plan.md
</execution_context>

<context>
@dotfiles/scripts/.local/bin/workspace-switch
@dotfiles/scripts/.local/bin/vulcan-workspace-init
@dotfiles/scripts/.local/bin/vulcan-monitor-identify

Cache file format (~/.cache/vulcan-monitors/sceptre-mapping):
```
LEFT_VERTICAL=DP-11
CENTER_TOP=DP-13
CENTER_BOTTOM=DP-15
FLOAT2_PRO=DP-6
```

User's workspace order requirement:
- Desktop (5 monitors): eDP-1 -> CENTER_BOTTOM -> FLOAT2_PRO -> CENTER_TOP -> LEFT_VERTICAL
- Campus (2 monitors): eDP-1 -> external
- Laptop (1 monitor): eDP-1
</context>

<tasks>

<task type="auto">
  <name>Task 1: Update workspace-switch to read from EDID cache</name>
  <files>dotfiles/scripts/.local/bin/workspace-switch</files>
  <action>
Replace the hardcoded MONITOR_INDEX associative array with dynamic reading from cache:

1. Add cache file path constant:
   ```bash
   CACHE_DIR="$HOME/.cache/vulcan-monitors"
   MAPPING_FILE="$CACHE_DIR/sceptre-mapping"
   ```

2. Create function to build monitor order dynamically:
   - Read MAPPING_FILE if exists (source it to get LEFT_VERTICAL, CENTER_TOP, etc.)
   - Build ordered array based on user requirement:
     - eDP-1 (workspaces 1-5, primary/laptop)
     - CENTER_BOTTOM (workspaces 6-10, main curved)
     - FLOAT2_PRO (workspaces 11-15)
     - CENTER_TOP (workspaces 16-20)
     - LEFT_VERTICAL (workspaces 21-25)
   - Only include monitors that are currently connected (check via hyprctl monitors -j)

3. Replace hardcoded MONITOR_INDEX with dynamically-built version:
   - Loop through ordered array, assign index 0, 1, 2...
   - Populate MONITOR_INDEX[$dp_name]=index for each connected monitor

4. Keep fallback: if cache doesn't exist, use eDP-1 only (laptop mode)
  </action>
  <verify>
Run: `workspace-switch 1` while focused on different monitors - should switch to correct absolute workspace.
Test: Focus eDP-1, run `workspace-switch 1` -> should go to workspace 1.
Test: Focus CENTER_BOTTOM monitor, run `workspace-switch 1` -> should go to workspace 6.
  </verify>
  <done>workspace-switch uses EDID cache for DP resolution, Super+1-5 works on all monitors</done>
</task>

<task type="auto">
  <name>Task 2: Update vulcan-workspace-init to use EDID cache</name>
  <files>dotfiles/scripts/.local/bin/vulcan-workspace-init</files>
  <action>
Replace hardcoded PROFILE_WORKSPACE_ORDER with dynamic resolution from cache:

1. Add cache file constants (same as workspace-switch):
   ```bash
   MONITOR_CACHE_DIR="$HOME/.cache/vulcan-monitors"
   MAPPING_FILE="$MONITOR_CACHE_DIR/sceptre-mapping"
   ```

2. Create function `resolve_monitor_order()`:
   - If MAPPING_FILE exists, source it to get named positions
   - Build profile-specific order using logical names:
     - desktop: "eDP-1 $CENTER_BOTTOM $FLOAT2_PRO $CENTER_TOP $LEFT_VERTICAL"
     - console: "eDP-1 $CENTER_BOTTOM $FLOAT2_PRO $CENTER_TOP" (4 monitors)
     - campus: "$FLOAT2_PRO eDP-1" (portable external)
     - laptop: "eDP-1"
   - Filter to only connected monitors (existing logic)

3. Update get_ordered_monitors() to use resolve_monitor_order():
   - Call resolve_monitor_order() to get the order string
   - Replace the static PROFILE_WORKSPACE_ORDER lookup

4. Keep existing fallback behavior if cache doesn't exist
  </action>
  <verify>
Run: `vulcan-workspace-init` and check output shows correct DP names from cache.
Run: `cat ~/.config/hypr/workspaces.conf` and verify monitor names match current system.
Run: `cat ~/.config/waybar/workspaces.json` and verify persistent-workspaces has correct DP names.
  </verify>
  <done>vulcan-workspace-init generates correct configs using EDID-resolved DP names</done>
</task>

<task type="auto">
  <name>Task 3: Sync updated scripts to archiso skeleton</name>
  <files>
archiso/airootfs/etc/skel/.local/bin/workspace-switch
archiso/airootfs/etc/skel/.local/bin/vulcan-workspace-init
  </files>
  <action>
Copy updated scripts from dotfiles to archiso skeleton:

1. Ensure target directory exists:
   `mkdir -p archiso/airootfs/etc/skel/.local/bin`

2. Copy both scripts:
   `cp dotfiles/scripts/.local/bin/workspace-switch archiso/airootfs/etc/skel/.local/bin/`
   `cp dotfiles/scripts/.local/bin/vulcan-workspace-init archiso/airootfs/etc/skel/.local/bin/`

3. Ensure scripts are executable:
   `chmod +x archiso/airootfs/etc/skel/.local/bin/workspace-switch`
   `chmod +x archiso/airootfs/etc/skel/.local/bin/vulcan-workspace-init`
  </action>
  <verify>
Run: `diff dotfiles/scripts/.local/bin/workspace-switch archiso/airootfs/etc/skel/.local/bin/workspace-switch` (should show no diff)
Run: `diff dotfiles/scripts/.local/bin/vulcan-workspace-init archiso/airootfs/etc/skel/.local/bin/vulcan-workspace-init` (should show no diff)
  </verify>
  <done>archiso skeleton has updated scripts matching dotfiles</done>
</task>

</tasks>

<verification>
1. Super+1 on eDP-1 -> workspace 1
2. Super+1 on CENTER_BOTTOM (curved) -> workspace 6
3. Super+3 on FLOAT2_PRO -> workspace 13
4. `vulcan-workspace-init` regenerates correct workspaces.conf with current DP names
5. Waybar shows correct workspace numbers per monitor
</verification>

<success_criteria>
- workspace-switch reads from ~/.cache/vulcan-monitors/sceptre-mapping
- vulcan-workspace-init reads from same cache file
- Both scripts handle missing cache gracefully (fallback to eDP-1 only)
- Super+1-5 works correctly on all 5 monitors
- Scripts synced to archiso skeleton
</success_criteria>

<output>
After completion, create `.planning/quick/001-fix-workspace-keybinds-dynamic-monitors/001-SUMMARY.md`
</output>
