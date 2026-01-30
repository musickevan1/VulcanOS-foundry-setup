//! Third-party application theming discovery service
//!
//! Detects installed applications that support theming and checks
//! their configuration status.

use std::path::PathBuf;

/// Information about a themeable application
#[derive(Debug, Clone)]
pub struct AppTheming {
    /// Display name of the application
    pub name: String,
    /// Whether the application is installed (binary found)
    pub installed: bool,
    /// Whether the application has theme configuration
    pub configured: bool,
    /// URL to theme marketplace or documentation
    pub docs_url: String,
    /// Icon name (freedesktop icon spec)
    pub icon: String,
}

/// Get XDG config directory, respecting XDG_CONFIG_HOME
fn config_dir() -> Option<PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .ok()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
}

/// Detect Neovim installation and theme configuration
fn detect_neovim() -> AppTheming {
    let installed = which::which("nvim").is_ok();

    let configured = config_dir()
        .map(|d| d.join("nvim"))
        .filter(|p| p.exists())
        .and_then(|nvim_dir| {
            // Check for colorscheme in init.lua or init.vim
            let init_lua = nvim_dir.join("init.lua");
            let init_vim = nvim_dir.join("init.vim");

            if init_lua.exists() {
                std::fs::read_to_string(init_lua).ok()
                    .map(|s| s.contains("colorscheme") || s.contains("vim.cmd.colorscheme"))
            } else if init_vim.exists() {
                std::fs::read_to_string(init_vim).ok()
                    .map(|s| s.contains("colorscheme"))
            } else {
                Some(false)
            }
        })
        .unwrap_or(false);

    AppTheming {
        name: "Neovim".to_string(),
        installed,
        configured,
        docs_url: "https://github.com/topics/neovim-colorscheme".to_string(),
        icon: "nvim".to_string(),
    }
}

/// Detect Kitty terminal installation and theme configuration
fn detect_kitty() -> AppTheming {
    let installed = which::which("kitty").is_ok();

    let configured = config_dir()
        .map(|d| d.join("kitty"))
        .filter(|p| p.exists())
        .map(|kitty_dir| {
            // Check for current-theme.conf or theme include in kitty.conf
            let current_theme = kitty_dir.join("current-theme.conf");
            let kitty_conf = kitty_dir.join("kitty.conf");

            current_theme.exists() ||
            kitty_conf.exists() && std::fs::read_to_string(kitty_conf)
                .map(|s| s.contains("include") && s.contains("theme"))
                .unwrap_or(false)
        })
        .unwrap_or(false);

    AppTheming {
        name: "Kitty".to_string(),
        installed,
        configured,
        docs_url: "https://sw.kovidgoyal.net/kitty/kittens/themes/".to_string(),
        icon: "kitty".to_string(),
    }
}

/// Detect btop installation and theme configuration
fn detect_btop() -> AppTheming {
    let installed = which::which("btop").is_ok();

    let configured = config_dir()
        .map(|d| d.join("btop/btop.conf"))
        .filter(|p| p.exists())
        .and_then(|conf| std::fs::read_to_string(conf).ok())
        .map(|s| s.contains("color_theme"))
        .unwrap_or(false);

    AppTheming {
        name: "btop".to_string(),
        installed,
        configured,
        docs_url: "https://github.com/aristocratos/btop#themes".to_string(),
        icon: "utilities-system-monitor".to_string(),
    }
}

/// Detect VS Code installation and theme configuration
fn detect_vscode() -> AppTheming {
    // VS Code can be installed as 'code', 'code-oss', or 'codium'
    let installed = which::which("code").is_ok()
        || which::which("code-oss").is_ok()
        || which::which("codium").is_ok();

    let configured = config_dir()
        .map(|d| d.join("Code/User/settings.json"))
        .filter(|p| p.exists())
        .and_then(|settings| std::fs::read_to_string(settings).ok())
        .map(|s| s.contains("workbench.colorTheme"))
        .unwrap_or(false);

    AppTheming {
        name: "VS Code".to_string(),
        installed,
        configured,
        docs_url: "https://marketplace.visualstudio.com/search?term=theme&target=VSCode&category=Themes".to_string(),
        icon: "visual-studio-code".to_string(),
    }
}

/// Detect Firefox installation and userChrome status
fn detect_firefox() -> AppTheming {
    let installed = which::which("firefox").is_ok();

    // Firefox profiles are in ~/.mozilla/firefox/
    let configured = dirs::home_dir()
        .map(|h| h.join(".mozilla/firefox"))
        .filter(|p| p.exists())
        .and_then(|ff_dir| {
            // Check any profile for chrome/userChrome.css
            std::fs::read_dir(ff_dir).ok().and_then(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .any(|profile| profile.path().join("chrome/userChrome.css").exists())
                    .then_some(true)
            })
        })
        .unwrap_or(false);

    AppTheming {
        name: "Firefox".to_string(),
        installed,
        configured,
        docs_url: "https://github.com/nicoth-in/nicothin-firefox-theme".to_string(),
        icon: "firefox".to_string(),
    }
}

/// Detect Alacritty terminal installation and theme configuration
fn detect_alacritty() -> AppTheming {
    let installed = which::which("alacritty").is_ok();

    let configured = config_dir()
        .map(|d| d.join("alacritty/alacritty.toml"))
        .filter(|p| p.exists())
        .and_then(|conf| std::fs::read_to_string(conf).ok())
        .map(|s| s.contains("[colors") || s.contains("import"))
        .unwrap_or(false);

    AppTheming {
        name: "Alacritty".to_string(),
        installed,
        configured,
        docs_url: "https://github.com/alacritty/alacritty-theme".to_string(),
        icon: "Alacritty".to_string(),
    }
}

/// Discover all supported themeable applications
pub fn discover_apps() -> Vec<AppTheming> {
    vec![
        detect_neovim(),
        detect_kitty(),
        detect_alacritty(),
        detect_btop(),
        detect_vscode(),
        detect_firefox(),
    ]
}

/// Open URL in default browser (non-blocking)
pub fn open_url(url: &str) {
    let url = url.to_string();
    std::thread::spawn(move || {
        if let Err(e) = open::that(&url) {
            eprintln!("Failed to open URL: {}", e);
        }
    });
}
