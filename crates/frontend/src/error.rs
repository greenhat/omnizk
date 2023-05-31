use derive_more::From;
use ozk_frontend_wasm::WasmError;

#[derive(Debug, From)]
pub enum FrontendError {
    WasmError(WasmError),
}
