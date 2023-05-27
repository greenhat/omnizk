use std::collections::HashMap;

use ozk_wasm_dialect::ops::FuncOp;
use ozk_wasm_dialect::ops::ModuleOp;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::op::Op;
use pliron::r#type::TypeObj;
use thiserror::Error;

// mod import_func_body;

// pub use import_func_body::ImportFuncBody;

use crate::func_builder::FuncBuilder;
use crate::func_builder::FuncBuilderError;
use crate::types::FuncIndex;
use crate::types::TypeIndex;

// use self::import_func_body::ImportFunc;

pub struct ModuleBuilder {
    types: Vec<Ptr<TypeObj>>,
    start_func_idx: Option<FuncIndex>,
    functions: Vec<FuncBuilder>,
    import_functions: Vec<FuncOp>,
    // import_func_body: ImportFuncBody,
    func_names: HashMap<FuncIndex, String>,
    func_types: HashMap<FuncIndex, TypeIndex>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
            // import_func_body: ImportFuncBody::new_stdlib(),
            func_names: HashMap::new(),
            func_types: HashMap::new(),
            import_functions: Vec::new(),
        }
    }

    pub fn push_type(&mut self, ty: Ptr<TypeObj>) {
        self.types.push(ty);
    }

    pub fn push_import_func(
        &mut self,
        _type_idx: u32,
        _module: &str,
        _name: &str,
    ) -> Result<(), ModuleBuilderError> {
        todo!();
        // // dbg!(&self.types);
        // let ty = self
        //     .types
        //     .get(type_idx as usize)
        //     .ok_or_else(|| ModuleBuilderError::InvalidTypeIndex(format!("type_idx: {}", type_idx)))?
        //     .clone();
        // // dbg!(name);
        // // dbg!(&ty);
        // let import_func = ImportFunc {
        //     module: module.to_string(),
        //     name: name.to_string(),
        //     ty,
        // };
        // let func = self
        //     .import_func_body
        //     .func(&import_func)
        //     .ok_or(ModuleBuilderError::ImportFuncBodyNotFound(import_func))?;
        // self.import_functions.push(func.clone());
        // Ok(())
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

    // pub fn build_func_call(&self, func_idx: u32) -> Result<Vec<Inst>, ModuleBuilderError> {
    //     Ok(vec![Inst::Call {
    //         func_idx: func_idx.into(),
    //     }])
    // }

    pub fn build(mut self, ctx: &mut Context) -> Result<ModuleOp, ModuleBuilderError> {
        let mut func_sigs: Vec<Ptr<TypeObj>> = Vec::new();
        for func_idx in 0..self.functions.len() {
            // TODO: and here we use "raw" func index without imported functions
            let func_type = self.get_func_type((func_idx as u32).into())?;
            func_sigs.push(func_type);
        }
        let imported_funcs_count = self.import_functions.len() as u32;

        if let Some(start_func_idx) = self.start_func_idx {
            let start_func_name = self
                .get_func_name(start_func_idx)
                .ok_or(ModuleBuilderError::FuncNameNotFound(start_func_idx))?;
            let module_op = ModuleOp::new(ctx, "module_name", start_func_name);
            let mut funcs = Vec::new();
            // first, imported functions
            for import_func in self.import_functions {
                funcs.push(import_func);
            }

            // TODO: since func indices should be shifted by imported funcs count change the storage and make it obvious
            for (func_idx, func_builder) in self.functions.iter_mut().enumerate() {
                if let Some(func_name) = self
                    .func_names
                    .get(&(func_idx as u32 + imported_funcs_count).into())
                {
                    func_builder.set_name(func_name.clone());
                }
                func_builder.set_signature(func_sigs[func_idx]);
            }
            for func_builder in self.functions {
                funcs.push(func_builder.build(ctx)?);
            }

            for func in funcs {
                module_op.add_operation(ctx, func.get_operation());
            }
            Ok(module_op)
        } else {
            Err(ModuleBuilderError::StartFuncUndefined)
        }
    }

    pub fn next_func_idx(&self) -> FuncIndex {
        (self.functions.len() as u32).into()
    }

    pub fn declare_func_name(&mut self, func_idx: FuncIndex, name: String) {
        // dbg!(&func_idx);
        // dbg!(&name);
        self.func_names.insert(func_idx, name);
    }

    pub fn get_func_name(&self, func_idx: FuncIndex) -> Option<String> {
        self.func_names.get(&func_idx).cloned()
    }

    pub fn get_func_type(&self, func_idx: FuncIndex) -> Result<Ptr<TypeObj>, ModuleBuilderError> {
        let type_idx = self
            .func_types
            .get(&func_idx)
            .ok_or_else(|| ModuleBuilderError::TypeIndexNotFound(u32::from(func_idx)))?;

        self.types
            .get(u32::from(*type_idx) as usize)
            .cloned()
            .ok_or_else(|| ModuleBuilderError::TypeNotFound(u32::from(*type_idx)))
    }

    // pub fn get_func_type_typed(
    //     &self,
    //     ctx: &'a mut Context,
    //     func_idx: FuncIndex,
    // ) -> Result<&FunctionType, ModuleBuilderError> {
    //     Ok(self
    //         .get_func_type(func_idx)?
    //         .deref(ctx)
    //         .downcast_ref::<FunctionType>()
    //         .unwrap())
    // }
}

#[derive(Error, Debug)]
pub enum ModuleBuilderError {
    #[error("start function is undefined")]
    StartFuncUndefined,
    // #[error("cannot find a body for import function `{0:?}`")]
    // ImportFuncBodyNotFound(ImportFunc),
    #[error("type index for func index `{0}` not found")]
    TypeIndexNotFound(u32),
    #[error("type with index {0} not found")]
    TypeNotFound(u32),
    #[error("func builder error: {0:?}")]
    FuncBuilderError(#[from] FuncBuilderError),
    #[error("invalid type index: {0}")]
    InvalidTypeIndex(String),
    #[error("func name not found for func index: {0:?}")]
    FuncNameNotFound(FuncIndex),
}
