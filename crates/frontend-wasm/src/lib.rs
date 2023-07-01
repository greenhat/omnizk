//! Performs translation from a wasm module in binary format to the in-memory form
//! of c2zk IR.

// Coding conventions
#![deny(unsafe_code)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
// #![deny(dead_code)]
#![allow(dead_code)]
// #![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(trivial_numeric_casts)]
#![deny(unused_extern_crates)]
#![deny(unstable_features)]
// Clippy exclusions
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// #![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]
#![warn(unused_import_braces)]

mod code_translator;
mod config;
mod error;
mod func_builder;
mod mod_builder;
mod module_translator;
mod op_builder;

pub use crate::config::WasmFrontendConfig;
pub use crate::error::WasmError;
pub use crate::module_translator::parse_module;

// Convenience reexport of the wasmparser crate that we're linking against,
// since a number of types in `wasmparser` show up in the public API of
// `c2zk-wasm`.
// pub use wasmparser;
