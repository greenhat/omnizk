//! Performs translation from a CLIF to ozk dialects

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
mod func_op_builder;
mod mod_builder;
mod module_translator;
