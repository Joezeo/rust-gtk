use std::f64::consts::PI;

use gtk::{prelude::*, Application, ApplicationWindow, DrawingArea};

fn main() {
    let app = Application::builder()
        .application_id("com.transition_seperator")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(300)
        .content_width(200)
        .build();
    drawing_area.set_draw_func(|_, cr, width, height| {
        let width = width as f64;
        let height = height as f64;

        let single = 150.;
        let spined = 1.5;
        let margin = 15.;
        let ver = (height - 1.) / 2.;

        // Background
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        cr.paint().unwrap();

        cr.set_source_rgba(0.5, 0.5, 0.5, 0.8);
        cr.set_line_width(0.5);

        let start = (width - (single * 2. + margin * 2. + spined)) / 2.;

        cr.move_to(start, ver);
        cr.line_to(start + single, ver);
        cr.stroke().unwrap();

        cr.move_to(start + single + margin * 2. + spined, ver);
        cr.line_to(width - start, ver);
        cr.stroke().unwrap();

        cr.set_source_rgba(0.5, 0.5, 0.5, 0.5);
        cr.arc(width / 2., ver, spined, -PI, PI);
        cr.fill().unwrap();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Transition seperator")
        .child(&drawing_area)
        .build();

    window.present();
}
