//! Wasm dialect

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

pub mod attributes;
pub mod op_builder;
pub mod op_interfaces;
pub mod ops;
pub mod types;

use pliron::context::Context;
use pliron::dialect::Dialect;
use pliron::dialect::DialectName;

/// Register the dialect in the context.
pub fn register(ctx: &mut Context) {
    let mut dialect = Dialect::new(WASM_DIALECT_NAME());
    ops::register(ctx, &mut dialect);
    // types::register(&mut dialect);
    // attributes::register(&mut dialect);
    dialect.register(ctx);
}

/// Returns the name of the dialect.
#[allow(non_snake_case)]
pub fn WASM_DIALECT_NAME() -> DialectName {
    DialectName::new("wasm")
}

/// Returns the dialect.
#[allow(non_snake_case)]
pub fn WASM_DIALECT(ctx: &Context) -> &Dialect {
    #[allow(clippy::expect_used)]
    Dialect::get_ref(ctx, WASM_DIALECT_NAME()).expect("wasm dialect not registered")
}
