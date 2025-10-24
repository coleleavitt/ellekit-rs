//! Raw FFI bindings to ElleKit
//!
//! This crate provides low-level, unsafe Rust bindings to the ElleKit hooking framework.
//! ElleKit is a modern C function and Objective-C method hooking library for iOS and macOS,
//! used by major jailbreaks like Dopamine and palera1n.
//!
//! # Safety
//!
//! All functions in this crate are `unsafe` because they directly interact with system memory
//! and can cause undefined behavior if used incorrectly. For a safe, idiomatic Rust API,
//! use the `ellekit` crate instead.
//!
//! # Features
//!
//! - `generate-bindings`: Generate bindings from headers at build time (requires clang)
//!
//! # Links
//!
//! - [ElleKit Repository](https://github.com/evelyneee/ellekit)
//! - [ElleKit Documentation](https://github.com/evelyneee/ellekit/tree/main/Documentation)

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Pre-generated bindings (used when generate-bindings feature is disabled)
// For now, we'll provide minimal manual bindings until we can generate them
#[cfg(not(feature = "generate-bindings"))]
mod manual_bindings;

#[cfg(not(feature = "generate-bindings"))]
pub use manual_bindings::*;

// Auto-generated bindings (used when generate-bindings feature is enabled)
#[cfg(feature = "generate-bindings")]
include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_libhooker_err_values() {
        // Ensure error codes are defined correctly
        assert_eq!(LIBHOOKER_ERR::LIBHOOKER_OK as i32, 0);
        assert_eq!(LIBHOOKER_ERR::LIBHOOKER_ERR_SELECTOR_NOT_FOUND as i32, 1);
    }
}
