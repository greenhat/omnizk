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
            .map(|inst| {
                let str = String::from(inst.clone());
                if str != "end" {
                    str
                } else {
                    format!("{str}\n")
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn push(&mut self, inst: MidenInst) {
        self.inner.push(inst);
    }
}
