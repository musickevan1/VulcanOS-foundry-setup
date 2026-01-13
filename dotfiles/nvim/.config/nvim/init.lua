-- VulcanOS Neovim Configuration
-- Bootstrap lazy.nvim and load configuration modules

-- Set leader key before lazy
vim.g.mapleader = " "
vim.g.maplocalleader = " "

-- Bootstrap lazy.nvim
require("config.lazy")

-- Load configuration
require("config.options")
require("config.keymaps")
require("config.autocmds")

-- Load VulcanOS customizations (Phase 1)
require("vulcan")
