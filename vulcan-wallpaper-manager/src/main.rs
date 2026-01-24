mod app;
mod components;
mod models;
mod services;

use relm4::RelmApp;
use app::App;

fn main() {
    let app = RelmApp::new("com.vulcanos.wallpaper-manager");
    app.run::<App>(());
}
