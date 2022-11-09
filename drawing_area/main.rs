use std::{sync::atomic::AtomicI32, time::Duration};

use gtk::{
    cairo::ImageSurface,
    glib::timeout_add_local,
    prelude::*,
    Application, ApplicationWindow, DrawingArea,
};

const SIZE: usize = 300 * 200;
const LEN: usize = SIZE * 4;
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

        drawing_area.queue_draw();
    }
}

fn build_ui(app: &Application) {
    let drawing_area = DrawingArea::builder().build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pixbuf")
        .child(&drawing_area)
        .build();

    unsafe {
        let surface = ImageSurface::create_for_data_unsafe(
            RGB_BYTES.as_mut_ptr(),
            gtk::cairo::Format::Rgb24,
            300,
            200,
            300 * 4,
        )
        .expect("Create `ImageSurface` failed.");

        drawing_area.set_draw_func(move |_, cr, _width, _height| {
            cr.set_source_surface(&surface, 0., 0.).expect("Set source surface failed.");
            cr.paint().expect("Invalid surface.");
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        });
    }

    timeout_add_local(Duration::from_millis(10), move || {
        change_rgb(&drawing_area);
        Continue(true)
    });

    window.present();
}
