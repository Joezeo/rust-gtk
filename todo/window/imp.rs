use std::cell::RefCell;

use gtk::glib::subclass::InitializingObject;
use gtk::prelude::InitializingWidgetExt;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    gio,
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass},
        widget::WidgetImpl,
        window::WindowImpl,
    },
    CompositeTemplate, Entry, TemplateChild, ListView,
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/todo/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,
    pub tasks: RefCell<Option<gio::ListStore>>
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";

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

        let instance = self.instance();
        instance.setup_tasks();
        instance.setup_callbacks();
        instance.setup_factory();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
