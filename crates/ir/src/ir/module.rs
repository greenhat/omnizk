use super::Func;

pub struct Module {
    functions: Vec<Func>,
}

impl Module {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn push_func(&mut self, func: Func) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }
}
