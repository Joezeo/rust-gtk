mod window;

use gtk::Application;
use gtk::prelude::*;
use gtk::gio;
use window::Window;

pub const APP_ID: &str = "com.toocol.actions";

fn main() {
    gio::resources_register_include!("action_composite_templates.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    Window::new(app).present();
}