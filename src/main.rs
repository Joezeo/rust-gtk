#![allow(dead_code)]
mod custom_button;
mod gobject_value;

use std::cell::Cell;
use std::rc::Rc;

use crate::custom_button::CustomButton;
use glib::clone;
use gtk::gdk::Display;
use gtk::glib::{closure_local, BindingFlags, Value};
use gtk::{
    glib, Align, Application, ApplicationWindow, Box, Button, CssProvider, Orientation,
    StyleContext, Switch,
};
use gtk::{prelude::*, HeaderBar};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

// $env:GSK_RENDERER="cairo"
// Use this environment parameter can reduce memory use
fn main() {
    // Set the rendering envrioment
    std::env::set_var("GSK_RENDERER", "cairo");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_startup(|_| load_css());
    app.connect_activate(build_button_ui);

    // Run the application
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_button_ui(app: &Application) {
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();
    button_1.set_margin_top(12);
    button_1.set_margin_bottom(12);
    button_1.set_margin_start(12);
    button_1.set_margin_end(12);
    button_2.set_margin_top(12);
    button_2.set_margin_bottom(12);
    button_2.set_margin_start(12);
    button_2.set_margin_end(12);

    // Assure number of button_2 is always 1 higher than number of button_1
    button_1
        .bind_property("number", &button_2, "number")
        // Transform 'number' from button_1 to button_2
        .transform_to(|_, number: &Value| {
            let incremented_number = number.get::<i32>().unwrap() + 1;
            Some(incremented_number.to_value())
        })
        // Transform 'number' from button_2 to button_1
        .transform_from(|_, number: &Value| {
            let decremented_number = number.get::<i32>().unwrap() - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();

    // The closure will be called,
    // whenever the "number" property has changed.
    button_1.connect_notify_local(Some("number"), move |button, _| {
        let number = button.property::<i32>("number");
        println!("The current number of `button_1` is {}", number);
    });

    // button_2.connect_clicked(move |_| {
    //     println!("Hello Wrold!");
    // });

    button_2.connect_closure(
        "clicked",
        false,
        closure_local!(move |_button: Button| {
            println!("Hello Wrold!");
        }),
    );

    button_2.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_button: CustomButton, number: i32| {
            println!("The maximum  number {} has been reached.", number);
        }),
    );

    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);

    // Connect callbacks, when a button is clicked `number` will be changed
    button_increase.connect_clicked(
        clone!(@weak number, @weak button_decrease => move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }),
    );
    button_decrease.connect_clicked(clone!(@weak button_increase, @weak gtk_box => move |_| {
        number.set(number.get() - 1);
        button_increase.set_label(&number.get().to_string());

        let dyn_button = Button::builder().label("Dynamic button")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        gtk_box.append(&dyn_button);
    }));

    let header_bar = HeaderBar::builder().build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1280)
        .default_height(800)
        .titlebar(&header_bar)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}

fn build_switch_ui(app: &Application) {
    let switch1 = Switch::new();
    let switch2 = Switch::new();

    // switch.set_state(true);
    // let current_state = switch.state();

    // println!("the current state is {}", current_state);

    // switch1.set_property("state", true);
    // let state = switch1.property::<bool>("state");
    // println!("the current state is {}", state);

    switch1
        .bind_property("state", &switch2, "state")
        .flags(BindingFlags::BIDIRECTIONAL)
        .build();

    // Set up box
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch1);
    gtk_box.append(&switch2);

    let _header_bar = HeaderBar::builder().css_name("header-bar").build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        // .titlebar(&header_bar)
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
