use std::cell::RefCell;

use gtk::glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Label;
use gtk::{glib, CheckButton, CompositeTemplate};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/todo/task_row.ui")]
pub struct TaskRow {
    #[template_child]
    pub completed_button: TemplateChild<CheckButton>,
    #[template_child]
    pub content_label: TemplateChild<Label>,

    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

#[glib::object_subclass]
impl ObjectSubclass for TaskRow {
    const NAME: &'static str = "TodoTaskRow";

    type Type = super::TaskRow;

    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TaskRow {}

impl WidgetImpl for TaskRow {}

impl BoxImpl for TaskRow {}
