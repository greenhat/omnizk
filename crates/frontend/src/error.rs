use c2zk_wasm::WasmError;
use derive_more::From;

#[derive(Debug, From)]
pub enum FrontendError {
    WasmError(WasmError),
}
