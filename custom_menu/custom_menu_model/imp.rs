use gtk::{glib, prelude::*, subclass::prelude::*};

#[derive(Default)]
pub struct CustomMenuModel {}

#[glib::object_subclass]
impl ObjectSubclass for CustomMenuModel {
    const NAME: &'static str = "CustomMenuModel";

    type Type = super::CustomMenuModel;

    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }
}

impl ObjectImpl for CustomMenuModel {
    fn constructed(&self) {
        self.parent_constructed();

        let layout = self
            .instance()
            .layout_manager()
            .unwrap()
            .downcast::<gtk::BoxLayout>()
            .unwrap();

        layout.set_orientation(gtk::Orientation::Vertical);
    }
}

impl WidgetImpl for CustomMenuModel {}
