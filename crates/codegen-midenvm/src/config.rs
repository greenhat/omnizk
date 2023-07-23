#![allow(unused_imports)]

use c2zk_ir_transform::miden::lowering::call_op_lowering::WasmToMidenCallOpLoweringPass;
use c2zk_ir_transform::miden::lowering::WasmToMidenArithLoweringPass;
use c2zk_ir_transform::miden::lowering::WasmToMidenCFLoweringPass;
use c2zk_ir_transform::miden::lowering::WasmToMidenFinalLoweringPass;
use c2zk_ir_transform::wasm::explicit_func_args_pass::WasmExplicitFuncArgsPass;
use c2zk_ir_transform::wasm::globals_to_mem::WasmGlobalsToMemPass;
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
        pass_manager.add_pass(Box::<WasmExplicitFuncArgsPass>::default());
        pass_manager.add_pass(Box::<WasmToMidenCallOpLoweringPass>::default());
        pass_manager.add_pass(Box::<WasmToMidenCFLoweringPass>::default());
        pass_manager.add_pass(Box::new(WasmGlobalsToMemPass::new(
            memory_layout.globals_start_address,
        )));
        pass_manager.add_pass(Box::<WasmToMidenArithLoweringPass>::default());
        // pass_manager.add_pass(Box::<WasmToMidenFinalLoweringPass>::default());
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
