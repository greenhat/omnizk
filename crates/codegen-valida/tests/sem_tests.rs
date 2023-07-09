//! Semantic equivalence tests for the Valida VM codegen.

#![allow(clippy::unwrap_used)]
#![allow(unused_variables)]
#![allow(dead_code)]

use ozk_codegen_valida::ValidaTargetConfig;
use ozk_frontend_wasm::WasmFrontendConfig;
use wasmtime::*;

pub fn check_wasm(
    source: &[u8],
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_wat: expect_test::Expect,
    expected_miden: expect_test::Expect,
) {
    let wat = wasmprinter::print_bytes(source).unwrap();
    expected_wat.assert_eq(&wat);

    check_valida(wat, input, secret_input, expected_output, expected_miden);
}

#[allow(unreachable_code)]
pub fn check_valida(
    source: String,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_valida: expect_test::Expect,
) {
    let frontend_config = WasmFrontendConfig::default();
    let target_config = ValidaTargetConfig::default();
    let wasm = wat::parse_str(source).unwrap();
    let program_bytes = todo!("codegen");
    let program = String::from_utf8(program_bytes).unwrap();
    expected_valida.assert_eq(&program);
}

pub fn check_wat(
    source: &str,
    input: Vec<u64>,
    secret_input: Vec<u64>,
    expected_output: Vec<u64>,
    expected_miden: expect_test::Expect,
) {
    struct Io {
        input: Vec<u64>,
        secret_input: Vec<u64>,
        output: Vec<u64>,
    }

    let mut store = Store::new(
        &Engine::default(),
        Io {
            input: input.clone().into_iter().rev().collect(),
            secret_input: secret_input.clone().into_iter().rev().collect(),
            output: Vec::new(),
        },
    );

    let wasm = wat::parse_str(source).unwrap();
    let module = Module::from_binary(store.engine(), &wasm).unwrap();

    let c2zk_stdlib_pub_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().input.pop().unwrap()
    });
    let c2zk_stdlib_pub_output =
        Func::wrap(&mut store, |mut caller: Caller<'_, Io>, output: i64| {
            caller.data_mut().output.push(output as u64);
        });
    let c2zk_stdlib_secret_input = Func::wrap(&mut store, |mut caller: Caller<'_, Io>| {
        caller.data_mut().secret_input.pop().unwrap()
    });
    let imports = [
        c2zk_stdlib_pub_input.into(),
        c2zk_stdlib_pub_output.into(),
        c2zk_stdlib_secret_input.into(),
    ];
    let _ = Instance::new(&mut store, &module, &imports).unwrap();

    assert_eq!(store.data().output, expected_output);
    check_valida(
        source.to_string(),
        input,
        secret_input,
        expected_output,
        expected_miden,
    );
}
