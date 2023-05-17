use intertrait::cast_to;
use pliron::attribute::attr_cast;
use pliron::basic_block::BasicBlock;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialects::builtin::attr_interfaces::TypedAttrInterface;
use pliron::dialects::builtin::attributes::TypeAttr;
use pliron::dialects::builtin::op_interfaces::OneRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::dialects::builtin::types::FunctionType;
use pliron::error::CompilerError;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::r#type::TypeObj;
use pliron::with_context::AttachContext;

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
    "wasm"
);

impl FuncOp {
    /// Attribute key for the constant value.
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

impl AttachContext for FuncOp {}
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
        Ok(())
    }
}
