//! Compiler

// Coding conventions
// #![deny(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
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

extern "C" {
    fn c2zk_stdlib_pub_input() -> u64;
    fn c2zk_stdlib_pub_output(x: u64);
}

pub fn read_io() -> u64 {
    unsafe { c2zk_stdlib_pub_input() }
}

pub fn write_io(x: u64) {
    unsafe { c2zk_stdlib_pub_output(x) }
}
