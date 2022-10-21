#![allow(unused_imports)]
mod custom_window;

use custom_window::Window;
use gtk::gio::{Settings, SettingsBindFlags};
use gtk::prelude::*;
use gtk::{Align, Application, Inhibit, Switch};

const APP_ID: &str = "com.toocol.settings";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    // Initialize the settings.
    let settings = Settings::new(APP_ID);

    let is_switch_enable = settings.boolean("is-switch-enabled");

    let switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enable)
        .build();

    // switch.connect_state_set(move |_, is_enabled| {
    //     settings
    //         .set_boolean("is-switch-enabled", is_enabled)
    //         .expect("Could not set settings.");
    //     Inhibit(false)
    // });

    settings
        .bind("is-switch-enabled", &switch, "state")
        .flags(SettingsBindFlags::DEFAULT)
        .build();

    let window = Window::new(app);
    window.set_child(Some(&switch));
    window.set_title(Some("My GTK App"));

    window.present();
}
