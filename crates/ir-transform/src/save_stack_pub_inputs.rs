//! In Miden VM public inputs are stored on the stack. This pass saves the public inputs from the stack
//! and stores them in the memory

use c2zk_ir::ir::ext::MidenExt;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
use c2zk_ir::pass::IrPass;

// TODO: since it's Miden specific we should move it to Miden crate or a miden module in this crate

pub struct SaveStackPubInputsPass {
    pub_inputs_start_address: i32,
    pub_outputs_start_address: i32,
}

impl SaveStackPubInputsPass {
    pub fn new(pub_inputs_start_address: i32, pub_outputs_start_address: i32) -> Self {
        Self {
            pub_inputs_start_address,
            pub_outputs_start_address,
        }
    }
}

pub const SAVE_PUB_INPUTS_FUNC_NAME: &str = "save_pub_inputs";
pub const GET_NEXT_PUB_INPUT_FUNC_NAME: &str = "omni_miden_pub_input";
pub const STORE_PUB_OUTPUT_FUNC_NAME: &str = "omni_miden_pub_output";
pub const LOAD_PUB_OUTPUTS_ON_STACK_FUNC_NAME: &str = "load_pub_outputs_on_stack";
pub const INIT_PUB_OUTPUTS_FUNC_NAME: &str = "init_pub_outputs";

impl IrPass for SaveStackPubInputsPass {
    fn run_mod_pass(&self, module: &mut Module) {
        let pub_inputs_addr_idx = module.add_global(Ty::I32);
        let pub_outputs_addr_idx = module.add_global(Ty::I32);
        let save_pub_inputs_func_idx = module
            .function_idx_by_name(SAVE_PUB_INPUTS_FUNC_NAME)
            .unwrap_or_else(|| {
                module.push_function(save_pub_inputs_func(
                    pub_inputs_addr_idx,
                    self.pub_inputs_start_address,
                ))
            });

        let get_next_pub_input_func_idx = module
            .function_idx_by_name(GET_NEXT_PUB_INPUT_FUNC_NAME)
            .unwrap_or_else(|| module.push_function(get_next_pub_input_func(pub_inputs_addr_idx)));

        let init_pub_outputs_func_idx = module
            .function_idx_by_name(INIT_PUB_OUTPUTS_FUNC_NAME)
            .unwrap_or_else(|| {
                module.push_function(init_pub_outputs_func(
                    pub_outputs_addr_idx,
                    self.pub_outputs_start_address,
                ))
            });

        let store_pub_output_func_idx = module
            .function_idx_by_name(STORE_PUB_OUTPUT_FUNC_NAME)
            .unwrap_or_else(|| module.push_function(store_pub_output_func(pub_outputs_addr_idx)));

        let load_pub_outputs_on_stack_func_idx = module
            .function_idx_by_name(LOAD_PUB_OUTPUTS_ON_STACK_FUNC_NAME)
            .unwrap_or_else(|| {
                module.push_function(load_pub_outputs_on_stack_func(
                    pub_outputs_addr_idx,
                    self.pub_outputs_start_address,
                ))
            });

        for (_idx, func) in module.functions_iter_mut() {
            for inst in func.instructions_mut() {
                if let Inst::PubInputRead = inst {
                    *inst = Inst::Call {
                        func_idx: get_next_pub_input_func_idx,
                    };
                } else if let Inst::PubOutputWrite = inst {
                    *inst = Inst::Call {
                        func_idx: store_pub_output_func_idx,
                    };
                }
            }
        }

        module.wrap_start_func(
            "start_with_miden_io_persistent".to_string(),
            vec![
                Inst::Call {
                    func_idx: save_pub_inputs_func_idx,
                },
                Inst::Call {
                    func_idx: init_pub_outputs_func_idx,
                },
            ],
            vec![
                Inst::Call {
                    func_idx: load_pub_outputs_on_stack_func_idx,
                },
                Inst::End,
            ],
        );
    }

    fn run_func_pass(&self, _func: &mut c2zk_ir::ir::Func) {
        unreachable!();
    }
}

