use c2zk_codegen_shared::CodegenError;
use c2zk_codegen_shared::Target;
use c2zk_codegen_tritonvm::TritonTarget;
use c2zk_ir::ir::Module;
use c2zk_ir::pass::IrPass;

use crate::TargetConfig;

pub fn codegen(module: Module, target_config: TargetConfig) -> Result<Vec<u8>, CodegenError> {
    Ok(match target_config {
        TargetConfig::Triton(config) => {
            let target = TritonTarget::new(config);
            let mut module = module;
            run_ir_passes(&mut module, target.ir_passes());
            target.compile_module(module)?
        }
    })
}

fn run_ir_passes(module: &mut Module, passes: Vec<Box<dyn IrPass>>) {
    for pass in passes {
        for mut func in module.functions.iter_mut() {
            pass.run_pass(&mut func, module);
        }
    }
}
