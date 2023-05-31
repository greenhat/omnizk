## OmniZK: Compiler Framework for Zero-Knowledge VMs.

OmniZK transforms code from various sources to various ZK VM's backends. Its design resembles the [MLIR](https://mlir.llvm.org/)(LLVM) architecture, where IR transformations are implemented generically and reused with different custom IR dialects. 

### Features:

- generic IR transformations and optimizations (DCE, CSE, etc.) when compiling to your target ZK VM;
- OmniZK IR transformation pass infrastructure with your added passes;
- IR dialects (Wasm, [Triton VM](https://github.com/TritonVM/triton-vm), [Miden VM](https://github.com/0xPolygonMiden/miden-vm/), Move IR, Sway IR, etc.) and their transformations and optimizations;
- compile high-level languages like Rust to your ZK VM (via Wasm);

OmniZK is highly modular, so you can use only crates needed for your use case - specific IRs, parsers, transformations, etc.

## Roadmap

### Implemented:

- Wasm frontend (parser, IR dialect, etc.);
- Triton VM backend (IR dialect, codegen, etc.);
- Wasm -> Triton VM transformations;

### Work in progress:

- Wasm -> Miden VM transformations (see [PR](https://github.com/greenhat/omnizk/pull/5));

### Next:
- complete Wasm instructions support;
- Move IR dialect;
- Sway IR dialect.


## Use case examples

### Compile Rust to Triton VM via Wasm

The following [Fibonacci example](https://github.com/greenhat/omnizk/blob/main/crates/rust-wasm-tests/fib/src/fib.rs) in Rust:
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

Is compiled to the following fully executable [Triton VM code](https://github.com/greenhat/omnizk/blob/main/crates/codegen-tritonvm/src/codegen/sem_tests/fib.rs#L146).