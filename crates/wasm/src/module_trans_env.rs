use wasmparser::FuncType;

pub struct ModuleTranslationEnv {
    pub(crate) types: Vec<FuncType>,
}

impl ModuleTranslationEnv {
    pub(crate) fn new() -> ModuleTranslationEnv {
        ModuleTranslationEnv { types: Vec::new() }
    }
}
