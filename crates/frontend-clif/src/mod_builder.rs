use ozk_ozk_dialect::types::FuncSym;
use ozk_wasm_dialect::ops::ModuleOp;
use pliron::context::Context;
use pliron::error::CompilerError;
use std::collections::HashMap;

use ozk_wasm_dialect::types::FuncIndex;
use ozk_wasm_dialect::types::TypeIndex;
use pliron::context::Ptr;
use pliron::r#type::TypeObj;
use thiserror::Error;

use crate::func_builder::FuncBuilder;
use crate::func_builder::FuncBuilderError;

#[derive(Debug, Clone)]
pub struct ImportFuncLabel {
    pub module: String,
    pub name: String,
}

pub struct ModuleBuilder {
    types: Vec<Ptr<TypeObj>>,
    start_func_idx: Option<FuncIndex>,
    functions: Vec<FuncBuilder>,
    import_functions: Vec<(ImportFuncLabel, TypeIndex)>,
    func_names: HashMap<FuncIndex, FuncSym>,
    func_types: HashMap<FuncIndex, TypeIndex>,
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            start_func_idx: None,
            functions: Vec::new(),
            func_names: HashMap::new(),
            func_types: HashMap::new(),
            import_functions: Vec::new(),
        }
    }

    pub fn push_func_builder(&mut self, func_builder: FuncBuilder) {
        self.functions.push(func_builder);
    }

    pub fn build(self, _ctx: &mut Context) -> Result<ModuleOp, ModuleBuilderError> {
        todo!()
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
    #[error("compiler error: {0:?}")]
    CompilerError(#[from] CompilerError),
}
