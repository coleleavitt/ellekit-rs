use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let _target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let _target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Find iOS SDK from submodules
    let sdk_path = find_ios_sdk();

    if let Some(sdk) = &sdk_path {
        println!("cargo:warning=ios-sys: Using iOS SDK: {}", sdk.display());

        // Setup linker search paths (always)
        setup_linking(sdk);

        // Generate bindings from SDK headers
        generate_bindings(sdk);

        // Link frameworks if 'runtime' feature is enabled
        #[cfg(feature = "runtime")]
        {
            link_frameworks();
        }

        #[cfg(not(feature = "runtime"))]
        {
            println!("cargo:warning=ios-sys: Header-only mode (enable 'runtime' feature to link)");
        }
    } else {
        println!("cargo:warning=ios-sys: No iOS SDK found - cannot generate bindings!");
        println!("cargo:warning=To use Theos SDKs, clone them into submodules/sdks/");
        panic!("ios-sys requires iOS SDK in submodules/sdks/ to generate bindings");
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

    // Set sysroot for clang (used by bindgen)
    println!("cargo:rustc-env=IPHONEOS_SDK_PATH={}", sdk_path.display());
}

/// Link iOS frameworks based on enabled features
#[cfg(feature = "runtime")]
fn link_frameworks() {
    let sdk_path = find_ios_sdk().expect("SDK required for runtime linking");

    // On macOS, use framework linking
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=dylib=System");

        #[cfg(feature = "uikit")]
        println!("cargo:rustc-link-lib=framework=UIKit");
    }

    // On other platforms (Linux cross-compiling), we need to explicitly link .tbd files
    #[cfg(not(target_os = "macos"))]
    {
        // libobjc is in /usr/lib/libobjc.tbd
        let libobjc_path = sdk_path.join("usr/lib/libobjc.tbd");
        if libobjc_path.exists() {
            println!("cargo:rustc-link-arg={}", libobjc_path.display());
        }

        // Framework TBDs
        let frameworks_dir = sdk_path.join("System/Library/Frameworks");

        // Foundation
        let foundation_tbd = frameworks_dir.join("Foundation.framework/Foundation.tbd");
        if foundation_tbd.exists() {
            println!("cargo:rustc-link-arg={}", foundation_tbd.display());
        }

        // CoreFoundation
        let corefoundation_tbd = frameworks_dir.join("CoreFoundation.framework/CoreFoundation.tbd");
        if corefoundation_tbd.exists() {
            println!("cargo:rustc-link-arg={}", corefoundation_tbd.display());
        }

        // UIKit
        #[cfg(feature = "uikit")]
        {
            let uikit_tbd = frameworks_dir.join("UIKit.framework/UIKit.tbd");
            if uikit_tbd.exists() {
                println!("cargo:rustc-link-arg={}", uikit_tbd.display());
            }
        }

        // libSystem
        let libsystem_tbd = sdk_path.join("usr/lib/libSystem.tbd");
        if libsystem_tbd.exists() {
            println!("cargo:rustc-link-arg={}", libsystem_tbd.display());
        }
    }
}

/// Generate bindings using bindgen from SDK headers
fn generate_bindings(sdk_path: &Path) {
    let sysroot = format!("-isysroot{}", sdk_path.display());
    let include_path = format!("-I{}/usr/include", sdk_path.display());

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Common clang args
    let common_args = vec![
        sysroot.as_str(),
        include_path.as_str(),
        "-target",
        "arm64-apple-ios",
        "-fembed-bitcode",
    ];

    // Generate Objective-C runtime bindings
    println!("cargo:warning=Generating Objective-C runtime bindings...");
    let objc_bindings = bindgen::Builder::default()
        .header_contents(
            "objc_wrapper.h",
            r#"
#include <objc/runtime.h>
#include <objc/message.h>
#include <objc/objc.h>
            "#,
        )
        .clang_args(&common_args)
        .allowlist_function("objc_.*")
        .allowlist_function("sel_.*")
        .allowlist_function("class_.*")
        .allowlist_function("method_.*")
        .allowlist_function("object_.*")
        .allowlist_function("ivar_.*")
        .allowlist_function("protocol_.*")
        .allowlist_function("property_.*")
        .allowlist_type("objc_.*")
        .allowlist_type("Protocol")
        .allowlist_var("OBJC_.*")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate objc bindings");

    objc_bindings
        .write_to_file(out_path.join("objc.rs"))
        .expect("Couldn't write objc bindings!");

    // Generate Mach kernel bindings
    println!("cargo:warning=Generating Mach kernel bindings...");
    let mach_bindings = bindgen::Builder::default()
        .header_contents(
            "mach_wrapper.h",
            r#"
#include <mach/mach.h>
#include <mach/task.h>
#include <mach/thread_act.h>
#include <mach/vm_map.h>
#include <mach/kern_return.h>
#include <mach/port.h>
            "#,
        )
        .clang_args(&common_args)
        .allowlist_function("mach_.*")
        .allowlist_function("task_.*")
        .allowlist_function("thread_.*")
        .allowlist_function("vm_.*")
        .allowlist_function("host_.*")
        .allowlist_type("mach_.*")
        .allowlist_type("task_.*")
        .allowlist_type("thread_.*")
        .allowlist_type("vm_.*")
        .allowlist_type("kern_return_t")
        .allowlist_type("natural_t")
        .allowlist_type("integer_t")
        .allowlist_type("boolean_t")
        .allowlist_var("KERN_.*")
        .allowlist_var("VM_.*")
        .allowlist_var("MACH_.*")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate mach bindings");

    mach_bindings
        .write_to_file(out_path.join("mach.rs"))
        .expect("Couldn't write mach bindings!");

    // Generate Foundation/CoreGraphics types
    println!("cargo:warning=Generating Foundation type bindings...");
    let foundation_bindings = bindgen::Builder::default()
        .header_contents(
            "foundation_wrapper.h",
            r#"
// Include objc runtime first
#include <objc/objc.h>
#include <objc/NSObjCRuntime.h>

// Basic CoreGraphics types (defined manually to avoid CG framework dependencies)
typedef double CGFloat;

typedef struct CGPoint {
    CGFloat x;
    CGFloat y;
} CGPoint;

typedef struct CGSize {
    CGFloat width;
    CGFloat height;
} CGSize;

typedef struct CGRect {
    CGPoint origin;
    CGSize size;
} CGRect;

typedef struct NSRange {
    NSUInteger location;
    NSUInteger length;
} NSRange;

typedef struct UIEdgeInsets {
    CGFloat top;
    CGFloat left;
    CGFloat bottom;
    CGFloat right;
} UIEdgeInsets;

typedef struct UIOffset {
    CGFloat horizontal;
    CGFloat vertical;
} UIOffset;

// Common type aliases
typedef NSInteger NSComparisonResult;
typedef NSUInteger NSStringEncoding;
            "#,
        )
        .clang_args(&common_args)
        .allowlist_type("CGFloat")
        .allowlist_type("CGPoint")
        .allowlist_type("CGSize")
        .allowlist_type("CGRect")
        .allowlist_type("NSRange")
        .allowlist_type("UIEdgeInsets")
        .allowlist_type("UIOffset")
        .allowlist_type("NSInteger")
        .allowlist_type("NSUInteger")
        .allowlist_type("NSComparisonResult")
        .allowlist_type("NSStringEncoding")
        .use_core()
        .ctypes_prefix("::core::ffi")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate foundation bindings");

    foundation_bindings
        .write_to_file(out_path.join("foundation.rs"))
        .expect("Couldn't write foundation bindings!");

    println!("cargo:warning=Bindings generated successfully!");
}
