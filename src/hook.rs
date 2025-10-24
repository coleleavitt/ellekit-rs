//! Function hooking APIs

use crate::error::{Error, Result};
use core::ffi::{c_char, c_int, c_void};
use core::ptr;
use ellekit_sys::{
    EKEnableThreadSafety, EKHookFunction, EKPrecisionHook, LHFunctionHook, LHHookFunctions,
    MSFindSymbol, MSHookFunction,
};

/// A handle to a hooked function
///
/// Maintains the original function pointer so you can call the original implementation.
pub struct FunctionHook {
    original: *mut c_void,
}

impl FunctionHook {
    /// Hook a function by symbol name
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - The replacement function must have the same signature as the original
    /// - The original function pointer must be stored and used correctly
    /// - Hooking system functions can cause instability
    ///
    /// # Arguments
    ///
    /// * `symbol_name` - The name of the symbol to hook (e.g., "strlen")
    /// * `replacement` - Pointer to the replacement function
    ///
    /// # Returns
    ///
    /// Returns a `FunctionHook` containing the original function pointer,
    /// or an error if the hook failed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ellekit::hook::FunctionHook;
    /// use std::ffi::c_char;
    ///
    /// static mut ORIG_STRLEN: Option<unsafe extern "C" fn(*const c_char) -> usize> = None;
    ///
    /// unsafe extern "C" fn my_strlen(s: *const c_char) -> usize {
    ///     println!("strlen hooked!");
    ///     ORIG_STRLEN.unwrap()(s)
    /// }
    ///
    /// let hook = unsafe {
    ///     FunctionHook::hook_symbol("strlen", my_strlen as *mut _)
    /// }.expect("Hook failed");
    ///
    /// unsafe {
    ///     ORIG_STRLEN = Some(std::mem::transmute(hook.original()));
    /// }
    /// ```
    pub unsafe fn hook_symbol(symbol_name: &str, replacement: *mut c_void) -> Result<Self> {
        // Convert symbol name to C string
        let mut symbol_bytes = [0u8; 256];
        if symbol_name.len() >= symbol_bytes.len() {
            return Err(Error::Other("Symbol name too long"));
        }

        symbol_bytes[..symbol_name.len()].copy_from_slice(symbol_name.as_bytes());
        symbol_bytes[symbol_name.len()] = 0; // Null terminator

        let symbol_ptr = symbol_bytes.as_ptr() as *const c_char;

        // Find the symbol
        let target = unsafe { MSFindSymbol(ptr::null_mut(), symbol_ptr) };
        if target.is_null() {
            return Err(Error::SymbolNotFound);
        }

        // Hook it
        let mut original = ptr::null_mut();
        MSHookFunction(target, replacement, &mut original);

        if original.is_null() {
            return Err(Error::Other("Hook failed - original is null"));
        }

        Ok(FunctionHook { original })
    }

    /// Hook a function by direct pointer
    ///
    /// # Safety
    ///
    /// Same safety requirements as `hook_symbol`, plus:
    /// - `target` must be a valid function pointer
    ///
    /// # Arguments
    ///
    /// * `target` - Pointer to the function to hook
    /// * `replacement` - Pointer to the replacement function
    pub unsafe fn hook_function(target: *mut c_void, replacement: *mut c_void) -> Result<Self> {
        if target.is_null() {
            return Err(Error::NullPointer);
        }

        let mut original = ptr::null_mut();
        MSHookFunction(target, replacement, &mut original);

        if original.is_null() {
            return Err(Error::Other("Hook failed - original is null"));
        }

        Ok(FunctionHook { original })
    }

    /// Get the original function pointer
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is cast to the correct function signature.
    pub fn original(&self) -> *mut c_void {
        self.original
    }
}

/// Batch function hooking
///
/// Allows hooking multiple functions in a single operation, which can be more efficient.
pub struct FunctionHookBatch {
    hooks: [LHFunctionHook; 32], // Max 32 hooks in a batch
    count: usize,
    originals: [*mut c_void; 32],
}

impl FunctionHookBatch {
    /// Create a new empty batch
    pub fn new() -> Self {
        Self {
            hooks: [LHFunctionHook::default(); 32],
            count: 0,
            originals: [ptr::null_mut(); 32],
        }
    }

    /// Add a function hook to the batch
    ///
    /// # Safety
    ///
    /// Same safety requirements as `FunctionHook::hook_function`.
    ///
    /// # Arguments
    ///
    /// * `target` - Pointer to the function to hook
    /// * `replacement` - Pointer to the replacement function
    ///
    /// # Returns
    ///
    /// Returns the index of this hook in the batch, which can be used
    /// to retrieve the original function pointer after calling `commit()`.
    pub unsafe fn add(&mut self, target: *mut c_void, replacement: *mut c_void) -> Result<usize> {
        if self.count >= self.hooks.len() {
            return Err(Error::Other("Batch full"));
        }

        if target.is_null() || replacement.is_null() {
            return Err(Error::NullPointer);
        }

        let idx = self.count;
        self.hooks[idx] = LHFunctionHook {
            function: target,
            replacement,
            oldptr: &mut self.originals[idx] as *mut _ as *mut c_void,
            options: ptr::null_mut(),
        };

        self.count += 1;
        Ok(idx)
    }

