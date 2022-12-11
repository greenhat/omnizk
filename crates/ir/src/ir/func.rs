use super::Inst;

#[derive(Debug)]
pub struct Func {
    ins: Vec<Inst>,
}

impl Func {
    pub fn new(ins: Vec<Inst>) -> Self {
        Self { ins }
    }

    pub fn instructions(&self) -> &[Inst] {
        &self.ins
    }
}
