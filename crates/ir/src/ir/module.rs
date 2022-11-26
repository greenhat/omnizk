use super::Func;
use super::FuncIndex;

pub struct Module {
    functions: Vec<Func>,
    pub start_func_idx: FuncIndex,
}

impl Module {
    pub fn new(functions: Vec<Func>, start_func_idx: FuncIndex) -> Self {
        Self {
            functions,
            start_func_idx,
        }
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }
}
