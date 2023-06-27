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
use pliron::dialects::builtin::op_interfaces::SingleBlockRegionInterface;
use pliron::dialects::builtin::op_interfaces::SymbolOpInterface;
use pliron::linked_list::ContainsLinkedList;
use pliron::op::Op;
use rustc_hash::FxHashMap;
use thiserror::Error;
use topological_sort::TopologicalSort;

use crate::MidenError;
use crate::MidenTargetConfig;

pub fn emit_prog(
    ctx: &Context,
    prog_op: &ProgramOp,
    target_config: &MidenTargetConfig,
) -> Result<InstBuffer, MidenError> {
    let mut inst_buf = InstBuffer::new(target_config);
    let body = prog_op.get_body(ctx, 0);
    let mut procs = Vec::new();
    for op in body.deref(ctx).iter(ctx) {
        let Ok(proc_op) = op
                    .deref(ctx)
                    .get_op(ctx)
                    .downcast::<ProcOp>() else {
                todo!("error. there should be only miden.proc ops in miden.program body");
            };
        procs.push(*proc_op);
    }
    let proc_map: FxHashMap<String, ProcOp> = procs
        .iter()
        .map(|proc| (proc.get_symbol_name(ctx), *proc))
        .collect();
    let sorted_procs = topo_sort_procedures(ctx, procs.into_iter())?;
    for proc_name in sorted_procs {
        #[allow(clippy::unwrap_used)] // topo sort should not introduce new proc syms
        let proc_op = proc_map.get(&proc_name).unwrap();
        let is_main_proc = proc_name == prog_op.get_main_proc_sym(ctx);
        emit_proc(ctx, proc_op, is_main_proc, target_config, &mut inst_buf)?;
    }
    Ok(inst_buf)
}

pub fn emit_proc(
    ctx: &Context,
    proc_op: &ProcOp,
    is_main_proc: bool,
    target_config: &MidenTargetConfig,
    sink: &mut InstBuffer,
) -> Result<(), MidenError> {
    let b = MidenAssemblyBuilder::new();
    if is_main_proc {
        sink.push(b.begin());
    } else {
        sink.push(b.proc(proc_op.get_symbol_name(ctx), 0));
    }
    for op in proc_op.get_entry_block(ctx).deref(ctx).iter(ctx) {
        emit_op(ctx, op, target_config, sink)?;
    }
    sink.push(b.end());
    Ok(())
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
        let mut proc_names = topo_sort.pop_all();
        if proc_names.is_empty() {
            return Err(TopoSortError::Cycle(topo_sort));
        }
        proc_names.sort();
        sorted.append(&mut proc_names);
    }
    Ok(sorted.into_iter())
}
