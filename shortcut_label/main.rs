use std::rc::Rc;
use std::{cell::RefCell, f64::consts::PI};

use gtk::{prelude::*, Application, ApplicationWindow, DrawingArea};

const APP_ID: &str = "com.shortcut_label";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let shortcut_label = DrawingArea::builder().build();
    let shortcuts = ["Ctrl", "Shift", "D"];
    let font_family = "Consolas";
    let font_size = 10;
    let hor_inset = 5.;
    let ver_inset = 3.;
    let join_inset = 2.;
    let radius = 5.;

    let layouts = Rc::new(RefCell::new(vec![]));
    let mut size = (0, 0);

    for i in 0..shortcuts.len() {
        let key = shortcuts[i];
        let layout = shortcut_label.create_pango_layout(None);
        layout.set_markup(
            format!(
                "<span font_desc=\"{} {}\">{}</span>",
                font_family, font_size, key
            )
            .as_str(),
        );
        let pixel_size = layout.pixel_size();
        size.0 += pixel_size.0;
        size.1 = size.1.max(pixel_size.1 + ver_inset as i32 * 2);
        layouts.borrow_mut().push(layout);

        if i != shortcuts.len() - 1 {
            let layout = shortcut_label.create_pango_layout(None);
            layout.set_markup(
                format!("<span font_desc=\"{} {}\">+</span>", font_family, font_size).as_str(),
            );
            let pixel_size = layout.pixel_size();
            size.0 += pixel_size.0;
            size.1 = size.1.max(pixel_size.1 + ver_inset as i32 * 2);
            layouts.borrow_mut().push(layout);
        }
    }

    shortcut_label.set_content_width(size.0);
    shortcut_label.set_content_height(size.1);
    let layouts_clone = layouts.clone();

    shortcut_label.set_draw_func(move |_, cr, _, _| {
        let mut current_rec_x = 0.;
        let mut current_text_x = hor_inset;
        let degress = PI / 180.;

        for i in 0..layouts_clone.borrow().len() {
            let layout = &layouts_clone.borrow()[i];
            let pixel_size = layout.pixel_size();

            if i % 2 == 0 {
                let width = pixel_size.0 as f64 + hor_inset * 2.;
                let height = pixel_size.1 as f64 + ver_inset;

                cr.set_line_width(0.5);

                // Draw the round corner.
                cr.new_sub_path();
                cr.arc(
                    current_rec_x + width - radius,
                    0. + radius,
                    radius,
                    -90. * degress,
                    0. * degress,
                );
                cr.arc(
                    current_rec_x + width - radius,
                    0. + height - radius,
                    radius,
                    0. * degress,
                    90. * degress,
                );
                cr.arc(
                    current_rec_x + radius,
                    0. + height - radius,
                    radius,
                    90. * degress,
                    180. * degress,
                );
                cr.arc(
                    current_rec_x + radius,
                    0. + radius,
                    radius,
                    180. * degress,
                    270. * degress,
                );
                cr.close_path();

                cr.set_source_rgb(1., 1., 1.);
                cr.fill_preserve().unwrap();
                cr.set_source_rgb(0.5, 0.5, 0.5);
                cr.stroke().unwrap();

                current_rec_x += pixel_size.0 as f64 + hor_inset * 2.;

                cr.set_source_rgb(0.2, 0.2, 0.2);
                cr.move_to(current_text_x, ver_inset);
                pangocairo::show_layout(cr, layout);
                current_text_x += pixel_size.0 as f64 + hor_inset;
            } else {
                cr.set_source_rgb(0.5, 0.5, 0.5);
                cr.move_to(current_text_x + join_inset, ver_inset);
                pangocairo::show_layout(cr, layout);
                current_text_x += pixel_size.0 as f64 + hor_inset + join_inset * 2.;
                current_rec_x += pixel_size.0 as f64 + join_inset * 2.;
            }
        }
    });

    let shortcut = gtk::ShortcutLabel::builder()
        .accelerator("<Shift>A")
        .build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    vbox.append(&shortcut_label);
    vbox.append(&shortcut);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shortcut Label")
        .child(&vbox)
        .build();
    window.present();
}
