use crate::FrontendConfig;
use crate::FrontendError;
use c2zk_ir::ir::Module;
use c2zk_wasm::translate_module;

pub fn translate(source: &[u8], config: FrontendConfig) -> Result<Module, FrontendError> {
    Ok(match config {
        FrontendConfig::Wasm(_) => translate_module(source)?,
    })
}
