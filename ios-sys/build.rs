use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let _target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let _target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Find iOS SDK from submodules
    let sdk_path = find_ios_sdk();

    if let Some(sdk) = &sdk_path {
        println!("cargo:warning=ios-sys: Using iOS SDK: {}", sdk.display());
        setup_linking(sdk);
    } else {
        println!("cargo:warning=ios-sys: No iOS SDK found - building header-only");
        println!("cargo:warning=To use Theos SDKs, clone them into submodules/sdks/");
    }

    // Only link frameworks if 'runtime' feature is enabled
    #[cfg(feature = "runtime")]
    {
        if sdk_path.is_some() {
            link_frameworks();
        }
    }

    #[cfg(not(feature = "runtime"))]
    {
        println!("cargo:warning=ios-sys: Header-only mode (enable 'runtime' feature to link)");
    }

    // Generate bindings if requested
    #[cfg(feature = "generate-bindings")]
    {
        if let Some(sdk) = &sdk_path {
            generate_bindings(sdk);
        }
    }
}

/// Find iOS SDK in submodules
fn find_ios_sdk() -> Option<PathBuf> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let repo_root = manifest_dir.parent().unwrap();
    let sdk_dir = repo_root.join("submodules/sdks");

    if !sdk_dir.exists() {
        return None;
    }

    // Try to find the most recent iOS SDK
    let sdk_candidates = vec![
        "iPhoneOS16.5.sdk",
        "iPhoneOS15.6.sdk",
        "iPhoneOS14.5.sdk",
        "iPhoneOS13.7.sdk",
        "iPhoneOS12.4.sdk",
        "iPhoneOS11.4.sdk",
        "iPhoneOS10.3.sdk",
    ];

    for sdk_name in sdk_candidates {
        let sdk_path = sdk_dir.join(sdk_name);
        if sdk_path.exists() {
            return Some(sdk_path);
        }
    }

    None
}

/// Setup linking paths for iOS SDK
fn setup_linking(sdk_path: &Path) {
    // Add framework search paths
    println!(
        "cargo:rustc-link-search=framework={}/System/Library/Frameworks",
        sdk_path.display()
    );
    println!(
        "cargo:rustc-link-search=framework={}/System/Library/PrivateFrameworks",
        sdk_path.display()
    );

    // Add library search paths
    println!("cargo:rustc-link-search={}/usr/lib", sdk_path.display());
    println!(
        "cargo:rustc-link-search={}/usr/lib/system",
        sdk_path.display()
    );

    // Set sysroot for clang (if generating bindings)
    println!("cargo:rustc-env=IPHONEOS_SDK_PATH={}", sdk_path.display());
}

/// Link iOS frameworks based on enabled features
#[allow(dead_code)]
fn link_frameworks() {
    // Core frameworks (always needed)
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    // Mach/system libraries
    println!("cargo:rustc-link-lib=dylib=System");

    // Optional frameworks based on features
    #[cfg(feature = "uikit")]
    {
        println!("cargo:rustc-link-lib=framework=UIKit");
    }

    #[cfg(feature = "coreanimation")]
    {
        println!("cargo:rustc-link-lib=framework=CoreAnimation");
    }

    #[cfg(feature = "quartzcore")]
    {
        println!("cargo:rustc-link-lib=framework=QuartzCore");
    }

    #[cfg(feature = "security")]
    {
        println!("cargo:rustc-link-lib=framework=Security");
    }
}

/// Generate bindings using bindgen
#[cfg(feature = "generate-bindings")]
fn generate_bindings(sdk_path: &Path) {

    let sysroot = format!("-isysroot{}", sdk_path.display());
    let framework_path = format!("-F{}/System/Library/Frameworks", sdk_path.display());
    let include_path = format!("-I{}/usr/include", sdk_path.display());

    // Generate Foundation bindings
    generate_framework_bindings(
        "Foundation",
        sdk_path,
        &[&sysroot, &framework_path, &include_path],
    );

    // Generate Objective-C runtime bindings
    generate_objc_bindings(sdk_path, &[&sysroot, &include_path]);

    // Generate Mach bindings
    generate_mach_bindings(sdk_path, &[&sysroot, &include_path]);
}

#[cfg(feature = "generate-bindings")]
fn generate_framework_bindings(
    framework: &str,
    sdk_path: &Path,
    clang_args: &[&str],
) {
    let header = format!("{}/{}.h", framework, framework);

    let bindings = bindgen::Builder::default()
        .header_contents(&header, &format!("#import <{}>", header))
        .clang_args(clang_args)
        .allowlist_type(&format!("{}.*", framework))
        .allowlist_function(&format!("{}.*", framework))
        .allowlist_var(&format!("{}.*", framework))
        .use_core()
        .ctypes_prefix("::core::ffi")
        .generate()
        .expect(&format!("Unable to generate {} bindings", framework));

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join(format!("{}.rs", framework.to_lowercase())))
        .expect(&format!("Couldn't write {} bindings!", framework));
}

#[cfg(feature = "generate-bindings")]
fn generate_objc_bindings(sdk_path: &Path, clang_args: &[&str]) {
    let bindings = bindgen::Builder::default()
        .header_contents(
            "objc.h",
            r#"
#include <objc/runtime.h>
#include <objc/message.h>
        "#,
        )
        .clang_args(clang_args)
        .allowlist_function("objc_.*")
        .allowlist_function("sel_.*")
        .allowlist_function("class_.*")
        .allowlist_function("method_.*")
        .allowlist_function("object_.*")
        .allowlist_type("objc_.*")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .generate()
        .expect("Unable to generate objc bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("objc.rs"))
        .expect("Couldn't write objc bindings!");
}

#[cfg(feature = "generate-bindings")]
fn generate_mach_bindings(sdk_path: &Path, clang_args: &[&str]) {
    let bindings = bindgen::Builder::default()
        .header_contents(
            "mach.h",
            r#"
#include <mach/mach.h>
#include <mach/task.h>
#include <mach/thread_act.h>
#include <mach/vm_map.h>
        "#,
        )
        .clang_args(clang_args)
        .allowlist_function("mach_.*")
        .allowlist_function("task_.*")
        .allowlist_function("thread_.*")
        .allowlist_function("vm_.*")
        .allowlist_type("mach_.*")
        .allowlist_type("task_.*")
        .allowlist_type("thread_.*")
        .allowlist_type("vm_.*")
        .allowlist_type("kern_return_t")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .generate()
        .expect("Unable to generate mach bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("mach.rs"))
        .expect("Couldn't write mach bindings!");
}
