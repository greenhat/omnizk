use derive_more::From;
use derive_more::Into;

#[derive(Debug, Clone, Into, From)]
pub struct MidenInst(String);

pub struct MidenAssemblyBuilder {}

impl MidenAssemblyBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn begin(&self) -> MidenInst {
        "begin".to_string().into()
    }

    pub fn proc(&self, name: String) -> MidenInst {
        format!("proc.{name}").into()
    }

    pub fn exec(&self, name: String) -> MidenInst {
        format!("exec.{name}").into()
    }

    pub fn push(&self, num: i64) -> MidenInst {
        format!("push.{num}").into()
    }

    pub fn adv_push(&self, num: u32) -> MidenInst {
        format!("adv_push.{num}").into()
    }

    pub fn end(&self) -> MidenInst {
        "end".to_string().into()
    }

    pub fn add(&self) -> MidenInst {
        "add".to_string().into()
    }
}
