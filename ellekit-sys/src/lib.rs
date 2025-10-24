//! Raw FFI bindings to ElleKit
//!
//! This crate provides low-level, unsafe Rust bindings to the ElleKit hooking framework.
//! ElleKit is a modern C function and Objective-C method hooking library for iOS and macOS,
//! used by major jailbreaks like Dopamine and palera1n.
//!
//! # System Types
//!
//! All system types (Mach kernel, Objective-C runtime, etc.) are re-exported from `ios-sys`.
//! ElleKit-specific types and functions are manually defined to match the C API.
//!
//! # Safety
//!
//! All functions in this crate are `unsafe` because they directly interact with system memory
//! and can cause undefined behavior if used incorrectly. For a safe, idiomatic Rust API,
//! use the `ellekit` crate instead.
//!
//! # Features
//!
//! - `runtime`: Link against the actual ElleKit library (only available on jailbroken iOS/macOS)
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
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

// ============================================================================
// MARK: - Re-export System Types from ios-sys
// ============================================================================

// Mach kernel types
pub use ios_sys::mach::{
    boolean_t, kern_return_t, mach_msg_type_number_t, mach_port_name_t, mach_port_t,
    mach_vm_address_t, mach_vm_offset_t, mach_vm_size_t, vm_inherit_t, vm_map_t, vm_offset_t,
    vm_prot_t,
};

// Objective-C types
pub use ios_sys::objc::{Class as objc_class, IMP, SEL as objc_selector};

// ============================================================================
// MARK: - Libhooker Types (ElleKit-specific)
// ============================================================================

/// Function hook descriptor for LHHookFunctions
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LHFunctionHook {
    /// Pointer to the function to hook
    pub function: *mut c_void,
    /// Pointer to the replacement function
    pub replacement: *mut c_void,
    /// Pointer to store the original function pointer
    pub oldptr: *mut c_void,
    /// Optional hook options
    pub options: *mut LHFunctionHookOptions,
}

impl Default for LHFunctionHook {
    fn default() -> Self {
        Self {
            function: core::ptr::null_mut(),
            replacement: core::ptr::null_mut(),
            oldptr: core::ptr::null_mut(),
            options: core::ptr::null_mut(),
        }
    }
}

/// Hook options
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LHOptions {
    LHOptionsNone = 0,
    LHOptionsSetJumpReg = 1,
}

/// Function hook options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LHFunctionHookOptions {
    pub options: LHOptions,
    pub jmp_reg: c_int,
}

impl Default for LHFunctionHookOptions {
    fn default() -> Self {
        Self {
            options: LHOptions::LHOptionsNone,
            jmp_reg: 0,
        }
    }
}

/// Libhooker error codes
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LIBHOOKER_ERR {
    LIBHOOKER_OK = 0,
    LIBHOOKER_ERR_SELECTOR_NOT_FOUND = 1,
    LIBHOOKER_ERR_SHORT_FUNC = 2,
    LIBHOOKER_ERR_BAD_INSN_AT_START = 3,
    LIBHOOKER_ERR_VM = 4,
    LIBHOOKER_ERR_NO_SYMBOL = 5,
}

/// Memory patch descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LHMemoryPatch {
    pub destination: *mut c_void,
    pub data: *const c_void,
    pub size: usize,
    pub options: *mut c_void,
}

impl Default for LHMemoryPatch {
    fn default() -> Self {
        Self {
            destination: core::ptr::null_mut(),
            data: core::ptr::null(),
            size: 0,
            options: core::ptr::null_mut(),
        }
    }
}

// ============================================================================
// MARK: - Code Signing Types
// ============================================================================

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CSRange {
    pub location: u64,
    pub length: u64,
}

