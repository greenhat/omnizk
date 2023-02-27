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
            let func_out = run(func_in, module, 0);
            module.set_function(i.into(), func_out);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
struct CaptureState {
    levels: Vec<BlockKind>,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self { levels: Vec::new() }
    }
}

impl CaptureState {
    fn inc_nested_level(&mut self, block_kind: BlockKind) {
        self.levels.push(block_kind);
    }

    fn dec_nested_level(&mut self) -> Option<BlockKind> {
        self.levels.pop()
    }

    fn nested_level(&self) -> usize {
        self.levels.len()
    }
}

fn run(func: Func, module: &mut Module, block_nested_level: u32) -> Func {
    // dbg!(&block_nested_level);
    // TODO: exit early if there are no blocks
    // TODO: use FuncBuilder?
    let mut new_func = Func::new(
        func.name().to_string(),
        func.sig().clone(),
        func.locals().to_vec(),
        Vec::new(),
        HashMap::new(),
    );
    let mut capture_state = CaptureState::default();
    let mut extracted_func_count = 0;
    // TODO: extract into a closure (use in "reset" below)
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
                capture_state.inc_nested_level(BlockKind::Block);
            }
            Inst::End => {
                dbg!(&capture_state);
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
                            match block_kind {
                                BlockKind::Block => {
                                    new_func.push_with_comment(
                                        Inst::I32Const { value: -1 },
                                        format!(
                                            "Begin: propagate Br* in block ({block_nested_level})"
                                        ),
                                    );
                                    new_func.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited the targeted by Br op block
                                    // TODO: does it mean we have to put on stack increased relative_depth for Block
                                    // and untouched for Loop?
                                    new_func.push(TritonExt::Skiz.into());
                                    new_func.push_with_comment(
                                        Inst::Return,
                                        "End: propagate Br* in block".to_string(),
                                    );
                                }
                                BlockKind::Loop => {
                                    new_func.push(Inst::I32Const { value: -1 });
                                    new_func.push_with_comment(
                                        Inst::I32Add,
                                        format!(
                                            "Begin: propagate Br* in loop ({block_nested_level})"
                                        ),
                                    );
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited into the targeted by Br op loop
                                    new_func.push(TritonExt::Skiz.into());
                                    new_func.push(Inst::Return);
                                    new_func.push_with_comment(
                                        TritonExt::Recurse.into(),
                                        "End: propagate Br* in loop".to_string(),
                                    );
                                }
                            }
                            // recursevely extract nested blocks into functions
                            let mut processed_func =
                                run(extracted_func, module, block_nested_level + 1);
                            extracted_func_count += 1;
                            extracted_func_builder = FuncBuilder::new(format!(
                                "{}_l{block_nested_level}_b{extracted_func_count}",
                                func.name()
                            ));
                            extracted_func_builder.set_signature(FuncType::new(vec![], vec![]));

                            // extracted func prologue
                            processed_func.set_comment(
                                processed_func.instructions().len(),
                                format!("Begin: extracted func prologue ({block_nested_level})"),
                            );
                            // exiting the function not with a Br* op and stop exit propagation
                            processed_func.push(Inst::I32Const { value: 1 });
                            processed_func.set_comment(
                                processed_func.instructions().len(),
                                "End: extracted func prologue".to_string(),
                            );
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
                if capture_state.nested_level() == 0 {
                    extracted_func_builder.push_with_comment(
                        Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        },
                        format!("Begin: Br call on nested ({block_nested_level})"),
                    );
                    extracted_func_builder.push_with_comment(
                        Inst::Return,
                        format!("End: Br call on nested ({block_nested_level})"),
                    );
                } else {
                    extracted_func_builder.push(inst.clone());
                }
            }
            Inst::BrIf { relative_depth } => {
                if capture_state.nested_level() == 0 {
                    extracted_func_builder.push_with_comment(
                        Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        },
                        format!("Begin: BrIf call on nested ({block_nested_level})"),
                    );
                    extracted_func_builder.push(Inst::Swap { idx: 1 });
                    // TODO: "invert stack value since BrIf jumps on true (non zero)"
                    extracted_func_builder.push(TritonExt::Skiz.into());
                    extracted_func_builder.push(Inst::Return);
                    extracted_func_builder.push_with_comment(
                        TritonExt::Pop.into(),
                        format!("End: BrIf call on nested ({block_nested_level})"),
                    );
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
