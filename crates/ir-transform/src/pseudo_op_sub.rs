use ozk_ir::ir::Func;
use ozk_ir::ir::FuncType;
use ozk_ir::ir::Inst;
use ozk_ir::ir::Module;
use ozk_ir::ir::Ty;
use ozk_ir::pass::IrPass;

#[derive(Default)]
pub struct PseudoOpSubPass;

const PSEUDO_OP_SUB_FUNC_NAME: &str = "i32_sub";

impl IrPass for PseudoOpSubPass {
    fn run_mod_pass(&self, module: &mut Module) {
        let mut made_subst = false;
        let existing_func_idx = module.function_idx_by_name(PSEUDO_OP_SUB_FUNC_NAME);
        let next_free_func_idx = module.next_free_function_idx();
        for (_idx, func) in module.functions_iter_mut() {
            for inst in func.instructions_mut().iter_mut() {
                if let Inst::I32Sub = inst {
                    *inst = Inst::Call {
                        func_idx: existing_func_idx.unwrap_or(next_free_func_idx),
                    };
                    made_subst = true;
                }
            }
        }
        if made_subst && existing_func_idx.is_none() {
            let actual_func_idx = module.push_function(triton_i32_sub_func());
            assert_eq!(actual_func_idx, next_free_func_idx);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

fn triton_i32_sub_func() -> Func {
    let ins = vec![
        Inst::Swap { idx: 2 },
        Inst::I32Const { value: -1 },
        Inst::I32Mul,
        Inst::I32Add,
    ];
    Func::new(
        PSEUDO_OP_SUB_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32, Ty::I32],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
    )
}
