use super::Func;
use super::FuncIndex;

#[derive(Debug)]
pub struct Module {
    functions: Vec<Func>,
    pub start_func_idx: FuncIndex,
}

impl Module {
    pub fn new(functions: Vec<Func>, start_func_idx: FuncIndex) -> Self {
        Self {
            functions,
            start_func_idx,
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

    pub fn global_index_storing_base_local_offset(&self) -> u32 {
        // TODO: last existing global index + 1
        0
    }
}
