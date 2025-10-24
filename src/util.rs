//! Utility functions for ElleKit
//!
//! This module provides helper functions for working with ElleKit errors
//! and other common operations.

use crate::error::Error;
use ellekit_sys::{LHStrError, LIBHOOKER_ERR};

/// Get a human-readable error message for a libhooker error code
///
/// # Safety
///
/// This function calls the C function `LHStrError` which returns a static string.
/// The returned string is valid for the lifetime of the program.
///
/// # Example
///
/// ```no_run
/// use ellekit::util::error_string;
/// use ellekit_sys::LIBHOOKER_ERR;
///
/// let msg = error_string(LIBHOOKER_ERR::LIBHOOKER_ERR_SHORT_FUNC);
/// println!("Error: {}", msg);
/// ```
pub fn error_string(err: LIBHOOKER_ERR) -> &'static str {
    unsafe {
        let c_str = LHStrError(err);
        if c_str.is_null() {
            return "Unknown error";
        }

        // Convert C string to Rust str
        let bytes = core::ffi::CStr::from_ptr(c_str).to_bytes();
        core::str::from_utf8(bytes).unwrap_or("Invalid UTF-8 in error message")
    }
}

/// Get a human-readable error message for an Error
///
/// This is a convenience wrapper around `Error::message()` that
/// also handles libhooker-specific errors via `LHStrError`.
///
/// # Example
///
/// ```no_run
/// use ellekit::error::Error;
/// use ellekit::util::error_message;
///
/// let err = Error::FunctionTooShort;
/// println!("Error: {}", error_message(&err));
/// ```
pub fn error_message(err: &Error) -> &str {
    // For libhooker-specific errors, try to get the official message
    match err {
        Error::SelectorNotFound
        | Error::FunctionTooShort
        | Error::BadInstructionAtStart
        | Error::VirtualMemoryError
        | Error::SymbolNotFound => {
            let lh_err = err.to_libhooker_err();
            error_string(lh_err)
        }
        // For other errors, use the Display impl from thiserror
        Error::Other(msg) => msg,
        _ => "unknown error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_string() {
        // Test that we can get error strings for libhooker errors
        let msg = error_string(LIBHOOKER_ERR::LIBHOOKER_OK);
        assert!(!msg.is_empty());

        let msg = error_string(LIBHOOKER_ERR::LIBHOOKER_ERR_SHORT_FUNC);
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_error_message() {
        let err = Error::FunctionTooShort;
        let msg = error_message(&err);
        assert!(!msg.is_empty());

        let err = Error::NullPointer;
        let msg = error_message(&err);
        assert_eq!(msg, "Null pointer provided");
    }
}
