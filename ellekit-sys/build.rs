fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../submodules/ellekit-tbd/libellekit.tbd");

    // Link to ElleKit library ONLY if 'runtime' feature is enabled
    #[cfg(feature = "runtime")]
    {
        let target = std::env::var("TARGET").unwrap();

        if target.contains("ios") {
            // For iOS builds, link against the TBD stub from device dylib
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let tbd_dir = format!("{}/../submodules/ellekit-tbd", manifest_dir);

            println!("cargo:rustc-link-search={}", tbd_dir);
            println!("cargo:rustc-link-lib=dylib=ellekit");

            // Don't link libc/libm - they cause issues with cross-compilation
            println!("cargo:rustc-link-arg=-nostdlib++");
        } else if target.contains("macos") {
            println!("cargo:rustc-link-lib=dylib=ellekit");
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
