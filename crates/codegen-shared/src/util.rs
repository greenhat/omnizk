use std::collections::HashMap;

use c2zk_ir::ir::FuncIndex;

// TODO: it should be a part of new FuncNameResolver struct
pub fn func_index_to_label(
    func_index: FuncIndex,
    func_names: &HashMap<FuncIndex, String>,
) -> String {
    func_names
        .get(&func_index)
        .unwrap_or(&format!("f{}", u32::from(func_index)))
        .to_string()
}
