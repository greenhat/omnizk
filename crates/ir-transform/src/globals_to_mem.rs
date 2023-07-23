use ozk_ir::ir::Func;
use ozk_ir::ir::FuncType;
use ozk_ir::ir::GlobalIndex;
use ozk_ir::ir::Inst;
use ozk_ir::ir::Module;
use ozk_ir::ir::Ty;
use ozk_ir::pass::IrPass;

pub struct GlobalsToMemPass {
    start_addr: i32,
}

impl GlobalsToMemPass {
    pub fn new(start_addr: i32) -> Self {
        Self { start_addr }
    }
}

const GLOBALS_GET_FUNC_NAME: &str = "globals_get";
const GLOBALS_SET_FUNC_NAME: &str = "globals_set";

#[derive(Debug, Clone)]
enum GlobalInst {
    GlobalGet { global_idx: GlobalIndex },
    GlobalSet { global_idx: GlobalIndex },
}

impl GlobalInst {
    fn global_idx(&self) -> GlobalIndex {
        match self {
            GlobalInst::GlobalGet { global_idx } => *global_idx,
            GlobalInst::GlobalSet { global_idx } => *global_idx,
        }
    }
}

impl IrPass for GlobalsToMemPass {
    fn run_mod_pass(&self, module: &mut Module) {
        let global_get_func_idx = module
            .function_idx_by_name(GLOBALS_GET_FUNC_NAME)
            .unwrap_or_else(|| module.push_function(global_get_func(self.start_addr)));
        let global_set_func_idx = module
            .function_idx_by_name(GLOBALS_SET_FUNC_NAME)
            .unwrap_or_else(|| module.push_function(global_set_func(self.start_addr)));

        // dbg!(global_get_func_idx);
        // dbg!(global_set_func_idx);

        for (_idx, func) in module.functions_iter_mut() {
            let found_globals: Vec<(usize, GlobalInst)> = func
                .instructions()
                .iter()
                .enumerate()
                .filter_map(|(idx, inst)| to_global_inst(idx, inst))
                .collect();

            for (idx, global_inst) in found_globals.clone() {
                let inst_mut = &mut func.instructions_as_vec_mut()[idx];
                match global_inst {
                    GlobalInst::GlobalGet { global_idx: _ } => {
                        *inst_mut = Inst::Call {
                            func_idx: global_get_func_idx,
                        };
                    }
                    GlobalInst::GlobalSet { global_idx: _ } => {
                        *inst_mut = Inst::Call {
                            func_idx: global_set_func_idx,
                        };
                    }
                };
            }

            // insert the global index as an argument to the call (before the call)
            // inserting an op shifts the indices of the following instructions
            for (offset, (idx, global_inst)) in found_globals.into_iter().enumerate() {
                func.instructions_as_vec_mut().insert(
                    idx + offset,
                    Inst::I32Const {
                        value: u32::from(global_inst.global_idx()) as i32,
                    },
                );
            }
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

fn to_global_inst(idx: usize, inst: &Inst) -> Option<(usize, GlobalInst)> {
    #[allow(clippy::wildcard_enum_match_arm)]
    match inst {
        Inst::GlobalGet { global_idx } => Some((
            idx,
            GlobalInst::GlobalGet {
                global_idx: *global_idx,
            },
        )),
        Inst::GlobalSet { global_idx } => Some((
            idx,
            GlobalInst::GlobalSet {
                global_idx: *global_idx,
            },
        )),
        _ => None,
    }
}

fn global_get_func(start_addr: i32) -> Func {
    let ins = vec![
        // treat each global value size as 4 bytes (i32)
        Inst::I32Const {
            value: -Ty::I32.size(),
        },
        Inst::I32Mul,
        Inst::I32Const { value: start_addr },
        // it's actually a decrease (see negative type size above)
        Inst::I32Add,
        Inst::I32Load { offset: 0 },
        Inst::Return,
    ];
    Func::new(
        GLOBALS_GET_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
    )
}

fn global_set_func(start_addr: i32) -> Func {
    // first value, next pointer
    let ins = vec![
        // treat each global value size as 4 bytes (i32)
        Inst::I32Const {
            value: -Ty::I32.size(),
        },
        Inst::I32Mul,
        Inst::I32Const { value: start_addr },
        // it's actually a decrease (see negative type size above)
        Inst::I32Add,
        Inst::Swap { idx: 1 },
        Inst::I32Store { offset: 0 },
        Inst::Return,
    ];
    Func::new(
        GLOBALS_SET_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![],
        },
        vec![],
        ins,
    )
}
