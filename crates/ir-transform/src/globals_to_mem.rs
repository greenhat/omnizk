use c2zk_ir::ir::Func;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct GlobalsToMemPass;

const GLOBALS_GET_FUNC_NAME: &str = "globals_get";
const GLOBALS_SET_FUNC_NAME: &str = "globals_set";

impl IrPass for GlobalsToMemPass {
    fn run_mod_pass(&self, module: &mut Module) {
        let mut made_subst = false;
        let existing_global_get_func_idx = module.function_idx_by_name(GLOBALS_GET_FUNC_NAME);
        todo!("do global_set");
        // let existing_global_set_func_idx = module.function_idx_by_name(GLOBALS_SET_FUNC_NAME);
        let next_free_func_idx = module.next_free_function_idx();
        for func in module.functions_mut().iter_mut() {
            let found_global_get: Vec<(usize, GlobalIndex)> = func
                .instructions()
                .iter()
                .enumerate()
                .filter_map(|(idx, inst)| {
                    if let Inst::GlobalGet { global_idx } = inst {
                        Some((idx, *global_idx))
                    } else {
                        None
                    }
                })
                .collect();

            for (idx, global_idx) in found_global_get {
                func.instructions_as_vec_mut().insert(
                    idx,
                    Inst::I32Const {
                        value: u32::from(global_idx) as i32,
                    },
                );
                let inst = &mut func.instructions_as_vec_mut()[idx];
                if let Inst::GlobalGet { .. } = inst {
                    *inst = Inst::Call {
                        func_idx: existing_global_get_func_idx.unwrap_or(next_free_func_idx),
                    };
                    made_subst = true;
                }
            }
        }
        if made_subst && existing_global_get_func_idx.is_none() {
            let actual_func_idx = module.push_function(global_get_func());
            assert_eq!(actual_func_idx, next_free_func_idx);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

fn global_get_func() -> Func {
    todo!()
}
