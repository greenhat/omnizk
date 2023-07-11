use ozk_valida_dialect as valida;
use ozk_wasm_dialect as wasm;
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
        let prog_op = valida::ops::ProgramOp::new(ctx, func_ops);
        rewriter.replace_op_with(ctx, wasm_module_op.get_operation(), prog_op.get_operation())?;
        Ok(())
    }
}
