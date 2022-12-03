use gtk::{glib, prelude::*, subclass::prelude::*, Align};
use once_cell::sync::OnceCell;

#[derive(Default)]
pub struct CommandPanel {
    entry: OnceCell<gtk::Entry>,
}

#[glib::object_subclass]
impl ObjectSubclass for CommandPanel {
    const NAME: &'static str = "CommandPanel";

    type Type = super::CommandPanel;

    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }
}

impl ObjectImpl for CommandPanel {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.instance();

        let layout = self
            .instance()
            .layout_manager()
            .unwrap()
            .downcast::<gtk::BoxLayout>()
            .unwrap();
        layout.set_orientation(gtk::Orientation::Vertical);

        let entry = gtk::Entry::builder().build();
        entry.set_parent(&*obj);
        self.entry
            .set(entry)
            .expect("`entry` of CommandPanel can only set once.");
        
        obj.set_width_request(500);
        obj.set_halign(Align::Center);
        obj.set_margin_top(50);
    }

    fn dispose(&self) {
        self.entry
            .get()
            .expect("`entry` of CommandPanel should set before use.")
            .unparent();
    }
}

impl WidgetImpl for CommandPanel {}
