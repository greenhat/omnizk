use intertrait::cast_to;
use pliron::basic_block::BasicBlock;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::op_interfaces::OneRegionInterface;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::error::CompilerError;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

use crate::op_interfaces::HasOperands;
use crate::types::Operands;

declare_op!(
    /// Write the immediate values b,c,d,e to the cell located at offset a.
    Imm32Op,
    "imm32",
    "valida"
);

impl Imm32Op {
    /// Create a new [Imm32Op]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, operands: Operands) -> Imm32Op {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let op_op = Imm32Op { op };
        op_op.set_operands(ctx, operands);
        op_op
    }
}

#[intertrait::cast_to]
impl HasOperands for Imm32Op {}

impl DisplayWithContext for Imm32Op {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let operands = self.get_operands(ctx);
        write!(
            f,
            "{} {}(fp) {} {} {} {}",
            self.get_opid().with_ctx(ctx),
            operands.a(),
            operands.b(),
            operands.c(),
            operands.d(),
            operands.e(),
        )
    }
}

impl Verify for Imm32Op {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

declare_op!(
    /// Represents a Valida program
    ProgramOp,
    "program",
    "valida"
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
    /// Create a new [ProgramOP].
    /// The returned programm has a single [crate::region::Region] with a single (BasicBlock)[crate::basic_block::BasicBlock].
    pub fn new(ctx: &mut Context, funcs: Vec<Ptr<Operation>>) -> ProgramOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let opop = ProgramOp { op };
        // Create an empty block.
        let region = opop.get_region(ctx);
        let block = BasicBlock::new(ctx, None, vec![]);
        for op in funcs {
            op.insert_at_back(block, ctx);
        }
        block.insert_at_front(region, ctx);
        opop
    }

    /// Add an [Operation] into this module.
    pub fn add_operation(&self, ctx: &mut Context, op: Ptr<Operation>) {
        self.append_operation(ctx, op, 0)
    }
}

impl OneRegionInterface for ProgramOp {}
impl SingleBlockRegionInterface for ProgramOp {}

declare_op!(
    /// An operation representing a function in Valida
    FuncOp,
    "func",
    "valida"
);

impl FuncOp {
    /// Create a new [FuncOp].
    /// The underlying [Operation] is not linked to a [BasicBlock](crate::basic_block::BasicBlock).
    /// The returned function has a single region with an empty `entry` block.
    pub fn new_unlinked(ctx: &mut Context, name: String) -> FuncOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let opop = FuncOp { op };
        // Create an empty entry block.
        #[allow(clippy::expect_used)]
        let region = opop.get_region(ctx);
        let body = BasicBlock::new(ctx, Some("entry".to_string()), vec![]);
        body.insert_at_front(region, ctx);
        opop.set_symbol_name(ctx, &name);
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

impl OneRegionInterface for FuncOp {}
#[cast_to]
impl SymbolOpInterface for FuncOp {}

impl DisplayWithContext for FuncOp {
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

impl Verify for FuncOp {
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
    /// add two values
    /// Compute the unchecked addition of the U32 values at cell offsets b and c and write the sum to cell offset a .
    /// Note that because a full 32-bit value does not fit within one field element,
    /// we assume that values have been decomposed into 4 8-byte elements. The summed output is stored at cell offset a.
    AddOp,
    "add",
    "valida"
);

impl AddOp {
    /// Create a new [AddOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, operands: Operands) -> AddOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let op_op = AddOp { op };
        op_op.set_operands(ctx, operands);
        op_op
    }
}

impl DisplayWithContext for AddOp {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let operands = self.get_operands(ctx);
        write!(
            f,
            "{} {}(fp) {}(fp) {}(fp) {} {}",
            self.get_opid().with_ctx(ctx),
            operands.a(),
            operands.b(),
            operands.c(),
            operands.d(),
            operands.e(),
        )
    }
}

impl Verify for AddOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

#[intertrait::cast_to]
impl HasOperands for AddOp {}

declare_op!(
    /// jump to variable and link
    /// Store the pc + 1 to local stack variable at offset "a" then set pc to field element "b".
    /// Set fp to fp + c.
    JalvOp,
    "jalv",
    "valida"
);

impl JalvOp {
    /// Create a new [JalvOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_return_pseudo_op(ctx: &mut Context) -> JalvOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let jalv_op = JalvOp { op };
        let operands = Operands::from_i32(
            -4, // pc + 1
            0,  // pc
            8,  // fp + 8
            0, 0,
        );
        jalv_op.set_operands(ctx, operands);
        jalv_op
    }
}

impl DisplayWithContext for JalvOp {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let operands = self.get_operands(ctx);
        write!(
            f,
            "{} {}(fp) {}(fp) {}(fp) {} {}",
            self.get_opid().with_ctx(ctx),
            operands.a(),
            operands.b(),
            operands.c(),
            operands.d(),
            operands.e()
        )
    }
}

impl Verify for JalvOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

#[intertrait::cast_to]
impl HasOperands for JalvOp {}

declare_op!(
    /// Write the 4 byte values beginning at the address stroed at offset c to those beginning at offset b.
    /// Operand a is unused, but is constrained to [c] in the trace.
    SwOp,
    "sw",
    "valida"
);

impl SwOp {
    /// Create a new [SwOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context, operands: Operands) -> SwOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let op_op = SwOp { op };
        op_op.set_operands(ctx, operands);
        op_op
    }
}

impl DisplayWithContext for SwOp {
    #[allow(clippy::expect_used)]
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let operands = self.get_operands(ctx);
        write!(
            f,
            "{} {} {}(fp) {}(fp) {} {}",
            self.get_opid().with_ctx(ctx),
            operands.a(),
            operands.b(),
            operands.c(),
            operands.d(),
            operands.e()
        )
    }
}

impl Verify for SwOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

#[intertrait::cast_to]
impl HasOperands for SwOp {}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    Imm32Op::register(ctx, dialect);
    ProgramOp::register(ctx, dialect);
    FuncOp::register(ctx, dialect);
    AddOp::register(ctx, dialect);
    JalvOp::register(ctx, dialect);
    SwOp::register(ctx, dialect);
}
