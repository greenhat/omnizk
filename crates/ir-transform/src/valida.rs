use ozk_valida_dialect::types::FramePointer;
use ozk_wasm_dialect::types::StackDepth;

pub mod lowering;

pub fn fp_from_wasm_stack(stack_depth: StackDepth) -> FramePointer {
    let v: i32 = stack_depth.into();
    (-v * 4).into()
}
