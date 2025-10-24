//! Objective-C runtime bindings
//!
//! Low-level bindings to the Objective-C runtime API from <objc/runtime.h>
//! and <objc/message.h>.

use core::ffi::{c_char, c_int, c_uint, c_void};

// Opaque types
#[repr(C)]
pub struct objc_class {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_method {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_ivar {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_category {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_property {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objc_method_description {
    pub name: SEL,
    pub types: *const c_char,
}

// Type aliases
pub type id = *mut objc_object;
pub type Class = *mut objc_class;
pub type Method = *mut objc_method;
pub type Ivar = *mut objc_ivar;
pub type Category = *mut objc_category;
pub type objc_property_t = *mut objc_property;
pub type SEL = *const objc_selector;
pub type IMP = extern "C" fn();

#[repr(C)]
pub struct objc_selector {
    _private: [u8; 0],
}

// Protocol type
#[repr(C)]
pub struct Protocol {
    _private: [u8; 0],
}

// Asociative references policy
pub type objc_AssociationPolicy = c_uint;
pub const OBJC_ASSOCIATION_ASSIGN: objc_AssociationPolicy = 0;
pub const OBJC_ASSOCIATION_RETAIN_NONATOMIC: objc_AssociationPolicy = 1;
pub const OBJC_ASSOCIATION_COPY_NONATOMIC: objc_AssociationPolicy = 3;
pub const OBJC_ASSOCIATION_RETAIN: objc_AssociationPolicy = 0o1401;
pub const OBJC_ASSOCIATION_COPY: objc_AssociationPolicy = 0o1403;

extern "C" {
    // Working with Classes
    pub fn objc_getClass(name: *const c_char) -> Class;
    pub fn objc_getMetaClass(name: *const c_char) -> Class;
    pub fn objc_lookUpClass(name: *const c_char) -> Class;
    pub fn objc_getRequiredClass(name: *const c_char) -> Class;
    pub fn objc_getClassList(buffer: *mut Class, bufferCount: c_int) -> c_int;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut Class;

    // Working with Class Instances
    pub fn class_getName(cls: Class) -> *const c_char;
    pub fn class_getSuperclass(cls: Class) -> Class;
    pub fn class_isMetaClass(cls: Class) -> bool;
    pub fn class_getInstanceSize(cls: Class) -> usize;
    pub fn class_getInstanceVariable(cls: Class, name: *const c_char) -> Ivar;
    pub fn class_getClassVariable(cls: Class, name: *const c_char) -> Ivar;
    pub fn class_addIvar(
        cls: Class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> bool;
    pub fn class_copyIvarList(cls: Class, outCount: *mut c_uint) -> *mut Ivar;

    // Working with Methods
    pub fn class_getInstanceMethod(cls: Class, name: SEL) -> Method;
    pub fn class_getClassMethod(cls: Class, name: SEL) -> Method;
    pub fn class_copyMethodList(cls: Class, outCount: *mut c_uint) -> *mut Method;
    pub fn class_addMethod(cls: Class, name: SEL, imp: IMP, types: *const c_char) -> bool;
    pub fn class_replaceMethod(cls: Class, name: SEL, imp: IMP, types: *const c_char) -> IMP;
    pub fn class_getMethodImplementation(cls: Class, name: SEL) -> IMP;
    pub fn class_respondsToSelector(cls: Class, sel: SEL) -> bool;

    // Working with Protocols
    pub fn class_addProtocol(cls: Class, protocol: *const Protocol) -> bool;
    pub fn class_conformsToProtocol(cls: Class, protocol: *const Protocol) -> bool;
    pub fn class_copyProtocolList(cls: Class, outCount: *mut c_uint) -> *mut *const Protocol;

    // Working with Properties
    pub fn class_getProperty(cls: Class, name: *const c_char) -> objc_property_t;
    pub fn class_copyPropertyList(cls: Class, outCount: *mut c_uint) -> *mut objc_property_t;
    pub fn class_addProperty(
        cls: Class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    ) -> bool;
    pub fn class_replaceProperty(
        cls: Class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    );

    // Creating Classes
    pub fn objc_allocateClassPair(
        superclass: Class,
        name: *const c_char,
        extraBytes: usize,
    ) -> Class;
    pub fn objc_registerClassPair(cls: Class);
    pub fn objc_disposeClassPair(cls: Class);

    // Working with Selectors
    pub fn sel_registerName(str: *const c_char) -> SEL;
    pub fn sel_getName(sel: SEL) -> *const c_char;
    pub fn sel_isEqual(lhs: SEL, rhs: SEL) -> bool;

    // Working with Methods
    pub fn method_getName(m: Method) -> SEL;
    pub fn method_getImplementation(m: Method) -> IMP;
    pub fn method_getTypeEncoding(m: Method) -> *const c_char;
    pub fn method_setImplementation(m: Method, imp: IMP) -> IMP;
    pub fn method_exchangeImplementations(m1: Method, m2: Method);

    // Working with Instance Variables
    pub fn ivar_getName(v: Ivar) -> *const c_char;
    pub fn ivar_getTypeEncoding(v: Ivar) -> *const c_char;
    pub fn ivar_getOffset(v: Ivar) -> isize;

    // Working with Objects
    pub fn object_getClass(obj: id) -> Class;
    pub fn object_setClass(obj: id, cls: Class) -> Class;
    pub fn object_getClassName(obj: id) -> *const c_char;
    pub fn object_getIvarValue(obj: id, ivar: Ivar) -> id;
    pub fn object_setIvarValue(obj: id, ivar: Ivar, value: id);

    // Associative References
    pub fn objc_setAssociatedObject(object: id, key: *const c_void, value: id, policy: objc_AssociationPolicy);
    pub fn objc_getAssociatedObject(object: id, key: *const c_void) -> id;
    pub fn objc_removeAssociatedObjects(object: id);

    // Message sending (objc/message.h)
    pub fn objc_msgSend();
    pub fn objc_msgSendSuper();
    pub fn objc_msgSend_stret();
    pub fn objc_msgSendSuper_stret();

    // Memory management
    pub fn objc_retain(obj: id) -> id;
    pub fn objc_release(obj: id);
    pub fn objc_autorelease(obj: id) -> id;
}

#[repr(C)]
pub struct objc_property_attribute_t {
    pub name: *const c_char,
    pub value: *const c_char,
}

#[repr(C)]
pub struct objc_super {
    pub receiver: id,
    pub super_class: Class,
}
