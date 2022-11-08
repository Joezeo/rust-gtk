use std::{sync::atomic::AtomicI32, time::Duration};

use gtk::{
    gdk_pixbuf::Pixbuf,
    glib::{timeout_add_local, Bytes},
    prelude::*,
    Application, ApplicationWindow, DrawingArea,
};

const SIZE: usize = 300 * 200;
const LEN: usize = SIZE * 3;
static mut RGB_BYTES: [u8; LEN] = [140; LEN];
static DRIRECT: AtomicI32 = AtomicI32::new(1);

fn main() {
    let app = Application::builder().application_id("com.pixbuf").build();

    app.connect_activate(build_ui);

    app.run();
}

fn change_rgb(drawing_area: &DrawingArea) {
    unsafe {
        for i in 0..LEN {
            DRIRECT.store(
                match RGB_BYTES[i] {
                    255 => -1,
                    0 => 1,
                    _ => DRIRECT.load(std::sync::atomic::Ordering::SeqCst),
                },
                std::sync::atomic::Ordering::SeqCst,
            );
            RGB_BYTES[i] =
                (RGB_BYTES[i] as i32 + DRIRECT.load(std::sync::atomic::Ordering::SeqCst)) as u8;
        }

        drawing_area.set_draw_func(move |_, ctx, x, y| {
            let pixbuf = Pixbuf::from_bytes(
                &Bytes::from_static(&RGB_BYTES),
                gtk::gdk_pixbuf::Colorspace::Rgb,
                false,
                8,
                300,
                200,
                300 * 3,
            );
            ctx.set_source_pixbuf(&pixbuf, x as f64, y as f64);
            ctx.paint().expect("Invalid surface.");
            ctx.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        });
        drawing_area.queue_draw();
    }
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pixbuf")
        .child(&drawing_area)
        .build();

    timeout_add_local(Duration::from_millis(10), move || {
        change_rgb(&drawing_area);
        Continue(true)
    });

    window.present();
}
