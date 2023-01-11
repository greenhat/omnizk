use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use thiserror::Error;

use crate::InstBuilder;

#[derive(Debug)]
pub struct FuncBuilder {
    name: String,
    sig: Option<FuncType>,
    ins: Vec<Inst>,
}

impl FuncBuilder {
    pub fn new(name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            ins: Vec::new(),
            sig: None,
        }
    }

    pub fn build(self) -> Result<Func, FuncBuilderError> {
        let sig = self.sig.ok_or(FuncBuilderError::MissingSignature)?;
        Ok(Func::new(self.name, sig, self.ins))
    }

    pub fn ins(&mut self) -> InstBuilder {
        InstBuilder::new(self)
    }

    pub fn push_inst(&mut self, inst: Inst) {
        self.ins.push(inst);
    }

    pub fn push_insts(&mut self, insts: Vec<Inst>) {
        self.ins.extend(insts);
    }

    pub fn set_sig(&mut self, sig: FuncType) {
        self.sig = Some(sig);
    }
}

#[derive(Debug, Error)]
pub enum FuncBuilderError {
    #[error("missing function signature")]
    MissingSignature,
}