// Code signing constants
pub const CS_VALID: u32 = 0x0000001;
pub const CS_ADHOC: u32 = 0x0000002;
pub const CS_GET_TASK_ALLOW: u32 = 0x0000004;
pub const CS_INSTALLER: u32 = 0x0000008;
pub const CS_INVALID_ALLOWED: u32 = 0x00000020;
pub const CS_HARD: u32 = 0x0000100;
pub const CS_KILL: u32 = 0x0000200;
pub const CS_CHECK_EXPIRATION: u32 = 0x0000400;
pub const CS_RESTRICT: u32 = 0x0000800;
pub const CS_ENFORCEMENT: u32 = 0x0001000;
pub const CS_REQUIRE_LV: u32 = 0x0002000;
pub const CS_ENTITLEMENTS_VALIDATED: u32 = 0x0004000;
pub const CS_ALLOWED_MACHO: u32 = 0x00ffffe;
pub const CS_EXEC_SET_HARD: u32 = 0x0100000;
pub const CS_EXEC_SET_KILL: u32 = 0x0200000;
pub const CS_EXEC_SET_ENFORCEMENT: u32 = 0x0400000;
pub const CS_EXEC_SET_INSTALLER: u32 = 0x0800000;
pub const CS_KILLED: u32 = 0x1000000;
pub const CS_DYLD_PLATFORM: u32 = 0x2000000;
pub const CS_PLATFORM_BINARY: u32 = 0x4000000;
pub const CS_PLATFORM_PATH: u32 = 0x8000000;
pub const CS_DEBUGGED: u32 = 0x10000000;
pub const CS_SIGNED: u32 = 0x20000000;
pub const CS_DEV_CODE: u32 = 0x40000000;

// csops operations
pub const CS_OPS_STATUS: c_uint = 0;
pub const CS_OPS_MARKINVALID: c_uint = 1;
pub const CS_OPS_MARKHARD: c_uint = 2;
pub const CS_OPS_MARKKILL: c_uint = 3;
pub const CS_OPS_PIDPATH: c_uint = 4;
pub const CS_OPS_CDHASH: c_uint = 5;

// ============================================================================
// MARK: - Mach-O Types
// ============================================================================

// Mach-O header types (opaque pointers for ElleKit API)
pub type mach_header = *mut c_void;
pub type mach_header_64 = *mut c_void;

// ============================================================================
// MARK: - ElleKit C API Functions
// ============================================================================

