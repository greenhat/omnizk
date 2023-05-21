use wasmparser::{FuncValidator, Operator, WasmModuleResources};

use crate::{func_builder::FuncBuilder, mod_builder::ModuleBuilder, types::IntoIr, WasmError};

/// Translates wasm operators into c2zk IR instructions.
#[allow(unused_variables)]
pub fn translate_operator(
    validator: &mut FuncValidator<impl WasmModuleResources>,
    op: &Operator,
    func_builder: &mut FuncBuilder,
    mod_builder: &mut ModuleBuilder,
) -> Result<(), WasmError> {
    match op {
        Operator::Unreachable => {
            func_builder.op().unreachable();
        }
        Operator::Nop => {
            func_builder.op().nop();
        }
        Operator::End => func_builder.op().end(),
        Operator::Return => func_builder.op().ret(),
        Operator::Call { function_index } => {
            let callee_name = mod_builder.get_func_name(*function_index)?;
            func_builder.op().call(callee_name);
        }
        Operator::Loop { blockty } => {
            func_builder.op().bloop(blockty.into_ir());
        }
        Operator::Block { blockty } => {
            func_builder.op().block(blockty.into_ir());
        }
        Operator::BrIf { relative_depth } => {
            func_builder.op().br_if(*relative_depth);
        }
        Operator::Br { relative_depth } => {
            func_builder.op().br(*relative_depth);
        }
        Operator::LocalGet { local_index } => func_builder.op().local_get(*local_index),
        Operator::LocalTee { local_index } => func_builder.op().local_tee(*local_index),
        Operator::LocalSet { local_index } => func_builder.op().local_set(*local_index),
        Operator::I32Const { value } => func_builder.op().i32const(*value),
        Operator::I64Const { value } => func_builder.op().i64const(*value),
        Operator::I32Add => func_builder.op().i32add(),
        Operator::I32Eqz => func_builder.op().i32eqz(),
        Operator::I32WrapI64 => func_builder.op().i32wrapi64(),
        Operator::I32GeU => func_builder.op().i32geu(),
        Operator::I32And => func_builder.op().i32and(),
        Operator::I64Add => func_builder.op().i64add(),
        Operator::I64Eqz => func_builder.op().i64eqz(),
        Operator::I64And => func_builder.op().i64and(),
        Operator::I64GeU => func_builder.op().i64geu(),
        Operator::I64Ne => func_builder.op().i64ne(),
        Operator::I64Eq => func_builder.op().i64eq(),
        Operator::I64ExtendI32U => func_builder.op().i64extendi32u(),
        _ => todo!("Wasm op not implemented: {:?}", op),
    };
    Ok(())
}
