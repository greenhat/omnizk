use intertrait::cast_to;
use pliron::basic_block::BasicBlock;
use pliron::common_traits::DisplayWithContext;
use pliron::common_traits::Verify;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::declare_op;
use pliron::dialect::Dialect;
use pliron::dialects::builtin::attributes::StringAttr;
use pliron::dialects::builtin::op_interfaces::OneRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::error::CompilerError;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;
use pliron::with_context::AttachContext;

use crate::op_interfaces::HasOperands;
use crate::op_interfaces::TrackedProgramCounter;
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
    /// The returned programm has a single [crate::region::Region] with an entry block (BasicBlock)[crate::basic_block::BasicBlock] and block with function definitions.
    pub fn new(
        ctx: &mut Context,
        entry_block: Ptr<BasicBlock>,
        funcs: Vec<Ptr<Operation>>,
    ) -> ProgramOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 1);
        let opop = ProgramOp { op };
        // Create an empty block.
        let region = opop.get_region(ctx);
        entry_block.insert_at_front(region, ctx);
        let funcs_block = BasicBlock::new(ctx, None, vec![]);
        for op in funcs {
            op.insert_at_back(funcs_block, ctx);
        }
        funcs_block.insert_at_back(region, ctx);
        opop
    }

    /// Returns the entry block of this program.
    pub fn get_entry_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        #[allow(clippy::unwrap_used)]
        self.get_region(ctx).deref(ctx).get_head().unwrap()
    }

    /// Returns the block with function definitions.
    pub fn get_funcs_block(&self, ctx: &Context) -> Ptr<BasicBlock> {
        #[allow(clippy::unwrap_used)]
        self.get_region(ctx).deref(ctx).get_tail().unwrap()
    }

    pub fn get_func(&self, ctx: &Context, sym: &str) -> Option<FuncOp> {
        for func_op in self.get_funcs_block(ctx).deref(ctx).iter(ctx) {
            let deref_func_op = func_op.deref(ctx).get_op(ctx);
            if let Some(func_op) = deref_func_op.downcast_ref::<FuncOp>() {
                if func_op.get_symbol_name(ctx) == sym {
                    return Some(*func_op);
                }
            }
        }
        None
    }
}

impl OneRegionInterface for ProgramOp {}

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
        let pc_str = if let Some(pc) = self.get_pc_opt(ctx) {
            format!(" pc={}", pc)
        } else {
            "".to_string()
        };
        write!(
            f,
            "{} @{}{} {{\n{}}}",
            self.get_opid().with_ctx(ctx),
            self.get_symbol_name(ctx),
            pc_str,
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
    /// add two values
    /// Compute the unchecked addition of the U32 values at cell offsets b and c
    /// and write the sum to cell offset a.
    /// Note that because a full 32-bit value does not fit within one field element,
    /// we assume that values have been decomposed into 4 8-byte elements. The summed output is stored at cell offset a.
    pub fn new(ctx: &mut Context, result_fp: i32, arg1_fp: i32, arg2_fp: i32) -> AddOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let op_op = AddOp { op };
        let operands = Operands::from_i32(result_fp, arg1_fp, arg2_fp, 0, 0);
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
            // 8,  // fp + 8
            4, // fp + 4
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
    /// Write the 4 byte values beginning at the address stroed at offset c
    /// to those beginning at offset b.
    pub fn new(ctx: &mut Context, to_fp: i32, from_fp: i32) -> SwOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let op_op = SwOp { op };
        let operands = Operands::from_i32(0, to_fp, from_fp, 0, 0);
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

declare_op!(
    /// jump to address and link
    /// Store the pc + 1 to local stack variable at offset "a" then set pc to field element "b".
    /// Set fp to fp + c.
    JalOp,
    "jal",
    "valida"
);

impl JalOp {
    /// Create a new [JalOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn from_operands(ctx: &mut Context, operands: Operands) -> JalOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let jalv_op = JalOp { op };
        jalv_op.set_operands(ctx, operands);
        jalv_op
    }

    /// Jump to address and link
    /// Store the pc + 1 to local stack variable at offset "a" then set pc to field element "b".
    /// Set fp to fp + c.
    pub fn new_from_i32(
        ctx: &mut Context,
        fp_offset_for_pc_plus_1: i32,
        new_pc: i32,
        new_fp_offset: i32,
    ) -> JalOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        let jalv_op = JalOp { op };
        let operands = Operands::from_i32(fp_offset_for_pc_plus_1, new_pc, new_fp_offset, 0, 0);
        jalv_op.set_operands(ctx, operands);
        jalv_op
    }
}

