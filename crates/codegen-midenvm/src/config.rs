use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::GlobalsToMemPass;

pub struct MidenTargetConfig {
    pub output_format: MidenOutputFormat,
    pub ir_passes: Vec<Box<dyn IrPass>>,
}
impl Default for MidenTargetConfig {
    fn default() -> Self {
        Self {
            output_format: MidenOutputFormat::Source,
            ir_passes: vec![Box::<GlobalsToMemPass>::default()],
        }
    }
}

pub enum MidenOutputFormat {
    Binary,
    Source,
}
