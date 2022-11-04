use std::cell::RefCell;
use std::fs::File;

use gtk::gio::Settings;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::InitializingWidgetExt;
use gtk::subclass::prelude::*;
use gtk::{
    gio, glib,
    prelude::*,
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectSubclass},
        widget::WidgetImpl,
        window::WindowImpl,
    },
    CompositeTemplate, Entry, ListView, TemplateChild,
};
use once_cell::sync::OnceCell;
use log::info;

use crate::task_object::{TaskObject, TaskData};
use crate::util::data_path;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/todo/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListView>,

    pub tasks: RefCell<Option<gio::ListStore>>,
    pub settings: OnceCell<Settings>,
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
        instance.setup_settings();
        instance.setup_tasks();
        instance.restore_data();
        instance.setup_callbacks();
        instance.setup_factory();
        instance.setup_actions();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::signal::Inhibit {
        // Store task data in Vector
        let backup_data: Vec<TaskData> = self
            .instance()
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();
        
        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file.");

        info!("Close request, save task data. size = {}", backup_data.len());

        // Parent close
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for Window {}
