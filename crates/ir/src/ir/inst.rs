use self::ext::Ext;

use super::BlockType;
use super::FuncIndex;
use super::GlobalIndex;

pub mod ext;

#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Nop,
    Call {
        func_idx: FuncIndex,
    },
    End,
    Return,
    Loop {
        block_type: BlockType,
    },
    Block {
        blockty: BlockType,
    },
    BrIf {
        relative_depth: u32,
    }, // branch out of the current block if the top of the stack is not zero
    Br {
        relative_depth: u32,
    },
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    GlobalGet {
        global_idx: GlobalIndex,
    },
    GlobalSet {
        global_idx: GlobalIndex,
    },
    LocalGet {
        local_idx: u32,
    },
    LocalTee {
        local_idx: u32,
    },
    LocalSet {
        local_idx: u32,
    },
    I32Load {
        offset: u32,
    },
    /// The stack is expected to be [value, addr]
    I32Store {
        offset: u32,
    },
    I32Add,
    I32Sub,
    I32Mul,
    I32Eqz,
    I32WrapI64,
    I32And,
    I32GeU,
    I64Add,
    I64Mul,
    I64Eqz,
    I64And,
    I64GeU,
    I64Ne,
    I64Eq,
    I64ExtendI32U,
    PubInputRead,
    PubOutputWrite,
    SecretInputRead,
    // Extra (besides the wasm instructions)
    // -------------------------------------
    /// 0..=15, swap the top of stack with the idx-th element from the top of stack
    Swap {
        idx: u8,
    },
    /// 0..=15, copy the idx-th element to the top of the stack
    Dup {
        idx: u8,
    },
    // Extention instructions for target arch
    Ext(Ext),
}
