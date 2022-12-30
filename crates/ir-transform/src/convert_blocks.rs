use std::collections::HashMap;

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
            dbg!(&func_in);
            let func_out = run(func_in, module, 0);
            module.set_function(i as u32, func_out);
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
    let mut new_func_insts = Vec::new();
    let mut new_func_comments = HashMap::new();
    let mut capture_opt: Option<Capture> = None;
    let mut extracted_func_inst = Vec::new();
    let mut extracted_func_comments = HashMap::new();
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
                    extracted_func_inst.push(inst.clone());
                }
            },
            Inst::Loop { block_type: _ } => match capture_opt {
                None => {
                    capture_opt = Some(Capture::start_loop());
                }
                Some(mut capture) => {
                    capture.inc_nested_level();
                    capture_opt = Some(capture);
                    // nested block, keep extracting
                    extracted_func_inst.push(inst.clone());
                }
            },
            Inst::End => {
                match capture_opt {
                    Some(mut capture) => {
                        if capture.nested_level == 0 {
                            // extracted func prologue
                            extracted_func_comments.insert(
                                extracted_func_inst.len(),
                                format!("Begin: extracted func prologue ({block_nested_level})"),
                            );
                            extracted_func_inst.push(Inst::I32Const { value: 1 }); // exiting the function not with a Br* op and stop exit propagation
                            extracted_func_comments.insert(
                                extracted_func_inst.len(),
                                "End: extracted func prologue".to_string(),
                            );
                            extracted_func_inst.push(Inst::Return);
                            let extracted_func = Func::new_with_comments(
                                extracted_func_inst.clone(),
                                extracted_func_comments.clone(),
                            );
                            dbg!(&extracted_func_inst);
                            extracted_func_inst.clear();
                            extracted_func_comments.clear();
                            let extracted_func_idx = module.push_function(extracted_func.clone());
                            // call the extracted func
                            new_func_insts.push(Inst::Call {
                                func_idx: extracted_func_idx,
                            });

                            // handle Br* op
                            match capture.block_kind {
                                BlockKind::Block => {
                                    new_func_comments.insert(
                                        new_func_insts.len(),
                                        format!(
                                            "Begin: propagate Br* in block ({block_nested_level})"
                                        ),
                                    );
                                    new_func_insts.push(Inst::I32Const { value: -1 });
                                    new_func_insts.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited the targeted by Br op block
                                    // TODO: does it mean we have to put on stack increased relative_depth for Block and untouched for Loop?
                                    new_func_insts.push(TritonExt::Skiz.into());
                                    new_func_comments.insert(
                                        new_func_insts.len(),
                                        "End: propagate Br* in block".to_string(),
                                    );
                                    new_func_insts.push(Inst::Return);
                                }
                                BlockKind::Loop => {
                                    new_func_insts.push(Inst::I32Const { value: -1 });
                                    new_func_comments.insert(
                                        new_func_insts.len(),
                                        format!(
                                            "Begin: propagate Br* in loop ({block_nested_level})"
                                        ),
                                    );
                                    new_func_insts.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited into the targeted by Br op loop
                                    new_func_insts.push(TritonExt::Skiz.into());
                                    new_func_insts.push(Inst::Return);
                                    new_func_comments.insert(
                                        new_func_insts.len(),
                                        "End: propagate Br* in loop".to_string(),
                                    );
                                    new_func_insts.push(TritonExt::Recurse.into());
                                }
                            }
                            // recursevely extract nested blocks into functions
                            let func = run(extracted_func, module, block_nested_level + 1);
                            // TODO: comments in the replaced func are lost
                            module.set_function(extracted_func_idx.into(), func);
                            capture_opt = None;
                        } else {
                            capture.dec_nested_level();
                            capture_opt = Some(capture);
                            // nested block, keep extracting
                            extracted_func_inst.push(inst.clone());
                        }
                    }
                    None => {
                        new_func_insts.push(inst.clone());
                    }
                }
            }
            Inst::Br { relative_depth } => {
                if capture_opt.is_none() {
                    extracted_func_comments.insert(
                        extracted_func_inst.len(),
                        format!("Begin: Br call on nested ({block_nested_level})"),
                    );
                    extracted_func_inst.push(Inst::I32Const {
                        value: (relative_depth + 1) as i32,
                    });
                    extracted_func_comments.insert(
                        extracted_func_inst.len(),
                        format!("End: Br call on nested ({block_nested_level})"),
                    );
                    extracted_func_inst.push(Inst::Return);
                } else {
                    extracted_func_inst.push(inst.clone());
                }
            }
            Inst::BrIf { relative_depth } => {
                if capture_opt.is_none() {
                    extracted_func_comments.insert(
                        extracted_func_inst.len(),
                        format!("Begin: BrIf call on nested ({block_nested_level})"),
                    );
                    extracted_func_inst.push(Inst::I32Const {
                        value: (relative_depth + 1) as i32,
                    });
                    extracted_func_inst.push(TritonExt::Swap { idx: 1 }.into());
                    // TODO: "invert stack value since BrIf jumps on true (non zero)"
                    extracted_func_inst.push(TritonExt::Skiz.into());
                    extracted_func_inst.push(Inst::Return);
                    extracted_func_comments.insert(
                        extracted_func_inst.len(),
                        format!("End: BrIf call on nested ({block_nested_level})"),
                    );
                    extracted_func_inst.push(TritonExt::Pop.into());
                } else {
                    extracted_func_inst.push(inst.clone());
                }
            }
            _ => {
                if capture_opt.is_some() {
                    extracted_func_inst.push(inst.clone());
                } else {
                    new_func_insts.push(inst.clone())
                }
            }
        }
    }
    dbg!(&new_func_insts);
    Func::new_with_comments(new_func_insts, new_func_comments)
}
