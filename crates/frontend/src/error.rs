use ozk_frontend_wasm::WasmError;

#[derive(Debug, thiserror::Error)]
pub enum FrontendError {
    #[error("Wasm error: {0}")]
    WasmError(#[from] WasmError),
}
