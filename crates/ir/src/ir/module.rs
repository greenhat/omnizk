use std::collections::HashMap;

use indexmap::IndexMap;

use super::Func;
use super::FuncIndex;
use super::GlobalIndex;
use super::Inst;
use super::Ty;

#[derive(Debug)]
pub struct Module {
    functions: IndexMap<FuncIndex, Func>,
    pub start_func_idx: FuncIndex,
    globals: Vec<Ty>,
}

impl Module {
    pub fn new(functions: Vec<Func>, start_func_idx: FuncIndex, globals: Vec<Ty>) -> Self {
        // TODO: check for start_func_idx is present in functions
        Self {
            functions: functions
                .into_iter()
                .enumerate()
                .map(|(idx, f)| (FuncIndex::from(idx), f))
                .collect(),
            start_func_idx,
            globals,
        }
    }

    pub fn start_function_mut(&mut self) -> &mut Func {
        // we checked that the start_func_idx is present in functions in constructor
        #[allow(clippy::unwrap_used)]
        self.functions.get_mut(&self.start_func_idx).unwrap()
    }

    pub fn functions_iter(&self) -> impl Iterator<Item = (&FuncIndex, &Func)> {
        self.functions.iter()
    }

    pub fn functions_into_iter(self) -> impl Iterator<Item = (FuncIndex, Func)> {
        self.functions.into_iter()
    }

    pub fn functions_iter_mut(&mut self) -> impl Iterator<Item = (&FuncIndex, &mut Func)> {
        self.functions.iter_mut()
    }

    pub fn function(&self, idx: &FuncIndex) -> Option<&Func> {
        self.functions.get(idx)
    }

    pub fn function_by_name(&self, name: &str) -> Option<&Func> {
        self.functions.values().find(|f| f.name() == name)
    }

    pub fn function_idx_by_name(&self, name: &str) -> Option<FuncIndex> {
        self.functions
            .iter()
            .find(|(_idx, f)| f.name() == name)
            .map(|(idx, _)| *idx)
    }

    pub fn push_function(&mut self, func: Func) -> FuncIndex {
        // TODO: check for duplicate func names
        let idx = self.next_free_function_idx();
        self.functions.insert(idx, func);
        idx
    }

    pub fn set_function(&mut self, idx: FuncIndex, func: Func) {
        // TODO: check for duplicate func names
        self.functions.insert(idx, func);
    }

    pub fn remove_function(&mut self, idx: &FuncIndex) {
        self.functions.remove(idx);
    }

    pub fn func_names(&self) -> HashMap<FuncIndex, String> {
        self.functions
            .iter()
            .map(|(idx, func)| (*idx, func.name().to_string()))
            .collect()
    }

    #[allow(clippy::unwrap_used)]
    pub fn next_free_function_idx(&self) -> FuncIndex {
        *self.functions.keys().max().unwrap() + FuncIndex::from(1u32)
    }

    pub fn add_global(&mut self, ty: Ty) -> GlobalIndex {
        self.globals.push(ty);
        GlobalIndex::from(self.globals.len() as u32 - 1)
    }

    /// Adds the function and prepends it's call in the beginning of the start function.
    pub fn add_prologue_function(&mut self, func: Func) -> FuncIndex {
        let prologue_idx = self.push_function(func);
        let start_func = self.start_function_mut();
        start_func.prepend(Inst::Call {
            func_idx: prologue_idx,
        });
        prologue_idx
    }

    pub fn wrap_start_func(&mut self, name: String, before: Vec<Inst>, after: Vec<Inst>) {
        let start_func = self.start_function_mut();
        let new_start_func_body = before
            .into_iter()
            .chain(start_func.instructions().iter().cloned())
            .chain(after.into_iter())
            .collect();
        let new_start_func = Func::new(
            name,
            start_func.sig().clone(),
            Vec::new(),
            new_start_func_body,
        );
        let new_start_func_idx = self.push_function(new_start_func);
        self.start_func_idx = new_start_func_idx;
    }
}
