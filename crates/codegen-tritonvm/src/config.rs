use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::BlocksToFuncPass;

pub struct TritonTargetConfig {
    pub output_format: TritonOutputFormat,
    pub ir_passes: Vec<Box<dyn IrPass>>,
}
impl Default for TritonTargetConfig {
    fn default() -> Self {
        Self {
            output_format: TritonOutputFormat::Source,
            ir_passes: vec![Box::new(BlocksToFuncPass::new())],
        }
    }
}

pub enum TritonOutputFormat {
    Binary,
    Source,
}
