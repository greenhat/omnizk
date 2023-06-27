#![allow(unused_variables)]
#![allow(dead_code)]

mod inst_buf;

pub use inst_buf::InstBuffer;
mod emit;
pub use emit::*;
mod miden_inst;
pub use miden_inst::*;
use ozk_miden_dialect::ops::*;
use pliron::context::Context;
use pliron::dialects::builtin::op_interfaces::get_callees_syms;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::op::Op;
use thiserror::Error;
use topological_sort::TopologicalSort;

use crate::MidenError;
use crate::MidenTargetConfig;

pub fn emit_prog(
    ctx: &Context,
    op: &ProgramOp,
    target_config: &MidenTargetConfig,
) -> Result<InstBuffer, MidenError> {
    todo!("compile miden program");
}

#[derive(Debug, Error)]
pub enum TopoSortError {
    #[error("Cycle in function dependencies: {0:?}")]
    Cycle(TopologicalSort<String>),
}

pub fn topo_sort_procedures(
    ctx: &Context,
    procedures: impl Iterator<Item = ProcOp>,
) -> Result<impl Iterator<Item = String>, TopoSortError> {
    let mut topo_sort = TopologicalSort::new();

    for proc in procedures {
        let proc_name = proc.get_symbol_name(ctx);
        topo_sort.insert(proc_name.clone());
        for dep in get_callees_syms(ctx, proc.get_operation()) {
            topo_sort.add_dependency(dep, proc_name.clone());
        }
    }
    let mut sorted = Vec::new();
    while !topo_sort.is_empty() {
        let mut func_indices = topo_sort.pop_all();
        if func_indices.is_empty() {
            return Err(TopoSortError::Cycle(topo_sort));
        }
        func_indices.sort();
        sorted.append(&mut func_indices);
    }
    Ok(sorted.into_iter())
}
