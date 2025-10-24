//! Integration tests for batch hooking functionality
//!
//! These tests verify the FunctionHookBatch API which allows
//! hooking multiple functions in a single operation.

use ellekit::hook::FunctionHookBatch;
use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

mod common;
use common::{mock_c_functions, test_cstring, TestConfig};

#[test]
fn test_batch_creation() {
    // Test basic batch operations without actual hooking
    let batch = FunctionHookBatch::new();
    assert_eq!(batch.len(), 0, "New batch should be empty");
    assert!(batch.is_empty(), "New batch should report as empty");
}

#[test]
fn test_batch_add_single() {
    // Test adding a single hook to a batch
    unsafe {
        let mut batch = FunctionHookBatch::new();

        // Get a symbol address
        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);

        if !strlen_addr.is_null() {
            let idx = batch
                .add(strlen_addr, mock_c_functions::mock_strlen as *mut c_void)
                .expect("Failed to add strlen to batch");

            assert_eq!(batch.len(), 1, "Batch should contain 1 hook");
            assert_eq!(idx, 0, "First hook should have index 0");
            assert!(!batch.is_empty(), "Batch should not be empty");
        } else {
            println!("Skipping: couldn't find strlen symbol");
        }
    }
}

#[test]
fn test_batch_add_multiple() {
    // Test adding multiple hooks to a batch
    unsafe {
        let mut batch = FunctionHookBatch::new();

        // Get some symbol addresses
        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);
        let strcmp_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strcmp\0".as_ptr() as *const _);

        if !strlen_addr.is_null() && !strcmp_addr.is_null() {
            // Add hooks
            let idx1 = batch
                .add(strlen_addr, mock_c_functions::mock_strlen as *mut c_void)
                .expect("Failed to add strlen to batch");

            // For this test, we'll use mock_strlen for both to avoid defining another mock
            let idx2 = batch
                .add(strcmp_addr, mock_c_functions::mock_strlen as *mut c_void)
                .expect("Failed to add strcmp to batch");

            assert_eq!(batch.len(), 2, "Batch should contain 2 hooks");
            assert_ne!(idx1, idx2, "Hook indices should be different");
        } else {
            println!("Skipping: couldn't find test symbols");
        }
    }
}

#[test]
fn test_batch_null_pointer_rejection() {
    // Test that adding null pointers to batch fails
    unsafe {
        let mut batch = FunctionHookBatch::new();

        let result = batch.add(
            std::ptr::null_mut(),
            mock_c_functions::mock_strlen as *mut c_void,
        );
        assert!(result.is_err(), "Adding null target should fail");

        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);
        if !strlen_addr.is_null() {
            let result = batch.add(strlen_addr, std::ptr::null_mut());
            assert!(result.is_err(), "Adding null replacement should fail");
        }
    }
}

#[test]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), ignore)]
fn test_batch_commit() {
    // Test committing a batch of hooks (requires hooking capability)
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        let mut batch = FunctionHookBatch::new();

        // Add a hook
        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);

        if strlen_addr.is_null() {
            panic!("Could not find strlen symbol");
        }

        let idx = batch
            .add(strlen_addr, mock_c_functions::mock_strlen as *mut c_void)
            .expect("Failed to add to batch");

        // Commit the batch
        let result = batch.commit().expect("Batch commit failed");

        // Verify we can get the original function
        let original = result
            .original(idx)
            .expect("Failed to get original function");
        assert!(
            !original.is_null(),
            "Original function pointer should not be null"
        );
    }
}

#[test]
#[cfg_attr(not(any(target_os = "ios", target_os = "macos")), ignore)]
fn test_batch_hook_multiple_functions() {
    // Test batch hooking multiple functions
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        static STRLEN_CALLS: AtomicUsize = AtomicUsize::new(0);
        static STRCMP_CALLS: AtomicUsize = AtomicUsize::new(0);

        static mut ORIGINAL_STRLEN: Option<unsafe extern "C" fn(*const c_char) -> usize> = None;
        static mut ORIGINAL_STRCMP: Option<
            unsafe extern "C" fn(*const c_char, *const c_char) -> i32,
        > = None;

        unsafe extern "C" fn hooked_strlen(s: *const c_char) -> usize {
            STRLEN_CALLS.fetch_add(1, Ordering::SeqCst);
            ORIGINAL_STRLEN.unwrap()(s)
        }

        unsafe extern "C" fn hooked_strcmp(s1: *const c_char, s2: *const c_char) -> i32 {
            STRCMP_CALLS.fetch_add(1, Ordering::SeqCst);
            ORIGINAL_STRCMP.unwrap()(s1, s2)
        }

        // Create a batch
        let mut batch = FunctionHookBatch::new();

        // Find symbols
        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);
        let strcmp_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strcmp\0".as_ptr() as *const _);

        assert!(!strlen_addr.is_null(), "Could not find strlen");
        assert!(!strcmp_addr.is_null(), "Could not find strcmp");

        // Add hooks to the batch
        let strlen_idx = batch
            .add(strlen_addr, hooked_strlen as *mut c_void)
            .expect("Failed to add strlen to batch");

        let strcmp_idx = batch
            .add(strcmp_addr, hooked_strcmp as *mut c_void)
            .expect("Failed to add strcmp to batch");

        assert_eq!(batch.len(), 2, "Batch should contain 2 hooks");

        // Reset counters
        STRLEN_CALLS.store(0, Ordering::SeqCst);
        STRCMP_CALLS.store(0, Ordering::SeqCst);

        // Commit all hooks at once
        let result = batch.commit().expect("Batch commit failed");

        // Store original function pointers
        ORIGINAL_STRLEN = Some(std::mem::transmute(result.original(strlen_idx).unwrap()));
        ORIGINAL_STRCMP = Some(std::mem::transmute(result.original(strcmp_idx).unwrap()));

        // Test the hooks
        let test_str = test_cstring("test");
        let test_str2 = test_cstring("test");

        let len = libc::strlen(test_str.as_ptr());
        assert_eq!(len, 4);
        assert!(
            STRLEN_CALLS.load(Ordering::SeqCst) > 0,
            "strlen hook not called"
        );

        let cmp = libc::strcmp(test_str.as_ptr(), test_str2.as_ptr());
        assert_eq!(cmp, 0);
        assert!(
            STRCMP_CALLS.load(Ordering::SeqCst) > 0,
            "strcmp hook not called"
        );
    }
}

#[test]
fn test_batch_result_index_bounds() {
    // Test that querying out-of-bounds indices returns None
    let config = TestConfig::from_env();
    if !config.can_run_hook_tests() {
        println!("Skipping: requires jailbroken iOS/macOS device (set ELLEKIT_JAILBROKEN=1)");
        return;
    }

    unsafe {
        let mut batch = FunctionHookBatch::new();

        let strlen_addr = libc::dlsym(libc::RTLD_DEFAULT, b"strlen\0".as_ptr() as *const _);
        if strlen_addr.is_null() {
            println!("Skipping: couldn't find strlen");
            return;
        }

        batch
            .add(strlen_addr, mock_c_functions::mock_strlen as *mut c_void)
            .expect("Failed to add to batch");

        if let Ok(result) = batch.commit() {
            // Valid index should return Some
            assert!(result.original(0).is_some());

            // Out of bounds indices should return None
            assert!(result.original(1).is_none());
            assert!(result.original(999).is_none());
        }
    }
}
