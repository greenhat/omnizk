use super::Func;
use super::FuncIndex;
use super::GlobalIndex;
use super::Inst;
use super::Ty;

#[derive(Debug)]
pub struct Module {
    functions: Vec<Func>,
    pub start_func_idx: FuncIndex,
    globals: Vec<Ty>,
}

impl Module {
    pub fn new(functions: Vec<Func>, start_func_idx: FuncIndex, globals: Vec<Ty>) -> Self {
        Self {
            functions,
            start_func_idx,
            globals,
        }
    }

    pub fn functions(&self) -> &[Func] {
        &self.functions
    }

    pub fn into_functions(self) -> Vec<Func> {
        self.functions
    }

    pub fn functions_mut(&mut self) -> &mut [Func] {
        &mut self.functions
    }

    pub fn function(&self, idx: u32) -> Option<&Func> {
        self.functions.get(idx as usize)
    }

    pub fn function_by_name(&self, name: &str) -> Option<&Func> {
        self.functions.iter().find(|f| f.name() == name)
    }

    pub fn function_idx_by_name(&self, name: &str) -> Option<FuncIndex> {
        self.functions
            .iter()
            .position(|f| f.name() == name)
            .map(|i| i as u32)
            .map(Into::into)
    }

    pub fn push_function(&mut self, func: Func) -> FuncIndex {
        // TODO: check for duplicate func names
        self.functions.push(func);
        FuncIndex::from(self.functions.len() - 1)
    }

    pub fn set_function(&mut self, idx: FuncIndex, func: Func) {
        // TODO: check for duplicate func names
        self.functions[u32::from(idx) as usize] = func;
    }

    pub fn func_names(&self) -> Vec<String> {
        self.functions
            .iter()
            .map(|func| func.name().to_string())
            .collect()
    }

    pub fn next_free_function_idx(&self) -> FuncIndex {
        FuncIndex::from(self.functions.len() as u32)
    }

    pub fn add_global(&mut self, ty: Ty) -> GlobalIndex {
        self.globals.push(ty);
        GlobalIndex::from(self.globals.len() as u32 - 1)
    }

    /// Adds the function and prepends it's call in the beginning of the start function.
    pub fn add_prologue_function(&mut self, func: Func) -> FuncIndex {
        let prologue_idx = self.push_function(func);
        let start_func = &mut self.functions[u32::from(self.start_func_idx) as usize];
        start_func.prepend(Inst::Call {
            func_idx: prologue_idx,
        });
        prologue_idx
    }

    pub fn wrap_start_func(&mut self, name: String, before: Vec<Inst>, after: Vec<Inst>) {
        let start_func = &mut self.functions[u32::from(self.start_func_idx) as usize];
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
