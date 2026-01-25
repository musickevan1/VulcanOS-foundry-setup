use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

use crate::models::Theme;
use crate::services::theme_storage;
use super::theme_card::{ThemeItem, ThemeCardOutput};

/// Input messages for ThemeBrowser
#[derive(Debug)]
pub enum ThemeBrowserInput {
    Refresh,
    SetCurrentTheme(String),
}

/// Output messages from ThemeBrowser
#[derive(Debug)]
pub enum ThemeBrowserOutput {
    ThemeSelected(Theme),
}

/// Theme browser component with FlowBox grid of themes
pub struct ThemeBrowserModel {
    themes: FactoryVecDeque<ThemeItem>,
    current_theme_id: String,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeBrowserModel {
    type Init = ();
    type Input = ThemeBrowserInput;
    type Output = ThemeBrowserOutput;

    view! {
        gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,
            set_policy: (gtk::PolicyType::Never, gtk::PolicyType::Automatic),

            #[local_ref]
            themes_box -> gtk::FlowBox {
                set_selection_mode: gtk::SelectionMode::Single,
                set_homogeneous: false,
                set_max_children_per_line: 6,
                set_min_children_per_line: 2,
                set_row_spacing: 12,
                set_column_spacing: 12,
                set_margin_all: 12,
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Get current theme
        let current_theme_id = crate::services::theme_applier::get_current_theme()
            .unwrap_or_else(|_| "tokyonight".to_string());

        // Create factory with message forwarding
        let mut themes = FactoryVecDeque::builder()
            .launch(gtk::FlowBox::default())
            .forward(sender.output_sender(), |msg| {
                match msg {
                    ThemeCardOutput::Selected(theme) => ThemeBrowserOutput::ThemeSelected(theme),
                }
            });

        // Load themes
        if let Ok(theme_list) = theme_storage::load_all_themes() {
            for theme in theme_list {
                let is_current = theme.theme_id == current_theme_id;
                themes.guard().push_back((theme, is_current));
            }
        }

        let model = ThemeBrowserModel {
            themes,
            current_theme_id,
        };

        let themes_box = model.themes.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ThemeBrowserInput::Refresh => {
                // Clear and reload themes
                self.themes.guard().clear();

                if let Ok(current) = crate::services::theme_applier::get_current_theme() {
                    self.current_theme_id = current;
                }

                if let Ok(theme_list) = theme_storage::load_all_themes() {
                    for theme in theme_list {
                        let is_current = theme.theme_id == self.current_theme_id;
                        self.themes.guard().push_back((theme, is_current));
                    }
                }
            }

            ThemeBrowserInput::SetCurrentTheme(theme_id) => {
                self.current_theme_id = theme_id.clone();

                // Update current indicators - need to rebuild
                let mut guard = self.themes.guard();
                let count = guard.len();
                for i in 0..count {
                    if let Some(item) = guard.get_mut(i) {
                        item.is_current = item.theme.theme_id == theme_id;
                    }
                }
            }
        }
    }
}
