mod imp;

use glib::Object;
use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};

use crate::task_object::TaskData;
use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct CollectionObject(ObjectSubclass<imp::CollectionObject>);
}

impl CollectionObject {
    pub fn new(title: &str, tasks: gio::ListStore) -> Self {
        Object::builder()
            .property("title", title)
            .property("tasks", tasks)
            .build()
    }

    pub fn tasks(&self) -> gio::ListStore {
        self.imp()
            .tasks
            .get()
            .expect("Could not get tasks.")
            .clone()
    }

    pub fn to_collection_data(&self) -> CollectionData {
        let title = self.imp().title.borrow().clone();
        let task_data = self
            .tasks()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<TaskObject>)
            .map(TaskObject::task_data)
            .collect();

        CollectionData { title, task_data }
    }

    pub fn from_collection_data(collection_data: CollectionData) -> Self {
        let title = collection_data.title;
        let tasks_to_extend: Vec<TaskObject> = collection_data
            .task_data
            .into_iter()
            .map(TaskObject::from_task_data)
            .collect();

        let tasks = gio::ListStore::new(TaskObject::static_type());
        tasks.extend_from_slice(&tasks_to_extend);

        Self::new(&title, tasks)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub title: String,
    pub task_data: Vec<TaskData>,
}
