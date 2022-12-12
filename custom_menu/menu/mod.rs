mod imp;

use gtk::glib;
use gtk::glib::Object;

glib::wrapper! {
    pub struct CustomMenu(ObjectSubclass<imp::CustomMenu>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomMenu {
    pub fn new() -> Self {
        Object::new(&[])
    }
}