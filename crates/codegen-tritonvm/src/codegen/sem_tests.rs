//! Semantic equivalence tests for the TritonVM codegen.

#![allow(clippy::unwrap_used)]

mod add;
mod fib;

use c2zk_ir::pass::run_ir_passes;

use crate::compile_module;
use crate::TritonTargetConfig;

fn check_wasm(
    source: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_triton: expect_test::Expect,
) {
    use c2zk_frontend::translate;
    use c2zk_frontend::FrontendConfig;
    use c2zk_frontend::WasmFrontendConfig;

    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);
    let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
    let triton_target_config = TritonTargetConfig::default();

    let mut module = translate(source, frontend).unwrap();
    run_ir_passes(&mut module, &triton_target_config.ir_passes);
    let inst_buf = compile_module(module, &triton_target_config).unwrap();
    let out_source = inst_buf.pretty_print();
    expected_triton.assert_eq(&out_source);
    let program = inst_buf.program();
    let input = input.into_iter().map(Into::into).collect();
    let secret_input = secret_input.into_iter().map(Into::into).collect();
    let (_trace, out, err) = triton_vm::vm::run(&program, input, secret_input);
    dbg!(&err);
    // dbg!(&_trace);
    assert!(err.is_none());
    assert_eq!(
        out.into_iter().map(|b| b.into()).collect::<Vec<u64>>(),
        expected_output
    );
}
