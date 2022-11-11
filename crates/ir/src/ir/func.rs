use super::Inst;

pub struct Func {
    ins: Vec<Inst>,
}

impl Func {
    pub fn new(ins: Vec<Inst>) -> Self {
        Self { ins }
    }

    pub fn inst(&self) -> &[Inst] {
        &self.ins
    }
}
