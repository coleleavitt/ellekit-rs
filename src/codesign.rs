//! Code signing utilities
//!
//! This module provides utilities for working with code signing on iOS/macOS.

use crate::error::{Error, Result};
use core::ffi::c_void;
use ellekit_sys::{csops, CS_ADHOC, CS_OPS_STATUS, CS_PLATFORM_BINARY, CS_SIGNED, CS_VALID};

/// Code signing status flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeSignStatus {
    /// Raw status flags
    pub raw: u32,
}

impl CodeSignStatus {
    /// Check if the code signature is valid
    pub fn is_valid(&self) -> bool {
        self.raw & CS_VALID != 0
    }

    /// Check if this is an ad-hoc signature (no certificate)
    pub fn is_adhoc(&self) -> bool {
        self.raw & CS_ADHOC != 0
    }

    /// Check if the binary is signed
    pub fn is_signed(&self) -> bool {
        self.raw & CS_SIGNED != 0
    }

    /// Check if this is a platform binary (Apple-signed)
    pub fn is_platform_binary(&self) -> bool {
        self.raw & CS_PLATFORM_BINARY != 0
    }
}

/// Get code signing status for the current process
///
/// # Example
///
/// ```no_run
/// use ellekit::codesign::get_codesign_status;
///
/// match get_codesign_status() {
///     Ok(status) => {
///         println!("Valid: {}", status.is_valid());
///         println!("Signed: {}", status.is_signed());
///         println!("Platform: {}", status.is_platform_binary());
///     }
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn get_codesign_status() -> Result<CodeSignStatus> {
    get_codesign_status_for_pid(0) // 0 = current process
}

/// Get code signing status for a specific process
///
/// # Arguments
///
/// * `pid` - Process ID (0 for current process)
///
/// # Example
///
/// ```no_run
/// use ellekit::codesign::get_codesign_status_for_pid;
///
/// match get_codesign_status_for_pid(1234) {
///     Ok(status) => println!("Process is valid: {}", status.is_valid()),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn get_codesign_status_for_pid(pid: i32) -> Result<CodeSignStatus> {
    let mut status: u32 = 0;

    unsafe {
        let ret = csops(
            pid,
            CS_OPS_STATUS,
            &mut status as *mut u32 as *mut c_void,
            core::mem::size_of::<u32>(),
        );

        if ret != 0 {
            return Err(Error::Other("csops failed"));
        }
    }

    Ok(CodeSignStatus { raw: status })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_codesign_status() {
        // This test will only work on macOS/iOS
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            match get_codesign_status() {
                Ok(status) => {
                    // Current process should have some code signing
                    println!("Code signing status: {:?}", status);
                }
                Err(e) => {
                    println!("Note: csops may not be available: {}", e);
                }
            }
        }
    }
}
