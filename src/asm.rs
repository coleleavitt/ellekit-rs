//! Low-level assembly and synchronization utilities
//!
//! This module provides access to low-level CPU operations like memory barriers
//! and inline assembly execution. These are advanced features for experienced users.

use core::ffi::c_void;
use ellekit_sys::{assembly, dmb_sy, shared_region_check};

/// Execute a data memory barrier
///
/// This ensures that all memory operations before this barrier complete
/// before any memory operations after it begin. Useful for ensuring
/// visibility of memory writes across CPU cores.
///
/// # Example
///
/// ```no_run
/// use ellekit::asm::memory_barrier;
///
/// // Write to shared memory
/// unsafe {
///     *shared_ptr = new_value;
///     memory_barrier(); // Ensure write is visible to other cores
/// }
/// ```
#[inline]
pub fn memory_barrier() {
    unsafe {
        dmb_sy();
    }
}

/// Execute inline assembly code
///
/// # Safety
///
/// This is extremely unsafe:
/// - The code must be valid machine code for the current architecture
/// - The code must not corrupt the stack or registers in unexpected ways
/// - Incorrect use will cause immediate crashes or undefined behavior
///
/// # Arguments
///
/// * `code` - Byte slice containing machine code
///
/// # Returns
///
/// Returns a function pointer to the executed code (if applicable)
///
/// # Example
///
/// ```no_run
/// use ellekit::asm::execute_assembly;
///
/// unsafe {
///     // ARM64 NOP instruction (0xD503201F)
///     let nop = [0x1F, 0x20, 0x03, 0xD5];
///     execute_assembly(&nop);
/// }
/// ```
pub unsafe fn execute_assembly(code: &[u8]) -> *const c_void {
    assembly(code.as_ptr(), code.len())
}

/// Check if an address is in the shared region
///
/// The shared region on iOS/macOS contains shared system libraries
/// that are mapped identically across all processes.
///
/// # Safety
///
/// The address must be a valid pointer.
///
/// # Arguments
///
/// * `address` - Pointer to check
///
/// # Returns
///
/// Returns `true` if the address is in the shared region
///
/// # Example
///
/// ```no_run
/// use ellekit::asm::is_shared_region;
///
/// unsafe {
///     let addr = some_function as *const () as u64;
///     if is_shared_region(addr) {
///         println!("Function is in shared region");
///     }
/// }
/// ```
pub unsafe fn is_shared_region(address: u64) -> bool {
    let mut addr = address;
    let result = shared_region_check(&mut addr);
    result != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_barrier() {
        // Memory barrier should execute without crashing
        memory_barrier();
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn test_shared_region() {
        unsafe {
            // Test with a known library function
            let addr = libc::strlen as *const () as u64;
            // strlen is in libc which is in the shared region
            let in_shared = is_shared_region(addr);
            println!("strlen in shared region: {}", in_shared);
        }
    }
}
