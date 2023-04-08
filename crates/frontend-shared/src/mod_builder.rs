use std::collections::HashMap;

use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::TypeIndex;
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
    functions: Vec<FuncBuilder>,
    import_functions: Vec<FuncBuilder>,
    import_func_body: ImportFuncBody,
    func_names: HashMap<FuncIndex, String>,
    func_types: HashMap<FuncIndex, TypeIndex>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
            import_func_body: ImportFuncBody::new_stdlib(),
            func_names: HashMap::new(),
            func_types: HashMap::new(),
            import_functions: Vec::new(),
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
        // dbg!(&self.types);
        let ty = self
            .types
            .get(type_idx as usize)
            .ok_or_else(|| {
                ModuleBuilderError::InvalidTypeIndex(format!(
                    "type_idx: {}, types: {:?}",
                    type_idx, self.types
                ))
            })?
            .clone();
        // dbg!(name);
        // dbg!(&ty);
        let import_func = ImportFunc {
            module: module.to_string(),
            name: name.to_string(),
            ty,
        };
        let mut func_builder = FuncBuilder::new(name.to_string());
        func_builder.set_signature(import_func.ty.clone());
        let func_body = self
            .import_func_body
            .body(&import_func)
            .ok_or(ModuleBuilderError::ImportFuncBodyNotFound(import_func))?;
        func_builder.push_insts(func_body.clone());
        self.import_functions.push(func_builder);
        Ok(())
    }

    pub fn push_func_type(&mut self, func_idx: u32, type_idx: u32) {
        let func_idx = func_idx.into();
        let type_idx = type_idx.into();
        self.func_types.insert(func_idx, type_idx);
    }

    pub fn set_start_func(&mut self, func_idx: u32) {
        self.start_func_idx = Some(func_idx.into());
    }

    pub fn push_func_builder(&mut self, func_builder: FuncBuilder) {
        self.functions.push(func_builder);
    }

    pub fn build_func_call(&self, func_idx: u32) -> Result<Vec<Inst>, ModuleBuilderError> {
        Ok(vec![Inst::Call {
            func_idx: func_idx.into(),
        }])
    }

    pub fn build(mut self) -> Result<Module, ModuleBuilderError> {
        let mut func_sigs: Vec<FuncType> = Vec::new();
        for func_idx in 0..self.functions.len() {
            // TODO: and here we use "raw" func index without imported functions
            let func_type = self.get_func_type(func_idx.into())?;
            func_sigs.push(func_type.clone());
        }
        let imported_funcs_count = self.import_functions.len() as u32;

        let mut funcs = Vec::new();
        // first, imported functions
        for import_func in self.import_functions {
            funcs.push(import_func.build()?);
        }

        // TODO: since func indices should be shifted by imported funcs count change the storage and make it obvious
        for (func_idx, func_builder) in self.functions.iter_mut().enumerate() {
            if let Some(func_name) = self
                .func_names
                .get(&(func_idx as u32 + imported_funcs_count).into())
            {
                func_builder.set_name(func_name.clone());
            }
            func_builder.set_signature(func_sigs[func_idx].clone());
        }
        for func_builder in self.functions {
            funcs.push(func_builder.build()?);
        }

        // dbg!(&funcs);
        if let Some(start_func_idx) = self.start_func_idx {
            Ok(Module::new(funcs, start_func_idx, Vec::new()))
        } else {
            Err(ModuleBuilderError::StartFuncUndefined)
        }
    }

    pub fn next_func_idx(&self) -> FuncIndex {
        self.functions.len().into()
    }

    pub fn declare_func_name(&mut self, func_idx: FuncIndex, name: String) {
        // dbg!(&func_idx);
        // dbg!(&name);
        self.func_names.insert(func_idx, name);
    }

    pub fn get_func_name(&self, func_idx: FuncIndex) -> Option<String> {
        self.func_names.get(&func_idx).cloned()
    }

    pub fn get_func_type(&self, func_idx: FuncIndex) -> Result<&FuncType, ModuleBuilderError> {
        let type_idx = self
            .func_types
            .get(&func_idx)
            .ok_or_else(|| ModuleBuilderError::TypeIndexNotFound(usize::from(func_idx) as u32))?;

        self.types
            .get(u32::from(*type_idx) as usize)
            .ok_or_else(|| ModuleBuilderError::TypeNotFound(u32::from(*type_idx)))
    }
}

impl Default for ModuleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ModuleBuilderError {
    #[error("start function is undefined")]
    StartFuncUndefined,
    #[error("cannot find a body for import function `{0:?}`")]
    ImportFuncBodyNotFound(ImportFunc),
    #[error("type index for func index `{0}` not found")]
    TypeIndexNotFound(u32),
    #[error("type with index {0} not found")]
    TypeNotFound(u32),
    #[error("func builder error: {0:?}")]
    FuncBuilderError(#[from] FuncBuilderError),
    #[error("invalid type index: {0}")]
    InvalidTypeIndex(String),
}
