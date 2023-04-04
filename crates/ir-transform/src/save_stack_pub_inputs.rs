//! In Miden VM public inputs are stored on the stack. This pass saves the public inputs from the stack
//! and stores them in the memory

use std::collections::HashMap;

use c2zk_ir::ir::ext::MidenExt;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
use c2zk_ir::pass::IrPass;

// TODO: we should convert Inst::PubInputRead and Inst::PubOutputWrite to memory access in this pass as well.
// TODO: since it's Miden specific we should move it to Miden crate or a miden module in this crate

#[derive(Default)]
pub struct SaveStackPubInputsPass;

pub const SAVE_PUB_INPUTS_FUNC_NAME: &str = "save_pub_inputs";

impl IrPass for SaveStackPubInputsPass {
    fn run_mod_pass(&self, module: &mut Module) {
        for func in module.functions_mut().iter_mut() {
            self.run_func_pass(func);
        }
    }

    fn run_func_pass(&self, func: &mut c2zk_ir::ir::Func) {
        todo!()
    }
}

fn save_pub_inputs_func() -> Func {
    // TODO: set proper
    let global_idx = GlobalIndex::from(0);
    // TODO: can be re-written with GlobalGet/GlobalSet moved outside the loop
    let ins = vec![
        MidenExt::SDepth.into(), // to enter the while.true loop
        MidenExt::While.into(),
        MidenExt::SDepth.into(),
        Inst::GlobalGet { global_idx }, // get the address
        Inst::Dup { idx: 0 },           // duplicate the address
        Inst::Swap { idx: 3 },          // put value on top
        Inst::I32Store { offset: 0 },   // store the stack value
        Inst::I32Const { value: -1 },
        Inst::I32Add,                   // decrement the address
        Inst::GlobalSet { global_idx }, // set the new address
        Inst::I32Const { value: -1 },
        Inst::I32Add, // decrement the stack depth counter (brought by SDepth)
        MidenExt::End.into(),
    ];
    Func::new(
        SAVE_PUB_INPUTS_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}
