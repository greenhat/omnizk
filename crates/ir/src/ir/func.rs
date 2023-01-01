use std::collections::HashMap;

use super::Inst;

#[derive(Debug, Clone)]
pub struct Func {
    name: String,
    ins: Vec<Inst>,
    comments: HashMap<usize, String>,
}

impl Func {
    pub fn new(name: String, ins: Vec<Inst>) -> Self {
        Self {
            name,
            ins,
            comments: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instructions(&self) -> &[Inst] {
        &self.ins
    }

    pub fn comments(&self) -> &HashMap<usize, String> {
        &self.comments
    }

    pub fn push(&mut self, inst: Inst) {
        self.ins.push(inst);
    }

    pub fn push_with_comment(&mut self, inst: Inst, comment: String) {
        self.ins.push(inst);
        self.comments.insert(self.ins.len() - 1, comment);
    }

    pub fn set_comment(&mut self, idx: usize, comment: String) {
        self.comments.insert(idx, comment);
    }
}
