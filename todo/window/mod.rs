#![allow(dead_code)]
mod imp;

use gtk::glib::{clone, Object};
use gtk::prelude::StaticType;
use gtk::subclass::prelude::*;
use gtk::traits::EntryExt;
use gtk::{gio, glib, Application, NoSelection};
use gtk::{prelude::*, SignalListItemFactory};

use crate::task_object::TaskObject;
use crate::task_row::TaskRow;

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
        let selection_model = NoSelection::new(Some(&self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
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
}
