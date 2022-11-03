mod imp;

use glib::{clone, Object};
use gtk::gio::PropertyAction;
use gtk::{prelude::*, Orientation};
use gtk::subclass::prelude::*;
use gtk::{
    gio::{self, SimpleAction},
    glib, Application,
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_actions(&self) {
        let label = self.imp().label.get();

        // Add stateful action `count` to `Window`, taking an integer as parameter.
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
            // Get state
            let mut state = action
                .state()
                .expect("Count not get state.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {state}"));
        }));

        self.add_action(&action_count);

        // Add property action "sensitive-button" to `Window`.
        let button = self.imp().button.get();
        let action_sensitive_button = PropertyAction::new("sensitive-button", &button, "sensitive");
        self.add_action(&action_sensitive_button);

        // Add stateful action `orientation` to `Window` taking a string as parameter.
        let gtk_box = self.imp().gtk_box.get();
        let action_orientation = SimpleAction::new_stateful(
            "orientation",
            Some(&String::static_variant_type()),
            &"Vertical".to_variant(),
        );

        action_orientation.connect_activate(clone!(@weak gtk_box => move |action, parameter| {
            // Get parameter
            let parameter = parameter
                .expect("Counld not get parameter.")
                .get::<String>()
                .expect("The value needs to by type of `String`.");
            
            let orientation = match parameter.as_str() {
                "Horizontal" => Orientation::Horizontal, 
                "Vertical" => Orientation::Vertical,
                _ => unreachable!(),
            };

            // Set the orientation
            gtk_box.set_orientation(orientation);
            action.set_state(&parameter.to_variant());
        }));
        self.add_action(&action_orientation);
    }
}
