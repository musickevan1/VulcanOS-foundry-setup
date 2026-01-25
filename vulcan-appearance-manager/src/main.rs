mod app;
mod brand_css;
mod components;
mod models;
mod services;
mod state;

use relm4::RelmApp;
use app::App;

fn main() {
    let app = RelmApp::new("com.vulcanos.appearance-manager");

    // Load Vulcan brand CSS globally from shared module
    relm4::set_global_css(brand_css::FULL_CSS);

    app.run::<App>(());
}
