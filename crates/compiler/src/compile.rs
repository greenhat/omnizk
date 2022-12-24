use c2zk_codegen::codegen;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::translate;
use c2zk_frontend::FrontendConfig;
use c2zk_ir::pass::run_ir_passes;

use crate::CompileError;

pub fn compile(
    source: &[u8],
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let mut module = translate(source, frontend)?;
    run_ir_passes(&mut module, target.ir_passes());
    let code = codegen(module, target)?;
    Ok(code)
}
