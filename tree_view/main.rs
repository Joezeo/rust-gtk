use gtk::{
    glib::Type, prelude::*, Application, ApplicationWindow, CellAreaBox, CellRendererText,
    ScrolledWindow, TreeStore, TreeView, TreeViewColumn,
};

const APP_ID: &str = "com.test.tree_view";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let tree_view = TreeView::builder().headers_visible(false).build();

    let cell_renderer = CellRendererText::new();
    let cell_area = CellAreaBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    cell_area.pack_end(&cell_renderer, true, true, true);

    let column = TreeViewColumn::builder().cell_area(&cell_area).build();
    column.add_attribute(&cell_renderer, "text", 0);

    tree_view.append_column(&column);

    let tree_store = TreeStore::new(&[Type::STRING]);
    tree_view.set_model(Some(&tree_store));

    let iter = tree_store.append(None);
    tree_store.set_value(&iter, 0, &"Hello World".to_value());

    let iter_child = tree_store.append(Some(&iter));
    tree_store.set_value(&iter_child, 0, &"Hello World".to_value());

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(300)
        .min_content_height(200)
        .child(&tree_view)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tree View")
        .child(&scrolled_window)
        .build();

    window.present();
}
