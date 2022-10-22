#![allow(unused_imports)]
use gtk::{
    prelude::*, Application, ApplicationWindow, HeaderBar, Label, ListView, NoSelection,
    PolicyType, ScrolledWindow, SignalListItemFactory, StringList, StringObject, Widget,
};

const APP_ID: &str = "com.toocol.string_list";

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let model: StringList = (0..100_000)
        .into_iter()
        .map(|num| num.to_string())
        .collect();

    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        // Create label
        let label = Label::new(None);
        list_item.set_child(Some(&label));

        list_item
            .property_expression("item")
            .chain_property::<StringObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let selection_mode = NoSelection::new(Some(&model));
    let list_view = ListView::new(Some(&selection_mode), Some(&factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();

    // let title_bar = HeaderBar::builder()
    // .show_title_buttons(false)
    // .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My gtk App")
        // .titlebar(&title_bar)
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    window.present();
}
