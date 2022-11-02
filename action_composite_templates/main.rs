mod window;

use gtk::Application;
use gtk::prelude::*;
use gtk::gio;
use window::Window;

fn main() {
    gio::resources_register_include!("action_composite_templates.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id("com.toocol.action.tempalte").build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    Window::new(app).present();
}