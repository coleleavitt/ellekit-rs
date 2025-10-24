fn main() {
    println!("cargo:rerun-if-changed=frameworks.rs");

    // Link to ElleKit library ONLY if 'runtime' feature is enabled
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
        println!("cargo:warning=ellekit-sys: Building in header-only mode (no runtime linking)");
        println!("cargo:warning=To link against ElleKit library, enable the 'runtime' feature");
    }
}
