//! Memory management and patching utilities

use crate::error::{Error, Result};
use core::ffi::{c_int, c_void};
use core::ptr;
use ellekit_sys::{
    kern_return_t, mach_vm_address_t, mach_vm_allocate, mach_vm_protect, mach_vm_size_t,
    mach_vm_write, manual_memcpy, sys_icache_invalidate, vm_prot_t, LHMemoryPatch, LHPatchMemory,
    MSHookMemory,
};

// Mach return codes
const KERN_SUCCESS: kern_return_t = 0;

// VM protection flags
pub const VM_PROT_NONE: vm_prot_t = 0x00;
pub const VM_PROT_READ: vm_prot_t = 0x01;
pub const VM_PROT_WRITE: vm_prot_t = 0x02;
pub const VM_PROT_EXECUTE: vm_prot_t = 0x04;

/// Patch memory at a specific address
///
/// # Safety
///
/// This function is extremely unsafe:
/// - `destination` must be a valid, writable memory address
/// - `data` must point to valid memory of at least `size` bytes
/// - Patching arbitrary memory can cause crashes or security issues
pub unsafe fn patch_memory(destination: *mut c_void, data: &[u8]) -> Result<()> {
    if destination.is_null() {
        return Err(Error::NullPointer);
    }

    let success = MSHookMemory(destination, data.as_ptr() as *const c_void, data.len());

    if success {
        Ok(())
    } else {
        Err(Error::VirtualMemoryError)
    }
}

/// Batch patch multiple memory locations
///
/// # Safety
///
/// Same safety requirements as `patch_memory`, applied to all patches.
pub unsafe fn patch_memory_batch(patches: &[MemoryPatch]) -> Result<()> {
    if patches.is_empty() {
        return Ok(());
    }

    if patches.len() > 32 {
        return Err(Error::Other("Too many patches in batch (max 32)"));
    }

    let mut lh_patches: [LHMemoryPatch; 32] = [LHMemoryPatch::default(); 32];

    for (i, patch) in patches.iter().enumerate() {
        lh_patches[i] = LHMemoryPatch {
            destination: patch.destination,
            data: patch.data,
            size: patch.size,
            options: ptr::null_mut(),
        };
    }

    let result = LHPatchMemory(lh_patches.as_ptr(), patches.len() as c_int);

    if result == 0 {
        Ok(())
    } else {
        Err(Error::VirtualMemoryError)
    }
}

/// A memory patch descriptor
#[derive(Debug, Clone, Copy)]
pub struct MemoryPatch {
    pub destination: *mut c_void,
    pub data: *const c_void,
    pub size: usize,
}

/// Allocate virtual memory
///
/// # Safety
///
/// The allocated memory must be properly managed and eventually freed.
pub unsafe fn allocate_memory(size: usize) -> Result<*mut c_void> {
    let mut address: mach_vm_address_t = 0;
    let result = mach_vm_allocate(
        -1i32 as u32, // mach_task_self()
        &mut address,
        size as mach_vm_size_t,
        1, // VM_FLAGS_ANYWHERE
    );

    if result == KERN_SUCCESS {
        Ok(address as *mut c_void)
    } else {
        Err(Error::VirtualMemoryError)
    }
}

/// Change memory protection flags
///
/// # Safety
///
/// - `address` must be a valid memory address
/// - Changing protection can enable writing to read-only memory or executing data
pub unsafe fn protect_memory(
    address: *mut c_void,
    size: usize,
    protection: vm_prot_t,
) -> Result<()> {
    let result = mach_vm_protect(
        -1i32 as u32, // mach_task_self()
        address as mach_vm_address_t,
        size as mach_vm_size_t,
        0, // set_maximum = false
        protection,
    );

    if result == KERN_SUCCESS {
        Ok(())
    } else {
        Err(Error::VirtualMemoryError)
    }
}

/// Write to virtual memory
///
/// # Safety
///
/// - `address` must be a valid, writable memory address
/// - `data` must be valid for reads of `len` bytes
pub unsafe fn write_memory(address: *mut c_void, data: &[u8]) -> Result<()> {
    let result = mach_vm_write(
        -1i32 as u32, // mach_task_self()
        address as mach_vm_address_t,
        data.as_ptr() as usize,
        data.len() as u32,
    );

    if result == KERN_SUCCESS {
        Ok(())
    } else {
        Err(Error::VirtualMemoryError)
    }
}

/// Manually copy memory (bypasses some protections)
///
/// # Safety
///
/// - Both pointers must be valid
/// - Ranges must not overlap (use `memmove` for overlapping ranges)
/// - Destination must be writable
pub unsafe fn copy_memory(dest: *mut c_void, src: *const c_void, len: usize) {
    manual_memcpy(dest, src, len);
}

/// Invalidate instruction cache
///
/// This must be called after modifying executable code to ensure the CPU
/// sees the updated instructions.
///
/// # Safety
///
/// - `start` must point to valid memory
/// - `length` must not exceed the valid memory region
pub unsafe fn invalidate_icache(start: *mut c_void, length: usize) {
    sys_icache_invalidate(start, length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protection_constants() {
        assert_eq!(VM_PROT_NONE, 0);
        assert_eq!(VM_PROT_READ, 1);
        assert_eq!(VM_PROT_WRITE, 2);
        assert_eq!(VM_PROT_EXECUTE, 4);
    }

    #[test]
    fn test_patch_memory_null() {
        unsafe {
            let result = patch_memory(ptr::null_mut(), &[0u8; 4]);
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), Error::NullPointer);
        }
    }
}
