use gtk::prelude::*;
use gtk::gio;
use relm4::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::models::Monitor;
use crate::services::image_splitter;

#[derive(Debug)]
pub enum SplitDialogInput {
    /// Open file chooser
    SelectImage,
    /// File was selected
    ImageSelected(PathBuf),
    /// User entered a name
    SetName(String),
    /// Perform the split
    DoSplit,
    /// Cancel the dialog
    Cancel,
}

#[derive(Debug)]
pub enum SplitDialogOutput {
    /// Wallpapers generated - map of monitor -> path
    Generated(HashMap<String, PathBuf>),
    /// User cancelled
    Cancelled,
    /// Error occurred
    Error(String),
}

pub struct SplitDialogModel {
    monitors: Vec<Monitor>,
    source_path: Option<PathBuf>,
    wallpaper_name: String,
    is_splitting: bool,
}

#[relm4::component(pub)]
impl SimpleComponent for SplitDialogModel {
    type Init = Vec<Monitor>;
    type Input = SplitDialogInput;
    type Output = SplitDialogOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 12,
            set_margin_all: 12,
            set_width_request: 400,

            // Title
            gtk::Label {
                set_markup: "<b>Import Panoramic Wallpaper</b>",
                set_halign: gtk::Align::Start,
            },

            // Description
            gtk::Label {
                set_text: "Select a wide image to split across your monitors.",
                set_wrap: true,
                set_halign: gtk::Align::Start,
            },

            // File selection row
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,

                gtk::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("No image selected"),
                    set_editable: false,
                    #[watch]
                    set_text: model.source_path.as_ref()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .unwrap_or(""),
                },

                gtk::Button {
                    set_label: "Browse...",
                    connect_clicked => SplitDialogInput::SelectImage,
                },
            },

            // Name entry
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,

                gtk::Label {
                    set_text: "Name:",
                    set_width_request: 60,
                },

                #[name = "name_entry"]
                gtk::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("e.g., volcanic-forge"),
                    #[watch]
                    set_text: &model.wallpaper_name,
                },
            },

            // Monitor preview info
            gtk::Label {
                set_markup: &format!(
                    "<small>Will generate wallpapers for {} monitors:\n{}</small>",
                    model.monitors.len(),
                    model.monitors.iter()
                        .map(|m| format!("{} ({}x{})", m.name, m.width, m.height))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                set_wrap: true,
                set_halign: gtk::Align::Start,
            },

            // Buttons
            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,
                set_halign: gtk::Align::End,
                set_margin_top: 12,

                gtk::Button {
                    set_label: "Cancel",
                    connect_clicked => SplitDialogInput::Cancel,
                },

                gtk::Button {
                    set_label: if model.is_splitting { "Splitting..." } else { "Split & Apply" },
                    add_css_class: "suggested-action",
                    #[watch]
                    set_sensitive: model.source_path.is_some()
                        && !model.wallpaper_name.is_empty()
                        && !model.is_splitting,
                    connect_clicked => SplitDialogInput::DoSplit,
                },
            },
        }
    }

    fn init(
        monitors: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SplitDialogModel {
            monitors,
            source_path: None,
            wallpaper_name: "split-wallpaper".to_string(),
            is_splitting: false,
        };

        let widgets = view_output!();

        // Connect name entry changes
        {
            let sender = sender.clone();
            widgets.name_entry.connect_changed(move |entry| {
                sender.input(SplitDialogInput::SetName(entry.text().to_string()));
            });
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            SplitDialogInput::SelectImage => {
                // Use native file chooser
                let dialog = gtk::FileDialog::builder()
                    .title("Select Panoramic Image")
                    .build();

                // Add image filter
                let filter = gtk::FileFilter::new();
                filter.set_name(Some("Images"));
                filter.add_mime_type("image/*");
                let filters = gio::ListStore::new::<gtk::FileFilter>();
                filters.append(&filter);
                dialog.set_filters(Some(&filters));

                // Open async
                let sender_clone = sender.clone();
                dialog.open(
                    None::<&gtk::Window>,
                    None::<&gio::Cancellable>,
                    move |result| {
                        if let Ok(file) = result {
                            if let Some(path) = file.path() {
                                sender_clone.input(SplitDialogInput::ImageSelected(path));
                            }
                        }
                    },
                );
            }

            SplitDialogInput::ImageSelected(path) => {
                // Auto-generate name from filename if not set
                if self.wallpaper_name == "split-wallpaper" {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        self.wallpaper_name = stem.to_string();
                    }
                }
                self.source_path = Some(path);
            }

            SplitDialogInput::SetName(name) => {
                self.wallpaper_name = name;
            }

            SplitDialogInput::DoSplit => {
                if let Some(source) = &self.source_path {
                    self.is_splitting = true;

                    let output_dir = image_splitter::default_split_output_dir()
                        .join(&self.wallpaper_name);

                    match image_splitter::split_panoramic(
                        source,
                        &self.monitors,
                        &output_dir,
                        &self.wallpaper_name,
                    ) {
                        Ok(result) => {
                            let wallpapers: HashMap<String, PathBuf> = result.wallpapers
                                .into_iter()
                                .collect();
                            let _ = sender.output(SplitDialogOutput::Generated(wallpapers));
                        }
                        Err(e) => {
                            let _ = sender.output(SplitDialogOutput::Error(e.to_string()));
                        }
                    }

                    self.is_splitting = false;
                }
            }

            SplitDialogInput::Cancel => {
                let _ = sender.output(SplitDialogOutput::Cancelled);
            }
        }
    }
}
