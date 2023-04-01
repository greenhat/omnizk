use crate::MidenInst;
use crate::MidenOutputFormat;
use crate::MidenTargetConfig;

pub struct InstBuffer {
    inner: Vec<MidenInst>,
}
impl InstBuffer {
    pub(crate) fn new(config: &MidenTargetConfig) -> Self {
        match config.output_format {
            MidenOutputFormat::Binary => todo!(),
            MidenOutputFormat::Source => Self { inner: Vec::new() },
        }
    }

    pub(crate) fn pretty_print(&self) -> String {
        self.inner
            .iter()
            .map(|inst| String::from(inst.clone()))
            .collect()
    }

    pub(crate) fn push(&mut self, inst: MidenInst) {
        self.inner.push(inst);
    }

    pub(crate) fn append(&mut self, mut insts: Vec<MidenInst>) {
        self.inner.append(&mut insts);
    }
}
