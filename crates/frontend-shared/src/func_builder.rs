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
    locals: Vec<Ty>,
}

impl FuncBuilder {
    pub fn new(name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            ins: Vec::new(),
            sig: None,
            locals: Vec::new(),
        }
    }

    pub fn declare_local(&mut self, count: u32, ty: Ty) {
        for _ in 0..count {
            self.locals.push(ty);
        }
    }

    pub fn declare_locals(&mut self, locals: Vec<Ty>) {
        self.locals.extend(locals);
    }

    pub fn build(self) -> Result<Func, FuncBuilderError> {
        let sig = self.sig.clone().ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder: {:?}", &self))
        })?;
        let mut locals_with_params = self.locals.clone();
        let mut prepended_ins = self.ins.clone();
        for (idx, param) in sig.params.iter().enumerate().rev() {
            locals_with_params.insert(0, *param);
            prepended_ins.insert(
                0,
                Inst::LocalSet {
                    local_idx: idx as u32,
                },
            );
        }
        Ok(Func::new(self.name, sig, locals_with_params, prepended_ins))
    }

    pub fn ins(&mut self) -> InstBuilder {
        InstBuilder::new(self)
    }

    pub fn push(&mut self, inst: Inst) {
        self.ins.push(inst);
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
