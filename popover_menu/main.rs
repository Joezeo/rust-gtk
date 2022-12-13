use gtk::{
    gdk::Rectangle,
    gio::{Menu, MenuItem},
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, DrawingArea, GestureClick, PopoverMenu, ScrolledWindow,
};

const APP_ID: &str = "com.popover_menu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Popover Menu")
        .default_width(1280)
        .default_height(800)
        .build();

    let drawing_area = DrawingArea::builder()
        .content_width(1280)
        .content_height(800)
        .can_focus(true)
        .focus_on_click(true)
        .build();
    drawing_area.set_draw_func(|_, cr, _, _| {
        cr.set_source_rgb(0., 0., 0.);
        cr.paint().unwrap();
    });

    let scrolled_window = ScrolledWindow::builder()
        .child(&drawing_area)
        .min_content_height(300)
        .min_content_width(400)
        .build();

    let model = Menu::new();
    let item = MenuItem::new(Some("<span>Close window</span>"), None);
    item.set_attribute_value("use-markup", Some(&true.to_variant()));
    item.set_attribute_value("icon", Some(&"list-add-symbolic".to_variant()));
    model.append_item(&item);
    // model.append(Some("Close window"), None);
    model.append(Some("Sensitive button"), None);

    let orientation_section = Menu::new();
    orientation_section.append(Some("Vertical"), None);
    orientation_section.append(Some("Horizontal"), None);
    model.append_section(Some("Orientation"), &orientation_section);

    let language_section = Menu::new();
    language_section.append(Some("English"), None);
    language_section.append(Some("Chinese"), None);
    model.append_section(None, &language_section);

    let menu = PopoverMenu::builder()
        .has_arrow(false)
        .position(gtk::PositionType::Bottom)
        .menu_model(&model)
        .cascade_popdown(true)
        .build();
    menu.set_parent(&drawing_area);

    let gesture_click = GestureClick::new();
    gesture_click.set_button(3);
    gesture_click.connect_released(clone!(@weak menu => move |_, _, x, y| {
        menu.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 1, 1,)));
        menu.show();
    }));
    drawing_area.add_controller(&gesture_click);

    window.set_child(Some(&scrolled_window));
    window.present();
}
