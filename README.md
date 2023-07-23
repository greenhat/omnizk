OmniZK is a framework to transform code from various sources to various ZK VM's backends. Its design resembles the [MLIR](https://mlir.llvm.org/)(LLVM) architecture, where IR transformations are implemented generically and reused with different custom IR dialects. I started OmniZk because I believe that in the compilers for different ZK VMs will be a lot of shared code that can be shared and reused. The project is at an early development stage.

Benefits of using OmniZK:
- ZK VM builders don't need to implement the whole compiler stack, just OmniZK dialect conversion into your instruction set (backend), and get all the high-level languages support for your ZK VM for free.
- High-level languages get all ZK VM backends support by only converting from their IR to OmniZK dialects(IR).
- Everyone gets all the OmniZK optimizations.

For example, If you want to compile Rust code to your ZK VM via Wasm, OmniZK gives you the Wasm parser and Wasm dialect(IR), IR transformations, ZK VMs IR, and codegen. You can build your custom compilation pipeline by choosing what OmniZK transformations (optimizations, lowering, etc.) you want to use, adding your custom transformations as additional passes, and extending IRs with your custom ops.

Optimizations include:
- Conversions into ZK-friendly ops for comparisons, loop unrolling, bit shifts, etc.
- Float arithmetic conversion;
- Generic optimizations (constant folding, dead code elimination, common sub-expression elimination, etc.)

OmniZK is highly modular so that you can use only crates needed for your use case - specific IRs, parsers, transformations, etc.


## Features:

- Wasm IR dialect and Wasm parser;
- IR dialects for ZK VMs ([Triton VM](https://github.com/TritonVM/triton-vm), [Miden VM](https://github.com/0xPolygonMiden/miden-vm/), [Valida VM](https://github.com/valida-xyz/valida), etc.) and lowering conversion passes;
- IR transformations (optimizations, etc.) and codegen;
- Extend with your IR dialects for your custom ops and add your own IR transformations;


## Use case examples

### Compile Rust to Triton VM via Wasm

The following [Fibonacci example](https://github.com/greenhat/omnizk/blob/2b7c7dd325ebf92711ad9f344dbef07dc14581a8/crates/rust-wasm-tests/fib/src/fib.rs) in Rust:
```rust
use ozk_stdlib::*;

pub fn fib_seq() {
    let n = pub_input() as u32;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    pub_output(a as u64);
}
```

is compiled into the following fully executable [Triton VM code](https://github.com/greenhat/omnizk/blob/2b7c7dd325ebf92711ad9f344dbef07dc14581a8/crates/codegen-tritonvm/src/codegen/sem_tests/fib.rs#L156).


### Compilation pipeline example (Wasm -> Miden VM)

Here is an example of a compiler's pipeline from Wasm to Miden VM using OmniZK framework:

```rust
pub fn compile(wasm: &[u8]) -> String {
    let frontend_config = WasmFrontendConfig::default();
    let target_config = MidenTargetConfig::default();
    let mut ctx = Context::new();
    frontend_config.register(&mut ctx);
    target_config.register(&mut ctx);
    let wasm_module_op =
        ozk_frontend_wasm::parse_module(&mut ctx, wasm, &frontend_config).unwrap();
    let miden_prog = run_conversion_passes(&mut ctx, wasm_module_op);
    let inst_buf = emit_prog(&ctx, miden_prog, &target_config).unwrap();
    inst_buf.pretty_print()
}

fn run_conversion_passes(ctx: &mut Context, wasm_module: ModuleOp) -> Ptr<Operation> {
    // we need to wrap the wasm in an op because passes cannot replace the root op
    let wrapper_module = builtin::ops::ModuleOp::new(ctx, "wrapper");
    wasm_module
        .get_operation()
        .insert_at_back(wrapper_module.get_body(ctx, 0), ctx);
    let mut pass_manager = PassManager::new();
    pass_manager.add_pass(Box::<WasmToMidenCFLoweringPass>::default());
    pass_manager.add_pass(Box::<WasmToMidenArithLoweringPass>::default());
    pass_manager.add_pass(Box::<WasmToMidenFinalLoweringPass>::default());
    pass_manager
        .run(ctx, wrapper_module.get_operation())
        .unwrap();
    let inner_module = wrapper_module
        .get_body(ctx, 0)
        .deref(ctx)
        .iter(ctx)
        .collect::<Vec<Ptr<Operation>>>()
        .first()
        .cloned()
        .unwrap();
    inner_module
}
```
from https://github.com/greenhat/omnizk/blob/ce2f0ebc7efa7bd82487000a0df2f7733be7304d/crates/codegen-midenvm/tests/sem_tests.rs#L35-L70

You can define your custom transformations as passes and extend IRs with your custom ops.


## How to build and run tests

Add rust Wasm target:
```bash
rustup target add wasm32-unknown-unknown
```
and `cargo build` and `cargo test` should work fine.
