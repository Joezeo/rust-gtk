use gtk::{
    gdk_pixbuf::Pixbuf, prelude::*, Align, Application, ApplicationWindow, Inhibit, Picture,
};

fn main() {
    let app = Application::builder()
        .application_id("com.toocol.picture")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let pixbuf = Pixbuf::from_file("D:/test.png").unwrap();

    let picture = Picture::for_pixbuf(&pixbuf);
    picture.set_valign(Align::Start);
    picture.set_halign(Align::Start);
    picture.set_can_shrink(false);
    picture.set_focusable(true);
    picture.set_focus_on_click(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Picture")
        .build();

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::External) // Disable horizontal scrolling
        .vscrollbar_policy(gtk::PolicyType::External) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&picture)
        .build();

    ///// Key Events
    let controller = gtk::EventControllerKey::new();
    controller.connect_key_pressed(move |controller, key, keycode, modfier| {
        if let Some(_) = controller.im_context() {
            return Inhibit(false);
        }
        let character = match key.to_unicode() {
            Some(c) => c,
            None => '\0',
        };
        println!(
            "Key pressed -> key: {:?}, name: {:?}, code: {}, modifier: {:?}, char: '{}'",
            key,
            key.name(),
            keycode,
            modfier,
            character
        );
        Inhibit(false)
    });
    picture.add_controller(&controller);

    let gtk_box = gtk::Box::builder().build();
    gtk_box.append(&scrolled_window);

    window.set_child(Some(&gtk_box));

    window.present();
}
