use glib::clone;
use gtk::gio::SimpleAction;
use gtk::gio::SimpleActionGroup;
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
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

    // action_and_group(&window);
    state_and_parameter(&window);

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
        .label(&format!("Begin counter: {original_state}"))
        .build();

    // Create a button with label.
    let button = Button::builder().label("Press me!").build();

    // Connect to "clicked" signal of Button.
    button.connect_clicked(move |button| {
        // Activate the `win.count` action, and pass "1" as parameter.
        let parameter = 1;
        button
            .activate_action("win.count", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
    });

    // Create `Gtk::Box`, add `button` and `label` to it.
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_bottom(12)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .halign(gtk::Align::Center)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    window.set_child(Some(&gtk_box));

    // Add 'count' action to `window`, taking an integer as parameter.
    let action_count = SimpleAction::new_stateful(
        "count",
        Some(&i32::static_variant_type()),
        &original_state.to_variant(),
    );
    // Process when action `count` was activated.
    action_count.connect_activate(clone!(@weak label => move |action, parameter| {
        // Get state
        let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<i32>()
            .expect("The variant needs to be of type `i32`.");

        // Get parameter
        let parameter = parameter
            .expect("Could not get parameter.")
            .get::<i32>()
            .expect("The variant needs to be of type `i32`.");

        // Increase state by parameter and store state.
        state += parameter;
        action.set_state(&state.to_variant());

        // Update label with new state.
        label.set_label(&format!("Counter: {state}"));
    }));
    window.add_action(&action_count);
}