    /// Commit all hooks in the batch
    ///
    /// # Safety
    ///
    /// All added hooks must be valid.
    pub unsafe fn commit(self) -> Result<FunctionHookBatchResult> {
        if self.count == 0 {
            return Err(Error::Other("No hooks in batch"));
        }

        let result = LHHookFunctions(self.hooks.as_ptr(), self.count as c_int);

        if result != 0 {
            return Err(Error::Other("Batch hook failed"));
        }

        Ok(FunctionHookBatchResult {
            originals: self.originals,
            count: self.count,
        })
    }

    /// Get the number of hooks in the batch
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the batch is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl Default for FunctionHookBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a batch hook operation
pub struct FunctionHookBatchResult {
    originals: [*mut c_void; 32],
    count: usize,
}

impl FunctionHookBatchResult {
    /// Get the original function pointer for a hook by index
    ///
    /// # Arguments
    ///
    /// * `index` - The index returned by `FunctionHookBatch::add()`
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is cast to the correct function signature.
    pub fn original(&self, index: usize) -> Option<*mut c_void> {
        if index < self.count {
            Some(self.originals[index])
        } else {
            None
        }
    }
}

/// Enable or disable thread safety in ElleKit
///
/// When thread safety is enabled, ElleKit will suspend all threads before
/// installing hooks to prevent race conditions.
///
/// # Arguments
///
/// * `enabled` - `true` to enable thread safety, `false` to disable
///
/// # Example
///
/// ```no_run
/// use ellekit::hook::set_thread_safety;
///
/// // Enable thread safety for critical hooks
/// set_thread_safety(true);
///
/// // ... install hooks ...
///
/// // Disable for performance
/// set_thread_safety(false);
/// ```
pub fn set_thread_safety(enabled: bool) {
    unsafe {
        EKEnableThreadSafety(if enabled { 1 } else { 0 });
    }
}

/// Hook a function using ElleKit's native API
///
/// This is ElleKit's native hooking function which may provide more control
/// than the Substrate-compatible API.
///
/// # Safety
///
/// Same safety requirements as `FunctionHook::hook_function`, plus:
/// - `skip_checks` controls whether ElleKit skips safety checks (dangerous!)
///
/// # Arguments
///
/// * `target` - Pointer to the function to hook
/// * `replacement` - Pointer to the replacement function
/// * `skip_checks` - Whether to skip safety checks (use with caution!)
///
/// # Example
///
/// ```no_run
/// use ellekit::hook::native_hook;
///
/// unsafe {
///     let orig = native_hook(target_ptr, replacement_ptr, false)?;
/// }
/// # Ok::<(), ellekit::error::Error>(())
/// ```
pub unsafe fn native_hook(
    target: *mut c_void,
    replacement: *mut c_void,
    skip_checks: bool,
) -> Result<*mut c_void> {
    if target.is_null() || replacement.is_null() {
        return Err(Error::NullPointer);
    }

    let original = EKHookFunction(target, replacement, if skip_checks { 1 } else { 0 });

    if original.is_null() {
        return Err(Error::Other("Native hook failed"));
    }

    Ok(original)
}

/// Precision hook - hook at a single instruction
///
/// This is an advanced feature that allows hooking at a specific instruction
/// rather than at a function boundary.
///
/// # Safety
///
/// This is extremely unsafe:
/// - You must understand the exact assembly at the target location
/// - The target must be a valid instruction address
/// - Incorrect use can cause immediate crashes
///
/// # Arguments
///
/// * `target` - Pointer to the instruction to hook
/// * `replacement` - Pointer to the replacement code
///
/// # Example
///
/// ```no_run
/// use ellekit::hook::precision_hook;
///
/// unsafe {
///     // Hook at a specific instruction
///     let orig = precision_hook(instruction_ptr, replacement_ptr)?;
/// }
/// # Ok::<(), ellekit::error::Error>(())
/// ```
pub unsafe fn precision_hook(target: *mut c_void, replacement: *mut c_void) -> Result<*mut c_void> {
    if target.is_null() || replacement.is_null() {
        return Err(Error::NullPointer);
    }

    let original = EKPrecisionHook(target, replacement);

    if original.is_null() {
        return Err(Error::Other("Precision hook failed"));
    }

    Ok(original)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_creation() {
        let batch = FunctionHookBatch::new();
        assert_eq!(batch.len(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_thread_safety_toggle() {
        // Should not panic
        set_thread_safety(true);
        set_thread_safety(false);
    }
}
