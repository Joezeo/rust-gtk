mod font_config;

use gtk::{
    gdk::Display,
    glib::{self, clone},
    pango::{FontDescription, Layout},
    prelude::*,
    Application, ApplicationWindow, CssProvider, DrawingArea, FontChooserDialog, Label,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION, Orientation,
};
use pangocairo::FontMap;

const APP_ID: &str = "com.font_icon";

fn main() {
    // std::env::set_var("PANGOCAIRO_BACKEND", "fc");
    load_font!(
        "Font-Awesome-6-Free-Regular-400.otf",
        "SegMDL2.ttf",
        "Segoe-Fluent-Icons.ttf"
    );

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));

        if let Some(display) = &Display::default() {
            StyleContext::add_provider_for_display(
                display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        };
    });

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Font Icon")
        .default_width(600)
        .default_height(400)
        .build();

    let dw = DrawingArea::builder()
        .content_width(30)
        .content_height(30)
        .build();
    // Seems css not work.
    dw.add_css_class("font-icon");

    let font_map = FontMap::for_font_type(gtk::cairo::FontType::FontTypeWin32).unwrap();
    font_map.list_families().iter().for_each(|font| {
        println!("{}", font.name());
    });
    let context = font_map.create_context();
    let layout = Layout::new(&context);
    let font_description = FontDescription::from_string("Font Awesome 6 Free Regular");
    let text = "\u{f02e}";
    layout.set_font_description(Some(&font_description));
    layout.set_markup(format!("<span foreground=\"#6c698d\">{}</span>", text).as_str());

    dw.set_draw_func(move |_, cr, _, _| {
        cr.move_to(10., 10.);
        pangocairo::show_layout(&cr, &layout);
    });

    let size = 13;
    let label_1 = Label::new(None);
    label_1.set_font_map(Some(&font_map));
    label_1.set_markup(format!("<span foreground=\"#6c698d\" font_desc=\"Font Awesome 6 Free Regular {}\" font_features=\"dlig=1\">{}</span>", size, text).as_str());

    let text_2 = "\u{e76e}";
    let label_2 = Label::new(None);
    label_2.set_font_map(Some(&font_map));
    label_2.set_markup(format!("<span foreground=\"#6c698d\" font_desc=\"Segoe Fluent Icons {}\" font_features=\"dlig=1\"><b>{}</b></span>", size, text_2).as_str());

    let text_3 = "\u{E705}";
    let label_3 = Label::new(None);
    label_3.set_font_map(Some(&font_map));
    label_3.set_markup(format!("<span foreground=\"#6c698d\" font_desc=\"Segoe MDL2 Assets {}\" font_features=\"dlig=1\"><b>{}</b></span>", size, text_3).as_str());

    let button = gtk::Button::builder()
        .label("Click to change color")
        .vexpand(false)
        .build();
    button.connect_clicked(clone!(@weak label_1 => move |_| {
        label_1.set_markup(format!("<span foreground=\"#3244df\" font_desc=\"Font Awesome 6 Free light {}\" font_features=\"dlig=1\">{}</span>", size, text).as_str());
    }));
    let button_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .width_request(30)
        .height_request(20)
        .vexpand(false)
        .homogeneous(false)
        .build();
    button_box.append(&button);

    let font_chooser = FontChooserDialog::new(Some("Font Chooose"), Some(&window));

    button.connect_clicked(clone!(@weak font_chooser => move |_| {
        font_chooser.present();
    }));

    gtk_box.append(&dw);
    gtk_box.append(&label_1);
    gtk_box.append(&label_2);
    gtk_box.append(&label_3);
    gtk_box.append(&button_box);

    window.set_child(Some(&gtk_box));
    window.present();
}
