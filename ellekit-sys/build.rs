fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=../submodules/ellekit/ellekitc/include/ellekit.h");

    // Link to ElleKit library ONLY if 'runtime' feature is enabled
    // This allows building on non-iOS/macOS systems for development
    #[cfg(feature = "runtime")]
    {
        println!("cargo:rustc-link-lib=dylib=ellekit");

        // Search for the library in common locations
        if cfg!(target_os = "ios") {
            println!("cargo:rustc-link-search=/usr/lib");
        } else if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=/usr/local/lib");
            println!("cargo:rustc-link-search=/opt/ellekit/lib");
        }
    }

    #[cfg(not(feature = "runtime"))]
    {
        // Header-only mode: symbols are declared but not linked
        // This allows building on any platform for development/testing
        println!("cargo:warning=ellekit-sys: Building in header-only mode (no runtime linking)");
        println!("cargo:warning=To link against ElleKit library, enable the 'runtime' feature:");
        println!("cargo:warning=  cargo build --features runtime");
    }

    // Only generate bindings if the feature is enabled
    #[cfg(feature = "generate-bindings")]
    {
        generate_bindings();
    }
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings() {
    use std::env;
    use std::path::PathBuf;

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Add include paths for system headers
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include")
        // Allowlist only the ElleKit-specific items we want
        .allowlist_function("LH.*")
        .allowlist_function("MS.*")
        .allowlist_function("EK.*")
        .allowlist_function("sign_.*")
        .allowlist_function("strip_pointer")
        .allowlist_function("assembly")
        .allowlist_function("sys_icache_invalidate")
        .allowlist_function("manual_memcpy")
        .allowlist_function("dmb_sy")
        .allowlist_function("shared_region_check")
        .allowlist_function("csops")
        .allowlist_type("LH.*")
        .allowlist_type("LIBHOOKER_ERR")
        .allowlist_type("CSRange")
        .allowlist_type("arm_thread_state64")
        .allowlist_type("exception_raise.*")
        .allowlist_var("CS_.*")
        .allowlist_var("EKHookMemoryRaw")
        .allowlist_function("mach_vm_.*")
        .allowlist_function("custom_mach_vm_protect")
        // Generate rust-style enums
        .rustified_enum("LIBHOOKER_ERR")
        .rustified_enum("LHOptions")
        // Derive common traits
        .derive_default(true)
        .derive_debug(true)
        .derive_eq(true)
        .derive_partialeq(true)
        // Use core instead of std for no_std compatibility
        .use_core()
        .ctypes_prefix("::core::ffi")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
