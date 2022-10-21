#![allow(unused_imports)]

use glib::{clone, Continue, MainContext, PRIORITY_DEFAULT};
use gtk::glib::timeout_future_seconds;
use gtk::{glib, prelude::*, Application, ApplicationWindow, Button};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

const APP_ID: &str = "com.toocol.event_loop";

static COUNT: AtomicI32 = AtomicI32::new(0);

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    button.connect_clicked(move |button| {
        // let sender = sender.clone();
        // spawn a new thread to process the time consuming operations.
        // thread::spawn(move || {
        //     COUNT.fetch_add(1, Ordering::SeqCst);
        //     println!("One new thread! count = {}", COUNT.load(Ordering::SeqCst));
        //     sender.send(false).expect("Could not send through channel.");
        //     let five_seconds = Duration::from_secs(5);
        //     thread::sleep(five_seconds);
        //     COUNT.fetch_sub(1, Ordering::SeqCst);
        //     sender.send(true).expect("Could not send through channel.");
        // });

        // Let the main loop executes the asynchonous block
        let main_context = MainContext::default();
        main_context.spawn_local(clone!(@weak button => async move {
            COUNT.fetch_add(1, Ordering::SeqCst);
            println!("One new thread! count = {}", COUNT.load(Ordering::SeqCst));
            // sender.send(false).expect("Could not send through channel.");
            button.set_sensitive(false);
            timeout_future_seconds(5).await;
            COUNT.fetch_sub(1, Ordering::SeqCst);
            button.set_sensitive(true);
            // sender.send(true).expect("Could not send through channel.");
        }));
    });

    // receiver.attach(
    //     None,
    //     clone!(@weak button => @default-return Continue(false),
    //             move |enable_button| {
    //                 button.set_sensitive(enable_button);
    //                 Continue(true)
    //             }
    //     ),
    // );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
