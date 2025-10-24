//! Objective-C method hooking

use crate::error::{Error, Result};
use core::ffi::c_char;
use core::ptr;
use ellekit_sys::{
    objc_class, objc_selector, LBHookMessage, MSHookClassPair, MSHookIvar, MSHookMessageEx, IMP,
};

/// A handle to a hooked Objective-C message
#[derive(Debug)]
pub struct MessageHook {
    original: IMP,
}

impl MessageHook {
    /// Hook an Objective-C message using MobileSubstrate API
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - The class and selector must be valid
    /// - The replacement must have the correct signature for the method
    /// - Hooking methods can cause application instability
    ///
    /// # Arguments
    ///
    /// * `class` - Pointer to the Objective-C class
    /// * `selector` - Pointer to the selector
    /// * `replacement` - The replacement IMP (method implementation)
    ///
    /// # Example
    ///
    /// ```no_run
    /// // This requires the objc crate for getting class/selector pointers
    /// // Example pseudocode:
    /// // let cls = objc::class!(NSObject);
    /// // let sel = objc::sel!(description);
    /// // let hook = MessageHook::hook_message_ex(cls, sel, my_impl);
    /// ```
    pub unsafe fn hook_message_ex(
        class: *mut objc_class,
        selector: *const objc_selector,
        replacement: IMP,
    ) -> Result<Self> {
        if class.is_null() || selector.is_null() || replacement.is_null() {
            return Err(Error::NullPointer);
        }

        let mut original: IMP = ptr::null_mut();
        MSHookMessageEx(class, selector, replacement, &mut original);

        if original.is_null() {
            return Err(Error::Other("Message hook failed"));
        }

        Ok(MessageHook { original })
    }

    /// Hook an Objective-C message using libhooker API
    ///
    /// # Safety
    ///
    /// Same safety requirements as `hook_message_ex`.
    pub unsafe fn hook_message(
        class: *mut objc_class,
        selector: *mut objc_selector,
        replacement: IMP,
    ) -> Result<Self> {
        if class.is_null() || selector.is_null() || replacement.is_null() {
            return Err(Error::NullPointer);
        }

        let mut original: IMP = ptr::null_mut();
        LBHookMessage(class, selector, replacement, &mut original);

        if original.is_null() {
            return Err(Error::Other("Message hook failed"));
        }

        Ok(MessageHook { original })
    }

    /// Get the original method implementation
    ///
    /// # Safety
    ///
    /// The caller must ensure the IMP is called with the correct signature.
    pub fn original(&self) -> IMP {
        self.original
    }
}

/// Hook an entire Objective-C class pair
///
/// This replaces all methods from `hook_class` into `target_class`.
///
/// # Safety
///
/// This function is unsafe because:
/// - All three class pointers must be valid Objective-C classes
/// - The hook class must have compatible method signatures with the target
/// - Can cause crashes if method signatures don't match
///
/// # Arguments
///
/// * `target_class` - The class to hook
/// * `hook_class` - The class containing replacement methods
/// * `base_class` - The original base class (usually the target's superclass)
///
/// # Example
///
/// ```no_run
/// // Requires objc crate for class manipulation
/// // unsafe {
/// //     hook_class_pair(target_cls, hook_cls, base_cls)?;
/// // }
/// ```
pub unsafe fn hook_class_pair(
    target_class: *mut objc_class,
    hook_class: *mut objc_class,
    base_class: *mut objc_class,
) -> Result<()> {
    if target_class.is_null() || hook_class.is_null() || base_class.is_null() {
        return Err(Error::NullPointer);
    }

    MSHookClassPair(target_class, hook_class, base_class);
    Ok(())
}

/// Hook an Objective-C instance variable
///
/// Returns a pointer to the instance variable offset.
///
/// # Safety
///
/// This function is unsafe because:
/// - `class` must be a valid Objective-C class
/// - `name` must be a valid null-terminated C string
/// - The returned pointer must be used correctly
///
/// # Arguments
///
/// * `class` - The Objective-C class containing the ivar
/// * `name` - The name of the instance variable
///
/// # Returns
///
/// Returns a pointer to the ivar offset, or `Err` if the ivar doesn't exist.
///
/// # Example
///
/// ```no_run
/// use ellekit::objc::hook_ivar;
/// use std::ffi::CString;
///
/// unsafe {
///     let name = CString::new("_myIvar").unwrap();
///     let ivar_ptr = hook_ivar(class_ptr, name.as_ptr())?;
/// }
/// ```
pub unsafe fn hook_ivar(
    class: *mut objc_class,
    name: *const c_char,
) -> Result<*mut core::ffi::c_void> {
    if class.is_null() || name.is_null() {
        return Err(Error::NullPointer);
    }

    let ivar_ptr = MSHookIvar(class, name);

    if ivar_ptr.is_null() {
        return Err(Error::Other("Instance variable not found"));
    }

    Ok(ivar_ptr)
}

/// Helper for working with Objective-C selectors
pub mod selector {
    use core::ffi::c_char;

    /// Register a selector from a string
    ///
    /// This is a placeholder - in real usage, you'd use the objc runtime.
    /// For actual Objective-C integration, use the `objc` crate.
    ///
    /// # Safety
    ///
    /// The string must be a valid selector name.
    #[allow(dead_code)]
    pub unsafe fn register(_name: *const c_char) -> *mut super::objc_selector {
        // This would call sel_registerName from objc runtime
        // For now, return null as this is just a placeholder
        core::ptr::null_mut()
    }
}

/// Helper for working with Objective-C classes
pub mod class {
    use core::ffi::c_char;

    /// Get a class by name
    ///
    /// This is a placeholder - in real usage, you'd use the objc runtime.
    /// For actual Objective-C integration, use the `objc` crate.
    ///
    /// # Safety
    ///
    /// The string must be a valid class name.
    #[allow(dead_code)]
    pub unsafe fn get(_name: *const c_char) -> *mut super::objc_class {
        // This would call objc_getClass from objc runtime
        // For now, return null as this is just a placeholder
        core::ptr::null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_hook_null_checks() {
        unsafe {
            let result =
                MessageHook::hook_message_ex(ptr::null_mut(), ptr::null(), ptr::null_mut());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), Error::NullPointer);
        }
    }
}
