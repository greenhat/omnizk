use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::pass::IrPass;

/// Removes unused functions.
#[derive(Default)]
pub struct DceUnusedFunctionsPass;

impl IrPass for DceUnusedFunctionsPass {
    fn run_mod_pass(&self, module: &mut c2zk_ir::ir::Module) {
        let called_function_idxs = find_called_functions(module);
        let mut indeces_to_remove: Vec<FuncIndex> = Vec::new();
        for (idx, _func) in module.functions_iter() {
            if !called_function_idxs.contains(idx) && *idx != module.start_func_idx {
                indeces_to_remove.push(*idx);
            }
        }
        for idx in indeces_to_remove {
            module.remove_function(&idx);
        }
    }

    fn run_func_pass(&self, _func: &mut c2zk_ir::ir::Func) {
        unreachable!()
    }
}

fn find_called_functions(module: &c2zk_ir::ir::Module) -> Vec<FuncIndex> {
    let mut called_functions: Vec<FuncIndex> = Vec::new();
    for (_idx, func) in module.functions_iter() {
        for inst in func.instructions() {
            if let Inst::Call { func_idx } = inst {
                if called_functions.contains(func_idx) {
                    continue;
                }
                called_functions.push(*func_idx);
            }
        }
    }
    called_functions
}
