
use gtk::{subclass::prelude::*, glib};

#[derive(Default)]
pub struct CustomButton;

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyCustomButton";

    type Type = super::CustomButton;

    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton {}

impl WidgetImpl for CustomButton {}

impl ButtonImpl for CustomButton {}