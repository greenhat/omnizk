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

fn run(func: Func, module: &mut Module, nested_level: u32) -> Func {
    // TODO: exit early if there are no blocks
    let mut new_func_insts = Vec::new();
    let mut capture_opt: Option<Capture> = None;
    // let mut block_count = 0;
    let mut extracted_func_inst = Vec::new();
    for inst in func.instructions() {
        #[allow(clippy::wildcard_enum_match_arm)]
        match inst {
            Inst::Block { blockty: _ } => match capture_opt {
                None => {
                    capture_opt = Some(Capture::start_block());
                }
                Some(mut capture) => {
                    capture.inc_nested_level();
                }
            },
            Inst::Loop { block_type: _ } => match capture_opt {
                None => {
                    capture_opt = Some(Capture::start_block());
                }
                Some(mut capture) => {
                    capture.inc_nested_level();
                }
            },
            Inst::End => {
                match capture_opt {
                    Some(mut capture) => {
                        if capture.nested_level == 0 {
                            // extracted func prologue
                            extracted_func_inst.push(Inst::I32Const { value: 1 }); // exiting the function not with a Br* op and stop exit propagation
                            extracted_func_inst.push(Inst::Return);
                            let extracted_func = Func::new(extracted_func_inst.clone());
                            dbg!(&extracted_func_inst);
                            extracted_func_inst.clear();
                            let extracted_func_idx = module.push_function(extracted_func.clone());
                            // call the extracted func
                            new_func_insts.push(Inst::Call {
                                func_idx: extracted_func_idx,
                            });

                            // handle Br* op
                            match capture.block_kind {
                                BlockKind::Block => {
                                    new_func_insts.push(Inst::I32Const { value: -1 });
                                    new_func_insts.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited the targeted by Br op block
                                    // TODO: does it mean we have to put on stack increased relative_depth for Block and untouched for Loop?
                                    new_func_insts.push(TritonExt::Skiz.into());
                                    new_func_insts.push(Inst::Return);
                                }
                                BlockKind::Loop => {
                                    new_func_insts.push(Inst::I32Const { value: -1 });
                                    new_func_insts.push(Inst::I32Add);
                                    // if not zero then return to the parent func(block), keep bailing out
                                    // zero is expected when we exited into the targeted by Br op loop
                                    new_func_insts.push(TritonExt::Skiz.into());
                                    new_func_insts.push(Inst::Return);
                                    new_func_insts.push(TritonExt::Recurse.into());
                                }
                            }
                            // recursevely extract nested blocks into functions
                            run(extracted_func, module, nested_level + 1);
                            capture_opt = None;
                        } else {
                            capture.dec_nested_level();
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
                extracted_func_inst.push(Inst::I32Const {
                    value: (relative_depth + 1) as i32,
                });
                extracted_func_inst.push(Inst::Return);
            }
            Inst::BrIf { relative_depth } => {
                extracted_func_inst.push(Inst::I32Const {
                    value: (relative_depth + 1) as i32,
                });
                extracted_func_inst.push(TritonExt::Swap { idx: 1 }.into());
                extracted_func_inst.push(TritonExt::Skiz.into());
                extracted_func_inst.push(Inst::Return);
                extracted_func_inst.push(TritonExt::Pop.into());
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
    // dbg!(&new_func_insts);
    Func::new(new_func_insts)
}
