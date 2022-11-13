use gtk::{
    gdk::Display, prelude::*, Align, Application, ApplicationWindow, CssProvider, DrawingArea,
    Overlay, ScrolledWindow, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

const APP_ID: &str = "com.test.shadow_border";

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
        };
    });

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(600)
        .content_height(400)
        .build();
    drawing_area.set_draw_func(|_, cr, _, _| {
        cr.paint().unwrap();
    });

    let drawing_area_overlay_left_bottom = DrawingArea::builder()
        .content_width(180)
        .content_height(120)
        .halign(Align::Start)
        .valign(Align::End)
        .build();
    drawing_area_overlay_left_bottom.add_css_class("overlay-node");
    drawing_area_overlay_left_bottom.set_draw_func(|_, cr, width, height| {
        cr.set_source_rgb(0.1, 0.1, 0.1);
        cr.rectangle(0., 0., width as f64, height as f64);
        cr.fill().unwrap();
        cr.paint().unwrap();
    });

    let overlay = Overlay::builder().child(&drawing_area).build();
    overlay.add_overlay(&drawing_area_overlay_left_bottom);

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(150)
        .min_content_height(100)
        .hscrollbar_policy(gtk::PolicyType::External)
        .vscrollbar_policy(gtk::PolicyType::External)
        .child(&overlay)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .child(&scrolled_window)
        .default_width(600)
        .default_height(450)
        .title("Overlay App")
        .build();

    window.present();
}
