//! Pointer Authentication Code (PAC) utilities
//!
//! PAC is an ARM64e security feature that signs pointers to prevent tampering.
//! These utilities help work with PAC-signed pointers on devices that support it.

use core::ffi::c_void;
use ellekit_sys::{sign_pc, sign_pointer, strip_pointer};

/// Sign a pointer with PAC
///
/// On ARM64e devices with PAC enabled, this signs a pointer.
/// On non-PAC devices, this is a no-op.
///
/// # Safety
///
/// The pointer should be valid. Signing an invalid pointer doesn't make it valid.
///
/// # Example
///
/// ```no_run
/// use ellekit::pac;
///
/// unsafe {
///     let ptr = some_function as *mut _;
///     let signed = pac::sign(ptr);
///     // Use signed pointer
/// }
/// ```
pub unsafe fn sign(ptr: *mut c_void) -> *mut c_void {
    sign_pointer(ptr)
}

/// Strip PAC signature from a pointer
///
/// Removes the PAC signature bits from a pointer, returning the raw address.
/// On non-PAC devices, this is a no-op.
///
/// # Safety
///
/// The returned pointer has no PAC protection. Use carefully.
///
/// # Example
///
/// ```no_run
/// use ellekit::pac;
///
/// unsafe {
///     let signed_ptr = get_some_signed_pointer();
///     let raw_ptr = pac::strip(signed_ptr);
///     // raw_ptr is now unsigned
/// }
/// ```
pub unsafe fn strip(ptr: *mut c_void) -> *mut c_void {
    strip_pointer(ptr)
}

/// Sign a pointer as a program counter (PC)
///
/// Signs a pointer using the PC signing key. This is used for return addresses
/// and function pointers that will be used as jump targets.
///
/// # Safety
///
/// The pointer should be a valid code address.
pub unsafe fn sign_as_pc(ptr: *mut c_void) -> *mut c_void {
    sign_pc(ptr)
}

/// Check if we're likely running on a PAC-enabled device
///
/// This is a heuristic based on the architecture.
/// On ARM64e, PAC is available; on ARM64, it's not.
#[cfg(target_arch = "aarch64")]
pub fn is_pac_available() -> bool {
    // On ARM64e (iPhone XS and later with iOS 12+), PAC is available
    // This is a compile-time check based on the target
    cfg!(target_feature = "paca")
}

#[cfg(not(target_arch = "aarch64"))]
pub fn is_pac_available() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pac_availability() {
        // Just ensure the function doesn't panic
        let _ = is_pac_available();
    }

    #[test]
    fn test_strip_null() {
        unsafe {
            let result = strip(core::ptr::null_mut());
            assert!(result.is_null());
        }
    }
}
