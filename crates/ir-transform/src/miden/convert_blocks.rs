#![allow(unused_variables)]
#![allow(dead_code)]

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
    todo!()
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

/*


Block:
  ...
  BrIf 0
  ...
  Block:
    ...
    BrIf 1 // to the top block
    ...
    Block:
      ...
      BrIf 1 // to the below-top block
      ...
      BrIf 2 // to the top block
      ...
    End
    ...
  End
  ...
End

 */
