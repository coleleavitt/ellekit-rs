//! Shared test utilities for ellekit integration tests
//!
//! This module is not itself a test file (it's tests/common/mod.rs not tests/common.rs)
//! so Cargo won't try to compile it as a test binary.

use std::ffi::{c_char, c_void, CString};
use std::sync::Mutex;

/// Test configuration for platform-specific tests
pub struct TestConfig {
    pub is_jailbroken: bool,
    pub is_ios: bool,
    pub is_macos: bool,
    pub can_hook: bool,
}

impl TestConfig {
    pub fn from_env() -> Self {
        Self {
            is_jailbroken: std::env::var("ELLEKIT_JAILBROKEN").is_ok(),
            is_ios: cfg!(target_os = "ios"),
            is_macos: cfg!(target_os = "macos"),
            can_hook: std::env::var("ELLEKIT_JAILBROKEN").is_ok(),
        }
    }

    /// Check if we can run actual hooking tests
    pub fn can_run_hook_tests(&self) -> bool {
        self.can_hook && (self.is_ios || self.is_macos)
    }
}

/// Mock C functions for testing without actual hooking
pub mod mock_c_functions {
    use super::*;

    static CALL_COUNT: Mutex<usize> = Mutex::new(0);

    /// Mock strlen implementation for testing
    #[no_mangle]
    pub unsafe extern "C" fn mock_strlen(s: *const c_char) -> usize {
        *CALL_COUNT.lock().unwrap() += 1;
        if s.is_null() {
            0
        } else {
            libc::strlen(s)
        }
    }

    /// Reset the call counter
    pub fn reset_call_count() {
        *CALL_COUNT.lock().unwrap() = 0;
    }

    /// Get the call count
    pub fn get_call_count() -> usize {
        *CALL_COUNT.lock().unwrap()
    }
}

/// Helper to create test C strings
pub fn test_cstring(s: &str) -> CString {
    CString::new(s).expect("Failed to create test CString")
}

/// Assert that a pointer is not null and aligned
pub fn assert_valid_function_ptr(ptr: *const c_void) {
    assert!(!ptr.is_null(), "Function pointer is null");
    assert_eq!(
        ptr as usize % std::mem::align_of::<usize>(),
        0,
        "Function pointer is not properly aligned"
    );
}

/// Check if the test environment supports hooking, or skip the test
pub fn check_jailbreak_or_skip(test_name: &str) {
    if std::env::var("ELLEKIT_JAILBROKEN").is_err() {
        println!(
            "SKIPPED [{}]: Requires jailbroken device (set ELLEKIT_JAILBROKEN=1)",
            test_name
        );
        return;
    }
}
