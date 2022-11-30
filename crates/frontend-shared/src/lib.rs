//! IR builder helper

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

mod func_builder;
mod inst_builder;
mod mod_builder;
#[cfg(feature = "rust-wasm-tests")]
// #[cfg_attr(feature = "rust-wasm-tests")]
pub mod rust_wasm_tests;

pub use crate::func_builder::*;
pub use crate::inst_builder::*;
pub use crate::mod_builder::*;
