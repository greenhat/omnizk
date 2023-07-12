## OmniZK: Compiler Framework for Zero-Knowledge VMs.

OmniZK is a framework to transform code from various sources to various ZK VM's backends. Its design resembles the [MLIR](https://mlir.llvm.org/)(LLVM) architecture, where IR transformations are implemented generically and reused with different custom IR dialects. I started OmniZk because I believe that in the compilers for different ZK VMs will be a lot of similar code that can be shared and reused.

For example, If you want to compile Rust code to your ZK VM via Wasm, OmniZK gives you the Wasm parser and IR, IR transformations, ZK VM IR, and codegen. You can build your custom compilation pipeline by choosing what OmniZK transformations (optimizations, lowering, etc.) you want to use, adding your custom transformations as additional passes, and extending IRs with your custom ops.

OmniZK is highly modular so that you can use only crates needed for your use case - specific IRs, parsers, transformations, etc.

### Features:

- Wasm IR dialect and Wasm parser;
- IR dialects for ZK VMs ([Triton VM](https://github.com/TritonVM/triton-vm), [Miden VM](https://github.com/0xPolygonMiden/miden-vm/), etc.) and lowering conversion passes;
- IR transformations (lowering, optimizations, etc.) and codegen;
- Extend IRs with your custom ops and add your own IR transformations;

## Roadmap

The project is at an early development stage.

### Implemented:

- Wasm frontend (parser, IR dialect, etc.) for a small subset of instructions ;
- Triton VM backend (IR dialect, codegen, etc.);
- Wasm -> Triton VM transformations for a small subset of Wasm instructions;

### Work in progress:

- Wasm -> Miden VM transformations for a small subset of Wasm instructions (see [PR](https://github.com/greenhat/omnizk/pull/5));

### Next:
- look into adding Valida VM support;
- complete Wasm instructions support with lowering to Triton VM and Miden VM;
- custom ops support for complex computations in ZK VMs (hash, crypto, etc.) for the whole Rust->Wasm->ZKVM pipeline;
- More IR dialects support (LLVM, Move IR, Sway IR, etc.);


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

Here is how one can build a compiler from Wasm to Miden VM using OmniZK framework:

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
