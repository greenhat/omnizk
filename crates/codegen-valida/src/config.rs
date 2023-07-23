#![allow(unused_imports)]

use ozk_ir_transform::valida::lowering::arith_op_lowering::WasmToValidaArithLoweringPass;
use ozk_ir_transform::valida::lowering::func_lowering::WasmToValidaFuncLoweringPass;
use ozk_ir_transform::valida::lowering::module_lowering::WasmToValidaModuleLoweringPass;
use ozk_ir_transform::valida::lowering::resolve_target_sym_to_pc::ValidaResolveTargetSymToPcPass;
use ozk_ir_transform::valida::lowering::WasmToValidaFinalLoweringPass;
use ozk_ir_transform::valida::track_pc::ValidaTrackProgramCounterPass;
use ozk_ir_transform::wasm::resolve_call_op::WasmCallOpToOzkCallOpPass;
use ozk_ir_transform::wasm::track_stack_depth::WasmTrackStackDepthPass;
use pliron::context::Context;
use pliron::pass::PassManager;

pub struct ValidaTargetConfig {
    pub pass_manager: PassManager,
}

impl Default for ValidaTargetConfig {
    fn default() -> Self {
        let mut pass_manager = PassManager::new();
        pass_manager.add_pass(Box::<WasmCallOpToOzkCallOpPass>::default());
        pass_manager.add_pass(Box::new(
            WasmTrackStackDepthPass::new_reserve_space_for_locals(),
        ));
        pass_manager.add_pass(Box::<WasmToValidaArithLoweringPass>::default());
        pass_manager.add_pass(Box::<WasmToValidaFuncLoweringPass>::default());
        pass_manager.add_pass(Box::<WasmToValidaModuleLoweringPass>::default());
        pass_manager.add_pass(Box::<ValidaTrackProgramCounterPass>::default());
        pass_manager.add_pass(Box::<ValidaResolveTargetSymToPcPass>::default());
        pass_manager.add_pass(Box::<WasmToValidaFinalLoweringPass>::default());
        Self { pass_manager }
    }
}

impl ValidaTargetConfig {
    pub fn register(&self, ctx: &mut Context) {
        ozk_valida_dialect::register(ctx);
    }
}
