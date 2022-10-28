//! Derived from Cranelift/wasm code.
//!
//! This module contains the bulk of the interesting code performing the translation between
//! WebAssembly and IR.
//!
//! The translation is done in one pass, opcode by opcode. Two main data structures are used during
//! code translations: the value stack and the control stack. The value stack mimics the execution
//! of the WebAssembly stack machine: each instruction result is pushed onto the stack and
//! instruction arguments are popped off the stack. Similarly, when encountering a control flow
//! block, it is pushed onto the control stack and popped off when encountering the corresponding
//! `End`.

use wasmparser::{FuncValidator, Operator, WasmModuleResources};

use crate::error::WasmResult;

// Clippy warns about "align: _" but its important to document that the flags field is ignored
#[cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::unneeded_field_pattern, clippy::cognitive_complexity)
)]
/// Translates wasm operators into Cranelift IR instructions. Returns `true` if it inserted
/// a return.
pub fn translate_operator(
    validator: &mut FuncValidator<impl WasmModuleResources>,
    op: &Operator,
) -> WasmResult<()> {
    // if !state.reachable {
    //     // translate_unreachable_operator(validator, &op, builder, state, environ)?;
    //     return Ok(());
    // }

    // Given that we believe the current block is reachable, the FunctionBuilder ought to agree.
    // debug_assert!(!builder.is_unreachable());

    // This big match treats all Wasm code operators.
    match op {
        /********************************** Locals ****************************************
         *  `get_local` and `set_local` are treated as non-SSA variables and will completely
         *  disappear in the Cranelift Code
         ***********************************************************************************/
        Operator::LocalGet { local_index } => {
            todo!();
            // let val = builder.use_var(Variable::from_u32(*local_index));
            // state.push1(val);
            // let label = ValueLabel::from_u32(*local_index);
            // builder.set_val_label(val, label);
        }
        Operator::LocalSet { local_index } => {
            todo!();
            // let mut val = state.pop1();

            // // Ensure SIMD values are cast to their default Cranelift type, I8x16.
            // let ty = builder.func.dfg.value_type(val);
            // if ty.is_vector() {
            //     val = optionally_bitcast_vector(val, I8X16, builder);
            // }

            // builder.def_var(Variable::from_u32(*local_index), val);
            // let label = ValueLabel::from_u32(*local_index);
            // builder.set_val_label(val, label);
        }
        Operator::LocalTee { local_index } => {
            todo!();
            // let mut val = state.peek1();

            // // Ensure SIMD values are cast to their default Cranelift type, I8x16.
            // let ty = builder.func.dfg.value_type(val);
            // if ty.is_vector() {
            //     val = optionally_bitcast_vector(val, I8X16, builder);
            // }

            // builder.def_var(Variable::from_u32(*local_index), val);
            // let label = ValueLabel::from_u32(*local_index);
            // builder.set_val_label(val, label);
        }
        /********************************** Globals ****************************************
         *  `get_global` and `set_global` are handled by the environment.
         ***********************************************************************************/
        Operator::GlobalGet { global_index } => {
            todo!();
            // let val = match state.get_global(builder.func, *global_index, environ)? {
            //     GlobalVariable::Const(val) => val,
            //     GlobalVariable::Memory { gv, offset, ty } => {
            //         let addr = builder.ins().global_value(environ.pointer_type(), gv);
            //         let mut flags = ir::MemFlags::trusted();
            //         // Put globals in the "table" abstract heap category as well.
            //         flags.set_table();
            //         builder.ins().load(ty, flags, addr, offset)
            //     }
            //     GlobalVariable::Custom => environ.translate_custom_global_get(
            //         builder.cursor(),
            //         GlobalIndex::from_u32(*global_index),
            //     )?,
            // };
            // state.push1(val);
        }
        Operator::GlobalSet { global_index } => {
            todo!();
            // match state.get_global(builder.func, *global_index, environ)? {
            //     GlobalVariable::Const(_) => panic!("global #{} is a constant", *global_index),
            //     GlobalVariable::Memory { gv, offset, ty } => {
            //         let addr = builder.ins().global_value(environ.pointer_type(), gv);
            //         let mut flags = ir::MemFlags::trusted();
            //         // Put globals in the "table" abstract heap category as well.
            //         flags.set_table();
            //         let mut val = state.pop1();
            //         // Ensure SIMD values are cast to their default Cranelift type, I8x16.
            //         if ty.is_vector() {
            //             val = optionally_bitcast_vector(val, I8X16, builder);
            //         }
            //         debug_assert_eq!(ty, builder.func.dfg.value_type(val));
            //         builder.ins().store(flags, val, addr, offset);
            //     }
            //     GlobalVariable::Custom => {
            //         let val = state.pop1();
            //         environ.translate_custom_global_set(
            //             builder.cursor(),
            //             GlobalIndex::from_u32(*global_index),
            //             val,
            //         )?;
            //     }
            // }
        }
        /********************************* Stack misc ***************************************
         *  `drop`, `nop`, `unreachable` and `select`.
         ***********************************************************************************/
        Operator::Drop => {
            todo!();
            // state.pop1();
        }
        Operator::Select => {
            todo!();
            // let (mut arg1, mut arg2, cond) = state.pop3();
            // if builder.func.dfg.value_type(arg1).is_vector() {
            //     arg1 = optionally_bitcast_vector(arg1, I8X16, builder);
            // }
            // if builder.func.dfg.value_type(arg2).is_vector() {
            //     arg2 = optionally_bitcast_vector(arg2, I8X16, builder);
            // }
            // state.push1(builder.ins().select(cond, arg1, arg2));
        }
        Operator::TypedSelect { ty: _ } => {
            todo!();
            // // We ignore the explicit type parameter as it is only needed for
            // // validation, which we require to have been performed before
            // // translation.
            // let (mut arg1, mut arg2, cond) = state.pop3();
            // if builder.func.dfg.value_type(arg1).is_vector() {
            //     arg1 = optionally_bitcast_vector(arg1, I8X16, builder);
            // }
            // if builder.func.dfg.value_type(arg2).is_vector() {
            //     arg2 = optionally_bitcast_vector(arg2, I8X16, builder);
            // }
            // state.push1(builder.ins().select(cond, arg1, arg2));
        }
        Operator::Nop => {
            // We do nothing
        }
        Operator::Unreachable => {
            todo!();
            // builder.ins().trap(ir::TrapCode::UnreachableCodeReached);
            // state.reachable = false;
        }
        /***************************** Control flow blocks **********************************
         *  When starting a control flow block, we create a new `Block` that will hold the code
         *  after the block, and we push a frame on the control stack. Depending on the type
         *  of block, we create a new `Block` for the body of the block with an associated
         *  jump instruction.
         *
         *  The `End` instruction pops the last control frame from the control stack, seals
         *  the destination block (since `br` instructions targeting it only appear inside the
         *  block and have already been translated) and modify the value stack to use the
         *  possible `Block`'s arguments values.
         ***********************************************************************************/
        Operator::Block { blockty } => {
            todo!();
            // let (params, results) = blocktype_params_results(validator, *blockty)?;
            // let next = block_with_params(builder, results.clone(), environ)?;
            // state.push_block(next, params.len(), results.len());
        }
        Operator::Loop { blockty } => {
            todo!();
            // let (params, results) = blocktype_params_results(validator, *blockty)?;
            // let loop_body = block_with_params(builder, params.clone(), environ)?;
            // let next = block_with_params(builder, results.clone(), environ)?;
            // canonicalise_then_jump(builder, loop_body, state.peekn(params.len()));
            // state.push_loop(loop_body, next, params.len(), results.len());

            // // Pop the initial `Block` actuals and replace them with the `Block`'s
            // // params since control flow joins at the top of the loop.
            // state.popn(params.len());
            // state
            //     .stack
            //     .extend_from_slice(builder.block_params(loop_body));

            // builder.switch_to_block(loop_body);
            // environ.translate_loop_header(builder)?;
        }
        Operator::If { blockty } => {
            todo!();
            // let val = state.pop1();

            // let (params, results) = blocktype_params_results(validator, *blockty)?;
            // let (destination, else_data) = if params.clone().eq(results.clone()) {
            //     // It is possible there is no `else` block, so we will only
            //     // allocate a block for it if/when we find the `else`. For now,
            //     // we if the condition isn't true, then we jump directly to the
            //     // destination block following the whole `if...end`. If we do end
            //     // up discovering an `else`, then we will allocate a block for it
            //     // and go back and patch the jump.
            //     let destination = block_with_params(builder, results.clone(), environ)?;
            //     let branch_inst =
            //         canonicalise_then_brz(builder, val, destination, state.peekn(params.len()));
            //     (destination, ElseData::NoElse { branch_inst })
            // } else {
            //     // The `if` type signature is not valid without an `else` block,
            //     // so we eagerly allocate the `else` block here.
            //     let destination = block_with_params(builder, results.clone(), environ)?;
            //     let else_block = block_with_params(builder, params.clone(), environ)?;
            //     canonicalise_then_brz(builder, val, else_block, state.peekn(params.len()));
            //     builder.seal_block(else_block);
            //     (destination, ElseData::WithElse { else_block })
            // };

            // let next_block = builder.create_block();
            // canonicalise_then_jump(builder, next_block, &[]);
            // builder.seal_block(next_block); // Only predecessor is the current block.
            // builder.switch_to_block(next_block);

            // // Here we append an argument to a Block targeted by an argumentless jump instruction
            // // But in fact there are two cases:
            // // - either the If does not have a Else clause, in that case ty = EmptyBlock
            // //   and we add nothing;
            // // - either the If have an Else clause, in that case the destination of this jump
            // //   instruction will be changed later when we translate the Else operator.
            // state.push_if(
            //     destination,
            //     else_data,
            //     params.len(),
            //     results.len(),
            //     *blockty,
            // );
        }
        Operator::Else => {
            todo!();
            //let i = state.control_stack.len() - 1;
            //match state.control_stack[i] {
            //    ControlStackFrame::If {
            //        ref else_data,
            //        head_is_reachable,
            //        ref mut consequent_ends_reachable,
            //        num_return_values,
            //        blocktype,
            //        destination,
            //        ..
            //    } => {
            //        // We finished the consequent, so record its final
            //        // reachability state.
            //        debug_assert!(consequent_ends_reachable.is_none());
            //        *consequent_ends_reachable = Some(state.reachable);

            //        if head_is_reachable {
            //            // We have a branch from the head of the `if` to the `else`.
            //            state.reachable = true;

            //            // Ensure we have a block for the `else` block (it may have
            //            // already been pre-allocated, see `ElseData` for details).
            //            let else_block = match *else_data {
            //                ElseData::NoElse { branch_inst } => {
            //                    let (params, _results) =
            //                        blocktype_params_results(validator, blocktype)?;
            //                    debug_assert_eq!(params.len(), num_return_values);
            //                    let else_block =
            //                        block_with_params(builder, params.clone(), environ)?;
            //                    canonicalise_then_jump(
            //                        builder,
            //                        destination,
            //                        state.peekn(params.len()),
            //                    );
            //                    state.popn(params.len());

            //                    builder.change_jump_destination(branch_inst, else_block);
            //                    builder.seal_block(else_block);
            //                    else_block
            //                }
            //                ElseData::WithElse { else_block } => {
            //                    canonicalise_then_jump(
            //                        builder,
            //                        destination,
            //                        state.peekn(num_return_values),
            //                    );
            //                    state.popn(num_return_values);
            //                    else_block
            //                }
            //            };

            //            // You might be expecting that we push the parameters for this
            //            // `else` block here, something like this:
            //            //
            //            //     state.pushn(&control_stack_frame.params);
            //            //
            //            // We don't do that because they are already on the top of the stack
            //            // for us: we pushed the parameters twice when we saw the initial
            //            // `if` so that we wouldn't have to save the parameters in the
            //            // `ControlStackFrame` as another `Vec` allocation.

            //            builder.switch_to_block(else_block);

            //            // We don't bother updating the control frame's `ElseData`
            //            // to `WithElse` because nothing else will read it.
            //        }
            //    }
            //    _ => unreachable!(),
            //}
        }
        Operator::End => {
            todo!();
            // let frame = state.control_stack.pop().unwrap();
            // let next_block = frame.following_code();
            // let return_count = frame.num_return_values();
            // let return_args = state.peekn_mut(return_count);

            // canonicalise_then_jump(builder, next_block, return_args);
            // // You might expect that if we just finished an `if` block that
            // // didn't have a corresponding `else` block, then we would clean
            // // up our duplicate set of parameters that we pushed earlier
            // // right here. However, we don't have to explicitly do that,
            // // since we truncate the stack back to the original height
            // // below.

            // builder.switch_to_block(next_block);
            // builder.seal_block(next_block);

            // // If it is a loop we also have to seal the body loop block
            // if let ControlStackFrame::Loop { header, .. } = frame {
            //     builder.seal_block(header)
            // }

            // frame.truncate_value_stack_to_original_size(&mut state.stack);
            // state
            //     .stack
            //     .extend_from_slice(builder.block_params(next_block));
        }
        /**************************** Branch instructions *********************************
         * The branch instructions all have as arguments a target nesting level, which
         * corresponds to how many control stack frames do we have to pop to get the
         * destination `Block`.
         *
         * Once the destination `Block` is found, we sometimes have to declare a certain depth
         * of the stack unreachable, because some branch instructions are terminator.
         *
         * The `br_table` case is much more complicated because Cranelift's `br_table` instruction
         * does not support jump arguments like all the other branch instructions. That is why, in
         * the case where we would use jump arguments for every other branch instruction, we
         * need to split the critical edges leaving the `br_tables` by creating one `Block` per
         * table destination; the `br_table` will point to these newly created `Blocks` and these
         * `Block`s contain only a jump instruction pointing to the final destination, this time with
         * jump arguments.
         *
         * This system is also implemented in Cranelift's SSA construction algorithm, because
         * `use_var` located in a destination `Block` of a `br_table` might trigger the addition
         * of jump arguments in each predecessor branch instruction, one of which might be a
         * `br_table`.
         ***********************************************************************************/
        Operator::Br { relative_depth } => {
            todo!();
            // let i = state.control_stack.len() - 1 - (*relative_depth as usize);
            // let (return_count, br_destination) = {
            //     let frame = &mut state.control_stack[i];
            //     // We signal that all the code that follows until the next End is unreachable
            //     frame.set_branched_to_exit();
            //     let return_count = if frame.is_loop() {
            //         frame.num_param_values()
            //     } else {
            //         frame.num_return_values()
            //     };
            //     (return_count, frame.br_destination())
            // };
            // let destination_args = state.peekn_mut(return_count);
            // canonicalise_then_jump(builder, br_destination, destination_args);
            // state.popn(return_count);
            // state.reachable = false;
        }
        Operator::BrIf { relative_depth } => todo!(), //translate_br_if(*relative_depth, builder, state),
        Operator::BrTable { targets } => {
            todo!();
            // let default = targets.default();
            // let mut min_depth = default;
            // for depth in targets.targets() {
            //     let depth = depth?;
            //     if depth < min_depth {
            //         min_depth = depth;
            //     }
            // }
            // let jump_args_count = {
            //     let i = state.control_stack.len() - 1 - (min_depth as usize);
            //     let min_depth_frame = &state.control_stack[i];
            //     if min_depth_frame.is_loop() {
            //         min_depth_frame.num_param_values()
            //     } else {
            //         min_depth_frame.num_return_values()
            //     }
            // };
            // let val = state.pop1();
            // let mut data = JumpTableData::with_capacity(targets.len() as usize);
            // if jump_args_count == 0 {
            //     // No jump arguments
            //     for depth in targets.targets() {
            //         let depth = depth?;
            //         let block = {
            //             let i = state.control_stack.len() - 1 - (depth as usize);
            //             let frame = &mut state.control_stack[i];
            //             frame.set_branched_to_exit();
            //             frame.br_destination()
            //         };
            //         data.push_entry(block);
            //     }
            //     let jt = builder.create_jump_table(data);
            //     let block = {
            //         let i = state.control_stack.len() - 1 - (default as usize);
            //         let frame = &mut state.control_stack[i];
            //         frame.set_branched_to_exit();
            //         frame.br_destination()
            //     };
            //     builder.ins().br_table(val, block, jt);
            // } else {
            //     // Here we have jump arguments, but Cranelift's br_table doesn't support them
            //     // We then proceed to split the edges going out of the br_table
            //     let return_count = jump_args_count;
            //     let mut dest_block_sequence = vec![];
            //     let mut dest_block_map = HashMap::new();
            //     for depth in targets.targets() {
            //         let depth = depth?;
            //         let branch_block = match dest_block_map.entry(depth as usize) {
            //             hash_map::Entry::Occupied(entry) => *entry.get(),
            //             hash_map::Entry::Vacant(entry) => {
            //                 let block = builder.create_block();
            //                 dest_block_sequence.push((depth as usize, block));
            //                 *entry.insert(block)
            //             }
            //         };
            //         data.push_entry(branch_block);
            //     }
            //     let default_branch_block = match dest_block_map.entry(default as usize) {
            //         hash_map::Entry::Occupied(entry) => *entry.get(),
            //         hash_map::Entry::Vacant(entry) => {
            //             let block = builder.create_block();
            //             dest_block_sequence.push((default as usize, block));
            //             *entry.insert(block)
            //         }
            //     };
            //     let jt = builder.create_jump_table(data);
            //     builder.ins().br_table(val, default_branch_block, jt);
            //     for (depth, dest_block) in dest_block_sequence {
            //         builder.switch_to_block(dest_block);
            //         builder.seal_block(dest_block);
            //         let real_dest_block = {
            //             let i = state.control_stack.len() - 1 - depth;
            //             let frame = &mut state.control_stack[i];
            //             frame.set_branched_to_exit();
            //             frame.br_destination()
            //         };
            //         let destination_args = state.peekn_mut(return_count);
            //         canonicalise_then_jump(builder, real_dest_block, destination_args);
            //     }
            //     state.popn(return_count);
            // }
            // state.reachable = false;
        }
        Operator::Return => {
            todo!();
            // let return_count = {
            //     let frame = &mut state.control_stack[0];
            //     frame.num_return_values()
            // };
            // {
            //     let return_args = state.peekn_mut(return_count);
            //     bitcast_wasm_returns(environ, return_args, builder);
            //     builder.ins().return_(return_args);
            // }
            // state.popn(return_count);
            // state.reachable = false;
        }
        /********************************** Exception handing **********************************/
        Operator::Try { .. }
        | Operator::Catch { .. }
        | Operator::Throw { .. }
        | Operator::Rethrow { .. }
        | Operator::Delegate { .. }
        | Operator::CatchAll => {
            todo!();
            // return Err(wasm_unsupported!(
            //     "proposed exception handling operator {:?}",
            //     op
            // ));
        }
        /************************************ Calls ****************************************
         * The call instructions pop off their arguments from the stack and append their
         * return values to it. `call_indirect` needs environment support because there is an
         * argument referring to an index in the external functions table of the module.
         ************************************************************************************/
        Operator::Call { function_index } => {
            todo!();
            // let (fref, num_args) = state.get_direct_func(builder.func, *function_index, environ)?;

            // // Bitcast any vector arguments to their default type, I8X16, before calling.
            // let args = state.peekn_mut(num_args);
            // bitcast_wasm_params(
            //     environ,
            //     builder.func.dfg.ext_funcs[fref].signature,
            //     args,
            //     builder,
            // );

            // let call = environ.translate_call(
            //     builder.cursor(),
            //     FuncIndex::from_u32(*function_index),
            //     fref,
            //     args,
            // )?;
            // let inst_results = builder.inst_results(call);
            // debug_assert_eq!(
            //     inst_results.len(),
            //     builder.func.dfg.signatures[builder.func.dfg.ext_funcs[fref].signature]
            //         .returns
            //         .len(),
            //     "translate_call results should match the call signature"
            // );
            // state.popn(num_args);
            // state.pushn(inst_results);
        }
        Operator::CallIndirect {
            type_index,
            table_index,
            table_byte: _,
        } => {
            todo!();
            // // `type_index` is the index of the function's signature and
            // // `table_index` is the index of the table to search the function
            // // in.
            // let (sigref, num_args) = state.get_indirect_sig(builder.func, *type_index, environ)?;
            // let table = state.get_or_create_table(builder.func, *table_index, environ)?;
            // let callee = state.pop1();

            // // Bitcast any vector arguments to their default type, I8X16, before calling.
            // let args = state.peekn_mut(num_args);
            // bitcast_wasm_params(environ, sigref, args, builder);

            // let call = environ.translate_call_indirect(
            //     builder,
            //     TableIndex::from_u32(*table_index),
            //     table,
            //     TypeIndex::from_u32(*type_index),
            //     sigref,
            //     callee,
            //     state.peekn(num_args),
            // )?;
            // let inst_results = builder.inst_results(call);
            // debug_assert_eq!(
            //     inst_results.len(),
            //     builder.func.dfg.signatures[sigref].returns.len(),
            //     "translate_call_indirect results should match the call signature"
            // );
            // state.popn(num_args);
            // state.pushn(inst_results);
        }
        /******************************* Memory management ***********************************
         * Memory management is handled by environment. It is usually translated into calls to
         * special functions.
         ************************************************************************************/
        Operator::MemoryGrow { mem, mem_byte: _ } => {
            todo!();
            // // The WebAssembly MVP only supports one linear memory, but we expect the reserved
            // // argument to be a memory index.
            // let heap_index = MemoryIndex::from_u32(*mem);
            // let heap = state.get_heap(builder.func, *mem, environ)?;
            // let val = state.pop1();
            // state.push1(environ.translate_memory_grow(builder.cursor(), heap_index, heap, val)?)
        }
        Operator::MemorySize { mem, mem_byte: _ } => {
            todo!();
            // let heap_index = MemoryIndex::from_u32(*mem);
            // let heap = state.get_heap(builder.func, *mem, environ)?;
            // state.push1(environ.translate_memory_size(builder.cursor(), heap_index, heap)?);
        }
        /******************************* Load instructions ***********************************
         * Wasm specifies an integer alignment flag but we drop it in Cranelift.
         * The memory base address is provided by the environment.
         ************************************************************************************/
        Operator::I32Load8U { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Uload8, I32, builder, state, environ)?;
        }
        Operator::I32Load16U { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Uload16, I32, builder, state, environ)?;
        }
        Operator::I32Load8S { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Sload8, I32, builder, state, environ)?;
        }
        Operator::I32Load16S { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Sload16, I32, builder, state, environ)?;
        }
        Operator::I64Load8U { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Uload8, I64, builder, state, environ)?;
        }
        Operator::I64Load16U { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Uload16, I64, builder, state, environ)?;
        }
        Operator::I64Load8S { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Sload8, I64, builder, state, environ)?;
        }
        Operator::I64Load16S { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Sload16, I64, builder, state, environ)?;
        }
        Operator::I64Load32S { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Sload32, I64, builder, state, environ)?;
        }
        Operator::I64Load32U { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Uload32, I64, builder, state, environ)?;
        }
        Operator::I32Load { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Load, I32, builder, state, environ)?;
        }
        Operator::F32Load { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Load, F32, builder, state, environ)?;
        }
        Operator::I64Load { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Load, I64, builder, state, environ)?;
        }
        Operator::F64Load { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Load, F64, builder, state, environ)?;
        }
        Operator::V128Load { memarg } => {
            todo!();
            // translate_load(memarg, ir::Opcode::Load, I8X16, builder, state, environ)?;
        }
        Operator::V128Load8x8S { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().sload8x8(flags, base, offset);
            // state.push1(loaded);
        }
        Operator::V128Load8x8U { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().uload8x8(flags, base, offset);
            // state.push1(loaded);
        }
        Operator::V128Load16x4S { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().sload16x4(flags, base, offset);
            // state.push1(loaded);
        }
        Operator::V128Load16x4U { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().uload16x4(flags, base, offset);
            // state.push1(loaded);
        }
        Operator::V128Load32x2S { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().sload32x2(flags, base, offset);
            // state.push1(loaded);
        }
        Operator::V128Load32x2U { memarg } => {
            todo!();
            // let (flags, base, offset) = prepare_addr(memarg, 8, builder, state, environ)?;
            // let loaded = builder.ins().uload32x2(flags, base, offset);
            // state.push1(loaded);
        }
        /****************************** Store instructions ***********************************
         * Wasm specifies an integer alignment flag but we drop it in Cranelift.
         * The memory base address is provided by the environment.
         ************************************************************************************/
        Operator::I32Store { memarg }
        | Operator::I64Store { memarg }
        | Operator::F32Store { memarg }
        | Operator::F64Store { memarg } => {
            todo!();
            // translate_store(memarg, ir::Opcode::Store, builder, state, environ)?;
        }
        Operator::I32Store8 { memarg } | Operator::I64Store8 { memarg } => {
            todo!();
            // translate_store(memarg, ir::Opcode::Istore8, builder, state, environ)?;
        }
        Operator::I32Store16 { memarg } | Operator::I64Store16 { memarg } => {
            todo!();
            // translate_store(memarg, ir::Opcode::Istore16, builder, state, environ)?;
        }
        Operator::I64Store32 { memarg } => {
            todo!();
            // translate_store(memarg, ir::Opcode::Istore32, builder, state, environ)?;
        }
        Operator::V128Store { memarg } => {
            todo!();
            // translate_store(memarg, ir::Opcode::Store, builder, state, environ)?;
        }
        /****************************** Nullary Operators ************************************/
        Operator::I32Const { value } => todo!(), //state.push1(builder.ins().iconst(I32, i64::from(*value))),
        Operator::I64Const { value } => todo!(), //state.push1(builder.ins().iconst(I64, *value)),
        Operator::F32Const { value } => {
            todo!();
            // state.push1(builder.ins().f32const(f32_translation(*value)));
        }
        Operator::F64Const { value } => {
            todo!();
            // state.push1(builder.ins().f64const(f64_translation(*value)));
        }
        /******************************* Unary Operators *************************************/
        Operator::I32Clz | Operator::I64Clz => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().clz(arg));
        }
        Operator::I32Ctz | Operator::I64Ctz => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().ctz(arg));
        }
        Operator::I32Popcnt | Operator::I64Popcnt => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().popcnt(arg));
        }
        Operator::I64ExtendI32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I64, val));
        }
        Operator::I64ExtendI32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().uextend(I64, val));
        }
        Operator::I32WrapI64 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I32, val));
        }
        Operator::F32Sqrt | Operator::F64Sqrt => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().sqrt(arg));
        }
        Operator::F32Ceil | Operator::F64Ceil => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().ceil(arg));
        }
        Operator::F32Floor | Operator::F64Floor => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().floor(arg));
        }
        Operator::F32Trunc | Operator::F64Trunc => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().trunc(arg));
        }
        Operator::F32Nearest | Operator::F64Nearest => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().nearest(arg));
        }
        Operator::F32Abs | Operator::F64Abs => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fabs(val));
        }
        Operator::F32Neg | Operator::F64Neg => {
            todo!();
            // let arg = state.pop1();
            // state.push1(builder.ins().fneg(arg));
        }
        Operator::F64ConvertI64U | Operator::F64ConvertI32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_from_uint(F64, val));
        }
        Operator::F64ConvertI64S | Operator::F64ConvertI32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_from_sint(F64, val));
        }
        Operator::F32ConvertI64S | Operator::F32ConvertI32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_from_sint(F32, val));
        }
        Operator::F32ConvertI64U | Operator::F32ConvertI32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_from_uint(F32, val));
        }
        Operator::F64PromoteF32 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fpromote(F64, val));
        }
        Operator::F32DemoteF64 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fdemote(F32, val));
        }
        Operator::I64TruncF64S | Operator::I64TruncF32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_sint(I64, val));
        }
        Operator::I32TruncF64S | Operator::I32TruncF32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_sint(I32, val));
        }
        Operator::I64TruncF64U | Operator::I64TruncF32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_uint(I64, val));
        }
        Operator::I32TruncF64U | Operator::I32TruncF32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_uint(I32, val));
        }
        Operator::I64TruncSatF64S | Operator::I64TruncSatF32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_sint_sat(I64, val));
        }
        Operator::I32TruncSatF64S | Operator::I32TruncSatF32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_sint_sat(I32, val));
        }
        Operator::I64TruncSatF64U | Operator::I64TruncSatF32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_uint_sat(I64, val));
        }
        Operator::I32TruncSatF64U | Operator::I32TruncSatF32U => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().fcvt_to_uint_sat(I32, val));
        }
        Operator::F32ReinterpretI32 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().bitcast(F32, val));
        }
        Operator::F64ReinterpretI64 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().bitcast(F64, val));
        }
        Operator::I32ReinterpretF32 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().bitcast(I32, val));
        }
        Operator::I64ReinterpretF64 => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().bitcast(I64, val));
        }
        Operator::I32Extend8S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I8, val));
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I32, val));
        }
        Operator::I32Extend16S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I16, val));
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I32, val));
        }
        Operator::I64Extend8S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I8, val));
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I64, val));
        }
        Operator::I64Extend16S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I16, val));
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I64, val));
        }
        Operator::I64Extend32S => {
            todo!();
            // let val = state.pop1();
            // state.push1(builder.ins().ireduce(I32, val));
            // let val = state.pop1();
            // state.push1(builder.ins().sextend(I64, val));
        }
        /****************************** Binary Operators ************************************/
        Operator::I32Add | Operator::I64Add => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().iadd(arg1, arg2));
        }
        Operator::I32And | Operator::I64And => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().band(arg1, arg2));
        }
        Operator::I32Or | Operator::I64Or => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().bor(arg1, arg2));
        }
        Operator::I32Xor | Operator::I64Xor => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().bxor(arg1, arg2));
        }
        Operator::I32Shl | Operator::I64Shl => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().ishl(arg1, arg2));
        }
        Operator::I32ShrS | Operator::I64ShrS => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().sshr(arg1, arg2));
        }
        Operator::I32ShrU | Operator::I64ShrU => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().ushr(arg1, arg2));
        }
        Operator::I32Rotl | Operator::I64Rotl => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().rotl(arg1, arg2));
        }
        Operator::I32Rotr | Operator::I64Rotr => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().rotr(arg1, arg2));
        }
        Operator::F32Add | Operator::F64Add => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fadd(arg1, arg2));
        }
        Operator::I32Sub | Operator::I64Sub => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().isub(arg1, arg2));
        }
        Operator::F32Sub | Operator::F64Sub => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fsub(arg1, arg2));
        }
        Operator::I32Mul | Operator::I64Mul => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().imul(arg1, arg2));
        }
        Operator::F32Mul | Operator::F64Mul => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fmul(arg1, arg2));
        }
        Operator::F32Div | Operator::F64Div => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fdiv(arg1, arg2));
        }
        Operator::I32DivS | Operator::I64DivS => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().sdiv(arg1, arg2));
        }
        Operator::I32DivU | Operator::I64DivU => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().udiv(arg1, arg2));
        }
        Operator::I32RemS | Operator::I64RemS => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().srem(arg1, arg2));
        }
        Operator::I32RemU | Operator::I64RemU => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().urem(arg1, arg2));
        }
        Operator::F32Min | Operator::F64Min => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fmin(arg1, arg2));
        }
        Operator::F32Max | Operator::F64Max => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fmax(arg1, arg2));
        }
        Operator::F32Copysign | Operator::F64Copysign => {
            todo!();
            // let (arg1, arg2) = state.pop2();
            // state.push1(builder.ins().fcopysign(arg1, arg2));
        }
        /**************************** Comparison Operators **********************************/
        Operator::I32LtS | Operator::I64LtS => {
            todo!();
            // translate_icmp(IntCC::SignedLessThan, builder, state)
        }
        Operator::I32LtU | Operator::I64LtU => {
            todo!();
            // translate_icmp(IntCC::UnsignedLessThan, builder, state)
        }
        Operator::I32LeS | Operator::I64LeS => {
            todo!();
            // translate_icmp(IntCC::SignedLessThanOrEqual, builder, state)
        }
        Operator::I32LeU | Operator::I64LeU => {
            todo!();
            // translate_icmp(IntCC::UnsignedLessThanOrEqual, builder, state)
        }
        Operator::I32GtS | Operator::I64GtS => {
            todo!();
            // translate_icmp(IntCC::SignedGreaterThan, builder, state)
        }
        Operator::I32GtU | Operator::I64GtU => {
            todo!();
            // translate_icmp(IntCC::UnsignedGreaterThan, builder, state)
        }
        Operator::I32GeS | Operator::I64GeS => {
            todo!();
            // translate_icmp(IntCC::SignedGreaterThanOrEqual, builder, state)
        }
        Operator::I32GeU | Operator::I64GeU => {
            todo!();
            // translate_icmp(IntCC::UnsignedGreaterThanOrEqual, builder, state)
        }
        Operator::I32Eqz | Operator::I64Eqz => {
            todo!();
            // let arg = state.pop1();
            // let val = builder.ins().icmp_imm(IntCC::Equal, arg, 0);
            // state.push1(builder.ins().uextend(I32, val));
        }
        Operator::I32Eq | Operator::I64Eq => todo!(), //translate_icmp(IntCC::Equal, builder, state),
        Operator::F32Eq | Operator::F64Eq => todo!(), //translate_fcmp(FloatCC::Equal, builder, state),
        Operator::I32Ne | Operator::I64Ne => todo!(), //translate_icmp(IntCC::NotEqual, builder, state),
        Operator::F32Ne | Operator::F64Ne => todo!(), //translate_fcmp(FloatCC::NotEqual, builder, state),
        Operator::F32Gt | Operator::F64Gt => todo!(), //translate_fcmp(FloatCC::GreaterThan, builder, state),
        Operator::F32Ge | Operator::F64Ge => {
            todo!();
            // translate_fcmp(FloatCC::GreaterThanOrEqual, builder, state)
        }
        Operator::F32Lt | Operator::F64Lt => todo!(), //translate_fcmp(FloatCC::LessThan, builder, state),
        Operator::F32Le | Operator::F64Le => {
            todo!();
            // translate_fcmp(FloatCC::LessThanOrEqual, builder, state)
        }
        Operator::RefNull { ty } => {
            todo!();
            // state.push1(environ.translate_ref_null(builder.cursor(), (*ty).try_into()?)?)
        }
        Operator::RefIsNull => {
            todo!();
            // let value = state.pop1();
            // state.push1(environ.translate_ref_is_null(builder.cursor(), value)?);
        }
        Operator::RefFunc { function_index } => {
            todo!();
            // let index = FuncIndex::from_u32(*function_index);
            // state.push1(environ.translate_ref_func(builder.cursor(), index)?);
        }
        Operator::MemoryAtomicWait32 { memarg } | Operator::MemoryAtomicWait64 { memarg } => {
            todo!();
            // // The WebAssembly MVP only supports one linear memory and
            // // wasmparser will ensure that the memory indices specified are
            // // zero.
            // let implied_ty = match op {
            //     Operator::MemoryAtomicWait64 { .. } => I64,
            //     Operator::MemoryAtomicWait32 { .. } => I32,
            //     _ => unreachable!(),
            // };
            // let heap_index = MemoryIndex::from_u32(memarg.memory);
            // let heap = state.get_heap(builder.func, memarg.memory, environ)?;
            // let timeout = state.pop1(); // 64 (fixed)
            // let expected = state.pop1(); // 32 or 64 (per the `Ixx` in `IxxAtomicWait`)
            // let (_flags, addr) =
            //     prepare_atomic_addr(memarg, implied_ty.bytes(), builder, state, environ)?;
            // assert!(builder.func.dfg.value_type(expected) == implied_ty);
            // // `fn translate_atomic_wait` can inspect the type of `expected` to figure out what
            // // code it needs to generate, if it wants.
            // let res = environ.translate_atomic_wait(
            //     builder.cursor(),
            //     heap_index,
            //     heap,
            //     addr,
            //     expected,
            //     timeout,
            // )?;
            // state.push1(res);
        }
        Operator::MemoryAtomicNotify { memarg } => {
            todo!();
            // let heap_index = MemoryIndex::from_u32(memarg.memory);
            // let heap = state.get_heap(builder.func, memarg.memory, environ)?;
            // let count = state.pop1(); // 32 (fixed)

            // // `memory.atomic.notify` is defined to have an access size of 4
            // // bytes in the spec, even though it doesn't necessarily access memory.
            // let (_flags, addr) = prepare_atomic_addr(memarg, 4, builder, state, environ)?;
            // let res =
            //     environ.translate_atomic_notify(builder.cursor(), heap_index, heap, addr, count)?;
            // state.push1(res);
        }
        Operator::I32AtomicLoad { memarg } => {
            todo!();
            // translate_atomic_load(I32, I32, memarg, builder, state, environ)?
        }
        Operator::I64AtomicLoad { memarg } => {
            todo!();
            // translate_atomic_load(I64, I64, memarg, builder, state, environ)?
        }
        Operator::I32AtomicLoad8U { memarg } => {
            todo!();
            // translate_atomic_load(I32, I8, memarg, builder, state, environ)?
        }
        Operator::I32AtomicLoad16U { memarg } => {
            todo!();
            // translate_atomic_load(I32, I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicLoad8U { memarg } => {
            todo!();
            // translate_atomic_load(I64, I8, memarg, builder, state, environ)?
        }
        Operator::I64AtomicLoad16U { memarg } => {
            todo!();
            // translate_atomic_load(I64, I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicLoad32U { memarg } => {
            todo!();
            // translate_atomic_load(I64, I32, memarg, builder, state, environ)?
        }

        Operator::I32AtomicStore { memarg } => {
            todo!();
            // translate_atomic_store(I32, memarg, builder, state, environ)?
        }
        Operator::I64AtomicStore { memarg } => {
            todo!();
            // translate_atomic_store(I64, memarg, builder, state, environ)?
        }
        Operator::I32AtomicStore8 { memarg } => {
            todo!();
            // translate_atomic_store(I8, memarg, builder, state, environ)?
        }
        Operator::I32AtomicStore16 { memarg } => {
            todo!();
            // translate_atomic_store(I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicStore8 { memarg } => {
            todo!();
            // translate_atomic_store(I8, memarg, builder, state, environ)?
        }
        Operator::I64AtomicStore16 { memarg } => {
            todo!();
            // translate_atomic_store(I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicStore32 { memarg } => {
            todo!();
            // translate_atomic_store(I32, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwAdd { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwAdd { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8AddU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16AddU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8AddU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16AddU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32AddU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::Add, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwSub { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwSub { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8SubU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16SubU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8SubU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16SubU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32SubU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::Sub, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwAnd { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwAnd { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8AndU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16AndU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8AndU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16AndU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::And, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32AndU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::And, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwOr { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwOr { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8OrU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16OrU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8OrU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16OrU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32OrU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::Or, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwXor { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwXor { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8XorU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16XorU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8XorU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16XorU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32XorU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::Xor, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwXchg { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I32, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwXchg { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I64, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8XchgU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I8, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16XchgU { memarg } => {
            todo!();
            // translate_atomic_rmw(I32, I16, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8XchgU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I8, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16XchgU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I16, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32XchgU { memarg } => {
            todo!();
            // translate_atomic_rmw(I64, I32, AtomicRmwOp::Xchg, memarg, builder, state, environ)?
        }

        Operator::I32AtomicRmwCmpxchg { memarg } => {
            todo!();
            // translate_atomic_cas(I32, I32, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmwCmpxchg { memarg } => {
            todo!();
            // translate_atomic_cas(I64, I64, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw8CmpxchgU { memarg } => {
            todo!();
            // translate_atomic_cas(I32, I8, memarg, builder, state, environ)?
        }
        Operator::I32AtomicRmw16CmpxchgU { memarg } => {
            todo!();
            // translate_atomic_cas(I32, I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw8CmpxchgU { memarg } => {
            todo!();
            // translate_atomic_cas(I64, I8, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw16CmpxchgU { memarg } => {
            todo!();
            // translate_atomic_cas(I64, I16, memarg, builder, state, environ)?
        }
        Operator::I64AtomicRmw32CmpxchgU { memarg } => {
            todo!();
            // translate_atomic_cas(I64, I32, memarg, builder, state, environ)?
        }

        Operator::AtomicFence { .. } => {
            todo!();
            // builder.ins().fence();
        }
        Operator::MemoryCopy { src_mem, dst_mem } => {
            todo!();
            // let src_index = MemoryIndex::from_u32(*src_mem);
            // let dst_index = MemoryIndex::from_u32(*dst_mem);
            // let src_heap = state.get_heap(builder.func, *src_mem, environ)?;
            // let dst_heap = state.get_heap(builder.func, *dst_mem, environ)?;
            // let len = state.pop1();
            // let src_pos = state.pop1();
            // let dst_pos = state.pop1();
            // environ.translate_memory_copy(
            //     builder.cursor(),
            //     src_index,
            //     src_heap,
            //     dst_index,
            //     dst_heap,
            //     dst_pos,
            //     src_pos,
            //     len,
            // )?;
        }
        Operator::MemoryFill { mem } => {
            todo!();
            // let heap_index = MemoryIndex::from_u32(*mem);
            // let heap = state.get_heap(builder.func, *mem, environ)?;
            // let len = state.pop1();
            // let val = state.pop1();
            // let dest = state.pop1();
            // environ.translate_memory_fill(builder.cursor(), heap_index, heap, dest, val, len)?;
        }
        Operator::MemoryInit { data_index, mem } => {
            todo!();
            // let heap_index = MemoryIndex::from_u32(*mem);
            // let heap = state.get_heap(builder.func, *mem, environ)?;
            // let len = state.pop1();
            // let src = state.pop1();
            // let dest = state.pop1();
            // environ.translate_memory_init(
            //     builder.cursor(),
            //     heap_index,
            //     heap,
            //     *data_index,
            //     dest,
            //     src,
            //     len,
            // )?;
        }
        Operator::DataDrop { data_index } => {
            todo!();
            // environ.translate_data_drop(builder.cursor(), *data_index)?;
        }
        Operator::TableSize { table: index } => {
            todo!();
            // let table = state.get_or_create_table(builder.func, *index, environ)?;
            // state.push1(environ.translate_table_size(
            //     builder.cursor(),
            //     TableIndex::from_u32(*index),
            //     table,
            // )?);
        }
        Operator::TableGrow { table: index } => {
            todo!();
            // let table_index = TableIndex::from_u32(*index);
            // let table = state.get_or_create_table(builder.func, *index, environ)?;
            // let delta = state.pop1();
            // let init_value = state.pop1();
            // state.push1(environ.translate_table_grow(
            //     builder.cursor(),
            //     table_index,
            //     table,
            //     delta,
            //     init_value,
            // )?);
        }
        Operator::TableGet { table: index } => {
            todo!();
            // let table_index = TableIndex::from_u32(*index);
            // let table = state.get_or_create_table(builder.func, *index, environ)?;
            // let index = state.pop1();
            // state.push1(environ.translate_table_get(builder, table_index, table, index)?);
        }
        Operator::TableSet { table: index } => {
            todo!();
            // let table_index = TableIndex::from_u32(*index);
            // let table = state.get_or_create_table(builder.func, *index, environ)?;
            // let value = state.pop1();
            // let index = state.pop1();
            // environ.translate_table_set(builder, table_index, table, value, index)?;
        }
        Operator::TableCopy {
            dst_table: dst_table_index,
            src_table: src_table_index,
        } => {
            todo!();
            // let dst_table = state.get_or_create_table(builder.func, *dst_table_index, environ)?;
            // let src_table = state.get_or_create_table(builder.func, *src_table_index, environ)?;
            // let len = state.pop1();
            // let src = state.pop1();
            // let dest = state.pop1();
            // environ.translate_table_copy(
            //     builder.cursor(),
            //     TableIndex::from_u32(*dst_table_index),
            //     dst_table,
            //     TableIndex::from_u32(*src_table_index),
            //     src_table,
            //     dest,
            //     src,
            //     len,
            // )?;
        }
        Operator::TableFill { table } => {
            todo!();
            // let table_index = TableIndex::from_u32(*table);
            // let len = state.pop1();
            // let val = state.pop1();
            // let dest = state.pop1();
            // environ.translate_table_fill(builder.cursor(), table_index, dest, val, len)?;
        }
        Operator::TableInit {
            elem_index,
            table: table_index,
        } => {
            todo!();
            // let table = state.get_or_create_table(builder.func, *table_index, environ)?;
            // let len = state.pop1();
            // let src = state.pop1();
            // let dest = state.pop1();
            // environ.translate_table_init(
            //     builder.cursor(),
            //     *elem_index,
            //     TableIndex::from_u32(*table_index),
            //     table,
            //     dest,
            //     src,
            //     len,
            // )?;
        }
        Operator::ElemDrop { elem_index } => {
            todo!();
            // environ.translate_elem_drop(builder.cursor(), *elem_index)?;
        }
        Operator::V128Const { value } => {
            todo!();
            // let data = value.bytes().to_vec().into();
            // let handle = builder.func.dfg.constants.insert(data);
            // let value = builder.ins().vconst(I8X16, handle);
            // // the v128.const is typed in CLIF as a I8x16 but raw_bitcast to a different type
            // // before use
            // state.push1(value)
        }
        Operator::I8x16Splat | Operator::I16x8Splat => {
            todo!();
            // let reduced = builder.ins().ireduce(type_of(op).lane_type(), state.pop1());
            // let splatted = builder.ins().splat(type_of(op), reduced);
            // state.push1(splatted)
        }
        Operator::I32x4Splat
        | Operator::I64x2Splat
        | Operator::F32x4Splat
        | Operator::F64x2Splat => {
            todo!();
            // let splatted = builder.ins().splat(type_of(op), state.pop1());
            // state.push1(splatted)
        }
        Operator::V128Load8Splat { memarg }
        | Operator::V128Load16Splat { memarg }
        | Operator::V128Load32Splat { memarg }
        | Operator::V128Load64Splat { memarg } => {
            todo!();
            // translate_load(
            //     memarg,
            //     ir::Opcode::Load,
            //     type_of(op).lane_type(),
            //     builder,
            //     state,
            //     environ,
            // )?;
            // let splatted = builder.ins().splat(type_of(op), state.pop1());
            // state.push1(splatted)
        }
        Operator::V128Load32Zero { memarg } | Operator::V128Load64Zero { memarg } => {
            todo!();
            // translate_load(
            //     memarg,
            //     ir::Opcode::Load,
            //     type_of(op).lane_type(),
            //     builder,
            //     state,
            //     environ,
            // )?;
            // let as_vector = builder.ins().scalar_to_vector(type_of(op), state.pop1());
            // state.push1(as_vector)
        }
        Operator::V128Load8Lane { memarg, lane }
        | Operator::V128Load16Lane { memarg, lane }
        | Operator::V128Load32Lane { memarg, lane }
        | Operator::V128Load64Lane { memarg, lane } => {
            todo!();
            // let vector = pop1_with_bitcast(state, type_of(op), builder);
            // translate_load(
            //     memarg,
            //     ir::Opcode::Load,
            //     type_of(op).lane_type(),
            //     builder,
            //     state,
            //     environ,
            // )?;
            // let replacement = state.pop1();
            // state.push1(builder.ins().insertlane(vector, replacement, *lane))
        }
        Operator::V128Store8Lane { memarg, lane }
        | Operator::V128Store16Lane { memarg, lane }
        | Operator::V128Store32Lane { memarg, lane }
        | Operator::V128Store64Lane { memarg, lane } => {
            todo!();
            // let vector = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().extractlane(vector, lane.clone()));
            // translate_store(memarg, ir::Opcode::Store, builder, state, environ)?;
        }
        Operator::I8x16ExtractLaneS { lane } | Operator::I16x8ExtractLaneS { lane } => {
            todo!();
            // let vector = pop1_with_bitcast(state, type_of(op), builder);
            // let extracted = builder.ins().extractlane(vector, lane.clone());
            // state.push1(builder.ins().sextend(I32, extracted))
        }
        Operator::I8x16ExtractLaneU { lane } | Operator::I16x8ExtractLaneU { lane } => {
            todo!();
            // let vector = pop1_with_bitcast(state, type_of(op), builder);
            // let extracted = builder.ins().extractlane(vector, lane.clone());
            // state.push1(builder.ins().uextend(I32, extracted));
            // // On x86, PEXTRB zeroes the upper bits of the destination register of extractlane so
            // // uextend could be elided; for now, uextend is needed for Cranelift's type checks to
            // // work.
        }
        Operator::I32x4ExtractLane { lane }
        | Operator::I64x2ExtractLane { lane }
        | Operator::F32x4ExtractLane { lane }
        | Operator::F64x2ExtractLane { lane } => {
            todo!();
            // let vector = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().extractlane(vector, lane.clone()))
        }
        Operator::I8x16ReplaceLane { lane } | Operator::I16x8ReplaceLane { lane } => {
            todo!();
            // let (vector, replacement) = state.pop2();
            // let ty = type_of(op);
            // let reduced = builder.ins().ireduce(ty.lane_type(), replacement);
            // let vector = optionally_bitcast_vector(vector, ty, builder);
            // state.push1(builder.ins().insertlane(vector, reduced, *lane))
        }
        Operator::I32x4ReplaceLane { lane }
        | Operator::I64x2ReplaceLane { lane }
        | Operator::F32x4ReplaceLane { lane }
        | Operator::F64x2ReplaceLane { lane } => {
            todo!();
            // let (vector, replacement) = state.pop2();
            // let vector = optionally_bitcast_vector(vector, type_of(op), builder);
            // state.push1(builder.ins().insertlane(vector, replacement, *lane))
        }
        Operator::I8x16Shuffle { lanes, .. } => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // let lanes = ConstantData::from(lanes.as_ref());
            // let mask = builder.func.dfg.immediates.push(lanes);
            // let shuffled = builder.ins().shuffle(a, b, mask);
            // state.push1(shuffled)
            // // At this point the original types of a and b are lost; users of this value (i.e. this
            // // WASM-to-CLIF translator) may need to raw_bitcast for type-correctness. This is due
            // // to WASM using the less specific v128 type for certain operations and more specific
            // // types (e.g. i8x16) for others.
        }
        Operator::I8x16Swizzle => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // state.push1(builder.ins().swizzle(I8X16, a, b))
        }
        Operator::I8x16Add | Operator::I16x8Add | Operator::I32x4Add | Operator::I64x2Add => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().iadd(a, b))
        }
        Operator::I8x16AddSatS | Operator::I16x8AddSatS => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().sadd_sat(a, b))
        }
        Operator::I8x16AddSatU | Operator::I16x8AddSatU => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().uadd_sat(a, b))
        }
        Operator::I8x16Sub | Operator::I16x8Sub | Operator::I32x4Sub | Operator::I64x2Sub => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().isub(a, b))
        }
        Operator::I8x16SubSatS | Operator::I16x8SubSatS => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().ssub_sat(a, b))
        }
        Operator::I8x16SubSatU | Operator::I16x8SubSatU => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().usub_sat(a, b))
        }
        Operator::I8x16MinS | Operator::I16x8MinS | Operator::I32x4MinS => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().imin(a, b))
        }
        Operator::I8x16MinU | Operator::I16x8MinU | Operator::I32x4MinU => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().umin(a, b))
        }
        Operator::I8x16MaxS | Operator::I16x8MaxS | Operator::I32x4MaxS => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().imax(a, b))
        }
        Operator::I8x16MaxU | Operator::I16x8MaxU | Operator::I32x4MaxU => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().umax(a, b))
        }
        Operator::I8x16Neg | Operator::I16x8Neg | Operator::I32x4Neg | Operator::I64x2Neg => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().ineg(a))
        }
        Operator::I8x16Abs | Operator::I16x8Abs | Operator::I32x4Abs | Operator::I64x2Abs => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().iabs(a))
        }
        Operator::I16x8Mul | Operator::I32x4Mul | Operator::I64x2Mul => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().imul(a, b))
        }
        Operator::V128Or => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().bor(a, b))
        }
        Operator::V128Xor => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().bxor(a, b))
        }
        Operator::V128And => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().band(a, b))
        }
        Operator::V128AndNot => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().band_not(a, b))
        }
        Operator::V128Not => {
            todo!();
            // let a = state.pop1();
            // state.push1(builder.ins().bnot(a));
        }
        Operator::I8x16Shl | Operator::I16x8Shl | Operator::I32x4Shl | Operator::I64x2Shl => {
            todo!();
            // let (a, b) = state.pop2();
            // let bitcast_a = optionally_bitcast_vector(a, type_of(op), builder);
            // // The spec expects to shift with `b mod lanewidth`; This is directly compatible
            // // with cranelift's instruction.
            // state.push1(builder.ins().ishl(bitcast_a, b))
        }
        Operator::I8x16ShrU | Operator::I16x8ShrU | Operator::I32x4ShrU | Operator::I64x2ShrU => {
            todo!();
            // let (a, b) = state.pop2();
            // let bitcast_a = optionally_bitcast_vector(a, type_of(op), builder);
            // // The spec expects to shift with `b mod lanewidth`; This is directly compatible
            // // with cranelift's instruction.
            // state.push1(builder.ins().ushr(bitcast_a, b))
        }
        Operator::I8x16ShrS | Operator::I16x8ShrS | Operator::I32x4ShrS | Operator::I64x2ShrS => {
            todo!();
            // let (a, b) = state.pop2();
            // let bitcast_a = optionally_bitcast_vector(a, type_of(op), builder);
            // // The spec expects to shift with `b mod lanewidth`; This is directly compatible
            // // with cranelift's instruction.
            // state.push1(builder.ins().sshr(bitcast_a, b))
        }
        Operator::V128Bitselect => {
            todo!();
            // let (a, b, c) = state.pop3();
            // let bitcast_a = optionally_bitcast_vector(a, I8X16, builder);
            // let bitcast_b = optionally_bitcast_vector(b, I8X16, builder);
            // let bitcast_c = optionally_bitcast_vector(c, I8X16, builder);
            // // The CLIF operand ordering is slightly different and the types of all three
            // // operands must match (hence the bitcast).
            // state.push1(builder.ins().bitselect(bitcast_c, bitcast_a, bitcast_b))
        }
        Operator::V128AnyTrue => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // let bool_result = builder.ins().vany_true(a);
            // state.push1(builder.ins().uextend(I32, bool_result))
        }
        Operator::I8x16AllTrue
        | Operator::I16x8AllTrue
        | Operator::I32x4AllTrue
        | Operator::I64x2AllTrue => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // let bool_result = builder.ins().vall_true(a);
            // state.push1(builder.ins().uextend(I32, bool_result))
        }
        Operator::I8x16Bitmask
        | Operator::I16x8Bitmask
        | Operator::I32x4Bitmask
        | Operator::I64x2Bitmask => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().vhigh_bits(I32, a));
        }
        Operator::I8x16Eq | Operator::I16x8Eq | Operator::I32x4Eq | Operator::I64x2Eq => {
            todo!();
            // translate_vector_icmp(IntCC::Equal, type_of(op), builder, state)
        }
        Operator::I8x16Ne | Operator::I16x8Ne | Operator::I32x4Ne | Operator::I64x2Ne => {
            todo!();
            // translate_vector_icmp(IntCC::NotEqual, type_of(op), builder, state)
        }
        Operator::I8x16GtS | Operator::I16x8GtS | Operator::I32x4GtS | Operator::I64x2GtS => {
            todo!();
            // translate_vector_icmp(IntCC::SignedGreaterThan, type_of(op), builder, state)
        }
        Operator::I8x16LtS | Operator::I16x8LtS | Operator::I32x4LtS | Operator::I64x2LtS => {
            todo!();
            // translate_vector_icmp(IntCC::SignedLessThan, type_of(op), builder, state)
        }
        Operator::I8x16GtU | Operator::I16x8GtU | Operator::I32x4GtU => {
            todo!();
            // translate_vector_icmp(IntCC::UnsignedGreaterThan, type_of(op), builder, state)
        }
        Operator::I8x16LtU | Operator::I16x8LtU | Operator::I32x4LtU => {
            todo!();
            // translate_vector_icmp(IntCC::UnsignedLessThan, type_of(op), builder, state)
        }
        Operator::I8x16GeS | Operator::I16x8GeS | Operator::I32x4GeS | Operator::I64x2GeS => {
            todo!();
            // translate_vector_icmp(IntCC::SignedGreaterThanOrEqual, type_of(op), builder, state)
        }
        Operator::I8x16LeS | Operator::I16x8LeS | Operator::I32x4LeS | Operator::I64x2LeS => {
            todo!();
            // translate_vector_icmp(IntCC::SignedLessThanOrEqual, type_of(op), builder, state)
        }
        Operator::I8x16GeU | Operator::I16x8GeU | Operator::I32x4GeU => todo!(), //translate_vector_icmp(
        // IntCC::UnsignedGreaterThanOrEqual,
        // type_of(op),
        // builder,
        // state,
        // ),
        Operator::I8x16LeU | Operator::I16x8LeU | Operator::I32x4LeU => {
            todo!();
            // translate_vector_icmp(IntCC::UnsignedLessThanOrEqual, type_of(op), builder, state)
        }
        Operator::F32x4Eq | Operator::F64x2Eq => {
            todo!();
            // translate_vector_fcmp(FloatCC::Equal, type_of(op), builder, state)
        }
        Operator::F32x4Ne | Operator::F64x2Ne => {
            todo!();
            // translate_vector_fcmp(FloatCC::NotEqual, type_of(op), builder, state)
        }
        Operator::F32x4Lt | Operator::F64x2Lt => {
            todo!();
            // translate_vector_fcmp(FloatCC::LessThan, type_of(op), builder, state)
        }
        Operator::F32x4Gt | Operator::F64x2Gt => {
            todo!();
            // translate_vector_fcmp(FloatCC::GreaterThan, type_of(op), builder, state)
        }
        Operator::F32x4Le | Operator::F64x2Le => {
            todo!();
            // translate_vector_fcmp(FloatCC::LessThanOrEqual, type_of(op), builder, state)
        }
        Operator::F32x4Ge | Operator::F64x2Ge => {
            todo!();
            // translate_vector_fcmp(FloatCC::GreaterThanOrEqual, type_of(op), builder, state)
        }
        Operator::F32x4Add | Operator::F64x2Add => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fadd(a, b))
        }
        Operator::F32x4Sub | Operator::F64x2Sub => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fsub(a, b))
        }
        Operator::F32x4Mul | Operator::F64x2Mul => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fmul(a, b))
        }
        Operator::F32x4Div | Operator::F64x2Div => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fdiv(a, b))
        }
        Operator::F32x4Max | Operator::F64x2Max => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fmax(a, b))
        }
        Operator::F32x4Min | Operator::F64x2Min => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fmin(a, b))
        }
        Operator::F32x4PMax | Operator::F64x2PMax => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fmax_pseudo(a, b))
        }
        Operator::F32x4PMin | Operator::F64x2PMin => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fmin_pseudo(a, b))
        }
        Operator::F32x4Sqrt | Operator::F64x2Sqrt => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().sqrt(a))
        }
        Operator::F32x4Neg | Operator::F64x2Neg => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fneg(a))
        }
        Operator::F32x4Abs | Operator::F64x2Abs => {
            todo!();
            // let a = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().fabs(a))
        }
        Operator::F32x4ConvertI32x4S => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().fcvt_from_sint(F32X4, a))
        }
        Operator::F32x4ConvertI32x4U => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().fcvt_from_uint(F32X4, a))
        }
        Operator::F64x2ConvertLowI32x4S => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().fcvt_low_from_sint(F64X2, a));
        }
        Operator::F64x2ConvertLowI32x4U => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // let widened_a = builder.ins().uwiden_low(a);
            // state.push1(builder.ins().fcvt_from_uint(F64X2, widened_a));
        }
        Operator::F64x2PromoteLowF32x4 => {
            todo!();
            // let a = pop1_with_bitcast(state, F32X4, builder);
            // state.push1(builder.ins().fvpromote_low(a));
        }
        Operator::F32x4DemoteF64x2Zero => {
            todo!();
            // let a = pop1_with_bitcast(state, F64X2, builder);
            // state.push1(builder.ins().fvdemote(a));
        }
        Operator::I32x4TruncSatF32x4S => {
            todo!();
            // let a = pop1_with_bitcast(state, F32X4, builder);
            // state.push1(builder.ins().fcvt_to_sint_sat(I32X4, a))
        }
        Operator::I32x4TruncSatF64x2SZero => {
            todo!();
            // let a = pop1_with_bitcast(state, F64X2, builder);
            // let converted_a = builder.ins().fcvt_to_sint_sat(I64X2, a);
            // let handle = builder.func.dfg.constants.insert(vec![0u8; 16].into());
            // let zero = builder.ins().vconst(I64X2, handle);

            // state.push1(builder.ins().snarrow(converted_a, zero));
        }
        Operator::I32x4TruncSatF32x4U => {
            todo!();
            // let a = pop1_with_bitcast(state, F32X4, builder);
            // state.push1(builder.ins().fcvt_to_uint_sat(I32X4, a))
        }
        Operator::I32x4TruncSatF64x2UZero => {
            todo!();
            // let a = pop1_with_bitcast(state, F64X2, builder);
            // let converted_a = builder.ins().fcvt_to_uint_sat(I64X2, a);
            // let handle = builder.func.dfg.constants.insert(vec![0u8; 16].into());
            // let zero = builder.ins().vconst(I64X2, handle);

            // state.push1(builder.ins().uunarrow(converted_a, zero));
        }
        Operator::I8x16NarrowI16x8S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().snarrow(a, b))
        }
        Operator::I16x8NarrowI32x4S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().snarrow(a, b))
        }
        Operator::I8x16NarrowI16x8U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().unarrow(a, b))
        }
        Operator::I16x8NarrowI32x4U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().unarrow(a, b))
        }
        Operator::I16x8ExtendLowI8x16S => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // state.push1(builder.ins().swiden_low(a))
        }
        Operator::I16x8ExtendHighI8x16S => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // state.push1(builder.ins().swiden_high(a))
        }
        Operator::I16x8ExtendLowI8x16U => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // state.push1(builder.ins().uwiden_low(a))
        }
        Operator::I16x8ExtendHighI8x16U => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // state.push1(builder.ins().uwiden_high(a))
        }
        Operator::I32x4ExtendLowI16x8S => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().swiden_low(a))
        }
        Operator::I32x4ExtendHighI16x8S => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().swiden_high(a))
        }
        Operator::I32x4ExtendLowI16x8U => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().uwiden_low(a))
        }
        Operator::I32x4ExtendHighI16x8U => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().uwiden_high(a))
        }
        Operator::I64x2ExtendLowI32x4S => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().swiden_low(a))
        }
        Operator::I64x2ExtendHighI32x4S => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().swiden_high(a))
        }
        Operator::I64x2ExtendLowI32x4U => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().uwiden_low(a))
        }
        Operator::I64x2ExtendHighI32x4U => {
            todo!();
            // let a = pop1_with_bitcast(state, I32X4, builder);
            // state.push1(builder.ins().uwiden_high(a))
        }
        Operator::I16x8ExtAddPairwiseI8x16S => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // let widen_low = builder.ins().swiden_low(a);
            // let widen_high = builder.ins().swiden_high(a);
            // state.push1(builder.ins().iadd_pairwise(widen_low, widen_high));
        }
        Operator::I32x4ExtAddPairwiseI16x8S => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // let widen_low = builder.ins().swiden_low(a);
            // let widen_high = builder.ins().swiden_high(a);
            // state.push1(builder.ins().iadd_pairwise(widen_low, widen_high));
        }
        Operator::I16x8ExtAddPairwiseI8x16U => {
            todo!();
            // let a = pop1_with_bitcast(state, I8X16, builder);
            // let widen_low = builder.ins().uwiden_low(a);
            // let widen_high = builder.ins().uwiden_high(a);
            // state.push1(builder.ins().iadd_pairwise(widen_low, widen_high));
        }
        Operator::I32x4ExtAddPairwiseI16x8U => {
            todo!();
            // let a = pop1_with_bitcast(state, I16X8, builder);
            // let widen_low = builder.ins().uwiden_low(a);
            // let widen_high = builder.ins().uwiden_high(a);
            // state.push1(builder.ins().iadd_pairwise(widen_low, widen_high));
        }
        Operator::F32x4Ceil | Operator::F64x2Ceil => {
            todo!();
            // // This is something of a misuse of `type_of`, because that produces the return type
            // // of `op`.  In this case we want the arg type, but we know it's the same as the
            // // return type.  Same for the 3 cases below.
            // let arg = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().ceil(arg));
        }
        Operator::F32x4Floor | Operator::F64x2Floor => {
            todo!();
            // let arg = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().floor(arg));
        }
        Operator::F32x4Trunc | Operator::F64x2Trunc => {
            todo!();
            // let arg = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().trunc(arg));
        }
        Operator::F32x4Nearest | Operator::F64x2Nearest => {
            todo!();
            // let arg = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().nearest(arg));
        }
        Operator::I32x4DotI16x8S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().widening_pairwise_dot_product_s(a, b));
        }
        Operator::I8x16Popcnt => {
            todo!();
            // let arg = pop1_with_bitcast(state, type_of(op), builder);
            // state.push1(builder.ins().popcnt(arg));
        }
        Operator::I16x8Q15MulrSatS => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // state.push1(builder.ins().sqmul_round_sat(a, b))
        }
        Operator::I16x8ExtMulLowI8x16S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // let a_low = builder.ins().swiden_low(a);
            // let b_low = builder.ins().swiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I16x8ExtMulHighI8x16S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // let a_high = builder.ins().swiden_high(a);
            // let b_high = builder.ins().swiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::I16x8ExtMulLowI8x16U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // let a_low = builder.ins().uwiden_low(a);
            // let b_low = builder.ins().uwiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I16x8ExtMulHighI8x16U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I8X16, builder);
            // let a_high = builder.ins().uwiden_high(a);
            // let b_high = builder.ins().uwiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::I32x4ExtMulLowI16x8S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // let a_low = builder.ins().swiden_low(a);
            // let b_low = builder.ins().swiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I32x4ExtMulHighI16x8S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // let a_high = builder.ins().swiden_high(a);
            // let b_high = builder.ins().swiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::I32x4ExtMulLowI16x8U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // let a_low = builder.ins().uwiden_low(a);
            // let b_low = builder.ins().uwiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I32x4ExtMulHighI16x8U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I16X8, builder);
            // let a_high = builder.ins().uwiden_high(a);
            // let b_high = builder.ins().uwiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::I64x2ExtMulLowI32x4S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // let a_low = builder.ins().swiden_low(a);
            // let b_low = builder.ins().swiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I64x2ExtMulHighI32x4S => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // let a_high = builder.ins().swiden_high(a);
            // let b_high = builder.ins().swiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::I64x2ExtMulLowI32x4U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // let a_low = builder.ins().uwiden_low(a);
            // let b_low = builder.ins().uwiden_low(b);
            // state.push1(builder.ins().imul(a_low, b_low));
        }
        Operator::I64x2ExtMulHighI32x4U => {
            todo!();
            // let (a, b) = pop2_with_bitcast(state, I32X4, builder);
            // let a_high = builder.ins().uwiden_high(a);
            // let b_high = builder.ins().uwiden_high(b);
            // state.push1(builder.ins().imul(a_high, b_high));
        }
        Operator::ReturnCall { .. } | Operator::ReturnCallIndirect { .. } => {
            todo!();
            // return Err(wasm_unsupported!("proposed tail-call operator {:?}", op));
        }
        Operator::I8x16RelaxedSwizzle
        | Operator::I32x4RelaxedTruncSatF32x4S
        | Operator::I32x4RelaxedTruncSatF32x4U
        | Operator::I32x4RelaxedTruncSatF64x2SZero
        | Operator::I32x4RelaxedTruncSatF64x2UZero
        | Operator::F32x4RelaxedFma
        | Operator::F32x4RelaxedFnma
        | Operator::F64x2RelaxedFma
        | Operator::F64x2RelaxedFnma
        | Operator::I8x16RelaxedLaneselect
        | Operator::I16x8RelaxedLaneselect
        | Operator::I32x4RelaxedLaneselect
        | Operator::I64x2RelaxedLaneselect
        | Operator::F32x4RelaxedMin
        | Operator::F32x4RelaxedMax
        | Operator::F64x2RelaxedMin
        | Operator::F64x2RelaxedMax
        | Operator::I16x8RelaxedQ15mulrS
        | Operator::I16x8DotI8x16I7x16S
        | Operator::I32x4DotI8x16I7x16AddS
        | Operator::F32x4RelaxedDotBf16x8AddF32x4 => {
            todo!();
            // return Err(wasm_unsupported!("proposed relaxed-simd operator {:?}", op));
        }
        Operator::I8x16AvgrU => todo!(),
        Operator::I16x8AvgrU => todo!(),
    };
    Ok(())
}
