//! Performs translation from a wasm module in binary format to the in-memory form
//! of c2zk IR.
//! The main function of this module is [`translate_module`](fn.translate_module.html).

#![deny(
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unstable_features
)]
#![warn(unused_import_braces)]

mod code_translator;
mod error;
mod module_translator;
mod types;

pub use crate::module_translator::translate_module;

// Convenience reexport of the wasmparser crate that we're linking against,
// since a number of types in `wasmparser` show up in the public API of
// `c2zk-wasm`.
pub use wasmparser;
