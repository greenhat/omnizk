use super::Func;

pub struct Module {
    functions: Vec<Func>,
}

impl Module {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }
}
