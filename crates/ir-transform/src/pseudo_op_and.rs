use c2zk_ir::ir::ext::TritonExt;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

#[derive(Debug, Clone, Default)]
pub struct PseudoOpAndPass;

const PSEUDO_OP_AND_FUNC_NAME: &str = "i64_and";

impl IrPass for PseudoOpAndPass {
    fn run_mod_pass(&self, module: &mut Module) {
        let mut made_subst = false;
        let existing_func_idx = module.function_idx_by_name(PSEUDO_OP_AND_FUNC_NAME);
        let next_free_func_idx = module.next_free_function_idx();
        for func in module.functions_mut().iter_mut() {
            for inst in func.instructions_mut().iter_mut() {
                if let Inst::I64And = inst {
                    *inst = Inst::Call {
                        func_idx: existing_func_idx.unwrap_or(next_free_func_idx),
                    };
                    made_subst = true;
                }
            }
        }
        if made_subst && existing_func_idx.is_none() {
            let actual_func_idx = module.push_function(triton_i64_and_func());
            assert_eq!(actual_func_idx, next_free_func_idx);
        }
    }

    fn run_func_pass(&self, _func: &mut Func) {
        unreachable!()
    }
}

fn triton_i64_and_func() -> Func {
    let mut ins: Vec<Inst> = Vec::new();
    for _ in 0..32 {
        ins.push(TritonExt::Lsb.into());
        ins.push(TritonExt::Swap { idx: 2 }.into());
        ins.push(TritonExt::Lsb.into());
        ins.push(TritonExt::Swap { idx: 2 }.into());
    }
    for _ in 0..2 {
        ins.push(Inst::I64Eqz);
        ins.push(TritonExt::Assert.into());
    }
    ins.push(Inst::I64Const { value: 0 });
    for i in (0..32).rev() {
        ins.push(TritonExt::Swap { idx: 2 }.into());
        ins.push(Inst::I64Mul);
        ins.push(Inst::I64Const { value: 1 << i });
        ins.push(Inst::I64Mul);
        ins.push(Inst::I64Add);
    }
    Func::new(PSEUDO_OP_AND_FUNC_NAME.to_string(), ins)
}
