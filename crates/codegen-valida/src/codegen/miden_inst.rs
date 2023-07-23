use derive_more::From;
use derive_more::Into;

#[derive(Debug, Clone, Into, From)]
pub struct ValidaInst(String);

pub struct ValidaAssemblyBuilder {}

impl ValidaAssemblyBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn begin(&self) -> ValidaInst {
        "begin".to_string().into()
    }

    pub fn proc(&self, name: String, num_of_locals: usize) -> ValidaInst {
        format!("proc.{name}.{num_of_locals}").into()
    }

    pub fn exec(&self, name: String) -> ValidaInst {
        format!("exec.{name}").into()
    }

    pub fn push_i64(&self, num: i64) -> ValidaInst {
        todo!()
    }

    pub fn adv_push(&self, num: u32) -> ValidaInst {
        format!("adv_push.{num}").into()
    }

    pub fn end(&self) -> ValidaInst {
        "end".to_string().into()
    }

    pub fn add(&self) -> ValidaInst {
        "add".to_string().into()
    }

    pub fn while_true(&self) -> ValidaInst {
        "while.true".to_string().into()
    }

    pub fn sdepth(&self) -> ValidaInst {
        "sdepth".to_string().into()
    }

    pub fn dup(&self, idx: u8) -> ValidaInst {
        format!("dup.{idx}").into()
    }

    pub fn swap(&self, idx: u8) -> ValidaInst {
        format!("swap.{idx}").into()
    }

    pub fn mul(&self) -> ValidaInst {
        "mul".to_string().into()
    }

    pub fn mem_store(&self) -> ValidaInst {
        "mem_store".to_string().into()
    }

    pub(crate) fn mem_load(&self) -> ValidaInst {
        "mem_load".to_string().into()
    }

    pub(crate) fn sub(&self) -> ValidaInst {
        "sub".to_string().into()
    }

    pub(crate) fn neq_imm(&self, imm: i32) -> ValidaInst {
        format!("neq.{imm}").into()
    }

    pub(crate) fn loc_load(&self, local_idx: u32) -> ValidaInst {
        format!("loc_load.{local_idx}").into()
    }

    pub(crate) fn loc_store(&self, local_idx: u32) -> ValidaInst {
        format!("loc_store.{local_idx}").into()
    }

    pub(crate) fn neq(&self) -> ValidaInst {
        "neq".to_string().into()
    }

    pub(crate) fn drop(&self) -> ValidaInst {
        "drop".to_string().into()
    }

    pub(crate) fn if_true(&self) -> ValidaInst {
        "if.true".to_string().into()
    }

    pub(crate) fn if_else(&self) -> ValidaInst {
        "else".to_string().into()
    }
}

impl Default for ValidaAssemblyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
