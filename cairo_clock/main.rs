#![allow(non_snake_case)]
use std::{f64::consts::PI, time::Duration};
use time::*;

use gtk::{
    cairo::Context, glib::timeout_add_local, prelude::*, Application, ApplicationWindow,
    DrawingArea,
};

fn main() {
    let app = Application::builder()
        .application_id("com.transition_seperator")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder().build();
    drawing_area.set_draw_func(|_, cr, width, height| {
        let now = now();
        let sec = now.tm_sec as f64;
        let minute = now.tm_min as f64;
        let hour = (now.tm_hour % 12) as f64;

        let hor: f64;
        let ver: f64;
        let meter_radius: f64;
        let mut needle_length: f64;
        let mut needle_position: f64;

        hor = width as f64 / 2.;
        ver = height as f64 / 2.;

        cr.set_source_rgb(0., 0., 0.);
        cr.paint().unwrap();

        /*
                cr.set_source_rgb(1., 0., 1.);
                cr.arc(hor, ver, 50., -PI, 0.);
                cr.fill().unwrap();
                cr.stroke().unwrap();

                cr.set_source_rgb(1., 1., 1.);
                cr.arc(hor, ver, 40., -PI, PI);
                cr.fill().unwrap();
                cr.stroke().unwrap();
        */

        meter_radius = 60.;

        // Hour
        needle_length = 35.;
        needle_position = hour + 3. + minute / 60.;
        needle_position = needle_position * (PI / 6.);
        needles(
            cr,
            hor,
            ver,
            needle_length,
            needle_position,
            meter_radius,
            3,
        );

        // Minute 
        needle_length = 45.;
        needle_position = minute + 15.;
        needle_position = needle_position * (PI / 30.);
        needles(
            cr,
            hor,
            ver,
            needle_length,
            needle_position,
            meter_radius,
            2,
        );

        // Second
        needle_length = 60.;
        needle_position = sec + 15.;
        needle_position = needle_position * (PI / 30.);
        needles(
            cr,
            hor,
            ver,
            needle_length,
            needle_position,
            meter_radius,
            1,
        );

        // Show digit text of time
        cr.set_source_rgb(1., 1., 1.);
        cr.move_to(hor - 60., ver + 90.);
        cr.show_text(now.asctime().to_string().as_str()).unwrap();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Spark Gap Radio Clock")
        .child(&drawing_area)
        .build();

    timeout_add_local(Duration::from_secs(1), move || {
        drawing_area.queue_draw();
        Continue(true)
    });

    window.present();
}

pub fn needles(
    cr: &Context,
    hor: f64,
    ver: f64,
    needle_length: f64,
    needle_position: f64,
    meter_radius: f64,
    c_flg: i32,
) {
    let mut X = 0.0;
    let mut x: f64;
    let mut x1: f64;
    let mut y: f64;
    let mut y1: f64;
    cr.set_source_rgb(0.5, 0.5, 1.0); // blue
    cr.set_line_width(2.5);

    for i in 0..60 {
        if i % 5 == 0 {
            // Draw long length hash mark
            cr.arc(hor, ver, meter_radius + 10., -PI, -PI + X);
            // Get current point position to x,y
            (x, y) = cr.current_point().unwrap();

            // Draw short length hash mark
            cr.arc(hor, ver, meter_radius, -PI, -PI + X);
            // Get current point position to x1,y1
            (x1, y1) = cr.current_point().unwrap();

            // Draw hash mark from (x,y) to (x1,y1)
            cr.new_path();
            cr.move_to(x, y);
            cr.line_to(x1, y1);

            cr.stroke().unwrap();
        }
        X = X + PI / 30.;
    }

    X = PI / 2.;

    // Pick a color
    match c_flg {
        1 => cr.set_source_rgb(1., 1., 0.3),  // yellow
        2 => cr.set_source_rgb(1., 0., 0.),   // red
        3 => cr.set_source_rgb(0.5, 1., 0.5), // green
        _ => unimplemented!(),
    }

    if c_flg == 1 {
        for i in 0..60 {
            if i % 5 != 0 {
                if X > needle_position {
                    break;
                }
                // Draw long length hash mark
                cr.arc(hor, ver, meter_radius + 2., -PI, -PI + X);
                // Get current point position to x,y
                (x, y) = cr.current_point().unwrap();

                // Draw short length hash mark
                cr.arc(hor, ver, meter_radius - 1., -PI, -PI + X);
                // Get current point position to x1,y1
                (x1, y1) = cr.current_point().unwrap();

                // Draw hash mark from (x,y) to (x1,y1)
                cr.new_path();
                cr.move_to(x, y);
                cr.line_to(x1, y1);

                cr.stroke().unwrap();
            }
            X = X + PI / 30.;
        }
    }

    // Draw needle
    cr.arc(hor, ver, needle_length, -PI, -PI + needle_position);
    (x, y) = cr.current_point().unwrap();
    cr.new_path();
    cr.move_to(x, y);
    cr.line_to(hor, ver);
    cr.stroke().unwrap();

    // Draw spindle
    cr.arc(hor, ver, 4., -PI, PI);
    cr.fill().unwrap();
}
