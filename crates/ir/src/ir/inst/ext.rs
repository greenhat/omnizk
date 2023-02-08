mod triton;
pub use triton::TritonExt;

#[derive(Debug, Clone)]
pub enum Ext {
    Triton(TritonExt),
}
