//! Integration tests for Mach kernel API bindings

#[test]
fn test_mach_types_accessible() {
    // Just verify the mach module is accessible
    // The actual types are defined by bindgen
    use ios_sys::mach;

    // This test just verifies the module exists
    let _ = core::mem::size_of::<ios_sys::mach::kern_return_t>();
}

#[test]
fn test_mach_module_exists() {
    // Verify we can access the mach module
    use ios_sys::mach::*;

    // Just ensure this compiles - the bindings are generated
    // and we can't assume what constants exist without looking
    // at the actual generated code
}
