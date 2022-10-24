#![cfg_attr(not(debug_assertions), not(debug_flags), windows_subsystem = "windows")]
mod window;
mod task_object;
mod task_row;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use window::Window;

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");
    std::env::set_var("INSPECTOR_ENABLE", "0");

    gio::resources_register_include!("todo.gresource").expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.toocol.todo")
        .build();

    app.connect_activate(build_ui);

    app.run_with_args(&["-Ddebug=false"]);
}

fn build_ui(app: &Application) {
    Window::new(app).present();
}
