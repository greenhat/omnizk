use c2zk_ir_transform::miden::WasmToMidenLoweringPass;
use pliron::context::Context;
use pliron::pass::PassManager;

use crate::MidenMemoryLayout;

pub struct MidenTargetConfig {
    pub output_format: MidenOutputFormat,
    pub pass_manager: PassManager,
    pub memory_layout: MidenMemoryLayout,
}

impl Default for MidenTargetConfig {
    fn default() -> Self {
        let memory_layout = MidenMemoryLayout::default();
        let mut pass_manager = PassManager::new();
        pass_manager.add_pass(Box::<WasmToMidenLoweringPass>::default());
        Self {
            output_format: MidenOutputFormat::Source,
            // ir_passes: vec![
            // Box::new(SaveStackPubInputsPass::new(
            //     memory_layout.pub_inputs_start_address,
            //     memory_layout.pub_outputs_start_address,
            // )),
            // Box::<BlocksToFuncPass>::default(),
            // Box::new(GlobalsToMemPass::new(memory_layout.globals_start_address)),
            // Box::<DceUnusedFunctionsPass>::default(),
            // ],
            memory_layout,
            pass_manager,
        }
    }
}

impl MidenTargetConfig {
    pub fn register(&self, ctx: &mut Context) {
        ozk_miden_dialect::register(ctx);
    }
}

pub enum MidenOutputFormat {
    Binary,
    Source,
}
