#![allow(dead_code)]
use std::ffi::{c_char, CString};

#[link(name = "native-fontconfig")]
extern "C" {
    fn load_font(font_path: *const c_char);
    fn load_font_private(font_path: *const c_char);
}

pub struct FontConfig;
impl FontConfig {
    pub fn native_load_font(font_path: String) {
        unsafe {
            let font_path = CString::new(font_path).unwrap();
            load_font(font_path.as_ptr())
        }
    }
    pub fn native_load_font_private(font_path: String) {
        unsafe {
            let font_path = CString::new(font_path).unwrap();
            load_font_private(font_path.as_ptr())
        }
    }
}

#[macro_export]
macro_rules! load_font {
    () => {};
    ( $($x:expr),* ) => {
        {
            $(
                let mut path = "font/".to_string();
                path.push_str($x);
                font_config::FontConfig::native_load_font(path);
            )*
        }
     };
}