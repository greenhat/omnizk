use ozk_wasm_dialect::ops::BlockOp;
use ozk_wasm_dialect::ops::FuncOp;
use ozk_wasm_dialect::ops::LoopOp;
use ozk_wasm_dialect::types::FuncSym;
use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use pliron::with_context::AttachContext;
use thiserror::Error;

use crate::op_builder::OpBuilder;

pub struct FuncBuilder {
    name: FuncSym,
    sig: Option<Ptr<TypeObj>>,
    blocks: Vec<BlockBuilder>,
    locals: Vec<Ptr<TypeObj>>,
}

impl FuncBuilder {
    pub fn new(ctx: &mut Context, name: FuncSym) -> FuncBuilder {
        FuncBuilder {
            name,
            sig: None,
            locals: Vec::new(),
            blocks: vec![BlockBuilder::FuncEntryBlock(BasicBlock::new(
                ctx,
                Some("entry".to_string()),
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

    pub fn build(mut self, ctx: &mut Context) -> Result<FuncOp, FuncBuilderError> {
        let sig = self.sig.ok_or_else(|| {
            FuncBuilderError::MissingSignature(format!("FuncBuilder for {:?}", self.name))
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

        match self.blocks.pop() {
            Some(BlockBuilder::FuncEntryBlock(entry_bb)) => {
                let func_op =
                    FuncOp::new_unlinked_with_block(ctx, self.name.clone(), sig, entry_bb);
                Ok(func_op)
            }
            _ => todo!("error"),
        }
    }

    pub fn op(&mut self) -> OpBuilder {
        OpBuilder::new(self)
    }

    pub fn push(&mut self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), FuncBuilderError> {
        // dbg!(op.with_ctx(ctx).to_string());
        let opop = &op.deref(ctx).get_op(ctx);
        if let Some(block) = opop.downcast_ref::<BlockOp>() {
            self.blocks.push(BlockBuilder::Block(*block));
        } else if let Some(loopop) = opop.downcast_ref::<LoopOp>() {
            self.blocks.push(BlockBuilder::Loop(*loopop));
        } else {
            let current_bb = self
                .blocks
                .last()
                .ok_or(FuncBuilderError::PushOnEmptyBlocks(
                    op.with_ctx(ctx).to_string(),
                ))?
                .get_bb(ctx);
            op.insert_at_back(current_bb, ctx);
        }
        Ok(())
    }

    pub fn push_end(&mut self, ctx: &mut Context) -> Result<(), FuncBuilderError> {
        if let Some(ending_block_builder) = self.blocks.pop() {
            match ending_block_builder {
                BlockBuilder::FuncEntryBlock(entry_bb) => {
                    // it's function end, add it back to the stack as FuncEntryBlock
                    // TODO: ugly. fix it.
                    self.blocks.push(BlockBuilder::FuncEntryBlock(entry_bb));
                    Ok(())
                }
                BlockBuilder::Block(block) => {
                    let current_bb = self
                        .blocks
                        .last()
                        .ok_or(FuncBuilderError::PushOnEmptyBlocks(
                            block.with_ctx(ctx).to_string(),
                        ))?
                        .get_bb(ctx);
                    block.get_operation().insert_at_back(current_bb, ctx);
                    Ok(())
                }
                BlockBuilder::Loop(loopop) => {
                    let current_bb = self
                        .blocks
                        .last()
                        .ok_or(FuncBuilderError::PushOnEmptyBlocks(
                            loopop.with_ctx(ctx).to_string(),
                        ))?
                        .get_bb(ctx);
                    loopop.get_operation().insert_at_back(current_bb, ctx);
                    Ok(())
                }
            }
        } else {
            Err(FuncBuilderError::PushOnEmptyBlocks(
                "push_end called on empty blocks".into(),
            ))
        }
    }

    // pub fn push_insts(&mut self, insts: Vec<Inst>) {
    //     self.ins.extend(insts);
    // }

    pub fn set_signature(&mut self, signature: Ptr<TypeObj>) {
        self.sig = Some(signature);
    }

    pub fn set_name(&mut self, clone: FuncSym) {
        self.name = clone;
    }
}

#[derive(Debug, Error)]
pub enum FuncBuilderError {
    #[error("missing function signature")]
    MissingSignature(String),
    #[error("pushing {0} to empty block stack")]
    PushOnEmptyBlocks(String),
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
