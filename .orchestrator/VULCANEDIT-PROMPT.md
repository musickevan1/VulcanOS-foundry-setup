# VulcanEdit Phase 1 Implementation

**Orchestrator Session: VulcanEdit - Custom Neovim Distribution for VulcanOS**

---

## Context

VulcanOS is a custom Arch Linux distribution for developers, featuring:
- Hyprland compositor with keyboard-first workflow
- Gruvbox + Vulcan orange (#f97316) color scheme
- Custom `vulcan-*` scripts for system integration
- T2 MacBook Pro hardware support

**Current State:**
- Neovim config exists at `dotfiles/nvim/.config/nvim/`
- Uses tokyonight theme (needs replacement)
- Basic LSP setup functional but needs refinement
- No VulcanOS-specific branding or integration

**Goal:**
Create "VulcanEdit" - a branded, keyboard-first nvim distribution with gruvbox-forge theming and VulcanOS workflow integration.

---

## Detailed Plan Reference

**Full implementation plan:** `.orchestrator/VULCANEDIT-PLAN.md`

This prompt covers **Phase 1 only** - the foundation. Phase 2 (branding/polish) will be a separate session.

---

## Phase 1 Objectives

Implement a functional VulcanEdit with:

1. **Gruvbox-Forge Theme**
   - Base: Gruvbox Dark (#282828 bg, #ebdbb2 fg)
   - Accents: Vulcan orange (#f97316) replaces gruvbox blue
   - Keywords in orange, strings in green, comments in gray
   - Orange borders for active windows

2. **LSP & Markdown Workflow**
   - Verify all LSP servers work (TypeScript, Rust, Go, Python, Lua)
   - Add markdown checkbox toggling (`<leader>mt`)
   - Quick-open TODO.md with `<leader>tt`

3. **VulcanOS Integration**
   - Custom dashboard with VulcanOS ASCII art + volcano emoji 🌋
   - Custom commands: `:VulcanTasks`, `:VulcanConfig`, `:VulcanScripts`
   - Project detection (is_vulcanos_repo() helper)
   - VulcanOS-specific keybindings

---

## Implementation Tasks

### Task 1: Create Gruvbox-Forge Colorscheme

**Create colorscheme file:**
- Location: `dotfiles/nvim/.config/nvim/colors/gruvbox-forge.lua`
- Base on gruvbox.nvim structure (research existing gruvbox themes)
- Color palette (from VULCANEDIT-PLAN.md):
  ```lua
  -- Backgrounds (gruvbox)
  bg0 = "#282828"
  bg1 = "#3c3836"
  bg2 = "#504945"
  
  -- Foregrounds (gruvbox)
  fg0 = "#ebdbb2"
  fg1 = "#d5c4a1"
  gray = "#928374"
  
  -- Accents (VULCAN ORANGE PRIMARY)
  orange = "#f97316"  -- Replaces gruvbox blue
  yellow = "#fabd2f"  -- Gruvbox bright yellow
  green = "#b8bb26"   -- Gruvbox bright green
  pink = "#d3869b"    -- Gruvbox pink
  
  -- Syntax highlighting
  Keyword = orange    -- KEY CHANGE
  Function = yellow
  String = green
  Comment = gray (italic)
  ```

**Create theme helper module:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/theme.lua`
- Export color palette constants
- Helper functions for theme detection

**Update colorscheme plugin spec:**
- Modify: `dotfiles/nvim/.config/nvim/lua/plugins/colorscheme.lua`
- Remove tokyonight
- Add gruvbox-forge configuration
- Set as default: `vim.cmd.colorscheme("gruvbox-forge")`

**Expected Result:**
- ✅ Theme loads without errors
- ✅ Keywords appear in Vulcan orange (#f97316)
- ✅ Syntax highlighting looks like gruvbox but with orange keywords
- ✅ Borders and accents use orange

---

### Task 2: Fix LSP Configuration

**Review and fix LSP servers:**
- Modify: `dotfiles/nvim/.config/nvim/lua/plugins/lsp.lua`
- Verify server names are correct:
  - TypeScript: `ts_ls` (may need `tsserver` depending on nvim version)
  - Rust: `rust_analyzer`
  - Go: `gopls`
  - Python: `pyright`
  - Lua: `lua_ls`
  - Bash: Check if configured
- Ensure on_attach keybindings work (gd, gr, K, <leader>rn, etc.)
- Test that capabilities include completion support

**Expected Result:**
- ✅ All 6+ language servers start without errors
- ✅ Code completion works
- ✅ Go-to-definition works (gd)
- ✅ Hover documentation works (K)
- ✅ Diagnostics appear inline

---

### Task 3: Markdown Workflow

**Create markdown helper module:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/markdown.lua`
- Implement functions:
  - `toggle_checkbox()` - Toggles `[ ]` ↔ `[x]` on current line
  - `insert_date()` - Inserts current date in YYYY-MM-DD format
  - `insert_heading()` - Inserts markdown heading

**Add markdown plugin (optional):**
- Modify: `dotfiles/nvim/.config/nvim/lua/plugins/editor.lua`
- Consider adding `bullets.vim` or similar if needed
- Or rely on custom Lua functions only

**Add markdown keybindings:**
- Modify: `dotfiles/nvim/.config/nvim/lua/config/keymaps.lua`
- `<leader>mt` → Toggle checkbox (calls toggle_checkbox())
- `<leader>md` → Insert date
- `<leader>mh` → Insert heading

**Expected Result:**
- ✅ `<leader>mt` toggles checkboxes in markdown files
- ✅ Works in TODO.md specifically
- ✅ Date/heading insertion functional

---

### Task 4: VulcanOS Dashboard

**Create dashboard module:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/dashboard.lua`
- VulcanOS ASCII art (VULCAN text + volcano emoji)
- Quick links configuration:
  - "📝 Open TODO" → opens ~/VulcanOS/TODO.md
  - "📁 Find Files" → Telescope find_files
  - "🔍 Search Project" → Telescope live_grep
  - "📚 Help" → :help vulcan
  - "⚙️  Config" → Edit nvim config
  - "🚀 Plugins" → :Lazy

**Update alpha-nvim config:**
- Modify: `dotfiles/nvim/.config/nvim/lua/plugins/ui.lua`
- Replace existing dashboard with VulcanOS version
- Use dashboard.lua for ASCII art and buttons
- Set volcano emoji in header/footer

**Expected Result:**
- ✅ Dashboard appears on nvim startup
- ✅ Shows VulcanOS branding
- ✅ Quick links work and navigate correctly
- ✅ Volcano emoji visible (🌋)

---

### Task 5: VulcanOS Commands & Keybindings

**Create custom commands:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/commands.lua`
- `:VulcanTasks` → Opens ~/VulcanOS/TODO.md
- `:VulcanConfig` → Opens ~/.config/nvim/init.lua
- `:VulcanScripts` → Telescope search in ~/VulcanOS/dotfiles/scripts/

**Create VulcanOS keybindings:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/keymaps.lua`
- `<leader>tt` → Open TODO.md (calls :VulcanTasks)
- `<leader>ve` → Edit nvim config (calls :VulcanConfig)
- `<leader>vs` → Search vulcan scripts (calls :VulcanScripts)
- `<leader>vd` → Open ~/VulcanOS/dotfiles/ directory
- `<leader>vr` → Open ~/VulcanOS/README.md

**Create project detection:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/projects.lua`
- Function: `is_vulcanos_repo()` - detects if pwd is VulcanOS
- Helper: `get_vulcanos_root()` - returns path to VulcanOS root
- Used for context-aware features (Phase 2: git branch emoji)

**Expected Result:**
- ✅ `:VulcanTasks` opens TODO.md
- ✅ `<leader>tt` opens TODO.md
- ✅ All custom commands work
- ✅ Keybindings fire without errors
- ✅ Project detection functions return correct values

---

### Task 6: Module Integration

**Create vulcan module init:**
- Location: `dotfiles/nvim/.config/nvim/lua/vulcan/init.lua`
- Loads all vulcan submodules:
  - `require("vulcan.theme")`
  - `require("vulcan.commands")`
  - `require("vulcan.keymaps")`
  - `require("vulcan.dashboard")`
  - `require("vulcan.markdown")`
  - `require("vulcan.projects")`

**Update main init.lua:**
- Modify: `dotfiles/nvim/.config/nvim/init.lua`
- Add after existing requires:
  ```lua
  -- VulcanOS customizations
  require("vulcan")
  ```

**Expected Result:**
- ✅ All vulcan modules load without errors
- ✅ No conflicts with existing config
- ✅ Startup time remains fast (<100ms)

---

## File Structure After Phase 1

```
dotfiles/nvim/.config/nvim/
├── init.lua                          # MODIFY: Add require("vulcan")
├── lua/
│   ├── config/
│   │   ├── options.lua              # Existing
│   │   ├── keymaps.lua              # MODIFY: Add markdown keybindings
│   │   ├── autocmds.lua             # Existing
│   │   └── lazy.lua                 # Existing
│   │
│   ├── plugins/
│   │   ├── colorscheme.lua          # REPLACE: gruvbox-forge
│   │   ├── lsp.lua                  # VERIFY/FIX: Server names
│   │   ├── editor.lua               # Existing (maybe add markdown plugin)
│   │   ├── ui.lua                   # MODIFY: Dashboard update
│   │   ├── completion.lua           # Existing
│   │   ├── telescope.lua            # Existing
│   │   └── treesitter.lua           # Existing
│   │
│   └── vulcan/                       # NEW MODULE
│       ├── init.lua                 # Module loader
│       ├── theme.lua                # Color palette helpers
│       ├── commands.lua             # :Vulcan* commands
│       ├── keymaps.lua              # VulcanOS keybindings
│       ├── dashboard.lua            # ASCII art, quick links
│       ├── markdown.lua             # Checkbox toggle, etc.
│       └── projects.lua             # Project detection
│
└── colors/
    └── gruvbox-forge.lua            # NEW: Colorscheme definition
```

---

## Testing Checklist

### After Implementation:
1. **Startup test:**
   ```bash
   nvim
   # Should show VulcanOS dashboard
   # No errors in :messages
   ```

2. **Theme test:**
   ```bash
   nvim test.lua
   # Keywords should be orange (#f97316)
   # Strings should be green
   # Comments should be gray italic
   ```

3. **LSP test:**
   ```bash
   nvim test.ts
   # Type code, verify completion works
   # Hover over function (K), verify docs appear
   # Go to definition (gd), verify navigation
   ```

4. **Markdown test:**
   ```bash
   nvim TODO.md
   # Go to line with [ ] or [x]
   # Press <leader>mt
   # Should toggle checkbox
   ```

5. **Commands test:**
   ```bash
   nvim
   :VulcanTasks    # Should open TODO.md
   :VulcanConfig   # Should open init.lua
   ```

6. **Keybindings test:**
   ```bash
   nvim
   # Press <leader>tt
   # Should open TODO.md
   ```

---

## Success Criteria

**Phase 1 is complete when:**
- ✅ Gruvbox-forge theme loads and looks correct
- ✅ All 6+ LSP servers functional (completion, diagnostics, navigation)
- ✅ Markdown checkboxes toggle with `<leader>mt`
- ✅ TODO.md opens with `<leader>tt`
- ✅ VulcanOS dashboard shows on startup
- ✅ Custom `:Vulcan*` commands work
- ✅ No errors on startup (`:messages` is clean)
- ✅ Keybindings don't conflict with existing mappings
- ✅ Project detection functions work

---

## Important Notes

### Color Scheme Research
- Look at existing gruvbox.nvim implementation for structure
- Neovim colorscheme uses highlight groups (`:h highlight`)
- Key groups to override:
  - `Keyword` → orange
  - `Function` → yellow
  - `String` → green
  - `Comment` → gray italic
  - `Normal` → bg/fg
  - `BorderActive` → orange
  - `StatusLine` → gruvbox bg with orange accents

### Leader Key
- Already set to Space: `vim.g.mapleader = " "`
- All new keybindings use `<leader>` prefix
- Namespace: `<leader>t*` for tasks, `<leader>v*` for VulcanOS

### Paths
- TODO.md location: `~/VulcanOS/TODO.md`
- VulcanOS root: `~/VulcanOS/`
- Dotfiles: `~/VulcanOS/dotfiles/`
- Scripts: `~/VulcanOS/dotfiles/scripts/.local/bin/vulcan-*`
- Use `vim.fn.expand("~/VulcanOS/...")` for paths

### Project Detection
- Check if `vim.fn.getcwd()` contains "VulcanOS"
- Or check for existence of specific files (VERSION, profiledef.sh)
- Return boolean for is_vulcanos_repo()

### ASCII Art Guidelines
- Keep it simple and readable
- Max width: ~60 characters (fits 80-column terminal)
- Use `figlet -f standard "VULCAN"` as reference
- Add volcano emoji above/below text, not inline

---

## Expected Deliverables

After this orchestrator session completes:

1. **Working colorscheme**
   - `colors/gruvbox-forge.lua` created
   - Theme loads without errors
   - Matches design spec (gruvbox + orange)

2. **VulcanOS module**
   - `lua/vulcan/` directory with 6+ files
   - All functions exported and documented
   - No errors when loaded

3. **Updated configs**
   - `init.lua` requires vulcan module
   - `lua/plugins/colorscheme.lua` uses gruvbox-forge
   - `lua/plugins/ui.lua` uses VulcanOS dashboard
   - `lua/config/keymaps.lua` includes markdown shortcuts

4. **Functional features**
   - ✅ Theme applied
   - ✅ Dashboard shows on startup
   - ✅ TODO.md opens with `<leader>tt`
   - ✅ Checkboxes toggle with `<leader>mt`
   - ✅ LSPs work for all languages
   - ✅ Custom commands execute

5. **Documentation**
   - Code comments in vulcan modules
   - Keybinding documentation in keymaps.lua
   - README-style comments in dashboard.lua

---

## Post-Implementation

After Phase 1 completes successfully:

1. **Test locally:**
   ```bash
   cd ~/VulcanOS
   nvim
   # Verify everything works
   ```

2. **Commit changes:**
   ```bash
   git add dotfiles/nvim/
   git commit -m "feat(nvim): VulcanEdit Phase 1 - Foundation
   
   - Add gruvbox-forge colorscheme (gruvbox + Vulcan orange)
   - Create vulcan module for VulcanOS integration
   - Add VulcanOS dashboard with ASCII art
   - Implement markdown workflow (checkbox toggling)
   - Add custom :Vulcan* commands
   - Configure keybindings for VulcanOS workflow"
   ```

3. **Prepare for Phase 2:**
   - Review Phase 1 implementation
   - Identify any issues or improvements
   - Plan Phase 2 session (statusline, launcher, system integration)

---

## Orchestrator Instructions

**Session Approach:**
1. **Research Phase:**
   - Examine existing nvim colorscheme structures (gruvbox.nvim)
   - Review current nvim config to understand architecture
   - Identify any potential conflicts or issues

2. **Implementation Phase:**
   - Create colorscheme first (foundation)
   - Build vulcan module structure
   - Implement features incrementally
   - Test each component before moving to next

3. **Integration Phase:**
   - Connect all pieces together
   - Update main init.lua
   - Test end-to-end workflow
   - Fix any integration issues

4. **Validation Phase:**
   - Run all tests from checklist
   - Verify no errors on startup
   - Confirm all keybindings work
   - Document any issues found

**Recommended Agents:**
- **@researcher** - Initial research on gruvbox colorschemes
- **@build** - Implementation of colorscheme and modules
- **@frontend-engineer** - UI components (dashboard, statusline prep)
- **@test-engineer** - Validation and testing (if needed)
- **@reviewer** - Final review before completion

**Parallelization:**
- Colorscheme can be built independently
- VulcanOS modules (commands, keymaps, dashboard) can be built in parallel
- LSP fixes can happen alongside other work

---

## Resources

- **Plan Document:** `.orchestrator/VULCANEDIT-PLAN.md`
- **Current Config:** `dotfiles/nvim/.config/nvim/`
- **Color Reference:** `dotfiles/themes/colors/gruvbox-dark.sh`
- **Existing Nvim Config:** Already functional, just needs enhancement

**External References:**
- Neovim colorscheme guide: `:h highlight`
- Gruvbox official: https://github.com/morhetz/gruvbox
- Lazy.nvim docs: https://github.com/folke/lazy.nvim

---

**Ready to implement Phase 1 of VulcanEdit!** 🌋
