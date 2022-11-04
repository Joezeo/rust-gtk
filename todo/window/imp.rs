use std::cell::RefCell;
use std::fs::File;

use adw::{
    prelude::*,
    subclass::prelude::{AdwApplicationWindowImpl, *},
    Leaflet,
};
use gtk::{
    gio::{self, Settings},
    glib::{self, subclass::InitializingObject, SignalHandlerId},
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{ObjectImpl, ObjectSubclass},
        widget::WidgetImpl,
        window::WindowImpl,
    },
    Button, CompositeTemplate, Entry, FilterListModel, ListBox, Stack, TemplateChild,
};
use log::info;
use once_cell::sync::OnceCell;

use crate::collection_object::CollectionObject;
use crate::{collection_object::CollectionData, util::data_path};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/todo/window.ui")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    // pub tasks_list: TemplateChild<ListView>,
    pub tasks_list: TemplateChild<ListBox>,
    #[template_child]
    pub collections_list: TemplateChild<ListBox>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub back_button: TemplateChild<Button>,

    // pub tasks: RefCell<Option<gio::ListStore>>,
    pub settings: OnceCell<Settings>,
    pub collections: OnceCell<gio::ListStore>,
    pub current_collection: RefCell<Option<CollectionObject>>,
    pub current_filter_model: RefCell<Option<FilterListModel>>,
    pub tasks_changed_handler_id: RefCell<Option<SignalHandlerId>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";

    type Type = super::Window;

    type ParentType = adw::ApplicationWindow;

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
        instance.setup_collections();
        instance.restore_data();
        instance.setup_callbacks();
        instance.setup_actions();
        // instance.setup_tasks();
        // instance.setup_factory();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::signal::Inhibit {
        // Store task data in Vector
        // let backup_data: Vec<TaskData> = self
        //     .instance()
        //     .tasks()
        //     .snapshot()
        //     .iter()
        //     .filter_map(Cast::downcast_ref::<TaskObject>)
        //     .map(TaskObject::task_data)
        //     .collect();
        let backup_data: Vec<CollectionData> = self
            .instance()
            .collections()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<CollectionObject>)
            .map(CollectionObject::to_collection_data)
            .collect();

        // Save state to file
        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data).expect("Could not write data to json file.");

        info!(
            "Close request, save task data. size = {}",
            backup_data.len()
        );

        // Parent close
        self.parent_close_request()
    }
}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
