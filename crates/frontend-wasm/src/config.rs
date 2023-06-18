use pliron::context::Context;
use pliron::dialects::builtin;

/// Translation(parsing) options for Wasm frontend
#[derive(Default, Debug)]
pub struct WasmFrontendConfig {}

impl WasmFrontendConfig {
    /// Register dialects used in Wasm frontend
    pub fn register(&self, ctx: &mut Context) {
        ozk_wasm_dialect::register(ctx);
        builtin::register(ctx);
    }
}
