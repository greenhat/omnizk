use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::BlockKind;
use c2zk_ir::ir::Func;
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

#[derive(Debug, Clone, Copy)]
struct Capture {
    nested_level: u32,
    block_kind: BlockKind,
}

impl Capture {
    fn start_block() -> Self {
        Self {
            nested_level: 0,
            block_kind: BlockKind::Block,
        }
    }

    fn start_loop() -> Self {
        Self {
            nested_level: 0,
            block_kind: BlockKind::Loop,
        }
    }

    fn inc_nested_level(&mut self) {
        self.nested_level += 1;
    }

    fn dec_nested_level(&mut self) {
        self.nested_level -= 1;
    }
}

fn run(func: Func, module: &mut Module, block_nested_level: u32) -> Func {
    // dbg!(&block_nested_level);
    // TODO: exit early if there are no blocks
    let mut new_func = Func::new(func.name().to_string(), Vec::new());
    let mut capture_opt: Option<Capture> = None;
    let mut extracted_func_count = 0;
    // TODO: extract into a closure (use in "reset" below)
    let mut extracted_func = Func::new(
        format!(
            "{}_l{block_nested_level}_b{extracted_func_count}",
            func.name()
        ),
        Vec::new(),
    );
    for inst in func.instructions() {
        // dbg!(&capture_opt);
        #[allow(clippy::wildcard_enum_match_arm)]
        match inst {
            Inst::Block { blockty: _ } => match capture_opt {
                None => {
                    capture_opt = Some(Capture::start_block());
                }
                Some(mut capture) => {
                    capture.inc_nested_level();
                    capture_opt = Some(capture);
                    // nested block, keep extracting
                    extracted_func.push(inst.clone());
                }
            },
            Inst::Loop { block_type: _ } => match capture_opt {
                None => {
                    capture_opt = Some(Capture::start_loop());
                }
                Some(mut capture) => {
                    capture.inc_nested_level();
                    // TODO: rewrite to avoid this
                    capture_opt = Some(capture);
                    // nested block, keep extracting
                    extracted_func.push(inst.clone());
                }
            },
            Inst::End => {
                match capture_opt {
                    Some(mut capture) => {
                        if capture.nested_level == 0 {
                            // dbg!(&extracted_func);
                            let extracted_func_idx = module.push_function(extracted_func.clone());
                            // call the extracted func
                            new_func.push(Inst::Call {
                                func_idx: extracted_func_idx,
                            });

                            // handle Br* op
                            match capture.block_kind {
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
                            extracted_func = Func::new(
                                format!(
                                    "{}_l{block_nested_level}_b{extracted_func_count}",
                                    func.name()
                                ),
                                Vec::new(),
                            );

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
                            capture_opt = None;
                        } else {
                            capture.dec_nested_level();
                            capture_opt = Some(capture);
                            // nested block, keep extracting
                            extracted_func.push(inst.clone());
                        }
                    }
                    None => {
                        new_func.push(inst.clone());
                    }
                }
            }
            Inst::Br { relative_depth } => {
                if capture_opt.is_none() {
                    extracted_func.push_with_comment(
                        Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        },
                        format!("Begin: Br call on nested ({block_nested_level})"),
                    );
                    extracted_func.push_with_comment(
                        Inst::Return,
                        format!("End: Br call on nested ({block_nested_level})"),
                    );
                } else {
                    extracted_func.push(inst.clone());
                }
            }
            Inst::BrIf { relative_depth } => {
                if capture_opt.is_none() {
                    extracted_func.push_with_comment(
                        Inst::I32Const {
                            value: (relative_depth + 1) as i32,
                        },
                        format!("Begin: BrIf call on nested ({block_nested_level})"),
                    );
                    extracted_func.push(TritonExt::Swap { idx: 1 }.into());
                    // TODO: "invert stack value since BrIf jumps on true (non zero)"
                    extracted_func.push(TritonExt::Skiz.into());
                    extracted_func.push(Inst::Return);
                    extracted_func.push_with_comment(
                        TritonExt::Pop.into(),
                        format!("End: BrIf call on nested ({block_nested_level})"),
                    );
                } else {
                    extracted_func.push(inst.clone());
                }
            }
            _ => {
                if capture_opt.is_some() {
                    extracted_func.push(inst.clone());
                } else {
                    new_func.push(inst.clone())
                }
            }
        }
    }
    // dbg!(&new_func);
    new_func
}
