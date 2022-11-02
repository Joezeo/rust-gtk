use glib::clone;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Label;

const APP_ID: &str = "com.toocol.actions";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .build();

    action_and_group(&window);

    window.present();
}

pub fn action_and_group(window: &ApplicationWindow) {
    // let action_close = SimpleAction::new("close", None);
    // action_close.connect_activate(clone!(@weak window => move |_,_| {
    //     window.close();
    // }));
    // window.add_action(&action_close);

    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_,_| {
        window.close();
    }));

    let actions = SimpleActionGroup::new();
    actions.add_action(&action_close);
    window.insert_action_group("win", Some(&actions));
}

pub fn state_and_parameter(window: &ApplicationWindow) {
    let original_state = 0;
    let label = Label::builder()
        .label(&format!("Counter: {original_state}"))
        .build();
    
    // Create a button with label.
}
