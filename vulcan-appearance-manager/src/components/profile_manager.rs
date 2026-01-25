use gtk::prelude::*;
use gtk::glib::clone;
use relm4::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::services::profile_storage::{self, WallpaperProfile, KNOWN_PROFILES};

#[derive(Debug)]
pub enum ProfileManagerInput {
    /// Update with current wallpaper assignments
    UpdateWallpapers(HashMap<String, PathBuf>),
    /// Trigger save dialog
    SaveProfile,
    /// Load selected profile
    LoadProfile,
    /// Delete selected profile
    DeleteProfile,
    /// Profile selected from dropdown
    ProfileSelected(String),
    /// Refresh profile list
    Refresh,
}

#[derive(Debug)]
pub enum ProfileManagerOutput {
    /// Profile loaded - apply these wallpapers
    ApplyProfile(HashMap<String, PathBuf>),
    /// Profile saved successfully
    ProfileSaved(String),
    /// Error occurred
    Error(String),
}

pub struct ProfileManagerModel {
    profiles: Vec<String>,
    selected_profile: Option<String>,
    current_wallpapers: HashMap<String, PathBuf>,
    /// Flag to prevent infinite loops when updating dropdown programmatically
    updating_dropdown: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for ProfileManagerModel {
    type Init = ();
    type Input = ProfileManagerInput;
    type Output = ProfileManagerOutput;

    view! {
        #[name = "profile_box"]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 8,
            set_margin_all: 4,

            // Profile dropdown - use ComboBoxText for simpler handling
            #[name = "profile_combo"]
            gtk::ComboBoxText {
                set_tooltip_text: Some("Select profile"),
                set_hexpand: false,

                connect_changed[sender] => move |combo| {
                    if let Some(text) = combo.active_text() {
                        sender.input(ProfileManagerInput::ProfileSelected(text.to_string()));
                    }
                },
            },

            // Load button
            gtk::Button {
                set_icon_name: "document-open-symbolic",
                set_tooltip_text: Some("Load profile"),
                #[watch]
                set_sensitive: model.selected_profile.is_some(),
                connect_clicked => ProfileManagerInput::LoadProfile,
            },

            // Save button
            gtk::Button {
                set_icon_name: "document-save-symbolic",
                set_tooltip_text: Some("Save current as profile"),
                connect_clicked => ProfileManagerInput::SaveProfile,
            },

            // Delete button
            gtk::Button {
                set_icon_name: "user-trash-symbolic",
                set_tooltip_text: Some("Delete profile"),
                #[watch]
                set_sensitive: model.selected_profile.is_some() && !KNOWN_PROFILES.contains(&model.selected_profile.as_deref().unwrap_or("")),
                connect_clicked => ProfileManagerInput::DeleteProfile,
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Ensure known profiles exist as files
        profile_storage::ensure_known_profiles();

        let profiles = profile_storage::list_profiles().unwrap_or_default();
        let detected = profile_storage::detect_current_profile();

        let model = ProfileManagerModel {
            profiles: profiles.clone(),
            selected_profile: detected.clone(),
            current_wallpapers: HashMap::new(),
            updating_dropdown: false,
        };

        let widgets = view_output!();

        // Populate combo box
        for profile in &profiles {
            widgets.profile_combo.append_text(profile);
        }

        // Set active item if detected
        if let Some(ref detected_name) = detected {
            if let Some(idx) = profiles.iter().position(|p| p == detected_name) {
                widgets.profile_combo.set_active(Some(idx as u32));
            }
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ProfileManagerInput::UpdateWallpapers(wallpapers) => {
                self.current_wallpapers = wallpapers;
            }

            ProfileManagerInput::ProfileSelected(name) => {
                self.selected_profile = Some(name.clone());
                // Auto-load the selected profile
                match profile_storage::load_profile(&name) {
                    Ok(profile) => {
                        let _ = sender.output(ProfileManagerOutput::ApplyProfile(
                            profile.monitor_wallpapers
                        ));
                        println!("Auto-loaded profile: {}", name);
                    }
                    Err(e) => {
                        eprintln!("Failed to load profile {}: {}", name, e);
                    }
                }
            }

            ProfileManagerInput::SaveProfile => {
                // Get profile name from selected or prompt
                let name = self.selected_profile.clone()
                    .unwrap_or_else(|| "custom".to_string());

                let profile = WallpaperProfile::with_wallpapers(
                    name.clone(),
                    self.current_wallpapers.clone(),
                );

                match profile_storage::save_profile(&profile) {
                    Ok(_) => {
                        let _ = sender.output(ProfileManagerOutput::ProfileSaved(name.clone()));
                        // Refresh profile list
                        self.profiles = profile_storage::list_profiles().unwrap_or_default();
                        println!("Saved profile: {}", name);
                    }
                    Err(e) => {
                        let _ = sender.output(ProfileManagerOutput::Error(e.to_string()));
                    }
                }
            }

            ProfileManagerInput::LoadProfile => {
                if let Some(name) = &self.selected_profile {
                    match profile_storage::load_profile(name) {
                        Ok(profile) => {
                            let _ = sender.output(ProfileManagerOutput::ApplyProfile(
                                profile.monitor_wallpapers
                            ));
                            println!("Loaded profile: {}", name);
                        }
                        Err(e) => {
                            let _ = sender.output(ProfileManagerOutput::Error(e.to_string()));
                        }
                    }
                }
            }

            ProfileManagerInput::DeleteProfile => {
                if let Some(name) = self.selected_profile.clone() {
                    // Don't allow deleting known profiles
                    if KNOWN_PROFILES.contains(&name.as_str()) {
                        let _ = sender.output(ProfileManagerOutput::Error(
                            format!("Cannot delete built-in profile: {}", name)
                        ));
                        return;
                    }

                    match profile_storage::delete_profile(&name) {
                        Ok(_) => {
                            self.profiles = profile_storage::list_profiles().unwrap_or_default();
                            self.selected_profile = None;
                            println!("Deleted profile: {}", name);
                        }
                        Err(e) => {
                            let _ = sender.output(ProfileManagerOutput::Error(e.to_string()));
                        }
                    }
                }
            }

            ProfileManagerInput::Refresh => {
                self.profiles = profile_storage::list_profiles().unwrap_or_default();
            }
        }
    }
}
