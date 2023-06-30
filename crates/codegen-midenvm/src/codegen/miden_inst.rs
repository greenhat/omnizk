use derive_more::From;
use derive_more::Into;
use winter_math::fields::f64::BaseElement;
use winter_math::StarkField;

use crate::InstBuffer;

#[derive(Debug, Clone, Into, From)]
pub struct MidenInst(String);

pub struct MidenAssemblyBuilder {
    sink: InstBuffer,
}

impl MidenAssemblyBuilder {
    pub fn new(sink: InstBuffer) -> Self {
        Self { sink }
    }

    pub fn build(self) -> InstBuffer {
        self.sink
    }

    pub fn begin(&mut self) {
        self.sink.push("begin".to_string().into());
    }

    pub fn proc(&mut self, name: String, num_of_locals: usize) {
        self.sink
            .push(format!("proc.{name}.{num_of_locals}").into());
    }

    pub fn exec(&mut self, name: String) {
        self.sink.push(format!("exec.{name}").into());
    }

    pub fn push(&mut self, felt: BaseElement) {
        self.sink.push(format!("push.{felt}").into());
    }

    pub fn adv_push(&mut self, num: u32) {
        self.sink.push(format!("adv_push.{num}").into());
    }

    pub fn end(&mut self) {
        self.sink.push("end".to_string().into());
    }

    pub fn add(&mut self) {
        self.sink.push("add".to_string().into());
    }

    pub fn while_true(&mut self) {
        self.sink.push("while.true".to_string().into());
    }

    pub fn sdepth(&mut self) {
        self.sink.push("sdepth".to_string().into());
    }

    pub fn dup(&mut self, idx: u8) {
        self.sink.push(format!("dup.{idx}").into());
    }

    pub fn swap(&mut self, idx: u8) {
        self.sink.push(format!("swap.{idx}").into());
    }

    pub fn mul(&mut self) {
        self.sink.push("mul".to_string().into());
    }

    pub fn mem_store(&mut self) {
        self.sink.push("mem_store".to_string().into());
    }

    pub(crate) fn mem_load(&mut self) {
        self.sink.push("mem_load".to_string().into());
    }

    pub(crate) fn sub(&mut self) {
        self.sink.push("sub".to_string().into());
    }

    pub(crate) fn neq_imm(&mut self, imm: i32) {
        self.sink.push(format!("neq.{imm}").into());
    }

    pub(crate) fn loc_load(&mut self, local_idx: u32) {
        self.sink.push(format!("loc_load.{local_idx}").into());
    }

    pub(crate) fn loc_store(&mut self, local_idx: u32) {
        self.sink.push(format!("loc_store.{local_idx}").into());
    }

    pub(crate) fn neq(&mut self) {
        self.sink.push("neq".to_string().into());
    }

    pub(crate) fn drop(&mut self) {
        self.sink.push("drop".to_string().into());
    }

    pub(crate) fn if_true(&mut self) {
        self.sink.push("if.true".to_string().into());
    }

    pub(crate) fn if_else(&mut self) {
        self.sink.push("else".to_string().into());
    }
}

fn felt_i64(v: i64) -> BaseElement {
    if v < 0 {
        BaseElement::new(BaseElement::MODULUS - v.unsigned_abs())
    } else {
        BaseElement::new(v as u64)
    }
}
