mod triton;
pub use triton::TritonExt;
mod miden;
pub use miden::MidenExt;

#[derive(Debug, Clone)]
pub enum Ext {
    Triton(TritonExt),
    Miden(MidenExt),
}
