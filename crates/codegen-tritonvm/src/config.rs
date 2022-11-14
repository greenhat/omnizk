pub struct TritonTargetConfig {
    pub output_format: TritonOutputFormat,
}

impl Default for TritonTargetConfig {
    fn default() -> Self {
        Self {
            output_format: TritonOutputFormat::Source,
        }
    }
}

pub enum TritonOutputFormat {
    Binary,
    Source,
}
