use derive_more::From;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    I32,
    I64,
    F32,
    F64,
    V128,
    /// The value type is a function reference.
    FuncRef,
    /// The value type is an extern reference.
    ExternRef,
}

pub struct FuncType {
    pub params: Vec<Ty>,
    pub results: Vec<Ty>,
}

#[derive(Debug, Clone, Copy, From, PartialEq, Eq, Hash)]
pub struct FuncIndex(pub u32);

impl FuncIndex {
    pub fn as_u32(self) -> u32 {
        self.0
    }
}
