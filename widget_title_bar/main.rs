mod title_bar;

use gtk::{prelude::*, Application, ApplicationWindow, DrawingArea, CssProvider, gdk::Display, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
use title_bar::WidgetTitleBar;

const APP_ID: &str = "com.test.wiget_title_bar";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));

        if let Some(display) = &Display::default() {
            StyleContext::add_provider_for_display(
                display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    });

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let child = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .build();
    child.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0.5, 0.5, 0.5);
        cr.paint().unwrap();
    });

    let title_bar = WidgetTitleBar::new();

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&title_bar);
    gtk_box.append(&child);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Widget title bar")
        .child(&gtk_box)
        .build();
    window.present();
}
