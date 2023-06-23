use ozk_ozk_dialect::attributes::ValidaFieldElem;
use pliron::attribute::AttrObj;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attributes::VecAttr;
use pliron::error::CompilerError;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

pub struct ValidaOperands {
    pub a: ValidaFieldElem,
    pub b: ValidaFieldElem,
    pub c: ValidaFieldElem,
    pub d: ValidaFieldElem,
    pub e: ValidaFieldElem,
}

declare_op!(
    /// Write the immediate values b,c,d,e to the cell located at offset a.
    Imm32Op,
    "imm32",
    "valida"
);

impl Imm32Op {
    /// Attribute key for operands.
    pub const ATTR_KEY_OPERANDS: &str = "imm32.operands";

    /// Get the constant value that this Op defines.
    pub fn get_value(&self, ctx: &Context) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let value = op
            .attributes
            .get(Self::ATTR_KEY_OPERANDS)
            .expect("no attribute found");
        if value.is::<VecAttr>() {
            todo!("extract ValidaOperands from VecAttr")
        } else {
            todo!("panic?")
        }
    }

    /// Create a new [ConstantOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, value: AttrObj) -> Imm32Op {
        #[allow(clippy::expect_used)]
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERANDS, value);
        Imm32Op { op }
    }
}

impl DisplayWithContext for Imm32Op {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_value(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for Imm32Op {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    Imm32Op::register(ctx, dialect);
}
