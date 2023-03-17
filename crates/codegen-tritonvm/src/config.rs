use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::AndMinus8Pass;
use c2zk_ir_transform::BlocksToFuncPass;
use c2zk_ir_transform::GlobalsToMemPass;
use c2zk_ir_transform::LocalsToMemPass;
use c2zk_ir_transform::PseudoOpSubPass;

pub struct TritonTargetConfig {
    pub output_format: TritonOutputFormat,
    pub ir_passes: Vec<Box<dyn IrPass>>,
}
impl Default for TritonTargetConfig {
    fn default() -> Self {
        Self {
            output_format: TritonOutputFormat::Source,
            ir_passes: vec![
                Box::<AndMinus8Pass>::default(),
                Box::<LocalsToMemPass>::default(),
                Box::<GlobalsToMemPass>::default(),
                Box::<BlocksToFuncPass>::default(),
                // we might've added GlobalSet/Gets in the BlocksToFuncPass
                Box::<GlobalsToMemPass>::default(),
                Box::<PseudoOpSubPass>::default(),
            ],
        }
    }
}

pub enum TritonOutputFormat {
    Binary,
    Source,
}
