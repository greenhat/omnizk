// #[cfg_attr(feature = "rust-wasm-tests")]
#[cfg(feature = "rust-wasm-tests")]
extern crate c2zk_rust_wasm_tests_bundle1;

pub struct RustWasmTestCode {
    pub wasm_bytes: Vec<u8>,
    pub main_func: Box<dyn Fn(Vec<u64>) -> Vec<u64>>,
}

fn wrap_main_with_io(main_func: &'static dyn Fn()) -> Box<dyn Fn(Vec<u64>) -> Vec<u64>> {
    Box::new(|input: Vec<u64>| {
        // TODO: pass input
        main_func();
        // TODO: collect output
        let output = vec![];
        output
    })
}

#[allow(clippy::unwrap_used)]
fn compile_rust_wasm(file_path: &str) -> Vec<u8> {
    // TODO: put every test Rust source into a crate?
    let pwd = std::process::Command::new("pwd").output().unwrap();
    dbg!(&pwd);
    let comp_status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(file_path)
        .status()
        .unwrap();
    dbg!(&comp_status);
    // TODO: read wasm file
    vec![]
}

#[cfg(feature = "rust-wasm-tests")]
pub fn add_test() -> RustWasmTestCode {
    let wasm_bytes = compile_rust_wasm("../rust-wasm-tests/bundle1/Cargo.toml");
    let main_func = &c2zk_rust_wasm_tests_bundle1::main;
    RustWasmTestCode {
        wasm_bytes,
        main_func: wrap_main_with_io(main_func),
    }
}
