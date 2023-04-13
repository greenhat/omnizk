use super::FuncType;
use super::Inst;
use super::Ty;

#[derive(Debug, Clone)]
pub struct Func {
    name: String,
    sig: FuncType,
    locals: Vec<Ty>,
    ins: Vec<Inst>,
}

impl Func {
    pub fn new(name: String, sig: FuncType, locals: Vec<Ty>, ins: Vec<Inst>) -> Self {
        Self {
            name,
            sig,
            ins,
            locals,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instructions(&self) -> &[Inst] {
        &self.ins
    }

    pub fn instructions_mut(&mut self) -> &mut [Inst] {
        &mut self.ins
    }

    pub fn instructions_as_vec_mut(&mut self) -> &mut Vec<Inst> {
        &mut self.ins
    }

    pub fn instructions_into_iter(self) -> impl Iterator<Item = Inst> {
        self.ins.into_iter()
    }

    pub fn push(&mut self, inst: Inst) {
        self.ins.push(inst);
    }

    // Prepends an instruction to the beginning of the function.
    pub fn prepend(&mut self, inst: Inst) {
        self.ins.insert(0, inst);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn sig(&self) -> &FuncType {
        &self.sig
    }

    pub fn locals(&self) -> &[Ty] {
        &self.locals
    }
}
