use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use std::collections::HashMap;
use thiserror::Error;

mod import_func_resolver;

pub use import_func_resolver::ImportFuncResolver;

use self::import_func_resolver::ImportFunc;

pub struct ModuleBuilder {
    types: Vec<FuncType>,
    start_func_idx: Option<FuncIndex>,
    functions: Vec<Func>,
    imports_func_resolver: ImportFuncResolver,
    import_funcs: HashMap<FuncIndex, ImportFunc>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
            imports_func_resolver: ImportFuncResolver::new_stdlib(),
            import_funcs: HashMap::new(),
        }
    }

    pub fn push_type(&mut self, ty: FuncType) {
        self.types.push(ty);
    }

    pub fn push_import_func(&mut self, func_idx: u32, _module: &str, name: &str) {
        let callable_func_idx = (self.types.len() as u32 + func_idx).into();
        self.import_funcs.insert(
            callable_func_idx,
            ImportFunc {
                module: _module.to_string(),
                name: name.to_string(),
            },
        );
    }

    pub fn set_start_func(&mut self, func_idx: u32) {
        self.start_func_idx = Some(func_idx.into());
    }

    pub fn push_func(&mut self, func: Func) {
        self.functions.push(func);
    }

    pub fn build_func_call(&self, func_idx: u32) -> Result<Vec<Inst>, ModuleBuilderError> {
        let func_idx: FuncIndex = func_idx.into();
        if let Some(import_func) = self.import_funcs.get(&func_idx) {
            if let Some(insts) = self.imports_func_resolver.resolve(&import_func.name) {
                Ok(insts.clone())
            } else {
                Err(ModuleBuilderError::ImportFuncNotFound(
                    import_func.name.clone(),
                ))
            }
        } else {
            Ok(vec![Inst::Call { func_idx }])
        }
    }

    pub fn build(self) -> Result<Module, ModuleBuilderError> {
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
    #[error("import function `{0}` is not found")]
    ImportFuncNotFound(String),
}
