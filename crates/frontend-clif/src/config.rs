use pliron::context::Context;
use pliron::dialects::builtin;

/// Translation(parsing) options for CLIF frontend
#[derive(Default, Debug)]
pub struct ClifFrontendConfig {}

impl ClifFrontendConfig {
    /// Register dialects used in Wasm frontend
    pub fn register(&self, ctx: &mut Context) {
        ozk_wasm_dialect::register(ctx);
        ozk_ozk_dialect::register(ctx);
        builtin::register(ctx);
    }
}
