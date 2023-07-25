//! FuncOp builder

use ozk_ozk_dialect::types::FuncSym;
use ozk_wasm_dialect::ops::FuncOp;
use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use thiserror::Error;

use crate::func_op_builder::FuncOpBuilder;

// TODO: move to func dialect crate?
/// FuncOp builder
pub struct FuncBuilder {
    name: FuncSym,
    sig: Option<Ptr<TypeObj>>,
    blocks: Vec<Ptr<BasicBlock>>,
    position: Option<Ptr<BasicBlock>>,
}

impl FuncBuilder {
    /// Create a new FuncBuilder
    pub fn new(name: FuncSym) -> FuncBuilder {
        FuncBuilder {
            name,
            sig: None,
            blocks: Vec::new(),
            position: None,
        }
    }

    pub fn create_block(&mut self, ctx: &mut Context, label: Option<String>) -> Ptr<BasicBlock> {
        let block = BasicBlock::new(ctx, label, Vec::new());
        self.blocks.push(block);
        block
    }

    pub fn switch_to_block(&mut self, block: Ptr<BasicBlock>) {
        self.position = Some(block);
    }

    pub fn op(&mut self) -> FuncOpBuilder {
        // #[allow(clippy::expect_used)]
        // let block = self
        //     .position
        //     .expect("Please call switch_to_block before inserting instructions");
        FuncOpBuilder::new(self)
    }

    pub fn append(&mut self, ctx: &mut Context, op: Ptr<Operation>) {
        #[allow(clippy::expect_used)]
        let block = self
            .position
            .expect("position is not set, call switch_to_block before inserting instructions");
        op.insert_at_back(block, ctx);
    }

    /// Builds and returns the FuncOp
    pub fn build(self, _ctx: &mut Context) -> Result<FuncOp, FuncBuilderError> {
        // let sig = self.sig.ok_or_else(|| {
        //     FuncBuilderError::MissingSignature(format!("FuncBuilder for {:?}", self.name))
        // })?;
        todo!();
    }

    /// Sets the function signature
    pub fn set_signature(&mut self, signature: Ptr<TypeObj>) {
        self.sig = Some(signature);
    }

    /// Sets the function name
    pub fn set_name(&mut self, clone: FuncSym) {
        self.name = clone;
    }
}

// Error type for FuncBuilder
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum FuncBuilderError {
    #[error("missing function signature")]
    MissingSignature(String),
}
