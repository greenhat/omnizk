use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::BlocksToFuncPass;
use c2zk_ir_transform::PseudoOpAndPass;
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
                Box::new(BlocksToFuncPass::default()),
                Box::new(PseudoOpAndPass::default()),
                Box::new(PseudoOpSubPass::default()),
            ],
        }
    }
}

pub enum TritonOutputFormat {
    Binary,
    Source,
}
