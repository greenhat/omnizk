use std::collections::HashMap;

use c2zk_codegen_shared::func_index_to_label;
use c2zk_ir::ir::ext::Ext;
use c2zk_ir::ir::ext::MidenExt;
use c2zk_ir::ir::FuncIndex;
use c2zk_ir::ir::Inst;
use thiserror::Error;

use crate::InstBuffer;
use crate::MidenAssemblyBuilder;
use crate::MidenTargetConfig;

#[derive(Debug, Error)]
pub enum EmitError {
    #[error("Unsupported instruction: {0:?}")]
    UnsupportedInstruction(Inst),
}

// TODO: add IR pass to remove LocalGet for accessing function parameters

#[allow(unused_variables)]
pub fn emit_inst(
    inst_iter: &mut impl Iterator<Item = Inst>,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &HashMap<FuncIndex, String>,
) -> Result<(), EmitError> {
    let b = MidenAssemblyBuilder::new();
    while let Some(inst) = inst_iter.next() {
        #[allow(clippy::wildcard_enum_match_arm)]
        match inst {
            Inst::End => sink.push(b.end()),
            Inst::Return => {
                if let Some(Inst::End) = inst_iter.peekable().peek() {
                    // Return followed by End shuold be replaced with just End
                    sink.push(b.end());
                    // consume the End
                    inst_iter.next();
                } else if inst_iter.peekable().peek().is_none() {
                    // Return at the end of the function should be replaced with End
                    sink.push(b.end());
                }
            }
            Inst::Dup { idx } => sink.push(b.dup(idx)),
            Inst::Swap { idx } => sink.push(b.swap(idx)),
            Inst::Call { func_idx } => sink.push(b.exec(func_index_to_label(func_idx, func_names))),
            Inst::I32Const { value } => sink.push(b.push_i64(value as i64)),
            Inst::I32Add => sink.push(b.add()),
            Inst::I32Sub => sink.push(b.sub()),
            Inst::I32Mul => sink.push(b.mul()),
            Inst::I32Store { offset } => emit_mem_store(sink, &b, offset as i32),
            Inst::I32Load { offset } => emit_mem_load(sink, &b, offset as i32),
            Inst::LocalSet { local_idx } => sink.push(b.loc_store(local_idx)),
            Inst::LocalGet { local_idx } => sink.push(b.loc_load(local_idx)),
            Inst::Drop => sink.push(b.drop()),
            Inst::Ext(Ext::Miden(miden_inst)) => match miden_inst {
                MidenExt::SDepth => sink.push(b.sdepth()),
                MidenExt::While => sink.push(b.while_true()),
                MidenExt::NeqImm(imm) => sink.push(b.neq_imm(imm)),
                MidenExt::Neq => sink.push(b.neq()),
            },
            inst => return Err(EmitError::UnsupportedInstruction(inst)),
        }
    }
    Ok(())
}

fn emit_mem_store(sink: &mut InstBuffer, builder: &MidenAssemblyBuilder, offset: i32) {
    // Midex expects address to be on top of the stack, but Wasm Store expects value to be on top
    sink.push(builder.swap(1));
    if offset != 0 {
        sink.push(builder.push_i64(offset as i64));
        sink.push(builder.add());
    }
    sink.push(builder.mem_store());
}

fn emit_mem_load(sink: &mut InstBuffer, builder: &MidenAssemblyBuilder, offset: i32) {
    if offset != 0 {
        sink.push(builder.push_i64(offset as i64));
        sink.push(builder.add());
    }
    sink.push(builder.mem_load());
}
