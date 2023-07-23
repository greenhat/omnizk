#![allow(clippy::expect_used)]

use pliron::attribute;
use pliron::attribute::attr_cast;
use pliron::attribute::AttrObj;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::attributes::StringAttr;
use pliron::dialects::builtin::attributes::TypeAttr;
use pliron::dialects::builtin::types::FunctionType;
use pliron::error::CompilerError;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::Type;
use pliron::r#type::TypeObj;
use pliron::with_context::AttachContext;

use crate::attributes::apint_to_i32;
use crate::attributes::u32_attr;
use crate::attributes::FieldElemAttr;
use crate::ord_n::Ord16;
use crate::types::FuncSym;

declare_op!(
    /// Pushes numeric constant on the stack.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// |[ATTR_KEY_VALUE](ConstantOp::ATTR_KEY_VALUE) | [FieldElemAttr] or [IntegerAttr] |
    ///
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
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_VALUE, value);
        ConstantOp { op }
    }
}

impl DisplayWithContext for ConstantOp {
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
        if op.get_num_results() != 0 || op.get_num_operands() != 0 {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect number of results or operands".to_string(),
            });
        }
        Ok(())
    }
}

declare_op!(
    /// Swap top value on the stack with the value at the given index.
    SwapOp,
    "swap",
    "ozk"
);

impl SwapOp {
    pub const ATTR_KEY_INDEX: &str = "swap.index";

    /// Get the index
    pub fn get_index(&self, ctx: &Context) -> Ord16 {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let value = op
            .attributes
            .get(Self::ATTR_KEY_INDEX)
            .expect("no attribute for index found");
        let value_u32 = apint_to_i32(
            value
                .downcast_ref::<IntegerAttr>()
                .expect("index is not an IntegerAttr")
                .clone()
                .into(),
        ) as u32;
        value_u32.try_into().expect("index is not an Ord16")
    }

    /// Create a new [SwapOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, index: Ord16) -> SwapOp {
        #[allow(clippy::expect_used)]
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let attr = u32_attr(ctx, index.into());
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_INDEX, attr);
        SwapOp { op }
    }
}

impl DisplayWithContext for SwapOp {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_index(ctx)
        )
    }
}

impl Verify for SwapOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let op = &*self.get_operation().deref(ctx);
        if op.get_opid() != Self::get_opid_static() {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect OpId".to_string(),
            });
        }
        if op.get_num_results() != 0 || op.get_num_operands() != 0 {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect number of results or operands".to_string(),
            });
        }
        Ok(())
    }
}

declare_op!(
    /// Call a function
    ///
    CallOp,
    "call",
    "ozk"
);

impl CallOp {
    const ATTR_KEY_FUNC_SYM: &str = "call.func_sym";
    const ATTR_KEY_FUNC_TYPE: &str = "call.func_type";

    /// Create a new [CallOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, func_sym: FuncSym, func_type: FunctionType) -> CallOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_FUNC_SYM, StringAttr::create(func_sym.into()));
        let ty = Type::register_instance(func_type, ctx);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_FUNC_TYPE, TypeAttr::create(ty));
        CallOp { op }
    }

    /// Get the target function symbol
    pub fn get_func_sym(&self, ctx: &Context) -> String {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let func_sym_attr = op
            .attributes
            .get(Self::ATTR_KEY_FUNC_SYM)
            .expect("no attribute found");
        #[allow(clippy::expect_used)]
        let func_sym: String = func_sym_attr
            .downcast_ref::<StringAttr>()
            .expect("expected StringAttr")
            .clone()
            .into();
        func_sym
    }

    /// Get the function signature (type).
    pub fn get_func_type_attr(&self, ctx: &Context) -> Ptr<TypeObj> {
        let opref = self.get_operation().deref(ctx);
        #[allow(clippy::unwrap_used)]
        let ty_attr = opref.attributes.get(Self::ATTR_KEY_FUNC_TYPE).unwrap();
        #[allow(clippy::unwrap_used)]
        attr_cast::<dyn TypedAttrInterface>(&**ty_attr)
            .unwrap()
            .get_type()
    }

    /// Get the target function signature (type).
    pub fn get_func_type(&self, ctx: &Context) -> FunctionType {
        let func_type_obj = self.get_func_type_attr(ctx).deref(ctx);
        #[allow(clippy::panic)]
        let Some(func_type) = func_type_obj.downcast_ref::<FunctionType>() else {
            panic!("FuncOp type is not a FunctionType");
        };
        func_type.clone()
    }
}

impl DisplayWithContext for CallOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_func_sym(ctx)
        )
    }
}

impl Verify for CallOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let op = &*self.get_operation().deref(ctx);
        if op.get_opid() != Self::get_opid_static() {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect OpId".to_string(),
            });
        }
        if op.get_num_results() != 0 || op.get_num_operands() != 0 {
            return Err(CompilerError::VerificationError {
                msg: "Incorrect number of results or operands".to_string(),
            });
        }
        Ok(())
    }
}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    ConstantOp::register(ctx, dialect);
    SwapOp::register(ctx, dialect);
    CallOp::register(ctx, dialect);
}
