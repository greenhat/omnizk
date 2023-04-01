use crate::MidenOutputFormat;
use crate::MidenTargetConfig;

pub struct InstBuffer {
    inner: Vec<String>,
}
impl InstBuffer {
    pub(crate) fn new(config: &MidenTargetConfig) -> Self {
        match config.output_format {
            MidenOutputFormat::Binary => todo!(),
            MidenOutputFormat::Source => Self { inner: Vec::new() },
        }
    }

    pub(crate) fn pretty_print(&self) -> String {
        self.inner.join("\n")
    }

    pub(crate) fn push(&mut self, inst: String) {
        self.inner.push(inst);
    }

    pub(crate) fn append(&mut self, mut insts: Vec<String>) {
        self.inner.append(&mut insts);
    }

    pub(crate) fn push_func_label(&mut self, label: String) {
        self.inner.push(format!("proc.{label}"));
    }
}
