#![allow(unused_imports)]

use pliron::context::Context;
use pliron::pass::PassManager;

pub struct ValidaTargetConfig {
    pub pass_manager: PassManager,
}

impl Default for ValidaTargetConfig {
    fn default() -> Self {
        let pass_manager = PassManager::new();
        // pass_manager.add_pass(Box::<WasmToMidenCFLoweringPass>::default());
        // pass_manager.add_pass(Box::<WasmToMidenArithLoweringPass>::default());
        // pass_manager.add_pass(Box::<WasmToMidenFinalLoweringPass>::default());
        Self { pass_manager }
    }
}

impl ValidaTargetConfig {
    pub fn register(&self, ctx: &mut Context) {
        ozk_valida_dialect::register(ctx);
    }
}
