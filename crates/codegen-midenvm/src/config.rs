use c2zk_ir::pass::IrPass;

pub struct MidenTargetConfig {
    pub output_format: MidenOutputFormat,
    pub ir_passes: Vec<Box<dyn IrPass>>,
}
impl Default for MidenTargetConfig {
    fn default() -> Self {
        Self {
            output_format: MidenOutputFormat::Source,
            ir_passes: vec![todo!()],
        }
    }
}

pub enum MidenOutputFormat {
    Binary,
    Source,
}
