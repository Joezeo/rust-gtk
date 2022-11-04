#![allow(dead_code)]
mod imp;

use std::fs::File;

use gtk::gio::{Settings, SimpleAction};
use gtk::glib::{clone, Object};
use gtk::prelude::StaticType;
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{gio, glib, Application, CustomFilter, FilterListModel, NoSelection};
use gtk::{prelude::*, SignalListItemFactory};

use crate::task_object::{TaskData, TaskObject};
use crate::task_row::TaskRow;
use crate::util::data_path;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }

    pub fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("Could not get current tasks.")
    }

    fn setup_tasks(&self) {
        // Create new model
        let model = gio::ListStore::new(TaskObject::static_type());

        // Get state and set model
        self.imp().tasks.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        // let selection_model = NoSelection::new(Some(&self.tasks()));
        // self.imp().tasks_list.set_model(Some(&selection_model));

        // Wrap model with filter and selection and pass it to the list view
        let filter_model = FilterListModel::new(Some(&self.tasks()), self.filter().as_ref());
        let selection_model = NoSelection::new(Some(&filter_model));
        self.imp().tasks_list.set_model(Some(&selection_model));

        // Filter model whenever the value of the key "filter" changes
        self.settings().connect_changed(
            Some("filter"),
            clone!(@weak self as window, @weak filter_model => move |_, _| {
                filter_model.set_filter(window.filter().as_ref());
            }),
        );
    }

    fn setup_callbacks(&self) {
        // Setup callback for activation of entry
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_task();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp()
            .entry
            .connect_icon_release(clone!(@weak self as window => move |_,_| {
                window.new_task();
            }));
    }

    fn new_task(&self) {
        // Get the content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");

        // Add new task to model
        let task = TaskObject::new(false, content);
        self.tasks().append(&task);
    }

    fn setup_factory(&self) {
        // Create new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            let task_row = TaskRow::new();
            list_item.set_child(Some(&task_row));
        });

        // Tell factory how to bind `TaskRow` ot `TaskObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TaskObject` rom list_item
            let task_object = list_item
                .item()
                .expect("Get task object from list_item failed.")
                .downcast::<TaskObject>()
                .expect("Item has to be TaskObject");

            // Get `TaskRow` from list_item
            let task_row = list_item
                .child()
                .expect("Get task row from list_item failed.")
                .downcast::<TaskRow>()
                .expect("Child has to be TaskRow");

            task_row.bind(&task_object);
        });

        // Tell factory how to unbind `TaskRow` ot `TaskObject`
        factory.connect_unbind(move |_, list_item| {
            let task_row = list_item
                .child()
                .expect("Get task row from list_item failed.")
                .downcast::<TaskRow>()
                .expect("The child has to be a `TaskRow`.");
            task_row.unbind();
        });

        self.imp().tasks_list.set_factory(Some(&factory));
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`")
    }

    fn setup_actions(&self) {
        // Create action from key `filter` and add to action group "win"
        let filter_action = self.settings().create_action("filter");
        self.add_action(&filter_action);

        // Create action to remove done tasks and add to action group "win"
        let remove_done_tasks_action = SimpleAction::new("remove-done-tasks", None);
        remove_done_tasks_action.connect_activate(clone!(@weak self as window => move |_, _| {
            let tasks = window.tasks();
            let mut position = 0;
            while let Some(item) = tasks.item(position) {
                // Get `TaskObject` from `glib::object`
                let task_object = item
                    .downcast_ref::<TaskObject>()
                    .expect("The object needs to be of type `TaskObject`.");

                if task_object.is_completed() {
                    tasks.remove(position);
                } else {
                    position += 1;
                }
            }
        }));
        self.add_action(&remove_done_tasks_action);
    }

    fn filter(&self) -> Option<CustomFilter> {
        // Get filter_state from `settings`
        let filter_state: String = self.settings().get("filter");

        // Create CustomFilter
        let filter_open = CustomFilter::new(|obj| {
            let task_obj = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow none completed tasks
            !task_obj.is_completed()
        });
        let filter_done = CustomFilter::new(|obj| {
            let task_obj = obj
                .downcast_ref::<TaskObject>()
                .expect("The object needs to be of type `TaskObject`.");

            // Only allow none completed tasks
            task_obj.is_completed()
        });

        // Return the correct filter
        match filter_state.as_str() {
            "All" => None,
            "Open" => Some(filter_open),
            "Done" => Some(filter_done),
            _ => unreachable!(),
        }
    }

    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            // Deserialize data from file to Vector
            let buckup_data: Vec<TaskData> = serde_json::from_reader(file)
                .expect("It should be possible to read `backup_data` from the json file.");

            // Cenvert `Vec<TaskData>` to `Vec<TaskObject>`
            let task_objects: Vec<TaskObject> = buckup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();
            
            // Insert restored objects into model
            self.tasks().extend_from_slice(&task_objects);
        }
    }
}
