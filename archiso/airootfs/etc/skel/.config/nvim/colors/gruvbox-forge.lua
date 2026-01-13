-- Gruvbox-Forge Colorscheme for VulcanOS
-- Based on gruvbox with Vulcan orange (#f97316) accents
-- Created for VulcanEdit - Phase 1

local M = {}

-- Color palette - Gruvbox base with Vulcan orange accent
M.colors = {
  -- Backgrounds (gruvbox dark)
  bg0 = "#282828",  -- Main background
  bg1 = "#3c3836",  -- Popup background
  bg2 = "#504945",  -- Selection, cursor line
  bg3 = "#665c54",  -- Comment background
  bg4 = "#7c6f64",  -- Statusline, visual selection

  -- Foregrounds (gruvbox)
  fg0 = "#ebdbb2",  -- Main foreground
  fg1 = "#d5c4a1",  -- Lighter foreground
  fg2 = "#c0b69e",  -- Dimmed foreground
  gray = "#928374", -- Comments, muted text

  -- Accents (VULCAN ORANGE PRIMARY)
  orange = "#f97316",    -- Vulcan orange - replaces gruvbox blue
  orange_dim = "#ea580c", -- Darker orange for subtle accents
  yellow = "#fabd2f",    -- Functions
  green = "#b8bb26",     -- Strings
  pink = "#d3869b",      -- Constants, special
  cyan = "#8ec07c",      -- Classes, types

  -- Semantic colors (gruvbox)
  red = "#cc241d",
  red_bright = "#fb4934",
  green_dim = "#98971a",
  yellow_dim = "#d79921",
  blue_gruvbox = "#458588",
  purple = "#b16286",
  cyan_dim = "#689d6a",
  orange_gruvbox = "#d65d0e",

  -- UI colors
  border = "#504945",
  border_active = "#f97316", -- Orange for active borders
  cursor = "#ebdbb2",
  cursorline = "#3c3836",
  selection = "#504945",
  nontext = "#504945",

  -- Diagnostic colors
  error = "#cc241d",
  warn = "#d65d0e",
  hint = "#98971a",
  info = "#458588",

  -- Telescope
  telescope_fg = "#ebdbb2",
  telescope_bg = "#282828",
  telescope_border = "#504945",
  telescope_match = "#f97316",
}

-- Helper function to set highlights
local function hi(group, fg, bg, attr)
  local cmd = {}
  if fg then cmd[#cmd + 1] = "guifg=" .. fg end
  if bg then cmd[#cmd + 1] = "guibg=" .. bg end
  if attr then cmd[#cmd + 1] = "gui=" .. attr end
  if attr == "none" then cmd[#cmd + 1] = "guinone=1" end
  vim.cmd("hi! " .. group .. " " .. table.concat(cmd, " "))
end

-- Highlight group definitions
function M.setup()
  local colors = M.colors

  -- Clear existing highlights
  vim.cmd("hi clear")

  -- Set background and syntax
  vim.o.background = "dark"
  vim.cmd("syntax reset")
  vim.cmd("set termguicolors")

  -- Normal
  hi("Normal", colors.fg0, colors.bg0)
  hi("NormalNC", colors.fg0, colors.bg0)
  hi("Comment", colors.gray, nil, "italic")
  hi("Constant", colors.pink)
  hi("String", colors.green)
  hi("Character", colors.pink)
  hi("Number", colors.orange_gruvbox)
  hi("Boolean", colors.orange_gruvbox)
  hi("Float", colors.orange_gruvbox)

  -- Identifier (VULCAN ORANGE for keywords)
  hi("Keyword", colors.orange, nil, "none") -- Vulcan orange!
  hi("Identifier", colors.cyan)
  hi("Function", colors.yellow)
  hi("Method", colors.yellow)
  hi("Field", colors.cyan)
  hi("Property", colors.cyan)

  -- Statement
  hi("Statement", colors.orange, nil, "bold")
  hi("Conditional", colors.orange)
  hi("Repeat", colors.orange)
  hi("Label", colors.orange)
  hi("Operator", colors.fg2)
  hi("Exception", colors.orange)

  -- Preprocessor
  hi("PreProc", colors.cyan_dim)
  hi("Macro", colors.orange)
  hi("Define", colors.cyan_dim)
  hi("PreCondit", colors.cyan_dim)

  -- Type
  hi("Type", colors.cyan)
  hi("StorageClass", colors.orange)
  hi("Structure", colors.cyan)
  hi("Typedef", colors.orange)

  -- Special
  hi("Special", colors.pink)
  hi("SpecialChar", colors.pink)
  hi("Tag", colors.orange)
  hi("Delimiter", colors.fg2)
  hi("SpecialComment", colors.gray, nil, "italic")
  hi("Debug", colors.gray)

  -- Underlined
  hi("Underlined", colors.blue_gruvbox, nil, "underline")

  -- Ignore
  hi("Ignore", colors.gray)

  -- Error
  hi("Error", colors.red, nil, "bold")
  hi("ErrorMsg", colors.red)
  hi("WarningMsg", colors.orange_gruvbox)
  hi("MoreMsg", colors.fg0)
  hi("ModeMsg", colors.fg0)
  hi("MsgArea", colors.fg0)

  -- Line numbers
  hi("LineNr", colors.gray)
  hi("LineNrAbove", colors.gray)
  hi("LineNrBelow", colors.gray)
  hi("CursorLineNr", colors.orange, colors.bg2, "bold")

  -- Cursor
  hi("Cursor", colors.bg0, colors.orange)
  hi("CursorColumn", nil, colors.bg2)
  hi("CursorLine", nil, colors.bg2)

  -- Visual
  hi("Visual", nil, colors.bg4)
  hi("VisualNOS", nil, colors.bg4)

  -- Search
  hi("Search", colors.bg0, colors.orange)
  hi("IncSearch", colors.bg0, colors.orange)
  hi("Substitute", colors.bg0, colors.orange)

  -- Folds
  hi("FoldColumn", colors.gray, colors.bg1)
  hi("Folded", colors.fg2, colors.bg1)

  -- Pmenu
  hi("Pmenu", colors.fg0, colors.bg1)
  hi("PmenuSel", colors.bg0, colors.orange, "bold")
  hi("PmenuSbar", nil, colors.bg2)
  hi("PmenuThumb", nil, colors.bg4)

  -- Wild menu
  hi("WildMenu", colors.bg0, colors.orange)

  -- Tab line
  hi("TabLine", colors.fg1, colors.bg1)
  hi("TabLineSel", colors.bg0, colors.orange, "bold")
  hi("TabLineFill", colors.gray, colors.bg1)

  -- Statusline
  hi("StatusLine", colors.fg0, colors.bg2)
  hi("StatusLineNC", colors.gray, colors.bg1)
  hi("StatusLineError", colors.red, colors.bg2)
  hi("StatusLineWarn", colors.orange_gruvbox, colors.bg2)

  -- Winbar
  hi("WinBar", colors.fg1, colors.bg0)
  hi("WinBarNC", colors.gray, colors.bg0)

  -- Borders
  hi("FloatBorder", colors.gray, colors.bg1)
  hi("NormalFloat", colors.fg0, colors.bg1)
  hi("CursorIM", colors.bg0, colors.orange)

  -- Diagnostic
  hi("DiagnosticError", colors.error)
  hi("DiagnosticWarn", colors.warn)
  hi("DiagnosticHint", colors.hint)
  hi("DiagnosticInfo", colors.info)
  hi("DiagnosticUnderlineError", nil, nil, "underline")
  hi("DiagnosticUnderlineWarn", nil, nil, "underline")
  hi("DiagnosticUnderlineHint", nil, nil, "underline")
  hi("DiagnosticUnderlineInfo", nil, nil, "underline")
  hi("DiagnosticVirtualTextError", colors.error, colors.bg2)
  hi("DiagnosticVirtualTextWarn", colors.warn, colors.bg2)
  hi("DiagnosticVirtualTextHint", colors.hint, colors.bg2)
  hi("DiagnosticVirtualTextInfo", colors.info, colors.bg2)
  hi("DiagnosticSignError", colors.error, colors.bg1)
  hi("DiagnosticSignWarn", colors.warn, colors.bg1)
  hi("DiagnosticSignHint", colors.hint, colors.bg1)
  hi("DiagnosticSignInfo", colors.info, colors.bg1)

  -- LSP
  hi("LspReferenceText", nil, colors.bg2)
  hi("LspReferenceRead", nil, colors.bg2)
  hi("LspReferenceWrite", nil, colors.bg2)
  hi("LspCodeLens", colors.gray)
  hi("LspCodeLensSeparator", colors.gray)
  hi("LspInlayHint", colors.gray, colors.bg2)

  -- Treesitter
  hi("@attribute", colors.orange)
  hi("@boolean", colors.orange_gruvbox)
  hi("@character", colors.pink)
  hi("@character.special", colors.pink)
  hi("@comment", colors.gray, nil, "italic")
  hi("@conceal", colors.gray)
  hi("@conditional", colors.orange)
  hi("@constant", colors.pink)
  hi("@constant.builtin", colors.pink)
  hi("@constant.macro", colors.pink)
  hi("@constructor", colors.cyan)
  hi("@debug", colors.gray)
  hi("@definition", colors.yellow)
  hi("@definition.readonly", colors.pink)
  hi("@definition.var", colors.yellow)
  hi("@deprecated", colors.gray, nil, "strikethrough")
  hi("@emphasis", nil, nil, "italic")
  hi("@enum", colors.cyan)
  hi("@enumMember", colors.pink)
  hi("@error", colors.error)
  hi("@escape", colors.pink)
  hi("@event", colors.orange)
  hi("@exception", colors.orange)
  hi("@field", colors.cyan)
  hi("@float", colors.orange_gruvbox)
  hi("@function", colors.yellow)
  hi("@function.builtin", colors.yellow)
  hi("@function.call", colors.yellow)
  hi("@function.macro", colors.orange)
  hi("@include", colors.orange)
  hi("@keyword", colors.orange, nil, "none")
  hi("@keyword.function", colors.orange)
  hi("@keyword.operator", colors.orange)
  hi("@keyword.return", colors.orange)
  hi("@label", colors.orange)
  hi("@macro", colors.orange)
  hi("@math", colors.yellow)
  hi("@method", colors.yellow)
  hi("@method.call", colors.yellow)
  hi("@namespace", colors.cyan)
  hi("@none", colors.fg0)
  hi("@number", colors.orange_gruvbox)
  hi("@operator", colors.fg2)
  hi("@parameter", colors.cyan)
  hi("@property", colors.cyan)
  hi("@punctuation", colors.fg2)
  hi("@punctuation.bracket", colors.fg2)
  hi("@punctuation.delimiter", colors.fg2)
  hi("@punctuation.special", colors.orange)
  hi("@repeat", colors.orange)
  hi("@storageclass", colors.orange)
  hi("@string", colors.green)
  hi("@string.escape", colors.pink)
  hi("@string.regex", colors.pink)
  hi("@string.special", colors.pink)
  hi("@structure", colors.cyan)
  hi("@tag", colors.orange)
  hi("@tag.attribute", colors.orange)
  hi("@tag.delimiter", colors.fg2)
  hi("@text", colors.fg0)
  hi("@text.strong", nil, nil, "bold")
  hi("@text.emphasis", nil, nil, "italic")
  hi("@text.underline", nil, nil, "underline")
  hi("@text.strike", nil, nil, "strikethrough")
  hi("@text.title", colors.yellow)
  hi("@text.literal", colors.green)
  hi("@text.uri", colors.blue_gruvbox, nil, "underline")
  hi("@text.math", colors.yellow)
  hi("@text.environment", colors.orange)
  hi("@text.environment.name", colors.cyan)
  hi("@text.note", colors.gray)
  hi("@text.warning", colors.warn)
  hi("@text.danger", colors.error)
  hi("@todo", colors.orange, nil, "bold")
  hi("@type", colors.cyan)
  hi("@type.builtin", colors.cyan)
  hi("@type.definition", colors.cyan)
  hi("@type.qualifier", colors.orange)
  hi("@variable", colors.fg0)
  hi("@variable.builtin", colors.pink)

  -- HTML/JSX/TSX specific
  hi("@tag.html", colors.orange)
  hi("@tag.delimiter.html", colors.fg2)
  hi("@attribute.html", colors.cyan)
  hi("@tag.jsx", colors.orange)
  hi("@tag.delimiter.jsx", colors.fg2)
  hi("@attribute.jsx", colors.cyan)

  -- GitSigns
  hi("GitSignsAdd", colors.green)
  hi("GitSignsChange", colors.yellow)
  hi("GitSignsDelete", colors.red)
  hi("GitSignsAddNr", colors.green)
  hi("GitSignsChangeNr", colors.yellow)
  hi("GitSignsDeleteNr", colors.red)
  hi("GitSignsAddLn", colors.green)
  hi("GitSignsChangeLn", colors.yellow)
  hi("GitSignsDeleteLn", colors.red)

  -- Diff
  hi("DiffAdd", colors.green, colors.bg1)
  hi("DiffChange", colors.yellow, colors.bg1)
  hi("DiffDelete", colors.red, colors.bg1)
  hi("DiffText", colors.orange, colors.bg1)
  hi("DiffAdded", colors.green)
  hi("DiffChanged", colors.yellow)
  hi("DiffRemoved", colors.red)
  hi("DiffFile", colors.orange)
  hi("DiffIndexLine", colors.gray)

  -- Telescope
  hi("TelescopeMatching", colors.orange)
  hi("TelescopeSelection", colors.bg0, colors.orange, "bold")
  hi("TelescopePromptPrefix", colors.orange)
  hi("TelescopeBorder", colors.gray, colors.bg0)
  hi("TelescopeTitle", colors.fg0)
  hi("TelescopePreviewTitle", colors.orange)

  -- Nvim-tree
  hi("NvimTreeSymlink", colors.cyan)
  hi("NvimTreeFolderName", colors.cyan)
  hi("NvimTreeFolderIcon", colors.cyan)
  hi("NvimTreeIndentMarker", colors.gray)
  hi("NvimTreeNormal", colors.fg0, colors.bg0)
  hi("NvimTreeCursorLine", nil, colors.bg2)
  hi("NvimTreeWinSeparator", colors.border)
  hi("NvimTreeCursor", colors.bg0, colors.orange)

  -- Bufferline
  hi("BufferLineIndicatorSelected", colors.orange)
  hi("BufferLineIndicator", colors.gray)
  hi("BufferLineCloseButton", colors.gray)
  hi("BufferLineCloseButtonSelected", colors.orange)
  hi("BufferLineFill", colors.bg1, colors.bg0)

  -- Notify (nvim-notify)
  hi("NotifyDEBUGBorder", colors.gray)
  hi("NotifyDEBUGIcon", colors.gray)
  hi("NotifyDEBUGTitle", colors.gray)
  hi("NotifyINFOBorder", colors.info)
  hi("NotifyINFOIcon", colors.info)
  hi("NotifyINFOTitle", colors.info)
  hi("NotifyWARNBorder", colors.warn)
  hi("NotifyWARNIcon", colors.warn)
  hi("NotifyWARNTitle", colors.warn)
  hi("NotifyERRORBorder", colors.error)
  hi("NotifyERRORIcon", colors.error)
  hi("NotifyERRORTitle", colors.error)
  hi("NotifyTRACEBorder", colors.cyan)
  hi("NotifyTRACEIcon", colors.cyan)
  hi("NotifyTRACETitle", colors.cyan)

  -- Noice
  hi("NoiceConfirmBorder", colors.gray)
  hi("NoiceCmdlineIcon", colors.orange)
  hi("NoiceCmdlinePopupBorder", colors.gray)
  hi("NoiceCmdlinePopupBorderError", colors.error)
  hi("NoiceCmdlinePopupBorderWarn", colors.warn)
  hi("NoiceCmdlinePopupBorderInfo", colors.info)
  hi("NoiceCmdlinePopupBorderHint", colors.hint)
  hi("NoiceCmdlinePopupBorderDebug", colors.gray)

  -- Which-key
  hi("WhichKey", colors.orange)
  hi("WhichKeyGroup", colors.cyan)
  hi("WhichKeyDesc", colors.yellow)
  hi("WhichKeySeperator", colors.gray)
  hi("WhichKeyFloat", colors.fg0, colors.bg1)

  -- Neo-tree
  hi("NeoTreeDimText", colors.gray)
  hi("NeoTreeDirectoryIcon", colors.cyan)
  hi("NeoTreeDirectoryName", colors.cyan)
  hi("NeoTreeNormal", colors.fg0, colors.bg0)
  hi("NeoTreeNormalNC", colors.fg0, colors.bg0)
  hi("NeoTreeCursorLine", nil, colors.bg2)
  hi("NeoTreeWinSeparator", colors.border)

  -- Indent blankline
  hi("IndentBlanklineChar", colors.bg3)
  hi("IndentBlanklineContextChar", colors.orange)

  -- Todo comments
  hi("Todo", colors.orange, nil, "bold")
  hi("TodoTODO", colors.orange, nil, "bold")
  hi("TodoFIXME", colors.error, nil, "bold")
  hi("TodoHACK", colors.warn, nil, "bold")
  hi("TodoWARN", colors.warn, nil, "bold")
  hi("TodoPERF", colors.yellow)
  hi("TodoNOTE", colors.hint)
  hi("TodoTEST", colors.info)

  -- Lazy
  hi("LazyButton", colors.fg2, colors.bg1)
  hi("LazyButtonActive", colors.bg0, colors.orange, "bold")
  hi("LazySpecial", colors.orange)
  hi("LazyUrl", colors.blue_gruvbox, nil, "underline")
  hi("LazyCommit", colors.pink)

  -- Alpha dashboard
  hi("AlphaHeader", colors.orange)
  hi("AlphaButton", colors.fg0)
  hi("AlphaButtonShortcut", colors.orange)
  hi("AlphaFooter", colors.gray)

  -- Yanky
  hi("YankYank", colors.orange)

  -- Leap
  hi("LeapMatch", colors.bg0, colors.orange, "bold")
  hi("LeapLabelPrimary", colors.bg0, colors.orange, "bold")
  hi("LeapLabelSecondary", colors.bg0, colors.yellow)
  hi("LeapLabelSelected", colors.bg0, colors.orange)
  hi("LeapBackdrop", colors.gray)
  hi("LeapCursor", colors.bg0, colors.orange)
  hi("LeapMotion", colors.orange)

  -- Flash
  hi("FlashBackdrop", colors.gray)
  hi("FlashLabel", colors.bg0, colors.orange, "bold")
  hi("FlashMatch", colors.bg0, colors.orange, "bold")
  hi("FlashCurrent", colors.bg0, colors.orange)
  hi("FlashPrompt", colors.bg0, colors.orange)
  hi("FlashSelected", colors.bg0, colors.orange)

  -- CMP (nvim-cmp)
  hi("CmpDocumentation", colors.fg0, colors.bg1)
  hi("CmpDocumentationBorder", colors.gray, colors.bg1)
  hi("CmpGhostText", colors.gray)
  hi("CmpMenu", colors.fg0, colors.bg1)
  hi("CmpMenuBorder", colors.gray, colors.bg1)
  hi("CmpItemAbbr", colors.fg0)
  hi("CmpItemAbbrDeprecated", colors.gray, nil, "strikethrough")
  hi("CmpItemAbbrMatch", colors.orange)
  hi("CmpItemAbbrMatchFuzzy", colors.orange)
  hi("CmpItemKindDefault", colors.orange)
  hi("CmpItemKindKeyword", colors.orange)
  hi("CmpItemKindVariable", colors.pink)
  hi("CmpItemKindConstant", colors.pink)
  hi("CmpItemKindClass", colors.cyan)
  hi("CmpItemKindStruct", colors.cyan)
  hi("CmpItemKindTypeParameter", colors.cyan)
  hi("CmpItemKindFunction", colors.yellow)
  hi("CmpItemKindMethod", colors.yellow)
  hi("CmpItemKindConstructor", colors.cyan)
  hi("CmpItemKindField", colors.cyan)
  hi("CmpItemKindProperty", colors.cyan)
  hi("CmpItemKindEvent", colors.orange)
  hi("CmpItemKindOperator", colors.fg2)
  hi("CmpItemKindReference", colors.pink)
  hi("CmpItemKindEnum", colors.cyan)
  hi("CmpItemKindValue", colors.orange_gruvbox)
  hi("CmpItemKindModule", colors.cyan)
  hi("CmpItemKindSnippet", colors.green)
  hi("CmpItemKindFile", colors.cyan)
  hi("CmpItemKindFolder", colors.cyan)
  hi("CmpItemKindEnumMember", colors.pink)
  hi("CmpItemKindInterface", colors.cyan)
  hi("CmpItemKindColor", colors.pink)
  hi("CmpItemKindUnit", colors.orange_gruvbox)
  hi("CmpItemKindText", colors.fg0)
  hi("CmpItemKindNull", colors.gray)

  -- Apply colorscheme
  vim.cmd("colorscheme gruvbox-forge")
end

-- Auto-setup on load
M.setup()

return M
