use std::collections::HashMap;

use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct GlobalsToMemPass;

const GLOBALS_GET_FUNC_NAME: &str = "globals_get";
const GLOBALS_SET_FUNC_NAME: &str = "globals_set";

const GLOBAL_MEMORY_BASE: u32 = i32::MAX as u32;

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
            .unwrap_or_else(|| module.push_function(global_get_func()));
        let global_set_func_idx = module
            .function_idx_by_name(GLOBALS_SET_FUNC_NAME)
            .unwrap_or_else(|| module.push_function(global_set_func()));

        // dbg!(global_get_func_idx);
        // dbg!(global_set_func_idx);

        for func in module.functions_mut().iter_mut() {
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
                        value: -(u32::from(global_inst.global_idx()) as i32),
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

fn global_get_func() -> Func {
    let ins = vec![
        Inst::I32Const {
            value: GLOBAL_MEMORY_BASE as i32,
        },
        Inst::I32Add,
        Inst::I32Load { offset: 0 },
    ];
    Func::new(
        GLOBALS_GET_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}

fn global_set_func() -> Func {
    // first value, next pointer
    let ins = vec![
        Inst::I32Const {
            value: GLOBAL_MEMORY_BASE as i32,
        },
        Inst::I32Add,
        Inst::Swap { idx: 1 },
        Inst::I32Store { offset: 0 },
    ];
    Func::new(
        GLOBALS_SET_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}
