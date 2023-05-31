use c2zk_codegen::codegen;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::translate;
use c2zk_frontend::translate_old;
use c2zk_frontend::FrontendConfig;
use c2zk_ir::pass::run_ir_passes;
use pliron::context::Context;

use crate::CompileError;

pub fn compile(
    source: &[u8],
    frontend: FrontendConfig,
    _target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let mut ctx = Context::new();
    let _module = translate(&mut ctx, source, frontend)?;
    todo!("run_ir_passes(&mut module, target.ir_passes()");
    // run_ir_passes(&mut module, target.ir_passes());
    // let code = codegen(module, target)?;
    // Ok(code)
}

pub fn compile_old(
    source: &[u8],
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let mut module = translate_old(source, frontend)?;
    run_ir_passes(&mut module, target.ir_passes());
    let code = codegen(module, target)?;
    Ok(code)
}
