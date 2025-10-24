//! Foundation framework bindings
//!
//! Minimal bindings to Foundation framework types and functions.
//! These are commonly needed for iOS development and hooking.

use crate::objc::{id, Class, SEL};
use core::ffi::{c_char, c_double, c_int, c_long, c_uchar, c_uint, c_void};

// NSInteger and NSUInteger
#[cfg(target_pointer_width = "64")]
pub type NSInteger = c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = c_uint;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = c_uint;

// CGFloat
#[cfg(target_pointer_width = "64")]
pub type CGFloat = c_double;
#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;

// NSRange
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    pub fn new(location: NSUInteger, length: NSUInteger) -> Self {
        NSRange { location, length }
    }
}

// NSPoint / CGPoint
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

pub type CGPoint = NSPoint;

// NSSize / CGSize
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

pub type CGSize = NSSize;

// NSRect / CGRect
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

pub type CGRect = NSRect;

// NSComparisonResult
pub type NSComparisonResult = NSInteger;
pub const NSOrderedAscending: NSComparisonResult = -1;
pub const NSOrderedSame: NSComparisonResult = 0;
pub const NSOrderedDescending: NSComparisonResult = 1;

// NSStringEncoding
pub type NSStringEncoding = NSUInteger;
pub const NSASCIIStringEncoding: NSStringEncoding = 1;
pub const NSUTF8StringEncoding: NSStringEncoding = 4;
pub const NSUTF16StringEncoding: NSStringEncoding = 10;
pub const NSUTF32StringEncoding: NSStringEncoding = 0x8c000100;

// NSNotificationName
pub type NSNotificationName = id;

extern "C" {
    // NSString
    pub static _NSConcreteGlobalBlock: *mut c_void;
    pub static _NSConcreteStackBlock: *mut c_void;

    // Common string constants
    pub static NSDefaultRunLoopMode: id;
    pub static NSRunLoopCommonModes: id;
}

// Block support
#[repr(C)]
pub struct Block_descriptor_1 {
    pub reserved: c_uint,
    pub size: c_uint,
}

#[repr(C)]
pub struct Block_literal_1 {
    pub isa: *mut c_void,
    pub flags: c_int,
    pub reserved: c_int,
    pub invoke: *mut c_void,
    pub descriptor: *mut Block_descriptor_1,
}

// NSTimeInterval
pub type NSTimeInterval = c_double;

// Common selector names (can be used with sel_registerName)
pub const SELECTOR_INIT: &[u8] = b"init\0";
pub const SELECTOR_ALLOC: &[u8] = b"alloc\0";
pub const SELECTOR_NEW: &[u8] = b"new\0";
pub const SELECTOR_DEALLOC: &[u8] = b"dealloc\0";
pub const SELECTOR_RETAIN: &[u8] = b"retain\0";
pub const SELECTOR_RELEASE: &[u8] = b"release\0";
pub const SELECTOR_AUTORELEASE: &[u8] = b"autorelease\0";
pub const SELECTOR_CLASS: &[u8] = b"class\0";
pub const SELECTOR_DESCRIPTION: &[u8] = b"description\0";
