use crate::MidenOutputFormat;
use crate::MidenTargetConfig;

pub struct InstBuffer {
    inner: Vec<LabelledInstruction>,
}
impl InstBuffer {
    pub(crate) fn new(config: &MidenTargetConfig) -> Self {
        match config.output_format {
            MidenOutputFormat::Binary => todo!(),
            MidenOutputFormat::Source => Self { inner: Vec::new() },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn program(&self) -> Program {
        Program::new(&self.inner)
    }

    pub(crate) fn pretty_print(&self) -> String {
        self.inner
            .iter()
            .enumerate()
            .map(|(idx, ins)| format!("{}", ins))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn push(&mut self, inst: AnInstruction<String>) {
        self.inner.push(LabelledInstruction::Instruction(inst));
    }

    pub(crate) fn append(&mut self, insts: Vec<AnInstruction<String>>) {
        let mut insts = insts
            .into_iter()
            .map(LabelledInstruction::Instruction)
            .collect::<Vec<_>>();
        self.inner.append(&mut insts);
    }

    pub(crate) fn push_label(&mut self, label: String) {
        self.inner.push(LabelledInstruction::Label(label));
    }
}
