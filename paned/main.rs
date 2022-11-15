use gtk::{prelude::*, Application, ApplicationWindow, DrawingArea, Paned};

const APP_ID: &str = "com.test.paned";
fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let dw1 = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .build();
    dw1.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0.5, 0.5, 0.5);
        cr.paint().unwrap();
    });

    let dw2 = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .width_request(100)
        .build();
    dw2.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0.1, 0.1, 0.1);
        cr.paint().unwrap();
    });

    let paned = Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .start_child(&dw1)
        .end_child(&dw2)
        .build();
    paned.set_shrink_start_child(false);
    paned.set_shrink_end_child(false);
    paned.set_resize_start_child(true);
    paned.set_resize_end_child(true);

    paned.connect_max_position_notify(|paned| {
        println!("Max position: {}", paned.max_position());
    });

    paned.connect_min_position_notify(|paned| {
        println!("Min position: {}", paned.min_position());
    });

    paned.connect_position_notify(|paned| println!("Position: {}", paned.position()));

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(400)
        .title("Paned")
        .child(&paned)
        .build();
    window.present();
}
