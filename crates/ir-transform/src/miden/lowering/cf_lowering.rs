use ozk_miden_dialect as miden;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::operation::Operation;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;

/// Converts Wasm module  into Miden program
/// converting Wasm blocks/loops and branching ops into Miden functions
#[derive(Default)]
pub struct ControlFlowLowering {}

impl RewritePattern for ControlFlowLowering {
    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ops::ModuleOp>()
            .is_some())
    }

    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        if let Some(module_op) = &op
            .deref(ctx)
            .get_op(ctx)
            .downcast_mut::<wasm::ops::ModuleOp>()
        {
            let body = module_op.get_body(ctx, 0);
            for op in body.deref(ctx).iter(ctx) {
                if let Some(func_op) = op
                    .deref(ctx)
                    .get_op(ctx)
                    .downcast_ref::<wasm::ops::FuncOp>()
                {
                    // TODO: check that all locals are converted to mem access
                    todo!("extract func blocks and loops into miden functions");
                    // TODO: traverse the function body and for every block extract the block's body into the separate function, replace the whole block with a call to this function and after the call put operations to check the exit depth and whether we need to exit the current function.
                    // Repeat this process for every block in the extracted functions recursively.
                } else {
                    todo!("error. there should be only func ops in module body");
                }
            }
        } else {
            todo!("error");
        }
        todo!("swap wasm module op into miden program op");
        Ok(())
    }
}
