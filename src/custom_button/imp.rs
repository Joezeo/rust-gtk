use gtk::glib::subclass::{Signal};
use gtk::glib::{self, ParamSpecInt};
use gtk::glib::{BindingFlags, ParamSpec};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    // Customize property "number"
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecInt::builder("number").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number = value.get().expect("The number need to be type 'i32'");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }

    // customize own signals
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder(
                "max-number-reached",
                &[i32::static_type().into()],
                <()>::static_type().into(),
            )
            .build()]
        });
        SIGNALS.as_ref()
    }

    // construct
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // obj.set_label(&self.number.get().to_string());
        let instance = self.instance();
        instance
            .bind_property("number", &instance, "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

static MAX_NUMBER: i32 = 8;

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self, _button: &Self::Type) {
        // self.number.set(self.number.get() + 1);
        // button.set_label(&self.number.get().to_string())
        let incremented_number = self.number.get() + 1;
        let instance = self.instance();
        if incremented_number == MAX_NUMBER {
            instance.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            instance.set_property("number", 0);
        } else {
            instance.set_property("number", &incremented_number);
        }
    }
}
