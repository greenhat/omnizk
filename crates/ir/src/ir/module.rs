use super::Func;

pub struct Module {
    functions: Vec<Func>,
    // TODO: FuncIndex newtype?
    pub start_func_idx: u32,
}

impl Module {
    pub fn new(functions: Vec<Func>, start_func_idx: u32) -> Self {
        Self {
            functions,
            start_func_idx,
        }
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }
}
