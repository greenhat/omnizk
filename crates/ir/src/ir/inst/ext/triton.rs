use crate::ir::Inst;

use super::Ext;

/// TritonVM instructions
/// see https://triton-vm.org/spec/instructions.html
#[derive(Debug, Clone)]
pub enum TritonExt {
    // TODO: remove and use Inst::Drop instead
    /// drops the top of the stack
    Pop,
    /// skip the next op if top of stack is 0
    Skiz,
    // jumps to the beginning of the current function
    Recurse,
    /// Bit-shifts a to the right by 1 bit and pushes the least significant bit of a to the stack.
    Lsb,
    /// Pops a if a == 1, else crashes the virtual machine.
    Assert,
}

impl From<TritonExt> for Inst {
    fn from(ext: TritonExt) -> Self {
        Inst::Ext(Ext::Triton(ext))
    }
}
