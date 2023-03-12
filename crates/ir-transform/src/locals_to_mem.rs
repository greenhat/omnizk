use std::collections::HashMap;

use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct LocalsToMemPass;

impl IrPass for LocalsToMemPass {
    #[allow(clippy::wildcard_enum_match_arm)]
    fn run_mod_pass(&self, module: &mut Module) {
        let global_idx_for_base_local_offset = module.global_index_storing_base_local_offset();
        // dbg!(&module);
        let prologue_func = mod_prologue_func(
            global_idx_for_base_local_offset,
            module.globals_alloc_size(),
        );
        for func in module.functions_mut().iter_mut() {
            let mut new_func = Func::new(
                func.name().to_string(),
                func.sig().clone(),
                Vec::new(),
                Vec::new(),
                HashMap::new(),
            );
            // dbg!(&func);
            if !func.sig().params.is_empty() {
                new_func.push(Inst::GlobalGet {
                    global_idx: global_idx_for_base_local_offset,
                });
                // store the function parameters to memory
                for (i, _param) in func.sig().params.iter().enumerate() {
                    new_func.push(Inst::Dup { idx: 0 });
                    // put func param on top
                    new_func.push(Inst::Swap { idx: 2 });
                    // TODO: store op according to the param type
                    new_func.push_with_comment(
                        Inst::I32Store { offset: 0 },
                        format!("store param {} to memory", i),
                    );
                    // decrease the pointer by the size of the param (4 bytes/i32 for now)
                    new_func.push(Inst::I32Const {
                        value: -(Ty::I32.size() as i32),
                    });
                    new_func.push(Inst::I32Add);
                }
                // store the pointer to the global
                new_func.push(Inst::GlobalSet {
                    global_idx: global_idx_for_base_local_offset,
                });
            }
            if !func.locals().is_empty() {
                new_func.push(Inst::GlobalGet {
                    global_idx: global_idx_for_base_local_offset,
                });
                new_func.push(Inst::I32Const {
                    value: -(func.locals().len() as i32 * Ty::I32.size() as i32),
                });
                new_func.push(Inst::I32Add);
                new_func.push_with_comment(
                    Inst::GlobalSet {
                        global_idx: global_idx_for_base_local_offset,
                    },
                    "END prologue for locals access via memory".to_string(),
                );
            }

            let param_count = func.sig().params.len() as u32;
            let local_count = func.locals().len() as u32;
            // TODO: get type of the local and use the appropriate load instruction.
            let total_local_count = param_count + local_count;
            // dbg!(&total_local_count);
            let reverse_index_base = if total_local_count > 0 {
                // although it looks like here should be total_local_count - 1
                // but the last pointer stored in global base_local_offset is NEXT address
                // after the last stored local, so it's total_local_count - 1 + 1
                total_local_count
            } else {
                0
            };
            for inst in func.instructions_mut().iter_mut() {
                match inst {
                    Inst::LocalGet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        new_func.push(Inst::I32Load {
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size(),
                        });
                    }
                    Inst::LocalSet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        new_func.push(Inst::Swap { idx: 1 });
                        new_func.push(Inst::I32Store {
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size(),
                        });
                    }
                    Inst::LocalTee { local_idx } => {
                        // we need to leave the original value on the stack
                        new_func.push(Inst::Dup { idx: 0 });
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        new_func.push(Inst::Swap { idx: 1 });
                        new_func.push(Inst::I32Store {
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size(),
                        });
                    }
                    Inst::Return | Inst::End => {
                        if total_local_count > 0 {
                            // increase (rollback) the pointer stored in global base_local_offset
                            // by the number of locals upon return
                            new_func.push(Inst::GlobalGet {
                                global_idx: global_idx_for_base_local_offset,
                            });
                            new_func.push(Inst::I32Const {
                                value: total_local_count as i32 * Ty::I32.size() as i32,
                            });
                            new_func.push(Inst::I32Add);
                            new_func.push(Inst::GlobalSet {
                                global_idx: global_idx_for_base_local_offset,
                            });
                        }
                        // original (return or end) instruction
                        new_func.push(inst.clone());
                    }
                    _ => new_func.push(inst.clone()),
                };
            }
            *func = new_func;
        }
        module.add_prologue_function(prologue_func);
    }

    fn run_func_pass(&self, _func: &mut c2zk_ir::ir::Func) {
        unreachable!();
    }
}

fn mod_prologue_func(
    global_idx_for_base_local_offset: GlobalIndex,
    globals_alloc_size: u32,
) -> Func {
    Func::new(
        "init_mem_for_locals".to_string(),
        FuncType::void_void(),
        Vec::new(),
        vec![
            Inst::I32Const {
                value: i32::MAX - globals_alloc_size as i32,
            },
            Inst::GlobalSet {
                global_idx: global_idx_for_base_local_offset,
            },
            Inst::Return,
        ],
        HashMap::new(),
    )
}
