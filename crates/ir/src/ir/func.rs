use std::collections::HashMap;

use super::FuncType;
use super::Inst;
use super::Ty;

#[derive(Debug, Clone)]
pub struct Func {
    name: String,
    sig: FuncType,
    locals: Vec<Ty>,
    ins: Vec<Inst>,
    comments: HashMap<usize, String>,
}

impl Func {
    pub fn new(
        name: String,
        sig: FuncType,
        locals: Vec<Ty>,
        ins: Vec<Inst>,
        comments: HashMap<usize, String>,
    ) -> Self {
        Self {
            name,
            sig,
            ins,
            comments,
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

    pub fn comments(&self) -> &HashMap<usize, String> {
        &self.comments
    }

    pub fn push(&mut self, inst: Inst) {
        self.ins.push(inst);
    }

    // Prepends an instruction to the beginning of the function.
    pub fn prepend(&mut self, inst: Inst) {
        self.ins.insert(0, inst);
    }

    pub fn push_with_comment(&mut self, inst: Inst, comment: String) {
        self.ins.push(inst);
        self.comments.insert(self.ins.len() - 1, comment);
    }

    pub fn set_comment(&mut self, idx: usize, comment: String) {
        self.comments.insert(idx, comment);
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
