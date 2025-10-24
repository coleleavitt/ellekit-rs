//! Error types for ElleKit operations

use ellekit_sys::LIBHOOKER_ERR;

/// Result type for ElleKit operations
pub type Result<T> = core::result::Result<T, Error>;

/// Error types that can occur when using ElleKit
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The operation completed successfully (shouldn't normally be returned as an error)
    #[error("operation completed successfully")]
    Ok,

    /// Objective-C selector not found
    #[error("Objective-C selector not found")]
    SelectorNotFound,

    /// Function too short to hook safely
    #[error("function too short to hook safely")]
    FunctionTooShort,

    /// Bad instruction at start of function
    #[error("bad instruction at start of function")]
    BadInstructionAtStart,

    /// Virtual memory operation failed
    #[error("virtual memory operation failed")]
    VirtualMemoryError,

    /// Symbol not found in image
    #[error("symbol not found in image")]
    SymbolNotFound,

    /// Null pointer provided where valid pointer expected
    #[error("null pointer provided")]
    NullPointer,

    /// Invalid UTF-8 in string
    #[error("invalid UTF-8 in string")]
    InvalidUtf8,

    /// Other error with custom message
    #[error("{0}")]
    Other(&'static str),
}

impl Error {
    /// Convert from libhooker error code
    pub fn from_libhooker_err(err: LIBHOOKER_ERR) -> Self {
        match err {
            LIBHOOKER_ERR::LIBHOOKER_OK => Self::Ok,
            LIBHOOKER_ERR::LIBHOOKER_ERR_SELECTOR_NOT_FOUND => Self::SelectorNotFound,
            LIBHOOKER_ERR::LIBHOOKER_ERR_SHORT_FUNC => Self::FunctionTooShort,
            LIBHOOKER_ERR::LIBHOOKER_ERR_BAD_INSN_AT_START => Self::BadInstructionAtStart,
            LIBHOOKER_ERR::LIBHOOKER_ERR_VM => Self::VirtualMemoryError,
            LIBHOOKER_ERR::LIBHOOKER_ERR_NO_SYMBOL => Self::SymbolNotFound,
        }
    }

    /// Convert to libhooker error code
    pub fn to_libhooker_err(self) -> LIBHOOKER_ERR {
        match self {
            Self::Ok => LIBHOOKER_ERR::LIBHOOKER_OK,
            Self::SelectorNotFound => LIBHOOKER_ERR::LIBHOOKER_ERR_SELECTOR_NOT_FOUND,
            Self::FunctionTooShort => LIBHOOKER_ERR::LIBHOOKER_ERR_SHORT_FUNC,
            Self::BadInstructionAtStart => LIBHOOKER_ERR::LIBHOOKER_ERR_BAD_INSN_AT_START,
            Self::VirtualMemoryError => LIBHOOKER_ERR::LIBHOOKER_ERR_VM,
            Self::SymbolNotFound => LIBHOOKER_ERR::LIBHOOKER_ERR_NO_SYMBOL,
            _ => LIBHOOKER_ERR::LIBHOOKER_OK, // Default to OK for non-libhooker errors
        }
    }
}

// Re-export for backwards compatibility
#[deprecated(since = "0.2.0", note = "use `Error` instead")]
pub type ElleKitError = Error;