extern "C" {
    // ========================================================================
    // Libhooker API
    // ========================================================================

    /// Hook Objective-C message
    pub fn LBHookMessage(cls: *mut objc_class, sel: *mut objc_selector, imp: IMP, oldptr: *mut IMP);

    /// Get error string for libhooker error code
    pub fn LHStrError(err: LIBHOOKER_ERR) -> *const c_char;

    /// Patch memory at multiple locations
    pub fn LHPatchMemory(hooks: *const LHMemoryPatch, count: c_int) -> c_int;

    /// Allocate executable memory
    pub fn LHExecMemory(page: *mut *mut c_void, data: *mut c_void, size: usize) -> c_int;

    /// Hook multiple functions in batch (main hooking API)
    pub fn LHHookFunctions(allHooks: *const LHFunctionHook, count: c_int) -> c_int;

    /// Open a Mach-O image by path
    pub fn LHOpenImage(path: *const c_char) -> *mut mach_header;

    /// Close a Mach-O image
    pub fn LHCloseImage(image: *mut mach_header);

    /// Find symbols in a Mach-O image
    pub fn LHFindSymbols(
        image: *mut mach_header_64,
        search: *const *const c_char,
        searchSyms: *mut *mut c_void,
        searchSymCount: usize,
    ) -> bool;

    // ========================================================================
    // MobileSubstrate Compatibility API
    // ========================================================================

    /// Get image by name (Substrate API)
    pub fn MSGetImageByName(name: *const c_char) -> *mut c_void;

    /// Close image (Substrate API)
    pub fn MSCloseImage(image: *mut c_void);

    /// Find symbol in image (Substrate API)
    pub fn MSFindSymbol(image: *mut c_void, name: *const c_char) -> *mut c_void;

    /// Hook a C function (Substrate API)
    pub fn MSHookFunction(symbol: *mut c_void, hook: *mut c_void, old: *mut *mut c_void);

    /// Hook Objective-C message (Substrate API)
    pub fn MSHookMessageEx(
        cls: *mut objc_class,
        sel: *const objc_selector,
        hook: IMP,
        old: *mut IMP,
    );

    /// Hook Objective-C class pair (Substrate API)
    pub fn MSHookClassPair(
        target_class: *mut objc_class,
        hook_class: *mut objc_class,
        base_class: *mut objc_class,
    );

    /// Hook instance variable (Substrate API)
    pub fn MSHookIvar(class: *mut objc_class, name: *const c_char) -> *mut c_void;

    /// Hook memory (Substrate API)
    pub fn MSHookMemory(target: *mut c_void, data: *const c_void, size: usize) -> bool;

    // ========================================================================
    // ElleKit Native API
    // ========================================================================

    /// Enable or disable thread safety (ElleKit native)
    pub fn EKEnableThreadSafety(on: c_int);

    /// Hook function with ElleKit native API
    pub fn EKHookFunction(
        target: *mut c_void,
        replacement: *mut c_void,
        skip_checks: c_int,
    ) -> *mut c_void;

    /// Precision hook - single instruction hook (ElleKit native)
    pub fn EKPrecisionHook(target: *mut c_void, replacement: *mut c_void) -> *mut c_void;

    // ========================================================================
    // PAC (Pointer Authentication) Functions
    // ========================================================================

    /// Sign a pointer with PAC
    pub fn sign_pointer(ptr: *mut c_void) -> *mut c_void;

    /// Strip PAC from a pointer
    pub fn strip_pointer(ptr: *mut c_void) -> *mut c_void;

    /// Sign a pointer as a program counter
    pub fn sign_pc(ptr: *mut c_void) -> *mut c_void;

    // ========================================================================
    // Memory Management
    // ========================================================================

    /// Invalidate instruction cache
    pub fn sys_icache_invalidate(start: *mut c_void, length: usize);

    /// Manual memory copy
    pub fn manual_memcpy(dest: *mut c_void, src: *const c_void, len: usize);

    /// ElleKit raw memory hook function pointer
    pub static EKHookMemoryRaw: Option<
        unsafe extern "C" fn(
            target: *mut c_void,
            data: *const c_void,
            size: usize,
        ) -> kern_return_t,
    >;

    /// Allocate virtual memory
    pub fn mach_vm_allocate(
        target: mach_port_name_t,
        address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        flags: c_int,
    ) -> kern_return_t;

    /// Deallocate virtual memory
    pub fn mach_vm_deallocate(
        target: vm_map_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
    ) -> kern_return_t;

    /// Set memory protection
    pub fn mach_vm_protect(
        task: mach_port_name_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;

    /// Custom memory protection (ElleKit-specific)
    pub fn custom_mach_vm_protect(
        task: mach_port_name_t,
        address: mach_vm_address_t,
        size: mach_vm_size_t,
        set_maximum: boolean_t,
        new_protection: vm_prot_t,
    ) -> kern_return_t;

    /// Write to virtual memory
    pub fn mach_vm_write(
        target_task: vm_map_t,
        address: mach_vm_address_t,
        data: vm_offset_t,
        dataCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    /// Remap virtual memory
    pub fn mach_vm_remap(
        target_task: vm_map_t,
        target_address: *mut mach_vm_address_t,
        size: mach_vm_size_t,
        mask: mach_vm_offset_t,
        flags: c_int,
        src_task: vm_map_t,
        src_address: mach_vm_address_t,
        copy: boolean_t,
        cur_protection: *mut vm_prot_t,
        max_protection: *mut vm_prot_t,
        inheritance: vm_inherit_t,
    ) -> kern_return_t;

    // ========================================================================
    // Code Signing
    // ========================================================================

    /// Perform code signing operations
    pub fn csops(pid: c_int, ops: c_uint, useraddr: *mut c_void, usersize: usize) -> c_int;

    // ========================================================================
    // Utilities
    // ========================================================================

    /// Execute inline assembly
    pub fn assembly(code: *const u8, size: usize) -> *const c_void;

    /// Data memory barrier
    pub fn dmb_sy();

    /// Check shared region
    pub fn shared_region_check(address: *mut u64) -> c_int;
}

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
