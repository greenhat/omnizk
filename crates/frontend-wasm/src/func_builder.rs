use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::FuncOp;
use ozk_wasm_dialect::ops::LoopOp;
use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use thiserror::Error;

use crate::op_builder::OpBuilder;

pub struct FuncBuilder {
    // ctx: &'a mut Context,
    name: String,
    sig: Option<Ptr<TypeObj>>,
    blocks: Vec<BlockBuilder>,
    locals: Vec<Ptr<TypeObj>>,
}

impl FuncBuilder {
    pub fn new(ctx: &mut Context, name: String) -> FuncBuilder {
        FuncBuilder {
            name,
            sig: None,
            locals: Vec::new(),
            blocks: vec![BlockBuilder::FuncEntryBlock(BasicBlock::new(
                ctx,
                None,
                Vec::new(),
            ))],
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

    pub fn build(self, ctx: &mut Context) -> Result<FuncOp, FuncBuilderError> {
        let sig = self.sig.ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder for {}", self.name))
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
        Ok(FuncOp::new_unlinked(ctx, &self.name, sig))
    }

    pub fn op(&mut self) -> OpBuilder {
        OpBuilder::new(self)
    }

    pub fn push(&mut self, ctx: &mut Context, op: Ptr<Operation>) {
        let opop = &op.deref(ctx).get_op(ctx);
        if let Some(block) = opop.downcast_ref::<BlockOp>() {
            self.blocks.push(BlockBuilder::Block(*block));
        } else if let Some(loopop) = opop.downcast_ref::<LoopOp>() {
            self.blocks.push(BlockBuilder::Loop(*loopop));
        } else {
            let current_bb = self.blocks.last_mut().unwrap().get_bb(ctx);
            op.insert_at_back(current_bb, ctx);
        }
    }

    pub fn push_end(&mut self, ctx: &mut Context) {
        if let Some(ending_block_builder) = self.blocks.pop() {
            match ending_block_builder {
                BlockBuilder::FuncEntryBlock(bb) => (), // do nothing, it's function end
                BlockBuilder::Block(block) => {
                    let current_bb = self.blocks.last().unwrap().get_bb(ctx);
                    block.get_operation().insert_at_back(current_bb, ctx)
                }
                BlockBuilder::Loop(loopop) => {
                    let current_bb = self.blocks.last().unwrap().get_bb(ctx);
                    loopop.get_operation().insert_at_back(current_bb, ctx)
                }
            }
        } else {
            panic!("push_end called on empty block stack")
        }
    }

    // pub fn push_insts(&mut self, insts: Vec<Inst>) {
    //     self.ins.extend(insts);
    // }

    pub fn set_signature(&mut self, signature: Ptr<TypeObj>) {
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

pub enum BlockBuilder {
    FuncEntryBlock(Ptr<BasicBlock>),
    Block(BlockOp),
    Loop(LoopOp),
}

impl BlockBuilder {
    pub fn get_bb(&self, ctx: &Context) -> Ptr<BasicBlock> {
        match self {
            BlockBuilder::FuncEntryBlock(bb) => *bb,
            BlockBuilder::Block(block) => block.get_block(ctx),
            BlockBuilder::Loop(loopop) => loopop.get_block(ctx),
        }
    }
}
