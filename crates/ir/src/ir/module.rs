use super::Func;

pub struct Module {
    functions: Vec<Func>,
    start_func_idx: Option<u32>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            start_func_idx: None,
        }
    }

    pub fn set_start_func(&mut self, func_idx: u32) {
        self.start_func_idx = Some(func_idx);
    }

    pub fn push_func(&mut self, func: Func) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }
}
