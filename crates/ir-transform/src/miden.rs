//! Miden specific transformations
mod convert_blocks;
pub mod lowering;

pub use convert_blocks::BlocksToFuncPass;
