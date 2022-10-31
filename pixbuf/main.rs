use std::{time::Duration, sync::atomic::{AtomicI32}};

use gtk::{gdk_pixbuf::Pixbuf, glib::{Bytes, timeout_add_local}, prelude::*, Application, ApplicationWindow, Picture};

const SIZE: usize = 300 * 200;
const LEN: usize = SIZE * 3;
static mut RGB_BYTES: [u8; LEN] = [0; LEN];
static DRIRECT: AtomicI32 = AtomicI32::new(1);

fn main() {
    let app = Application::builder().application_id("com.pixbuf").build();

    app.connect_activate(build_ui);

    app.run();
}

fn change_rgb(picture: &Picture) {
    unsafe {
        for i in 0..LEN {
            DRIRECT.store(match RGB_BYTES[i] {
                255 => -1,
                0 => 1,
                _ => DRIRECT.load(std::sync::atomic::Ordering::SeqCst),
            }, std::sync::atomic::Ordering::SeqCst);
            RGB_BYTES[i] = (RGB_BYTES[i] as i32 + DRIRECT.load(std::sync::atomic::Ordering::SeqCst)) as u8;
        }

        let pixbuf = Pixbuf::from_bytes(
            &Bytes::from_static(&RGB_BYTES),
            gtk::gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            300,
            200,
            300 * 3,
        );
        println!("Rgb bytes changed.");
        picture.set_pixbuf(Some(&pixbuf))
    }
}

fn build_ui(app: &Application) {
    unsafe {
        let pixbuf = Pixbuf::from_bytes(
            &Bytes::from_static(&RGB_BYTES),
            gtk::gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            300,
            200,
            300 * 3,
        );

        let picture = Picture::for_pixbuf(&pixbuf);
        picture.set_can_shrink(false);
        picture.set_halign(gtk::Align::Start);
        picture.set_valign(gtk::Align::Start);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Pixbuf")
            .child(&picture)
            .build();

        timeout_add_local(Duration::from_millis(10), move || {
            change_rgb(&picture);
            Continue(true)
        });

        window.present();
    }
}
