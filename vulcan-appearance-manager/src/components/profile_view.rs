use gtk::prelude::*;
use relm4::prelude::*;
use relm4::factory::FactoryVecDeque;

use crate::models::{UnifiedProfile, BindingMode};
use crate::services::profile_storage;
use super::profile_card::{ProfileItem, ProfileCardOutput};

#[derive(Debug)]
pub enum ProfileViewMsg {
    Refresh,
    SaveCurrent,
    LoadProfile(String),
    DeleteProfile(String),
    ConfirmDelete(String),
    UpdateCurrentState {
        theme_id: Option<String>,
        wallpapers: std::collections::HashMap<String, std::path::PathBuf>,
        binding_mode: BindingMode,
    },
    SaveDialogResponse(String),  // profile name from dialog
}

#[derive(Debug)]
pub enum ProfileViewOutput {
    ShowToast(String),
    LoadProfile(UnifiedProfile),
    ProfileDeleted(String),
}

pub struct ProfileViewModel {
    profiles: FactoryVecDeque<ProfileItem>,
    current_state: Option<CurrentAppState>,
    active_profile: Option<String>,
}

#[derive(Debug, Clone)]
struct CurrentAppState {
    theme_id: Option<String>,
    wallpapers: std::collections::HashMap<String, std::path::PathBuf>,
    binding_mode: BindingMode,
}

#[relm4::component(pub)]
impl SimpleComponent for ProfileViewModel {
    type Init = ();
    type Input = ProfileViewMsg;
    type Output = ProfileViewOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 12,

            // Header with Save button
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,

                gtk::Label {
                    set_markup: "<b>Saved Profiles</b>",
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,
                },

                gtk::Button {
                    set_icon_name: "list-add-symbolic",
                    set_label: "Save Current",
                    set_tooltip_text: Some("Save current appearance as profile"),
                    connect_clicked => ProfileViewMsg::SaveCurrent,
                },

