#![allow(unused_imports)]

use std::ops::Deref;

use apint::ApInt;
use intertrait::cast_to;
use ozk_ozk_dialect::types::FuncSym;
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
use pliron::dialects::builtin::op_interfaces::CallOpInterface;
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
    /// Represents a Miden program
    ProgramOp,
    "program",
    "miden"
);

impl DisplayWithContext for ProgramOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let region = self.get_region(ctx).with_ctx(ctx).to_string();
        write!(
            f,
            "{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            indent::indent_all_by(2, region),
        )
    }
}

impl Verify for ProgramOp {
    fn verify(&self, ctx: &Context) -> Result<(), CompilerError> {
        self.verify_interfaces(ctx)?;
        self.get_region(ctx).deref(ctx).verify(ctx)
    }
}

impl ProgramOp {
    /// Attribute key for the main proc symbol.
    pub const ATTR_KEY_MAIN_PROC_SYM: &'static str = "program.main_proc_sym";

    /// Create a new [ProgramOP].
    /// The returned programm has a single [crate::region::Region] with a single (BasicBlock)[crate::basic_block::BasicBlock].
    pub fn new(ctx: &mut Context, main_proc: ProcOp) -> ProgramOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let main_proc_name = main_proc.get_symbol_name(ctx);
        {
            let opref = &mut *op.deref_mut(ctx);
            opref.attributes.insert(
                Self::ATTR_KEY_MAIN_PROC_SYM,
                StringAttr::create(main_proc_name),
            );
        }
        let opop = ProgramOp { op };
        // Create an empty block.
        let region = opop.get_region(ctx);
        let block = BasicBlock::new(ctx, None, vec![]);
        main_proc.get_operation().insert_at_back(block, ctx);
        block.insert_at_front(region, ctx);
        opop
    }

    #[allow(clippy::unwrap_used)]
    pub fn get_main_proc_sym(&self, ctx: &Context) -> String {
        let self_op = self.get_operation().deref(ctx);
        let attr = self_op
            .attributes
            .get(Self::ATTR_KEY_MAIN_PROC_SYM)
            .unwrap();
        String::from(attr.downcast_ref::<StringAttr>().unwrap().clone())
    }

    /// Add an [ProcOp] into this program.
    pub fn add_proc_op(&self, ctx: &mut Context, proc_op: ProcOp) {
        // TODO: check for procedure name clashes with existing procedures?
        // In case the procedure with the same name exist, return an error or substitute the old one?
        self.append_operation(ctx, proc_op.get_operation(), 0)
    }
}

impl OneRegionInterface for ProgramOp {}
impl SingleBlockRegionInterface for ProgramOp {}

declare_op!(
    /// An operation representing a procedure in Miden
    ProcOp,
    "proc",
    "miden"
);

impl ProcOp {
    /// Create a new [FuncOp].
    /// The underlying [Operation] is not linked to a [BasicBlock](crate::basic_block::BasicBlock).
    /// The returned function has a single region with an empty `entry` block.
    pub fn new_unlinked(ctx: &mut Context, name: &str) -> ProcOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let opop = ProcOp { op };
        // Create an empty entry block.
        #[allow(clippy::expect_used)]
        let region = opop.get_region(ctx);
        let body = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
        body.insert_at_front(region, ctx);
        opop.set_symbol_name(ctx, name);
        opop
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

impl OneRegionInterface for ProcOp {}
#[cast_to]
impl SymbolOpInterface for ProcOp {}

impl DisplayWithContext for ProcOp {
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

impl Verify for ProcOp {
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
    pub fn get_value(&self, ctx: &Context) -> FieldElemAttr {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let value = op
            .attributes
            .get(Self::ATTR_KEY_VALUE)
            .expect("no attribute found");
        #[allow(clippy::unwrap_used)]
        value.downcast_ref::<FieldElemAttr>().unwrap().clone()
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
    /// Pop two top stack items, sums them and push result on stack
    ///
    AddOp,
    "add",
    "miden"
);

impl AddOp {
    /// Create a new [AddOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context) -> ConstantOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        ConstantOp { op }
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
    /// Call miden exec on provided symbol.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// | [ATTR_KEY_SYM_NAME](super::ATTR_KEY_SYM_NAME) | [StringAttr](super::attributes::StringAttr) |
    ///
    ExecOp,
    "exec",
    "miden"
);

impl ExecOp {
    /// Attribute key for the callee symbol name.
    pub const ATTR_KEY_CALLEE_SYM: &str = "call.callee_sym";

    /// Get the callee symbol name.
    pub fn get_callee_sym(&self, ctx: &Context) -> String {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let callee_sym = op
            .attributes
            .get(Self::ATTR_KEY_CALLEE_SYM)
            .expect("no attribute found");
        callee_sym.with_ctx(ctx).to_string()
    }

    /// Create a new [CallOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, callee_name: FuncSym) -> ExecOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let callee_sym = StringAttr::create(callee_name.into());
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_CALLEE_SYM, callee_sym);
        ExecOp { op }
    }
}

impl DisplayWithContext for ExecOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_callee_sym(ctx)
        )
    }
}

impl Verify for ExecOp {
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

impl CallOpInterface for ExecOp {
    fn get_callee_sym(&self, ctx: &Context) -> String {
        self.get_callee_sym(ctx)
    }
}

declare_op!(
    /// Push local variable value with the given index onto the stack.
    ///
    /// Attributes:
    ///
    /// | key | value |
    /// |-----|-------|
    /// |[ATTR_KEY_INDEX](Self::ATTR_KEY_INDEX) | [IntegerAttr] |
    ///
    LocLoadOp,
    "loc.load",
    "miden"
);

impl LocLoadOp {
    /// Attribute key for the index
    pub const ATTR_KEY_INDEX: &str = "loc.load.index";

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

    /// Get the index of the local variable as u32.
    #[allow(clippy::unwrap_used)]
    pub fn get_index_as_u32(&self, ctx: &Context) -> u32 {
        let attr = self.get_index(ctx);
        #[allow(clippy::unwrap_used)]
        let apint: ApInt = attr.downcast_ref::<IntegerAttr>().unwrap().clone().into();
        apint.try_to_u32().unwrap()
    }

    /// Create a new [LocalGetOp].
    pub fn new_unlinked(ctx: &mut Context, index: AttrObj) -> LocLoadOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_INDEX, index);
        LocLoadOp { op }
    }
}

impl DisplayWithContext for LocLoadOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {}",
            self.get_opid().with_ctx(ctx),
            self.get_index(ctx).with_ctx(ctx)
        )
    }
}

impl Verify for LocLoadOp {
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
    ExecOp::register(ctx, dialect);
    LocLoadOp::register(ctx, dialect);
    ProgramOp::register(ctx, dialect);
    ProcOp::register(ctx, dialect);
}
