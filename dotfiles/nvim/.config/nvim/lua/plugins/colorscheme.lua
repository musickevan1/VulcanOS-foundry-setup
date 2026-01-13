-- Gruvbox-Forge Colorscheme with Vulcan Orange Accents
-- Replaces Tokyo Night with gruvbox-forge
-- Created for VulcanEdit - Phase 1

return {
  {
    "gruvbox-community/gruvbox.nvim",
    lazy = false,
    priority = 1000,
    opts = {
      undercurl = true,
      underline = true,
      bold = true,
      italic = {
        comments = true,
        strings = true,
        operators = false,
        folds = false,
      },
      strikethrough = true,
      invert_selection = false,
      invert_signs = false,
      invert_tabline = false,
      invert_intend_guides = false,
      contrast = "hard",
      palette_overrides = {
        -- Override gruvbox blue with Vulcan orange
        bright_blue = "#f97316",
        blue = "#f97316",
        dark_blue = "#ea580c",
        -- Keep original colors that work well
        bright_green = "#b8bb26",
        green = "#98971a",
        bright_yellow = "#fabd2f",
        yellow = "#d79921",
        bright_orange = "#f97316",
        orange = "#d65d0e",
        bright_red = "#fb4934",
        red = "#cc241d",
        bright_purple = "#d3869b",
        purple = "#b16286",
        bright_aqua = "#8ec07c",
        aqua = "#689d6a",
        bright_gray = "#928374",
        gray = "#928374",
      },
      overrides = {},
      dim_inactive = false,
      transparent_mode = false,
    },
    config = function(_, opts)
      require("gruvbox").setup(opts)
      -- Apply Vulcan orange overrides via our custom colorscheme
      vim.cmd("colorscheme gruvbox")

      -- Apply Vulcan orange accents to key highlight groups
      vim.cmd([[
        " Keywords in Vulcan orange
        hi Keyword guifg=#f97316 guibg=NONE gui=NONE
        hi @keyword guifg=#f97316

        " Statements and conditionals
        hi Statement guifg=#f97316 gui=bold
        hi @statement guifg=#f97316
        hi Conditional guifg=#f97316
        hi @conditional guifg=#f97316
        hi Repeat guifg=#f97316
        hi @repeat guifg=#f97316

        " Operators and special
        hi Operator guifg=#c0b69e
        hi @operator guifg=#c0b69e
        hi Special guifg=#d3869b

        " Function calls in yellow
        hi Function guifg=#fabd2f
        hi @function guifg=#fabd2f
        hi @method guifg=#fabd2f

        " Strings in green
        hi String guifg=#b8bb26
        hi @string guifg=#b8bb26

        " Comments stay gray
        hi Comment guifg=#928374 gui=italic
        hi @comment guifg=#928374 gui=italic

        " Types in cyan
        hi Type guifg=#8ec07c
        hi @type guifg=#8ec07c
        hi @type.builtin guifg=#8ec07c

        " Active borders in orange
        hi CursorLineNr guifg=#f97316 guibg=#504945 gui=bold
        hi Search guifg=#282828 guibg=#f97316
        hi IncSearch guifg=#282828 guibg=#f97316
        hi Substitute guifg=#282828 guibg=#f97316
        hi WildMenu guifg=#282828 guibg=#f97316 gui=bold

        " Pmenu selection in orange
        hi PmenuSel guifg=#282828 guibg=#f97316 gui=bold

        " Telescope matching
        hi TelescopeMatching guifg=#f97316
        hi TelescopeSelection guifg=#282828 guibg=#f97316 gui=bold

        " Diagnostic virtual text
        hi DiagnosticVirtualTextWarn guifg=#d65d0e guibg=#504945
        hi DiagnosticVirtualTextHint guifg=#98971a guibg=#504945
        hi DiagnosticVirtualTextInfo guifg=#458588 guibg=#504945
        hi DiagnosticUnderlineWarn guifg=NONE guibg=NONE gui=underline
        hi DiagnosticUnderlineHint guifg=NONE guibg=NONE gui=underline
        hi DiagnosticUnderlineInfo guifg=NONE guibg=NONE gui=underline
      ]])
    end,
  },
}
