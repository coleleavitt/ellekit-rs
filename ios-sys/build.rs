//! Build script for ios-sys
//!
//! This frameworks script generates Rust FFI bindings for iOS frameworks using:
//! - bindgen for public frameworks (from SDK headers)
//! - TBD parsing for private frameworks (from symbol stub files)

// Include the frameworks module from frameworks/mod.rs
#[path = "frameworks/mod.rs"]
mod build_impl;

fn main() {
    build_impl::main();
}
