use gtk::subclass::prelude::*;
use gtk::glib;

#[derive(Default)]
pub struct CustomMenu {}

#[glib::object_subclass]
impl ObjectSubclass for CustomMenu {
    const NAME: &'static str = "CustomMenu";

    type Type = super::CustomMenu;

    type ParentType = gtk::Widget;
}

impl ObjectImpl for CustomMenu {}

impl WidgetImpl for CustomMenu {}