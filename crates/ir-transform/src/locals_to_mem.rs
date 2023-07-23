/*
use ozk_ir::ir::Func;
use ozk_ir::ir::FuncType;
use ozk_ir::ir::GlobalIndex;
use ozk_ir::ir::Inst;
use ozk_ir::ir::Module;
use ozk_ir::ir::Ty;
use ozk_ir::pass::IrPass;

pub struct LocalsToMemPass {
    start_addr: i32,
}

impl LocalsToMemPass {
    pub fn new(start_addr: i32) -> Self {
        Self { start_addr }
    }
}

impl IrPass for LocalsToMemPass {
    #[allow(clippy::wildcard_enum_match_arm)]
    fn run_mod_pass(&self, module: &mut Module) {
        let global_idx_for_base_local_offset = module.add_global(Ty::I32);
        // dbg!(&module);
        let prologue_func =
            init_mem_for_locals_func(global_idx_for_base_local_offset, self.start_addr);
        for (_idx, func) in module.functions_iter_mut() {
            // dbg!(&func);
            let mut new_func = Func::new(
                func.name().to_string(),
                func.sig().clone(),
                Vec::new(),
                Vec::new(),
            );
            // dbg!(&func);
            if !func.locals().is_empty() {
                new_func.push(Inst::GlobalGet {
                    global_idx: global_idx_for_base_local_offset,
                });
                new_func.push(Inst::I32Const {
                    value: -(func.locals().len() as i32 * Ty::I32.size()),
                });
                new_func.push(Inst::I32Add);
                new_func.push(Inst::GlobalSet {
                    global_idx: global_idx_for_base_local_offset,
                });
            }

            // TODO: get type of the local and use the appropriate load instruction.
            let local_count = func.locals().len() as u32;
            // dbg!(&total_local_count);
            let reverse_index_base = if local_count > 0 {
                // although it looks like here should be total_local_count - 1
                // but the last pointer stored in global base_local_offset is NEXT address
                // after the last stored local, so it's total_local_count - 1 + 1
                // EDIT: now, since we decrease the start address by the size of the first local
                // we need to shift the index by 1
                local_count - 1
            } else {
                0
            };

            let mut iter = func.instructions_mut().iter_mut().peekable();
            while let Some(inst) = iter.next() {
                match inst {
                    Inst::LocalGet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        new_func.push(Inst::I32Load {
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size() as u32,
                        });
                    }
                    Inst::LocalSet { local_idx } => {
                        new_func.push(Inst::GlobalGet {
                            global_idx: global_idx_for_base_local_offset,
                        });
                        new_func.push(Inst::Swap { idx: 1 });
                        new_func.push(Inst::I32Store {
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size() as u32,
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
                            offset: (reverse_index_base - *local_idx) * Ty::I32.size() as u32,
                        });
                    }
                    Inst::Return => {
                        restore_base_local_offset(
                            inst,
                            &mut new_func,
                            global_idx_for_base_local_offset,
                            local_count,
                        );
                    }
                    Inst::End if iter.peek().is_none() => {
                        restore_base_local_offset(
                            inst,
                            &mut new_func,
                            global_idx_for_base_local_offset,
                            local_count,
                        );
                    }

                    _ => new_func.push(inst.clone()),
                };
            }
            *func = new_func;
        }
        module.add_prologue_function(prologue_func);
    }

    fn run_func_pass(&self, _func: &mut ozk_ir::ir::Func) {
        unreachable!();
    }
}

fn restore_base_local_offset(
    inst: &Inst,
    new_func: &mut Func,
    global_idx_for_base_local_offset: GlobalIndex,
    total_local_count: u32,
) {
    if total_local_count > 0 {
        // increase (rollback) the pointer stored in global base_local_offset
        // by the number of locals upon return
        new_func.push(Inst::GlobalGet {
            global_idx: global_idx_for_base_local_offset,
        });
        new_func.push(Inst::I32Const {
            value: total_local_count as i32 * Ty::I32.size(),
        });
        new_func.push(Inst::I32Add);
        new_func.push(Inst::GlobalSet {
            global_idx: global_idx_for_base_local_offset,
        });
    }
    // original (return or end) instruction
    new_func.push(inst.clone());
}

fn init_mem_for_locals_func(
    global_idx_for_base_local_offset: GlobalIndex,
    start_addr: i32,
) -> Func {
    Func::new(
        "init_mem_for_locals".to_string(),
        FuncType::void_void(),
        Vec::new(),
        vec![
            Inst::I32Const { value: start_addr },
            Inst::GlobalSet {
                global_idx: global_idx_for_base_local_offset,
            },
            Inst::Return,
        ],
    )
}
*/
