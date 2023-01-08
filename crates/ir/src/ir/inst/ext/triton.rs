use crate::ir::Inst;

use super::Ext;

/// TritonVM instructions
/// see https://triton-vm.org/spec/instructions.html
#[derive(Debug, Clone)]
pub enum TritonExt {
    Pop,
    /// skip the next op if top of stack is 0
    Skiz,
    /// 0..=15, swap the top of stack with the idx-th element from the top of stack
    Swap {
        idx: u8,
    },
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
