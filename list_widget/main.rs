#![allow(unused_imports)]
mod integer_object;

use gtk::{
    gio::{self, ListStore},
    glib::BindingFlags,
    prelude::*,
    Application, ApplicationWindow, CustomFilter, CustomSorter, FilterListModel, Label, ListBox,
    ListView, PolicyType, ScrolledWindow, SignalListItemFactory, SingleSelection, Widget, SortListModel, FilterChange, SorterChange,
};
use integer_object::IntegerObject;

const APP_ID: &str = "com.toocol.list_widget";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    // let list_box = ListBox::new();
    // for number in 0..100000 {
    //     let label = Label::new(Some(&number.to_string()));
    //     list_box.append(&label);
    // }

    // create the vec of IntegerObject
    let vector: Vec<IntegerObject> = (0..=100_000).into_iter().map(IntegerObject::new).collect();
    // create new model
    let model = ListStore::new(IntegerObject::static_type());
    // add the vector to model
    model.extend_from_slice(&vector);

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = Label::new(None);
        list_item.set_child(Some(&label));

        // Bind `list_item->item->number` to `label->label`
        // Expressions allow us to describe relationships between objects or properties that might not even exist yet
        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            .bind(&label, "label", Widget::NONE);
    });

    // Setup a filter model
    let filter = CustomFilter::new(move |obj| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object = obj.downcast_ref::<IntegerObject>().expect("Type mismatch");

        let number = integer_object.property::<i32>("number");

        number % 2 == 0
    });
    let filter_model = FilterListModel::new(Some(&model), Some(&filter));

    let sorter = CustomSorter::new(move |obj1, obj2| {
        let integer_object_1 = obj1.downcast_ref::<IntegerObject>().expect("Type mismatch");
        let integer_object_2 = obj2.downcast_ref::<IntegerObject>().expect("Type mismatch");

        let number1 = integer_object_1.property::<i32>("number");
        let number2 = integer_object_2.property::<i32>("number");

        number2.cmp(&number1).into()
    });
    let sort_model = SortListModel::new(Some(&filter_model), Some(&sorter));

    /*  Bind data is unuse when set property expression. */
    // bind the data
    // factory.connect_bind(move |_, list_item| {
    //     let integer_object = list_item
    //         .item()
    //         .expect("The item has to exist.")
    //         .downcast::<IntegerObject>()
    //         .expect("The item has to be an IntegerObject");

    //     // get label from list_item
    //     let label = list_item
    //         .child()
    //         .expect("Child not exist")
    //         .downcast::<Label>()
    //         .expect("Label type mismatch");

    // get i32 number from integet_object
    // let number = integer_object.property::<i32>("number");
    // set number to label
    // label.set_label(&number.to_string());

    // bind label to num
    // integer_object
    //     .bind_property("number", &label, "label")
    //     .flags(BindingFlags::SYNC_CREATE)
    //     .build();
    // });

    // use SingleSelection to wrap model
    let selection_model = SingleSelection::new(Some(&sort_model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));

    list_view.connect_activate(move |list_view, position| {
        let model = list_view.model().expect("The module not exist");
        let integer_object = model
            .item(position)
            .expect("Item not exist.")
            .downcast::<IntegerObject>()
            .expect("Item has to be an `Integer Obejct`");

        integer_object.increase_number();

        // Notify that the filter and sorter have been changed
        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    });

    let scroll_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My gtk app")
        .default_width(600)
        .default_height(300)
        .child(&scroll_window)
        .build();

    window.present();
}
