use derive_more::From;
use ozk_frontend_wasm::WasmFrontendConfig;
use pliron::context::Context;

#[derive(Debug, From)]
pub enum FrontendConfig {
    Wasm(WasmFrontendConfig),
}

impl FrontendConfig {
    pub fn register(&self, ctx: &mut Context) {
        match self {
            FrontendConfig::Wasm(c) => c.register(ctx),
        }
    }
}
