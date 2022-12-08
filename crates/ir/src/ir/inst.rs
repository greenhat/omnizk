use super::FuncIndex;

#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Nop,
    End,
    Return,
    I32Const { value: i32 },
    LocalGet { local_idx: u32 },
    I32Add,
    I64Add,
    Call { func_idx: FuncIndex },
    PubInputRead,
    PubOutputWrite,
    SecretInputRead,
}
