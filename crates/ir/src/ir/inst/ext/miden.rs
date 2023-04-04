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
    End,
}

impl From<MidenExt> for Inst {
    fn from(ext: MidenExt) -> Self {
        Inst::Ext(Ext::Miden(ext))
    }
}
