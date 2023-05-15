use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::ops::FuncOp;
use pliron::dialects::builtin::types::FunctionType;
use pliron::r#type::Type;
use pliron::r#type::TypeObj;
use thiserror::Error;

pub struct FuncBuilder<'a> {
    ctx: &'a mut Context,
    name: String,
    sig: Option<FunctionType>,
    // ins: Vec<Inst>,
    entry_block: Ptr<BasicBlock>,
    locals: Vec<Ptr<TypeObj>>,
}

impl<'a> FuncBuilder<'a> {
    pub fn new(ctx: &mut Context, name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            sig: None,
            locals: Vec::new(),
            entry_block: BasicBlock::new(ctx, None, Vec::new()),
            ctx,
        }
    }

    pub fn declare_local(&mut self, count: u32, ty: Ptr<TypeObj>) {
        for _ in 0..count {
            self.locals.push(ty);
        }
    }

    pub fn declare_locals(&mut self, locals: Vec<Ptr<TypeObj>>) {
        self.locals.extend(locals);
    }

    pub fn build(self) -> Result<FuncOp, FuncBuilderError> {
        let sig = self.sig.ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder: {:?}", &self))
        })?;
        // TODO: should be a separate lowering pass
        // let mut locals_with_params = self.locals.clone();
        // let mut prepended_ins = self.ins.clone();
        // for (idx, param) in sig.params.iter().enumerate().rev() {
        //     locals_with_params.insert(0, *param);
        //     prepended_ins.insert(
        //         0,
        //         Inst::LocalSet {
        //             local_idx: idx as u32,
        //         },
        //     );
        // }
        // TODO: make wasm FuncOp
        Ok(FuncOp::new_unlinked(
            self.ctx,
            &self.name,
            Type::register_instance(sig, self.ctx),
        ))
    }

    // pub fn ins(&mut self) -> InstBuilder {
    //     InstBuilder::new(self)
    // }

    pub fn get_entry_block(&self) -> Ptr<BasicBlock> {
        self.entry_block
    }

    // pub fn push(&mut self, inst: Inst) {
    //     self.ins.push(inst);
    // }

    // pub fn push_insts(&mut self, insts: Vec<Inst>) {
    //     self.ins.extend(insts);
    // }

    pub fn set_signature(&mut self, signature: FunctionType) {
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
