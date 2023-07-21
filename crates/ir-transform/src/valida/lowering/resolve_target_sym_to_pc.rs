use anyhow::anyhow;
use ozk_valida_dialect as valida;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialect_conversion::apply_partial_conversion;
use pliron::dialect_conversion::ConversionTarget;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::operation::WalkOrder;
use pliron::operation::WalkResult;
use pliron::pass::Pass;
use pliron::pattern_match::PatternRewriter;
use pliron::pattern_match::RewritePattern;
use pliron::rewrite::RewritePatternSet;
use valida::op_interfaces::HasOperands;
use valida::op_interfaces::TrackedProgramCounter;

#[derive(Default)]
pub struct ValidaResolveTargetSymToPcPass {}

impl Pass for ValidaResolveTargetSymToPcPass {
    fn run_on_operation(&self, ctx: &mut Context, op: Ptr<Operation>) -> Result<(), anyhow::Error> {
        let target = ConversionTarget::default();
        // target.add_illegal_dialect(WASM_DIALECT(ctx));
        // target.add_legal_dialect(VALIDA_DIALECT(ctx));
        // #[allow(clippy::expect_used)]
        // target.add_legal_dialect(
        //     Dialect::get_ref(ctx, DialectName::new("builtin"))
        //         .expect("builtin dialect not registered"),
        // );
        let mut patterns = RewritePatternSet::default();
        patterns.add(Box::<ValidaResolveTargetSymToPc>::default());
        apply_partial_conversion(ctx, op, target, patterns)?;
        Ok(())
    }
}

#[derive(Default)]
struct ValidaResolveTargetSymToPc;

impl RewritePattern for ValidaResolveTargetSymToPc {
    fn match_and_rewrite(
        &self,
        ctx: &mut Context,
        op: Ptr<Operation>,
        rewriter: &mut dyn PatternRewriter,
    ) -> Result<bool, anyhow::Error> {
        let opop = &op.deref(ctx).get_op(ctx);
        let Some(program_op) = opop.downcast_ref::<valida::ops::ProgramOp>() else {
           return Ok(false);
        };

        let mut jalsym_ops = Vec::new();
        program_op
            .get_operation()
            .walk_only::<valida::ops::JalSymOp>(ctx, WalkOrder::PostOrder, &mut |op| {
                jalsym_ops.push(*op);
                WalkResult::Advance
            });

        for jalsym_op in jalsym_ops {
            let sym = jalsym_op.get_target_sym(ctx);
            let func_op = program_op
                .get_func(ctx, &sym)
                .ok_or_else(|| anyhow!("not found function for sym: {}", sym))?;
            let b = func_op.get_pc(ctx);
            let mut operands = jalsym_op.get_operands(ctx);
            operands.set_b(b.into());
            let jal_op = valida::ops::JalOp::from_operands(ctx, operands);
            rewriter.replace_op_with(ctx, jalsym_op.get_operation(), jal_op.get_operation())?;
        }

        Ok(true)
    }
}
