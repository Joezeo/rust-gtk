use gtk::{prelude::*, Application, ApplicationWindow, GestureClick};

const APP_ID: &str = "com.gesture_click";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gesture Click")
        .build();

    let gesture_click = GestureClick::new();
    gesture_click.set_button(3);
    gesture_click.connect_released(|_, _, x, y| {
        println!("Press: {} {}", x, y);
    });
    window.add_controller(&gesture_click);

    window.present();
}
