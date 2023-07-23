use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ImportFunc {
    pub module: String,
    pub name: String,
    pub ty: FuncType,
}

#[derive(Debug)]
pub struct ImportFuncBody {
    mapping: HashMap<ImportFunc, Func>,
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
                build_import_func(
                    Self::PUB_INPUT_FUNC_MODULE,
                    Self::PUB_INPUT_FUNC_NAME,
                    FuncType::new(vec![], vec![Ty::I64]),
                    Vec::new(),
                    vec![Self::PUB_INPUT_OP, Inst::Return],
                ),
                build_import_func(
                    Self::PUB_OUTPUT_FUNC_MODULE,
                    Self::PUB_OUTPUT_FUNC_NAME,
                    FuncType::new(vec![Ty::I64], vec![]),
                    Vec::new(),
                    vec![
                        Inst::LocalGet { local_idx: 0 },
                        Inst::PubOutputWrite,
                        Inst::Return,
                    ],
                ),
                build_import_func(
                    Self::SECRET_INPUT_FUNC_MODULE,
                    Self::SECRET_INPUT_FUNC_NAME,
                    FuncType::new(vec![], vec![Ty::I64]),
                    Vec::new(),
                    vec![Self::SECRET_INPUT_OP, Inst::Return],
                ),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn func(&self, import_func: &ImportFunc) -> Option<&Func> {
        self.mapping.get(import_func)
    }
}

fn build_import_func(
    module: &str,
    name: &str,
    ty: FuncType,
    locals: Vec<Ty>,
    body: Vec<Inst>,
) -> (ImportFunc, Func) {
    let import = ImportFunc {
        module: module.to_string(),
        name: name.to_string(),
        ty: ty.clone(),
    };
    let mut func_builder = FuncBuilder::new(name.to_string());
    func_builder.set_signature(ty);
    func_builder.declare_locals(locals);
    func_builder.push_insts(body);
    #[allow(clippy::unwrap_used)] // we build the function manually and it's defined at compile time
    let func = func_builder.build().unwrap();
    (import, func)
}
