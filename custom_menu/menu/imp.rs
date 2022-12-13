use gtk::glib;
use gtk::subclass::prelude::*;
use once_cell::sync::OnceCell;

use crate::custom_menu_model::CustomMenuModel;

#[derive(Default)]
pub struct CustomMenu {
    pub model: OnceCell<CustomMenuModel>
}

#[glib::object_subclass]
impl ObjectSubclass for CustomMenu {
    const NAME: &'static str = "CustomMenu";

    type Type = super::CustomMenu;

    type ParentType = gtk::Popover;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name("menu");

        klass.set_accessible_role(gtk::AccessibleRole::Menu);
    }
}

impl ObjectImpl for CustomMenu {
    fn constructed(&self) {
        self.parent_constructed();
        
    }
}

impl WidgetImpl for CustomMenu {}

impl PopoverImpl for CustomMenu {}
