//! Integration tests for FunctionHook API
//!
//! These tests verify the function hooking capabilities of ellekit.
//! Tests that require actual hooking are conditionally compiled and
//! require setting ELLEKIT_JAILBROKEN=1 environment variable.

use ellekit::hook::FunctionHook;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

// Import our common test helpers
mod common;
use common::{assert_valid_function_ptr, mock_c_functions, test_cstring, TestConfig};

#[test]
fn test_hook_creation_requires_valid_pointer() {
    // This test doesn't require actual hooking, just API validation
    unsafe {
        let null_ptr = std::ptr::null_mut();

        // Attempting to hook a null pointer should fail
        let result =
            FunctionHook::hook_function(null_ptr, mock_c_functions::mock_strlen as *mut c_void);
        assert!(result.is_err(), "Hooking null pointer should fail");
    }
}

#[test]
fn test_hook_symbol_not_found() {
    // Test error handling for non-existent symbols
    unsafe {
        let result = FunctionHook::hook_symbol(
            "this_symbol_definitely_does_not_exist_123456",
            mock_c_functions::mock_strlen as *mut _,
        );

        assert!(result.is_err(), "Hooking non-existent symbol should fail");
    }
}

#[test]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), ignore)]
fn test_strlen_hook_integration() {
    // This test requires actual hooking capability
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        // Storage for original function and hook state
        static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_char) -> usize> = None;
        static HOOK_CALLED: AtomicUsize = AtomicUsize::new(0);

        // Our hook function
        unsafe extern "C" fn hooked_strlen(s: *const c_char) -> usize {
            HOOK_CALLED.fetch_add(1, Ordering::SeqCst);

            match ORIGINAL {
                Some(original) => original(s),
                None => 0,
            }
        }

        // Hook strlen
        let hook = match FunctionHook::hook_symbol("strlen", hooked_strlen as *mut _) {
            Ok(hook) => hook,
            Err(e) => {
                eprintln!("Failed to hook strlen: {}", e);
                panic!("Hook failed on platform that should support it");
            }
        };

        ORIGINAL = Some(std::mem::transmute(hook.original()));
        HOOK_CALLED.store(0, Ordering::SeqCst);

        // Test the hook
        let test_str = test_cstring("Hello, ElleKit!");
        let len = libc::strlen(test_str.as_ptr());

        assert!(
            HOOK_CALLED.load(Ordering::SeqCst) > 0,
            "Hook was not called"
        );
        assert_eq!(len, 15, "strlen returned incorrect length");

        // Test with empty string
        let empty = test_cstring("");
        let empty_len = libc::strlen(empty.as_ptr());
        assert_eq!(empty_len, 0);

        // Clean up
        drop(hook);
    }
}

#[test]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), ignore)]
fn test_hook_original_pointer_validity() {
    // Test that original function pointers are valid
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        if let Ok(hook) =
            FunctionHook::hook_symbol("strlen", mock_c_functions::mock_strlen as *mut _)
        {
            let original = hook.original();

            // Verify the pointer is valid
            assert_valid_function_ptr(original);

            // Verify we can call through it
            let test_str = test_cstring("test");
            let original_fn: unsafe extern "C" fn(*const c_char) -> usize =
                std::mem::transmute(original);
            let len = original_fn(test_str.as_ptr());
            assert_eq!(len, 4);

            drop(hook);
        } else {
            panic!("Failed to hook strlen on compatible platform");
        }
    }
}

#[test]
fn test_hook_api_basic_validation() {
    // This test just validates the API without requiring hooking
    unsafe {
        // Attempt to hook
        let result = FunctionHook::hook_symbol("strlen", mock_c_functions::mock_strlen as *mut _);

        match result {
            Ok(hook) => {
                // If hooking succeeded, verify original is valid
                assert!(!hook.original().is_null());
                drop(hook);
            }
            Err(_) => {
                // If hooking failed, that's expected on non-jailbroken devices
                println!("Hooking not available (expected on non-jailbroken platforms)");
            }
        }
    }
}

#[test]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), ignore)]
fn test_hook_function_by_pointer() {
    // Test hooking by function pointer instead of symbol name
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        // Get strlen address via dlsym
        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);

        if strlen_addr.is_null() {
            println!("Could not find strlen symbol via dlsym");
            return;
        }

        static mut ORIGINAL: Option<unsafe extern "C" fn(*const c_char) -> usize> = None;

        unsafe extern "C" fn hooked(s: *const c_char) -> usize {
            ORIGINAL.unwrap()(s)
        }

        let hook = FunctionHook::hook_function(strlen_addr, hooked as *mut c_void)
            .expect("Failed to hook by pointer");

        ORIGINAL = Some(std::mem::transmute(hook.original()));

        // Test it works
        let test_str = test_cstring("test");
        let len = libc::strlen(test_str.as_ptr());
        assert_eq!(len, 4);

        drop(hook);
    }
}
