mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomMenuModel(ObjectSubclass<imp::CustomMenuModel>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}