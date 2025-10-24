//! Simple test to verify Objective-C runtime linking works

use ios_sys::objc::{objc_getClass, sel_registerName};
use std::ffi::CString;

fn main() {
    unsafe {
        // Try to get NSString class
        let class_name = CString::new("NSString").unwrap();
        let class = objc_getClass(class_name.as_ptr());

        if class.is_null() {
            println!("Failed to get NSString class");
        } else {
            println!("Successfully got NSString class: {:?}", class);
        }

        // Try to register a selector
        let sel_name = CString::new("length").unwrap();
        let sel = sel_registerName(sel_name.as_ptr());

        if sel.is_null() {
            println!("Failed to register selector");
        } else {
            println!("Successfully registered selector: {:?}", sel);
        }
    }
}
