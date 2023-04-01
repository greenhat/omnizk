use derive_more::From;
use derive_more::Into;

#[derive(Debug, Clone, Into, From)]
pub struct MidenInst(String);

pub struct MidenAssemblyBuilder {}

impl MidenAssemblyBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn proc(&mut self, name: String) -> MidenInst {
        format!("proc.{name}").into()
    }

    pub fn exec(&mut self, name: String) -> MidenInst {
        format!("call.{name}").into()
    }
}
