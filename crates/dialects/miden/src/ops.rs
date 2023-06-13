#![allow(unused_imports)]

use intertrait::cast_to;
use pliron::attribute;
use pliron::attribute::attr_cast;
use pliron::attribute::AttrObj;
use pliron::basic_block::BasicBlock;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::dialects::builtin::attributes::FloatAttr;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::attributes::StringAttr;
use pliron::dialects::builtin::attributes::TypeAttr;
use pliron::dialects::builtin::op_interfaces::OneRegionInterface;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::dialects::builtin::types::FunctionType;
use pliron::dialects::builtin::types::IntegerType;
use pliron::dialects::builtin::types::Signedness;
use pliron::error::CompilerError;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use pliron::with_context::AttachContext;

use crate::attributes::FieldElemAttr;

declare_op!(
    /// Represents a Miden module, a top level container operation.
    ///
    /// See MLIR's [builtin.module](https://mlir.llvm.org/docs/Dialects/Builtin/#builtinmodule-mlirmoduleop).
    /// It contains a single [SSACFG](super::op_interfaces::RegionKind::SSACFG)
    /// region containing a single block which can contain any operations and
    /// does not have a terminator.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// | [ATTR_KEY_SYM_NAME](super::ATTR_KEY_SYM_NAME) | [StringAttr](super::attributes::StringAttr) |
    /// | [ATTR_KEY_START_FUNC_SYM](ModuleOp::ATTR_KEY_START_FUNC_SYM) | [StringAttr](super::attributes::StringAttr) |
    ModuleOp,
    "module",
    "miden"
);

impl DisplayWithContext for ModuleOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let region = self.get_region(ctx).with_ctx(ctx).to_string();
        write!(
            f,
            "{} @{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            self.get_symbol_name(ctx),
            indent::indent_all_by(2, region),
        )
    }
}

impl Verify for ModuleOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        self.verify_interfaces(ctx)?;
        self.get_region(ctx).deref(ctx).verify(ctx)
    }
}

impl ModuleOp {
    /// Attribute key for the the start function symbol.
    pub const ATTR_KEY_START_FUNC_SYM: &str = "module.start_func_sym";

    /// Create a new [ModuleOp].
    /// The underlying [Operation] is not linked to a [BasicBlock](crate::basic_block::BasicBlock).
    /// The returned module has a single [crate::region::Region] with a single (BasicBlock)[crate::basic_block::BasicBlock].
    pub fn new(ctx: &mut Context, name: &str, start_func_name: String) -> ModuleOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);

        {
            let opref = &mut *op.deref_mut(ctx);
            // Set function type attributes.
            opref.attributes.insert(
                Self::ATTR_KEY_START_FUNC_SYM,
                StringAttr::create(start_func_name),
            );
        }

        let opop = ModuleOp { op };
        opop.set_symbol_name(ctx, name);

        // Create an empty block.
        let region = opop.get_region(ctx);
        let block = BasicBlock::new(ctx, None, vec![]);
        block.insert_at_front(region, ctx);

        opop
    }

    /// Add an [Operation] into this module.
    pub fn add_operation(&self, ctx: &mut Context, op: Ptr<Operation>) {
        self.append_operation(ctx, op, 0)
    }

    #[allow(clippy::expect_used)]
    pub fn get_start_func_sym(&self, ctx: &Context) -> String {
        let self_op = self.get_operation().deref(ctx);
        let s_attr = self_op
            .attributes
            .get(Self::ATTR_KEY_START_FUNC_SYM)
            .expect("ModuleOp has no start function symbol attribute");
        String::from(
            s_attr
                .downcast_ref::<StringAttr>()
                .expect("ModuleOp start function symbol attribute is not a StringAttr")
                .clone(),
        )
    }
}

impl OneRegionInterface for ModuleOp {}
impl SingleBlockRegionInterface for ModuleOp {}
#[cast_to]
impl SymbolOpInterface for ModuleOp {}

declare_op!(
    /// An operation with a name containing a single region.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// | [ATTR_KEY_SYM_NAME](super::ATTR_KEY_SYM_NAME) | [StringAttr](super::attributes::StringAttr) |
    /// | [ATTR_KEY_FUNC_TYPE](FuncOp::ATTR_KEY_FUNC_TYPE) | [TypeAttr](super::attributes::TypeAttr) |
    FuncOp,
    "func",
    "miden"
);

impl FuncOp {
    /// Attribute key for the function type
    pub const ATTR_KEY_FUNC_TYPE: &str = "func.type";

    /// Create a new [FuncOp].
    /// The underlying [Operation] is not linked to a [BasicBlock](crate::basic_block::BasicBlock).
    /// The returned function has a single region with an empty `entry` block.
    pub fn new_unlinked(ctx: &mut Context, name: &str, ty: Ptr<TypeObj>) -> FuncOp {
        let ty_attr = TypeAttr::create(ty);
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        {
            let opref = &mut *op.deref_mut(ctx);
            // Set function type attributes.
            opref.attributes.insert(Self::ATTR_KEY_FUNC_TYPE, ty_attr);
        }
        let opop = FuncOp { op };
        // Create an empty entry block.
        #[allow(clippy::expect_used)]
        let region = opop.get_region(ctx);
        let body = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
        body.insert_at_front(region, ctx);

        opop.set_symbol_name(ctx, name);

        opop
    }

    /// Get the function signature (type).
    pub fn get_type(&self, ctx: &Context) -> Ptr<TypeObj> {
        let opref = self.get_operation().deref(ctx);
        #[allow(clippy::unwrap_used)]
        let ty_attr = opref.attributes.get(Self::ATTR_KEY_FUNC_TYPE).unwrap();
        #[allow(clippy::unwrap_used)]
        attr_cast::<dyn TypedAttrInterface>(&**ty_attr)
            .unwrap()
            .get_type()
    }

