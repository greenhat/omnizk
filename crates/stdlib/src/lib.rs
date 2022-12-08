//! std lib for c2zk runtime
#![no_std]
// Coding conventions
// #![deny(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
// #![deny(dead_code)]
#![deny(unused_imports)]
// #![deny(missing_docs)]
// Clippy exclusions
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// #![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate alloc;

#[cfg(not(target_arch = "wasm32"))]
pub mod io_native;

#[cfg(target_arch = "wasm32")]
mod io_wasm;

/// Used for defining a main entry point.
///
/// # Example
///
/// ```
/// c2zk_stdlib::entry!(main);
///
/// pub fn main() { }
/// ```
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[no_mangle]
        pub extern "C" fn __main() {
            // type check the given path
            let f: fn() = $path;
            f()
        }
    };
}

pub fn pub_input() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    return io_native::pub_input();

    #[cfg(target_arch = "wasm32")]
    return io_wasm::pub_input();
}

pub fn pub_output(x: u64) {
    #[cfg(not(target_arch = "wasm32"))]
    return io_native::pub_output(x);

    #[cfg(target_arch = "wasm32")]
    return io_wasm::pub_output(x);
}

pub fn secret_input() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    return io_native::secret_input();

    #[cfg(target_arch = "wasm32")]
    return io_wasm::secret_input();
}
