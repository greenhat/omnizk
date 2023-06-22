use bounded_vec::NonEmptyVec;
use derive_more::From;
use ozk_miden_dialect::ops as miden;
use ozk_wasm_dialect::ops as wasm;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;

#[derive(Default)]
pub struct WasmToMidenCFLoweringPass;

impl Pass for WasmToMidenCFLoweringPass {
    fn name(&self) -> &str {
        "WasmToMidenCFLoweringPass"
    }

    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ControlFlowLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

/// Converts Wasm module  into Miden program
/// converting Wasm blocks/loops and branching ops into Miden functions
#[derive(Default)]
struct ControlFlowLowering;

impl RewritePattern for ControlFlowLowering {
    fn match_op(&self, ctx: &Context, op: Ptr<Operation>) -> Result<bool, anyhow::Error> {
        Ok(op
            .deref(ctx)
            .get_op(ctx)
            .downcast_ref::<wasm::ModuleOp>()
            .is_some())
    }

    fn rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<(), anyhow::Error> {
        let Ok(module_op) = op
            .deref(ctx)
            .get_op(ctx)
            .downcast::<wasm::ModuleOp>() else {
            todo!("error");
        };
        let body = module_op.get_body(ctx, 0);
        let mut funcs = Vec::new();
        for op in body.deref(ctx).iter(ctx) {
            let Ok(func_op) = op
                    .deref(ctx)
                    .get_op(ctx)
                    .downcast::<wasm::FuncOp>() else {
                todo!("error. there should be only func ops in module body");
            };
            funcs.push(func_op);
        }
        let main_proc_op = miden::ProcOp::new_unlinked(ctx, "ozk_miden_main_proc");
        let start_func_call_op =
            miden::CallOp::new_unlinked(ctx, module_op.get_start_func_sym(ctx));
        start_func_call_op
            .get_operation()
            .insert_at_back(main_proc_op.get_entry_block(ctx), ctx);
        let prog_op = miden::ProgramOp::new(ctx, main_proc_op);
        // TODO: make a new pass for module->prog conversion
        // plus, handle there imports and all other module stuff
        for func_op in funcs {
            let root_proc_op = miden::ProcOp::new_unlinked(ctx, &func_op.get_symbol_name(ctx));
            let root_proc_bb = root_proc_op.get_entry_block(ctx);
            prog_op.add_operation(ctx, root_proc_op.get_operation());
            let mut func_ops = Vec::new();
            for op in func_op.op_iter(ctx) {
                func_ops.push(op.deref(ctx).get_op(ctx));
            }
            for op in func_ops {
                if let Some(block_op) = op.downcast_ref::<wasm::BlockOp>() {
                    let proc_ops = convert_block_to_proc(ctx, block_op.into())?;
                    block_op.get_operation().unlink(ctx);
                    let callee_proc_op = proc_ops.first();
                    let call_op =
                        miden::CallOp::new_unlinked(ctx, callee_proc_op.get_symbol_name(ctx));
                    call_op.get_operation().insert_at_back(root_proc_bb, ctx);
                    for proc_op in proc_ops {
                        prog_op.add_operation(ctx, proc_op.get_operation());
                    }
                }
                if op.downcast_ref::<wasm::ReturnOp>().is_some() {
                    // return in the entry block means that the rest of the ops
                    // are unreachable and can be removed
                    rewriter.erase_op(ctx, op.get_operation())?;
                    // TODO: erase the rest of the ops in this block
                    break;
                } else {
                    op.get_operation().unlink(ctx);
                    op.get_operation().insert_at_back(root_proc_bb, ctx);
                };
            }
            rewriter.erase_op(ctx, func_op.get_operation())?;
        }
        rewriter.replace_op_with(ctx, module_op.get_operation(), prog_op.get_operation())?;
        Ok(())
    }
}

#[derive(From)]
enum WasmStructuredOp<'a> {
    Block(&'a wasm::BlockOp),
    Loop(&'a wasm::LoopOp),
    // If(Box<wasm::ops::IfOp>),
}

fn convert_block_to_proc(
    ctx: &mut Context,
    _struct_op: WasmStructuredOp,
) -> Result<NonEmptyVec<Box<miden::ProcOp>>, anyhow::Error> {
    // TODO: check that all locals are converted to mem access
    // Repeat this process for every block in the extracted functions recursively.
    // TODO: expose the block label and get it here
    let func_name = "block label"; // struct_op.get_symbol_name(ctx);
    let proc_op = miden::ProcOp::new_unlinked(ctx, func_name);
    let proc_ops = vec![Box::new(proc_op)];
    // let mut block_ops = Vec::new();
    // struct_op.op_iter(ctx).for_each(|op| {
    //     if let Some(block_op) = op
    //         .deref(ctx)
    //         .get_op(ctx)
    //         .downcast_ref::<wasm::ops::BlockOp>()
    //     {
    //         block_ops.push(block_op);
    //     }
    // });
    // proc_ops.push(Box::new(proc_op));
    // TODO: swap the func operation from WebAssembly into procedure operation in Miden.
    match NonEmptyVec::from_vec(proc_ops) {
        Ok(ops) => Ok(ops),
        Err(_) => todo!("error"),
    }
}
