use std::collections::HashMap;

use c2zk_frontend_shared::FuncBuilder;
use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::BlockKind;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

// TODO: since it's Triton specific, rename and/or move it to c2zk_ir_transform_tritonvm?
pub struct BlocksToFuncPass;

impl Default for BlocksToFuncPass {
    fn default() -> Self {
        Self::new()
    }
}

impl BlocksToFuncPass {
    pub fn new() -> Self {
        BlocksToFuncPass {}
    }
}

impl IrPass for BlocksToFuncPass {
    fn run_mod_pass(&self, module: &mut Module) {
        for i in 0..module.functions().len() {
            #[allow(clippy::unwrap_used)]
            let func_in = module.function(i as u32).unwrap().clone();
            // TODO: this cloned Func is a hack to get around the borrow checker
            // dbg!(&func_in);
            let func_out = run(func_in, module, Vec::new());
            module.set_function(i.into(), func_out);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

#[derive(Debug, Clone, Default)]
struct CaptureState {
    levels: Vec<BlockKind>,
}

impl CaptureState {
    fn inc_nested_level(&mut self, block_kind: BlockKind) {
        self.levels.push(block_kind);
    }

    // fn dec_nested_level(&mut self) -> Option<BlockKind> {
    //     self.levels.pop()
    // }

    fn nested_level(&self) -> usize {
        self.levels.len()
    }

    // fn peek_last_block(&self) -> Option<&BlockKind> {
    //     self.levels.last()
    // }
}

fn run(func: Func, module: &mut Module, traversed_blocks: Vec<BlockKind>) -> Func {
    // dbg!(&block_nested_level);
    // TODO: exit early if there are no blocks
    let mut new_func = Func::new(
        func.name().to_string(),
        func.sig().clone(),
        func.locals().to_vec(),
        Vec::new(),
        HashMap::new(),
    );
    let block_nested_level = traversed_blocks.len();
    let mut capture_state = CaptureState::default();
    let mut extracted_func_count = 0;
    // TODO: extract into a closure (use in "reset" below)
    let mut extracted_func_builder = FuncBuilder::new(format!(
        "{}_l{block_nested_level}_b{extracted_func_count}",
        func.name()
    ));
    extracted_func_builder.set_signature(FuncType::new(vec![], vec![]));
    let br_propagation_global_idx = module.global_index_storing_br_propagation();
    for inst in func.instructions() {
        // dbg!(&capture_opt);
        #[allow(clippy::wildcard_enum_match_arm)]
        #[allow(clippy::panic)]
        match inst {
            Inst::LocalGet { local_idx: _ }
            | Inst::LocalSet { local_idx: _ }
            | Inst::LocalTee { local_idx: _ } => {
                panic!("locals should be converted prior to this pass");
            }
            Inst::Block { blockty: _ } => {
                if capture_state.nested_level() > 0 {
                    // nested block, keep extracting
                    extracted_func_builder.push(inst.clone());
                }
                capture_state.inc_nested_level(BlockKind::Block);
            }
            Inst::Loop { block_type: _ } => {
                if capture_state.nested_level() > 0 {
                    // nested block, keep extracting
                    extracted_func_builder.push(inst.clone());
                }
                capture_state.inc_nested_level(BlockKind::Loop);
            }
            Inst::End => {
                // dbg!(&capture_state);
                match capture_state.levels.pop() {
                    Some(block_kind) => {
                        if capture_state.nested_level() == 0 {
                            // end of the root block, stop extracting

                            // the signature should be set in Block/Loop above
                            #[allow(clippy::unwrap_used)]
                            let extracted_func = extracted_func_builder.build().unwrap();
                            // dbg!(&extracted_func);
                            let extracted_func_idx = module.push_function(extracted_func.clone());
                            // call the extracted func
                            new_func.push(Inst::Call {
                                func_idx: extracted_func_idx,
                            });

                            // handle Br* op
                            match traversed_blocks.last() {
                                Some(BlockKind::Block) => {
                                    new_func.push(Inst::GlobalGet {
                                        global_idx: br_propagation_global_idx,
                                    });
                                    new_func.push(Inst::I32Const { value: -1 });
                                    new_func.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited the targeted by Br op block
                                    new_func.push(TritonExt::Skiz.into());
                                    new_func.push(Inst::Return);
                                }
                                Some(BlockKind::Loop) => {
                                    // decrease by 2 since we're recursing ("exiting" propagation early)
                                    // earlier in loop than in block?
                                    new_func.push(Inst::GlobalGet {
                                        global_idx: br_propagation_global_idx,
                                    });
                                    new_func.push(Inst::I32Const { value: -1 });
                                    new_func.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited into the targeted by Br op loop
                                    new_func.push(TritonExt::Skiz.into());
                                    new_func.push(TritonExt::Recurse.into());
                                }
                                None => {
                                    // we're in the top block, no need to propagate Br*
                                    // dbg!("popping");
                                }
                            }
                            // recursevely extract nested blocks into functions
                            let mut processed_func = run(
                                extracted_func,
                                module,
                                itertools::concat(vec![traversed_blocks.clone(), vec![block_kind]]),
                            );
                            extracted_func_count += 1;
                            extracted_func_builder = FuncBuilder::new(format!(
                                "{}_l{block_nested_level}_b{extracted_func_count}",
                                func.name()
                            ));
                            extracted_func_builder.set_signature(FuncType::new(vec![], vec![]));
                            processed_func.push(Inst::Return);
                            module.set_function(extracted_func_idx, processed_func);
                        } else {
                            // nested block, keep extracting
                            extracted_func_builder.push(inst.clone());
                        }
                    }
                    None => {
                        new_func.push(inst.clone());
                    }
                };
            }
            Inst::Br { relative_depth } => {
                dbg!(&capture_state);
                dbg!(&traversed_blocks);
                if capture_state.nested_level() == 1 {
                    // dbg!(&extracted_func_builder);
                    if *relative_depth > 0 {
                        extracted_func_builder.push(Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        });
                        extracted_func_builder.push(Inst::GlobalSet {
                            global_idx: br_propagation_global_idx,
                        });
                        extracted_func_builder.push(Inst::Return);
                    } else {
                        match capture_state.levels.first() {
                            Some(BlockKind::Block) => {
                                extracted_func_builder.push(Inst::Return);
                            }
                            Some(BlockKind::Loop) => {
                                extracted_func_builder.push(TritonExt::Recurse.into());
                            }
                            None => {
                                panic!("unexpected Br outside the block");
                            }
                        }
                    }
                } else {
                    extracted_func_builder.push(inst.clone());
                }
            }
            Inst::BrIf { relative_depth } => {
                if capture_state.nested_level() == 1 {
                    if *relative_depth > 0 {
                        // we are in the nested block so put relative_depth in the global for Br propagation
                        extracted_func_builder.push(Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        });
                        extracted_func_builder.push(Inst::GlobalSet {
                            global_idx: br_propagation_global_idx,
                        });
                        extracted_func_builder.push(TritonExt::Skiz.into());
                        extracted_func_builder.push(Inst::Return);
                        // br_if did not exit so clean up the global
                        extracted_func_builder.push(Inst::I32Const { value: 0 });
                        extracted_func_builder.push(Inst::GlobalSet {
                            global_idx: br_propagation_global_idx,
                        });
                    } else {
                        match capture_state.levels.first() {
                            Some(BlockKind::Block) => {
                                extracted_func_builder.push(TritonExt::Skiz.into());
                                extracted_func_builder.push(Inst::Return);
                            }
                            Some(BlockKind::Loop) => {
                                extracted_func_builder.push(TritonExt::Skiz.into());
                                extracted_func_builder.push(TritonExt::Recurse.into());
                            }
                            None => {
                                panic!("unexpected BrIf outside the block");
                            }
                        }
                    }
                } else {
                    extracted_func_builder.push(inst.clone());
                }
            }
            _ => {
                if capture_state.nested_level() > 0 {
                    extracted_func_builder.push(inst.clone());
                } else {
                    new_func.push(inst.clone())
                }
            }
        }
    }
    // dbg!(&new_func);
    new_func
}
