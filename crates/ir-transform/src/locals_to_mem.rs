use std::collections::HashMap;

use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::pass::IrPass;

#[derive(Default)]
pub struct LocalsToMemPass;

impl IrPass for LocalsToMemPass {
    #[allow(clippy::wildcard_enum_match_arm)]
    fn run_mod_pass(&self, module: &mut c2zk_ir::ir::Module) {
        let global_idx_for_base_local_offset = module.global_index_storing_base_local_offset();
        // TODO: add func prologue read current base_local_offset from the global and decrement it by the declared locals count.
        // TODO: substitute all local references with memory references (base_local_offset + index).
        // base_local_offset should be put on the stack before every local access.
        // local index is used in load/store as offset from base_local_offset.

        // dbg!(&module);

        let prologue_func = mod_prologue_func(global_idx_for_base_local_offset);
        for func in module.functions_mut().iter_mut() {
            let mut new_func = Func::new(
                func.name().to_string(),
                func.sig().clone(),
                Vec::new(),
                HashMap::new(),
            );

            // dbg!(&func);
            // todo!("store func params and additionally shift by declared locals count");

            // store the function parameters to memory
            for (i, param) in func.sig().params.iter().enumerate() {
                new_func.push(Inst::GlobalGet {
                    global_idx: global_idx_for_base_local_offset,
                });
                // TODO: store op according to the param type
                new_func.push_with_comment(
                    Inst::I32Store { offset: 0 },
                    format!("store param {} to memory", i),
                );
                new_func.push(Inst::GlobalGet {
                    global_idx: global_idx_for_base_local_offset,
                });
                new_func.push(Inst::I32Const { value: -1 });
                new_func.push(Inst::I32Add);
                new_func.push(Inst::GlobalSet {
                    global_idx: global_idx_for_base_local_offset,
                });
            }
            new_func.push(Inst::GlobalGet {
                global_idx: global_idx_for_base_local_offset,
            });
            // TODO: get the number of locals besides function parameters from the function signature.
            new_func.push(Inst::I32Const { value: -2 });
            new_func.push(Inst::I32Add);
            new_func.push_with_comment(
                Inst::GlobalSet {
                    global_idx: global_idx_for_base_local_offset,
                },
                "END prologue for locals access via memory".to_string(),
            );
            for inst in func.instructions_mut().iter_mut() {
                match inst {
                    Inst::LocalGet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        // TODO: get type of the local and use the appropriate load instruction.
                        new_func.push(Inst::I32Load { offset: *local_idx });
                    }
                    Inst::LocalSet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        // TODO: get type of the local and use the appropriate load instruction.
                        new_func.push(Inst::I32Store { offset: *local_idx });
                    }
                    Inst::LocalTee { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        // TODO: get type of the local and use the appropriate load instruction.
                        new_func.push(Inst::I32Store { offset: *local_idx });
                        // we need to leave the original value on the stack
                        new_func.push(Inst::I32Load { offset: *local_idx });
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

fn mod_prologue_func(global_idx_for_base_local_offset: u32) -> Func {
    Func::new(
        "init_mem_for_locals".to_string(),
        FuncType::void_void(),
        vec![
            Inst::I32Const { value: i32::MAX },
            Inst::GlobalSet {
                global_idx: global_idx_for_base_local_offset,
            },
        ],
        HashMap::new(),
    )
}
