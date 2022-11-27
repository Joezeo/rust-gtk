use gtk::{prelude::*, Application, ApplicationWindow};

const APP_ID: &str = "com.command_panel";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Command Panel")
        .build();
    window.present();
}
