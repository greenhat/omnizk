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
    fn name(&self) -> &str {
        "WasmToValidaModuleLoweringPass"
    }

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
    fn name(&self) -> String {
        "ModuleLowering".to_string()
    }

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
        // TODO: reverse toposort should bubble main func on top
        for op in &func_ops {
            op.unlink(ctx);
        }
        let entry_block = build_prog_entry_block(ctx);
        let prog_op = valida::ops::ProgramOp::new(ctx, entry_block, func_ops);
        rewriter.replace_op_with(ctx, wasm_module_op.get_operation(), prog_op.get_operation())?;
        Ok(())
    }
}

fn build_prog_entry_block(ctx: &mut Context) -> Ptr<BasicBlock> {
    let bb = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
    // call the main function
    let size_of_current_stack = 16;
    let call_frame_size = 12;
    // call label is a pseudo op which consist of:
    // imm32 (-b+8)(fp), 0, 0, 0, b(fp)
    // jal -b(fp), label, -b(fp)
    // , where b is the size of the current stack frame plus the call frame size for instantiating a call to label
    let b = size_of_current_stack + call_frame_size;
    let main_func_pc = 4;
    // pc == 0
    let imm32_op = valida::ops::Imm32Op::new_unlinked(ctx, Operands::from_i32(-b + 8, 0, 0, 0, b));
    imm32_op.get_operation().insert_at_back(bb, ctx);
    // builder.imm32(Operands::from_i32(-b + 8, 0, 0, 0, b));
    // pc == 1
    let jal_op =
        valida::ops::JalOp::new_unlinked(ctx, Operands::from_i32(-b, main_func_pc, -b, 0, 0));
    jal_op.get_operation().insert_at_back(bb, ctx);
    // builder.jal(Operands::from_i32(-b, main_func_pc, -b, 0, 0));
    // pc == 2
    let sw_op = valida::ops::SwOp::new_unlinked(ctx, Operands::from_i32(0, 4, -24, 0, 0));
    sw_op.get_operation().insert_at_back(bb, ctx);
    // builder.sw(Operands::from_i32(0, 4, -24, 0, 0));
    // pc == 3
    let exit_op = valida::ops::ExitOp::new_unlinked(ctx);
    exit_op.get_operation().insert_at_back(bb, ctx);
    // builder.exit();
    // pc == 4 the start of the next(main) function
    bb
}
