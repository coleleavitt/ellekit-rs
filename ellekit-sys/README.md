# ellekit-sys

Raw FFI bindings to [ElleKit](https://github.com/evelyneee/ellekit).

This crate provides low-level, unsafe bindings to the ElleKit C API. For most use cases, you should use the safe `ellekit` crate instead.

## Usage

```toml
[dependencies]
ellekit-sys = "0.1"
```

## Features

- `generate-bindings` - Generate bindings from C headers at build time (requires clang/libclang)

By default, pre-generated bindings are used. Enable `generate-bindings` to regenerate them:

```toml
[dependencies]
ellekit-sys = { version = "0.1", features = ["generate-bindings"] }
```

## Safety

All functions in this crate are `unsafe`. You are responsible for:

- Ensuring pointers are valid
- Managing memory correctly
- Preventing data races
- Not violating ElleKit's API contracts

## Example

```rust
use ellekit_sys::*;
use std::ptr;

unsafe {
    let mut original = ptr::null_mut();
    MSHookFunction(
        target_ptr,
        replacement_ptr,
        &mut original,
    );
}
```

## License

MIT OR Apache-2.0
