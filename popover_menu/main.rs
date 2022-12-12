use gtk::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, DrawingArea, PopoverMenu, GestureClick,
};

const APP_ID: &str = "com.popover_menu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(400)
        .content_height(250)
        .focusable(true)
        .can_focus(true)
        .build();
    drawing_area.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0., 0., 0.);
        cr.paint().unwrap();
    });

    let menu = PopoverMenu::builder().build();

    let gesture_click = GestureClick::new();
    gesture_click.set_button(3);
    gesture_click.connect_pressed(clone!(@weak menu => move |_, _, x, y| {
        println!("Press: {} {}", x, y);
    }));
    drawing_area.add_controller(&gesture_click);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Popover Menu")
        .default_width(400)
        .default_height(250)
        .child(&drawing_area)
        .build();
    window.present();
}
