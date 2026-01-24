use gtk::prelude::*;
use gtk::glib;
use relm4::prelude::*;
use std::path::PathBuf;

use crate::services::thumbnail;

const THUMBNAIL_SIZE: i32 = 150;
const GRID_COLUMNS: u32 = 4;

#[derive(Debug)]
pub enum WallpaperPickerInput {
    LoadDirectory(PathBuf),
    Refresh,
}

#[derive(Debug)]
pub enum WallpaperPickerOutput {
    Selected(PathBuf),
}

pub struct WallpaperPickerModel {
    wallpapers: Vec<PathBuf>,
    current_dir: PathBuf,
}

#[relm4::component(pub)]
impl SimpleComponent for WallpaperPickerModel {
    type Init = PathBuf;
    type Input = WallpaperPickerInput;
    type Output = WallpaperPickerOutput;

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_vscrollbar_policy: gtk::PolicyType::Automatic,
            set_min_content_height: 300,
            set_hexpand: true,
            set_vexpand: true,

            gtk::FlowBox {
                set_homogeneous: true,
                set_max_children_per_line: GRID_COLUMNS,
                set_min_children_per_line: 2,
                set_selection_mode: gtk::SelectionMode::Single,
                set_row_spacing: 8,
                set_column_spacing: 8,

                connect_child_activated[sender] => move |_, child| {
                    if let Some(path) = get_wallpaper_path(child) {
                        let _ = sender.output(WallpaperPickerOutput::Selected(path));
                    }
                },

                #[iterate]
                append: model.wallpapers.iter().map(|path| {
                    create_wallpaper_item(path)
                }).collect::<Vec<_>>().as_slice(),
            },
        }
    }

    fn init(
        directory: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let wallpapers = thumbnail::scan_wallpaper_directory(&directory)
            .unwrap_or_default();

        let model = WallpaperPickerModel {
            wallpapers,
            current_dir: directory,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            WallpaperPickerInput::LoadDirectory(dir) => {
                self.current_dir = dir.clone();
                self.wallpapers = thumbnail::scan_wallpaper_directory(&dir)
                    .unwrap_or_default();
            }
            WallpaperPickerInput::Refresh => {
                self.wallpapers = thumbnail::scan_wallpaper_directory(&self.current_dir)
                    .unwrap_or_default();
            }
        }
    }
}

/// Create a wallpaper thumbnail item for the grid
fn create_wallpaper_item(path: &PathBuf) -> gtk::FlowBoxChild {
    let child = gtk::FlowBoxChild::new();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(4)
        .build();

    // Create frame for thumbnail
    let frame = gtk::Frame::builder()
        .width_request(THUMBNAIL_SIZE)
        .height_request(THUMBNAIL_SIZE)
        .build();

    // Try to load thumbnail
    let picture = if let Some(thumb) = thumbnail::get_cached_thumbnail(path) {
        gtk::Picture::for_filename(&thumb)
    } else {
        // Generate thumbnail in background (for now, synchronous)
        match thumbnail::generate_thumbnail(path) {
            Ok(thumb) => gtk::Picture::for_filename(&thumb),
            Err(_) => {
                // Fallback to icon
                let icon = gtk::Image::from_icon_name("image-missing");
                icon.set_pixel_size(THUMBNAIL_SIZE);
                frame.set_child(Some(&icon));

                // Still create a label
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("?");
                let label = gtk::Label::builder()
                    .label(name)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .max_width_chars(15)
                    .build();

                vbox.append(&frame);
                vbox.append(&label);
                child.set_child(Some(&vbox));

                // Store path for later retrieval
                child.set_widget_name(&path.to_string_lossy());

                return child;
            }
        }
    };

    picture.set_content_fit(gtk::ContentFit::Cover);
    frame.set_child(Some(&picture));

    // Filename label
    let name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("?");
    let label = gtk::Label::builder()
        .label(name)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .max_width_chars(15)
        .build();

    vbox.append(&frame);
    vbox.append(&label);
    child.set_child(Some(&vbox));

    // Store path for later retrieval
    child.set_widget_name(&path.to_string_lossy());

    child
}

/// Extract wallpaper path from FlowBoxChild widget name
fn get_wallpaper_path(child: &gtk::FlowBoxChild) -> Option<PathBuf> {
    let name = child.widget_name();
    if !name.is_empty() {
        Some(PathBuf::from(name.as_str()))
    } else {
        None
    }
}
