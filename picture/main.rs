use gtk::{
    gdk_pixbuf::Pixbuf, prelude::*, Align, Application, ApplicationWindow, Picture,
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

    window.set_child(Some(&scrolled_window));

    window.present();
}
