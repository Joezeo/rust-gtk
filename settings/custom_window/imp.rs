use gtk::{glib, gio::Settings, subclass::{prelude::{ObjectSubclass, ObjectImpl, ApplicationWindowImpl}, widget::WidgetImpl, window::WindowImpl}, ApplicationWindow, Inhibit};
use gtk::subclass::prelude::*;
use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MySettingsWindow";

    type Type = super::Window;

    type ParentType = ApplicationWindow;
}

impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let instance = self.instance();
        instance.setup_settings();
        instance.load_window_size();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self, _window: &Self::Type) -> glib::signal::Inhibit {
        // Save window size
        self.instance().save_window_size().expect("Failed to save window state.");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}

impl ApplicationWindowImpl for Window {}