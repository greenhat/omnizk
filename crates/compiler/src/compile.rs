use c2zk_codegen::codegen;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::translate;
use c2zk_frontend::FrontendConfig;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;
use c2zk_ir_transform::ConvertBlocksPass;

use crate::CompileError;

pub fn compile(
    source: &[u8],
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let module = translate(source, frontend)?;
    let code = codegen(module, target)?;
    Ok(code)
}
