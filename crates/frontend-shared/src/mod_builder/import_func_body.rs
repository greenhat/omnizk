use std::collections::HashMap;

use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Ty;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ImportFunc {
    pub module: String,
    pub name: String,
    pub ty: FuncType,
}

#[derive(Debug)]
pub struct ImportFuncBody {
    mapping: HashMap<ImportFunc, Vec<Inst>>,
}

impl ImportFuncBody {
    pub const PUB_INPUT_FUNC_MODULE: &str = "env";
    pub const PUB_INPUT_FUNC_NAME: &str = "c2zk_stdlib_pub_input";
    pub const PUB_INPUT_OP: Inst = Inst::PubInputRead;

    pub const PUB_OUTPUT_FUNC_MODULE: &str = "env";
    pub const PUB_OUTPUT_FUNC_NAME: &str = "c2zk_stdlib_pub_output";

    pub const SECRET_INPUT_FUNC_MODULE: &str = "env";
    pub const SECRET_INPUT_FUNC_NAME: &str = "c2zk_stdlib_secret_input";
    pub const SECRET_INPUT_OP: Inst = Inst::SecretInputRead;

    pub fn new_stdlib() -> Self {
        Self {
            mapping: vec![
                (
                    ImportFunc {
                        module: Self::PUB_INPUT_FUNC_MODULE.to_string(),
                        name: Self::PUB_INPUT_FUNC_NAME.to_string(),
                        ty: FuncType::new(vec![], vec![Ty::I64]),
                    },
                    vec![Self::PUB_INPUT_OP, Inst::Return],
                ),
                (
                    ImportFunc {
                        module: Self::PUB_OUTPUT_FUNC_MODULE.to_string(),
                        name: Self::PUB_OUTPUT_FUNC_NAME.to_string(),
                        ty: FuncType::new(vec![Ty::I64], vec![]),
                    },
                    vec![
                        Inst::LocalGet { local_idx: 0 },
                        Inst::PubOutputWrite,
                        Inst::Return,
                    ],
                ),
                (
                    ImportFunc {
                        module: Self::SECRET_INPUT_FUNC_MODULE.to_string(),
                        name: Self::SECRET_INPUT_FUNC_NAME.to_string(),
                        ty: FuncType::new(vec![], vec![Ty::I64]),
                    },
                    vec![Self::SECRET_INPUT_OP, Inst::Return],
                ),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn body(&self, import_func: &ImportFunc) -> Option<&Vec<Inst>> {
        self.mapping.get(import_func)
    }
}
