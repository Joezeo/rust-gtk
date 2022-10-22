// this proc-marco remove the cmd terminal
#![windows_subsystem = "windows"]

use gtk::Align;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::CssProvider;
use gtk::StyleContext;

const APP_ID: &str = "org.gtk_rs.Css1";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create button
    let button_1 = Button::with_label("Press me!");
    let button_2 = Button::with_label("Pressm me!");

    let button_3 = Button::with_label("Destructive");
    let button_4 = Button::with_label("Suggested");

    button_1.add_css_class("button-1");
    button_2.set_widget_name("button-2");

    button_3.add_css_class("destructive-action");
    button_4.add_css_class("suggested-action");

    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(12)
        .build();

    gtk_box.append(&button_1);
    gtk_box.append(&button_2);
    gtk_box.append(&button_3);
    gtk_box.append(&button_4);

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    window.show();
}
