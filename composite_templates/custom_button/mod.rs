mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable;
}

impl CustomButton {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        Self::new()
    }
}