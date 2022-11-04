#![cfg_attr(not(debug_assertions), not(debug_flags), windows_subsystem = "windows")]
mod collection_object;
mod task_object;
mod task_row;
mod util;
mod window;

use gtk::{
    gdk::Display, gio, prelude::*, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use window::Window;

pub const APP_ID: &str = "com.toocol.todo";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    log4rs::init_file("todo/resources/log4rs.yaml", Default::default()).unwrap();

    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect signal
    app.connect_startup(|app| {
        load_css();
        setup_shortcurs(app);
    });
    app.connect_activate(build_ui);

    app.run_with_args(&["-Ddebug=false"]);
}

fn build_ui(app: &adw::Application) {
    Window::new(app).present();
}

fn setup_shortcurs(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("resources/style.css"));

    if let Some(display) = &Display::default() {
        StyleContext::add_provider_for_display(
            display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
