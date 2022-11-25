#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

pub struct FuncType {
    pub params: Vec<Type>,
    pub results: Vec<Type>,
}
