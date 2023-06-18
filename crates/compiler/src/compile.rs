use c2zk_codegen::codegen;
use c2zk_codegen::codegen_old;
use c2zk_codegen::TargetConfig;
use c2zk_frontend::translate;
use c2zk_frontend::translate_old;
use c2zk_frontend::FrontendConfig;
use c2zk_ir::pass::run_ir_passes;
use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin;
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use pliron::operation::Operation;

use crate::CompileError;

pub fn compile(
    source: &[u8],
    frontend_config: FrontendConfig,
    target_config: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let mut ctx = Context::new();
    frontend_config.register(&mut ctx);
    target_config.register(&mut ctx);
    let wasm_module_op = translate(&mut ctx, source, frontend_config)?;
    let wrapper_module = builtin::ops::ModuleOp::new(&mut ctx, "wrapper");
    wasm_module_op
        .get_operation()
        .insert_at_back(wrapper_module.get_body(&ctx, 0), &ctx);
    target_config
        .pass_manager()
        .run(&mut ctx, wrapper_module.get_operation())?;
    #[allow(clippy::unwrap_used)]
    let inner_module = wrapper_module
        .get_body(&ctx, 0)
        .deref(&ctx)
        .iter(&ctx)
        .collect::<Vec<Ptr<Operation>>>()
        .first()
        .cloned()
        .unwrap();
    let code = codegen(&mut ctx, inner_module, target_config)?;
    Ok(code)
}

pub fn compile_old(
    source: &[u8],
    frontend: FrontendConfig,
    target: TargetConfig,
) -> Result<Vec<u8>, CompileError> {
    let mut module = translate_old(source, frontend)?;
    run_ir_passes(&mut module, target.ir_passes());
    let code = codegen_old(module, target)?;
    Ok(code)
}
