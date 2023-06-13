//! Miden specific transformations
mod convert_blocks;
mod lowering;

pub use convert_blocks::BlocksToFuncPass;
pub use lowering::WasmToMidenLoweringPass;
