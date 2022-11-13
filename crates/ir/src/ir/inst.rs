#[derive(Debug, Clone)]
pub enum Inst {
    Unreachable,
    Nop,
    End,
    Return,
    I32Const { value: i32 },
}
