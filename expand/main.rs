use gtk::{prelude::*, Align, Application, ApplicationWindow, DrawingArea, Paned};

const APP_ID: &str = "com.test.expand";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button1 = gtk::Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .hexpand(true)
        .width_request(10)
        .height_request(10)
        .build();
    let button2 = gtk::Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .hexpand(true)
        .width_request(10)
        .height_request(10)
        .build();
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .halign(Align::Center)
        .hexpand(true)
        .build();

    gtk_box.append(&button1);
    gtk_box.append(&button2);

    let dw = DrawingArea::builder()
        .content_width(300)
        .content_height(200)
        .build();
    dw.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0.5, 0.5, 0.5);
        cr.paint().unwrap();
    });

    let paned = Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    paned.set_start_child(Some(&gtk_box));
    paned.set_end_child(Some(&dw));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Box expand")
        .child(&paned)
        .build();

    window.present();
}
