use std::collections::HashMap;

use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Ty;
use thiserror::Error;

use crate::InstBuilder;

#[derive(Debug)]
pub struct FuncBuilder {
    name: String,
    sig: Option<FuncType>,
    ins: Vec<Inst>,
    comments: HashMap<usize, String>,
    locals: Vec<Ty>,
}

impl FuncBuilder {
    pub fn new(name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            ins: Vec::new(),
            sig: None,
            comments: HashMap::new(),
            locals: Vec::new(),
        }
    }

    pub fn declare_local(&mut self, idx: u32, ty: Ty) {
        dbg!(idx, ty);
        self.locals.push(ty);
    }

    pub fn build(self) -> Result<Func, FuncBuilderError> {
        let sig = self.sig.clone().ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder: {:?}", &self))
        })?;
        Ok(Func::new(
            self.name,
            sig,
            self.locals,
            self.ins,
            self.comments,
        ))
    }

    pub fn ins(&mut self) -> InstBuilder {
        InstBuilder::new(self)
    }

    pub fn push(&mut self, inst: Inst) {
        self.ins.push(inst);
    }
    pub fn push_with_comment(&mut self, inst: Inst, comment: String) {
        self.ins.push(inst);
        self.comments.insert(self.ins.len() - 1, comment);
    }

    pub fn push_insts(&mut self, insts: Vec<Inst>) {
        self.ins.extend(insts);
    }

    pub fn set_signature(&mut self, signature: FuncType) {
        self.sig = Some(signature);
    }

    pub fn set_name(&mut self, clone: String) {
        self.name = clone;
    }
}

#[derive(Debug, Error)]
pub enum FuncBuilderError {
    #[error("missing function signature")]
    MissingSignature(String),
}
