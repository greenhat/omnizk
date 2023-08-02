use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::isa::TargetFrontendConfig;
use cranelift_wasm::DummyEnvironment;
use cranelift_wasm::WasmError;
use ozk_wasm_dialect as wasm;
use pliron::context::Context;
use target_lexicon::PointerWidth;
use thiserror::Error;

use crate::code_translator::translate_op;
use crate::func_builder::FuncBuilder;
use crate::mod_builder::ModuleBuilder;
use crate::mod_builder::ModuleBuilderError;

pub fn parse_module(ctx: &mut Context, wasm: &[u8]) -> Result<wasm::ops::ModuleOp, ModuleError> {
    let cranelift_config = TargetFrontendConfig {
        default_call_conv: CallConv::SystemV,
        pointer_width: PointerWidth::U32,
    };
    let mut dummy_environ = DummyEnvironment::new(cranelift_config, true);
    cranelift_wasm::translate_module(wasm, &mut dummy_environ)?;

    let mut mod_builder = ModuleBuilder::new();
    let num_func_imports = dummy_environ.get_num_func_imports();
    for (def_index, func) in dummy_environ.info.function_bodies.iter() {
        let func_index = num_func_imports + def_index.index();
        if let Some(start_func) = dummy_environ.info.start_func {
            if func_index == start_func.index() {
                todo!("start func");
            }
        }
        let func_builder = build_func(ctx, func)?;
        mod_builder.push_func_builder(func_builder);
    }
    let module_op = mod_builder.build(ctx)?;
    Ok(module_op)
}

fn build_func(
    ctx: &mut Context,
    func: &cranelift_codegen::ir::Function,
) -> Result<FuncBuilder, ModuleError> {
    let mut fb = FuncBuilder::new(func.name.to_string().into());
    // TODO: set signature
    for clif_block in func.layout.blocks() {
        if func.layout.first_inst(clif_block).is_none() {
            todo!("Error: Empty block");
        }
        // TODO: get label
        let label = format!("block_{}", clif_block.as_u32());
        let block = fb.create_block(ctx, Some(label));
        fb.switch_to_block(block);
        for inst in func.layout.block_insts(clif_block) {
            let inst_data = func.dfg.insts[inst];
            // let dfg = &func.dfg;
            // let results = dfg.inst_results(inst);
            translate_op(ctx, &mut fb, inst_data);
        }
    }
    Ok(fb)
}

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("Wasm(CLIF) parsing error: {0}")]
    Wasm(#[from] WasmError),
    #[error("ModuleBuilder error: {0}")]
    ModuleBuilder(#[from] ModuleBuilderError),
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use expect_test::expect;
    use pliron::with_context::AttachContext;

    use crate::config::ClifFrontendConfig;

    use super::*;

    pub fn check_ir(wat: &str, expected_tree: expect_test::Expect) {
        let source = wat::parse_str(wat).unwrap();
        let mut ctx = Context::default();
        let frontend_config = ClifFrontendConfig::default();
        frontend_config.register(&mut ctx);
        let wasm_module_op = parse_module(&mut ctx, &source).unwrap();
        expected_tree.assert_eq(wasm_module_op.with_ctx(&ctx).to_string().as_str());
    }

    #[ignore]
    #[test]
    fn smoke() {
        check_ir(
            r#"
(module
    (start $main)
    (func $main
        i32.const 1
        i32.const 2
        i32.add
        return)
)"#,
            expect![[r#"
            }"#]],
        );
    }
}