fn save_pub_inputs_func(pub_inputs_addr_idx: GlobalIndex, pub_inputs_start_address: i32) -> Func {
    let ins = vec![
        // set the start address
        Inst::I32Const {
            value: pub_inputs_start_address,
        },
        Inst::LocalSet { local_idx: 0 },
        MidenExt::SDepth.into(), // get the current stack depth to enter the while.true loop
        Inst::LocalSet { local_idx: 1 },
        // Stack: [stack_depth, ...]
        Inst::I32Const { value: 1 },
        // // Stack: [1, ...]
        MidenExt::While.into(),
        Inst::Dup { idx: 0 },
        MidenExt::NeqImm(0).into(),
        MidenExt::If.into(),
        Inst::LocalGet { local_idx: 0 }, // get the current address
        Inst::Dup { idx: 0 },
        // Stack: [address, address, pub input values ...]
        Inst::Swap { idx: 2 }, // put the public input value on top
        // Stack: [pub input, address, address, pub input values ...]
        Inst::I32Store { offset: 0 }, // store the stack value
        // Stack: [address, pub input values ...]
        Inst::I32Const {
            value: Ty::I64.size(),
        },
        Inst::I32Sub, // decrement the address
        // Stack: [new address, pub input values ...]
        Inst::LocalSet { local_idx: 0 }, // set the new address
        MidenExt::Else.into(),
        Inst::Drop,
        // if end
        Inst::End,
        // Stack: [pub input values ...]
        Inst::LocalGet { local_idx: 1 }, // get the current stack depth
        Inst::I32Const { value: 1 },
        Inst::I32Sub,
        Inst::LocalTee { local_idx: 1 }, // decrement the stack depth
        MidenExt::NeqImm(0).into(),
        // while.true end
        Inst::End,
        Inst::LocalGet { local_idx: 0 }, // get the address
        Inst::GlobalSet {
            global_idx: pub_inputs_addr_idx,
        },
        // function end
        Inst::End,
    ];
    Func::new(
        SAVE_PUB_INPUTS_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![],
        },
        vec![Ty::I32, Ty::I32],
        ins,
    )
}

fn get_next_pub_input_func(pub_inputs_addr_idx: GlobalIndex) -> Func {
    let ins = vec![
        Inst::GlobalGet {
            global_idx: pub_inputs_addr_idx,
        }, // get the address
        Inst::I32Const {
            value: Ty::I64.size(),
        },
        Inst::I32Add,                // increment the address
        Inst::Dup { idx: 0 },        // duplicate the address
        Inst::I32Load { offset: 0 }, // load the previously saved public input
        Inst::Swap { idx: 1 },       // put the address on top
        Inst::GlobalSet {
            global_idx: pub_inputs_addr_idx,
        },
        Inst::End,
    ];
    Func::new(
        GET_NEXT_PUB_INPUT_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
    )
}

fn init_pub_outputs_func(
    pub_outputs_addr_idx: GlobalIndex,
    pub_outputs_start_address: i32,
) -> Func {
    let ins = vec![
        Inst::I32Const {
            value: pub_outputs_start_address,
        },
        Inst::GlobalSet {
            global_idx: pub_outputs_addr_idx,
        },
        Inst::End,
    ];
    Func::new(
        INIT_PUB_OUTPUTS_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![],
        },
        vec![],
        ins,
    )
}

fn store_pub_output_func(pub_outputs_addr_idx: GlobalIndex) -> Func {
    let ins = vec![
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_idx,
        }, // get the address
        Inst::Dup { idx: 0 },         // duplicate the address
        Inst::Swap { idx: 2 },        // put value on top
        Inst::I32Store { offset: 0 }, // store the stack value to public outputs memory region
        Inst::I32Const {
            value: -Ty::I64.size(),
        },
        Inst::I32Add, // decrement the address
        Inst::GlobalSet {
            global_idx: pub_outputs_addr_idx,
        },
        Inst::End,
    ];
    Func::new(
        STORE_PUB_OUTPUT_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![],
        },
        vec![],
        ins,
    )
}

fn load_pub_outputs_on_stack_func(
    pub_outputs_addr_idx: GlobalIndex,
    pub_inputs_start_address: i32,
) -> Func {
    let ins = vec![
        // get the address
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_idx,
        },
        Inst::LocalTee { local_idx: 0 },
        // get the original(start) address
        Inst::I32Const {
            value: pub_inputs_start_address,
        },
        Inst::I32Sub, // get the number of public outputs * 8 (i64 type size)
        MidenExt::NeqImm(0).into(), // while.true condition
        MidenExt::While.into(),
        Inst::LocalGet { local_idx: 0 }, // get the address
        Inst::Dup { idx: 0 },            // duplicate the address
        Inst::I32Load { offset: 0 },     // load the public output on the stack
        Inst::I32Const {
            value: Ty::I64.size(),
        },
        Inst::I32Add, // increment the address
        // set the address
        Inst::LocalTee { local_idx: 0 },
        // get the original(start) address
        Inst::I32Const {
            value: pub_inputs_start_address,
        },
        // get the number of public outputs * type size for while to continue (if > 0)
        Inst::I32Sub,
        Inst::Dup { idx: 0 },
        MidenExt::NeqImm(0).into(), // while.true condition
        // While.true end
        Inst::End,
        // function end
        Inst::End,
    ];
    Func::new(
        LOAD_PUB_OUTPUTS_ON_STACK_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![],
        },
        vec![Ty::I32],
        ins,
    )
}
