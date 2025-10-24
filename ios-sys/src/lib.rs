#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Low-level FFI bindings to iOS frameworks and APIs
//!
//! This crate provides raw bindings to iOS system frameworks including:
//! - Objective-C runtime
//! - Mach kernel APIs
//! - Foundation framework
//! - UIKit framework (when enabled)
//! - Core Foundation
//!
//! # Features
//!
//! - `runtime`: Link against actual iOS frameworks (requires iOS/macOS)
//! - `generate-bindings`: Generate bindings from SDK headers using bindgen
//! - `foundation`: Foundation framework bindings
//! - `uikit`: UIKit framework bindings
//! - `mach`: Mach kernel API bindings (enabled by default)
//!
//! # Build Modes
//!
//! By default, this crate builds in header-only mode, allowing development
//! on any platform. Enable the `runtime` feature to link against actual
//! iOS frameworks on jailbroken iOS/macOS devices.

pub mod objc;
pub mod mach;

#[cfg(feature = "foundation")]
pub mod foundation;

#[cfg(feature = "uikit")]
pub mod uikit;

// Re-export commonly used types
pub use objc::{objc_class, objc_method, objc_object, objc_selector, Class, Method, Ivar, SEL};
pub use mach::{
    kern_return_t, mach_port_t, mach_vm_address_t, mach_vm_size_t, task_t, thread_act_t,
    vm_inherit_t, vm_map_t, vm_prot_t,
};
