#![allow(unused_imports)]

use c2zk_ir_transform::valida::lowering::WasmToValidaArithLoweringPass;
use c2zk_ir_transform::valida::lowering::WasmToValidaFinalLoweringPass;
use c2zk_ir_transform::wasm::track_stack_depth::WasmTrackStackDepthPass;
use pliron::context::Context;
use pliron::pass::PassManager;

pub struct ValidaTargetConfig {
    pub pass_manager: PassManager,
}

impl Default for ValidaTargetConfig {
    fn default() -> Self {
        let mut pass_manager = PassManager::new();
        pass_manager.add_pass(Box::<WasmTrackStackDepthPass>::default());
        pass_manager.add_pass(Box::<WasmToValidaArithLoweringPass>::default());
        pass_manager.add_pass(Box::<WasmToValidaFinalLoweringPass>::default());
        Self { pass_manager }
    }
}

impl ValidaTargetConfig {
    pub fn register(&self, ctx: &mut Context) {
        ozk_valida_dialect::register(ctx);
    }
}
