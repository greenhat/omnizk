use derive_more::Add;
use derive_more::From;
use derive_more::Into;

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

impl Ty {
    pub fn size(&self) -> i32 {
        match self {
            Ty::I32 | Ty::F32 => 4,
            Ty::I64 | Ty::F64 => 8,
            Ty::V128 => 16,
            Ty::FuncRef | Ty::ExternRef => 4,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct FuncType {
    pub params: Vec<Ty>,
    pub results: Vec<Ty>,
}

impl FuncType {
    pub fn void_void() -> Self {
        Self {
            params: vec![],
            results: vec![],
        }
    }

    pub fn new(params: Vec<Ty>, results: Vec<Ty>) -> Self {
        Self { params, results }
    }
}

#[derive(Debug, Clone, Copy, From, Into, PartialEq, Eq, Hash, Ord, PartialOrd, Add)]
pub struct FuncIndex(u32);

#[derive(Debug, Clone, Copy, From, Into, PartialEq, Eq, Hash)]
pub struct TypeIndex(u32);

#[derive(Debug, Clone, Copy, From, Into, PartialEq, Eq, Hash)]
pub struct GlobalIndex(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    /// The block consumes nor produces any values.
    Empty,
    /// The block produces a singular value of the given type ([] -> \[t]).
    Type(Ty),
    /// The block is described by a function type.
    /// The index is to a function type in the types section.
    FuncType(TypeIndex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockKind {
    Block,
    Loop,
}

impl From<usize> for FuncIndex {
    fn from(idx: usize) -> Self {
        Self(idx as u32)
    }
}

impl From<FuncIndex> for usize {
    fn from(fi: FuncIndex) -> Self {
        fi.0 as usize
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum Field {
    /// a 64-bit prime field defined by modulus p = 2^64 - 2^32 + 1,
    /// all values that the VM operates with are field elements in this field (
    /// i.e., values between 0 and 2^64 −2^32 , both inclusive).
    Oxfoi,
}
