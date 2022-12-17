use c2zk_frontend_shared::{FuncBuilder, ModuleBuilder};
use wasmparser::{FuncValidator, Operator, WasmModuleResources};

use crate::error::WasmResult;

/// Translates wasm operators into c2zk IR instructions.
#[allow(unused_variables)]
pub fn translate_operator(
    validator: &mut FuncValidator<impl WasmModuleResources>,
    op: &Operator,
    func_builder: &mut FuncBuilder,
    mod_builder: &mut ModuleBuilder,
) -> WasmResult<()> {
    match op {
        Operator::Unreachable => {
            func_builder.ins().unreachable();
        }
        Operator::Nop => {
            func_builder.ins().nop();
        }
        Operator::End => func_builder.ins().end(),
        Operator::Return => func_builder.ins().ret(),
        Operator::Call { function_index } => {
            func_builder.push_insts(mod_builder.build_func_call(*function_index)?)
        }
        Operator::LocalGet { local_index } => func_builder.ins().local_get(*local_index),
        Operator::I32Const { value } => func_builder.ins().i32const(*value),
        Operator::I32Add => func_builder.ins().i32add(),
        Operator::I64Add => func_builder.ins().i64add(),
        _ => todo!("Wasm op not implemented: {:?}", op),
    };
    Ok(())
}
