mod window;
mod custom_button;

use gtk::gio;
use gtk::prelude::*;
use gtk::Application;
use window::Window;

const APP_ID: &str = "com.toocol.composite_templates";

fn main() {
    gio::resources_register_include!("composite_templates.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
