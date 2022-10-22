// this proc-marco remove the cmd terminal
#![windows_subsystem = "windows"]

use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::CssProvider;
use gtk::StyleContext;
use gtk::gdk::Display;
use gtk::prelude::*;

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
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create a new window and show it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();
    window.show();
}