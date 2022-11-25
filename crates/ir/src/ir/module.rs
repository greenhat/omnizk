use super::Func;

pub struct Module {
    functions: Vec<Func>,
    pub start_func_idx: u32,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            start_func_idx: 0,
        }
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }
}
