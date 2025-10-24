//! Safe Rust bindings to ElleKit
//!
//! ElleKit is a modern hooking framework for iOS and macOS that provides:
//! - C function hooking using direct memory patching
//! - Objective-C method hooking
//! - ARM64/ARM64e support with Pointer Authentication (PAC)
//! - Used by major jailbreaks: Dopamine, palera1n, meowbrek2
//!
//! # Features
//!
//! This crate provides safe, idiomatic Rust wrappers around the low-level ElleKit C API.
//!
//! # Examples
//!
//! ## Hooking a C function
//!
//! ```rust,no_run
//! use ellekit::hook::FunctionHook;
//! use std::ffi::c_char;
//!
//! // Original strlen
//! static mut ORIGINAL_STRLEN: Option<unsafe extern "C" fn(*const c_char) -> usize> = None;
//!
//! // Our hook
//! unsafe extern "C" fn hooked_strlen(s: *const c_char) -> usize {
//!     println!("strlen called!");
//!     ORIGINAL_STRLEN.unwrap()(s)
//! }
//!
//! // Install the hook
//! let hook = unsafe {
//!     FunctionHook::hook_symbol(
//!         "strlen",
//!         hooked_strlen as *mut _,
//!     )
//! }.expect("Failed to hook strlen");
//!
//! unsafe {
//!     ORIGINAL_STRLEN = Some(std::mem::transmute(hook.original()));
//! }
//! ```
//!
//! ## Hooking an Objective-C method
//!
//! ```rust,no_run
//! use ellekit::objc::MessageHook;
//!
//! // Hook -[NSObject description]
//! // (Example - actual implementation requires objc runtime integration)
//! ```

#![cfg_attr(not(test), no_std)]

pub mod asm;
pub mod codesign;
pub mod error;
pub mod hook;
pub mod image;
pub mod memory;
pub mod objc;
pub mod pac;
pub mod util;

// Re-export commonly used types
pub use error::{Error, Result};
pub use hook::{FunctionHook, FunctionHookBatch};
pub use image::Image;

/// ElleKit version information
pub mod version {
    /// Get the crate version
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    /// Get the crate name
    pub const NAME: &str = env!("CARGO_PKG_NAME");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version::VERSION.is_empty());
        assert_eq!(version::NAME, "ellekit");
    }
}
