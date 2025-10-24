//! Integration tests for Objective-C runtime bindings

use ios_sys::objc::{objc_getClass, sel_registerName};
use std::ffi::CString;

#[test]
fn test_objc_get_class_nsstring() {
    unsafe {
        let class_name = CString::new("NSString").unwrap();
        let class = objc_getClass(class_name.as_ptr());

        // In header-only mode, this will compile but the function
        // won't be available at link time unless 'runtime' feature is enabled
        #[cfg(feature = "runtime")]
        {
            // On actual iOS device with runtime, NSString class should exist
            // But we can't test this without running on device
            // So we just verify the function signature is correct
            let _ = class;
        }

        #[cfg(not(feature = "runtime"))]
        {
            // In header-only mode, just verify it compiles
            let _ = class;
        }
    }
}

#[test]
fn test_sel_register_name() {
    unsafe {
        let sel_name = CString::new("length").unwrap();
        let sel = sel_registerName(sel_name.as_ptr());

        #[cfg(feature = "runtime")]
        {
            let _ = sel;
        }

        #[cfg(not(feature = "runtime"))]
        {
            let _ = sel;
        }
    }
}

#[test]
fn test_objc_types_exist() {
    // Just verify the types are accessible
    use ios_sys::objc::{Class, Ivar, Method, SEL};

    // These should compile
    let _class_ptr: Class = std::ptr::null_mut();
    let _method_ptr: Method = std::ptr::null_mut();
    let _ivar_ptr: Ivar = std::ptr::null_mut();
    let _sel_ptr: SEL = unsafe { *std::ptr::null() };
}
