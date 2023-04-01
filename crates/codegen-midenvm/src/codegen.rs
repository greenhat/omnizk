use c2zk_ir::ir::Func;
use c2zk_ir::ir::Module;

mod inst_buf;
pub use inst_buf::InstBuffer;
mod emit;
pub use emit::emit_inst;

use crate::MidenError;
use crate::MidenTargetConfig;

#[cfg(test)]
mod sem_tests;

pub fn compile_module(
    module: Module,
    config: &MidenTargetConfig,
) -> Result<InstBuffer, MidenError> {
    todo!()
}

pub fn compile_function(
    func: Func,
    config: &MidenTargetConfig,
    sink: &mut InstBuffer,
    func_names: &[String],
) -> Result<(), MidenError> {
    todo!()
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(test)]
    fn check(input: &str, expected_tree: expect_test::Expect) {
        use c2zk_frontend::translate;
        use c2zk_frontend::FrontendConfig;
        use c2zk_frontend::WasmFrontendConfig;
        use c2zk_ir::pass::run_ir_passes;

        let source = wat::parse_str(input).unwrap();
        let frontend = FrontendConfig::Wasm(WasmFrontendConfig::default());
        let mut module = translate(&source, frontend).unwrap();
        let triton_target_config = MidenTargetConfig::default();
        run_ir_passes(&mut module, &triton_target_config.ir_passes);
        let triton_target_config = MidenTargetConfig::default();
        let inst_buf = compile_module(module, &triton_target_config).unwrap();
        let out_source = inst_buf.pretty_print();
        expected_tree.assert_eq(&out_source);
        let program = inst_buf.pretty_print();
        // TODO: execute the program
        todo!();
        // dbg!(&err);
        // assert!(err.is_none());
    }
}
