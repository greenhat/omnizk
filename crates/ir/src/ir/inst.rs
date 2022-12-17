use super::BlockType;
use super::FuncIndex;

#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Nop,
    Call { func_idx: FuncIndex },
    End,
    Return,
    Loop { block_type: BlockType },
    Block { blockty: BlockType },
    BrIf { relative_depth: u32 },
    Br { relative_depth: u32 },
    I32Const { value: i32 },
    I64Const { value: i64 },
    LocalGet { local_idx: u32 },
    LocalTee { local_idx: u32 },
    LocalSet { local_idx: u32 },
    I32Add,
    I32Eqz,
    I64Add,
    I64Eqz,
    I64And,
    I64GeU,
    I64Ne,
    PubInputRead,
    PubOutputWrite,
    SecretInputRead,
}
