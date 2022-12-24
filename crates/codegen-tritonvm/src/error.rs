#[derive(Debug)]
pub enum TritonError {
    UnexpectedInst(String),
    InvalidInst(String),
}