    /// Get the entry block of this function.
    pub fn get_entry_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        #[allow(clippy::unwrap_used)]
        self.get_region(ctx).deref(ctx).get_head().unwrap()
    }

    /// Get an iterator over all operations.
    pub fn op_iter<'a>(&self, ctx: &'a Context) -> impl Iterator<Item = Ptr<Operation>> + 'a {
        self.get_region(ctx)
            .deref(ctx)
            .iter(ctx)
            .flat_map(|bb| bb.deref(ctx).iter(ctx))
    }
}

impl OneRegionInterface for FuncOp {}
#[cast_to]
impl SymbolOpInterface for FuncOp {}

impl DisplayWithContext for FuncOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let region = self.get_region(ctx).with_ctx(ctx).to_string();
        write!(
            f,
            "{} @{}{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            self.get_symbol_name(ctx),
            self.get_type(ctx).with_ctx(ctx),
            indent::indent_all_by(2, region),
        )
    }
}

impl Verify for FuncOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let ty = self.get_type(ctx);

        if !(ty.deref(ctx).is::<FunctionType>()) {
            return Err(CompilerError::VerificationError {
                msg: "Unexpected Func type".to_string(),
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
        self.verify_interfaces(ctx)?;
        self.get_entry_block(ctx).verify(ctx)?;
        Ok(())
    }
}

declare_op!(
    /// Pushes numeric constant on the stack.
    ConstantOp,
    "constant",
    "miden"
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
    pub fn new_unlinked(ctx: &mut Context, value: FieldElemAttr) -> ConstantOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_VALUE, Box::new(value));
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
        if !value.is::<FieldElemAttr>() {
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

// TODO: store expected operand types (poped from stack)?

declare_op!(
    /// Push two top stack items, sums them and push result on stack
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// | [ATTR_KEY_OP_TYPE](FuncOp::ATTR_KEY_OP_TYPE) | [TypeAttr](super::attributes::TypeAttr) |
    ///
    AddOp,
    "add",
    "miden"
);

impl AddOp {
    /// Attribute key
    pub const ATTR_KEY_OP_TYPE: &str = "add.type";
    /// Create a new [AddOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, ty: Ptr<TypeObj>) -> ConstantOp {
        let ty_attr = TypeAttr::create(ty);
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_OP_TYPE, ty_attr);
        ConstantOp { op }
    }

    pub fn get_type(&self, ctx: &Context) -> Ptr<TypeObj> {
        let opref = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let ty_attr = opref
            .attributes
            .get(Self::ATTR_KEY_OP_TYPE)
            .expect("no type attribute");
        #[allow(clippy::expect_used)]
        attr_cast::<dyn TypedAttrInterface>(&**ty_attr)
            .expect("invalid type attribute")
            .get_type()
    }
}

impl DisplayWithContext for AddOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_opid().with_ctx(ctx),)
    }
}

impl Verify for AddOp {
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
    /// Call a function.
    ///
    /// https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-control
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// | [ATTR_KEY_SYM_NAME](super::ATTR_KEY_SYM_NAME) | [StringAttr](super::attributes::StringAttr) |
    ///
    CallOp,
    "call",
    "miden"
);

impl CallOp {
    /// Attribute key for the callee symbol name.
    pub const ATTR_KEY_CALLEE_SYM: &str = "call.callee_sym";

    /// Get the callee symbol name.
    pub fn get_callee_sym(&self, ctx: &Context) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let callee_sym = op
            .attributes
            .get(Self::ATTR_KEY_CALLEE_SYM)
            .expect("no attribute found");
        attribute::clone::<StringAttr>(callee_sym)
    }

    /// Create a new [CallOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, callee_name: String) -> CallOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let callee_sym = StringAttr::create(callee_name);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_CALLEE_SYM, callee_sym);
        CallOp { op }
    }
}

impl DisplayWithContext for CallOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_callee_sym(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for CallOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let callee_sym = self.get_callee_sym(ctx);
        if !callee_sym.is::<StringAttr>() {
            return Err(CompilerError::VerificationError {
                msg: "Unexpected callee symbol type".to_string(),
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
    /// Push local variable with the given index onto the stack.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// |[ATTR_KEY_INDEX](Self::ATTR_KEY_INDEX) | [IntegerAttr] |
    ///
    LocalGetOp,
    "local.get",
    "miden"
);

impl LocalGetOp {
    /// Attribute key for the index
    pub const ATTR_KEY_INDEX: &str = "local.get.index";

    /// Get the index of the local variable.
    pub fn get_index(&self, ctx: &Context) -> AttrObj {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let value = op
            .attributes
            .get(Self::ATTR_KEY_INDEX)
            .expect("no attribute found");
        attribute::clone::<IntegerAttr>(value)
    }

    /// Create a new [LocalGetOp].
    pub fn new_unlinked(ctx: &mut Context, index: AttrObj) -> LocalGetOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_INDEX, index);
        LocalGetOp { op }
    }
}

impl DisplayWithContext for LocalGetOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_index(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for LocalGetOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        let index = self.get_index(ctx);
        if let Ok(index_attr) = index.downcast::<IntegerAttr>() {
            #[allow(clippy::unwrap_used)]
            if index_attr.get_type()
                != IntegerType::get_existing(ctx, 32, Signedness::Unsigned).unwrap()
            {
                return Err(CompilerError::VerificationError {
                    msg: "Expected u32 for index".to_string(),
                });
            }
        } else {
            return Err(CompilerError::VerificationError {
                msg: "Unexpected index type".to_string(),
            });
        };
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
    AddOp::register(ctx, dialect);
    CallOp::register(ctx, dialect);
    LocalGetOp::register(ctx, dialect);
}