                gtk::Button {
                    set_icon_name: "view-refresh-symbolic",
                    set_tooltip_text: Some("Refresh profiles"),
                    connect_clicked => ProfileViewMsg::Refresh,
                },
            },

            // Profiles grid
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_hscrollbar_policy: gtk::PolicyType::Never,

                #[local_ref]
                profiles_box -> gtk::FlowBox {
                    set_selection_mode: gtk::SelectionMode::None,
                    set_homogeneous: true,
                    set_min_children_per_line: 2,
                    set_max_children_per_line: 4,
                    set_row_spacing: 8,
                    set_column_spacing: 8,
                },
            },

            // Empty state
            #[name = "empty_state"]
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 8,
                set_valign: gtk::Align::Center,
                set_vexpand: true,

                gtk::Image {
                    set_icon_name: Some("folder-symbolic"),
                    set_pixel_size: 64,
                    add_css_class: "dim-label",
                },

                gtk::Label {
                    set_label: "No saved profiles",
                    add_css_class: "dim-label",
                },

                gtk::Label {
                    set_label: "Save your current appearance to create one",
                    add_css_class: "dim-label",
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let profiles = FactoryVecDeque::builder()
            .launch(gtk::FlowBox::new())
            .forward(sender.input_sender(), |msg| {
                match msg {
                    ProfileCardOutput::Load(name) => ProfileViewMsg::LoadProfile(name),
                    ProfileCardOutput::Delete(name) => ProfileViewMsg::DeleteProfile(name),
                }
            });

        let model = ProfileViewModel {
            profiles,
            current_state: None,
            active_profile: None,
        };

        let profiles_box = model.profiles.widget();
        let widgets = view_output!();

        // Initial load
        sender.input(ProfileViewMsg::Refresh);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ProfileViewMsg::Refresh => {
                self.load_profiles();
            }

            ProfileViewMsg::SaveCurrent => {
                self.show_save_dialog(sender.clone());
            }

            ProfileViewMsg::SaveDialogResponse(name) => {
                if let Some(ref state) = self.current_state {
                    let profile = UnifiedProfile {
                        name: name.clone(),
                        description: String::new(),
                        theme_id: state.theme_id.clone(),
                        monitor_wallpapers: state.wallpapers.clone(),
                        binding_mode: state.binding_mode.clone(),
                    };

                    match profile_storage::save_unified_profile(&profile) {
                        Ok(_) => {
                            sender.output(ProfileViewOutput::ShowToast(
                                format!("Saved profile: {}", name)
                            )).ok();
                            self.active_profile = Some(name);
                            self.load_profiles();
                        }
                        Err(e) => {
                            sender.output(ProfileViewOutput::ShowToast(
                                format!("Failed to save: {}", e)
                            )).ok();
                        }
                    }
                } else {
                    sender.output(ProfileViewOutput::ShowToast(
                        "No current state to save".to_string()
                    )).ok();
                }
            }

            ProfileViewMsg::LoadProfile(name) => {
                match profile_storage::load_unified_profile(&name) {
                    Ok(profile) => {
                        self.active_profile = Some(name);
                        self.load_profiles();  // Refresh to show active badge
                        sender.output(ProfileViewOutput::LoadProfile(profile)).ok();
                    }
                    Err(e) => {
                        sender.output(ProfileViewOutput::ShowToast(
                            format!("Failed to load: {}", e)
                        )).ok();
                    }
                }
            }

            ProfileViewMsg::DeleteProfile(name) => {
                // Show confirmation dialog
                sender.input(ProfileViewMsg::ConfirmDelete(name));
            }

            ProfileViewMsg::ConfirmDelete(name) => {
                match profile_storage::delete_unified_profile(&name) {
                    Ok(_) => {
                        if self.active_profile.as_ref() == Some(&name) {
                            self.active_profile = None;
                        }
                        self.load_profiles();
                        sender.output(ProfileViewOutput::ProfileDeleted(name)).ok();
                    }
                    Err(e) => {
                        sender.output(ProfileViewOutput::ShowToast(
                            format!("Failed to delete: {}", e)
                        )).ok();
                    }
                }
            }

            ProfileViewMsg::UpdateCurrentState { theme_id, wallpapers, binding_mode } => {
                self.current_state = Some(CurrentAppState {
                    theme_id,
                    wallpapers,
                    binding_mode,
                });
            }
        }
    }
}

impl ProfileViewModel {
    fn load_profiles(&mut self) {
        let mut guard = self.profiles.guard();
        guard.clear();

        if let Ok(names) = profile_storage::list_unified_profiles() {
            for name in names {
                if let Ok(profile) = profile_storage::load_unified_profile(&name) {
                    let is_active = self.active_profile.as_ref() == Some(&name);
                    guard.push_back((profile, is_active));
                }
            }
        }
    }

    fn show_save_dialog(&self, sender: ComponentSender<Self>) {
        // Simple dialog to get profile name
        // Auto-suggest based on theme name if available
        let suggested_name = self.current_state
            .as_ref()
            .and_then(|s| s.theme_id.clone())
            .unwrap_or_else(|| "My Profile".to_string());

        // Create simple entry dialog
        let dialog = gtk::Dialog::builder()
            .title("Save Profile")
            .modal(true)
            .build();

        let content = dialog.content_area();
        content.set_margin_all(16);
        content.set_spacing(12);

        let label = gtk::Label::new(Some("Profile name:"));
        label.set_halign(gtk::Align::Start);
        content.append(&label);

        let entry = gtk::Entry::new();
        entry.set_text(&suggested_name);
        entry.set_activates_default(true);
        content.append(&entry);

        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
        dialog.add_button("Save", gtk::ResponseType::Accept);

        let entry_clone = entry.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let name = entry_clone.text().to_string().trim().to_string();
                if !name.is_empty() {
                    sender.input(ProfileViewMsg::SaveDialogResponse(name));
                }
            }
            dialog.close();
        });

        dialog.present();
    }
}
