mod menu;
mod custom_menu_model;

use gtk::{
    prelude::*, Application, ApplicationWindow, DrawingArea,
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
    menu.set_parent(&dw);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Custom Menu")
        .default_width(1280)
        .default_height(800)
        .child(&dw)
        .build();
    window.present();
}
