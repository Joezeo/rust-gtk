use gtk::glib::subclass::InitializingObject;
use gtk::{TemplateChild, Label, CompositeTemplate, subclass::prelude::ObjectSubclass};
use gtk::{glib, Button};
use gtk::gio::Settings;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::OnceCell;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/window.ui")]
pub struct Window {
    #[template_child]
    pub gtk_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub label: TemplateChild<Label>,

    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.instance();
        obj.setup_settings();
        obj.setup_actions();
        obj.bind_settings();
    }
} 

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}