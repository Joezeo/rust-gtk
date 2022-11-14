use gtk::{prelude::*, Application, ApplicationWindow, DrawingArea, Paned, Frame};

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
    let frame1 = Frame::builder().width_request(100).child(&dw1).build();

    let dw2 = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .build();
    dw2.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0.1, 0.1, 0.1);
        cr.paint().unwrap();
    });
    let frame2 = Frame::builder().width_request(300).child(&dw2).build();

    let paned = Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .start_child(&frame1)
        .end_child(&frame2)
        .build();

    paned.connect_max_position_notify(|paned| {
        println!("Max position: {}", paned.max_position());
    });

    paned.connect_min_position_notify(|paned| {
        println!("Min position: {}", paned.min_position());
    });

    paned.connect_position_notify(|paned| {
        println!("Position: {}", paned.position())
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(400)
        .title("Paned")
        .child(&paned)
        .build();
    window.present();
}
