use gtk::{
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, DrawingArea, GestureClick, Label, PopoverMenu, ScrolledWindow,
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
    drawing_area.connect_has_focus_notify(|dw| {
        if dw.is_focus() {
            println!("Widget get focus");
        } else {
            println!("Widget lose focus");
        }
    });

    let scrolled_window = ScrolledWindow::builder().child(&drawing_area).build();

    let menu = PopoverMenu::builder()
        .position(gtk::PositionType::Bottom)
        .build();
    menu.set_parent(&drawing_area);
    let label = Label::new(Some("test item"));
    menu.add_child(&label, "test");

    let gesture_click = GestureClick::new();
    gesture_click.set_button(3);
    gesture_click.connect_released(clone!(@weak menu, @weak drawing_area => move |_, _, _, _| {
        // menu.set_pointing_to(Some(&Rectangle::new(0, 0, 1280, 800)));
        menu.show();
    }));
    drawing_area.add_controller(&gesture_click);

    window.set_child(Some(&scrolled_window));
    window.present();
}
