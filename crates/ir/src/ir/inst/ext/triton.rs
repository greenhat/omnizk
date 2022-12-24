use crate::ir::Inst;

use super::Ext;

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
}

impl From<TritonExt> for Inst {
    fn from(ext: TritonExt) -> Self {
        Inst::Ext(Ext::Triton(ext))
    }
}
