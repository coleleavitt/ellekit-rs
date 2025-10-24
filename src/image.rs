//! Mach-O image and symbol management
//!
//! This module provides utilities for working with dynam loaded Mach-O images
//! and resolving symbols within them.

use crate::error::{Error, Result};
use core::ffi::{c_char, c_void};
use core::ptr;
use ellekit_sys::{
    mach_header, mach_header_64, LHCloseImage, LHFindSymbols, LHOpenImage, MSCloseImage,
    MSFindSymbol, MSGetImageByName,
};

/// A handle to an open Mach-O image
///
/// This provides RAII management for opened images.
#[derive(Debug)]
pub struct Image {
    header: *mut mach_header,
}

impl Image {
    /// Open a Mach-O image by path
    ///
    /// # Safety
    ///
    /// The path must be a valid null-terminated C string pointing to a valid Mach-O image.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ellekit::image::Image;
    /// use std::ffi::CString;
    ///
    /// unsafe {
    ///     let path = CString::new("/usr/lib/libc.dylib").unwrap();
    ///     let image = Image::open(path.as_ptr())?;
    /// }
    /// # Ok::<(), ellekit::error::Error>(())
    /// ```
    pub unsafe fn open(path: *const c_char) -> Result<Self> {
        if path.is_null() {
            return Err(Error::NullPointer);
        }

        let header = LHOpenImage(path);
        if header.is_null() {
            return Err(Error::Other("Failed to open image"));
        }

        Ok(Image { header })
    }

    /// Open an image by name (Substrate API)
    ///
    /// # Safety
    ///
    /// The name must be a valid null-terminated C string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ellekit::image::Image;
    /// use std::ffi::CString;
    ///
    /// unsafe {
    ///     let name = CString::new("libc.dylib").unwrap();
    ///     let image = Image::open_by_name(name.as_ptr())?;
    /// }
    /// # Ok::<(), ellekit::error::Error>(())
    /// ```
    pub unsafe fn open_by_name(name: *const c_char) -> Result<Self> {
        if name.is_null() {
            return Err(Error::NullPointer);
        }

        let header = MSGetImageByName(name) as *mut mach_header;
        if header.is_null() {
            return Err(Error::Other("Image not found"));
        }

        Ok(Image { header })
    }

    /// Get the raw mach_header pointer
    ///
    /// # Safety
    ///
    /// The returned pointer is only valid while this Image exists.
    pub fn as_ptr(&self) -> *mut mach_header {
        self.header
    }

    /// Find a symbol in this image
    ///
    /// # Safety
    ///
    /// The symbol name must be a valid null-terminated C string.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ellekit::image::Image;
    /// use std::ffi::CString;
    ///
    /// unsafe {
    ///     let image = Image::open_by_name(b"libc.dylib\0".as_ptr() as *const _)?;
    ///     let symbol_name = CString::new("strlen").unwrap();
    ///     let symbol_ptr = image.find_symbol(symbol_name.as_ptr())?;
    /// }
    /// # Ok::<(), ellekit::error::Error>(())
    /// ```
    pub unsafe fn find_symbol(&self, name: *const c_char) -> Result<*mut c_void> {
        if name.is_null() {
            return Err(Error::NullPointer);
        }

        let symbol = MSFindSymbol(self.header as *mut c_void, name);
        if symbol.is_null() {
            return Err(Error::SymbolNotFound);
        }

        Ok(symbol)
    }

    /// Find multiple symbols in this image
    ///
    /// # Safety
    ///
    /// - All symbol names must be valid null-terminated C strings
    /// - The output array must have space for `count` pointers
    ///
    /// # Arguments
    ///
    /// * `symbol_names` - Array of symbol name pointers
    /// * `output` - Array to store found symbol pointers
    /// * `count` - Number of symbols to find
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all symbols were found, `Err` otherwise.
    pub unsafe fn find_symbols(
        &self,
        symbol_names: &[*const c_char],
        output: &mut [*mut c_void],
    ) -> Result<()> {
        if symbol_names.len() != output.len() {
            return Err(Error::Other(
                "Symbol names and output arrays must have same length",
            ));
        }

        let success = LHFindSymbols(
            self.header as *mut mach_header_64,
            symbol_names.as_ptr(),
            output.as_mut_ptr(),
            symbol_names.len(),
        );

        if success {
            Ok(())
        } else {
            Err(Error::SymbolNotFound)
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            LHCloseImage(self.header);
            MSCloseImage(self.header as *mut c_void);
        }
    }
}

/// Find a symbol in any loaded image
///
/// Searches all loaded images for the specified symbol.
///
/// # Safety
///
/// The symbol name must be a valid null-terminated C string.
///
/// # Example
///
/// ```no_run
/// use ellekit::image::find_symbol;
/// use std::ffi::CString;
///
/// unsafe {
///     let name = CString::new("strlen").unwrap();
///     let symbol_ptr = find_symbol(name.as_ptr())?;
/// }
/// # Ok::<(), ellekit::error::Error>(())
/// ```
pub unsafe fn find_symbol(name: *const c_char) -> Result<*mut c_void> {
    if name.is_null() {
        return Err(Error::NullPointer);
    }

    // Pass NULL as image to search all loaded images
    let symbol = MSFindSymbol(ptr::null_mut(), name);
    if symbol.is_null() {
        return Err(Error::SymbolNotFound);
    }

    Ok(symbol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_symbol_null() {
        unsafe {
            let result = find_symbol(ptr::null());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), Error::NullPointer);
        }
    }
}
