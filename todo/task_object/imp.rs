use gtk::{
    glib::{self, ParamSpec, ParamSpecBoolean, ParamSpecString, Value},
    prelude::*,
    subclass::prelude::*,
};
use std::{cell::RefCell, rc::Rc};

use once_cell::sync::Lazy;

use super::TaskData;

#[derive(Default)]
pub struct TaskObject {
    pub data: Rc<RefCell<TaskData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";

    type Type = super::TaskObject;
}

impl ObjectImpl for TaskObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::builder("completed").build(),
                ParamSpecString::builder("content").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "completed" => {
                let input_value = value.get().expect("The value needs to be of type `bool`.");
                self.data.borrow_mut().completed = input_value;
            }
            "content" => {
                let input_value = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.data.borrow_mut().content = input_value;
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "completed" => self.data.borrow().completed.to_value(),
            "content" => self.data.borrow().content.to_value(),
            _ => unimplemented!(),
        }
    }
}
