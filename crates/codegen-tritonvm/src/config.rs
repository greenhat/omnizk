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
                Box::new(LocalsToMemPass::new(i32::MAX)),
                Box::<BlocksToFuncPass>::default(),
                // TODO: pass the start address for globals (determine in MemoryLayout)
                Box::new(GlobalsToMemPass::new(i32::MAX - 1024)),
                Box::<PseudoOpSubPass>::default(),
            ],
        }
    }
}

pub enum TritonOutputFormat {
    Binary,
    Source,
}

// TODO: introduce MemoryLayout to manage the addresses for globals and locals
