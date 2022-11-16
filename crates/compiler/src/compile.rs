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
    use expect_test::expect;

    use super::*;
    use crate::compile;

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        let source = wat::parse_str(input).unwrap();
        let target = TargetConfig::Triton(TritonTargetConfig::default());
        let out_bytes = compile(
            &source,
            FrontendConfig::Wasm(WasmFrontendConfig::default()),
            target,
        )
        .unwrap();
        let out_source = String::from_utf8(out_bytes).unwrap();
        expected_tree.assert_eq(&out_source);
    }

    #[test]
    fn test_const() {
        check(
            r#"
            (module (func 
              i32.const 1
              return))"#,
            expect![[r#"
               push 1
               return"#]],
        );
    }

    #[test]
    fn test_start_section() {
        check(
            r#"
            (module 
            (start $f1)
            (func $f1 
              i32.const 1
              return)
              )"#,
            expect![[r#"
               push 1
               return"#]],
        );
    }
}
