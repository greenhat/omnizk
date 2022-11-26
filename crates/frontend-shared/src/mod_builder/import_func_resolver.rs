use std::collections::HashMap;

use c2zk_ir::ir::Inst;

pub struct ImportFunc {
    pub module: String,
    pub name: String,
}

pub struct ImportFuncResolver {
    mapping: HashMap<String, Vec<Inst>>,
}

impl ImportFuncResolver {
    pub const PUB_INPUT_FUNC_NAME: &str = "c2zk_stdlib_pub_input";
    pub const PUB_INPUT_OP: Inst = Inst::PubInputRead;

    pub const PUB_OUTPUT_FUNC_NAME: &str = "c2zk_stdlib_pub_output";
    pub const PUB_OUTPUT_OP: Inst = Inst::PubOutputWrite;

    pub fn new_stdlib() -> Self {
        Self {
            mapping: vec![
                (
                    Self::PUB_INPUT_FUNC_NAME.to_string(),
                    vec![Self::PUB_INPUT_OP],
                ),
                (
                    Self::PUB_OUTPUT_FUNC_NAME.to_string(),
                    vec![Self::PUB_OUTPUT_OP],
                ),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&Vec<Inst>> {
        self.mapping.get(name)
    }
}
