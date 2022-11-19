use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_ir::ir::Module;
use triton_vm::instruction::AnInstruction;
use triton_vm::vm::Program;

use crate::emit_function;
use crate::emit_inst;
use crate::InstBuffer;
use crate::TritonTargetConfig;

pub struct TritonTarget {
    config: TritonTargetConfig,
}

impl Target for TritonTarget {
    fn name(&self) -> &str {
        "TritonVM"
    }

    fn compile_module(&self, module: &Module) -> Result<Vec<u8>, CodegenError> {
        let program = compile_module(module, &self.config)?;
        Ok(program.to_string().into_bytes())
    }
}

impl TritonTarget {
    pub fn new(config: TritonTargetConfig) -> TritonTarget {
        TritonTarget { config }
    }
}

pub fn compile_module(
    module: &Module,
    config: &TritonTargetConfig,
) -> Result<Program, CodegenError> {
    let mut sink = InstBuffer::new(config);
    let start_func = &module.functions()[module.start_func_idx as usize];
    emit_function(start_func, config, &mut sink)?;
    // TODO: remove the last op Return in start function
    sink.push(AnInstruction::Halt);
    for (idx, func) in module.functions().iter().enumerate() {
        if idx == module.start_func_idx as usize {
            continue;
        }
        sink.push_label(idx.to_string());
        for ins in func.instructions() {
            emit_inst(ins, config, &mut sink)?;
        }
    }
    Ok(sink.program())
}
