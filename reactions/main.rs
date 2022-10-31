use gtk::{prelude::*, Application, ApplicationWindow, Inhibit};

fn main() {
    let app = Application::builder()
        .application_id("com.test.reactions")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let controller = gtk::EventControllerKey::new();
    controller.connect_key_pressed(move |controller, key, keycode, _modfier| {
        if let Some(_) = controller.im_context() {
            return Inhibit(false);
        }
        println!("Terminal -> name: {:#?}, code: {}", key.name(), keycode);
        Inhibit(false)
    });

    let window = ApplicationWindow::builder().application(app).build();
    window.add_controller(&controller);

    window.present();
}
