use c2zk_codegen::codegen;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::translate;
use c2zk_frontend::FrontendConfig;

use crate::CompileError;

pub fn compile(
    source: &[u8],
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let module = translate(source, frontend)?;
    let code = codegen(&module, target)?;
    Ok(code)
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use c2zk_codegen::TritonTargetConfig;
    use c2zk_wasm::WasmFrontendConfig;

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
        let out_bytes = compile(
            &source,
            FrontendConfig::Wasm(WasmFrontendConfig::default()),
            target,
        )
        .unwrap();
        let expected_triton = r#"
           todo 
        "#;
        assert_eq!(expected_triton.as_bytes(), out_bytes);
    }
}
