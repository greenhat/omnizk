use crate::FrontendConfig;
use crate::FrontendError;
use ozk_wasm_dialect::ops::ModuleOp;
use pliron::context::Context;

pub fn translate(
    ctx: &mut Context,
    source: &[u8],
    config: FrontendConfig,
) -> Result<ModuleOp, FrontendError> {
    Ok(match config {
        FrontendConfig::Wasm(_) => ozk_frontend_wasm::translate_module(ctx, source)?,
    })
}

pub fn translate_old(
    _source: &[u8],
    _config: FrontendConfig,
) -> Result<c2zk_ir::ir::Module, FrontendError> {
    todo!("");
}
