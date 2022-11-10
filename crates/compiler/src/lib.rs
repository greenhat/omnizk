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

use c2zk_codegen::codegen;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::FrontendConfig;
use c2zk_wasm::translate_module;

#[derive(Debug)]
pub enum CompileError {}

pub fn compile(
    source: Vec<u8>,
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    match frontend {
        FrontendConfig::Wasm => {
            // TODO: hide behind a frontend func/trait
            let module = translate_module(source.as_slice())?;
            let code = codegen(&module, target)?;
            Ok(code)
        }
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use c2zk_codegen::TritonTargetConfig;

    use super::*;
    use crate::compile;

    #[test]
    fn test_const() {
        let wat = r#"
            (module (func (param i32) (result i32)
              i32.const 1
              return))"#;
        let source = wat::parse_str(wat).unwrap();
        let target = TargetConfig::Triton(TritonTargetConfig::default());
        let out_bytes = compile(source, FrontendConfig::Wasm, target).unwrap();
        let expected_triton = r#"
           todo 
        "#;
        assert_eq!(expected_triton.as_bytes(), out_bytes);
    }
}
