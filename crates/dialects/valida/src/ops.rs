use ozk_ozk_dialect::attributes::FieldElemAttr;
use pliron::attribute;
use pliron::attribute::AttrObj;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::error::CompilerError;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

declare_op!(
    /// Write the immediate values b,c,d,e to the cell located at offset a.
    Imm32Op,
    "imm32",
    "valida"
);

impl Imm32Op {
    /// Attribute key for operands.
    pub const ATTR_KEY_OPERAND_A: &str = "imm32.a";
    pub const ATTR_KEY_OPERAND_B: &str = "imm32.b";
    pub const ATTR_KEY_OPERAND_C: &str = "imm32.c";
    pub const ATTR_KEY_OPERAND_D: &str = "imm32.d";
    pub const ATTR_KEY_OPERAND_E: &str = "imm32.e";

    /// Create a new [ConstantOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(
        ctx: &mut Context,
        a: AttrObj,
        b: AttrObj,
        c: AttrObj,
        d: AttrObj,
        e: AttrObj,
    ) -> Imm32Op {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_A, a);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_B, b);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_C, c);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_D, d);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OPERAND_E, e);
        Imm32Op { op }
    }

    // /// Get the constant value that this Op defines.
    // pub fn get_value(&self, ctx: &Context) -> AttrObj {
    //     let op = self.get_operation().deref(ctx);
    //     #[allow(clippy::expect_used)]
    //     let value = op
    //         .attributes
    //         .get(Self::ATTR_KEY_OPERANDS)
    //         .expect("no attribute found");
    //     if value.is::<VecAttr>() {
    //         todo!("extract ValidaOperands from VecAttr")
    //     } else {
    //         todo!("panic?")
    //     }
    // }

    fn get_operand(&self, ctx: &Context, operand_name: &str) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::panic)]
        let value = op.attributes.get(operand_name).unwrap_or_else(|| {
            panic!("no attribute for operand '{}' found", operand_name);
        });
        #[allow(clippy::panic)]
        if value.is::<FieldElemAttr>() {
            attribute::clone::<FieldElemAttr>(value)
        } else {
            panic!("expected FieldElemAttr, found {}", value.with_ctx(ctx));
        }
    }

    pub fn get_operand_a(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_A)
    }

    pub fn get_operand_b(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_B)
    }

    pub fn get_operand_c(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_C)
    }

    pub fn get_operand_d(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_D)
    }

    pub fn get_operand_e(&self, ctx: &Context) -> AttrObj {
        self.get_operand(ctx, Self::ATTR_KEY_OPERAND_E)
    }
}

impl DisplayWithContext for Imm32Op {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}(fp) {} {} {} {}",
            self.get_opid().with_ctx(ctx),
            self.get_operand_a(ctx).with_ctx(ctx),
            self.get_operand_b(ctx).with_ctx(ctx),
            self.get_operand_c(ctx).with_ctx(ctx),
            self.get_operand_d(ctx).with_ctx(ctx),
            self.get_operand_e(ctx).with_ctx(ctx)
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
