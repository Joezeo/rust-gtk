use gtk::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, DrawingArea, Revealer,
};

fn main() {
    let app = Application::builder()
        .application_id("com.transition_seperator")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let dw1 = DrawingArea::builder()
        .content_width(200)
        .content_height(200)
        .hexpand(true)
        .build();
    dw1.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0., 0., 0.);
        cr.paint().unwrap();
    });
    // let box_dw1 = gtk::Box::builder().hexpand(true).homogeneous(true).build();
    // box_dw1.append(&dw1);

    let dw2 = DrawingArea::builder()
        .content_width(200)
        .content_height(200)
        .hexpand(true)
        .build();
    dw2.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(1., 1., 0.3);
        cr.paint().unwrap();
    });
    // let box_dw2 = gtk::Box::builder().hexpand(true).homogeneous(true).build();
    // box_dw2.append(&dw2);

    let hor_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .build();

    let revealer_dw1 = Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideLeft)
        .reveal_child(true)
        .transition_duration(1000)
        .child(&dw1)
        .build();
    let revealer_dw2 = Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideRight)
        .reveal_child(true)
        .transition_duration(1000)
        .child(&dw2)
        .build();
    hor_box.append(&revealer_dw1);
    hor_box.append(&revealer_dw2);

    let button_re = gtk::Button::builder().label("Revealer").build();
    let revealer_button = Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideUp)
        .reveal_child(true)
        .transition_duration(1000)
        .child(&button_re)
        .build();

    let button_button = gtk::Button::builder().label("Toggle Button").build();
    button_button.connect_clicked(clone!(@weak revealer_button as button => move |_| {
        button.set_reveal_child(!button.reveals_child());
    }));

    let button_left = gtk::Button::builder().label("Toggle Left").build();
    button_left.connect_clicked(clone!(@weak revealer_dw1 as dw1 => move |_| {
        dw1.set_reveal_child(!dw1.reveals_child());
    }));

    let button_right = gtk::Button::builder().label("Toggle Right").build();
    button_right.connect_clicked(clone!(@weak revealer_dw2 as dw2 => move |_| {
        dw2.set_reveal_child(!dw2.reveals_child());
    }));

    revealer_dw2.connect_reveal_child_notify(
        clone!(@weak revealer_dw2 as dw2 => move |_| {
            println!("{}", dw2.reveals_child())
        }),
    );

    // revealer_dw2.connect_child_revealed_notify(clone!(@weak revealer_dw2 as dw2 => move |_| {
    //     if dw2.reveals_child() {
    //         dw2.show()
    //     } else {
    //         dw2.hide()
    //     }
    // }));

    let ver_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    ver_box.append(&hor_box);
    ver_box.append(&revealer_button);
    ver_box.append(&button_button);
    ver_box.append(&button_left);
    ver_box.append(&button_right);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Transition Animation")
        .default_width(400)
        .child(&ver_box)
        .build();

    window.present();
}
