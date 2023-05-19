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
        Operator::Loop { blockty } => {
            func_builder.ins().bloop(blockty.into_ir());
        }
        Operator::Block { blockty } => {
            func_builder.ins().block(blockty.into_ir());
        }
        Operator::BrIf { relative_depth } => {
            func_builder.ins().br_if(*relative_depth);
        }
        Operator::Br { relative_depth } => {
            func_builder.ins().br(*relative_depth);
        }
        Operator::LocalGet { local_index } => func_builder.ins().local_get(*local_index),
        Operator::LocalTee { local_index } => func_builder.ins().local_tee(*local_index),
        Operator::LocalSet { local_index } => func_builder.ins().local_set(*local_index),
        Operator::I32Const { value } => func_builder.ins().i32const(*value),
        Operator::I64Const { value } => func_builder.ins().i64const(*value),
        Operator::I32Add => func_builder.ins().i32add(),
        Operator::I32Eqz => func_builder.ins().i32eqz(),
        Operator::I32WrapI64 => func_builder.ins().i32wrapi64(),
        Operator::I32GeU => func_builder.ins().i32geu(),
        Operator::I32And => func_builder.ins().i32and(),
        Operator::I64Add => func_builder.ins().i64add(),
        Operator::I64Eqz => func_builder.ins().i64eqz(),
        Operator::I64And => func_builder.ins().i64and(),
        Operator::I64GeU => func_builder.ins().i64geu(),
        Operator::I64Ne => func_builder.ins().i64ne(),
        Operator::I64Eq => func_builder.ins().i64eq(),
        Operator::I64ExtendI32U => func_builder.ins().i64extendi32u(),
        _ => todo!("Wasm op not implemented: {:?}", op),
    };
    Ok(())
}
