use anyhow::anyhow;
use ozk_valida_dialect as valida;
use ozk_wasm_dialect as wasm;
use pliron::basic_block::BasicBlock;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use valida::types::Operands;

#[derive(Default)]
pub struct WasmToValidaModuleLoweringPass;

impl Pass for WasmToValidaModuleLoweringPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // TODO: set illegal ops
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ModuleLowering>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct ModuleLowering {}

impl RewritePattern for ModuleLowering {
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
        let opop = &op.deref(ctx).get_op(ctx);
        #[allow(clippy::panic)]
        let Some(wasm_module_op) = opop.downcast_ref::<wasm::ops::ModuleOp>() else {
            panic!("expected ModuleOp");
        };
        let mut func_ops = Vec::new();
        for func_op in wasm_module_op.get_body(ctx, 0).deref(ctx).iter(ctx) {
            func_ops.push(func_op);
        }
        // TODO: reverse toposort should bubble main func on top. Do we really need that?
        for op in &func_ops {
            op.unlink(ctx);
        }
        let Some(main_func_sym) = wasm_module_op.get_start_func_sym(ctx) else {
            return Err(anyhow!("error. no start function in module"));
        };
        let entry_block = build_prog_entry_block(ctx, main_func_sym.into());
        let prog_op = valida::ops::ProgramOp::new(ctx, entry_block, func_ops);
        rewriter.replace_op_with(ctx, wasm_module_op.get_operation(), prog_op.get_operation())?;
        Ok(())
    }
}

fn build_prog_entry_block(ctx: &mut Context, main_func_sym: String) -> Ptr<BasicBlock> {
    let bb = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
    // call the main function
    let size_of_current_stack = 16;
    let call_frame_size = 12;
    let b = size_of_current_stack + call_frame_size;
    // let imm32_op = valida::ops::Imm32Op::new_unlinked(ctx, Operands::from_i32(-b + 8, 0, 0, 0, b));
    let imm32_op = valida::ops::Imm32Op::new_unlinked(ctx, Operands::from_i32(-b + 4, 0, 0, 0, b));
    imm32_op.get_operation().insert_at_back(bb, ctx);
    let jal_op = valida::ops::JalSymOp::new(ctx, -b, -b, main_func_sym);
    jal_op.get_operation().insert_at_back(bb, ctx);
    // let sw_op = valida::ops::SwOp::new_unlinked(ctx, Operands::from_i32(0, 4, -24, 0, 0));
    // todo!("return op should sw the return value in arg 1 fp place and 'return jalv' should set the fp accordingly");
    // TODO: it means that return op should know the arg number of the function it exits
    let sw_op = valida::ops::SwOp::new(ctx, 4, -20);
    sw_op.get_operation().insert_at_back(bb, ctx);
    let exit_op = valida::ops::ExitOp::new_unlinked(ctx);
    exit_op.get_operation().insert_at_back(bb, ctx);
    bb
}

/*
pub fn topo_sort_functions(
    ctx: &Context,
    procedures: impl Iterator<Item = FuncOp>,
) -> Result<impl Iterator<Item = String>, TopoSortError> {
    let mut topo_sort = TopologicalSort::new();

    for proc in procedures {
        let proc_name = proc.get_symbol_name(ctx);
        topo_sort.insert(proc_name.clone());
        for dep in get_callees_syms(ctx, proc.get_operation()) {
            topo_sort.add_dependency(dep, proc_name.clone());
        }
    }
    let mut sorted = Vec::new();
    while !topo_sort.is_empty() {
        let mut proc_names = topo_sort.pop_all();
        if proc_names.is_empty() {
            return Err(TopoSortError::Cycle(topo_sort));
        }
        proc_names.sort();
        sorted.append(&mut proc_names);
    }
    Ok(sorted.into_iter())
}

#[derive(Debug, Error)]
pub enum TopoSortError {
    #[error("Cycle in function dependencies: {0:?}")]
    Cycle(TopologicalSort<String>),
}
*/
