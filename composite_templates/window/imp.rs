use std::cell::Cell;

use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};
use gtk::template_callbacks;

use crate::custom_button::CustomButton;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/window.ui")]
pub struct Window {
    // #[template_child]
    // pub button: TemplateChild<CustomButton>,
    pub number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";

    type Type = super::Window;

    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // Register `Custom Button`
        CustomButton::ensure_type();

        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


#[template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, button: &CustomButton) {
        let number_increased = self.number.get() + 1;
        self.number.set(number_increased);
        button.set_label(number_increased.to_string().as_str());
    }
}


impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // self.button.connect_clicked(move |button| {
        //     button.set_label("Hello World");
        // });
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
