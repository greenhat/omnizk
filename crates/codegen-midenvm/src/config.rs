use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::DceUnusedFunctionsPass;
use c2zk_ir_transform::GlobalsToMemPass;
use c2zk_ir_transform::SaveStackPubInputsPass;

use crate::MidenMemoryLayout;

pub struct MidenTargetConfig {
    pub output_format: MidenOutputFormat,
    pub ir_passes: Vec<Box<dyn IrPass>>,
    pub memory_layout: MidenMemoryLayout,
}

impl Default for MidenTargetConfig {
    fn default() -> Self {
        let memory_layout = MidenMemoryLayout::default();
        Self {
            output_format: MidenOutputFormat::Source,
            ir_passes: vec![
                Box::new(SaveStackPubInputsPass::new(
                    memory_layout.pub_inputs_start_address,
                    memory_layout.pub_outputs_start_address,
                )),
                Box::new(GlobalsToMemPass::new(memory_layout.globals_start_address)),
                Box::<DceUnusedFunctionsPass>::default(),
            ],
            memory_layout,
        }
    }
}

pub enum MidenOutputFormat {
    Binary,
    Source,
}
