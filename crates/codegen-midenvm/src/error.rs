#[derive(Debug)]
pub enum MidenError {
    UnexpectedInst(String),
    InvalidInst(String),
}
