//! IR transformations

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

mod convert_blocks;
mod locals_to_mem;
mod pseudo_op_and;
mod pseudo_op_sub;

pub use convert_blocks::BlocksToFuncPass;
pub use locals_to_mem::LocalsToMemPass;
pub use pseudo_op_and::PseudoOpAndPass;
pub use pseudo_op_sub::PseudoOpSubPass;