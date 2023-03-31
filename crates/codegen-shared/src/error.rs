#[derive(Debug)]
pub enum CodegenError {
    Triton(String),
    Miden(String),
}
