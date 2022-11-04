#![cfg_attr(not(debug_assertions), not(debug_flags), windows_subsystem = "windows")]
mod window;
mod task_object;
mod task_row;
mod util;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use window::Window;

pub const APP_ID: &str = "com.toocol.todo";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    log4rs::init_file("todo/resources/log4rs.yaml", Default::default()).unwrap();

    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Connect signal
    app.connect_startup(setup_shortcurs);
    app.connect_activate(build_ui);

    app.run_with_args(&["-Ddebug=false"]);
}

fn build_ui(app: &Application) {
    Window::new(app).present();
}

fn setup_shortcurs(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}
