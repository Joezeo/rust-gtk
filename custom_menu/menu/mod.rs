mod imp;

use gtk::{
    gdk::Rectangle,
    glib::{self, Object},
    PositionType, prelude::ToValue,
};

glib::wrapper! {
    pub struct CustomMenu(ObjectSubclass<imp::CustomMenu>)
        @extends gtk::Popover, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::ShortcutManager;
}

impl CustomMenu {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn builder() -> CustomMenuBuilder {
        CustomMenuBuilder::new()
    }
}

#[derive(Default)]
pub struct CustomMenuBuilder {
    point_to: Option<Rectangle>,
    position: Option<PositionType>,
    has_arrow: Option<bool>,
}

impl CustomMenuBuilder {
    pub fn new() -> Self {
        CustomMenuBuilder::default()
    }

    pub fn build(self) -> CustomMenu {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref point_to) = self.point_to {
            properties.push(("point-to", point_to));
        }
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref has_arrow) = self.has_arrow {
            properties.push(("has-arrow", has_arrow));
        }
        Object::new(&properties)
    }

    pub fn point_to(mut self, point_to: Rectangle) -> Self {
        self.point_to = Some(point_to);
        self
    }

    pub fn position(mut self, position: PositionType) -> Self {
        self.position = Some(position);
        self
    }

    pub fn has_arrow(mut self, has_arrow: bool) -> Self {
        self.has_arrow = Some(has_arrow);
        self
    }
}
