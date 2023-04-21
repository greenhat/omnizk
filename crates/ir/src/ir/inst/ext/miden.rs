use crate::ir::Inst;

use super::Ext;

/// MidenVM instructions
/// see https://0xpolygonmiden.github.io/miden-vm/user_docs/assembly/main.html
#[derive(Debug, Clone)]
pub enum MidenExt {
    /// Pushes the current depth of the stack onto the stack.
    SDepth,
    /// https://0xpolygonmiden.github.io/miden-vm/user_docs/assembly/flow_control.html#condition-controlled-loops
    While,
    /// NeqImm compares the top of the stack with an immediate value and pushes 1 onto the stack if
    /// not equal.
    NeqImm(i32),
    /// Neq compares the top of the stack with the second element on the stack and pushes 1 onto the
    /// stack if not equal.
    Neq,
}

impl From<MidenExt> for Inst {
    fn from(ext: MidenExt) -> Self {
        Inst::Ext(Ext::Miden(ext))
    }
}
