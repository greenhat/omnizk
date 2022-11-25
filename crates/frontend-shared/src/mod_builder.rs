use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Module;
use thiserror::Error;

pub struct ModuleBuilder {
    types: Vec<FuncType>,
    // imports: Vec<Import>,
    start_func_idx: Option<u32>,
    functions: Vec<Func>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
        }
    }

    pub fn push_type(&mut self, ty: FuncType) {
        self.types.push(ty);
    }

    pub fn set_start_func(&mut self, func_idx: u32) {
        self.start_func_idx = Some(func_idx);
    }

    pub fn push_func(&mut self, func: Func) {
        self.functions.push(func);
    }

    pub fn build(self) -> Result<Module, ModuleBuilderError> {
        // TODO: throw error if start section is not present
        if let Some(start_func_idx) = self.start_func_idx {
            Ok(Module::new(self.functions, start_func_idx))
        } else {
            Err(ModuleBuilderError::StartFuncUndefined)
        }
    }
}

#[derive(Error, Debug)]
pub enum ModuleBuilderError {
    #[error("start function is undefined")]
    StartFuncUndefined,
}
