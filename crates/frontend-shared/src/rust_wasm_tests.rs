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
fn compile_rust_wasm_tests_bundle1(bin_name: &str) -> Vec<u8> {
    // TODO: make it relative to the root crate (not the one it is called from)
    let manifest_path = "../rust-wasm-tests/bundle1-bin/Cargo.toml";
    let pwd = std::process::Command::new("pwd").output().unwrap();
    dbg!(&pwd);
    let comp_status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg(format!("--bin {}", bin_name))
        .arg("--target=wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--target-dir")
        .arg("/tmp/c2zk-rust-wasm-tests")
        .status()
        .unwrap();
    dbg!(&comp_status);
    let target_bin_file_path = std::path::Path::new("/tmp/c2zk-rust-wasm-tests")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(bin_name)
        .with_extension("wasm");
    let mut target_bin_file = std::fs::File::open(target_bin_file_path).unwrap();
    let mut wasm_bytes = vec![];
    std::io::Read::read_to_end(&mut target_bin_file, &mut wasm_bytes).unwrap();
    wasm_bytes
}

#[cfg(feature = "rust-wasm-tests")]
pub fn add_test() -> RustWasmTestCode {
    let wasm_bytes = compile_rust_wasm_tests_bundle1("main");
    let main_func = &c2zk_rust_wasm_tests_bundle1::main;
    RustWasmTestCode {
        wasm_bytes,
        main_func: wrap_main_with_io(main_func),
    }
}
