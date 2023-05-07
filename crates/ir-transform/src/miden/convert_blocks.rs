use c2zk_frontend_shared::FuncBuilder;
use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::BlockKind;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
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
        let br_propagation_global_idx = module.add_global(Ty::I32);
        let func_indices: Vec<FuncIndex> = module.functions_iter().map(|(idx, _)| *idx).collect();
        for idx in func_indices {
            #[allow(clippy::unwrap_used)] // we just collected the indices (see above)
            let func_in = module.function(&idx).unwrap();
            let func_out = run(
                func_in.clone(),
                module,
                Vec::new(),
                br_propagation_global_idx,
            );
            module.set_function(idx, func_out);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

fn run(
    func: Func,
    module: &mut Module,
    traversed_blocks: Vec<BlockKind>,
    br_propagation_global_idx: GlobalIndex,
) -> Func {
    let mut new_func = Func::new(
        func.name().to_string(),
        func.sig().clone(),
        func.locals().to_vec(),
        Vec::new(),
    );
    let block_nested_level = traversed_blocks.len();
    let mut levels: Vec<BlockKind> = Vec::new();
    let mut extracted_func_count = 0;
    let mut extracted_func_builder = FuncBuilder::new(format!(
        "{}_l{block_nested_level}_b{extracted_func_count}",
        func.name()
    ));
    extracted_func_builder.set_signature(FuncType::new(vec![], vec![]));
    for inst in func.instructions() {
        // dbg!(&capture_opt);
        #[allow(clippy::wildcard_enum_match_arm)]
        #[allow(clippy::panic)]
        match inst {
            Inst::Block { blockty: _ } => {
                if !levels.is_empty() {
                    // nested block, keep extracting
                    extracted_func_builder.push(inst.clone());
                }
                levels.push(BlockKind::Block);
            }
            Inst::Loop { block_type: _ } => {
                if !levels.is_empty() {
                    // nested block, keep extracting
                    extracted_func_builder.push(inst.clone());
                }
                levels.push(BlockKind::Loop);
            }
            Inst::End => {
                match levels.pop() {
                    Some(block_kind) => {
                        if levels.is_empty() {
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
                                    new_func.push(Inst::Call {
                                        func_idx: module
                                            .function_idx_by_name(NEXT_BR_PROPAGATION_FUNC_NAME)
                                            .unwrap_or_else(|| {
                                                module.push_function(next_br_propagation(
                                                    br_propagation_global_idx,
                                                ))
                                            }),
                                    });
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited the targeted by Br op block
                                    new_func.push(TritonExt::Skiz.into());
                                    new_func.push(Inst::Return);
                                }
                                Some(BlockKind::Loop) => {
                                    new_func.push(Inst::Call {
                                        func_idx: module
                                            .function_idx_by_name(NEXT_BR_PROPAGATION_FUNC_NAME)
                                            .unwrap_or_else(|| {
                                                module.push_function(next_br_propagation(
                                                    br_propagation_global_idx,
                                                ))
                                            }),
                                    });
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
                                br_propagation_global_idx,
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
                if levels.len() == 1 {
                    if *relative_depth > 0 {
                        extracted_func_builder.push(Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        });
                        extracted_func_builder.push(Inst::GlobalSet {
                            global_idx: br_propagation_global_idx,
                        });
                        extracted_func_builder.push(Inst::Return);
                    } else {
                        match levels.first() {
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
                if levels.len() == 1 {
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
                        match levels.first() {
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
                if !levels.is_empty() {
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

const NEXT_BR_PROPAGATION_FUNC_NAME: &str = "next_br_propagation";

fn next_br_propagation(global_index_br_propagation: GlobalIndex) -> Func {
    let ins = vec![
        Inst::GlobalGet {
            global_idx: global_index_br_propagation,
        },
        Inst::Dup { idx: 0 },
        Inst::I32Eqz,
        TritonExt::Skiz.into(),
        // exiting with 0 on the stack
        Inst::Return,
        // decrease, store, return
        Inst::I32Const { value: -1 },
        Inst::I32Add,
        // leave return value
        Inst::Dup { idx: 0 },
        Inst::GlobalSet {
            global_idx: global_index_br_propagation,
        },
        Inst::Return,
    ];
    Func::new(
        NEXT_BR_PROPAGATION_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
    )
}
