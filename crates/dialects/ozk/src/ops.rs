use pliron::attribute;
use pliron::attribute::attr_cast;
use pliron::attribute::AttrObj;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Named;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::op_interfaces::OneResultInterface;
use pliron::error::CompilerError;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use pliron::use_def_lists::Value;
use pliron::with_context::AttachContext;

use crate::attributes::FieldElemAttr;

declare_op!(
    /// Pushes numeric constant on the stack.
    /// See MLIR's [arith.constant](https://mlir.llvm.org/docs/Dialects/ArithOps/#arithconstant-mlirarithconstantop).
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// |[ATTR_KEY_VALUE](ConstantOp::ATTR_KEY_VALUE) | [FieldElemAttr] or [IntegerAttr] |
    ///
    /// Results:
    ///
    /// | result | description |
    /// |-----|-------|
    /// | `result` | any type |
    ConstantOp,
    "constant",
    "ozk"
);

impl ConstantOp {
    /// Attribute key for the constant value.
    pub const ATTR_KEY_VALUE: &str = "constant.value";
    /// Get the constant value that this Op defines.
    pub fn get_value(&self, ctx: &Context) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let value = op
            .attributes
            .get(Self::ATTR_KEY_VALUE)
            .expect("no attribute found");
        if value.is::<IntegerAttr>() {
            attribute::clone::<IntegerAttr>(value)
        } else {
            attribute::clone::<FieldElemAttr>(value)
        }
    }

    /// Create a new [ConstantOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, value: AttrObj) -> ConstantOp {
        #[allow(clippy::expect_used)]
        let result_type = attr_cast::<dyn TypedAttrInterface>(&*value)
            .expect("ConstantOp const value must provide TypedAttrInterface")
            .get_type();
        let op = Operation::new(ctx, Self::get_opid_static(), vec![result_type], vec![]);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_VALUE, value);
        ConstantOp { op }
    }
}

impl AttachContext for ConstantOp {}
impl DisplayWithContext for ConstantOp {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} = {} {}",
            self.get_operation()
                .deref(ctx)
                .get_result(0)
                .expect("no result found")
                .get_name(ctx),
            self.get_opid().with_ctx(ctx),
            self.get_value(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for ConstantOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let value = self.get_value(ctx);
        if !(value.is::<IntegerAttr>() || value.is::<FieldElemAttr>()) {
            return Err(CompilerError::VerificationError {
                msg: "Unexpected constant type".to_string(),
            });
        }
        let op = &*self.get_operation().deref(ctx);
        if op.get_opid() != Self::get_opid_static() {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect OpId".to_string(),
            });
        }
        if op.get_num_results() != 1 || op.get_num_operands() != 0 {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect number of results or operands".to_string(),
            });
        }
        Ok(())
    }
}

impl OneResultInterface for ConstantOp {
    fn get_result(&self, ctx: &Context) -> Value {
        #[allow(clippy::expect_used)]
        self.get_operation()
            .deref(ctx)
            .get_result(0)
            .expect("ConstantOp must have one result")
    }

    fn get_type(&self, ctx: &Context) -> Ptr<TypeObj> {
        #[allow(clippy::expect_used)]
        self.get_operation()
            .deref(ctx)
            .get_type(0)
            .expect("ConstantOp must have one result")
    }
}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    ConstantOp::register(ctx, dialect);
}
