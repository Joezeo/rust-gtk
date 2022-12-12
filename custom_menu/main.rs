mod menu;

use gtk::{
    glib, glib::closure_local, prelude::*, Allocation, Application, ApplicationWindow, DrawingArea,
    Overlay, Widget,
};
use menu::CustomMenu;

const APP_ID: &str = "com.custom_menu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let dw = DrawingArea::builder()
        .content_width(1280)
        .content_height(800)
        .build();
    dw.set_draw_func(|_, cr, _, _| {
        let color = (15., 15., 15.);
        cr.set_source_rgb(color.0 / 255., color.1 / 255., color.2 / 255.);
        cr.paint().unwrap();
    });

    let menu = CustomMenu::new();
    menu.set_visible(false);

    let overlay = Overlay::builder().child(&dw).build();
    overlay.connect_closure(
        "get-child-position",
        false,
        closure_local!(move |_: Overlay, widget: Widget, _allocation: &Allocation| {
            println!("Get custom menu position, {}", widget.type_().name());
            false
        }),
    );
    overlay.add_overlay(&menu);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Menu")
        .default_width(1280)
        .default_height(800)
        .child(&overlay)
        .build();
    window.present();
}
