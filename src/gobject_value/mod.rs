#![allow(dead_code)]
#![allow(unused_imports)]
use gtk::prelude::*;

/*
Value was like this:

enum Value <T> {
    bool(bool),
    i8(i8),
    i32(i32),
    u32(u32),
    i64(i64),
    u64(u64),
    f32(f32),
    f64(f64),
    // boxed types
    String(Option<String>),
    Object(Option<dyn IsA<glib::Object>>),
}

*/

#[test]
fn gobject_value() {
    let integer_value = 10.to_value();
    let val = integer_value.get::<i32>().expect("error");
    assert_eq!(val, 10);

    let gstring = "Hello world".to_value();
    let str = gstring.get::<String>().expect("error");
    assert_eq!(str, "Hello world");

    let none_string_value = None::<String>.to_value();
    let none_string = none_string_value.get::<Option<String>>().expect("error");
    assert_eq!(none_string, None);
}

/**
 * Variant was used to things like saving settings to a file...
 */
#[test]
fn gobejct_variant() {
    let integer_variant = 10.to_variant();
    let integer = integer_variant.get::<i32>().expect("msg");
    assert_eq!(10, integer);

    let vec_variant = vec!["Hello", "World"].to_variant();
    assert_eq!(vec_variant.n_children(), 2);
    let vec = &vec_variant.get::<Vec<String>>().expect("msg");
    assert_eq!(vec[0], "Hello");
}