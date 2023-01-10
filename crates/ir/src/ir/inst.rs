use self::ext::Ext;

use super::BlockType;
use super::FuncIndex;

pub mod ext;

#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Nop,
    Call { func_idx: FuncIndex },
    End,
    Return,
    Loop { block_type: BlockType },
    Block { blockty: BlockType },
    BrIf { relative_depth: u32 }, // branch out of the current block if the top of the stack is not zero
    Br { relative_depth: u32 },
    I32Const { value: i32 },
    I64Const { value: i64 },
    GlobalGet { global_idx: u32 },
    GlobalSet { global_idx: u32 },
    LocalGet { local_idx: u32 },
    LocalTee { local_idx: u32 },
    LocalSet { local_idx: u32 },
    I32Load { offset: u32 },
    I32Store { offset: u32 },
    I32Add,
    I32Sub,
    I32Mul,
    I32Eqz,
    I64Add,
    I64Mul,
    I64Eqz,
    I64And,
    I64GeU,
    I64Ne,
    PubInputRead,
    PubOutputWrite,
    SecretInputRead,
    Ext(Ext),
}
