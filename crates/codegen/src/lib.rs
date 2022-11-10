//! Codegen

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

use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_tritonvm::emit;
pub use c2zk_codegen_tritonvm::TritonTargetConfig;
use c2zk_ir::ir::Module;

pub enum TargetConfig {
    Triton(TritonTargetConfig),
}

pub fn codegen(module: &Module, target_config: TargetConfig) -> Result<Vec<u8>, CodegenError> {
    match target_config {
        TargetConfig::Triton(config) => emit(module, config),
    }
}
