use std::collections::HashMap;

use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use thiserror::Error;

mod import_func_body;

pub use import_func_body::ImportFuncBody;

use crate::FuncBuilder;
use crate::FuncBuilderError;

use self::import_func_body::ImportFunc;

#[derive(Debug)]
pub struct ModuleBuilder {
    types: Vec<FuncType>,
    start_func_idx: Option<FuncIndex>,
    functions: Vec<Func>,
    import_func_body: ImportFuncBody,
    func_names: HashMap<FuncIndex, String>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
            import_func_body: ImportFuncBody::new_stdlib(),
            func_names: HashMap::new(),
        }
    }

    pub fn push_type(&mut self, ty: FuncType) {
        self.types.push(ty);
    }

    pub fn push_import_func(
        &mut self,
        type_idx: u32,
        module: &str,
        name: &str,
    ) -> Result<(), ModuleBuilderError> {
        let import_func = ImportFunc {
            module: module.to_string(),
            name: name.to_string(),
            ty: self
                .types
                .get(type_idx as usize)
                .ok_or(ModuleBuilderError::TypeIndexNotFound(type_idx))?
                .clone(),
        };
        let func_body = self
            .import_func_body
            .body(&import_func)
            .ok_or(ModuleBuilderError::ImportFuncBodyNotFound(import_func))?;
        let mut func_builder = FuncBuilder::new(name.to_string());
        func_builder.push_insts(func_body.clone());
        self.functions.push(func_builder.build()?);
        Ok(())
    }

    pub fn set_start_func(&mut self, func_idx: u32) {
        self.start_func_idx = Some(func_idx.into());
    }

    pub fn push_func(&mut self, func: Func) {
        self.functions.push(func);
    }

    pub fn build_func_call(&self, func_idx: u32) -> Result<Vec<Inst>, ModuleBuilderError> {
        Ok(vec![Inst::Call {
            func_idx: func_idx.into(),
        }])
    }

    pub fn build(mut self) -> Result<Module, ModuleBuilderError> {
        for (func_idx, func) in self.functions.iter_mut().enumerate() {
            if let Some(func_name) = self.func_names.get(&(func_idx as u32).into()) {
                func.set_name(func_name.clone());
            }
        }
        if let Some(start_func_idx) = self.start_func_idx {
            Ok(Module::new(self.functions, start_func_idx))
        } else {
            Err(ModuleBuilderError::StartFuncUndefined)
        }
    }

    pub fn next_func_idx(&self) -> FuncIndex {
        self.functions.len().into()
    }

    pub fn declare_func_name(&mut self, func_idx: FuncIndex, name: String) {
        self.func_names.insert(func_idx, name);
    }

    pub fn get_func_name(&self, func_idx: FuncIndex) -> Option<String> {
        self.func_names.get(&func_idx).cloned()
    }

    pub fn get_func_type(&self, func_idx: FuncIndex) -> Option<&FuncType> {
        self.types.get(usize::from(func_idx))
    }
}

#[derive(Error, Debug)]
pub enum ModuleBuilderError {
    #[error("start function is undefined")]
    StartFuncUndefined,
    #[error("cannot find a body for import function `{0:?}`")]
    ImportFuncBodyNotFound(ImportFunc),
    #[error("type index `{0}` not found")]
    TypeIndexNotFound(u32),
    #[error("func builder error: {0:?}")]
    FuncBuilderError(#[from] FuncBuilderError),
}