impl DisplayWithContext for JalOp {
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
            operands.e()
        )
    }
}

impl Verify for JalOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

#[intertrait::cast_to]
impl HasOperands for JalOp {}

// TODO: move to ozk dialect?
declare_op!(
    /// jump to address and link (symbolic name version)
    /// Store the pc + 1 to local stack variable at offset "a"
    /// then set pc to the first instruction of a function/block with given symbolic name.
    /// Set fp to fp + c.
    JalSymOp,
    "jalsym",
    "valida"
);

impl JalSymOp {
    const ATTR_KEY_TARGET_SYM: &str = "jalsym.target_sym";

    /// jump to address and link (symbolic name version)
    /// Store the pc + 1 to local stack variable at offset "a"
    /// then set pc to the first instruction of a function/block with given symbolic name.
    /// Set fp to fp + c.
    pub fn new(
        ctx: &mut Context,
        fp_offset_for_pc_plus_1: i32,
        new_fp_offset: i32,
        target_sym: String,
    ) -> JalSymOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        op.deref_mut(ctx)
            .attributes
            .insert(Self::ATTR_KEY_TARGET_SYM, StringAttr::create(target_sym));
        let jalv_op = JalSymOp { op };
        let operands = Operands::from_i32(fp_offset_for_pc_plus_1, 0, new_fp_offset, 0, 0);
        jalv_op.set_operands(ctx, operands);
        jalv_op
    }

    /// Get the target symbol
    pub fn get_target_sym(&self, ctx: &Context) -> String {
        let op = self.get_operation().deref(ctx);
        #[allow(clippy::expect_used)]
        let func_sym_attr = op
            .attributes
            .get(Self::ATTR_KEY_TARGET_SYM)
            .expect("no attribute found");
        #[allow(clippy::expect_used)]
        let func_sym: String = func_sym_attr
            .downcast_ref::<StringAttr>()
            .expect("expected StringAttr")
            .clone()
            .into();
        func_sym
    }
}

impl DisplayWithContext for JalSymOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let operands = self.get_operands(ctx);
        write!(
            f,
            "{} {}(fp) {} {} {} {}",
            self.get_opid().with_ctx(ctx),
            operands.a(),
            self.get_target_sym(ctx),
            operands.c(),
            operands.d(),
            operands.e()
        )
    }
}

impl Verify for JalSymOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        todo!()
    }
}

#[intertrait::cast_to]
impl HasOperands for JalSymOp {}

declare_op!(
    /// Exit the program (halts execution)
    ExitOp,
    "exit",
    "valida"
);

impl ExitOp {
    /// Create a new [ExitOp]. The underlying [Operation] is not linked to a
    /// [BasicBlock](crate::basic_block::BasicBlock).
    pub fn new_unlinked(ctx: &mut Context) -> ExitOp {
        let op = Operation::new(ctx, Self::get_opid_static(), vec![], vec![], 0);
        ExitOp { op }
    }
}

impl DisplayWithContext for ExitOp {
    fn fmt(&self, ctx: &Context, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get_opid().with_ctx(ctx),)
    }
}

impl Verify for ExitOp {
    fn verify(&self, _ctx: &Context) -> Result<(), CompilerError> {
        Ok(())
    }
}

pub(crate) fn register(ctx: &mut Context, dialect: &mut Dialect) {
    Imm32Op::register(ctx, dialect);
    ProgramOp::register(ctx, dialect);
    FuncOp::register(ctx, dialect);
    AddOp::register(ctx, dialect);
    JalvOp::register(ctx, dialect);
    SwOp::register(ctx, dialect);
    JalOp::register(ctx, dialect);
    JalSymOp::register(ctx, dialect);
    ExitOp::register(ctx, dialect);
}
