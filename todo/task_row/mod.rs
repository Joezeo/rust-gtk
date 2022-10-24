mod imp;

use glib::Object;
use gtk::glib::{self, BindingFlags};
use gtk::pango::{AttrInt, AttrList};
use gtk::prelude::{ObjectExt, ToValue};
use gtk::subclass::prelude::*;

use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TaskRow {
    pub fn new() -> Self {
        Object::new(&[])
    }

    // Bind TaskObject to TaskRow, Data => View
    pub fn bind(&self, task_object: &TaskObject) {
        // Get state
        let completed_button = self.imp().completed_button.get();
        let content_label = self.imp().content_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // bind `task_object.completed` to `task_row.completed_button.active`
        let completed_button_binding = task_object
            .bind_property("completed", &completed_button, "active")
            .flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
            .build();
        bindings.push(completed_button_binding);

        // Bind `task_object.content` to `task_row.content_label.label`
        let content_label_binding = task_object
            .bind_property("content", &content_label, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
        bindings.push(content_label_binding);

        // Bind `task_object.completed` to `task_row.content_label.attributes`
        let content_label_binding = task_object
            .bind_property("completed", &content_label, "attributes")
            .flags(BindingFlags::SYNC_CREATE)
            .transform_to(|_, active| {
                let attr_list = AttrList::new();
                if active {
                    let attr = AttrInt::new_strikethrough(true);
                    attr_list.insert(attr);
                }
                Some(attr_list.to_value())
            })
            .build();
        bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all store bind
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
