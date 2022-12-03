mod panel;
use gtk::{
    prelude::*, Align, Application, ApplicationWindow, DrawingArea, Overlay, ScrolledWindow,
    glib, glib::clone, Revealer
};
use panel::CommandPanel;

const APP_ID: &str = "com.command_panel";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(1280)
        .content_height(800)
        .build();
    drawing_area.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0., 0., 0.);
        cr.paint().unwrap();
    });

    let scrolled_window = ScrolledWindow::builder()
        .max_content_width(1280)
        .max_content_height(800)
        .min_content_width(1280)
        .min_content_height(800)
        .child(&drawing_area)
        .build();

    let overlay = Overlay::builder()
        .child(&scrolled_window)
        .vexpand(false)
        .build();

    let command_panel = CommandPanel::new();
    let reveal = Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideDown)
        .transition_duration(500)
        .reveal_child(true)
        .child(&command_panel)
        .build();
    overlay.add_overlay(&reveal);

    let button = gtk::Button::builder()
        .label("Toggle Command Panel")
        .halign(Align::Start)
        .valign(Align::Start)
        .margin_top(50)
        .margin_start(10)
        .height_request(25)
        .vexpand(false)
        .hexpand(false)
        .build();
    overlay.add_overlay(&button);

    button.connect_clicked(clone!(@weak reveal => move |_| {
        reveal.set_reveal_child(!reveal.reveals_child())
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Command Panel")
        .child(&overlay)
        .build();
    window.present();
}
